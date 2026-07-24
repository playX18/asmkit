#!/usr/bin/env python3
"""asmjit2rust.py — translate AsmJit's generated instruction-database tables to Rust.

Reads the vendored, pinned AsmJit tree at ``meta/asmjit`` (override with the
``ASMJIT_SRC`` environment variable) and regenerates:

  * ``src/aarch64/instdb.rs`` and ``meta/a64_rows.json``
      (``python3 meta/asmjit2rust.py aarch64``)
  * ``src/x86/instdb.rs`` and ``meta/x86_rows.json``
      (``python3 meta/asmjit2rust.py x86``)
  * ``all`` runs both.

The output is deterministic: same AsmJit input -> same bytes.

File layout:

  1. Shared machinery: source loading, ``${Key:Begin}``/``${Key:End}`` block
     extraction, C++ row parsing, constant-expression evaluation, C string
     literal decoding, Rust emission helpers, token-based normalized diff.
  2. AArch64 structural template (``A64_HEAD``/``A64_MID``/``A64_TABLE_NEW``):
     the hand-adapted port of the EncodingData POD structs and helper enums,
     spliced verbatim between the generated tables.
  3. AArch64 driver (``gen_aarch64``).
  4. X86 driver (``gen_x86``).
  5. CLI.

Self-checks fail loudly (``fail()``): unexpected row counts, broken ``// #N``
ordinals, unexpanded macro tokens, unparseable rows, unknown constants.

``aarch64 --check`` regenerates in memory and token-diffs the result against
``git show HEAD:src/aarch64/instdb.rs`` (whitespace/formatting-insensitive),
which must come back clean.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
ASMJIT_ROOT = Path(os.environ.get("ASMJIT_SRC", REPO_ROOT / "meta" / "asmjit")).resolve()


def fail(msg: str) -> None:
    raise SystemExit(f"asmjit2rust: error: {msg}")


def check(cond: bool, msg: str) -> None:
    if not cond:
        fail(msg)


def read_asmjit(rel: str) -> str:
    path = ASMJIT_ROOT / "asmjit" / rel
    if not path.is_file():
        fail(f"missing AsmJit source: {path}")
    return path.read_text(encoding="utf-8")



def extract_block(text: str, key: str) -> str:
    """Returns the text between `// ${key:Begin}` and `// ${key:End}`."""
    begin = re.search(r"//\s*\$\{" + re.escape(key) + r":Begin\}", text)
    end = re.search(r"//\s*\$\{" + re.escape(key) + r":End\}", text)
    check(begin is not None and end is not None, f"marker block ${{{key}}} not found")
    return text[begin.end() : end.start()]


def strip_generated_banner(block: str) -> str:
    """Drops the `// --- Automatically generated ... ---` banner lines."""
    lines = [
        ln
        for ln in block.splitlines()
        if not re.match(r"\s*//\s*-{10,}", ln) and "Automatically generated, do not edit" not in ln
    ]
    return "\n".join(lines)


def split_line_rows(block: str) -> list[tuple[str, str]]:
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


def split_top_level(s: str, sep: str = ",") -> list[str]:
    """Splits `s` at top-level `sep`, respecting (), {}, [] and string literals."""
    parts: list[str] = []
    depth = 0
    cur: list[str] = []
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


def strip_trailing_comma(code: str) -> tuple[str, bool]:
    code = code.strip()
    if code.endswith(","):
        return code[:-1].strip(), True
    return code, False


def parse_macro_invocation(code: str) -> tuple[str, list[str]]:
    """`NAME(a, b, ...)` -> ("NAME", [arg, ...])."""
    code, _ = strip_trailing_comma(code)
    m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*\(", code)
    check(m is not None, f"not a macro invocation: {code!r}")
    check(code.endswith(")"), f"unbalanced macro invocation: {code!r}")
    inner = code[m.end() : -1]
    return m.group(1), [a.strip() for a in split_top_level(inner)]


def parse_brace_row(code: str) -> list[str]:
    """`{ a, b, ... }` -> [arg, ...]."""
    code, _ = strip_trailing_comma(code)
    check(code.startswith("{") and code.endswith("}"), f"not a brace row: {code!r}")
    return [a.strip() for a in split_top_level(code[1:-1])]


def strip_int_suffix(tok: str) -> str:
    m = re.match(r"^((?:0x[0-9A-Fa-f]+|0b[01]+|\d+))u$", tok.strip())
    return m.group(1) if m else tok.strip()


def parse_int(tok: str) -> int:
    tok = strip_int_suffix(tok)
    if tok.startswith("0x") or tok.startswith("0X"):
        return int(tok, 16)
    if tok.startswith("0b"):
        return int(tok, 2)
    check(re.fullmatch(r"\d+", tok) is not None, f"not an integer literal: {tok!r}")
    return int(tok)


_C_ESCAPES = {"n": 10, "t": 9, "r": 13, "0": 0, "\\": 92, '"': 34, "'": 39}


def decode_c_escapes(s: str) -> bytes:
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
            out.append(int(s[i + 2 : j], 16))
            i = j
        else:
            check(n in _C_ESCAPES, f"unknown escape \\{n} in {s!r}")
            out.append(_C_ESCAPES[n])
            i += 2
    return bytes(out)


def decode_c_string_table(decl_body: str) -> bytes:
    """Concatenates and decodes all adjacent C string literals in a declaration."""
    out = bytearray()
    for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', decl_body):
        out += decode_c_escapes(m.group(1))
    return bytes(out)


def rust_byte_string(data: bytes) -> str:
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


_TOKEN_RE = re.compile(
    r"0x[0-9A-Fa-f]+u?|0b[01]+u?|\d+u?|[A-Za-z_][A-Za-z_0-9]*|<<|>>|[|&+()~]"
)


def eval_cpp_int(expr: str, consts: dict[str, int]) -> int:
    """Evaluates an integer constant expression with <<, >>, |, &, +, ~ and known names."""
    py: list[str] = []
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
    return eval(" ".join(py), {"__builtins__": {}}, {})  # noqa: S307 - ints and operators only


def parse_enum_consts(body: str) -> dict[str, int]:
    """Evaluates `kName = expr,` entries of a C++ enum body, in order."""
    consts: dict[str, int] = {}
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


def enum_body(text: str, decl_pattern: str) -> str:
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
    return text[start : i - 1]



def split_tables(block: str) -> dict[str, str]:
    """Splits a block into {variable_name: body} at `const ... name[...] = {` ... `};`."""
    decl = re.compile(r"^const\s+[\w:\s]*?(\w+)\s*\[[^\]]*\]\s*=\s*\{\s*$", re.M)
    matches = list(decl.finditer(block))
    tables: dict[str, str] = {}
    for idx, m in enumerate(matches):
        start = m.end()
        end_m = re.search(r"^\};", block[start:], re.M)
        check(end_m is not None, f"table {m.group(1)} has no closing '}};'")
        body = block[start : start + end_m.start()]
        check(m.group(1) not in tables, f"duplicate table {m.group(1)}")
        tables[m.group(1)] = body
    check(matches, "no table declarations found in block")
    return tables


def ordinal_of(comment: str) -> int | None:
    m = re.match(r"#(\d+)\b", comment)
    return int(m.group(1)) if m else None


def check_ordinals(comments: list[str], table: str, *, sparse: bool = False) -> None:
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



def split_camel(part: str) -> list[str]:
    """Splits before an uppercase letter that (a) follows a lowercase letter or
    digit, or (b) follows an uppercase letter and is followed by a lowercase
    one (`XAcquire` -> `X|Acquire`, `LdNStN` -> `Ld|N|St|N`).

    Glued acronym tails like `VVVe`/`VVVVx` still split (`VV|Ve`); call sites
    with different conventions use explicit overrides.
    """
    words: list[str] = []
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


def screaming(name: str) -> str:
    """camelCase -> SCREAMING_SNAKE (splitting on `_` first)."""
    words: list[str] = []
    for part in name.split("_"):
        words.extend(split_camel(part))
    return "_".join(w.upper() for w in words)


def strip_k(name: str) -> str:
    """`kIdAbs` -> `Abs`, `kAVX512_F` -> `AVX512_F`; digit-leading gets a `_` prefix."""
    check(name.startswith("k") and len(name) > 1, f"expected k-prefixed name, got {name!r}")
    out = name[1:]
    if out[0].isdigit():
        out = "_" + out
    return out



def substitute_macros(text: str, macro: str, func) -> str:
    """Replaces every `macro(arg)` (args contain no nested parens) with func(arg)."""
    pattern = re.compile(r"\b" + re.escape(macro) + r"\(\s*([^()]*?)\s*\)")
    while True:
        new = pattern.sub(lambda m: func(m.group(1).strip()), text)
        if new == text:
            return text
        text = new


def map_flag_expr(expr: str, func, zero: str = "0") -> str:
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


_RUST_TOKEN_RE = re.compile(
    r"/\*.*?\*/|//[^\n]*|b?\"(?:[^\"\\]|\\.)*\"|\w+|[^\w\s]", re.S
)


def tokenize_rust(text: str) -> list[str]:
    """Token stream of a Rust file; comments are whitespace-normalized, layout ignored."""
    out = []
    for t in _RUST_TOKEN_RE.findall(text):
        if t.startswith("//") or t.startswith("/*"):
            t = re.sub(r"\s+", " ", t).strip()
        out.append(t)
    return out


def token_diff(a_text: str, b_text: str, max_reports: int = 10) -> list[str]:
    """Returns human-readable mismatch reports between two Rust sources."""
    a, b = tokenize_rust(a_text), tokenize_rust(b_text)
    reports = []
    n = min(len(a), len(b))
    for i in range(n):
        if a[i] != b[i]:
            ctx_a = " ".join(a[max(0, i - 6) : i + 4])
            ctx_b = " ".join(b[max(0, i - 6) : i + 4])
            reports.append(
                f"first difference at token {i}:\n  A: ...{ctx_a}...\n  B: ...{ctx_b}..."
            )
            break
    if len(a) != len(b):
        reports.append(f"token count differs: A has {len(a)}, B has {len(b)}")
    if not reports:
        return []
    # Collect a few more mismatches after the first for context.
    mismatches = [i for i in range(n) if a[i] != b[i]][:max_reports]
    reports = [f"tokens differ at indexes {mismatches}"]
    for i in mismatches[:3]:
        ctx_a = " ".join(a[max(0, i - 6) : i + 4])
        ctx_b = " ".join(b[max(0, i - 6) : i + 4])
        reports.append(f"  at token {i}:\n    A: ...{ctx_a}...\n    B: ...{ctx_b}...")
    if len(a) != len(b):
        reports.append(f"token count differs: A has {len(a)}, B has {len(b)}")
    return reports


def git_show_head(rel: str) -> str:
    r = subprocess.run(
        ["git", "show", f"HEAD:{rel}"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    check(r.returncode == 0, f"git show HEAD:{rel} failed: {r.stderr.strip()}")
    return r.stdout


ZLIB_HEADER = """/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

