#!/usr/bin/env python3
"""Generates the x86 emitter traits (src/x86/emitter.rs) from meta/x86_emitter.txt.

The input is a curated copy of AsmJit's x86emitter.h ASMJIT_INST_* declarations.
The transformation mirrors meta/arm64.py (aarch64):

* one `pub trait {Camel}Emitter<T0..Tn>` per (mnemonic, arity) pair;
* one `impl {Camel}Emitter<...> for Assembler<'_>` per concrete operand tuple,
  forwarding to `self.emit_n(InstId::{Id}, &[op0.as_operand(), ...])`;
* when a mnemonic is declared with several arities, the first-seen arity keeps
  the plain name and later ones are suffixed (`cbw` / `cbw_0`);
* Rustdoc combines assembly forms from pinned AsmJit metadata with optional
  prose from the docenizer database (meta/docenizer_amd64.py).

X86-specific handling:

* AsmJit fixed-register alias operands (Gp_AX, DS_ZSI, XMM0, ...) collapse to
  their base operand kind (Gp, Mem, Vec); tuples that become duplicates after
  collapsing are dropped (first one wins);
* the conditional families j/set/cmov (ASMJIT_INST_1c/2c) generate one method
  taking a CondCode (dispatched through a condition -> InstId table) plus one
  named method per x86 condition suffix (jo, jno, ...), each calling a concrete
  InstId::{Id}{Suffix};
* the condition tables are reindexed to asmkit's ARM-ordered CondCode
  (AsmJit's own _*_from_cond tables are x86-ordered);
* Rust keywords are raw-escaped (r#loop, r#in); names already C++-escaped in
  the input (and_, or_, not_, xor_, int_) are kept as-is.

Sized-register impls:

* besides the abstract-kind impls (MovEmitter<Gp, Gp>), the generator emits
  impls for the concrete sized register wrappers (Gpq/Gpd/Gpw/GpbLo/GpbHi,
  Xmm/Ymm/Zmm, ...) so call sites can pass the register constants directly
  (`a.mov(RAX, RBX)`, no deref);
* the width data comes from the same instdb the rest of the pipeline uses:
  meta/asmjit_db parses AsmJit's x86instdb.cpp (InstId -> CommonInfo ->
  InstSignature -> OpSignature flag sets); only signature-valid width
  combinations are generated (mov r,r -> same-width pairs, movzx -> mixed);
* immediate positions are generated as `U: Into<Imm>` so integer literals work
  (`a.mov(RAX, 42)`); when a variant's sized expansion is exactly its abstract
  tuple with Imm generalized, the generic impl replaces the abstract one
  (they would overlap), otherwise the abstract impl is kept.

Usage: python3 meta/x86_emitter_gen.py [--docs-inputfolder DIR] [--check] OUTPUT
"""

import argparse
import itertools
import json
import os
import re
import sys

from docenizer_amd64 import collect_instruction_docs

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.dirname(SCRIPT_DIR)
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

INPUT_PATH = os.path.join(SCRIPT_DIR, "x86_emitter.txt")
X86GLOBALS_PATH = os.path.join(SCRIPT_DIR, "asmjit", "asmjit", "x86", "x86globals.h")
X86_ISA_PATH = os.path.join(SCRIPT_DIR, "asmjit", "db", "isa_x86.json")

DOCS_INPUT = os.environ.get("ASMKIT_X86_DOCS", "asm-docs")

# Operand tokens that appear verbatim in macro arguments and map one-to-one to
# asmkit types (src/x86/operands.rs and crate::core::operand).
BASE_OPERAND_TOKENS = {
    "Gp", "Vec", "Mem", "Imm", "Label", "KReg", "Mm", "St", "Tmm", "Bnd",
    "SReg", "CReg", "DReg",
}

# AsmJit fixed-register/pointer aliases that collapse to a base operand kind.
ALIAS_TOKENS = {
    **{token: "Gp" for token in (
        "Gp_AL", "Gp_AH", "Gp_CL", "Gp_AX", "Gp_DX", "Gp_EAX", "Gp_EBX",
        "Gp_ECX", "Gp_EDX", "Gp_RAX", "Gp_RBX", "Gp_RCX", "Gp_RDX", "Gp_ZAX",
        "Gp_ZBX", "Gp_ZCX", "Gp_ZDX",
    )},
    **{token: "Mem" for token in ("DS_ZAX", "DS_ZSI", "DS_ZDI", "ES_ZDI")},
    "XMM0": "Vec",
}

