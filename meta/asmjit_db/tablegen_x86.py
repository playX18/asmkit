#!/usr/bin/env python3
# This file is part of asmkit.
#
# Computes the X86 instruction-database tables from the vendored asmjit
# sources (meta/asmjit) and emits them as Rust, content-equal to
# src/x86/instdb.rs.

"""X86 table generator (port of the x86 side of asmjit's tablegen flow).

Inputs (all vendored, read-only):

  * `asmjit/x86/x86globals.h`  - `InstId` enum order, aliases, doc comments.
  * `asmjit/x86/x86instdb.h`   - Mode / OpFlags / InstFlags / Avx512Flags enums.
  * `asmjit/x86/x86instdb_p.h` - EncodingId + RW-info category/flags enums.
  * `asmjit/x86/x86opcode_p.h` - opcode bit-field constants for O()/V()/E().
  * `asmjit/core/cpuinfo.h`    - `CpuFeatures::X86` feature ids + docs.
  * `asmjit/x86/x86instdb.cpp` - the pinned output of asmjit's JS tablegen:
                                 INST() rows and all precomputed tables.
  * `db/isa_x86.json`          - alias names/format strings, used to compute
                                 the name data (not parsed back from C++).

Output (under `meta/asmjit_db/out/`):

  * `x86_instdb.rs` - content-equal to `src/x86/instdb.rs`
                      (`--check` compares them).

Name tables are computed from the `InstId` list and the db alias formats with
`tablegen.InstructionNameData` (the byte-exact port of the JS algorithm);
everything else is parsed from the pinned generated C++ tables and re-emitted
as Rust. Hand-adapted Rust scaffolding (POD struct ports) lives in
`meta/asmjit_db/templates/` and is spliced verbatim.
"""

import argparse
import os
import re
import sys

if __package__ in (None, ""):
    # Direct execution (`python3 meta/asmjit_db/tablegen_x86.py`).
    sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
    __package__ = "meta.asmjit_db"

from . import tablegen
from .gen_common import StringUtils
from .cxx_src import (
    OUT_DIR,
    check,
    check_ordinals,
    decode_c_string_table,
    emit_bitflags_value,
    enum_body,
    eval_cpp_int,
    extract_block,
    fail,
    map_flag_expr,
    ordinal_of,
    parse_brace_row,
    parse_cpp_enum_entries,
    parse_enum_consts,
    parse_inst_id_block,
    parse_int,
    parse_macro_invocation,
    read_asmjit,
    read_db_json,
    read_template,
    rust_byte_string,
    screaming,
    split_line_rows,
    split_tables,
    strip_generated_banner,
    strip_int_suffix,
    strip_k,
    strip_trailing_comma,
    token_diff,
)

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

SRC_RS = "src/x86/instdb.rs"

ZLIB_HEADER = """/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

"""

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