"""
#
# AArch64 structural template (hand-adapted port of a64instdb{,_p}.h).
# Extracted from src/aarch64/instdb.rs; see gen_aarch64() for how the
# generated sections are spliced between these constants.
#

A64_HEAD = "/* Copyright (c) 2008-2024 The AsmJit Authors\n\n   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.\n\n   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:\n\n   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.\n   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.\n   This notice may not be removed or altered from any source distribution.\n*/\n\nuse super::assembler::*;\nuse super::operands::*;\nuse crate::core::operand::RegType;\n\npub const W: u32 = 0x1;\npub const X: u32 = 0x2;\npub const WX: u32 = W | X;\n\npub const ZR: u32 = Gp::ID_ZR as u32;\npub const SP: u32 = Gp::ID_SP as u32;\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum RwInfo {\n    R,\n    RW,\n    RX,\n    RRW,\n    RWX,\n    W,\n    WRW,\n    WRX,\n    WRRW,\n    WRRX,\n    WW,\n    X,\n    XRX,\n    XXRRX,\n\n    LDn,\n    STn,\n}\n\npub const RWI_R: u16 = RwInfo::R as u16;\npub const RWI_RW: u16 = RwInfo::RW as u16;\npub const RWI_RX: u16 = RwInfo::RX as u16;\npub const RWI_RRW: u16 = RwInfo::RRW as u16;\npub const RWI_RWX: u16 = RwInfo::RWX as u16;\npub const RWI_W: u16 = RwInfo::W as u16;\npub const RWI_WRW: u16 = RwInfo::WRW as u16;\npub const RWI_WRX: u16 = RwInfo::WRX as u16;\npub const RWI_WRRW: u16 = RwInfo::WRRW as u16;\npub const RWI_WRRX: u16 = RwInfo::WRRX as u16;\npub const RWI_WW: u16 = RwInfo::WW as u16;\npub const RWI_X: u16 = RwInfo::X as u16;\npub const RWI_XRX: u16 = RwInfo::XRX as u16;\npub const RWI_XXRRX: u16 = RwInfo::XXRRX as u16;\npub const RWI_LDN: u16 = RwInfo::LDn as u16;\npub const RWI_STN: u16 = RwInfo::STn as u16;\n\nimpl RwInfo {\n    pub const SPECIAL_START: Self = Self::LDn;\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum InstElementType {\n    None = VecElementType::None as isize,\n    B = VecElementType::B as isize,\n    H = VecElementType::H as isize,\n    S = VecElementType::S as isize,\n    D = VecElementType::D as isize,\n    _2H = VecElementType::H2 as isize,\n    _4B = VecElementType::B4 as isize,\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum GpType {\n    X,\n    W,\n    XSp,\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u32)]\npub enum OpSignature {\n    GpW = Reg::signature_of(RegType::Gp32).bits,\n    GpX = Reg::signature_of(RegType::Gp64).bits,\n    B = Reg::signature_of(RegType::Vec8).bits,\n    H = Reg::signature_of(RegType::Vec16).bits,\n    S = Reg::signature_of(RegType::Vec32).bits,\n    D = Reg::signature_of(RegType::Vec64).bits,\n    Q = Reg::signature_of(RegType::Vec128).bits,\n    V8B = Self::D as u32 | Vec::SIGNATURE_ELEMENT_B,\n    V4H = Self::D as u32 | Vec::SIGNATURE_ELEMENT_H,\n    V2S = Self::D as u32 | Vec::SIGNATURE_ELEMENT_S,\n\n    V16B = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_B,\n    V8H = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_H,\n    V4S = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_S,\n    V2D = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_D,\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum HFConv {\n    N,\n    None,\n    A,\n    B,\n    C,\n    D,\n    Count,\n}\n\npub enum VOType {\n    VB,\n    VBH,\n    VBH4S,\n    VBHS,\n    VBHSD2,\n    VHS,\n    VS,\n\n    VB8H4,\n    VB8H4S2,\n    VB8D1,\n    VH4S2,\n\n    VB16,\n    VB16H8,\n    VB16H8S4,\n    VB16D2,\n    VH8S4,\n    VS4,\n    VD2,\n\n    SVBHS,\n    SVB8H4S2,\n    SVHS,\n    VAny,\n    SVAny,\n\n    Count,\n}\n\n"

A64_MID = 'impl TryFrom<u8> for Encoding {\n    type Error = ();\n\n    fn try_from(value: u8) -> Result<Self, Self::Error> {\n        unsafe {\n            if value >= Self::Count as u8 {\n                Err(())\n            } else {\n                Ok(::core::mem::transmute(value))\n            }\n        }\n    }\n}\n\nconst HF_C: u32 = HFConv::C as u32;\n\nmacro_rules! impl_const_new_zero {\n    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {\n        impl $name {\n            pub const fn new($($field: $ty),*) -> Self {\n                Self {\n                    $($field),*\n                }\n            }\n        }\n    };\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseOp {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseOpX16 {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseOpImm {\n    pub opcode: u32,\n    pub imm_bits: u16,\n    pub imm_offset: u16,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseR {\n    pub opcode: u32,\n    pub reg_type: u32,\n    pub reg_hi_id: u32,\n    pub r_shift: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRR {\n    pub opcode: u32,\n    pub a_type: u32,\n    pub a_hi_id: u32,\n    pub a_shift: u32,\n    pub b_type: u32,\n    pub b_hi_id: u32,\n    pub b_shift: u32,\n    pub uniform: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRRR {\n    opcode: u32,\n    pub a_type: u32,\n    pub a_hi_id: u32,\n    pub b_type: u32,\n    pub b_hi_id: u32,\n    pub c_type: u32,\n    pub c_hi_id: u32,\n    pub uniform: u32,\n}\n\nimpl BaseRRR {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRRRR {\n    opcode: u32,\n    pub a_type: u32,\n    pub a_hi_id: u32,\n    pub b_type: u32,\n    pub b_hi_id: u32,\n    pub c_type: u32,\n    pub c_hi_id: u32,\n    pub d_type: u32,\n    pub d_hi_id: u32,\n    pub uniform: u32,\n}\n\nimpl BaseRRRR {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRRII {\n    opcode: u32,\n    pub a_type: u32,\n    pub a_hi_id: u32,\n    pub b_type: u32,\n    pub b_hi_id: u32,\n    pub a_imm_size: u32,\n    pub a_imm_discard_lsb: u32,\n    pub a_imm_offset: u32,\n    pub b_imm_size: u32,\n    pub b_imm_discard_lsb: u32,\n    pub b_imm_offset: u32,\n}\n\nimpl BaseRRII {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAtDcIcTlbi {\n    pub imm_verify_mask: u32,\n    pub imm_verify_data: u32,\n    pub mandatory_reg: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAdcSbc {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseMinMax {\n    pub register_op: u32,\n    pub immediate_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAddSub {\n    pub shifted_op: u32,\n    pub extended_op: u32,\n    pub immediate_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAdr {\n    opcode: u32,\n    pub offset_type: u8,\n}\n\nimpl BaseAdr {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseBfm {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseCmpCmn {\n    pub shifted_op: u32,\n    pub extended_op: u32,\n    pub immediate_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseExtend {\n    opcode: u32,\n    pub reg_type: u32,\n    pub u: u32,\n}\n\nimpl BaseExtend {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseLogical {\n    pub shifted_op: u32,\n    pub immediate_op: u32,\n    pub negate_imm: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseMvnNeg {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseShift {\n    pub register_op: u32,\n    pub immediate_op: u32,\n    pub ror: u32,\n}\n\nimpl BaseShift {\n    pub const fn register_op(&self) -> u32 {\n        self.register_op << 10\n    }\n\n    pub const fn immediate_op(&self) -> u32 {\n        self.immediate_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseTst {\n    pub shifted_op: u32,\n    pub immediate_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRMNoImm {\n    opcode: u32,\n    pub reg_type: u32,\n    pub reg_hi_id: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseRMNoImm {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRMSImm9 {\n    pub offset_op: u32,\n    pub pre_post_op: u32,\n    pub reg_type: u32,\n    pub reg_hi_id: u32,\n    pub x_offset: u32,\n    pub imm_shift: u32,\n}\n\nimpl BaseRMSImm9 {\n    pub const fn offset_op(&self) -> u32 {\n        self.offset_op << 10\n    }\n\n    pub const fn pre_post_op(&self) -> u32 {\n        self.pre_post_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseRMSImm10 {\n    opcode: u32,\n    pub reg_type: u32,\n    pub reg_hi_id: u32,\n    pub x_offset: u32,\n    pub imm_shift: u32,\n}\n\nimpl BaseRMSImm10 {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BasePrfm {\n    pub register_op: u32,\n    pub s_offset_op: u32,\n    pub u_offset_op: u32,\n    pub literal_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseLdSt {\n    pub u_offset_op: u32,\n    pub pre_post_op: u32,\n    pub register_op: u32,\n    pub literal_op: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n    pub u_offset_shift: u32,\n    pub u_alt_inst_id: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseLdpStp {\n    pub offset_op: u32,\n    pub pre_post_op: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n    pub offset_shift: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseStx {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseStx {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseLdxp {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseLdxp {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseStxp {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseStxp {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAtomicOp {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n    pub zr_reg: u32,\n}\n\nimpl BaseAtomicOp {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAtomicSt {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseAtomicSt {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct BaseAtomicCasp {\n    opcode: u32,\n    pub reg_type: u32,\n    pub x_offset: u32,\n}\n\nimpl BaseAtomicCasp {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\npub type BaseBranchReg = BaseOp;\npub type BaseBranchRel = BaseOp;\npub type BaseBranchCmp = BaseOp;\npub type BaseBranchTst = BaseOp;\npub type BaseExtract = BaseOp;\npub type BaseBfc = BaseOp;\npub type BaseBfi = BaseOp;\npub type BaseBfx = BaseOp;\npub type BaseCCmp = BaseOp;\npub type BaseCInc = BaseOp;\npub type BaseCSet = BaseOp;\npub type BaseCSel = BaseOp;\npub type BaseMovKNZ = BaseOp;\npub type BaseMull = BaseOp;\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct FSimdGeneric {\n    scalar_op: u32,\n    scalar_hf: u32,\n    vector_op: u32,\n    vector_hf: u32,\n}\n\nimpl FSimdGeneric {\n    pub const fn scalar_op(&self) -> u32 {\n        self.scalar_op << 10\n    }\n\n    pub const fn vector_op(&self) -> u32 {\n        self.vector_op << 10\n    }\n\n    pub const fn scalar_hf(&self) -> u32 {\n        self.scalar_hf\n    }\n\n    pub const fn vector_hf(&self) -> u32 {\n        self.vector_hf\n    }\n}\n\npub type FSimdVV = FSimdGeneric;\npub type FSimdVVV = FSimdGeneric;\npub type FSimdVVVV = FSimdGeneric;\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct FSimdSV {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct FSimdVVVe {\n    scalar_op: u32,\n    scalar_hf: u32,\n    vector_op: u32,\n    element_op: u32,\n}\n\nimpl FSimdVVVe {\n    pub const fn scalar_op(&self) -> u32 {\n        self.scalar_op << 10\n    }\n\n    pub const fn scalar_hf(&self) -> u32 {\n        self.scalar_hf\n    }\n\n    pub const fn vector_op(&self) -> u32 {\n        self.vector_op << 10\n    }\n\n    pub const fn vector_hf(&self) -> u32 {\n        HF_C\n    }\n\n    pub const fn element_scalar_op(&self) -> u32 {\n        (self.element_op << 10) | (0x5 << 28)\n    }\n\n    pub const fn element_vector_op(&self) -> u32 {\n        self.element_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcadd {\n    opcode: u32,\n}\n\nimpl SimdFcadd {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcmla {\n    regular_op: u32,\n    element_op: u32,\n}\n\nimpl SimdFcmla {\n    pub const fn regular_op(&self) -> u32 {\n        self.regular_op << 10\n    }\n\n    pub const fn element_op(&self) -> u32 {\n        self.element_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFccmpFccmpe {\n    opcode: u32,\n}\n\nimpl SimdFccmpFccmpe {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcm {\n    register_op: u32,\n    register_hf: u32,\n    zero_op: u32,\n}\n\nimpl SimdFcm {\n    pub const fn has_register_op(&self) -> bool {\n        self.register_op != 0\n    }\n\n    pub const fn has_zero_op(&self) -> bool {\n        self.zero_op != 0\n    }\n\n    pub const fn register_scalar_op(&self) -> u32 {\n        (self.register_op << 10) | (0x5 << 28)\n    }\n\n    pub const fn register_vector_op(&self) -> u32 {\n        self.register_op << 10\n    }\n\n    pub const fn register_scalar_hf(&self) -> u32 {\n        self.register_hf\n    }\n\n    pub const fn register_vector_hf(&self) -> u32 {\n        self.register_hf\n    }\n\n    pub const fn zero_scalar_op(&self) -> u32 {\n        (self.zero_op << 10) | (0x5 << 28)\n    }\n\n    pub const fn zero_vector_op(&self) -> u32 {\n        self.zero_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcmpFcmpe {\n    opcode: u32,\n}\n\nimpl SimdFcmpFcmpe {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcvtLN {\n    opcode: u32,\n    is_cvtxn: u32,\n    has_scalar: u32,\n}\n\nimpl SimdFcvtLN {\n    pub const fn scalar_op(&self) -> u32 {\n        (self.opcode << 10) | (0x5 << 28)\n    }\n\n    pub const fn vector_op(&self) -> u32 {\n        self.opcode << 10\n    }\n\n    pub const fn is_cvtxn(&self) -> u32 {\n        self.is_cvtxn\n    }\n\n    pub const fn has_scalar(&self) -> u32 {\n        self.has_scalar\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFcvtSV {\n    vector_int_op: u32,\n    vector_fp_op: u32,\n    general_op: u32,\n    is_float_to_int: u32,\n}\n\nimpl SimdFcvtSV {\n    pub const fn scalar_int_op(&self) -> u32 {\n        (self.vector_int_op << 10) | (0x5 << 28)\n    }\n\n    pub const fn vector_int_op(&self) -> u32 {\n        self.vector_int_op << 10\n    }\n\n    pub const fn scalar_fp_op(&self) -> u32 {\n        (self.vector_fp_op << 10) | (0x5 << 28)\n    }\n\n    pub const fn vector_fp_op(&self) -> u32 {\n        self.vector_fp_op << 10\n    }\n\n    pub const fn general_op(&self) -> u32 {\n        self.general_op << 10\n    }\n\n    pub const fn is_float_to_int(&self) -> u32 {\n        self.is_float_to_int\n    }\n\n    pub const fn is_fixed_point(&self) -> bool {\n        self.vector_fp_op != 0\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdFmlal {\n    pub vector_op: u32,\n    pub element_op: u32,\n    pub optional_q: u8,\n    pub ta: u8,\n    pub tb: u8,\n    pub t_element: u8,\n}\n\nimpl SimdFmlal {\n    pub const fn vector_op(&self) -> u32 {\n        self.vector_op << 10\n    }\n\n    pub const fn element_op(&self) -> u32 {\n        self.element_op << 10\n    }\n\n    pub const fn optional_q(&self) -> u32 {\n        self.optional_q as u32\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct FSimdPair {\n    pub scalar_op: u32,\n    pub vector_op: u32,\n}\n\nimpl FSimdPair {\n    pub const fn scalar_op(&self) -> u32 {\n        self.scalar_op << 10\n    }\n\n    pub const fn vector_op(&self) -> u32 {\n        self.vector_op << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVV {\n    opcode: u32,\n    pub vec_op_type: u32,\n}\n\nimpl ISimdVV {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVx {\n    opcode: u32,\n    pub op0_signature: u32,\n    pub op1_signature: u32,\n}\n\nimpl ISimdVVx {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdSV {\n    opcode: u32,\n    pub vec_op_type: u32,\n}\n\nimpl ISimdSV {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVV {\n    opcode: u32,\n    pub vec_op_type: u32,\n}\n\nimpl ISimdVVV {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVVx {\n    opcode: u32,\n    pub op0_signature: u32,\n    pub op1_signature: u32,\n    pub op2_signature: u32,\n}\n\nimpl ISimdVVVx {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdWWV {\n    opcode: u32,\n    pub vec_op_type: u32,\n}\n\nimpl ISimdWWV {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVVe {\n    pub regular_op: u32,\n    pub regular_vec_type: u32,\n    pub element_op: u32,\n    pub element_vec_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVVI {\n    opcode: u32,\n    pub vec_op_type: u32,\n    pub imm_size: u32,\n    pub imm_shift: u32,\n    pub imm64_has_one_bit_less: u32,\n}\n\nimpl ISimdVVVI {\n    pub const fn opcode(&self) -> u32 {\n        self.opcode << 10\n    }\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVVV {\n    pub opcode: u32,\n    pub vec_op_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdVVVVx {\n    pub opcode: u32,\n    pub op0_signature: u32,\n    pub op1_signature: u32,\n    pub op2_signature: u32,\n    pub op3_signature: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdBicOrr {\n    pub register_op: u32,\n    pub immediate_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdCmp {\n    pub register_op: u32,\n    pub zero_op: u32,\n    pub vec_op_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdDot {\n    pub vector_op: u32,\n    pub element_op: u32,\n    pub ta: u8,\n    pub tb: u8,\n    pub t_element: u8,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdMoviMvni {\n    pub opcode: u32,\n    pub inverted: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdLdSt {\n    pub u_offset_op: u32,\n    pub pre_post_op: u32,\n    pub register_op: u32,\n    pub literal_op: u32,\n    pub u_alt_inst_id: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdLdNStN {\n    pub single_op: u32,\n    pub multiple_op: u32,\n    pub n: u32,\n    pub replicate: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdLdpStp {\n    pub offset_op: u32,\n    pub pre_post_op: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdLdurStur {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct ISimdPair {\n    pub opcode2: u32,\n    pub opcode3: u32,\n    pub op_type3: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdShift {\n    pub register_op: u32,\n    pub immediate_op: u32,\n    pub inverted_imm: u32,\n    pub vec_op_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdShiftES {\n    pub opcode: u32,\n    pub vec_op_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdSm3tt {\n    pub opcode: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdSmovUmov {\n    pub opcode: u32,\n    pub vec_op_type: u32,\n    pub is_signed: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdSxtlUxtl {\n    pub opcode: u32,\n    pub vec_op_type: u32,\n}\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct SimdTblTbx {\n    pub opcode: u32,\n}\n\nimpl_const_new_zero!(BaseOp { opcode: u32 });\nimpl_const_new_zero!(BaseOpX16 { opcode: u32 });\nimpl_const_new_zero!(BaseOpImm {\n    opcode: u32,\n    imm_bits: u16,\n    imm_offset: u16\n});\nimpl_const_new_zero!(BaseR {\n    opcode: u32,\n    reg_type: u32,\n    reg_hi_id: u32,\n    r_shift: u32\n});\nimpl_const_new_zero!(BaseRR {\n    opcode: u32,\n    a_type: u32,\n    a_hi_id: u32,\n    a_shift: u32,\n    b_type: u32,\n    b_hi_id: u32,\n    b_shift: u32,\n    uniform: u32,\n});\nimpl_const_new_zero!(BaseRRR {\n    opcode: u32,\n    a_type: u32,\n    a_hi_id: u32,\n    b_type: u32,\n    b_hi_id: u32,\n    c_type: u32,\n    c_hi_id: u32,\n    uniform: u32,\n});\nimpl_const_new_zero!(BaseRRRR {\n    opcode: u32,\n    a_type: u32,\n    a_hi_id: u32,\n    b_type: u32,\n    b_hi_id: u32,\n    c_type: u32,\n    c_hi_id: u32,\n    d_type: u32,\n    d_hi_id: u32,\n    uniform: u32,\n});\nimpl_const_new_zero!(BaseRRII {\n    opcode: u32,\n    a_type: u32,\n    a_hi_id: u32,\n    b_type: u32,\n    b_hi_id: u32,\n    a_imm_size: u32,\n    a_imm_discard_lsb: u32,\n    a_imm_offset: u32,\n    b_imm_size: u32,\n    b_imm_discard_lsb: u32,\n    b_imm_offset: u32,\n});\nimpl_const_new_zero!(BaseAtDcIcTlbi {\n    imm_verify_mask: u32,\n    imm_verify_data: u32,\n    mandatory_reg: u32,\n});\nimpl_const_new_zero!(BaseAdcSbc { opcode: u32 });\nimpl_const_new_zero!(BaseMinMax {\n    register_op: u32,\n    immediate_op: u32\n});\nimpl_const_new_zero!(BaseAddSub {\n    shifted_op: u32,\n    extended_op: u32,\n    immediate_op: u32\n});\nimpl_const_new_zero!(BaseAdr {\n    opcode: u32,\n    offset_type: u8\n});\nimpl_const_new_zero!(BaseBfm { opcode: u32 });\nimpl_const_new_zero!(BaseCmpCmn {\n    shifted_op: u32,\n    extended_op: u32,\n    immediate_op: u32\n});\nimpl_const_new_zero!(BaseExtend {\n    opcode: u32,\n    reg_type: u32,\n    u: u32\n});\nimpl_const_new_zero!(BaseLogical {\n    shifted_op: u32,\n    immediate_op: u32,\n    negate_imm: u32\n});\nimpl_const_new_zero!(BaseMvnNeg { opcode: u32 });\nimpl_const_new_zero!(BaseShift {\n    register_op: u32,\n    immediate_op: u32,\n    ror: u32\n});\nimpl_const_new_zero!(BaseTst {\n    shifted_op: u32,\n    immediate_op: u32\n});\nimpl_const_new_zero!(BaseRMNoImm {\n    opcode: u32,\n    reg_type: u32,\n    reg_hi_id: u32,\n    x_offset: u32\n});\nimpl_const_new_zero!(BaseRMSImm9 {\n    offset_op: u32,\n    pre_post_op: u32,\n    reg_type: u32,\n    reg_hi_id: u32,\n    x_offset: u32,\n    imm_shift: u32,\n});\nimpl_const_new_zero!(BaseRMSImm10 {\n    opcode: u32,\n    reg_type: u32,\n    reg_hi_id: u32,\n    x_offset: u32,\n    imm_shift: u32,\n});\nimpl_const_new_zero!(BasePrfm {\n    register_op: u32,\n    s_offset_op: u32,\n    u_offset_op: u32,\n    literal_op: u32\n});\nimpl_const_new_zero!(BaseLdSt {\n    u_offset_op: u32,\n    pre_post_op: u32,\n    register_op: u32,\n    literal_op: u32,\n    reg_type: u32,\n    x_offset: u32,\n    u_offset_shift: u32,\n    u_alt_inst_id: u32,\n});\nimpl_const_new_zero!(BaseLdpStp {\n    offset_op: u32,\n    pre_post_op: u32,\n    reg_type: u32,\n    x_offset: u32,\n    offset_shift: u32,\n});\nimpl_const_new_zero!(BaseStx {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32\n});\nimpl_const_new_zero!(BaseLdxp {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32\n});\nimpl_const_new_zero!(BaseStxp {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32\n});\nimpl_const_new_zero!(BaseAtomicOp {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32,\n    zr_reg: u32\n});\nimpl_const_new_zero!(BaseAtomicSt {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32\n});\nimpl_const_new_zero!(BaseAtomicCasp {\n    opcode: u32,\n    reg_type: u32,\n    x_offset: u32\n});\n\nimpl_const_new_zero!(FSimdGeneric {\n    scalar_op: u32,\n    scalar_hf: u32,\n    vector_op: u32,\n    vector_hf: u32\n});\nimpl_const_new_zero!(FSimdSV { opcode: u32 });\nimpl_const_new_zero!(FSimdVVVe {\n    scalar_op: u32,\n    scalar_hf: u32,\n    vector_op: u32,\n    element_op: u32\n});\nimpl_const_new_zero!(SimdFcadd { opcode: u32 });\nimpl_const_new_zero!(SimdFcmla {\n    regular_op: u32,\n    element_op: u32\n});\nimpl_const_new_zero!(SimdFccmpFccmpe { opcode: u32 });\nimpl_const_new_zero!(SimdFcm {\n    register_op: u32,\n    register_hf: u32,\n    zero_op: u32\n});\nimpl_const_new_zero!(SimdFcmpFcmpe { opcode: u32 });\nimpl_const_new_zero!(SimdFcvtLN {\n    opcode: u32,\n    is_cvtxn: u32,\n    has_scalar: u32\n});\nimpl_const_new_zero!(SimdFcvtSV {\n    vector_int_op: u32,\n    vector_fp_op: u32,\n    general_op: u32,\n    is_float_to_int: u32,\n});\nimpl_const_new_zero!(SimdFmlal {\n    vector_op: u32,\n    element_op: u32,\n    optional_q: u8,\n    ta: u8,\n    tb: u8,\n    t_element: u8,\n});\nimpl_const_new_zero!(FSimdPair {\n    scalar_op: u32,\n    vector_op: u32\n});\n\nimpl_const_new_zero!(ISimdVV {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(ISimdVVx {\n    opcode: u32,\n    op0_signature: u32,\n    op1_signature: u32\n});\nimpl_const_new_zero!(ISimdSV {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(ISimdVVV {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(ISimdVVVx {\n    opcode: u32,\n    op0_signature: u32,\n    op1_signature: u32,\n    op2_signature: u32,\n});\nimpl_const_new_zero!(ISimdWWV {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(ISimdVVVe {\n    regular_op: u32,\n    regular_vec_type: u32,\n    element_op: u32,\n    element_vec_type: u32,\n});\nimpl_const_new_zero!(ISimdVVVI {\n    opcode: u32,\n    vec_op_type: u32,\n    imm_size: u32,\n    imm_shift: u32,\n    imm64_has_one_bit_less: u32,\n});\nimpl_const_new_zero!(ISimdVVVV {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(ISimdVVVVx {\n    opcode: u32,\n    op0_signature: u32,\n    op1_signature: u32,\n    op2_signature: u32,\n    op3_signature: u32,\n});\nimpl_const_new_zero!(SimdBicOrr {\n    register_op: u32,\n    immediate_op: u32\n});\nimpl_const_new_zero!(SimdCmp {\n    register_op: u32,\n    zero_op: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(SimdDot {\n    vector_op: u32,\n    element_op: u32,\n    ta: u8,\n    tb: u8,\n    t_element: u8\n});\nimpl_const_new_zero!(SimdMoviMvni {\n    opcode: u32,\n    inverted: u32\n});\nimpl_const_new_zero!(SimdLdSt {\n    u_offset_op: u32,\n    pre_post_op: u32,\n    register_op: u32,\n    literal_op: u32,\n    u_alt_inst_id: u32,\n});\nimpl_const_new_zero!(SimdLdNStN {\n    single_op: u32,\n    multiple_op: u32,\n    n: u32,\n    replicate: u32\n});\nimpl_const_new_zero!(SimdLdpStp {\n    offset_op: u32,\n    pre_post_op: u32\n});\nimpl_const_new_zero!(SimdLdurStur { opcode: u32 });\nimpl_const_new_zero!(ISimdPair {\n    opcode2: u32,\n    opcode3: u32,\n    op_type3: u32\n});\nimpl_const_new_zero!(SimdShift {\n    register_op: u32,\n    immediate_op: u32,\n    inverted_imm: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(SimdShiftES {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(SimdSm3tt { opcode: u32 });\nimpl_const_new_zero!(SimdSmovUmov {\n    opcode: u32,\n    vec_op_type: u32,\n    is_signed: u32\n});\nimpl_const_new_zero!(SimdSxtlUxtl {\n    opcode: u32,\n    vec_op_type: u32\n});\nimpl_const_new_zero!(SimdTblTbx { opcode: u32 });\n\n#[derive(Debug, Clone, Copy)]\npub struct InstInfo {\n    pub encoding: u8,\n    pub encoding_data_index: u8,\n    pub reserved: u16,\n    pub rw_info_index: u16,\n    pub flags: u16,\n}\n\nimpl InstInfo {\n    pub const fn new(\n        encoding: u8,\n        encoding_data_index: u8,\n        reserved: u16,\n        rw_info_index: u16,\n        flags: u16,\n    ) -> Self {\n        Self {\n            encoding,\n            encoding_data_index,\n            reserved,\n            rw_info_index,\n            flags,\n        }\n    }\n}\n\n#[derive(Debug, Clone, Copy)]\npub enum InstFlag {\n    Cond = 0x00000001,\n    Pair = 0x00000002,\n    Long = 0x00000004,\n    Narrow = 0x00000008,\n    VH0_15 = 0x00000010,\n    Consecutive = 0x00000080,\n}\n\nmacro_rules! F {\n    ($name: ident) => {\n        InstFlag::$name as u16\n    };\n}\n\nmacro_rules! INST {\n    ($id: ident, $opcode_encoding: ident, $opcode_data: expr, $rw_info_index: expr, $flags: expr, $opcode_data_index: expr) => {\n        InstInfo::new(\n            Encoding::$opcode_encoding as u8,\n            $opcode_data_index as u8,\n            0,\n            $rw_info_index,\n            $flags,\n        )\n    };\n}\n\nmacro_rules! TABLE {\n    ($name: ident = {\n        $(INST(\n            $id: ident,\n            $opcode_encoding: ident,\n            $opcode_data: expr,\n            $rw_info_index: expr,\n            $flags: expr,\n            $opcode_data_index: expr\n        ),)*\n    }) => {\n        pub static $name: &[InstInfo] = &[\n            $(\n                INST!(\n                    $id,\n                    $opcode_encoding,\n                    $opcode_data,\n                    $rw_info_index,\n                    $flags,\n                    $opcode_data_index)\n            ),*\n        ];\n    };\n}\n\n'

A64_TABLE_NEW = 'macro_rules! table_new {\n        ($ty:ident, { $({ $($e:expr),* $(,)? }),* $(,)? }) => {\n                [$( $ty::new($($e as u64 as _),*) ),*]\n        };\n}\n\nconst kW: u32 = W;\nconst kX: u32 = X;\nconst kWX: u32 = WX;\nconst kZR: u32 = ZR;\nconst kSP: u32 = SP;\n\nconst kHF_N: u32 = HFConv::N as u32;\nconst kHF_A: u32 = HFConv::A as u32;\nconst kHF_B: u32 = HFConv::B as u32;\nconst kHF_C: u32 = HFConv::C as u32;\n\nconst kET_B: u8 = InstElementType::B as u8;\nconst kET_H: u8 = InstElementType::H as u8;\nconst kET_S: u8 = InstElementType::S as u8;\nconst kET_2H: u8 = InstElementType::_2H as u8;\nconst kET_4B: u8 = InstElementType::_4B as u8;\n\nconst kOp_GpW: u32 = OpSignature::GpW as u32;\nconst kOp_H: u32 = OpSignature::H as u32;\nconst kOp_S: u32 = OpSignature::S as u32;\nconst kOp_D: u32 = OpSignature::D as u32;\nconst kOp_Q: u32 = OpSignature::Q as u32;\nconst kOp_V4H: u32 = OpSignature::V4H as u32;\nconst kOp_V8H: u32 = OpSignature::V8H as u32;\nconst kOp_V4S: u32 = OpSignature::V4S as u32;\nconst kOp_V2D: u32 = OpSignature::V2D as u32;\nconst kOp_V16B: u32 = OpSignature::V16B as u32;\n\nconst kVO_V_B: u32 = VOType::VB as u32;\nconst kVO_V_BH: u32 = VOType::VBH as u32;\nconst kVO_V_BH_4S: u32 = VOType::VBH4S as u32;\nconst kVO_V_BHS: u32 = VOType::VBHS as u32;\nconst kVO_V_BHS_D2: u32 = VOType::VBHSD2 as u32;\nconst kVO_V_HS: u32 = VOType::VHS as u32;\nconst kVO_V_S: u32 = VOType::VS as u32;\nconst kVO_V_B8H4S2: u32 = VOType::VB8H4S2 as u32;\nconst kVO_V_B8D1: u32 = VOType::VB8D1 as u32;\nconst kVO_V_H4S2: u32 = VOType::VH4S2 as u32;\nconst kVO_V_B16: u32 = VOType::VB16 as u32;\nconst kVO_V_B16H8S4: u32 = VOType::VB16H8S4 as u32;\nconst kVO_V_B16D2: u32 = VOType::VB16D2 as u32;\nconst kVO_V_H8S4: u32 = VOType::VH8S4 as u32;\nconst kVO_V_D2: u32 = VOType::VD2 as u32;\nconst kVO_SV_BHS: u32 = VOType::SVBHS as u32;\nconst kVO_SV_B8H4S2: u32 = VOType::SVB8H4S2 as u32;\nconst kVO_SV_HS: u32 = VOType::SVHS as u32;\nconst kVO_V_Any: u32 = VOType::VAny as u32;\nconst kVO_SV_Any: u32 = VOType::SVAny as u32;\n\n'


A64_INST_ID_COUNT = 776
A64_ENCODING_COUNT = 95  # AsmJit variants; the Rust enum appends `Count`.
A64_ENCODING_TABLE_COUNT = 83

# The hand port renamed a handful of encoding-data tables; everything else is
# `screaming()` of the AsmJit variable name.
A64_TABLE_NAME_OVERRIDES = {
    "baseRM_SImm9": "BASE_RM_SIMM9",
    "baseRM_SImm10": "BASE_RM_SIMM10",
    "fSimdVVVe": "F_SIMD_VVVE",
    "iSimdVVVe": "I_SIMD_VVVE",
    "iSimdVVx": "I_SIMD_VVX",
    "iSimdVVVx": "I_SIMD_VVVX",
    "iSimdVVVVx": "I_SIMD_VVVVX",
}

A64_INST_INFO_HEADER = (
    "// +------------------+---------------------+--------------------------------------------------------------------------------------+-----------+---------------------------+----+\n"
    "// | Instruction Id   | Encoding            | Opcode Data                                                                          | RW Info   | Instruction Flags         |DatX|\n"
    "// +------------------+---------------------+--------------------------------------------------------------------------------------+-----------+---------------------------+----+\n"
    "// ${InstInfo:Begin}\n"
)


# The hand port suffixed four single-field rows with `u32 as i32` (harmless
# no-op casts); reproduce them to stay token-identical.
A64_CAST_ROWS = {("baseBranchReg", 0), ("baseBranchReg", 1), ("baseBranchReg", 2), ("baseBranchRel", 2)}


def a64_table_name(cpp_name: str) -> str:
    if cpp_name in A64_TABLE_NAME_OVERRIDES:
        return A64_TABLE_NAME_OVERRIDES[cpp_name]
    return screaming(cpp_name)


def parse_inst_id_block(text: str, *, strip_prefix: str) -> tuple[list[tuple[str, str]], list[tuple[str, str]]]:
    """Parses a `${InstId:Begin}` block.

    Returns (variants, aliases): variants are (rust_name, doc_text) where
    doc_text is the `//!< ...` comment with the marker stripped; aliases are
    (rust_name, rust_target) pairs from `kIdX = kIdY` lines (x86 only).
    `_kIdCount` terminates the variant list.
    """
    variants: list[tuple[str, str]] = []
    aliases: list[tuple[str, str]] = []
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
            aliases.append((name[len(strip_prefix) :], target[len(strip_prefix) :]))
        else:
            doc = comment.strip()
            check(doc, f"missing //!< doc comment: {raw!r}")
            variants.append((name[len(strip_prefix) :], doc))
    check(seen_count, "_kIdCount not found in InstId block")
    check(variants and variants[0][0] == "None", "first InstId variant must be None")
    return variants, aliases


def gen_aarch64(check_only: bool = False) -> str:
    globals_h = read_asmjit("arm/a64globals.h")
    instdb_p_h = read_asmjit("arm/a64instdb_p.h")
    instdb_cpp = read_asmjit("arm/a64instdb.cpp")

    inst_ids, aliases = parse_inst_id_block(
        extract_block(globals_h, "InstId"), strip_prefix="kId"
    )
    check(not aliases, "a64 InstId block must not contain aliases")
    check(
        len(inst_ids) == A64_INST_ID_COUNT,
        f"a64: expected {A64_INST_ID_COUNT} InstId values, got {len(inst_ids)}",
    )

    enc_block = extract_block(instdb_p_h, "EncodingId")
    enc_variants: list[str] = []
    for raw in enc_block.splitlines():
        code = raw.strip()
        if not code or code.startswith("//") or code.startswith("enum") or code in ("{", "};"):
            continue
        m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*(?:=\s*\d+\s*)?,?\s*$", code)
        check(m is not None, f"cannot parse EncodingId line: {raw!r}")
        name = m.group(1)
        check(name.startswith("kEncoding"), f"unexpected EncodingId name {name!r}")
        enc_variants.append(name[len("kEncoding") :].replace("_", ""))
    check(
        len(enc_variants) == A64_ENCODING_COUNT,
        f"a64: expected {A64_ENCODING_COUNT} encodings, got {len(enc_variants)}",
    )
    check(enc_variants[0] == "None", "first encoding must be None")
    enc_set = set(enc_variants)

    fwd_block = extract_block(instdb_p_h, "EncodingDataForward")
    fwd_decls: list[tuple[str, str, int]] = []  # (cpp_type, cpp_name, size)
    for raw in fwd_block.splitlines():
        m = re.match(r"\s*extern const ([A-Za-z_0-9]+) ([A-Za-z_0-9]+)\[(\d+)\];", raw)
        if m:
            fwd_decls.append((m.group(1), m.group(2), int(m.group(3))))
    check(
        len(fwd_decls) == A64_ENCODING_TABLE_COUNT,
        f"a64: expected {A64_ENCODING_TABLE_COUNT} forward declarations, got {len(fwd_decls)}",
    )

    inst_block = extract_block(instdb_cpp, "InstInfo")
    inst_rows: list[dict] = []
    for code, comment in split_line_rows(strip_generated_banner(inst_block)):
        name, args = parse_macro_invocation(code)
        check(name == "INST", f"a64: unexpected row macro {name!r}")
        check(len(args) == 6, f"a64: INST row with {len(args)} args: {code!r}")
        inst_rows.append(
            {
                "id": args[0],
                "encoding": args[1],
                "opcode_data": args[2],
                "rw_info_index": args[3],
                "flags": args[4],
                "opcode_data_index": args[5],
                "comment": comment,
            }
        )
    check(
        len(inst_rows) == A64_INST_ID_COUNT,
        f"a64: expected {A64_INST_ID_COUNT} INST rows, got {len(inst_rows)}",
    )
    check_ordinals([r["comment"] for r in inst_rows], "a64 _inst_info_table")
    for i, row in enumerate(inst_rows):
        check(
            row["id"] == inst_ids[i][0],
            f"a64: INST row #{i} id {row['id']!r} does not match InstId {inst_ids[i][0]!r}",
        )
        check(
            row["encoding"].replace("_", "") in enc_set,
            f"a64: INST row #{i} uses unknown encoding {row['encoding']!r}",
        )
        check(
            re.fullmatch(r"kRWI_[A-Za-z_0-9]+|0", row["rw_info_index"]) is not None,
            f"a64: INST row #{i} bad rw_info_index {row['rw_info_index']!r}",
        )
        check(
            re.fullmatch(r"0|F\([A-Za-z_0-9]+\)(\s*\|\s*F\([A-Za-z_0-9]+\))*", row["flags"])
            is not None,
            f"a64: INST row #{i} bad flags {row['flags']!r}",
        )
        check(
            re.fullmatch(r"\d+", row["opcode_data_index"]) is not None,
            f"a64: INST row #{i} bad opcode_data_index {row['opcode_data_index']!r}",
        )

    enc_data_block = extract_block(instdb_cpp, "EncodingData")
    cpp_tables = split_tables(strip_generated_banner(enc_data_block))
    check(
        len(cpp_tables) == A64_ENCODING_TABLE_COUNT,
        f"a64: expected {A64_ENCODING_TABLE_COUNT} encoding-data tables, got {len(cpp_tables)}",
    )
    fwd_names = {name for _, name, _ in fwd_decls}
    check(
        set(cpp_tables) == fwd_names,
        f"a64: table/forward-declaration mismatch: {set(cpp_tables) ^ fwd_names}",
    )

    parsed_tables: dict[str, list[tuple[list[str], str]]] = {}
    for cpp_type, cpp_name, size in fwd_decls:
        rows = []
        for code, comment in split_line_rows(cpp_tables[cpp_name]):
            rows.append((parse_brace_row(code), comment))
        check(
            len(rows) == size,
            f"a64: table {cpp_name} has {len(rows)} rows, declared {size}",
        )
        parsed_tables[cpp_name] = rows

    name_block = extract_block(instdb_cpp, "NameData")
    string_m = re.search(
        r"const char InstDB::_inst_name_string_table\[\] =\s*(.*?);", name_block, re.S
    )
    check(string_m is not None, "a64: _inst_name_string_table not found")
    name_string = decode_c_string_table(string_m.group(1))

    index_tables = split_tables(strip_generated_banner(name_block))
    check("_inst_name_index_table" in index_tables, "a64: _inst_name_index_table missing")
    name_index_rows = split_line_rows(index_tables["_inst_name_index_table"])
    check(
        len(name_index_rows) == A64_INST_ID_COUNT,
        f"a64: expected {A64_INST_ID_COUNT} name index rows, got {len(name_index_rows)}",
    )

    json_rows = [
        {
            "id": r["id"],
            "encoding": r["encoding"],
            "opcode0": None,
            "opcode1": None,
            "main_opcode_index": None,
            "alt_opcode_index": None,
            "common_info_index": None,
            "additional_info_index": None,
            "rw_info_index": r["rw_info_index"],
            "flags": r["flags"],
            "opcode_data_index": int(r["opcode_data_index"]),
            "opcode_data": r["opcode_data"],
        }
        for r in inst_rows
    ]
    if not check_only:
        write_json(
            REPO_ROOT / "meta" / "a64_rows.json",
            {"arch": "aarch64", "source": "arm/a64instdb.cpp ${InstInfo}", "rows": json_rows},
        )

    def map_data_arg(arg: str) -> str:
        arg = re.sub(r"\bOffsetType::kAArch64_ADR\b", "OffsetType::Adr as u8", arg)
        arg = re.sub(r"\bOffsetType::kAArch64_ADRP\b", "OffsetType::Adrp as u8", arg)
        arg = re.sub(r"\bInst::kId([A-Za-z_0-9]+)\b", r"InstId::\1", arg)
        return arg

    out: list[str] = [A64_HEAD]

    # Encoding enum.
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum Encoding {\n")
    out.append("    None = 0,\n")
    for name in enc_variants[1:]:
        out.append(f"    {name},\n")
    out.append("\n    Count,\n}\n\n")

    out.append(A64_MID)

    # INST_INFO_TABLE.
    out.append("TABLE!(INST_INFO_TABLE\n\n  = {\n")
    out.append(A64_INST_INFO_HEADER)
    for i, row in enumerate(inst_rows):
        encoding = row["encoding"].replace("_", "")
        # The INST! macro discards opcode_data; the hand port still maps
        # Inst::kIdXxx -> InstId::Xxx inside it (OffsetType stays verbatim).
        opcode_data = re.sub(r"\bInst::kId([A-Za-z_0-9]+)\b", r"InstId::\1", row["opcode_data"])
        rw = re.sub(r"\bkRWI_([A-Za-z_0-9]+)", lambda m: "RWI_" + m.group(1).upper(), row["rw_info_index"])
        flags = map_flag_expr(row["flags"], lambda f: f"F!({f[2:-1]})" if f.startswith("F(") else f)
        out.append(
            f"INST({row['id']}, {encoding}, {opcode_data}, {rw}, {flags}, {row['opcode_data_index']}), // #{i}\n"
        )
    out.append("\n  });\n\n")

    # First encoding-data table (plain array form in the hand port).
    first_type, first_name, _ = fwd_decls[0]
    check(first_name == "baseAddSub", "a64: first table must be baseAddSub")
    rust_first_type = first_type.replace("_", "")
    out.append(f"pub const {a64_table_name(first_name)}: [{rust_first_type}; {len(parsed_tables[first_name])}] = [\n")
    for args, comment in parsed_tables[first_name]:
        mapped = ", ".join(map_data_arg(a) for a in args)
        suffix = f" // {comment}" if comment else ""
        out.append(f"    {rust_first_type}::new({mapped}),{suffix}\n")
    out.append("];\n\n")

    out.append(A64_TABLE_NEW)

    # Remaining encoding-data tables via table_new!.
    for cpp_type, cpp_name, _ in fwd_decls[1:]:
        rust_type = cpp_type.replace("_", "")
        rows = parsed_tables[cpp_name]
        out.append(
            f"pub const {a64_table_name(cpp_name)}: [{rust_type}; {len(rows)}] = table_new!({rust_type}, {{\n"
        )
        for idx, (args, comment) in enumerate(rows):
            if (cpp_name, idx) in A64_CAST_ROWS:
                check(len(args) == 1 and args[0].startswith("0b"),
                      f"a64: unexpected cast-row shape in {cpp_name} row {idx}")
                args = [args[0] + "u32 as i32"]
            mapped = ", ".join(map_data_arg(a) for a in args)
            comma = "," if idx + 1 < len(rows) else " "
            suffix = f" // {comment}" if comment else ""
            out.append(f"    {{ {mapped} }}{comma}{suffix}\n")
        out.append("});\n\n")

    # Name tables (the hand port omits _inst_name_index spans).
    out.append(f"pub static INST_NAME_STRING_TABLE: &[u8] = {rust_byte_string(name_string)};\n")
    out.append("#[rustfmt::skip]\n")
    out.append("pub static INST_NAME_INDEX_TABLE: &[u32] = &[\n")
    for idx, (code, comment) in enumerate(name_index_rows):
        value, had_comma = strip_trailing_comma(code)
        comma = "," if idx + 1 < len(name_index_rows) else ""
        suffix = f" // {comment}" if comment else ""
        out.append(f"    {value}{comma}{suffix}\n")
    out.append("];\n\n")

    # InstId enum.
    out.append("#[rustfmt::skip]\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[allow(non_camel_case_types)]\n")
    out.append("#[repr(u32)]\n")
    out.append("pub enum InstId {\n")
    out.append(f"    None = 0, // {inst_ids[0][1]}\n")
    for name, doc in inst_ids[1:]:
        out.append(f"    {name}, // {doc}\n")
    out.append("    _Count\n}\n")

    result = "".join(out)

    # Unexpanded-token self-checks.
    for forbidden in ("kRWI_", "Inst::kId", "uint32_t(", "uint16_t(", "uint8_t("):
        check(forbidden not in result, f"a64: unexpanded token {forbidden!r} left in output")
    check(
        re.search(r"\bF\(", result) is None,
        "a64: unexpanded F() flag macro left in output",
    )

    if check_only:
        head = git_show_head("src/aarch64/instdb.rs")
        diffs = token_diff(head, result)
        if diffs:
            print("aarch64 normalized diff vs git HEAD:", file=sys.stderr)
            for d in diffs:
                print(d, file=sys.stderr)
            fail("aarch64 output differs from the hand port")
        print("aarch64: normalized diff vs git HEAD is clean")
    else:
        write_file(REPO_ROOT / "src" / "aarch64" / "instdb.rs", result)
    return result


X86_INST_ID_COUNT = 1648
X86_ALIAS_COUNT = 44
X86_ENCODING_COUNT = 162  # Includes kEncodingNone and kEncodingCount.
X86_MAIN_OPCODE_COUNT = 234
X86_ALT_OPCODE_COUNT = 135
X86_COMMON_INFO_COUNT = 449
X86_ADDITIONAL_INFO_COUNT = 199
X86_RW_FLAGS_COUNT = 35
X86_INST_FLAGS_COUNT = 2
X86_SIGNATURE_COUNT = 602
X86_OP_SIGNATURE_COUNT = 176
X86_RW_INFO_A_COUNT = 137
X86_RW_INFO_B_COUNT = 174
X86_RW_INFO_OP_COUNT = 88
X86_RW_INFO_RM_COUNT = 69

X86_DERIVED_NOTE = """// [AsmKit] This file is a derived work: it was translated from AsmJit's generated
// instruction-database tables (asmjit/x86/x86globals.h, asmjit/x86/x86instdb.h,
// asmjit/x86/x86instdb_p.h, asmjit/x86/x86instdb.cpp, asmjit/core/cpuinfo.h)
// by meta/asmjit2rust.py. Do not edit manually; fix the translator instead.