# ASMJIT_INST_nc CONV argument -> (condition table name, InstId prefix).
CONV_TABLES = {
    "Inst::jcc_from_cond": ("JCC_FROM_COND", "J"),
    "Inst::setcc_from_cond": ("SETCC_FROM_COND", "Set"),
    "Inst::cmovcc_from_cond": ("CMOVCC_FROM_COND", "Cmov"),
}

# Marker used in typed operand tuples for an immediate position generated as
# `U: Into<Imm>` (accepts Imm and the integer types Imm: From<int> covers).
IMM_GENERIC = "<imm>"

# AsmJit OpFlags (`F(...)` terms of the OpSignature table, `k` stripped) that
# map to the concrete sized wrapper types of src/x86/operands.rs. Order is
# significant: it is the emission order of typed impls.
GP_SIZED_TYPES = [
    ("RegGpbLo", "GpbLo"),
    ("RegGpbHi", "GpbHi"),
    ("RegGpw", "Gpw"),
    ("RegGpd", "Gpd"),
    ("RegGpq", "Gpq"),
]
VEC_SIZED_TYPES = [
    ("RegXmm", "Xmm"),
    ("RegYmm", "Ymm"),
    ("RegZmm", "Zmm"),
]
# Base kinds that map one-to-one to a single OpFlags register bit.
SINGLE_REG_FLAGS = {
    "KReg": "RegKReg",
    "Mm": "RegMm",
    "SReg": "RegSReg",
    "CReg": "RegCReg",
    "DReg": "RegDReg",
    "St": "RegSt",
    "Bnd": "RegBnd",
    "Tmm": "RegTmm",
}


def load_width_db():
    """Loads per-InstId operand signatures from the vendored AsmJit instdb via
    meta/asmjit_db (the same parser that generates src/x86/instdb.rs).

    Returns {InstId name: [(frozenset of OpFlags names per explicit operand)]}.
    """
    from meta.asmjit_db.cxx_src import (
        extract_block,
        parse_brace_row,
        parse_macro_invocation,
        split_line_rows,
        split_tables,
        strip_generated_banner,
    )
    from meta.asmjit_db.tablegen_x86 import X86Db, x86_parse_inst_rows

    db = X86Db()

    # CommonInfo rows: {flags, avx512_flags, signature_index, signature_count, ...}.
    common_tables = split_tables(strip_generated_banner(
        extract_block(db.inst_cpp, "InstCommonTable")))
    common_rows = []
    for code, _ in split_line_rows(common_tables["_inst_common_info_table"]):
        fields = parse_brace_row(code)
        common_rows.append((int(fields[2]), int(fields[3])))

    sig_tables = split_tables(strip_generated_banner(
        extract_block(db.inst_cpp, "InstSignatureTable")))
    inst_sigs = []  # (op_count, implicit_op_count, [op_signature_indexes x 6])
    for code, _ in split_line_rows(sig_tables["_inst_signature_table"]):
        name, args = parse_macro_invocation(code)
        assert name == "ROW" and len(args) == 10, f"bad InstSignature row {code!r}"
        inst_sigs.append((int(args[0]), int(args[3]), [int(a) for a in args[4:]]))

    op_sigs = []  # frozenset of OpFlags names per row
    for code, _ in split_line_rows(sig_tables["_op_signature_table"]):
        name, args = parse_macro_invocation(code)
        assert name == "ROW" and len(args) == 2, f"bad OpSignature row {code!r}"
        op_sigs.append(frozenset(re.findall(r"F\(([A-Za-z_0-9]+)\)", args[0])))

    width_db = {}
    for row in x86_parse_inst_rows(db):
        sig_index, sig_count = common_rows[int(row["common_info_index"])]
        forms = []
        for op_count, implicit, indexes in inst_sigs[sig_index:sig_index + sig_count]:
            explicit = tuple(op_sigs[i] for i in indexes[implicit:op_count])
            full = tuple(op_sigs[i] for i in indexes[:op_count])
            forms.append((explicit, full))
        width_db[row["id"]] = forms
    return width_db


def expand_position(base_kind, flags):
    """Maps one signature operand's OpFlags set to the concrete operand types a
    `base_kind` emitter parameter can take, or None when the base kind cannot
    appear at this position."""
    if base_kind == "Gp":
        types = [rust for flag, rust in GP_SIZED_TYPES if flag in flags]
        return types or None
    if base_kind == "Vec":
        types = [rust for flag, rust in VEC_SIZED_TYPES if flag in flags]
        return types or None
    if base_kind == "Mem":
        return ["Mem"] if any(f.startswith(("Mem", "Vm")) for f in flags) else None
    if base_kind == "Imm":
        if any(f.startswith(("Imm", "Rel")) for f in flags):
            return [IMM_GENERIC]
        return None
    if base_kind == "Label":
        return ["Label"] if any(f.startswith("Rel") for f in flags) else None
    flag = SINGLE_REG_FLAGS.get(base_kind)
    return [base_kind] if flag in flags else None