class X86Db:
    """The parsed vendored X86 sources."""

    def __init__(self):
        globals_h = read_asmjit("x86/x86globals.h")
        instdb_h = read_asmjit("x86/x86instdb.h")
        instdb_p_h = read_asmjit("x86/x86instdb_p.h")
        opcode_p_h = read_asmjit("x86/x86opcode_p.h")
        cpuinfo_h = read_asmjit("core/cpuinfo.h")
        self.inst_cpp = read_asmjit("x86/x86instdb.cpp")

        # --- InstId + aliases ---
        self.inst_ids, self.aliases = parse_inst_id_block(
            extract_block(globals_h, "InstId"), "kId")
        check(len(self.inst_ids) == X86_INST_ID_COUNT,
              f"x86: expected {X86_INST_ID_COUNT} InstId values, got {len(self.inst_ids)}")
        check(len(self.aliases) == X86_ALIAS_COUNT,
              f"x86: expected {X86_ALIAS_COUNT} aliases, got {len(self.aliases)}")

        # --- Enums from x86instdb.h ---
        self.mode_entries = parse_cpp_enum_entries(enum_body(instdb_h, r"enum class Mode : uint8_t"))
        check([e[0] for e in self.mode_entries] == ["kNone", "kX86", "kX64", "kAny"],
              "x86: unexpected Mode enum")
        self.op_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class OpFlags : uint64_t"))
        self.inst_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class InstFlags : uint32_t"))
        self.avx512_flags_entries = parse_cpp_enum_entries(
            enum_body(instdb_h, r"enum class Avx512Flags : uint32_t"))
        self.inst_flags_names = {e[0][1:] for e in self.inst_flags_entries if e[0] != "kNone"}
        self.avx512_flags_names = {e[0][1:] for e in self.avx512_flags_entries if e[0] not in ("kNone", "k_")}
        self.op_flags_names = {e[0][1:] for e in self.op_flags_entries if e[0] != "kNone"}

        # --- Enums from x86instdb_p.h ---
        self.encoding_entries = parse_cpp_enum_entries(
            enum_body(instdb_p_h, r"enum EncodingId : uint32_t"))
        check(len(self.encoding_entries) == X86_ENCODING_COUNT,
              f"x86: expected {X86_ENCODING_COUNT} encodings, got {len(self.encoding_entries)}")
        self.enc_names = [e[0][len("kEncoding"):] for e in self.encoding_entries]
        check(self.enc_names[0] == "None" and self.enc_names[-1] == "Count",
              "x86: EncodingId must start with None and end with Count")
        self.enc_set = set(self.enc_names)

        self.rw_info_category_entries = parse_cpp_enum_entries(
            enum_body(instdb_p_h, r"enum Category : uint8_t"))
        self.rw_info_rm_body = enum_body(instdb_p_h, r"struct RWInfoRm")
        self.rw_info_rm_category_entries = parse_cpp_enum_entries(
            enum_body(self.rw_info_rm_body, r"enum Category : uint8_t"))
        self.rw_info_rm_flags_entries = parse_cpp_enum_entries(
            enum_body(self.rw_info_rm_body, r"enum Flags : uint8_t"))

        # --- Opcode constants ---
        self.opcode_consts = parse_enum_consts(enum_body(opcode_p_h, r"enum Bits : uint32_t"))

        # --- CpuFeatures::X86 ---
        x86_features_body = enum_body(cpuinfo_h, r"struct X86 : public Data")
        feature_entries = parse_cpp_enum_entries(
            enum_body(x86_features_body, r"enum Id : uint8_t"))
        self.cpu_features = []  # (cpp_name, rust_variant)
        self.cpu_feature_docs = {}
        for name, _, doc in feature_entries:
            if name == "kMaxValue":
                break
            variant = strip_k(name)
            self.cpu_features.append((name[1:], variant))
            self.cpu_feature_docs[name[1:]] = doc
        self.cpu_feature_map = dict(self.cpu_features)

        # --- Name data, computed from the InstId list and db alias formats ---
        alias_map = read_db_json("isa_x86.json")["aliases"]
        self.inst_name_data = tablegen.InstructionNameData()
        self.alias_name_data = tablegen.InstructionNameData()
        aliases = []  # {"name", "alt"}
        for enum_name, _ in self.inst_ids:
            name = "" if enum_name == "None" else enum_name.lower()
            alias_data = alias_map.get(name)
            if alias_data:
                self.inst_name_data.add(name, alias_data["format"])
                for alias_name in alias_data["aliases"]:
                    aliases.append({"name": name, "alt": alias_name})
            else:
                self.inst_name_data.add(name)
        aliases.sort(key=lambda alias: alias["alt"])
        self.alias_link = []  # Inst::kId... strings
        for alias in aliases:
            self.alias_name_data.add(alias["alt"])
            self.alias_link.append(f"Inst::kId{StringUtils.make_enum_name(alias['name'])}")
        self.inst_name_data.index()
        self.alias_name_data.index()

        # 26 per-letter spans.
        self.inst_first = [None] * 26
        self.inst_last = [None] * 26
        for enum_name, _ in self.inst_ids:
            name = "" if enum_name == "None" else enum_name.lower()
            if not name:
                continue
            alpha = ord(name[0]) - ord("a")
            check(0 <= alpha < 26, f"x86: invalid lookup character '{name[0]}' of '{name}'")
            if self.inst_first[alpha] is None:
                self.inst_first[alpha] = f"Inst::kId{enum_name}"
            self.inst_last[alpha] = f"Inst::kId{enum_name}"

        self._cross_check_name_data()

    def _cross_check_name_data(self):
        """Asserts the computed name data equals the pinned C++ NameData section."""
        name_block = extract_block(self.inst_cpp, "NameData")
        string_m = re.search(
            r"const char InstDB::_inst_name_string_table\[\] =\s*(.*?);", name_block, re.S)
        check(string_m is not None, "x86: _inst_name_string_table not found")
        check(decode_c_string_table(string_m.group(1)).decode("latin-1") == self.inst_name_data.string_table,
              "x86: computed name string table differs from the pinned C++ table")

        name_tables = split_tables(strip_generated_banner(name_block))
        for var, data, expected in (
            ("_inst_name_index_table", self.inst_name_data, X86_INST_ID_COUNT),
            ("alias_name_index_table", self.alias_name_data, X86_ALIAS_COUNT),
        ):
            rows = split_line_rows(name_tables[var])
            check(len(rows) == expected, f"x86: {var} has {len(rows)} rows, expected {expected}")
            for i, (code, comment) in enumerate(rows):
                value, _ = strip_trailing_comma(code)
                check(int(value, 16) == data.primary_table[i],
                      f"x86: computed {var} row #{i} differs from the pinned C++ table")
                check(comment == data.index_comment[i],
                      f"x86: computed {var} comment #{i} differs from the pinned C++ table")

        alias_string_m = re.search(
            r"const char InstDB::alias_name_string_table\[\] =\s*(.*?);", name_block, re.S)
        check(alias_string_m is not None, "x86: alias_name_string_table not found")
        check(decode_c_string_table(alias_string_m.group(1)).decode("latin-1") == self.alias_name_data.string_table,
              "x86: computed alias string table differs from the pinned C++ table")

        alias_id_rows = split_line_rows(name_tables["alias_index_to_inst_id_table"])
        check(len(alias_id_rows) == X86_ALIAS_COUNT,
              f"x86: expected {X86_ALIAS_COUNT} alias id rows, got {len(alias_id_rows)}")
        check_ordinals([c for _, c in alias_id_rows], "x86 alias_index_to_inst_id_table")
        for i, (code, _) in enumerate(alias_id_rows):
            value, _ = strip_trailing_comma(code)
            check(value == self.alias_link[i],
                  f"x86: computed alias link row #{i} differs from the pinned C++ table")

    # --- Opcode macro evaluation ---------------------------------------------

    def opcode_const(self, name):
        check(name in self.opcode_consts, f"x86: unknown Opcode constant {name!r}")
        return self.opcode_consts[name]

    def eval_opcode(self, code):
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

    # --- Flag-expression mapping ----------------------------------------------

    def map_inst_flags(self, expr):
        def one(part):
            m = re.fullmatch(r"F\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad InstFlags term {part!r}")
            check(m.group(1) in self.inst_flags_names, f"x86: unknown InstFlag {m.group(1)!r}")
            return f"InstFlags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_avx512_flags(self, expr):
        def one(part):
            m = re.fullmatch(r"X\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad Avx512Flags term {part!r}")
            if m.group(1) == "_":
                return "0"
            check(m.group(1) in self.avx512_flags_names, f"x86: unknown Avx512Flag {m.group(1)!r}")
            return f"Avx512Flags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_cpu_rw_flags(self, expr):
        def one(part):
            m = re.fullmatch(r"FLAG\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad CpuRWFlags term {part!r}")
            check(m.group(1) in CPU_RW_FLAGS_X86, f"x86: unknown CpuRWFlags {m.group(1)!r}")
            return f"CpuRwFlags::X86_{m.group(1)}.bits()"
        return map_flag_expr(expr, one)

    def map_op_rw_flags(self, expr):
        def one(part):
            m = re.fullmatch(r"OpRWFlags::k([A-Za-z_0-9]+)", part)
            check(m is not None, f"x86: bad OpRWFlags term {part!r}")
            name = m.group(1)
            mapped = OP_RW_FLAGS_OVERRIDES.get(name, screaming(name))
            return f"OpRwFlags::{mapped}.bits()"
        return map_flag_expr(expr, one)

    def map_op_flags(self, expr):
        """Maps `F(RegGpbLo) | F(...)` (OpSignature table) to OpFlags consts."""
        def one(part):
            m = re.fullmatch(r"F\(([A-Za-z_0-9]+)\)", part)
            check(m is not None, f"x86: bad OpFlags term {part!r}")
            check(m.group(1) in self.op_flags_names, f"x86: unknown OpFlag {m.group(1)!r}")
            return f"OpFlags::{screaming(m.group(1))}.bits()"
        return map_flag_expr(expr, one)

    def map_ext_features(self, expr):
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