"""

# asmkit's OpRwFlags (src/core/rwinfo.rs) names that `screaming()` gets wrong.
OP_RW_FLAGS_OVERRIDES = {"ZExt": "ZEXT"}

CPU_RW_FLAGS_X86 = {
    "CF", "OF", "SF", "ZF", "AF", "PF", "DF", "IF", "AC", "C0", "C1", "C2", "C3"
}


def parse_cpp_enum_entries(body: str) -> list[tuple[str, str | None, str]]:
    """Entries of a C++ enum body: (name, explicit_value_expr, doc_comment).

    Attaches preceding `//!` lines and trailing `//!< ...` comments as docs.
    """
    entries: list[tuple[str, str | None, str]] = []
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


def emit_bitflags_value(expr: str) -> str:
    """Emits a bitflags const value: literals verbatim, `kXxx` refs -> `Self::XXX.bits()`."""

    def repl(m: re.Match) -> str:
        return f"Self::{screaming(m.group(1))}.bits()"

    out = re.sub(r"\bk([A-Za-z_][A-Za-z_0-9]*)\b", repl, expr)
    out = re.sub(r"((?:0x[0-9A-Fa-f]+|0b[01]+|\d+))u\b", r"\1", out)
    return out


class X86Db:
    """Parsed AsmJit x86 instruction database."""

    def __init__(self) -> None:
        globals_h = read_asmjit("x86/x86globals.h")
        instdb_h = read_asmjit("x86/x86instdb.h")
        instdb_p_h = read_asmjit("x86/x86instdb_p.h")
        opcode_p_h = read_asmjit("x86/x86opcode_p.h")
        cpuinfo_h = read_asmjit("core/cpuinfo.h")
        self.inst_cpp = read_asmjit("x86/x86instdb.cpp")

        self.inst_ids, self.aliases = parse_inst_id_block(
            extract_block(globals_h, "InstId"), strip_prefix="kId"
        )
        check(
            len(self.inst_ids) == X86_INST_ID_COUNT,
            f"x86: expected {X86_INST_ID_COUNT} InstId values, got {len(self.inst_ids)}",
        )
        check(
            len(self.aliases) == X86_ALIAS_COUNT,
            f"x86: expected {X86_ALIAS_COUNT} aliases, got {len(self.aliases)}",
        )

        self.mode_entries = parse_cpp_enum_entries(enum_body(instdb_h, r"enum class Mode : uint8_t"))
        check([e[0] for e in self.mode_entries] == ["kNone", "kX86", "kX64", "kAny"],
              "x86: unexpected Mode enum")
        self.op_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class OpFlags : uint64_t")
        )
        self.inst_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class InstFlags : uint32_t")
        )
        self.avx512_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class Avx512Flags : uint32_t")
        )
        self.inst_flags_names = {e[0][1:] for e in self.inst_flags_entries if e[0] != "kNone"}
        self.avx512_flags_names = {e[0][1:] for e in self.avx512_flags_entries if e[0] not in ("kNone", "k_")}
        self.op_flags_names = {e[0][1:] for e in self.op_flags_entries if e[0] != "kNone"}

        self.encoding_entries = parse_cpp_enum_entries(
            enum_body(instdb_p_h, r"enum EncodingId : uint32_t")
        )
        check(
            len(self.encoding_entries) == X86_ENCODING_COUNT,
            f"x86: expected {X86_ENCODING_COUNT} encodings, got {len(self.encoding_entries)}",
        )
        self.enc_names = [e[0][len("kEncoding"):] for e in self.encoding_entries]
        check(self.enc_names[0] == "None" and self.enc_names[-1] == "Count",
              "x86: EncodingId must start with None and end with Count")
        self.enc_set = set(self.enc_names)

        self.rw_info_category_entries = parse_cpp_enum_entries(
            enum_body(instdb_p_h, r"enum Category : uint8_t")
        )
        self.rw_info_rm_body = enum_body(instdb_p_h, r"struct RWInfoRm")
        self.rw_info_rm_category_entries = parse_cpp_enum_entries(
            enum_body(self.rw_info_rm_body, r"enum Category : uint8_t")
        )
        self.rw_info_rm_flags_entries = parse_cpp_enum_entries(
            enum_body(self.rw_info_rm_body, r"enum Flags : uint8_t")
        )

        self.opcode_consts = parse_enum_consts(enum_body(opcode_p_h, r"enum Bits : uint32_t"))

        x86_features_body = enum_body(cpuinfo_h, r"struct X86 : public Data")
        feature_entries = parse_cpp_enum_entries(
            enum_body(x86_features_body, r"enum Id : uint8_t")
        )
        self.cpu_features: list[tuple[str, str]] = []  # (cpp_name, rust_variant)
        self.cpu_feature_docs: dict[str, str] = {}
        for name, _, doc in feature_entries:
            if name == "kMaxValue":
                break
            variant = strip_k(name)
            self.cpu_features.append((name[1:], variant))
            self.cpu_feature_docs[name[1:]] = doc
        self.cpu_feature_map = dict(self.cpu_features)

    def opcode_const(self, name: str) -> int:
        check(name in self.opcode_consts, f"x86: unknown Opcode constant {name!r}")
        return self.opcode_consts[name]

    def eval_opcode(self, code: str) -> int:
        """Evaluates an O(...)/V(...)/E(...)/O_FPU(...)/0 opcode expression."""
        code = code.strip()
        if code == "0":
            return 0
        name, args = parse_macro_invocation(code)
        if name == "O":
            check(len(args) == 8, f"x86: O() with {len(args)} args: {code!r}")
            prefix, opcode, modo, ll, w, evexw, n, modrm = args
            return (
                self.opcode_const("k" + prefix)
                | int(opcode, 16)
                | self.opcode_const("kModO_" + modo)
                | self.opcode_const("kLL_" + ll)
                | self.opcode_const("kW_" + w)
                | self.opcode_const("kEvex_W_" + evexw)
                | self.opcode_const("kCDSHL_" + n)
                | self.opcode_const("kModRM_" + modrm)
            )
        if name in ("V", "E"):
            check(len(args) == 8, f"x86: {name}() with {len(args)} args: {code!r}")
            prefix, opcode, modo, ll, w, evexw, n, tt = args
            value = (
                self.opcode_const("k" + prefix)
                | int(opcode, 16)
                | self.opcode_const("kModO_" + modo)
                | self.opcode_const("kLL_" + ll)
                | self.opcode_const("kW_" + w)
                | self.opcode_const("kEvex_W_" + evexw)
                | self.opcode_const("kCDSHL_" + n)
                | self.opcode_const("kCDTT_" + tt)
            )
            if name == "E":
                value |= self.opcode_const("kMM_ForceEvex")
            return value
        if name == "O_FPU":
            check(len(args) == 3, f"x86: O_FPU() with {len(args)} args: {code!r}")
            prefix, opcode, modo = args
            opval = int(opcode, 16)
            return (
                self.opcode_const("kFPU_" + prefix)
                | (opval & 0xFF)
                | ((opval >> 8) << self.opcode_const("kFPU_2B_Shift"))
                | self.opcode_const("kModO_" + modo)
            )
        fail(f"x86: cannot evaluate opcode expression {code!r}")

    def map_inst_flags(self, expr: str) -> str:
        def one(part: str) -> str:
            m = re.fullmatch(r"F\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad InstFlags term {part!r}")
            check(m.group(1) in self.inst_flags_names, f"x86: unknown InstFlag {m.group(1)!r}")
            return f"InstFlags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_avx512_flags(self, expr: str) -> str:
        def one(part: str) -> str:
            m = re.fullmatch(r"X\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad Avx512Flags term {part!r}")
            if m.group(1) == "_":
                return "0"
            check(m.group(1) in self.avx512_flags_names, f"x86: unknown Avx512Flag {m.group(1)!r}")
            return f"Avx512Flags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_cpu_rw_flags(self, expr: str) -> str:
        def one(part: str) -> str:
            m = re.fullmatch(r"FLAG\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad CpuRWFlags term {part!r}")
            check(m.group(1) in CPU_RW_FLAGS_X86, f"x86: unknown CpuRWFlags {m.group(1)!r}")
            return f"CpuRwFlags::X86_{m.group(1)}.bits()"
        return map_flag_expr(expr, one)

    def map_op_rw_flags(self, expr: str) -> str:
        def one(part: str) -> str:
            m = re.fullmatch(r"OpRWFlags::k([A-Za-z_0-9]+)", part)
            check(m is not None, f"x86: bad OpRWFlags term {part!r}")
            name = m.group(1)
            mapped = OP_RW_FLAGS_OVERRIDES.get(name, screaming(name))
            return f"OpRwFlags::{mapped}.bits()"
        return map_flag_expr(expr, one)

    def map_op_flags(self, expr: str) -> str:
        """Maps `F(RegGpbLo) | F(...)` (OpSignature table) to OpFlags consts."""
        def one(part: str) -> str:
            m = re.fullmatch(r"F\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad OpFlags term {part!r}")
            check(m.group(1) in self.op_flags_names, f"x86: unknown OpFlag {m.group(1)!r}")
            return f"OpFlags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_ext_features(self, expr: str) -> list[str]:
        """`{ EXT(A), EXT(B) }` -> [`CpuFeature::A as u8`, ...]."""
        out = []
        for part in parse_brace_row(expr):
            if part == "0":
                continue
            m = re.fullmatch(r"EXT\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad EXT term {part!r}")
            check(m.group(1) in self.cpu_feature_map, f"x86: unknown CpuFeature {m.group(1)!r}")
            out.append(f"CpuFeature::{self.cpu_feature_map[m.group(1)]} as u8")
        return out


def x86_parse_inst_rows(db: X86Db) -> list[dict]:
    block = extract_block(db.inst_cpp, "InstInfo")
    rows = []
    for code, comment in split_line_rows(strip_generated_banner(block)):
        name, args = parse_macro_invocation(code)
        check(name == "INST", f"x86: unexpected row macro {name!r}")
        check(len(args) == 8, f"x86: INST row with {len(args)} args: {code!r}")
        rows.append(
            {
                "id": args[0],
                "encoding": args[1],
                "opcode0": args[2],
                "opcode1": args[3],
                "main_opcode_index": args[4],
                "alt_opcode_index": args[5],
                "common_info_index": args[6],
                "additional_info_index": args[7],
                "comment": comment,
            }
        )
    check(
        len(rows) == X86_INST_ID_COUNT,
        f"x86: expected {X86_INST_ID_COUNT} INST rows, got {len(rows)}",
    )
    check_ordinals([r["comment"] for r in rows], "x86 _inst_info_table")
    for i, row in enumerate(rows):
        check(
            row["id"] == db.inst_ids[i][0],
            f"x86: INST row #{i} id {row['id']!r} does not match InstId {db.inst_ids[i][0]!r}",
        )
        check(
            row["encoding"] in db.enc_set,
            f"x86: INST row #{i} uses unknown encoding {row['encoding']!r}",
        )
        for key in ("main_opcode_index", "alt_opcode_index", "common_info_index", "additional_info_index"):
            check(
                re.fullmatch(r"\d+", row[key]) is not None,
                f"x86: INST row #{i} bad {key} {row[key]!r}",
            )
        row["main_opcode_value"] = db.eval_opcode(row["opcode0"]) & 0xFF
    return rows


def x86_parse_opcode_table(db: X86Db, block_key: str, table: str, expected: int) -> list[tuple[int, str]]:
    block = extract_block(db.inst_cpp, block_key)
    tables = split_tables(strip_generated_banner(block))
    check(table in tables, f"x86: {table} missing in ${{{block_key}}}")
    rows = []
    for code, comment in split_line_rows(tables[table]):
        code_stripped, _ = strip_trailing_comma(code)
        rows.append((db.eval_opcode(code_stripped), comment))
    check(len(rows) == expected, f"x86: {table} has {len(rows)} rows, expected {expected}")
    check_ordinals([c for _, c in rows], f"x86 {table}")
    return rows


def emit_x86(db: X86Db) -> str:
    out: list[str] = [ZLIB_HEADER, X86_DERIVED_NOTE]
    out.append("use bitflags::bitflags;\n\n")
    out.append(
        "use crate::core::rwinfo::{CpuRwFlags, InstControlFlow, InstRwFlags, InstSameRegHint, OpRwFlags};\n\n"
    )

    out.append("/// X86 CPU feature identifiers (port of AsmJit's `CpuFeatures::X86`).\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[allow(non_camel_case_types)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum CpuFeature {\n")
    for cpp_name, variant in db.cpu_features:
        doc = db.cpu_feature_docs[cpp_name]
        if doc:
            out.append(f"    /// {doc}\n")
        out.append(f"    {variant},\n")
    out.append("}\n\n")
    out.append(f"pub const CPU_FEATURE_COUNT: usize = {len(db.cpu_features)};\n")
    names = ", ".join(f'"{variant}"' for _, variant in db.cpu_features)
    out.append(f"pub static CPU_FEATURE_NAMES: &[&str] = &[{names}];\n")
    defaults = [variant for _, variant in db.cpu_features if variant != "None"]
    default_values = ", ".join(f"CpuFeature::{variant}" for variant in defaults)
    out.append(
        f"pub const DEFAULT_X86_FEATURES: &[CpuFeature] = &[{default_values}];\n\n"
    )
    out.append("impl CpuFeature {\n")
    for old, new in (("AVX512F", "AVX512_F"), ("AVX512BW", "AVX512_BW"), ("AVX512CD", "AVX512_CD"), ("AVX512DQ", "AVX512_DQ"), ("AVX512VL", "AVX512_VL")):
        out.append(f"    pub const {old}: Self = Self::{new};\n")
    out.append("}\n\n")

    out.append("/// X86 instruction id.\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[repr(u32)]\n")
    out.append("pub enum InstId {\n")
    out.append(f"    /// {db.inst_ids[0][1]}\n    None = 0,\n")
    for name, doc in db.inst_ids[1:]:
        out.append(f"    /// {doc}\n    {name},\n")
    out.append("    _Count\n}\n\n")

    out.append("/// Instruction aliases (AsmJit `Inst::kIdX = kIdY`).\n")
    out.append("impl InstId {\n")
    for name, target in db.aliases:
        out.append(f"    pub const {screaming(name)}: InstId = InstId::{target};\n")
    out.append("}\n\n")

    out.append("/// Describes which operation mode is supported by an instruction.\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum Mode {\n")
    for name, value, doc in db.mode_entries:
        if doc:
            out.append(f"    /// {doc}\n")
        out.append(f"    {strip_k(name)} = {emit_bitflags_value(value)},\n")
    out.append("}\n\n")

    out.append("// Operand signature flags used by [`OpSignature`].\n")
    out.append("bitflags! {\n")
    out.append("    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]\n")
    out.append("    pub struct OpFlags: u64 {\n")
    for name, value, doc in db.op_flags_entries:
        if doc:
            out.append(f"        /// {doc}\n")
        check(value is not None, f"x86: OpFlags entry {name} has no value")
        out.append(f"        const {screaming(strip_k(name))} = {emit_bitflags_value(value)};\n")
    out.append("    }\n}\n\n")

    out.append(X86_STRUCTS)

    out.append("// Instruction flags.\n")
    out.append("bitflags! {\n")
    out.append("    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]\n")
    out.append("    pub struct InstFlags: u32 {\n")
    for name, value, doc in db.inst_flags_entries:
        if doc:
            out.append(f"        /// {doc}\n")
        check(value is not None, f"x86: InstFlags entry {name} has no value")
        out.append(f"        const {screaming(strip_k(name))} = {emit_bitflags_value(value)};\n")
    out.append("    }\n}\n\n")

    out.append("// AVX-512 flags.\n")
    out.append("bitflags! {\n")
    out.append("    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]\n")
    out.append("    pub struct Avx512Flags: u32 {\n")
    for name, value, doc in db.avx512_flags_entries:
        if name == "k_":
            continue
        if doc:
            out.append(f"        /// {doc}\n")
        check(value is not None, f"x86: Avx512Flags entry {name} has no value")
        out.append(f"        const {screaming(strip_k(name))} = {emit_bitflags_value(value)};\n")
    out.append("    }\n}\n\n")

    out.append("/// Instruction encoding (X86|X86_64).\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[allow(non_camel_case_types)]\n")
    out.append("#[repr(u32)]\n")
    out.append("pub enum Encoding {\n")
    for i, (name, _, doc) in enumerate(db.encoding_entries):
        rust_name = name[len("kEncoding"):]
        if doc:
            out.append(f"    /// {doc}\n")
        if i == 0:
            out.append(f"    {rust_name} = 0,\n")
        else:
            out.append(f"    {rust_name},\n")
    out.append("}\n\n")

    out.append(emit_x86_rw_structs(db))
    out.append(emit_x86_tables(db))
    result = "".join(out)

    # Unexpanded-token self-checks (on code only: docs legitimately name the
    # C++ types being ported).
    code_only = re.sub(r"/\*.*?\*/|//[^\n]*|b?\"(?:[^\"\\]|\\.)*\"", "", result, flags=re.S)
    banned = [
        r"\bF\(", r"\bX\(", r"\bEXT\(", r"\bFLAG\(", r"\bO\(", r"\bV\(", r"\bE\(",
        r"\bO_FPU\(", r"\bROW\(", r"\bCONTROL_FLOW\(", r"\bSAME_REG_HINT\(",
        r"\buint(8|16|32|64)_t\(", r"\bOpcode::", r"\bInstDB::", r"\bCpuFeatures::",
        r"\bCpuRWFlags::", r"\bOpRWFlags::", r"\bInstRWFlags\(", r"\bInst::kId",
        r"\bkEncoding[A-Za-z_0-9]*",
    ]
    for pattern in banned:
        m = re.search(pattern, code_only)
        if m is not None:
            fail(f"x86: unexpanded token {m.group(0)!r} left in output (pattern {pattern!r})")
    return result


# Fixed Rust ports of the POD structs from x86instdb.h (kept in AsmJit order).
X86_STRUCTS = """/// Operand signature: all possible operand combinations, memory size
/// information, and a fixed register id (port of AsmJit's `InstDB::OpSignature`).
#[derive(Debug, Clone, Copy, Default)]
pub struct OpSignature {
    pub flags: u64,
    pub reg_mask: u8,
}

