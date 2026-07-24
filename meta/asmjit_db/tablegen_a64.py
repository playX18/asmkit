#!/usr/bin/env python3
# This file is part of asmkit.
#
# Computes the AArch64 instruction-database tables from the vendored asmjit
# sources (meta/asmjit) and emits them as Rust, content-equal to
# src/aarch64/instdb.rs.

"""AArch64 table generator (port of the a64 side of asmjit's tablegen flow).

Inputs (all vendored, read-only):

  * `asmjit/arm/a64globals.h`   - `InstId` enum order + doc comments.
  * `asmjit/arm/a64instdb_p.h`  - `EncodingId` enum + encoding-data fwd decls.
  * `asmjit/arm/a64instdb.cpp`  - `INST()` rows + encoding-data tables
                                  (the pinned output of asmjit's JS tablegen).
  * `db/isa_aarch64.json`       - per-form `io` attributes (NZCV effects),
                                  used for the asmkit-specific RW_FLAGS_TABLE.

Outputs (under `meta/asmjit_db/out/`):

  * `aarch64_instdb.rs`   - content-equal to `src/aarch64/instdb.rs`
                            (`--check` compares them).
  * `aarch64_rw_flags.rs` - NZCV (PSTATE) flag effects per `InstId`, computed
                            from the db `io` fields; an asmkit extension that
                            has no counterpart in asmjit's generated tables.

Name tables are computed from the `InstId` list with
`tablegen.InstructionNameData` (the byte-exact port of the JS algorithm), not
parsed back from the generated C++.

Hand-adapted Rust scaffolding (the pieces that are not instruction tables)
lives in `meta/asmjit_db/templates/` and is spliced verbatim.
"""

import argparse
import os
import re
import sys

if __package__ in (None, ""):
    # Direct execution (`python3 meta/asmjit_db/tablegen_a64.py`).
    sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
    __package__ = "meta.asmjit_db"

from . import base, tablegen
from .cxx_src import (
    OUT_DIR,
    check,
    check_ordinals,
    decode_c_string_table,
    extract_block,
    fail,
    map_flag_expr,
    parse_brace_row,
    parse_inst_id_block,
    parse_macro_invocation,
    read_asmjit,
    read_db_json,
    read_template,
    rust_byte_string,
    screaming,
    split_line_rows,
    split_tables,
    strip_generated_banner,
    strip_trailing_comma,
    token_diff,
)

A64_INST_ID_COUNT = 776
A64_ENCODING_COUNT = 95  # AsmJit variants; the Rust enum appends `Count`.
A64_ENCODING_TABLE_COUNT = 83

SRC_RS = "src/aarch64/instdb.rs"

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


def a64_table_name(cpp_name):
    if cpp_name in A64_TABLE_NAME_OVERRIDES:
        return A64_TABLE_NAME_OVERRIDES[cpp_name]
    return screaming(cpp_name)


