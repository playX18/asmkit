#!/usr/bin/env python3
"""Generates the x86 emitter traits (src/x86/emitter.rs) from meta/x86_emitter.txt.

The input is a curated copy of AsmJit's x86emitter.h ASMJIT_INST_* declarations.
The transformation mirrors meta/arm64.py (aarch64):

* one `pub trait {Camel}Emitter<T0..Tn>` per (mnemonic, arity) pair;
* one `impl {Camel}Emitter<...> for Assembler<'_>` per concrete operand tuple,
  forwarding to `self.emit_n(InstId::{Id}, &[op0.as_operand(), ...])`;
* inherent forwarding methods on `Assembler<'_>` with `where Self: ...` bounds;
* when a mnemonic is declared with several arities, the first-seen arity keeps
  the plain name and later ones are suffixed (`cbw` / `cbw_0`);
* doc comments come from the docenizer database (meta/docenizer_amd64.py).

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

Usage: python3 meta/x86_emitter_gen.py [--docs-inputfolder DIR] [--check] OUTPUT
"""

import argparse
import os
import re

from docenizer_amd64 import collect_instruction_docs

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
INPUT_PATH = os.path.join(SCRIPT_DIR, "x86_emitter.txt")
X86GLOBALS_PATH = os.path.join(SCRIPT_DIR, "asmjit", "asmjit", "x86", "x86globals.h")

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


def doc_lines(name, opcode_docs, indent=""):
    canonical = doc_lookup_name(name)
    doc = opcode_docs.get(canonical)
    if doc:
        tooltip = " ".join(doc["tooltip"].split())
        lines = [f"{indent}/// Emits `{name.upper()}` (`{canonical}`). {tooltip}"]
        if doc.get("url"):
            lines.append(f"{indent}/// Reference: [Intel docs for {canonical}]({doc['url']})")
        return lines
    return [f"{indent}/// Emits `{name.upper()}`."]


def fn_params(operands, cc=False):
    parts = ["&mut self"]
    if cc:
        parts.append("cc: CondCode")
    parts.extend(f"op{i}: {operand}" for i, operand in enumerate(operands))
    return ", ".join(parts)


def trait_block(opcode, opcode_docs):
    n = len(opcode.variants[0].operands)
    generics = f"<{', '.join(f'T{i}' for i in range(n))}>" if n else ""
    generic_operands = [f"T{i}" for i in range(n)]
    method = rust_method_name(opcode.name)

    lines = doc_lines(opcode.name, opcode_docs)
    lines.append(f"pub trait {trait_name(opcode.name)}Emitter{generics} {{")
    if opcode.conv:
        lines.append(f"    fn {method}({fn_params(generic_operands, cc=True)});")
        for suffix in X86_CC_SUFFIXES:
            lines.append(f"    fn {method}{suffix}({fn_params(generic_operands)});")
    else:
        lines.append(f"    fn {method}({fn_params(generic_operands)});")
    lines.append("}")
    return "\n".join(lines)


def impl_block(opcode, variant):
    trait = trait_name(opcode.name)
    generics = f"<{', '.join(variant.operands)}>" if variant.operands else ""
    method = rust_method_name(opcode.name)
    ops = f"&[{', '.join(f'op{i}.as_operand()' for i in range(len(variant.operands)))}]"
    comment = f" // {variant.comment}" if variant.comment else ""

    lines = [f"impl {trait}Emitter{generics} for Assembler<'_> {{"]
    if opcode.conv:
        table, prefix = CONV_TABLES[opcode.conv]
        lines.append(f"    fn {method}({fn_params(variant.operands, cc=True)}) {{")
        lines.append(f"        self.emit_n({table}[cc as usize], {ops});{comment}")
        lines.append("    }")
        for suffix in X86_CC_SUFFIXES:
            lines.append(f"    fn {method}{suffix}({fn_params(variant.operands)}) {{")
            lines.append(f"        self.emit_n(InstId::{prefix}{suffix}, {ops});")
            lines.append("    }")
    else:
        lines.append(f"    fn {method}({fn_params(variant.operands)}) {{")
        lines.append(f"        self.emit_n(InstId::{variant.inst_id}, {ops});{comment}")
        lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def inherent_blocks(opcode, opcode_docs):
    n = len(opcode.variants[0].operands)
    generics = ", ".join(f"T{i}" for i in range(n))
    trait = trait_name(opcode.name)
    trait_ref = f"{trait}Emitter<{generics}>" if n else f"{trait}Emitter"
    generic_params = f"<{generics}>" if n else ""
    generic_operands = [f"T{i}" for i in range(n)]
    method = rust_method_name(opcode.name)
    call_args = ", ".join(f"op{i}" for i in range(n))

    def forwarder(doc_name, method_name, cc):
        call = f"cc, {call_args}" if cc and call_args else ("cc" if cc else call_args)
        lines = doc_lines(doc_name, opcode_docs, indent="    ")
        lines.append(f"    pub fn {method_name}{generic_params}"
                     f"({fn_params(generic_operands, cc=cc)})")
        lines.append("    where")
        lines.append(f"        Self: {trait_ref},")
        lines.append("    {")
        lines.append(f"        <Self as {trait_ref}>::{method_name}(self"
                     f"{', ' if call else ''}{call});")
        lines.append("    }")
        return "\n".join(lines)

    if opcode.conv:
        blocks = [forwarder(opcode.name, method, cc=True)]
        blocks.extend(forwarder(f"{opcode.name}{suffix}", f"{method}{suffix}", cc=False)
                      for suffix in X86_CC_SUFFIXES)
        return blocks
    return [forwarder(opcode.name, method, cc=False)]