impl OpSignature {
    pub const fn new(flags: u64, reg_mask: u8) -> Self {
        Self { flags, reg_mask }
    }
}

/// Instruction signature: a sequence of operand combinations and other
/// metadata defining a single instruction (port of AsmJit's `InstDB::InstSignature`).
#[derive(Debug, Clone, Copy, Default)]
pub struct InstSignature {
    pub op_count: u8,
    pub mode: u8,
    pub implicit_op_count: u8,
    pub reserved: u8,
    pub op_signature_indexes: [u8; 6],
}

impl InstSignature {
    pub const fn new(
        op_count: u8,
        mode: u8,
        implicit_op_count: u8,
        reserved: u8,
        op_signature_indexes: [u8; 6],
    ) -> Self {
        Self {
            op_count,
            mode,
            implicit_op_count,
            reserved,
            op_signature_indexes,
        }
    }
}

"""

# Fixed Rust ports of CommonInfo/InstInfo/AdditionalInfo and the RW-info PODs.
X86_STRUCTS_2 = """/// Aggregated information shared across one or more instructions
/// (port of AsmJit's `InstDB::CommonInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct CommonInfo {
    pub flags: u32,
    pub avx512_flags: u32,
    pub signature_index: u32,
    pub signature_count: u32,
    pub control_flow: InstControlFlow,
    pub same_reg_hint: InstSameRegHint,
}

