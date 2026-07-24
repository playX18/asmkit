# This file is part of asmkit.
#
# Port of asmjit's `db/exp.js` (asmjit pinned at
# 0bd5787b54b575ed94bf32ac452153b34385c514, SPDX-License-Identifier: Zlib).

"""Expression tokenizer, parser, AST, and numeric evaluator.

Faithful port of asmjit's `db/exp.js` with the following documented
decisions (see this package's README.md for the full divergence list):

- TERNARY: the JS ternary branch is dead code that references an undefined
  `info` variable (a ReferenceError if ever reached), and no asmjit JSON
  data uses `? :`. This port raises `ExpressionError` on `?` instead of
  replicating the bug.
- Bit access `x[i]` wraps the index in an `ImmNode`; JS stores the raw
  number, which breaks `evaluate()` and visitors on the resulting node.
- `evaluate()` emulates JS 32-bit semantics: arithmetic results are masked
  to signed int32, `/` and `%` truncate toward zero like JS `|0`, and shift
  counts are masked to 5 bits like JS `<<`/`>>`.
"""

import re
from typing import NamedTuple

UNARY_OPERATORS = {
    "-": {"prec": 3, "rtl": 1, "emit": "-@1"},
    "~": {"prec": 3, "rtl": 1, "emit": "~@1"},
    "!": {"prec": 3, "rtl": 1, "emit": "!@1"},
}

BINARY_OPERATORS = {
    "*":  {"prec": 5,  "rtl": 0, "emit": "@1 * @2"},
    "/":  {"prec": 5,  "rtl": 0, "emit": "@1 / @2"},
    "%":  {"prec": 5,  "rtl": 0, "emit": "@1 % @2"},
    "+":  {"prec": 6,  "rtl": 0, "emit": "@1 + @2"},
    "-":  {"prec": 6,  "rtl": 0, "emit": "@1 - @2"},
    ">>": {"prec": 7,  "rtl": 0, "emit": "@1 >> @2"},
    "<<": {"prec": 7,  "rtl": 0, "emit": "@1 << @2"},
    "<":  {"prec": 9,  "rtl": 0, "emit": "@1 < @2"},
    ">":  {"prec": 9,  "rtl": 0, "emit": "@1 > @2"},
    "<=": {"prec": 9,  "rtl": 0, "emit": "@1 <= @2"},
    ">=": {"prec": 9,  "rtl": 0, "emit": "@1 >= @2"},
    "==": {"prec": 10, "rtl": 0, "emit": "@1 == @2"},
    "!=": {"prec": 10, "rtl": 0, "emit": "@1 != @2"},
    "&":  {"prec": 11, "rtl": 0, "emit": "@1 & @2"},
    "^":  {"prec": 12, "rtl": 0, "emit": "@1 ^ @2"},
    "|":  {"prec": 13, "rtl": 0, "emit": "@1 | @2"},
    "&&": {"prec": 14, "rtl": 0, "emit": "@1 && @2"},
    "||": {"prec": 15, "rtl": 0, "emit": "@1 || @2"},
    "?":  {"prec": 16, "rtl": 0, "emit": "@1 ? @2"},
    ":":  {"prec": 16, "rtl": 0, "emit": "@1 : @2"},
}

MAX_OPERATOR_LEN = 4


def _right_associate(info, b_prec):
    return info["prec"] > b_prec or (info["prec"] == b_prec and info["rtl"])


class ExpressionError(Exception):
    """Carries `message` and `position` (-1 when not tied to a source offset)."""

    def __init__(self, message, position=None):
        super().__init__(message)
        self.message = message
        self.position = position if position is not None else -1


def _throw_tokenizer_error(token):
    raise ExpressionError(f"Unexpected token '{token.data}'", token.position)


def _to_int32(x):
    # JS ToInt32: truncate toward zero, then wrap into signed 32 bits.
    if isinstance(x, float):
        x = int(x)
    x &= 0xFFFFFFFF
    return x - 0x100000000 if x >= 0x80000000 else x


def _trunc_div(a, b):
    q = abs(a) // abs(b)
    return q if (a < 0) == (b < 0) else -q


def _trunc_mod(a, b):
    r = abs(a) % abs(b)
    return -r if a < 0 else r


def _must_enclose(node):
    if node.is_unary():
        return node.child.is_operator()
    if node.is_binary():
        return True
    return False