def typed_operand_tuples(variant, forms_list):
    """Computes the sized operand tuples for one emitter variant from the
    instruction's signature forms (see load_width_db). Returns a deduplicated
    list of tuples in which immediate positions carry IMM_GENERIC."""
    arity = len(variant.operands)
    seen, out = set(), []
    for explicit, full in forms_list:
        # Explicit decls match the non-implicit signature operands; decls that
        # spell fixed registers out (cbw(Gp_AX)) match the full operand list.
        if len(explicit) == arity:
            form = explicit
        elif len(full) == arity:
            form = full
        else:
            continue
        per_position = [expand_position(kind, flags)
                        for kind, flags in zip(variant.operands, form)]
        if any(types is None for types in per_position):
            continue
        for combo in itertools.product(*per_position):
            if combo not in seen:
                seen.add(combo)
                out.append(combo)
    return out

# All sixteen x86 condition suffixes, in AsmJit's table order.
X86_CC_SUFFIXES = [
    "o", "no", "b", "nb", "z", "nz", "be", "nbe",
    "s", "ns", "p", "np", "l", "nl", "le", "nle",
]

# asmkit's CondCode (src/core/globals.rs) is ARM-ordered; map each condition to
# the x86 suffix with the same meaning. AL/NA have no x86 cc form: `j` maps AL
# to the unconditional Jmp, everything else maps them to InstId::None.
ASMKIT_COND_ORDER = [
    "AL", "NA", "EQ", "NE", "CS", "LO", "MI", "PL",
    "VS", "VC", "HI", "LS", "GE", "LT", "GT", "LE",
]
ASMKIT_COND_TO_X86 = {
    "EQ": "z", "NE": "nz", "CS": "nb", "LO": "b",
    "MI": "s", "PL": "ns", "VS": "o", "VC": "no",
    "HI": "nbe", "LS": "be", "GE": "nl", "LT": "l",
    "GT": "nle", "LE": "le",
}

RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "try",
    "type", "unsafe", "use", "where", "while",
    # Reserved keywords.
    "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "yield",
}

MACRO_RE = re.compile(r"^ASMJIT_INST_(\d+)(x|c)\((.*)\)$")
SPOT_CHECK_NAMES = ["adc", "vaddps", "j", "cmpxchg16b", "tileloadd"]


class Variant:
    def __init__(self, inst_id, operands, comment):
        self.inst_id = inst_id
        # Operand tuple with alias tokens already collapsed to base kinds.
        self.operands = operands
        # Normalized trailing comment from the input line (feature tag, section
        # tag, description); empty when the declaration had none.
        self.comment = comment


class Opcode:
    def __init__(self, name, conv):
        self.name = name
        # CONV token (Inst::*_from_cond) for conditional families, else None.
        self.conv = conv
        self.variants = []


def clean_asmjit_form(form):
    """Turns an AsmJit ISA syntax row into a compact Rustdoc assembly form."""
    form = re.sub(r"^(?:\[[^]]+\]\s*)+", "", form)
    mnemonic, _, operands = form.partition(" ")
    operands = re.sub(r"(^|,\s*)~?[A-Za-z]+:", r"\1", operands)
    operands = re.sub(r"\{[^}]*\}", "", operands)
    operands = " ".join(operands.replace("~", "").split())
    operands = re.sub(r"\s*,\s*", ", ", operands)
    return mnemonic, f"{mnemonic}{' ' + operands if operands else ''}"


def load_asmjit_forms(path):
    """Loads assembly forms and operand access prefixes from pinned AsmJit JSON."""
    with open(path) as f:
        metadata = json.load(f)

    forms = {}

    def visit(value):
        if isinstance(value, dict):
            for key in ("any", "x86", "x64"):
                if key not in value:
                    continue
                raw = value[key]
                mnemonic, syntax = clean_asmjit_form(raw)
                operand_text = raw.partition(" ")[2]
                roles = []
                for operand in operand_text.split(","):
                    match = re.match(r"\s*~?([A-Za-z]+):", operand)
                    roles.append(match.group(1) if match else "r")
                forms.setdefault(mnemonic.upper(), []).append((syntax, roles))
            for child in value.values():
                visit(child)
        elif isinstance(value, list):
            for child in value:
                visit(child)

    visit(metadata)
    return forms