impl CommonInfo {
    pub const fn new(
        flags: u32,
        avx512_flags: u32,
        signature_index: u32,
        signature_count: u32,
        control_flow: InstControlFlow,
        same_reg_hint: InstSameRegHint,
    ) -> Self {
        Self {
            flags,
            avx512_flags,
            signature_index,
            signature_count,
            control_flow,
            same_reg_hint,
        }
    }

    pub const fn has_flag(&self, flag: InstFlags) -> bool {
        self.flags & flag.bits() != 0
    }

    pub const fn has_avx512_flag(&self, flag: Avx512Flags) -> bool {
        self.avx512_flags & flag.bits() != 0
    }
}

/// Instruction information (port of AsmJit's `InstDB::InstInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct InstInfo {
    pub reserved: u32,
    pub common_info_index: u32,
    pub additional_info_index: u32,
    pub encoding: u8,
    pub main_opcode_value: u8,
    pub main_opcode_index: u8,
    pub alt_opcode_index: u8,
}

impl InstInfo {
    pub const fn new(
        common_info_index: u32,
        additional_info_index: u32,
        encoding: u8,
        main_opcode_value: u8,
        main_opcode_index: u8,
        alt_opcode_index: u8,
    ) -> Self {
        Self {
            reserved: 0,
            common_info_index,
            additional_info_index,
            encoding,
            main_opcode_value,
            main_opcode_index,
            alt_opcode_index,
        }
    }
}