def x86_parse_inst_rows(db):
    block = extract_block(db.inst_cpp, "InstInfo")
    rows = []
    for code, comment in split_line_rows(strip_generated_banner(block)):
        name, args = parse_macro_invocation(code)
        check(name == "INST", f"x86: unexpected row macro {name!r}")
        check(len(args) == 8, f"x86: INST row with {len(args)} args: {code!r}")
        rows.append({
            "id": args[0],
            "encoding": args[1],
            "opcode0": args[2],
            "opcode1": args[3],
            "main_opcode_index": args[4],
            "alt_opcode_index": args[5],
            "common_info_index": args[6],
            "additional_info_index": args[7],
            "comment": comment,
        })
    check(len(rows) == X86_INST_ID_COUNT,
          f"x86: expected {X86_INST_ID_COUNT} INST rows, got {len(rows)}")
    check_ordinals([r["comment"] for r in rows], "x86 _inst_info_table")
    for i, row in enumerate(rows):
        check(row["id"] == db.inst_ids[i][0],
              f"x86: INST row #{i} id {row['id']!r} does not match InstId {db.inst_ids[i][0]!r}")
        check(row["encoding"] in db.enc_set,
              f"x86: INST row #{i} uses unknown encoding {row['encoding']!r}")
        for key in ("main_opcode_index", "alt_opcode_index", "common_info_index", "additional_info_index"):
            check(re.fullmatch(r"\d+", row[key]) is not None,
                  f"x86: INST row #{i} bad {key} {row[key]!r}")
        row["main_opcode_value"] = db.eval_opcode(row["opcode0"]) & 0xFF
    return rows