def load_opcode_docs(inputfolder):
    if not os.path.isdir(inputfolder):
        print(f"Note: docs folder '{inputfolder}' not found, "
              "emitting fallback doc comments")
        return {}
    try:
        docs = collect_instruction_docs(inputfolder)
        print(f"Loaded {len(docs)} instruction docs from '{inputfolder}'")
        return docs
    except Exception as exc:
        print(f"Warning: failed to load x86 docs from '{inputfolder}': {exc}")
        return {}


def parse_opcodes(path):
    opcodes = {}
    stats = {
        "decls": 0,
        "cc_decls": 0,
        "dropped_duplicates": 0,
        "unknown_tokens": [],
        "arity_mismatches": [],
        "conv_mismatches": [],
    }

    with open(path, "r") as f:
        for lineno, raw_line in enumerate(f, 1):
            line = raw_line.strip()
            if not line.startswith("ASMJIT_INST_"):
                continue

            macro, _, comment = line.partition("//")
            comment = " ".join(comment.split())
            match = MACRO_RE.match(macro.strip())
            if not match:
                stats["arity_mismatches"].append(f"line {lineno}: unparseable: {line}")
                continue

            args = [arg.strip() for arg in match.group(3).split(",")]
            name, inst_id = args[0], args[1]
            arity = int(match.group(1))
            is_cc = match.group(2) == "c"

            if is_cc:
                conv = args[2]
                operands = args[3:]
                if conv not in CONV_TABLES:
                    stats["conv_mismatches"].append(f"line {lineno}: unknown CONV '{conv}'")
                    continue
                if inst_id != CONV_TABLES[conv][1]:
                    stats["conv_mismatches"].append(
                        f"line {lineno}: {name}: id '{inst_id}' != table base "
                        f"'{CONV_TABLES[conv][1]}'")
            else:
                conv = None
                operands = args[2:]

            if len(operands) != arity:
                stats["arity_mismatches"].append(
                    f"line {lineno}: {name}: macro arity {arity} != "
                    f"operand count {len(operands)}")
                continue

            unknown = [t for t in operands
                       if t not in BASE_OPERAND_TOKENS and t not in ALIAS_TOKENS]
            if unknown:
                stats["unknown_tokens"].append(f"line {lineno}: {name}: {unknown}")
                continue

            stats["decls"] += 1
            if is_cc:
                stats["cc_decls"] += 1
            collapsed = tuple(ALIAS_TOKENS.get(t, t) for t in operands)

            # Bucket by (name, arity): the first-seen arity keeps the plain
            # name, later arities get a `{name}_{arity}` bucket (arm64 rule).
            if name not in opcodes:
                opcodes[name] = Opcode(name, conv)
                bucket = opcodes[name]
            elif len(collapsed) != len(opcodes[name].variants[0].operands):
                split_name = f"{name}_{len(collapsed)}"
                if split_name not in opcodes:
                    opcodes[split_name] = Opcode(split_name, conv)
                bucket = opcodes[split_name]
            else:
                bucket = opcodes[name]

            if any(v.operands == collapsed for v in bucket.variants):
                stats["dropped_duplicates"] += 1
                continue
            bucket.variants.append(Variant(inst_id, collapsed, comment))

    stats["distinct_names"] = len({re.sub(r"_\d+$", "", name) for name in opcodes})
    return opcodes, stats


def parse_inst_id_names(path):
    """Parse the ${InstId:Begin}..${InstId:End} enum body of x86globals.h and
    return the set of `kId`-stripped variant names (including aliases)."""
    with open(path, "r") as f:
        text = f.read()
    body = text.split("${InstId:Begin}")[1].split("${InstId:End}")[0]
    names = set()
    for line in body.splitlines():
        line = line.split("//")[0]
        names.update(m.group(1) for m in re.finditer(r"kId([A-Za-z0-9]+)", line))
    return names


def trait_name(name):
    camel = "".join(part.capitalize() for part in name.split("_"))
    # Keep the C++-escaping underscore (arm64's `mvn_` -> `Mvn_` precedent).
    if name.endswith("_"):
        camel += "_"
    return camel


def rust_method_name(name):
    if name in RUST_KEYWORDS:
        return f"r#{name}"
    return name


def doc_lookup_name(name):
    return re.sub(r"(_\d+|_)+$", "", name).upper()


def forms_for_opcode(name, asmjit_forms):
    canonical = doc_lookup_name(name)
    forms = asmjit_forms.get(canonical, [])
    if forms or canonical not in {"J", "SET", "CMOV"}:
        return forms

    # AsmJit stores each condition-code spelling separately. Keep the public
    # family API readable while still deriving its shapes from those rows.
    prefix = {"J": "J", "SET": "SET", "CMOV": "CMOV"}[canonical]
    family = []
    for mnemonic, entries in asmjit_forms.items():
        if mnemonic.startswith(prefix):
            family.extend((re.sub(r"^[a-z0-9]+", canonical.lower() + "cc", syntax), roles)
                          for syntax, roles in entries)
    return family


