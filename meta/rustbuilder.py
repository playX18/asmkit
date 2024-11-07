from typing import Self
from collections.abc import Callable
id = 0

class Formatter:

    def __init__(self):
        self.dst = ""
        self.spaces = 0
        self.indent_ = 4
    def isStartOfLine(self) -> bool: 
        return len(self.dst) == 0 or self.dst.endswith('\n')
    def pushSpaces(self):
        for _ in range(self.spaces):
            self.dst += " "
    
    def indent[R](self, f: Callable[[Self], R]) -> R: 
        self.spaces += self.indent_
        ret = f(self)
        self.spaces -= self.indent_
        ret
    def indentStart(self):
        self.spaces += self.indent_

    def indentEnd(self):
        self.spaces -= self.indent_
    def block(self, f: Callable[[Self], None]):
        if not self.isStartOfLine():
            self += " "

        self.write("{")
        self.indent(f)
        self.write("}\n")

    def __str__(self):
        return self.dst
        
    def __add__(self, other: str):
        self.write(other)
        return self

    def write(self, s: str):
        first = True
        should_indent = self.isStartOfLine()

        for line in s.splitlines():
            if not first:
                self.dst += "\n"
            first = False
            
            do_indent = should_indent and len(line) != 0 and not line.endswith('\n')
            
            if do_indent:
                self.pushSpaces()
            should_indent = True
            self.dst += line
        if s.endswith('\n'):
            self.dst += "\n"

class Expr:
    def fmt(self, fmt: Formatter):
        pass 

class Block(Expr):
   

    def __init__(self):
        self.before_ = None
        self.after_ = None 
        self.body = list()

    def line(self, line: str | Expr) -> Self:  
        self.body.append(line)
        return self 

    def pushBlock(self, block: Self) -> Self:
        self.body.append(block)
        return self 
    
    def after(self, after: str | Expr) -> Self: 
        self.after_ = after 
        return self 
    
    def before(self, before: str| Expr) -> Self: 
        self.before_ = before 
        return self 
    
    def fmt(self, fmt: Formatter):
        if self.before_:
            fmt.write(str(self.before_))

        if not fmt.isStartOfLine():
            fmt.write(" ")

        fmt.write("{\n")
        fmt.indentStart()

        for b in self.body: 
            if isinstance(b, Expr): 
                b.fmt(fmt)
            else: 
                fmt.write(f"{b}\n")
        fmt.indentEnd() 
        fmt.write("}")

        if self.after_:
            fmt.write(str(self.after_))
        fmt.write("\n")


class MatchArm(Expr):
    
    def default():
        this = MatchArm(None, None, Block())
        return this

    def __init__(self, pat, where, body):
        self.pat = pat 
        self.where = where 
        self.body = body

    def fmt(self, fmt: Formatter):
        pat = "_" if self.pat is None else self.pat 
        where = f" if {self.where}" if self.where else ""

        fmt.write(f"{pat}{where} => ")
        if isinstance(self.body, Expr):
            self.body.fmt(fmt)
        else: 
            fmt.write(str(self.body))


class Match(Expr):

    def __init__(self, expr):
        self.expr = expr 
        self.arms = list()


    def append(self, arm: MatchArm):
        self.arms.append(arm)

    def fmt(self, fmt: Formatter):
        fmt.write(f"match {self.expr} {{\n")
        fmt.indentStart()
        print(f"{len(self.arms)} arms for {self.expr}", )
        total = 0
        for arm in self.arms: 
            fmt.dst += f"{arm.pat}{arm.where if arm.where else ""} =>"
            arm.body.fmt(fmt)
            total += 1
            if total == 512:
                break
        fmt.indentEnd()
        fmt.write("}")