class ExpNode:
    def __init__(self, type_):
        self.type = type_

    def is_imm(self):
        return self.type == "imm"

    def is_var(self):
        return self.type == "var"

    def is_call(self):
        return self.type == "call"

    def is_unary(self):
        return self.type == "unary"

    def is_binary(self):
        return self.type == "binary"

    def is_operator(self):
        return self.type in ("unary", "binary")

    def info(self):
        return None

    def clone(self):
        raise NotImplementedError("ExpNode.clone() must be overridden")

    def evaluate(self, ctx=None):
        raise NotImplementedError("ExpNode.evaluate() must be overridden")

    def to_string(self, ctx=None):
        raise NotImplementedError("ExpNode.to_string() must be overridden")


class ImmNode(ExpNode):
    def __init__(self, imm=0):
        super().__init__("imm")
        self.imm = imm or 0

    def clone(self):
        return ImmNode(self.imm)

    def evaluate(self, ctx=None):
        return self.imm

    def to_string(self, ctx=None):
        return ctx.stringify_immediate(self.imm) if ctx else str(self.imm)


class VarNode(ExpNode):
    def __init__(self, name=""):
        super().__init__("var")
        self.name = name or ""

    def __getitem__(self, index):
        # Bit-access sugar: `x[i]` -> `$bit(x, i)`.
        return Call("$bit", [self, Imm(index)])

    def clone(self):
        return VarNode(self.name)

    def evaluate(self, ctx=None):
        if ctx is None:
            raise ExpressionError(f"Cannot evaluate variable '{self.name}' without a context")
        return ctx.variable(self.name)

    def to_string(self, ctx=None):
        return ctx.stringify_variable(self.name) if ctx else str(self.name)


class CallNode(ExpNode):
    def __init__(self, name="", args=None):
        super().__init__("call")
        self.name = name or ""
        self.args = args if args is not None else []

    def clone(self):
        return CallNode(self.name, [arg.clone() for arg in self.args])

    def evaluate(self, ctx=None):
        if ctx is None:
            raise ExpressionError(f"Cannot evaluate function '{self.name}' without a context")
        evaluated_args = [arg.evaluate(ctx) for arg in self.args]
        return ctx.function(self.name, evaluated_args)

    def to_string(self, ctx=None):
        if self.name == "$bit":
            return f"(({self.args[0].to_string()} >> {self.args[1].to_string()}) & 1)"
        args_code = ", ".join(arg.to_string(ctx) for arg in self.args)
        name = ctx.stringify_function(self.name) if ctx else self.name
        return f"{name}({args_code})"


class UnaryNode(ExpNode):
    def __init__(self, op, child=None):
        if op not in UNARY_OPERATORS:
            raise ValueError(f"Invalid unary operator '{op}'")
        super().__init__("unary")
        self.op = op
        self.child = child

    def info(self):
        return UNARY_OPERATORS[self.op]

    def clone(self):
        # JS clones the nonexistent `this.left` (yielding null); fixed here.
        return UnaryNode(self.op, self.child.clone() if self.child is not None else None)

    def evaluate(self, ctx=None):
        val = self.child.evaluate(ctx)
        if self.op == "-":
            return _to_int32(-val)
        if self.op == "~":
            return _to_int32(~_to_int32(val))
        if self.op == "!":
            return 0 if val else 1
        if ctx is None:
            raise ExpressionError(f"Cannot evaluate operator '{self.op}' without a context")
        return ctx.unary(self.op, val)

    def to_string(self, ctx=None):
        code = self.child.to_string(ctx)
        if _must_enclose(self.child):
            code = f"({code})"
        return self.info()["emit"].replace("@1", code)