def unique_names(names):
    seen = {}
    unique = []
    for name in names:
        seen[name] = seen.get(name, 0) + 1
        unique.append(name if seen[name] == 1 else f"{name}{seen[name]}")
    return unique


def operand_names(opcode, operands, asmjit_forms):
    """Names public emitter arguments from the ISA's access annotations."""
    if opcode.name == "enter":
        return ["frame_size", "nesting_level"]

    forms = forms_for_opcode(opcode.name, asmjit_forms)
    roles = next((roles for _, roles in forms if len(roles) == len(operands)), [])
    control_flow = opcode.name in {"call", "jmp", "j", "loop", "loope", "loopne"}
    all_read = len(roles) == len(operands) and all(role.lower() == "r" for role in roles)
    names = []
    for index, kind in enumerate(operands):
        role = roles[index] if index < len(roles) else ""
        if kind == "Imm":
            name = "imm"
        elif kind == "Label" or control_flow:
            name = "target"
        elif role.lower() in {"w", "x"}:
            name = "dst"
        elif all_read and len(operands) == 2:
            name = ("lhs", "rhs")[index]
        elif len(operands) == 1 and kind == "Mem":
            name = "mem"
        elif index == 0 and len(operands) > 1:
            name = "dst"
        else:
            name = "src"
        names.append(name)
    source_indexes = [index for index, name in enumerate(names) if name == "src"]
    if len(source_indexes) > 1:
        for source_number, index in enumerate(source_indexes, 1):
            names[index] = f"src{source_number}"
    return unique_names(names)


def doc_lines(opcode, opcode_docs, asmjit_forms, indent=""):
    canonical = doc_lookup_name(opcode.name)
    doc = opcode_docs.get(canonical)
    if doc:
        summary = " ".join(doc["tooltip"].split()).replace("`", "'")
        lines = [f"{indent}/// `{canonical}` — {summary}"]
    else:
        lines = [f"{indent}/// Emits `{opcode.name.upper()}`."]

    forms = []
    for syntax, _ in forms_for_opcode(opcode.name, asmjit_forms):
        if syntax not in forms:
            forms.append(syntax)
        if len(forms) == 3:
            break
    if forms:
        lines.extend([f"{indent}///", f"{indent}/// Assembly forms: "
                      + ", ".join(f"`{form}`" for form in forms) + "."])
    else:
        names = operand_names(opcode, opcode.variants[0].operands, asmjit_forms)
        syntax = " ".join([opcode.name] + [", ".join(names)]).strip()
        lines.extend([f"{indent}///", f"{indent}/// Syntax: `{syntax}`."])
    if doc and doc.get("url"):
        lines.extend([f"{indent}///", f"{indent}/// [Intel reference]({doc['url']})."])
    return lines


def fn_params(operands, names, cc=False):
    parts = ["&mut self"]
    if cc:
        parts.append("cc: CondCode")
    parts.extend(f"{name}: {operand}" for name, operand in zip(names, operands))
    return ", ".join(parts)


def trait_block(opcode, opcode_docs, asmjit_forms):
    n = len(opcode.variants[0].operands)
    generics = f"<{', '.join(f'T{i}' for i in range(n))}>" if n else ""
    generic_operands = [f"T{i}" for i in range(n)]
    method = rust_method_name(opcode.name)

    names = operand_names(opcode, opcode.variants[0].operands, asmjit_forms)
    lines = doc_lines(opcode, opcode_docs, asmjit_forms)
    lines.append(f"pub trait {trait_name(opcode.name)}Emitter{generics} {{")
    if opcode.conv:
        lines.append(f"    fn {method}({fn_params(generic_operands, names, cc=True)});")
        for suffix in X86_CC_SUFFIXES:
            lines.append(f"    fn {method}{suffix}({fn_params(generic_operands, names)});")
    else:
        lines.append(f"    fn {method}({fn_params(generic_operands, names)});")
    lines.append("}")
    return "\n".join(lines)