class A64Db:
    """The parsed vendored AArch64 sources."""

    def __init__(self):
        globals_h = read_asmjit("arm/a64globals.h")
        instdb_p_h = read_asmjit("arm/a64instdb_p.h")
        instdb_cpp = read_asmjit("arm/a64instdb.cpp")

        self.inst_ids, aliases = parse_inst_id_block(
            extract_block(globals_h, "InstId"), "kId")
        check(not aliases, "a64 InstId block must not contain aliases")
        check(len(self.inst_ids) == A64_INST_ID_COUNT,
              f"a64: expected {A64_INST_ID_COUNT} InstId values, got {len(self.inst_ids)}")

        enc_block = extract_block(instdb_p_h, "EncodingId")
        self.enc_variants = []
        for raw in enc_block.splitlines():
            code = raw.strip()
            if not code or code.startswith("//") or code.startswith("enum") or code in ("{", "};"):
                continue
            m = re.match(r"([A-Za-z_][A-Za-z_0-9]*)\s*(?:=\s*\d+\s*)?,?\s*$", code)
            check(m is not None, f"cannot parse EncodingId line: {raw!r}")
            name = m.group(1)
            check(name.startswith("kEncoding"), f"unexpected EncodingId name {name!r}")
            self.enc_variants.append(name[len("kEncoding"):].replace("_", ""))
        check(len(self.enc_variants) == A64_ENCODING_COUNT,
              f"a64: expected {A64_ENCODING_COUNT} encodings, got {len(self.enc_variants)}")
        check(self.enc_variants[0] == "None", "first encoding must be None")
        self.enc_set = set(self.enc_variants)

        fwd_block = extract_block(instdb_p_h, "EncodingDataForward")
        self.fwd_decls = []  # (cpp_type, cpp_name, size)
        for raw in fwd_block.splitlines():
            m = re.match(r"\s*extern const ([A-Za-z_0-9]+) ([A-Za-z_0-9]+)\[(\d+)\];", raw)
            if m:
                self.fwd_decls.append((m.group(1), m.group(2), int(m.group(3))))
        check(len(self.fwd_decls) == A64_ENCODING_TABLE_COUNT,
              f"a64: expected {A64_ENCODING_TABLE_COUNT} forward declarations, got {len(self.fwd_decls)}")

        inst_block = extract_block(instdb_cpp, "InstInfo")
        self.inst_rows = []
        for code, comment in split_line_rows(strip_generated_banner(inst_block)):
            name, args = parse_macro_invocation(code)
            check(name == "INST", f"a64: unexpected row macro {name!r}")
            check(len(args) == 6, f"a64: INST row with {len(args)} args: {code!r}")
            self.inst_rows.append({
                "id": args[0],
                "encoding": args[1],
                "opcode_data": args[2],
                "rw_info_index": args[3],
                "flags": args[4],
                "opcode_data_index": args[5],
                "comment": comment,
            })
        check(len(self.inst_rows) == A64_INST_ID_COUNT,
              f"a64: expected {A64_INST_ID_COUNT} INST rows, got {len(self.inst_rows)}")
        check_ordinals([r["comment"] for r in self.inst_rows], "a64 _inst_info_table")
        for i, row in enumerate(self.inst_rows):
            check(row["id"] == self.inst_ids[i][0],
                  f"a64: INST row #{i} id {row['id']!r} does not match InstId {self.inst_ids[i][0]!r}")
            check(row["encoding"].replace("_", "") in self.enc_set,
                  f"a64: INST row #{i} uses unknown encoding {row['encoding']!r}")
            check(re.fullmatch(r"kRWI_[A-Za-z_0-9]+|0", row["rw_info_index"]) is not None,
                  f"a64: INST row #{i} bad rw_info_index {row['rw_info_index']!r}")
            check(re.fullmatch(r"0|F\([A-Za-z_0-9]+\)(\s*\|\s*F\([A-Za-z_0-9]+\))*", row["flags"]) is not None,
                  f"a64: INST row #{i} bad flags {row['flags']!r}")
            check(re.fullmatch(r"\d+", row["opcode_data_index"]) is not None,
                  f"a64: INST row #{i} bad opcode_data_index {row['opcode_data_index']!r}")

        enc_data_block = extract_block(instdb_cpp, "EncodingData")
        cpp_tables = split_tables(strip_generated_banner(enc_data_block))
        check(len(cpp_tables) == A64_ENCODING_TABLE_COUNT,
              f"a64: expected {A64_ENCODING_TABLE_COUNT} encoding-data tables, got {len(cpp_tables)}")
        fwd_names = {name for _, name, _ in self.fwd_decls}
        check(set(cpp_tables) == fwd_names,
              f"a64: table/forward-declaration mismatch: {set(cpp_tables) ^ fwd_names}")

        self.enc_tables = {}  # cpp_name -> [(args, comment)]
        for cpp_type, cpp_name, size in self.fwd_decls:
            rows = []
            for code, comment in split_line_rows(cpp_tables[cpp_name]):
                rows.append((parse_brace_row(code), comment))
            check(len(rows) == size,
                  f"a64: table {cpp_name} has {len(rows)} rows, declared {size}")
            self.enc_tables[cpp_name] = rows

        self.name_data = tablegen.InstructionNameData()
        for enum_name, _ in self.inst_ids:
            name = "" if enum_name == "None" else enum_name.lower()
            # a64 displays ASIMD instructions without the `_v` suffix.
            display = name[:-2] if name.endswith("_v") else name
            self.name_data.add(display)
        self.name_data.index()

        # Cross-check the computed name data against the pinned C++ output.
        name_block = extract_block(instdb_cpp, "NameData")
        string_m = re.search(
            r"const char InstDB::_inst_name_string_table\[\] =\s*(.*?);", name_block, re.S)
        check(string_m is not None, "a64: _inst_name_string_table not found")
        check(decode_c_string_table(string_m.group(1)).decode("latin-1") == self.name_data.string_table,
              "a64: computed name string table differs from the pinned C++ table")
        index_tables = split_tables(strip_generated_banner(name_block))
        cpp_index_rows = split_line_rows(index_tables["_inst_name_index_table"])
        check(len(cpp_index_rows) == A64_INST_ID_COUNT,
              f"a64: expected {A64_INST_ID_COUNT} name index rows, got {len(cpp_index_rows)}")
        for i, (code, comment) in enumerate(cpp_index_rows):
            value, _ = strip_trailing_comma(code)
            check(int(value, 16) == self.name_data.primary_table[i],
                  f"a64: computed name index row #{i} differs from the pinned C++ table")
            check(comment == self.name_data.index_comment[i],
                  f"a64: computed name index comment #{i} differs from the pinned C++ table")