/// Additional information table entry: CPU extensions required to execute an
/// instruction plus RW flags (port of AsmJit's `InstDB::AdditionalInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct AdditionalInfo {
    pub inst_flags_index: u8,
    pub rw_flags_index: u8,
    pub features: [u8; 6],
}

impl AdditionalInfo {
    pub const fn new(inst_flags_index: u8, rw_flags_index: u8, features: [u8; 6]) -> Self {
        Self {
            inst_flags_index,
            rw_flags_index,
            features,
        }
    }
}

/// Read/write information of an instruction (port of AsmJit's `InstDB::RWInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfo {
    pub category: RwInfoCategory,
    pub rm_info: u8,
    pub op_info_index: [u8; 6],
}

impl RwInfo {
    pub const fn new(category: RwInfoCategory, rm_info: u8, op_info_index: [u8; 6]) -> Self {
        Self {
            category,
            rm_info,
            op_info_index,
        }
    }
}

/// Read/write information of a single operand (port of AsmJit's `InstDB::RWInfoOp`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfoOp {
    pub r_byte_mask: u64,
    pub w_byte_mask: u64,
    pub phys_id: u8,
    pub consecutive_lead_count: u8,
    pub flags: OpRwFlags,
}

impl RwInfoOp {
    pub const fn new(
        r_byte_mask: u64,
        w_byte_mask: u64,
        phys_id: u8,
        consecutive_lead_count: u8,
        flags: u32,
    ) -> Self {
        Self {
            r_byte_mask,
            w_byte_mask,
            phys_id,
            consecutive_lead_count,
            flags: OpRwFlags::from_bits_retain(flags),
        }
    }
}