def x86_parse_opcode_table(db, block_key, table, expected):
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


def emit_u8_table(name, values, doc):
    out = [doc]
    out.append(f"pub static {name}: &[u8] = &[\n")
    for i in range(0, len(values), 24):
        out.append("    " + " ".join(f"{v}," for v in values[i:i + 24]) + "\n")
    out.append("];\n\n")
    return "".join(out)


def emit_x86_rw_structs(db):
    out = [read_template("x86_structs_2.txt")]

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


def emit_x86_tables(db):
    out = []

    # --- INST_INFO_TABLE ---
    inst_rows = x86_parse_inst_rows(db)
    out.append("/// Instruction information table, indexed by [`InstId`].\n")
    out.append("pub static INST_INFO_TABLE: &[InstInfo] = &[\n")
    for i, row in enumerate(inst_rows):
        out.append(
            f"    InstInfo::new({row['common_info_index']}, {row['additional_info_index']}, "
            f"Encoding::{row['encoding']} as u8, 0x{row['main_opcode_value']:02X}, "
            f"{row['main_opcode_index']}, {row['alt_opcode_index']}), // #{i}\n")
    out.append("];\n\n")

    # --- Opcode tables ---
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

    # --- INST_COMMON_INFO_TABLE ---
    common_block = extract_block(db.inst_cpp, "InstCommonTable")
    common_tables = split_tables(strip_generated_banner(common_block))
    check("_inst_common_info_table" in common_tables, "x86: _inst_common_info_table missing")
    common_rows = split_line_rows(common_tables["_inst_common_info_table"])
    check(len(common_rows) == X86_COMMON_INFO_COUNT,
          f"x86: expected {X86_COMMON_INFO_COUNT} CommonInfo rows, got {len(common_rows)}")
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
            f"{sig_index}, {sig_count}, {cf}, {srh}), // {comment}\n")
    out.append("];\n\n")

    # --- ADDITIONAL_INFO_TABLE / RW_FLAGS_INFO_TABLE / INST_FLAGS_TABLE ---
    add_block = extract_block(db.inst_cpp, "AdditionalInfoTable")
    add_tables = split_tables(strip_generated_banner(add_block))

    add_rows = split_line_rows(add_tables["additional_info_table"])
    check(len(add_rows) == X86_ADDITIONAL_INFO_COUNT,
          f"x86: expected {X86_ADDITIONAL_INFO_COUNT} AdditionalInfo rows, got {len(add_rows)}")
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
            f"    AdditionalInfo::new({fields[0]}, {fields[1]}, [{', '.join(features)}]), // {comment}\n")
    out.append("];\n\n")

    rwf_rows = split_line_rows(add_tables["rw_flags_info_table"])
    check(len(rwf_rows) == X86_RW_FLAGS_COUNT,
          f"x86: expected {X86_RW_FLAGS_COUNT} RWFlagsInfo rows, got {len(rwf_rows)}")
    check_ordinals([c for _, c in rwf_rows], "x86 rw_flags_info_table")
    out.append("/// CPU flags read/written, indexed by [`AdditionalInfo::rw_flags_index`].\n")
    out.append("pub static RW_FLAGS_INFO_TABLE: &[RwFlagsInfo] = &[\n")
    for code, comment in rwf_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 2, f"x86: RWFlagsInfo row with {len(fields)} fields: {code!r}")
        out.append(
            f"    RwFlagsInfo::new({db.map_cpu_rw_flags(fields[0])}, {db.map_cpu_rw_flags(fields[1])}), // {comment}\n")
    out.append("];\n\n")

    inst_flags_map = {"None": "NONE", "MovOp": "MOV_OP"}
    ifl_rows = split_line_rows(add_tables["inst_flags_table"])
    check(len(ifl_rows) == X86_INST_FLAGS_COUNT,
          f"x86: expected {X86_INST_FLAGS_COUNT} inst_flags_table rows, got {len(ifl_rows)}")
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

    # --- Name data (computed via InstructionNameData) ---
    out.append("/// Maps the first letter of an instruction name to a span of [`InstId`] values.\n")
    out.append("pub static INST_NAME_INDEX: &[(u16, u16)] = &[\n")
    for i in range(26):
        def map_span(inst_kid, plus_one):
            m = re.fullmatch(r"Inst::kId([A-Za-z_0-9]+)", inst_kid)
            check(m is not None, f"x86: bad name index span value {inst_kid!r}")
            return f"InstId::{m.group(1)} as u16" + (" + 1" if plus_one else "")

        first = db.inst_first[i] or "Inst::kIdNone"
        last = db.inst_last[i] or "Inst::kIdNone"
        out.append(f"    ({map_span(first, False)}, {map_span(last, True)}),\n")
    out.append("];\n\n")
    out.append(f"pub const MAX_NAME_LENGTH: u16 = {db.inst_name_data.max_name_length};\n\n")

    out.append(f"pub static INST_NAME_STRING_TABLE: &[u8] = {rust_byte_string(db.inst_name_data.string_table.encode('latin-1'))};\n\n")

    out.append("#[rustfmt::skip]\n")
    out.append("pub static INST_NAME_INDEX_TABLE: &[u32] = &[\n")
    count = len(db.inst_name_data.primary_table)
    for idx in range(count):
        comma = "," if idx + 1 < count else ""
        out.append(f"    0x{db.inst_name_data.primary_table[idx]:08X}{comma} // {db.inst_name_data.index_comment[idx]}\n")
    out.append("];\n\n")

    out.append(f"pub static ALIAS_NAME_STRING_TABLE: &[u8] = {rust_byte_string(db.alias_name_data.string_table.encode('latin-1'))};\n\n")

    out.append("pub static ALIAS_NAME_INDEX_TABLE: &[u32] = &[\n")
    count = len(db.alias_name_data.primary_table)
    for idx in range(count):
        comma = "," if idx + 1 < count else ""
        out.append(f"    0x{db.alias_name_data.primary_table[idx]:08X}{comma} // {db.alias_name_data.index_comment[idx]}\n")
    out.append("];\n\n")

    out.append("pub static ALIAS_INDEX_TO_INST_ID_TABLE: &[u32] = &[\n")
    count = len(db.alias_link)
    for idx, link in enumerate(db.alias_link):
        m = re.fullmatch(r"Inst::kId([A-Za-z_0-9]+)", link)
        check(m is not None, f"x86: bad alias id row {link!r}")
        comma = "," if idx + 1 < count else ""
        out.append(f"    InstId::{m.group(1)} as u32{comma} // #{idx}\n")
    out.append("];\n\n")
    out.append(f"pub const ALIAS_TABLE_SIZE: u32 = {X86_ALIAS_COUNT};\n\n")

    # --- Signature tables ---
    sig_block = extract_block(db.inst_cpp, "InstSignatureTable")
    sig_tables = split_tables(strip_generated_banner(sig_block))

    sig_rows = split_line_rows(sig_tables["_inst_signature_table"])
    check(len(sig_rows) == X86_SIGNATURE_COUNT,
          f"x86: expected {X86_SIGNATURE_COUNT} InstSignature rows, got {len(sig_rows)}")
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
            f"    InstSignature::new({args[0]}, Mode::{mode} as u8, {args[3]}, 0, [{indexes}]),{suffix}\n")
    out.append("];\n\n")

    op_sig_rows = split_line_rows(sig_tables["_op_signature_table"])
    check(len(op_sig_rows) == X86_OP_SIGNATURE_COUNT,
          f"x86: expected {X86_OP_SIGNATURE_COUNT} OpSignature rows, got {len(op_sig_rows)}")
    out.append("/// Operand signatures, indexed by [`InstSignature::op_signature_indexes`].\n")
    out.append("pub static OP_SIGNATURE_TABLE: &[OpSignature] = &[\n")
    for code, comment in op_sig_rows:
        name, args = parse_macro_invocation(code)
        check(name == "ROW" and len(args) == 2, f"x86: bad OpSignature row {code!r}")
        out.append(f"    OpSignature::new({db.map_op_flags(args[0])}, {args[1]}),\n")
    out.append("];\n\n")

    # --- RW info tables ---
    rw_block = extract_block(db.inst_cpp, "InstRWInfoTable")
    rw_tables = split_tables(strip_generated_banner(rw_block))

    for var, rust_name in (
        ("rw_info_index_a_table", "RW_INFO_INDEX_A_TABLE"),
        ("rw_info_index_b_table", "RW_INFO_INDEX_B_TABLE"),
    ):
        body = re.sub(r"//[^\n]*", "", rw_tables[var])
        values = [parse_int(v) for v in body.replace("\n", " ").split(",") if v.strip()]
        check(len(values) == X86_INST_ID_COUNT,
              f"x86: expected {X86_INST_ID_COUNT} values in {var}, got {len(values)}")
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
                f"    RwInfo::new(RwInfoCategory::{m.group(1)}, {fields[1]}, [{indexes}]), // {comment}\n")
        out.append("];\n\n")

    # rw_info_op_table.
    op_rows = split_line_rows(rw_tables["rw_info_op_table"])
    check(len(op_rows) == X86_RW_INFO_OP_COUNT,
          f"x86: rw_info_op_table has {len(op_rows)} rows, expected {X86_RW_INFO_OP_COUNT}")
    check_ordinals([c for _, c in op_rows], "x86 rw_info_op_table")
    out.append("pub static RW_INFO_OP_TABLE: &[RwInfoOp] = &[\n")
    for code, comment in op_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 6, f"x86: RWInfoOp row with {len(fields)} fields: {code!r}")
        check(fields[4].replace(" ", "") == "{0}", f"x86: RWInfoOp reserved field not zero: {code!r}")
        out.append(
            f"    RwInfoOp::new({strip_int_suffix(fields[0])}, {strip_int_suffix(fields[1])}, "
            f"{fields[2]}, {fields[3]}, {db.map_op_rw_flags(fields[5])}), // {comment}\n")
    out.append("];\n\n")

    # rw_info_rm_table.
    rm_category_set = {n[len("kCategory"):] for n, _, _ in db.rw_info_rm_category_entries}
    rm_rows = split_line_rows(rw_tables["rw_info_rm_table"])
    check(len(rm_rows) == X86_RW_INFO_RM_COUNT,
          f"x86: rw_info_rm_table has {len(rm_rows)} rows, expected {X86_RW_INFO_RM_COUNT}")
    check_ordinals([c for _, c in rm_rows], "x86 rw_info_rm_table")
    out.append("pub static RW_INFO_RM_TABLE: &[RwInfoRm] = &[\n")
    for code, comment in rm_rows:
        fields = parse_brace_row(code)
        check(len(fields) == 5, f"x86: RWInfoRm row with {len(fields)} fields: {code!r}")
        m = re.fullmatch(r"InstDB::RWInfoRm::kCategory([A-Za-z_0-9]+)", fields[0])
        check(m is not None and m.group(1) in rm_category_set,
              f"x86: bad RWInfoRm category {fields[0]!r}")

        def rm_flag_one(part):
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
            f"{flags}, {feature}), // {comment}\n")
    out.append("];\n\n")

    return "".join(out)