def _map_data_arg(arg):
    arg = re.sub(r"\bOffsetType::kAArch64_ADR\b", "OffsetType::Adr as u8", arg)
    arg = re.sub(r"\bOffsetType::kAArch64_ADRP\b", "OffsetType::Adrp as u8", arg)
    arg = re.sub(r"\bInst::kId([A-Za-z_0-9]+)\b", r"InstId::\1", arg)
    return arg


def emit_a64(db):
    """The content of `aarch64_instdb.rs`."""
    out = [read_template("a64_head.txt")]

    # Encoding enum.
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum Encoding {\n")
    out.append("    None = 0,\n")
    for name in db.enc_variants[1:]:
        out.append(f"    {name},\n")
    out.append("\n    Count,\n}\n\n")

    out.append(read_template("a64_mid.txt"))

    # INST_INFO_TABLE.
    out.append("TABLE!(INST_INFO_TABLE\n\n  = {\n")
    out.append(A64_INST_INFO_HEADER)
    for i, row in enumerate(db.inst_rows):
        encoding = row["encoding"].replace("_", "")
        # The INST! macro discards opcode_data; the hand port still maps
        # Inst::kIdXxx -> InstId::Xxx inside it (OffsetType stays verbatim).
        opcode_data = re.sub(r"\bInst::kId([A-Za-z_0-9]+)\b", r"InstId::\1", row["opcode_data"])
        rw = re.sub(r"\bkRWI_([A-Za-z_0-9]+)", lambda m: "RWI_" + m.group(1).upper(), row["rw_info_index"])
        flags = map_flag_expr(row["flags"], lambda f: f"F!({f[2:-1]})" if f.startswith("F(") else f)
        out.append(
            f"INST({row['id']}, {encoding}, {opcode_data}, {rw}, {flags}, {row['opcode_data_index']}), // #{i}\n")
    out.append("\n  });\n\n")

    # First encoding-data table (plain array form in the hand port).
    first_type, first_name, _ = db.fwd_decls[0]
    check(first_name == "baseAddSub", "a64: first table must be baseAddSub")
    rust_first_type = first_type.replace("_", "")
    first_rows = db.enc_tables[first_name]
    out.append(f"pub const {a64_table_name(first_name)}: [{rust_first_type}; {len(first_rows)}] = [\n")
    for args, comment in first_rows:
        mapped = ", ".join(_map_data_arg(a) for a in args)
        suffix = f" // {comment}" if comment else ""
        out.append(f"    {rust_first_type}::new({mapped}),{suffix}\n")
    out.append("];\n\n")

    out.append(read_template("a64_table_new.txt"))

    # Remaining encoding-data tables via table_new!.
    for cpp_type, cpp_name, _ in db.fwd_decls[1:]:
        rust_type = cpp_type.replace("_", "")
        rows = db.enc_tables[cpp_name]
        out.append(f"pub const {a64_table_name(cpp_name)}: [{rust_type}; {len(rows)}] = table_new!({rust_type}, {{\n")
        for idx, (args, comment) in enumerate(rows):
            if (cpp_name, idx) in A64_CAST_ROWS:
                check(len(args) == 1 and args[0].startswith("0b"),
                      f"a64: unexpected cast-row shape in {cpp_name} row {idx}")
                args = [args[0] + "u32 as i32"]
            mapped = ", ".join(_map_data_arg(a) for a in args)
            comma = "," if idx + 1 < len(rows) else " "
            suffix = f" // {comment}" if comment else ""
            out.append(f"    {{ {mapped} }}{comma}{suffix}\n")
        out.append("});\n\n")

    # Name tables (computed; the hand port omits the 26 span entries).
    out.append(f"pub static INST_NAME_STRING_TABLE: &[u8] = {rust_byte_string(db.name_data.string_table.encode('latin-1'))};\n")
    out.append("#[rustfmt::skip]\n")
    out.append("pub static INST_NAME_INDEX_TABLE: &[u32] = &[\n")
    count = len(db.name_data.primary_table)
    for idx in range(count):
        comma = "," if idx + 1 < count else ""
        comment = db.name_data.index_comment[idx]
        suffix = f" // {comment}" if comment else ""
        out.append(f"    0x{db.name_data.primary_table[idx]:08X}{comma}{suffix}\n")
    out.append("];\n\n")

    # InstId enum.
    out.append("#[rustfmt::skip]\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[allow(non_camel_case_types)]\n")
    out.append("#[repr(u32)]\n")
    out.append("pub enum InstId {\n")
    out.append(f"    None = 0, // {db.inst_ids[0][1]}\n")
    for name, doc in db.inst_ids[1:]:
        out.append(f"    {name}, // {doc}\n")
    out.append("    _Count\n}\n")

    result = "".join(out)

    # Unexpanded-token self-checks.
    for forbidden in ("kRWI_", "Inst::kId", "uint32_t(", "uint16_t(", "uint8_t("):
        check(forbidden not in result, f"a64: unexpanded token {forbidden!r} left in output")
    check(re.search(r"\bF\(", result) is None, "a64: unexpanded F() flag macro left in output")
    return result


# CpuRwFlags bit values (src/core/rwinfo.rs): A64_V=OF, A64_C=CF, A64_Z=ZF,
# A64_N=SF reuse the common bits; A64_Q is the saturation flag.
A64_FLAG_BITS = {
    "N": ("A64_N", 0x0008),
    "Z": ("A64_Z", 0x0004),
    "C": ("A64_C", 0x0002),
    "V": ("A64_V", 0x0001),
    "QC": ("A64_Q", 0x0100),
}

# Canonical flag order in emitted expressions.
A64_FLAG_ORDER = ["A64_N", "A64_Z", "A64_C", "A64_V", "A64_Q"]


def parse_io_flags(io_string):
    """One db `io` string -> (read, write) CpuRwFlags bit sets.

    `R` = read, `W` = write, `X` = read+write; `QC=SAT` (emitted by the JS
    parser as key `QC` with value `SAT` plus an empty key from the trailing
    pipe) is a conditional write of the saturation flag A64_Q.
    """
    inst = base.Instruction()
    inst._assign_attribute("io", io_string)

    read_flags = set()
    write_flags = set()
    for key, value in inst.io.items():
        if not key:
            continue  # Quirk of the `QC|=SAT` token (see base.py io parser).
        entry = A64_FLAG_BITS.get(key)
        check(entry is not None, f"a64 io: unknown flag key {key!r} in {io_string!r}")
        rust_name, bit = entry
        if key == "QC":
            check(value == "SAT", f"a64 io: unexpected QC value {value!r} in {io_string!r}")
            write_flags.add(rust_name)
            continue
        check(value in ("R", "W", "X"), f"a64 io: bad access {value!r} in {io_string!r}")
        if value in ("R", "X"):
            read_flags.add(rust_name)
        if value in ("W", "X"):
            write_flags.add(rust_name)
    return frozenset(read_flags), frozenset(write_flags)


def load_a64_io_flags():
    """Instruction name -> (read, write) CpuRwFlags sets, union of all db forms."""
    data = read_db_json("isa_aarch64.json")
    flags = {}
    for group in data["instructions"]:
        for entry in group["data"]:
            io_string = entry.get("io")
            if not io_string:
                continue
            read_flags, write_flags = parse_io_flags(io_string)
            # `b.<cond>` is the db spelling of the conditional B/BC forms;
            # `asr|asrv`-style names share one entry between several ids.
            name = entry["inst"].split(" ")[0].replace(".<cond>", "")
            for alias in name.split("|"):
                r, w = flags.get(alias, (frozenset(), frozenset()))
                flags[alias] = (r | read_flags, w | write_flags)
    return flags


def _rw_flags_expr(flag_set):
    if not flag_set:
        return "0"
    return " | ".join(f"CpuRwFlags::{name}.bits()" for name in A64_FLAG_ORDER if name in flag_set)


def emit_a64_rw_flags(db, io_flags):
    """The content of `aarch64_rw_flags.rs` (NZCV effects per `InstId`)."""
    out = []
    out.append("// [AsmKit] AArch64 NZCV (PSTATE) flag effects per instruction, indexed by\n")
    out.append("// [`InstId`]. Computed from the AsmJit instruction database `io` attributes\n")
    out.append("// (meta/asmjit/db/isa_aarch64.json) by meta/asmjit_db/tablegen_a64.py;\n")
    out.append("// AsmJit does not expose these through its generated tables.\n\n")
    out.append("use crate::core::rwinfo::CpuRwFlags;\n\n")
    out.append("/// (read_flags, write_flags) pairs of [`CpuRwFlags`] bits per instruction.\n")
    out.append("pub static RW_FLAGS_TABLE: &[(u32, u32)] = &[\n")

    seen_read = set()
    seen_write = set()
    for i, (enum_name, _) in enumerate(db.inst_ids):
        name = "" if enum_name == "None" else enum_name.lower()
        db_name = name[:-2] if name.endswith("_v") else name
        read_flags, write_flags = io_flags.get(db_name, (frozenset(), frozenset()))
        seen_read |= read_flags
        seen_write |= write_flags
        out.append(f"    ({_rw_flags_expr(read_flags)}, {_rw_flags_expr(write_flags)}), // #{i}\n")
    out.append("];\n")

    result = "".join(out)
    check(re.search(r"\bF\(", result) is None, "a64 rw_flags: unexpanded F() flag macro left in output")
    return result, seen_read, seen_write


def build():
    db = A64Db()
    io_flags = load_a64_io_flags()
    rw_text, _, _ = emit_a64_rw_flags(db, io_flags)
    return emit_a64(db), rw_text


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true",
                        help=f"compare the generated instdb with {SRC_RS} (read-only) and exit")
    parser.add_argument("--stdout", action="store_true",
                        help="print the generated instdb to stdout instead of writing files")
    args = parser.parse_args(argv)

    instdb_text, rw_text = build()

    if args.stdout:
        sys.stdout.write(instdb_text)
        return 0

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    instdb_out = OUT_DIR / "aarch64_instdb.rs"
    rw_out = OUT_DIR / "aarch64_rw_flags.rs"
    instdb_out.write_text(instdb_text, encoding="utf-8")
    rw_out.write_text(rw_text, encoding="utf-8")
    print(f"{instdb_out}: written ({len(instdb_text)} bytes)")
    print(f"{rw_out}: written ({len(rw_text)} bytes)")

    if args.check:
        repo_rs = (OUT_DIR.parent.parent.parent / SRC_RS).resolve()
        if not repo_rs.is_file():
            fail(f"--check needs {repo_rs}")
        expected = repo_rs.read_text(encoding="utf-8")
        if instdb_text == expected:
            print(f"aarch64: byte-exact match with {SRC_RS}")
            return 0
        diffs = token_diff(instdb_text, expected)
        if not diffs:
            print(f"aarch64: token-equal to {SRC_RS} (not byte-exact)")
            return 0
        print(f"aarch64: output differs from {SRC_RS}:", file=sys.stderr)
        for d in diffs:
            print(d, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