# A conditional statement. If there are conds produced it will emit `if` and conds otherwise it emits expression itself
class Cond(Expr):

    def __init__(self) -> None:
        # conds: list[(cond, or_and)]
        self.body_ = None 
        self.conds: list[(str, bool)] = list()
        
    def body(self, body: str | Block):
        if isinstance(body, str):
            b = Block()
            b.line(body)
            body = b 
        self.body_ = body 
    def and_(self, expr: str):
        self.conds.append((expr, True))
        return self 
    def or_(self, expr: str):
        self.conds.append((expr, False))

    def fmt(self, fmt: Formatter):
        """
        Emit cond as `if`.
        `len(conds) == 0` => ` {self.body.fmt(fmt)}
        `len(conds) == 1` => `if {self.conds[0][0]} { {self.body} }
        else => `if {self.conds[0]} "and" if self.conds[0][1] else "or" ... 
        """
        if len(self.conds) == 0:
            self.body_.fmt(fmt)
        elif len(self.conds) == 1:
            cond, _ = self.conds[0]
            fmt.write(f"if {cond} ")
            if isinstance(self.body_, Expr):
                self.body_.fmt(fmt)
            else: 
                fmt.write(str(self.body_))
        else:

            fmt.write("if ")
            for i, (cond, is_and) in enumerate(self.conds):
                fmt.write(cond)
                
                if i < len(self.conds) - 1:
                    is_and = self.conds[i+1][1]
                    fmt.write(" && " if is_and else " || ")
            fmt.write(" ")
            if isinstance(self.body_, Expr):
                self.body_.fmt(fmt)
            else: 
                fmt.write(str(self.body_))

class Type(Expr):
    def __init__(self, name):
        self.name = name
        self.generics: list[Self] = list()
    
    def generic(self, t):
        self.generics.append(t)

    def fmt(self, fmt: Formatter):
        fmt.write(self.name)
        if len(self.generics) != 0:
            fmt.write("<")
            for i, ty in enumerate(self.generics):
                if i != 0:
                    fmt.write(", ")
                fmt.write(ty)
            fmt.write(">")

class Field(Expr):
    def __init__(self, name, ty):
        self.name = name 
        self.ty = ty 

    def fmt(self, fmt: Formatter):
        fmt.write(f"{self.name}: {self.ty}")
class Function(Expr):
    def __init__(self, name) -> None:
        self.name = name 
        self.args: list[Field] = list()
        self.vis = None 
        self.arg_self = None 
        self._ret = None
        self._body = None 

    def pub(self):
        self.vis = "pub"
        return self 
    def priv(self):
        self.vis = None 
        return self 
    def self(self):
        self.arg_self = "&self"
        return self 
    def self_mut(self):
        self.arg_self = "&mut self"
        return self 
    def self_copy(self):
        self.arg_self = "self"
        return self 
    def ret(self, ty: Type):
        self._ret = ty
        return self  
    
    def arg(self, arg: Field):
        self.args.append(arg)
        return self 
    def body(self, block: Block):
        self._body = block 

    def line(self, line: str | Expr):
        if not self._body:
            self._body = Block()
        self._body.line(line)
    
    def fmt(self, fmt: Formatter):
        fmt.write(f"{(self.vis + " ") if self.vis else ""}fn {self.name}({self.arg_self if self.arg_self else ""}")
        if self.arg_self:
            fmt.write(",")
        for i, arg in enumerate(self.args):
            arg.fmt(fmt)
            if i != len(self.args) - 1:
                fmt.write(",")
        fmt.write(f") -> {self._ret if self._ret else ""}")
        self._body.fmt(fmt)

class Impl(Expr):
    def __init__(self, name) -> None:
        self.name = name 
        self.items = list()
    
    def append(self, item: Expr):
        self.items.append(item)

    def fmt(self, fmt: Formatter):
        fmt.write(f"impl {self.name} {{\n")
        fmt.indentStart()
        for i in self.items: 
            i.fmt(fmt)
        fmt.indentEnd()
        fmt.write("}")

class Const(Expr):
    def __init__(self, name, ty=None, init: str | Expr = ""):
        self.name = name
        self.ty = ty 
        self.init = init
        self.vis = None 

    def pub(self):
        self.vis = "pub"

    def pub_crate(self):
        self.vis = "pub(crate)"
    def pub_super(self):
        self.vis = "pub(super)"

    def fmt(self, fmt: Formatter):
        fmt.write(f"{self.vis if self.vis else ""} const {self.name}: ")
        self.ty.fmt(fmt)
        fmt.write(" = ")
        if isinstance(self.init, Expr):
            self.init.fmt(fmt)
        else:
            fmt.write(self.init)
        fmt.write(";\n")

class TopCode:
    items: list[Expr]

    def __init__(self) -> None:
        self.items = []

    def fmt(self, fmt):
        for item in self.items: 
            item.fmt(fmt)

class Trait(Expr):
    items: list[Expr]
    name: str 

    def __init__(self, name) -> None:   
        self.name = name
        self.items = []

    def fmt(self, fmt):
        fmt.write(f"pub trait {self.name} {{\n")
        fmt.indentStart()
        for item in self.items: 
            item.fmt(fmt)
        fmt.indentEnd()
        fmt.write("}\n")