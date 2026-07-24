# This file is part of asmkit.

"""Vendored-C++ source parsing and Rust emission helpers.

Shared machinery for the arch tablegen drivers (`tablegen_x86.py` /
`tablegen_a64.py`): loading files from the pinned asmjit tree, extracting
`// ${Key:Begin}` / `// ${Key:End}` blocks, parsing C++ table rows, enum
bodies, constant expressions and string literals, plus Rust-side emission
helpers (byte strings, name mapping, token-based normalized diff for
`--check` modes).

Stdlib only; deterministic output.
"""

import re
from pathlib import Path

ASMJIT_DB_DIR = Path(__file__).resolve().parent
REPO_ROOT = ASMJIT_DB_DIR.parent.parent
ASMJIT_ROOT = (REPO_ROOT / "meta" / "asmjit").resolve()
TEMPLATE_DIR = ASMJIT_DB_DIR / "templates"
OUT_DIR = ASMJIT_DB_DIR / "out"


class CxxError(ValueError):
    """Raised when a vendored C++ source cannot be parsed as expected."""


def fail(msg):
    raise CxxError(msg)


def check(cond, msg):
    if not cond:
        fail(msg)


def read_asmjit(rel):
    """Reads a file from the vendored asmjit tree (`asmjit/<rel>`)."""
    path = ASMJIT_ROOT / "asmjit" / rel
    if not path.is_file():
        fail(f"missing AsmJit source: {path}")
    return path.read_text(encoding="utf-8")