/// R/M information, used to replace a register operand by a memory operand
/// reliably (port of AsmJit's `InstDB::RWInfoRm`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfoRm {
    pub category: RwInfoRmCategory,
    pub rm_ops_mask: u8,
    pub fixed_size: u8,
    pub flags: u8,
    pub rm_feature: u8,
}

impl RwInfoRm {
    pub const fn new(
        category: RwInfoRmCategory,
        rm_ops_mask: u8,
        fixed_size: u8,
        flags: u8,
        rm_feature: u8,
    ) -> Self {
        Self {
            category,
            rm_ops_mask,
            fixed_size,
            flags,
            rm_feature,
        }
    }
}

/// CPU/FPU flags read/written information (port of AsmJit's `InstDB::RWFlagsInfoTable`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwFlagsInfo {
    pub read_flags: u32,
    pub write_flags: u32,
}

impl RwFlagsInfo {
    pub const fn new(read_flags: u32, write_flags: u32) -> Self {
        Self {
            read_flags,
            write_flags,
        }
    }
}

"""


def emit_x86_rw_structs(db: X86Db) -> str:
    out: list[str] = [X86_STRUCTS_2]

    # RwInfoCategory (from InstDB::RWInfo::Category).
    variants = []
    for name, _, doc in db.rw_info_category_entries:
        check(name.startswith("kCategory"), f"x86: unexpected RWInfo::Category name {name!r}")
        variants.append((name[len("kCategory"):], doc))
    check(len(variants) == 17, f"x86: expected 17 RWInfo categories, got {len(variants)}")
    out.append("/// Category of [`RwInfo`] (port of AsmJit's `InstDB::RWInfo::Category`).\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\n")
    out.append("#[allow(non_camel_case_types)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum RwInfoCategory {\n")
    for i, (variant, doc) in enumerate(variants):
        if doc:
            out.append(f"    /// {doc}\n")
        if i == 0:
            out.append(f"    #[default]\n    {variant} = 0,\n")
        else:
            out.append(f"    {variant},\n")
    out.append("}\n\n")
    # Move RwInfo/RwInfoOp next to their categories by emitting them here is
    # not possible (they live in X86_STRUCTS_2); the file-level order is fine.

    # RwInfoRmCategory (from InstDB::RWInfoRm::Category).
    rm_variants = []
    for name, _, doc in db.rw_info_rm_category_entries:
        check(name.startswith("kCategory"), f"x86: unexpected RWInfoRm::Category name {name!r}")
        rm_variants.append((name[len("kCategory"):], doc))
    check(len(rm_variants) == 6, f"x86: expected 6 RWInfoRm categories, got {len(rm_variants)}")
    out.append("/// Category of [`RwInfoRm`] (port of AsmJit's `InstDB::RWInfoRm::Category`).\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum RwInfoRmCategory {\n")
    for i, (variant, doc) in enumerate(rm_variants):
        if doc:
            out.append(f"    /// {doc}\n")
        if i == 0:
            out.append(f"    #[default]\n    {variant} = 0,\n")
        else:
            out.append(f"    {variant},\n")
    out.append("}\n\n")

    # RwInfoRmFlags (from InstDB::RWInfoRm::Flags).
    out.append("// Flags of [`RwInfoRm`] (port of AsmJit's `InstDB::RWInfoRm::Flags`).\n")
    out.append("bitflags! {\n")
    out.append("    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]\n")
    out.append("    pub struct RwInfoRmFlags: u8 {\n")
    count = 0
    for name, value, doc in db.rw_info_rm_flags_entries:
        check(name.startswith("kFlag"), f"x86: unexpected RWInfoRm::Flags name {name!r}")
        if doc:
            out.append(f"        /// {doc}\n")
        check(value is not None, f"x86: RWInfoRm::Flags entry {name} has no value")
        out.append(f"        const {screaming(name[len('kFlag'):])} = {emit_bitflags_value(value)};\n")
        count += 1
    check(count == 4, f"x86: expected 4 RWInfoRm flags, got {count}")
    out.append("        /// No flags.\n        const NONE = 0;\n")
    out.append("    }\n}\n\n")
    return "".join(out)


def emit_u8_table(name: str, values: list[int], doc: str) -> str:
    out = [doc]
    out.append(f"pub static {name}: &[u8] = &[\n")
    for i in range(0, len(values), 24):
        out.append("    " + " ".join(f"{v}," for v in values[i : i + 24]) + "\n")
    out.append("];\n\n")
    return "".join(out)


def emit_x86_tables(db: X86Db) -> str:
    out: list[str] = []

    inst_rows = x86_parse_inst_rows(db)
    out.append("/// Instruction information table, indexed by [`InstId`].\n")
    out.append("pub static INST_INFO_TABLE: &[InstInfo] = &[\n")
    for i, row in enumerate(inst_rows):
        out.append(
            f"    InstInfo::new({row['common_info_index']}, {row['additional_info_index']}, "
            f"Encoding::{row['encoding']} as u8, 0x{row['main_opcode_value']:02X}, "
            f"{row['main_opcode_index']}, {row['alt_opcode_index']}), // #{i}\n"
        )
    out.append("];\n\n")

    for block_key, table, rust_name, expected in (
        ("MainOpcodeTable", "main_opcode_table", "MAIN_OPCODE_TABLE", X86_MAIN_OPCODE_COUNT),
        ("AltOpcodeTable", "alt_opcode_table", "ALT_OPCODE_TABLE", X86_ALT_OPCODE_COUNT),
    ):
        rows = x86_parse_opcode_table(db, block_key, table, expected)
        out.append(f"pub static {rust_name}: &[u32] = &[\n")
        for value, comment in rows:
            suffix = f" // {comment}" if comment else ""
            out.append(f"    0x{value:08X},{suffix}\n")
        out.append("];\n\n")

    common_block = extract_block(db.inst_cpp, "InstCommonTable")
    common_tables = split_tables(strip_generated_banner(common_block))
    check("_inst_common_info_table" in common_tables, "x86: _inst_common_info_table missing")
    common_rows = split_line_rows(common_tables["_inst_common_info_table"])
    check(
        len(common_rows) == X86_COMMON_INFO_COUNT,
        f"x86: expected {X86_COMMON_INFO_COUNT} CommonInfo rows, got {len(common_rows)}",
    )
    check_ordinals([c for _, c in common_rows], "x86 _inst_common_info_table")
    out.append("/// Aggregated instruction information, indexed by [`InstInfo::common_info_index`].\n")
    out.append("pub static INST_COMMON_INFO_TABLE: &[CommonInfo] = &[\n")
    for code, comment in common_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 6, f"x86: CommonInfo row with {len(fields)} fields: {code!r}")
        flags, xflags, sig_index, sig_count, control_flow, same_reg_hint = fields
        m = re.fullmatch(r"CONTROL_FLOW\(([A-Za-z]+)\)", control_flow)
        check(m is not None and m.group(1) in ("Regular", "Jump", "Branch", "Call", "Return"),
              f"x86: bad CONTROL_FLOW term {control_flow!r}")
        cf = f"InstControlFlow::{m.group(1)}"
        m = re.fullmatch(r"SAME_REG_HINT\(([A-Za-z]+)\)", same_reg_hint)
        check(m is not None and m.group(1) in ("None", "RO", "WO"),
              f"x86: bad SAME_REG_HINT term {same_reg_hint!r}")
        srh = f"InstSameRegHint::{m.group(1)}"
        out.append(
            f"    CommonInfo::new({db.map_inst_flags(flags)}, {db.map_avx512_flags(xflags)}, "
            f"{sig_index}, {sig_count}, {cf}, {srh}), // {comment}\n"
        )
    out.append("];\n\n")

    add_block = extract_block(db.inst_cpp, "AdditionalInfoTable")
    add_tables = split_tables(strip_generated_banner(add_block))

    add_rows = split_line_rows(add_tables["additional_info_table"])
    check(
        len(add_rows) == X86_ADDITIONAL_INFO_COUNT,
        f"x86: expected {X86_ADDITIONAL_INFO_COUNT} AdditionalInfo rows, got {len(add_rows)}",
    )
    check_ordinals([c for _, c in add_rows], "x86 additional_info_table")
    out.append("/// CPU feature requirements and RW flag indexes, indexed by [`InstInfo::additional_info_index`].\n")
    out.append("pub static ADDITIONAL_INFO_TABLE: &[AdditionalInfo] = &[\n")
    for code, comment in add_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 3, f"x86: AdditionalInfo row with {len(fields)} fields: {code!r}")
        features = db.map_ext_features(fields[2])
        check(len(features) <= 6, f"x86: too many features in {code!r}")
        features += ["0"] * (6 - len(features))
        out.append(
            f"    AdditionalInfo::new({fields[0]}, {fields[1]}, [{', '.join(features)}]), // {comment}\n"
        )
    out.append("];\n\n")

    rwf_rows = split_line_rows(add_tables["rw_flags_info_table"])
    check(
        len(rwf_rows) == X86_RW_FLAGS_COUNT,
        f"x86: expected {X86_RW_FLAGS_COUNT} RWFlagsInfo rows, got {len(rwf_rows)}",
    )
    check_ordinals([c for _, c in rwf_rows], "x86 rw_flags_info_table")
    out.append("/// CPU flags read/written, indexed by [`AdditionalInfo::rw_flags_index`].\n")
    out.append("pub static RW_FLAGS_INFO_TABLE: &[RwFlagsInfo] = &[\n")
    for code, comment in rwf_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 2, f"x86: RWFlagsInfo row with {len(fields)} fields: {code!r}")
        out.append(
            f"    RwFlagsInfo::new({db.map_cpu_rw_flags(fields[0])}, {db.map_cpu_rw_flags(fields[1])}), // {comment}\n"
        )
    out.append("];\n\n")

    inst_flags_map = {"None": "NONE", "MovOp": "MOV_OP"}
    ifl_rows = split_line_rows(add_tables["inst_flags_table"])
    check(
        len(ifl_rows) == X86_INST_FLAGS_COUNT,
        f"x86: expected {X86_INST_FLAGS_COUNT} inst_flags_table rows, got {len(ifl_rows)}",
    )
    check_ordinals([c for _, c in ifl_rows], "x86 inst_flags_table")
    out.append("/// Instruction RW flags, indexed by [`AdditionalInfo::inst_flags_index`].\n")
    out.append("pub static INST_FLAGS_TABLE: &[InstRwFlags] = &[\n")
    for code, comment in ifl_rows:
        name, args = parse_macro_invocation(code)
        check(name == "InstRWFlags" and len(args) == 1, f"x86: bad inst_flags row {code!r}")
        m = re.fullmatch(r"FLAG\(([A-Za-z]+)\)", args[0])
        check(m is not None and m.group(1) in inst_flags_map, f"x86: bad InstRWFlags term {args[0]!r}")
        out.append(f"    InstRwFlags::{inst_flags_map[m.group(1)]}, // {comment}\n")
    out.append("];\n\n")

    name_block = extract_block(db.inst_cpp, "NameData")
    spans_m = re.search(
        r"const InstNameIndex InstDB::_inst_name_index = \{\{(.*?)\},\s*uint16_t\((\d+)\)\};",
        name_block,
        re.S,
    )
    check(spans_m is not None, "x86: _inst_name_index not found")
    span_rows = []
    for raw in spans_m.group(1).splitlines():
        line = raw.strip()
        if not line:
            continue
        fields = parse_brace_row(line)
        check(len(fields) == 2, f"x86: bad name index span {line!r}")
        mapped = []
        for f in fields:
            m = re.fullmatch(r"Inst::kId([A-Za-z_0-9]+)(\s*\+\s*1)?", f.strip())
            check(m is not None, f"x86: bad name index span field {f!r}")
            mapped.append(f"InstId::{m.group(1)} as u16" + (" + 1" if m.group(2) else ""))
        span_rows.append(mapped)
    check(len(span_rows) == 26, f"x86: expected 26 name index spans, got {len(span_rows)}")
    out.append("/// Maps the first letter of an instruction name to a span of [`InstId`] values.\n")
    out.append("pub static INST_NAME_INDEX: &[(u16, u16)] = &[\n")
    for a, b in span_rows:
        out.append(f"    ({a}, {b}),\n")
    out.append("];\n\n")
    out.append(f"pub const MAX_NAME_LENGTH: u16 = {spans_m.group(2)};\n\n")

    string_m = re.search(
        r"const char InstDB::_inst_name_string_table\[\] =\s*(.*?);", name_block, re.S
    )
    check(string_m is not None, "x86: _inst_name_string_table not found")
    name_string = decode_c_string_table(string_m.group(1))
    out.append(f"pub static INST_NAME_STRING_TABLE: &[u8] = {rust_byte_string(name_string)};\n\n")

    name_tables = split_tables(strip_generated_banner(name_block))
    index_rows = split_line_rows(name_tables["_inst_name_index_table"])
    check(
        len(index_rows) == X86_INST_ID_COUNT,
        f"x86: expected {X86_INST_ID_COUNT} name index rows, got {len(index_rows)}",
    )
    out.append("#[rustfmt::skip]\n")
    out.append("pub static INST_NAME_INDEX_TABLE: &[u32] = &[\n")
    for idx, (code, comment) in enumerate(index_rows):
        value, _ = strip_trailing_comma(code)
        comma = "," if idx + 1 < len(index_rows) else ""
        out.append(f"    {value}{comma} // {comment}\n")
    out.append("];\n\n")

    alias_string_m = re.search(
        r"const char InstDB::alias_name_string_table\[\] =\s*(.*?);", name_block, re.S
    )
    check(alias_string_m is not None, "x86: alias_name_string_table not found")
    alias_string = decode_c_string_table(alias_string_m.group(1))
    out.append(f"pub static ALIAS_NAME_STRING_TABLE: &[u8] = {rust_byte_string(alias_string)};\n\n")

    alias_index_rows = split_line_rows(name_tables["alias_name_index_table"])
    check(
        len(alias_index_rows) == X86_ALIAS_COUNT,
        f"x86: expected {X86_ALIAS_COUNT} alias name index rows, got {len(alias_index_rows)}",
    )
    out.append("pub static ALIAS_NAME_INDEX_TABLE: &[u32] = &[\n")
    for idx, (code, comment) in enumerate(alias_index_rows):
        value, _ = strip_trailing_comma(code)
        comma = "," if idx + 1 < len(alias_index_rows) else ""
        out.append(f"    {value}{comma} // {comment}\n")
    out.append("];\n\n")

    alias_id_rows = split_line_rows(name_tables["alias_index_to_inst_id_table"])
    check(
        len(alias_id_rows) == X86_ALIAS_COUNT,
        f"x86: expected {X86_ALIAS_COUNT} alias id rows, got {len(alias_id_rows)}",
    )
    check_ordinals([c for _, c in alias_id_rows], "x86 alias_index_to_inst_id_table")
    out.append("pub static ALIAS_INDEX_TO_INST_ID_TABLE: &[u32] = &[\n")
    for idx, (code, comment) in enumerate(alias_id_rows):
        value, _ = strip_trailing_comma(code)
        m = re.fullmatch(r"Inst::kId([A-Za-z_0-9]+)", value)
        check(m is not None, f"x86: bad alias id row {code!r}")
        comma = "," if idx + 1 < len(alias_id_rows) else ""
        out.append(f"    InstId::{m.group(1)} as u32{comma} // {comment}\n")
    out.append("];\n\n")
    out.append(f"pub const ALIAS_TABLE_SIZE: u32 = {X86_ALIAS_COUNT};\n\n")

    sig_block = extract_block(db.inst_cpp, "InstSignatureTable")
    sig_tables = split_tables(strip_generated_banner(sig_block))

    sig_rows = split_line_rows(sig_tables["_inst_signature_table"])
    check(
        len(sig_rows) == X86_SIGNATURE_COUNT,
        f"x86: expected {X86_SIGNATURE_COUNT} InstSignature rows, got {len(sig_rows)}",
    )
    check_ordinals([c for _, c in sig_rows], "x86 _inst_signature_table", sparse=True)
    mode_map = {
        ("1", "1"): "Any",
        ("1", "0"): "X86",
        ("0", "1"): "X64",
        ("0", "0"): "None",
    }
    out.append("/// Instruction signatures, see [`CommonInfo::signature_index`].\n")
    out.append("pub static INST_SIGNATURE_TABLE: &[InstSignature] = &[\n")
    for code, comment in sig_rows:
        name, args = parse_macro_invocation(code)
        check(name == "ROW" and len(args) == 10, f"x86: bad InstSignature row {code!r}")
        check((args[1], args[2]) in mode_map, f"x86: bad mode in row {code!r}")
        mode = mode_map[(args[1], args[2])]
        indexes = ", ".join(args[4:])
        suffix = f" // {comment}" if comment else ""
        out.append(
            f"    InstSignature::new({args[0]}, Mode::{mode} as u8, {args[3]}, 0, [{indexes}]),{suffix}\n"
        )
    out.append("];\n\n")

    op_sig_rows = split_line_rows(sig_tables["_op_signature_table"])
    check(
        len(op_sig_rows) == X86_OP_SIGNATURE_COUNT,
        f"x86: expected {X86_OP_SIGNATURE_COUNT} OpSignature rows, got {len(op_sig_rows)}",
    )
    out.append("/// Operand signatures, indexed by [`InstSignature::op_signature_indexes`].\n")
    out.append("pub static OP_SIGNATURE_TABLE: &[OpSignature] = &[\n")
    for code, comment in op_sig_rows:
        name, args = parse_macro_invocation(code)
        check(name == "ROW" and len(args) == 2, f"x86: bad OpSignature row {code!r}")
        out.append(f"    OpSignature::new({db.map_op_flags(args[0])}, {args[1]}),\n")
    out.append("];\n\n")

    rw_block = extract_block(db.inst_cpp, "InstRWInfoTable")
    rw_tables = split_tables(strip_generated_banner(rw_block))

    for var, rust_name in (
        ("rw_info_index_a_table", "RW_INFO_INDEX_A_TABLE"),
        ("rw_info_index_b_table", "RW_INFO_INDEX_B_TABLE"),
    ):
        body = re.sub(r"//[^\n]*", "", rw_tables[var])
        values = [parse_int(v) for v in body.replace("\n", " ").split(",") if v.strip()]
        check(
            len(values) == X86_INST_ID_COUNT,
            f"x86: expected {X86_INST_ID_COUNT} values in {var}, got {len(values)}",
        )
        out.append(emit_u8_table(rust_name, values, ""))

    rw_category_set = {n[len("kCategory"):] for n, _, _ in db.rw_info_category_entries}
    for var, rust_name, expected in (
        ("rw_info_a_table", "RW_INFO_A_TABLE", X86_RW_INFO_A_COUNT),
        ("rw_info_b_table", "RW_INFO_B_TABLE", X86_RW_INFO_B_COUNT),
    ):
        rows = split_line_rows(rw_tables[var])
        check(len(rows) == expected, f"x86: {var} has {len(rows)} rows, expected {expected}")
        check_ordinals([c for _, c in rows], f"x86 {var}")
        out.append(f"pub static {rust_name}: &[RwInfo] = &[\n")
        for code, comment in rows:
            fields = parse_brace_row(code)
            check(len(fields) == 3, f"x86: RWInfo row with {len(fields)} fields: {code!r}")
            m = re.fullmatch(r"InstDB::RWInfo::kCategory([A-Za-z_0-9]+)", fields[0])
            check(m is not None and m.group(1) in rw_category_set,
                  f"x86: bad RWInfo category {fields[0]!r}")
            indexes = ", ".join(parse_brace_row(fields[2]))
            out.append(
                f"    RwInfo::new(RwInfoCategory::{m.group(1)}, {fields[1]}, [{indexes}]), // {comment}\n"
            )
        out.append("];\n\n")

    # rw_info_op_table.
    op_rows = split_line_rows(rw_tables["rw_info_op_table"])
    check(
        len(op_rows) == X86_RW_INFO_OP_COUNT,
        f"x86: rw_info_op_table has {len(op_rows)} rows, expected {X86_RW_INFO_OP_COUNT}",
    )
    check_ordinals([c for _, c in op_rows], "x86 rw_info_op_table")
    out.append("pub static RW_INFO_OP_TABLE: &[RwInfoOp] = &[\n")
    for code, comment in op_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 6, f"x86: RWInfoOp row with {len(fields)} fields: {code!r}")
        check(fields[4].replace(" ", "") == "{0}", f"x86: RWInfoOp reserved field not zero: {code!r}")
        out.append(
            f"    RwInfoOp::new({strip_int_suffix(fields[0])}, {strip_int_suffix(fields[1])}, "
            f"{fields[2]}, {fields[3]}, {db.map_op_rw_flags(fields[5])}), // {comment}\n"
        )
    out.append("];\n\n")

    # rw_info_rm_table.
    rm_category_set = {n[len("kCategory"):] for n, _, _ in db.rw_info_rm_category_entries}
    rm_rows = split_line_rows(rw_tables["rw_info_rm_table"])
    check(
        len(rm_rows) == X86_RW_INFO_RM_COUNT,
        f"x86: rw_info_rm_table has {len(rm_rows)} rows, expected {X86_RW_INFO_RM_COUNT}",
    )
    check_ordinals([c for _, c in rm_rows], "x86 rw_info_rm_table")
    out.append("pub static RW_INFO_RM_TABLE: &[RwInfoRm] = &[\n")
    for code, comment in rm_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 5, f"x86: RWInfoRm row with {len(fields)} fields: {code!r}")
        m = re.fullmatch(r"InstDB::RWInfoRm::kCategory([A-Za-z_0-9]+)", fields[0])
        check(m is not None and m.group(1) in rm_category_set,
              f"x86: bad RWInfoRm category {fields[0]!r}")

        def rm_flag_one(part: str) -> str:
            fm = re.fullmatch(r"InstDB::RWInfoRm::kFlag([A-Za-z_0-9]+)", part)
            check(fm is not None, f"x86: bad RWInfoRm flag {part!r}")
            return f"RwInfoRmFlags::{screaming(fm.group(1))}.bits()"

        flags = map_flag_expr(fields[3], rm_flag_one)
        feature = fields[4]
        if feature != "0":
            fm = re.fullmatch(r"uint32_t\(CpuFeatures::X86::k([A-Za-z_0-9]+)\)", feature)
            check(fm is not None and fm.group(1) in db.cpu_feature_map,
                  f"x86: bad RWInfoRm rm_feature {feature!r}")
            feature = f"CpuFeature::{db.cpu_feature_map[fm.group(1)]} as u8"
        out.append(
            f"    RwInfoRm::new(RwInfoRmCategory::{m.group(1)}, {fields[1]}, {fields[2]}, "
            f"{flags}, {feature}), // {comment}\n"
        )
    out.append("];\n\n")

    return "".join(out)



def gen_x86() -> str:
    db = X86Db()
    inst_rows = x86_parse_inst_rows(db)

    json_rows = [
        {
            "id": r["id"],
            "encoding": r["encoding"],
            "opcode0": r["opcode0"],
            "opcode1": r["opcode1"],
            "main_opcode_index": int(r["main_opcode_index"]),
            "alt_opcode_index": int(r["alt_opcode_index"]),
            "common_info_index": int(r["common_info_index"]),
            "additional_info_index": int(r["additional_info_index"]),
            "rw_info_index": None,
            "flags": None,
            "opcode_data_index": None,
            "opcode_data": None,
        }
        for r in inst_rows
    ]
    write_json(
        REPO_ROOT / "meta" / "x86_rows.json",
        {"arch": "x86", "source": "x86/x86instdb.cpp ${InstInfo}", "rows": json_rows},
    )

    result = emit_x86(db)
    write_file(REPO_ROOT / "src" / "x86" / "instdb.rs", result)
    return result



def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.read_text(encoding="utf-8") == content:
        print(f"{path.relative_to(REPO_ROOT)}: up to date")
        return
    path.write_text(content, encoding="utf-8")
    print(f"{path.relative_to(REPO_ROOT)}: written ({len(content)} bytes)")


def write_json(path: Path, data) -> None:
    write_file(path, json.dumps(data, indent=1) + "\n")


def main(argv: list[str]) -> int:
    check_only = "--check" in argv
    args = [a for a in argv[1:] if not a.startswith("--")]
    check(
        len(args) == 1 and args[0] in ("x86", "aarch64", "all"),
        "usage: python3 meta/asmjit2rust.py {x86|aarch64|all} [--check]",
    )
    target = args[0]
    if check_only:
        check(target == "aarch64", "--check is only supported for the aarch64 target")
        gen_aarch64(check_only=True)
        return 0
    if target in ("aarch64", "all"):
        gen_aarch64()
    if target in ("x86", "all"):
        gen_x86()
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