class BinaryNode(ExpNode):
    def __init__(self, op, left=None, right=None):
        if op not in BINARY_OPERATORS:
            raise ValueError(f"Invalid binary operator '{op}'")
        super().__init__("binary")
        self.op = op or ""
        self.left = left
        self.right = right

    def info(self):
        return BINARY_OPERATORS[self.op]

    def clone(self):
        return BinaryNode(
            self.op,
            self.left.clone() if self.left is not None else None,
            self.right.clone() if self.right is not None else None)

    def evaluate(self, ctx=None):
        left = self.left.evaluate(ctx)
        right = self.right.evaluate(ctx)
        op = self.op

        if op == "+":
            return _to_int32(left + right)
        if op == "-":
            return _to_int32(left - right)
        if op == "*":
            return _to_int32(left * right)
        if op == "/":
            return _to_int32(_trunc_div(left, right))
        if op == "%":
            return _to_int32(_trunc_mod(left, right))
        if op == "&":
            return _to_int32(left & right)
        if op == "|":
            return _to_int32(left | right)
        if op == "^":
            return _to_int32(left ^ right)
        if op == "<<":
            return _to_int32(left << (right & 31))
        if op == ">>":
            return _to_int32(_to_int32(left) >> (right & 31))
        if op == "==":
            return 1 if left == right else 0
        if op == "!=":
            return 1 if left != right else 0
        if op == "<":
            return 1 if left < right else 0
        if op == "<=":
            return 1 if left <= right else 0
        if op == ">":
            return 1 if left > right else 0
        if op == ">=":
            return 1 if left >= right else 0
        if op == "&&":
            return 1 if left and right else 0
        if op == "||":
            return 1 if left or right else 0

        if ctx is None:
            raise ExpressionError(f"Cannot evaluate operator '{op}' without a context")
        return ctx.binary(op, left, right)

    def to_string(self, ctx=None):
        left = self.left.to_string(ctx)
        if _must_enclose(self.left):
            left = f"({left})"
        right = self.right.to_string(ctx)
        if _must_enclose(self.right):
            right = f"({right})"
        return self.info()["emit"].replace("@1", left).replace("@2", right)


def Imm(imm):
    return ImmNode(imm)


def Var(name):
    return VarNode(name)


def Call(name, args):
    return CallNode(name, args)


def Unary(op, child=None):
    return UnaryNode(op, child)


def Binary(op, left=None, right=None):
    return BinaryNode(op, left, right)


def Negate(child):
    return Unary("-", child)


def BitNot(child):
    return Unary("~", child)


def Add(left, right):
    return Binary("+", left, right)


def Sub(left, right):
    return Binary("-", left, right)


def Mul(left, right):
    return Binary("*", left, right)


def Div(left, right):
    return Binary("/", left, right)


def Mod(left, right):
    return Binary("%", left, right)


def Shl(left, right):
    return Binary("<<", left, right)


def Shr(left, right):
    return Binary(">>", left, right)


def BitAnd(left, right):
    return Binary("&", left, right)


def BitOr(left, right):
    return Binary("|", left, right)


def BitXor(left, right):
    return Binary("^", left, right)


def Eq(left, right):
    return Binary("==", left, right)


def Ne(left, right):
    return Binary("!=", left, right)


def Lt(left, right):
    return Binary("<", left, right)


def Le(left, right):
    return Binary("<=", left, right)


def Gt(left, right):
    return Binary(">", left, right)


def Ge(left, right):
    return Binary(">=", left, right)


def And(left, right):
    return Binary("&&", left, right)


def Or(left, right):
    return Binary("||", left, right)


CHAR_NONE = 0   # '_' - Invalid or <end>.
CHAR_SPACE = 1  # 'S' - Space.
CHAR_ALPHA = 2  # 'A' - Alpha [A-Za-z_].
CHAR_DIGIT = 3  # 'D' - Digit [0-9].
CHAR_PUNCT = 4  # '$' - Punctuation.


def _char_category(code):
    # Same classification as the JS 160-entry Category table.
    if code < 0 or code >= 160:
        return CHAR_NONE
    if code == 32 or 9 <= code <= 13:
        return CHAR_SPACE
    if 48 <= code <= 57:
        return CHAR_DIGIT
    if 65 <= code <= 90 or 97 <= code <= 122 or code == 95:
        return CHAR_ALPHA
    if 32 < code < 127:
        return CHAR_PUNCT
    return CHAR_NONE


TOKEN_NONE = 0
TOKEN_PUNCT = 1
TOKEN_IDENT = 2
TOKEN_VALUE = 3


class Token(NamedTuple):
    type: int        # Token type, see `TOKEN_...`.
    position: int    # Token position in expression's source.
    data: str        # Token data (content) as string.
    value: object = None  # Token value (only if the token is a value).


NO_TOKEN = Token(TOKEN_NONE, -1, "<end>", None)

# Mirrors the JS `reNumValue`, including its `[E|e]` / `[+|-]` character
# class quirk (the `|` is a literal member of the class in JS).
_RE_NUM_VALUE = re.compile(r"(?:(?:\d*\.\d+|\d+)(?:[E|e][+|-]?\d+)?)")
_RE_FLOAT_PREFIX = re.compile(r"[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?")