def impl_block(opcode, inst_id, operands, comment, asmjit_forms):
    """Emits one impl. `operands` holds concrete Rust types; IMM_GENERIC marks
    an immediate position generated as a `U{i}: Into<Imm>` type parameter."""
    trait = trait_name(opcode.name)
    type_args = [f"U{i}" if op == IMM_GENERIC else op
                 for i, op in enumerate(operands)]
    generics = f"<{', '.join(type_args)}>" if type_args else ""
    bounds = [f"U{i}: Into<Imm>" for i, op in enumerate(operands) if op == IMM_GENERIC]
    impl_generics = f"<{', '.join(bounds)}>" if bounds else ""
    names = operand_names(opcode, operands, asmjit_forms)
    params = ["&mut self"]
    params.extend(f"{name}: {arg}" for name, arg in zip(names, type_args))
    ops = "&[{}]".format(", ".join(
        f"Into::<Imm>::into({name}).as_operand()" if op == IMM_GENERIC
        else f"{name}.as_operand()"
        for name, op in zip(names, operands)))
    comment = f" // {comment}" if comment else ""
    method = rust_method_name(opcode.name)

    lines = [f"impl{impl_generics} {trait}Emitter{generics} for Assembler<'_> {{"]
    if opcode.conv:
        table, prefix = CONV_TABLES[opcode.conv]
        cc_params = ", ".join([params[0], "cc: CondCode"] + params[1:])
        lines.append(f"    fn {method}({cc_params}) {{")
        lines.append(f"        self.emit_n({table}[cc as usize], {ops});{comment}")
        lines.append("    }")
        for suffix in X86_CC_SUFFIXES:
            lines.append(f"    fn {method}{suffix}({', '.join(params)}) {{")
            lines.append(f"        self.emit_n(InstId::{prefix}{suffix} as u32, {ops});")
            lines.append("    }")
    else:
        lines.append(f"    fn {method}({', '.join(params)}) {{")
        lines.append(f"        self.emit_n(InstId::{inst_id} as u32, {ops});{comment}")
        lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


HEADER = """\
//! X86 emitter traits, generated by `meta/x86_emitter_gen.py` from
//! `meta/x86_emitter.txt` (AsmJit's `x86emitter.h` declarations). Do not edit by
//! hand; regenerate instead.
//!
//! Conventions:
//! - One `{Name}Emitter<T0..Tn>` trait per (mnemonic, arity) pair. `Assembler<'_>`
//!   implements it once per declared concrete operand tuple, forwarding to
//!   `emit_n(InstId::{Id} as u32, &[...])`. Unlike the legacy `features/*` API there
//!   are no inherent forwarding methods, so both APIs can coexist.
//! - Both `[EXPLICIT]` (fixed-register operands spelled out) and `[IMPLICIT]`
//!   (fixed registers hidden) input declarations are kept. When a mnemonic is
//!   declared with more than one arity, the first-seen arity keeps the plain name
//!   and later arities are suffixed: the 1-operand explicit `cbw` keeps its name,
//!   the 0-operand implicit form becomes `cbw_0`.
//! - AsmJit fixed-register alias operands (`Gp_AX`, `DS_ZSI`, `XMM0`, ...) are
//!   collapsed to their base operand kind (`Gp`, `Mem`, `Vec`); operand tuples
//!   that become duplicates after collapsing are dropped (first one wins).
//! - Besides the abstract-kind impls (`MovEmitter<Gp, Gp>`), impls are generated
//!   for the concrete sized register wrappers (`Gpq`/`Gpd`/`Gpw`/`GpbLo`/`GpbHi`,
//!   `Xmm`/`Ymm`/`Zmm`, ...) so register constants work without dereferencing
//!   (`a.mov(RAX, RBX)`). Only width combinations present in the instruction's
//!   instdb signatures are generated. Immediate positions take `U: Into<Imm>`,
//!   so integer literals work too (`a.mov(RAX, 42)`); when a sized impl would
//!   overlap its abstract `Imm` counterpart, the generic one replaces it.
//! - The conditional families `j`, `set` and `cmov` take an explicit [`CondCode`]
//!   translated through the `JCC_FROM_COND` / `SETCC_FROM_COND` /
//!   `CMOVCC_FROM_COND` tables below, plus one named method per x86 condition
//!   suffix (`jo`, `jno`, ...) that calls a concrete `InstId::{Id}{Suffix}`.
//! - Rust keywords are raw-escaped (`r#loop`, `r#in`); names already C++-escaped
//!   in the input (`and_`, `or_`, `not_`, `xor_`, `int_`) are kept as-is.
//! - Rustdoc lists compact assembly forms derived from pinned AsmJit metadata.
//!   When `asm-docs/` is available, its Intel-derived summary and reference link
//!   are included as well.
#![allow(non_snake_case, non_camel_case_types)]
use super::{assembler::*, instdb::*, operands::*};
use crate::core::globals::CondCode;
use crate::core::operand::*;
"""