def emit_x86(db):
    """The content of `x86_instdb.rs`."""
    out = [ZLIB_HEADER, X86_DERIVED_NOTE]
    out.append("use bitflags::bitflags;\n\n")
    out.append("use crate::core::rwinfo::{CpuRwFlags, InstControlFlow, InstRwFlags, InstSameRegHint, OpRwFlags};\n\n")

    # --- CpuFeature ---
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

    # --- InstId ---
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

    # --- Mode ---
    out.append("/// Describes which operation mode is supported by an instruction.\n")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    out.append("#[repr(u8)]\n")
    out.append("pub enum Mode {\n")
    for name, value, doc in db.mode_entries:
        if doc:
            out.append(f"    /// {doc}\n")
        out.append(f"    {strip_k(name)} = {emit_bitflags_value(value)},\n")
    out.append("}\n\n")

    # --- OpFlags ---
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

    # --- Structs (ports of the POD types in x86instdb{,_p}.h) ---
    out.append(read_template("x86_structs.txt"))

    # --- InstFlags / Avx512Flags ---
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

    # --- Encoding enum ---
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

    # --- Remaining enums + tables ---
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


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------

def build():
    return emit_x86(X86Db())


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true",
                        help=f"compare the generated instdb with {SRC_RS} (read-only) and exit")
    parser.add_argument("--stdout", action="store_true",
                        help="print the generated instdb to stdout instead of writing files")
    args = parser.parse_args(argv)

    instdb_text = build()

    if args.stdout:
        sys.stdout.write(instdb_text)
        return 0

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    instdb_out = OUT_DIR / "x86_instdb.rs"
    instdb_out.write_text(instdb_text, encoding="utf-8")
    print(f"{instdb_out}: written ({len(instdb_text)} bytes)")

    if args.check:
        repo_rs = (OUT_DIR.parent.parent.parent / SRC_RS).resolve()
        if not repo_rs.is_file():
            fail(f"--check needs {repo_rs}")
        expected = repo_rs.read_text(encoding="utf-8")
        if instdb_text == expected:
            print(f"x86: byte-exact match with {SRC_RS}")
            return 0
        diffs = token_diff(instdb_text, expected)
        if not diffs:
            print(f"x86: token-equal to {SRC_RS} (not byte-exact)")
            return 0
        print(f"x86: output differs from {SRC_RS}:", file=sys.stderr)
        for d in diffs:
            print(d, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