def _js_parse_float(s):
    # JS parseFloat(): parses the longest valid float prefix.
    m = _RE_FLOAT_PREFIX.match(s)
    value = float(m.group(0)) if m else float("nan")
    # JS numbers are doubles; normalize integral values to int so that
    # `str(imm)` matches JS `String(imm)` ("1", not "1.0").
    return int(value) if value.is_integer() else value


def parse_hex(source, from_):
    i = from_
    number = 0

    while i < len(source):
        c = source[i]

        if "0" <= c <= "9":
            n = ord(c) - ord("0")
        elif "a" <= c <= "f":
            n = ord(c) - ord("a") + 10
        elif "A" <= c <= "F":
            n = ord(c) - ord("A") + 10
        elif "g" <= c <= "z" or "g" <= c <= "Z":
            # The second clause is dead in JS ('g' > 'Z'); kept for parity.
            raise ExpressionError(f"Invalid hex number 0x{source[from_:i + 1]}")
        else:
            break

        number = _to_int32((number << 4) | n)
        i += 1

    if i == from_:
        raise ExpressionError("Invalid number starting with 0x")

    return number, i


def tokenize(source):
    length = len(source)
    tokens = []

    i = 0
    while i < length:
        c = ord(source[i])
        cat = _char_category(c)

        if cat == CHAR_SPACE:
            i += 1
        elif cat == CHAR_DIGIT:
            # Hex number.
            if source[i] == "0" and i + 1 < length and source[i + 1] == "x":
                number, end = parse_hex(source, i + 2)
                tokens.append(Token(TOKEN_VALUE, i, source[i:end], number))
                i = end
            else:
                n = len(tokens) - 1
                if n >= 0 and tokens[n].data == "." and source[i - 1] == ".":
                    tokens.pop()
                    i -= 1

                data = _RE_NUM_VALUE.match(source, i).group(0)
                tokens.append(Token(TOKEN_VALUE, i, data, _js_parse_float(data)))
                i += len(data)
        elif cat == CHAR_ALPHA:
            start = i
            i += 1
            while i < length and _char_category(ord(source[i])) in (CHAR_ALPHA, CHAR_DIGIT):
                i += 1
            tokens.append(Token(TOKEN_IDENT, start, source[start:i]))
        elif cat == CHAR_PUNCT:
            start = i
            i += 1
            while i < length and _char_category(ord(source[i])) == CHAR_PUNCT:
                i += 1
            # Greedily split the punctuation run into known operators.
            while start < i:
                for j in range(min(i - start, MAX_OPERATOR_LEN), 0, -1):
                    part = source[start:start + j]
                    if part in UNARY_OPERATORS or part in BINARY_OPERATORS or j == 1:
                        tokens.append(Token(TOKEN_PUNCT, start, part))
                        start += j
                        break
        else:
            raise ExpressionError(f"Unrecognized character '0x{c:x}'", i)

    return tokens


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.t_index = 0

    def peek(self):
        return self.tokens[self.t_index] if self.t_index < len(self.tokens) else NO_TOKEN

    def next(self):
        token = self.peek()
        if token is not NO_TOKEN:
            self.t_index += 1
        return token

    def skip(self):
        self.t_index += 1
        return self

    def parse(self):
        # The root expression cannot be empty.
        if self.peek() is NO_TOKEN:
            raise ExpressionError("Expression cannot be empty", 0)

        node = self.parse_expression()

        # The root expression must reach the end of the input.
        token = self.peek()
        if token is not NO_TOKEN:
            _throw_tokenizer_error(token)

        return node

    def parse_expression(self):
        stack = []
        value = None

        while True:
            # The only case of value not being `None` is after ternary-if.
            if value is None:
                unary_first = None
                unary_last = None

                token = self.next()

                # Parse a possible unary operator(s).
                if token.type == TOKEN_PUNCT:
                    while True:
                        op_name = token.data
                        if op_name not in UNARY_OPERATORS:
                            break

                        node = Unary(op_name)
                        if unary_last is not None:
                            unary_last.child = node
                        else:
                            unary_first = node

                        unary_last = node
                        token = self.next()
                        if token.type != TOKEN_PUNCT:
                            break

                # Parse a value, variable, function call, or nested expression.
                if token.type == TOKEN_VALUE:
                    value = Imm(token.value)
                elif token.type == TOKEN_IDENT:
                    name = token.data
                    after = self.peek()

                    if after.data == "(":
                        value = self.parse_call(name)
                    elif after.data == "[":
                        value = self.parse_bit_access(name)
                    else:
                        value = Var(name)
                elif token.data == "(":
                    value = self.parse_expression()
                    token = self.next()

                    if token.data != ")":
                        _throw_tokenizer_error(token)
                else:
                    _throw_tokenizer_error(token)

                # Replace the value with the top-level unary operator, if parsed.
                if unary_first is not None:
                    unary_last.child = value
                    value = unary_first

            # Parse a possible binary operator - the loop must repeat, if present.
            token = self.peek()
            if token.type == TOKEN_PUNCT and token.data in BINARY_OPERATORS:
                op_name = token.data
                if op_name == ":":
                    break

                # Consume the token.
                self.skip()

                if op_name == "?":
                    # The JS branch here is broken dead code (it references an
                    # undefined `info` variable); ternary never occurs in the
                    # JSON data, so this port rejects it with a clear error.
                    raise ExpressionError(
                        "Ternary operator '?:' is not supported",
                        token.position)

                b_node = Binary(op_name, None, None)

                if not stack:
                    b_node.left = value
                    stack.append(b_node)
                else:
                    a_node = stack.pop()
                    a_prec = a_node.info()["prec"]
                    b_prec = b_node.info()["prec"]

                    if a_prec > b_prec:
                        a_node.right = b_node
                        b_node.left = value
                        stack.extend((a_node, b_node))
                    else:
                        a_node.right = value

                        # Advance to the top-most op that has less/equal
                        # precedence than `b_prec`.
                        while stack:
                            if _right_associate(a_node.info(), b_prec):
                                break
                            a_node = stack.pop()

                        if not stack and not _right_associate(a_node.info(), b_prec):
                            b_node.left = a_node
                            stack.append(b_node)
                        else:
                            tmp = a_node.right
                            a_node.right = b_node
                            b_node.left = tmp
                            stack.extend((a_node, b_node))

                value = None
                continue

            break

        if value is None:
            raise ExpressionError("Invalid expression")

        if stack:
            stack[-1].right = value
            value = stack[0]

        return value

    def parse_call(self, name):
        args = []

        token = self.next()
        if token.data != "(":
            _throw_tokenizer_error(token)

        while True:
            token = self.peek()
            if token.data == ")":
                break

            if args:
                if token.data != ",":
                    _throw_tokenizer_error(token)
                self.skip()

            args.append(self.parse_expression())

        self.skip()
        return Call(name, args)

    def parse_bit_access(self, name):
        token = self.next()
        if token.data != "[":
            _throw_tokenizer_error(token)

        token = self.next()
        if token.type != TOKEN_VALUE:
            _throw_tokenizer_error(token)

        index = token.value

        token = self.next()
        if token.data != "]":
            _throw_tokenizer_error(token)

        # JS passes the raw number as the second arg (breaking evaluate() and
        # visitors on that node); the port wraps it in an ImmNode.
        return Call("$bit", [Var(name), Imm(index)])


def parse(source):
    tokens = tokenize(source)
    return Parser(tokens).parse()


class Visitor:
    def visit(self, node):
        node_type = node.type

        if node_type in ("imm", "var"):
            pass
        elif node_type == "call":
            for arg in node.args:
                self.visit(arg)
        elif node_type == "unary":
            if node.child is not None:
                self.visit(node.child)
        elif node_type == "binary":
            if node.left is not None:
                self.visit(node.left)
            if node.right is not None:
                self.visit(node.right)
        else:
            raise ValueError(f"Visitor.visit(): Unknown node type '{node.type}'")


class Collector(Visitor):
    def __init__(self, node_type, dst=None):
        super().__init__()
        self.dict = dst if dst is not None else {}
        self.node_type = node_type

    def visit(self, node):
        if node.type == self.node_type:
            self.dict[node.name] = self.dict.get(node.name, 0) + 1
        super().visit(node)


def collect_vars(node, dst=None):
    collector = Collector("var", dst)
    collector.visit(node)
    return collector.dict


def collect_calls(node, dst=None):
    collector = Collector("call", dst)
    collector.visit(node)
    return collector.dict