TABLE_DESCRIPTIONS = {
    "JCC_FROM_COND": ("J", "a `jcc`"),
    "SETCC_FROM_COND": ("Set", "a `setcc`"),
    "CMOVCC_FROM_COND": ("Cmov", "a `cmovcc`"),
}


def cond_table_lines():
    lines = []
    for table, (prefix, description) in TABLE_DESCRIPTIONS.items():
        lines.append(f"/// Translates asmkit's ARM-ordered [`CondCode`] to {description}")
        lines.append("/// instruction id, reindexing AsmJit's x86-ordered")
        lines.append(f"/// `_{table.lower()}_table` (x86globals.h). Conditions with no x86")
        lines.append("/// form map to `InstId::None`.")
        lines.append(f"pub(crate) const {table}: [u32; 16] = [")
        for cond in ASMKIT_COND_ORDER:
            suffix = ASMKIT_COND_TO_X86.get(cond)
            if cond == "AL" and prefix == "J":
                entry, note = "InstId::Jmp", "unconditional jump"
            elif suffix is None:
                entry, note = "InstId::None", "no x86 equivalent"
            else:
                entry, note = f"InstId::{prefix}{suffix}", None
            comment = f"{cond}: {note}" if note else cond
            lines.append(f"    {entry} as u32, // {comment}")
        lines.append("];")
        lines.append("")
    return lines


def width_forms(opcode, variant, width_db):
    """Signature forms for one emitter variant. Conditional families
    (j/set/cmov) have no InstId of their own: merge the forms of the sixteen
    concrete condition instructions, deduplicated."""
    if opcode.conv:
        prefix = CONV_TABLES[opcode.conv][1]
        seen, forms = set(), []
        for suffix in X86_CC_SUFFIXES:
            for form in width_db.get(f"{prefix}{suffix}", []):
                if form not in seen:
                    seen.add(form)
                    forms.append(form)
        return forms
    return width_db.get(variant.inst_id, [])


def generate(opcodes, opcode_docs, asmjit_forms, width_db):
    """Returns (file_text, {opcode name: {"trait": str, "impls": [str]}}, gen
    stats) for spot printing."""
    blocks = {}
    traits, impls = [], []
    gen_stats = {"typed_impls": 0, "generic_imm_impls": 0, "replaced_abstract": 0}
    impl_keys = []
    for opcode in opcodes.values():
        opcode_impls = []
        for variant in opcode.variants:
            typed = typed_operand_tuples(variant, width_forms(opcode, variant, width_db))
            generalized = tuple(IMM_GENERIC if op == "Imm" else op
                                for op in variant.operands)
            replace = generalized in typed and generalized != tuple(variant.operands)
            if replace:
                gen_stats["replaced_abstract"] += 1
            else:
                opcode_impls.append(
                    impl_block(opcode, variant.inst_id, variant.operands, variant.comment,
                               asmjit_forms))
                impl_keys.append((trait_name(opcode.name), tuple(variant.operands)))
            for combo in typed:
                if combo == tuple(variant.operands):
                    continue
                opcode_impls.append(
                    impl_block(opcode, variant.inst_id, combo, variant.comment,
                               asmjit_forms))
                impl_keys.append((trait_name(opcode.name), combo))
                gen_stats["typed_impls"] += 1
                if IMM_GENERIC in combo:
                    gen_stats["generic_imm_impls"] += 1
        block = {
            "trait": trait_block(opcode, opcode_docs, asmjit_forms),
            "impls": opcode_impls,
        }
        blocks[opcode.name] = block
        traits.append(block["trait"])
        impls.extend(block["impls"])

    parts = [HEADER]
    parts.extend(cond_table_lines())
    parts.append("\n\n".join(traits))
    parts.append("\n\n".join(impls))
    return "\n".join(parts) + "\n", blocks, gen_stats, impl_keys