HEADER = """\
//! X86 emitter traits, generated by `meta/x86_emitter_gen.py` from
//! `meta/x86_emitter.txt` (AsmJit's `x86emitter.h` declarations). Do not edit by
//! hand; regenerate instead.
//!
//! Conventions:
//! - One `{Name}Emitter<T0..Tn>` trait per (mnemonic, arity) pair. `Assembler<'_>`
//!   implements it once per declared concrete operand tuple, forwarding to
//!   `emit_n(InstId::{Id}, &[...])`; an inherent method per trait method forwards
//!   to the trait implementation through a `where Self: {Name}Emitter<...>` bound.
//! - Both `[EXPLICIT]` (fixed-register operands spelled out) and `[IMPLICIT]`
//!   (fixed registers hidden) input declarations are kept. When a mnemonic is
//!   declared with more than one arity, the first-seen arity keeps the plain name
//!   and later arities are suffixed: the 1-operand explicit `cbw` keeps its name,
//!   the 0-operand implicit form becomes `cbw_0`.
//! - AsmJit fixed-register alias operands (`Gp_AX`, `DS_ZSI`, `XMM0`, ...) are
//!   collapsed to their base operand kind (`Gp`, `Mem`, `Vec`); operand tuples
//!   that become duplicates after collapsing are dropped (first one wins).
//! - The conditional families `j`, `set` and `cmov` take an explicit [`CondCode`]
//!   translated through the `JCC_FROM_COND` / `SETCC_FROM_COND` /
//!   `CMOVCC_FROM_COND` tables below, plus one named method per x86 condition
//!   suffix (`jo`, `jno`, ...) that calls a concrete `InstId::{Id}{Suffix}`.
//! - Rust keywords are raw-escaped (`r#loop`, `r#in`); names already C++-escaped
//!   in the input (`and_`, `or_`, `not_`, `xor_`, `int_`) are kept as-is.
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


def generate(opcodes, opcode_docs):
    """Returns (file_text, {opcode name: {"trait": str, "impls": [str],
    "inherent": [str]}}) for spot printing."""
    blocks = {}
    traits, impls, inherent = [], [], []
    for opcode in opcodes.values():
        block = {
            "trait": trait_block(opcode, opcode_docs),
            "impls": [impl_block(opcode, variant) for variant in opcode.variants],
            "inherent": inherent_blocks(opcode, opcode_docs),
        }
        blocks[opcode.name] = block
        traits.append(block["trait"])
        impls.extend(block["impls"])
        inherent.extend(block["inherent"])

    parts = [HEADER]
    parts.extend(cond_table_lines())
    parts.append("\n\n".join(traits))
    parts.append("\n\n".join(impls))
    parts.append("impl Assembler<'_> {\n" + "\n\n".join(inherent) + "\n}")
    return "\n".join(parts) + "\n", blocks


def validate(opcodes, inst_id_names):
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
    for opcode in opcodes.values():
        trait = trait_name(opcode.name)
        for variant in opcode.variants:
            key = (trait, variant.operands)
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

    return problems


def print_report(opcodes, stats, inst_id_names, problems):
    traits = len(opcodes)
    impls = sum(len(opcode.variants) for opcode in opcodes.values())
    cc_traits = sum(1 for opcode in opcodes.values() if opcode.conv)
    methods = sum((17 if opcode.conv else 1) for opcode in opcodes.values())
    inherent = methods  # one inherent forwarder per trait method
    splits = traits - stats["distinct_names"]

    print(f"Parsed {stats['decls']} declarations "
          f"({stats['distinct_names']} distinct mnemonics, "
          f"{stats['cc_decls']} conditional-family declarations)")
    print(f"Parsed {len(inst_id_names)} InstId enum names from x86globals.h")
    print(f"Generated {traits} traits ({splits} arity-split), "
          f"{impls} impls ({stats['dropped_duplicates']} alias-collapse duplicates dropped), "
          f"{methods} trait methods ({cc_traits} cc traits x 17), "
          f"{inherent} inherent methods")
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
            for forwarder in block["inherent"]:
                print(forwarder)
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
    text, blocks = generate(opcodes, opcode_docs)

    output_dir = os.path.dirname(os.path.abspath(args.output))
    os.makedirs(output_dir, exist_ok=True)
    with open(args.output, "w") as out:
        out.write(text)
    print(f"Wrote {args.output} ({len(text.splitlines())} lines)")

    problems = validate(opcodes, inst_id_names)
    print_report(opcodes, stats, inst_id_names, problems)
    if args.check:
        print_spot_checks(opcodes, blocks)
    if problems:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