def read_db_json(rel):
    import json

    path = ASMJIT_ROOT / "db" / rel
    if not path.is_file():
        fail(f"missing AsmJit db file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def read_template(name):
    return (TEMPLATE_DIR / name).read_text(encoding="utf-8")


def extract_block(text, key):
    """Returns the text between `// ${key:Begin}` and `// ${key:End}`."""
    begin = re.search(r"//\s*\$\{" + re.escape(key) + r":Begin\}", text)
    end = re.search(r"//\s*\$\{" + re.escape(key) + r":End\}", text)
    check(begin is not None and end is not None, f"marker block ${{{key}}} not found")
    return text[begin.end():end.start()]


def strip_generated_banner(block):
    """Drops the `// --- Automatically generated ... ---` banner lines."""
    lines = [
        ln for ln in block.splitlines()
        if not re.match(r"\s*//\s*-{10,}", ln) and "Automatically generated, do not edit" not in ln
    ]
    return "\n".join(lines)


def split_line_rows(block):
    """Splits a marker block into (code, trailing_comment) rows, one per line.

    Full-line comments and blank lines are skipped. Table rows never contain
    `//` inside the code part (string-table declarations are parsed elsewhere).
    """
    rows = []
    for raw in block.splitlines():
        line = raw.strip()
        if not line or line.startswith("//") or line.startswith("#"):
            continue
        code, _, comment = line.partition("//")
        code = code.strip()
        if not code:
            continue
        rows.append((code, comment.strip()))
    return rows


def split_top_level(s, sep=","):
    """Splits `s` at top-level `sep`, respecting (), {}, [] and string literals."""
    parts = []
    depth = 0
    cur = []
    i = 0
    while i < len(s):
        c = s[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            check(depth >= 0, f"unbalanced brackets in {s!r}")
        elif c == '"':
            cur.append(c)
            i += 1
            while i < len(s) and s[i] != '"':
                if s[i] == "\\":
                    cur.append(s[i])
                    i += 1
                if i < len(s):
                    cur.append(s[i])
                    i += 1
            if i < len(s):
                cur.append(s[i])
            i += 1
            continue
        if c == sep and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
        i += 1
    check(depth == 0, f"unbalanced brackets in {s!r}")
    tail = "".join(cur)
    if tail.strip():
        parts.append(tail)
    return parts


def strip_trailing_comma(code):
    code = code.strip()
    if code.endswith(","):
        return code[:-1].strip(), True
    return code, False


def parse_macro_invocation(code):
    """`NAME(a, b, ...)` -> ("NAME", [arg, ...])."""
    code, _ = strip_trailing_comma(code)
    m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*\(", code)
    check(m is not None, f"not a macro invocation: {code!r}")
    check(code.endswith(")"), f"unbalanced macro invocation: {code!r}")
    inner = code[m.end():-1]
    return m.group(1), [a.strip() for a in split_top_level(inner)]


def parse_brace_row(code):
    """`{ a, b, ... }` -> [arg, ...]."""
    code, _ = strip_trailing_comma(code)
    check(code.startswith("{") and code.endswith("}"), f"not a brace row: {code!r}")
    return [a.strip() for a in split_top_level(code[1:-1])]


def strip_int_suffix(tok):
    m = re.match(r"^((?:0x[0-9A-Fa-f]+|0b[01]+|\d+))u$", tok.strip())
    return m.group(1) if m else tok.strip()


def parse_int(tok):
    tok = strip_int_suffix(tok)
    if tok.startswith(("0x", "0X")):
        return int(tok, 16)
    if tok.startswith("0b"):
        return int(tok, 2)
    check(re.fullmatch(r"\d+", tok) is not None, f"not an integer literal: {tok!r}")
    return int(tok)


_C_ESCAPES = {"n": 10, "t": 9, "r": 13, "0": 0, "\\": 92, '"': 34, "'": 39}


def decode_c_escapes(s):
    out = bytearray()
    i = 0
    while i < len(s):
        c = s[i]
        if c != "\\":
            out.extend(c.encode("latin-1"))
            i += 1
            continue
        n = s[i + 1]
        if n == "x":
            j = i + 2
            while j < len(s) and s[j] in "0123456789abcdefABCDEF":
                j += 1
            check(j > i + 2, f"bad \\x escape in {s!r}")
            out.append(int(s[i + 2:j], 16))
            i = j
        else:
            check(n in _C_ESCAPES, f"unknown escape \\{n} in {s!r}")
            out.append(_C_ESCAPES[n])
            i += 2
    return bytes(out)


def decode_c_string_table(decl_body):
    """Concatenates and decodes all adjacent C string literals in a declaration."""
    out = bytearray()
    for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', decl_body):
        out += decode_c_escapes(m.group(1))
    return bytes(out)


def rust_byte_string(data):
    """Rust `b"..."` literal: printable ASCII verbatim, others escaped."""
    parts = []
    for b in data:
        if b == 0x22:
            parts.append('\\"')
        elif b == 0x5C:
            parts.append("\\\\")
        elif 0x20 <= b <= 0x7E:
            parts.append(chr(b))
        else:
            parts.append(f"\\x{b:02X}")
    return 'b"' + "".join(parts) + '"'


_TOKEN_RE = re.compile(r"0x[0-9A-Fa-f]+u?|0b[01]+u?|\d+u?|[A-Za-z_][A-Za-z_0-9]*|<<|>>|[|&+()~]")


def eval_cpp_int(expr, consts):
    """Evaluates an integer constant expression with <<, >>, |, &, +, ~ and known names."""
    py = []
    covered = ""
    for m in _TOKEN_RE.finditer(expr):
        covered += m.group(0)
        t = m.group(0)
        if t in consts:
            py.append(str(consts[t]))
        elif t[0].isdigit():
            py.append(str(parse_int(t)))
        elif t in ("<<", ">>", "|", "&", "+", "(", ")", "~"):
            py.append(t)
        else:
            fail(f"unknown identifier {t!r} in constant expression {expr!r}")
    check(covered == re.sub(r"\s+", "", expr), f"cannot tokenize constant expression {expr!r}")
    return eval(" ".join(py), {"__builtins__": {}}, {})


def parse_enum_consts(body):
    """Evaluates `kName = expr,` entries of a C++ enum body, in order."""
    consts = {}
    next_value = 0
    for raw in body.splitlines():
        code, _, _ = raw.partition("//")
        code = code.strip()
        if not code:
            continue
        m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*(?:=\s*(.+?))?,?\s*$", code)
        check(m is not None, f"cannot parse enum entry: {raw!r}")
        if m.group(2) is not None:
            next_value = eval_cpp_int(m.group(2), consts)
        consts[m.group(1)] = next_value
        next_value += 1
    return consts


def enum_body(text, decl_pattern):
    """Extracts the `{ ... };` body of the enum matched by `decl_pattern`."""
    m = re.search(decl_pattern + r"[^{]*\{", text)
    check(m is not None, f"enum declaration not found: {decl_pattern}")
    start = m.end()
    depth = 1
    i = start
    while i < len(text) and depth:
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
        i += 1
    check(depth == 0, f"unbalanced enum body for {decl_pattern}")
    return text[start:i - 1]


def parse_cpp_enum_entries(body):
    """Entries of a C++ enum body: (name, explicit_value_expr, doc_comment).

    Attaches preceding `//!` lines and trailing `//!< ...` comments as docs.
    """
    entries = []
    pending_doc = ""
    for raw in body.splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("//!") and not line.startswith("//!<"):
            pending_doc = (pending_doc + " " + line[3:].strip()).strip()
            continue
        if line.startswith("//"):
            continue
        code, sep, comment = line.partition("//!<")
        code = code.strip().rstrip(",").strip()
        if not code or code in ("{", "};") or code.startswith("enum") or code.startswith("ASMJIT_"):
            continue
        m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*(?:=\s*(.+))?$", code)
        check(m is not None, f"cannot parse enum entry: {raw!r}")
        doc = comment.strip() if sep else pending_doc
        entries.append((m.group(1), m.group(2).strip() if m.group(2) else None, doc))
        pending_doc = ""
    check(entries, "enum body parsed to zero entries")
    return entries


def split_tables(block):
    """Splits a block into {variable_name: body} at `const ... name[...] = {` ... `};`."""
    decl = re.compile(r"^const\s+[\w:\s]*?(\w+)\s*\[[^\]]*\]\s*=\s*\{\s*$", re.M)
    matches = list(decl.finditer(block))
    tables = {}
    for idx, m in enumerate(matches):
        start = m.end()
        end_m = re.search(r"^\};", block[start:], re.M)
        check(end_m is not None, f"table {m.group(1)} has no closing '}};'")
        body = block[start:start + end_m.start()]
        check(m.group(1) not in tables, f"duplicate table {m.group(1)}")
        tables[m.group(1)] = body
    check(matches, "no table declarations found in block")
    return tables


def ordinal_of(comment):
    m = re.match(r"#(\d+)\b", comment)
    return int(m.group(1)) if m else None


def check_ordinals(comments, table, sparse=False):
    """Asserts `// #N` ordinals: continuous 0..n-1, or merely increasing when sparse."""
    ordinals = [ordinal_of(c) for c in comments]
    present = [o for o in ordinals if o is not None]
    if sparse:
        check(
            all(b > a for a, b in zip(present, present[1:])),
            f"{table}: sparse ordinals not strictly increasing: {present[:8]}...",
        )
        return
    check(
        len(present) == len(ordinals) and present == list(range(len(present))),
        f"{table}: broken ordinal sequence (got {len(present)} ordinals for {len(ordinals)} rows)",
    )


def split_camel(part):
    """Splits before an uppercase letter that (a) follows a lowercase letter or
    digit, or (b) follows an uppercase letter and is followed by a lowercase
    one (`XAcquire` -> `X|Acquire`, `LdNStN` -> `Ld|N|St|N`)."""
    words = []
    start = 0
    for i in range(1, len(part)):
        c = part[i]
        if not c.isupper():
            continue
        prev_lower = part[i - 1].islower() or part[i - 1].isdigit()
        boundary_upper = part[i - 1].isupper() and i + 1 < len(part) and part[i + 1].islower()
        if prev_lower or boundary_upper:
            words.append(part[start:i])
            start = i
    words.append(part[start:])
    return [w for w in words if w]


def screaming(name):
    """camelCase -> SCREAMING_SNAKE (splitting on `_` first)."""
    words = []
    for part in name.split("_"):
        words.extend(split_camel(part))
    return "_".join(w.upper() for w in words)


def strip_k(name):
    """`kIdAbs` -> `Abs`, `kAVX512_F` -> `AVX512_F`; digit-leading gets a `_` prefix."""
    check(name.startswith("k") and len(name) > 1, f"expected k-prefixed name, got {name!r}")
    out = name[1:]
    if out[0].isdigit():
        out = "_" + out
    return out


def map_flag_expr(expr, func, zero="0"):
    """Maps a `F(A) | F(B) | ...`-style expression part-wise, dropping zero terms."""
    parts = [p.strip() for p in split_top_level(expr, sep="|")]
    mapped = []
    for p in parts:
        if p in ("0", "0u"):
            continue
        m = func(p)
        if m != zero:
            mapped.append(m)
    return " | ".join(mapped) if mapped else zero


def emit_bitflags_value(expr):
    """Emits a bitflags const value: literals verbatim, `kXxx` refs -> `Self::XXX.bits()`."""
    def repl(m):
        return f"Self::{screaming(m.group(1))}.bits()"

    out = re.sub(r"\bk([A-Za-z_][A-Za-z_0-9]*)\b", repl, expr)
    out = re.sub(r"((?:0x[0-9A-Fa-f]+|0b[01]+|\d+))u\b", r"\1", out)
    return out


def parse_inst_id_block(text, strip_prefix):
    """Parses a `${InstId:Begin}` block.

    Returns (variants, aliases): variants are (rust_name, doc_text) where
    doc_text is the `//!< ...` comment with the marker stripped; aliases are
    (rust_name, rust_target) pairs from `kIdX = kIdY` lines (x86 only).
    `_kIdCount` terminates the variant list.
    """
    variants = []
    aliases = []
    seen_count = False
    for raw in text.splitlines():
        code, _, comment = raw.partition("//!<")
        code = code.strip()
        if not code or code.startswith("//"):
            continue
        if code.startswith("_kIdCount"):
            seen_count = True
            continue
        m = re.match(
            r"([A-Za-z_][A-Za-z_0-9]*)\s*(?:=\s*([A-Za-z_][A-Za-z_0-9]*|\d+)\s*)?,?\s*$", code
        )
        check(m is not None, f"cannot parse InstId line: {raw!r}")
        name, target = m.group(1), m.group(2)
        check(name.startswith(strip_prefix), f"unexpected InstId name {name!r}")
        if seen_count or (target is not None and not target.isdigit()):
            check(target is not None, f"alias without target: {raw!r}")
            check(target.startswith(strip_prefix), f"unexpected alias target {target!r}")
            aliases.append((name[len(strip_prefix):], target[len(strip_prefix):]))
        else:
            doc = comment.strip()
            check(doc, f"missing //!< doc comment: {raw!r}")
            variants.append((name[len(strip_prefix):], doc))
    check(seen_count, "_kIdCount not found in InstId block")
    check(variants and variants[0][0] == "None", "first InstId variant must be None")
    return variants, aliases


_RUST_TOKEN_RE = re.compile(
    r"/\*.*?\*/|//[^\n]*|b?\"(?:[^\"\\]|\\.)*\"|\w+|[^\w\s]", re.S
)


def tokenize_rust(text):
    """Token stream of a Rust file; comments are whitespace-normalized, layout ignored."""
    out = []
    for t in _RUST_TOKEN_RE.findall(text):
        if t.startswith("//") or t.startswith("/*"):
            t = re.sub(r"\s+", " ", t).strip()
        out.append(t)
    return out


def token_diff(a_text, b_text, max_reports=10):
    """Returns human-readable mismatch reports between two Rust sources."""
    a, b = tokenize_rust(a_text), tokenize_rust(b_text)
    n = min(len(a), len(b))
    first = None
    for i in range(n):
        if a[i] != b[i]:
            first = i
            break
    if first is None and len(a) == len(b):
        return []

    reports = []
    mismatches = [i for i in range(n) if a[i] != b[i]][:max_reports]
    reports.append(f"tokens differ at indexes {mismatches}")
    for i in mismatches[:3]:
        ctx_a = " ".join(a[max(0, i - 6):i + 4])
        ctx_b = " ".join(b[max(0, i - 6):i + 4])
        reports.append(f"  at token {i}:\n    A: ...{ctx_a}...\n    B: ...{ctx_b}...")
    if len(a) != len(b):
        reports.append(f"token count differs: A has {len(a)}, B has {len(b)}")
    return reports