def validate(opcodes, inst_id_names, impl_keys, text):
    """Cross-checks the generated references against the parsed InstId enum.
    Returns a list of problem strings (empty when everything checks out)."""
    problems = []

    referenced = {"Jmp", "None"}
    for opcode in opcodes.values():
        if opcode.conv:
            prefix = CONV_TABLES[opcode.conv][1]
            referenced.update(f"{prefix}{suffix}" for suffix in X86_CC_SUFFIXES)
        else:
            referenced.update(variant.inst_id for variant in opcode.variants)
    missing = sorted(referenced - inst_id_names)
    if missing:
        problems.append(f"InstId variants referenced but not in x86globals.h: {missing}")

    seen = set()
    duplicates = []
    for key in impl_keys:
        if key in seen:
            duplicates.append(key)
        seen.add(key)
    if duplicates:
        problems.append(f"duplicate (trait, operands) impls: {duplicates}")

    trait_owners = {}
    for name in opcodes:
        trait_owners.setdefault(trait_name(name), []).append(name)
    trait_collisions = {t: ns for t, ns in trait_owners.items() if len(ns) > 1}
    if trait_collisions:
        problems.append(f"trait name collisions: {trait_collisions}")

    method_owners = {}
    for opcode in opcodes.values():
        method = rust_method_name(opcode.name)
        names = [method]
        if opcode.conv:
            names += [f"{method}{suffix}" for suffix in X86_CC_SUFFIXES]
        for name in names:
            method_owners.setdefault(name, []).append(opcode.name)
    method_collisions = {m: ns for m, ns in method_owners.items() if len(ns) > 1}
    if method_collisions:
        problems.append(f"inherent method name collisions: {method_collisions}")

    if re.search(r"\bop\d+\s*:", text):
        problems.append("generated public signatures contain opN placeholders")
    if "Assembly forms:" not in text:
        problems.append("generated Rustdoc is missing AsmJit assembly forms")

    return problems


def print_report(opcodes, stats, gen_stats, inst_id_names, problems):
    traits = len(opcodes)
    impls = sum(len(block["impls"]) for block in gen_stats["blocks"].values())
    cc_traits = sum(1 for opcode in opcodes.values() if opcode.conv)
    methods = sum((17 if opcode.conv else 1) for opcode in opcodes.values())
    splits = traits - stats["distinct_names"]

    print(f"Parsed {stats['decls']} declarations "
          f"({stats['distinct_names']} distinct mnemonics, "
          f"{stats['cc_decls']} conditional-family declarations)")
    print(f"Parsed {len(inst_id_names)} InstId enum names from x86globals.h")
    print(f"Generated {traits} traits ({splits} arity-split), "
          f"{impls} impls ({stats['dropped_duplicates']} alias-collapse duplicates dropped), "
          f"{methods} trait methods ({cc_traits} cc traits x 17)")
    print(f"Sized impls: {gen_stats['typed_impls']} typed "
          f"({gen_stats['generic_imm_impls']} with Into<Imm> immediates), "
          f"{gen_stats['replaced_abstract']} abstract Imm impls replaced by generic ones")
    for key in ("unknown_tokens", "arity_mismatches", "conv_mismatches"):
        for entry in stats[key]:
            print(f"WARNING: {entry}")
    if problems:
        print("VALIDATION FAILURES:")
        for problem in problems:
            print(f"  - {problem}")
    else:
        print("Validation: all referenced InstId variants exist, no unknown "
              "operand tokens, no duplicate impls, no name collisions")


def print_spot_checks(opcodes, blocks):
    for spot in SPOT_CHECK_NAMES:
        matches = [name for name in opcodes
                   if name == spot or name.startswith(f"{spot}_")]
        print(f"\n=== {spot} ({', '.join(matches)}) ===")
        for name in matches:
            block = blocks[name]
            print(block["trait"])
            print()
            for impl in block["impls"]:
                print(impl)
                print()


def main():
    parser = argparse.ArgumentParser(
        description="Generates src/x86/emitter.rs from meta/x86_emitter.txt")
    parser.add_argument("output", help="path of the generated Rust file")
    parser.add_argument("--docs-inputfolder", default=DOCS_INPUT,
                        help="docenizer HTML input folder "
                             "(env: ASMKIT_X86_DOCS, default: asm-docs)")
    parser.add_argument("--check", action="store_true",
                        help="print the validation report and spot-check examples")
    args = parser.parse_args()

    opcodes, stats = parse_opcodes(INPUT_PATH)
    inst_id_names = parse_inst_id_names(X86GLOBALS_PATH)
    opcode_docs = load_opcode_docs(args.docs_inputfolder)
    asmjit_forms = load_asmjit_forms(X86_ISA_PATH)
    width_db = load_width_db()
    text, blocks, gen_stats, impl_keys = generate(
        opcodes, opcode_docs, asmjit_forms, width_db)
    gen_stats["blocks"] = blocks

    output_dir = os.path.dirname(os.path.abspath(args.output))
    os.makedirs(output_dir, exist_ok=True)
    with open(args.output, "w") as out:
        out.write(text)
    print(f"Wrote {args.output} ({len(text.splitlines())} lines)")

    problems = validate(opcodes, inst_id_names, impl_keys, text)
    print_report(opcodes, stats, gen_stats, inst_id_names, problems)
    if args.check:
        print_spot_checks(opcodes, blocks)
    if problems:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
