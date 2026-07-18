# This file is part of asmkit.
#
# Unit tests and acceptance gates for the arch tablegen drivers
# (meta/asmjit_db/tablegen_x86.py and tablegen_a64.py) and their shared
# vendored-C++ machinery (meta/asmjit_db/cxx_src.py).

"""The gate tests regenerate the full instruction-database tables from the
vendored asmjit sources and require a byte-exact match with the
translator-generated `src/x86/instdb.rs` and `src/aarch64/instdb.rs`
(read-only). The a64 NZCV tests cover the asmkit-specific RW_FLAGS_TABLE
computed from `db/isa_aarch64.json` `io` attributes.

Run from the repo root with `python3 -m meta.asmjit_db.test_instdb`, or via
unittest discovery (`python3 -m unittest discover -s meta/asmjit_db -t .`).
"""

import os
import re
import unittest

try:
    from . import cxx_src, tablegen_a64, tablegen_x86
    from .cxx_src import REPO_ROOT
except ImportError:  # Direct execution / non-package discovery.
    import sys

    sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
    from meta.asmjit_db import cxx_src, tablegen_a64, tablegen_x86
    from meta.asmjit_db.cxx_src import REPO_ROOT


class CxxSrcTest(unittest.TestCase):
    def test_split_top_level(self):
        self.assertEqual(cxx_src.split_top_level("a, b{c, d}, e"), ["a", " b{c, d}", " e"])
        self.assertEqual(cxx_src.split_top_level('F("a,b"), G'), ['F("a,b")', ' G'])
        with self.assertRaises(cxx_src.CxxError):
            cxx_src.split_top_level("a, b}")

    def test_parse_macro_invocation(self):
        self.assertEqual(cxx_src.parse_macro_invocation("INST(A, B, {c, d})"),
                         ("INST", ["A", "B", "{c, d}"]))
        with self.assertRaises(cxx_src.CxxError):
            cxx_src.parse_macro_invocation("not a macro")

    def test_parse_brace_row(self):
        self.assertEqual(cxx_src.parse_brace_row("{ 1, {2, 3}, 4 }"), ["1", "{2, 3}", "4"])

    def test_int_parsing(self):
        self.assertEqual(cxx_src.parse_int("0x1F"), 31)
        self.assertEqual(cxx_src.parse_int("0b101"), 5)
        self.assertEqual(cxx_src.parse_int("42"), 42)
        self.assertEqual(cxx_src.strip_int_suffix("12u"), "12")
        with self.assertRaises(cxx_src.CxxError):
            cxx_src.parse_int("1.5")

    def test_c_string_decoding(self):
        self.assertEqual(cxx_src.decode_c_escapes(r"a\x42\x0c\n"), b"aB\x0c\n")
        self.assertEqual(cxx_src.decode_c_string_table('"ab" "cd"'), b"abcd")
        with self.assertRaises(cxx_src.CxxError):
            cxx_src.decode_c_escapes(r"\q")

    def test_rust_byte_string(self):
        self.assertEqual(cxx_src.rust_byte_string(b"ab\x0c\x7f\""),
                         'b"ab\\x0C\\x7F\\""')

    def test_eval_cpp_int(self):
        self.assertEqual(cxx_src.eval_cpp_int("(1 << 4) | kA", {"kA": 3}), 19)
        with self.assertRaises(cxx_src.CxxError):
            cxx_src.eval_cpp_int("kB + 1", {})

    def test_name_mapping(self):
        # `baseRM_SImm9` becomes BASE_RM_S_IMM9; the hand port's override
        # table renames the emitted table to BASE_RM_SIMM9 instead.
        self.assertEqual(cxx_src.screaming("baseRM_SImm9"), "BASE_RM_S_IMM9")
        self.assertEqual(cxx_src.screaming("ZExt"), "Z_EXT")
        self.assertEqual(cxx_src.strip_k("kIdAbs"), "IdAbs")
        self.assertEqual(cxx_src.strip_k("k387"), "_387")

    def test_emit_bitflags_value(self):
        self.assertEqual(cxx_src.emit_bitflags_value("kA | kBc"),
                         "Self::A.bits() | Self::BC.bits()")
        self.assertEqual(cxx_src.emit_bitflags_value("0x10u"), "0x10")

    def test_token_diff(self):
        a = "pub static X: &[u8] = &[1, 2];\n"
        b = "pub static X: &[u8] = &[1,\n2];\n"
        self.assertEqual(cxx_src.token_diff(a, b), [])  # Layout-insensitive.
        self.assertTrue(cxx_src.token_diff(a, a + "extra"))


class A64IoFlagsTest(unittest.TestCase):
    def test_all_eight_io_strings(self):
        parse = tablegen_a64.parse_io_flags
        self.assertEqual(parse("C=R"), (frozenset({"A64_C"}), frozenset()))
        self.assertEqual(parse("C=X"), (frozenset({"A64_C"}), frozenset({"A64_C"})))
        self.assertEqual(parse("N=R Z=R C=R V=R"),
                         (frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"}), frozenset()))
        self.assertEqual(parse("N=W Z=W C=W V=W"),
                         (frozenset(), frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"})))
        self.assertEqual(parse("N=W Z=W C=X V=W"),
                         (frozenset({"A64_C"}), frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"})))
        self.assertEqual(parse("N=W Z=W V=W"),
                         (frozenset(), frozenset({"A64_N", "A64_Z", "A64_V"})))
        self.assertEqual(parse("N=X Z=X C=X V=X"),
                         (frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"}),
                          frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"})))
        # `QC|=SAT` is a conditional write of the saturation flag A64_Q.
        self.assertEqual(parse("QC|=SAT"), (frozenset(), frozenset({"A64_Q"})))

    def test_io_strings_cover_exactly_eight_variants(self):
        data = cxx_src.read_db_json("isa_aarch64.json")
        variants = set()
        for group in data["instructions"]:
            for entry in group["data"]:
                if entry.get("io"):
                    variants.add(entry["io"])
        self.assertEqual(len(variants), 8)
        for variant in variants:
            tablegen_a64.parse_io_flags(variant)  # Must not raise.

    def test_name_union_and_aliases(self):
        flags = tablegen_a64.load_a64_io_flags()
        # `b.<cond>` maps to `b`.
        self.assertEqual(flags["b"], (frozenset({"A64_N", "A64_Z", "A64_C", "A64_V"}), frozenset()))
        # Saturating instructions (e.g. sli) write A64_Q; union across forms.
        self.assertEqual(flags["sli"], (frozenset(), frozenset({"A64_Q"})))
        # `asr|asrv`-style pipe names exist in the db but carry no `io`
        # (shifts do not set flags), so they contribute nothing.
        self.assertNotIn("asr", flags)
        self.assertNotIn("asrv", flags)


class A64RwFlagsTableTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.db = tablegen_a64.A64Db()
        cls.io_flags = tablegen_a64.load_a64_io_flags()
        cls.text, cls.seen_read, cls.seen_write = tablegen_a64.emit_a64_rw_flags(cls.db, cls.io_flags)
        cls.rows = [ln for ln in cls.text.splitlines() if re.search(r"// #\d+$", ln)]

    def _row(self, enum_name):
        idx = [e[0] for e in self.db.inst_ids].index(enum_name)
        return self.rows[idx]

    def test_row_count_and_ordinals(self):
        self.assertEqual(len(self.rows), 776)
        for i, row in enumerate(self.rows):
            self.assertTrue(row.endswith(f"// #{i}"), row)

    def test_spot_checks(self):
        self.assertEqual(self._row("Adc"), "    (CpuRwFlags::A64_C.bits(), 0), // #2")
        self.assertEqual(self._row("Adds"),
                         "    (0, CpuRwFlags::A64_N.bits() | CpuRwFlags::A64_Z.bits() | "
                         "CpuRwFlags::A64_C.bits() | CpuRwFlags::A64_V.bits()), // #6")
        self.assertEqual(self._row("B"),
                         "    (CpuRwFlags::A64_N.bits() | CpuRwFlags::A64_Z.bits() | "
                         "CpuRwFlags::A64_C.bits() | CpuRwFlags::A64_V.bits(), 0), // #29")
        self.assertIn("CpuRwFlags::A64_Q.bits()", self._row("Sli_v"))
        self.assertIn("CpuRwFlags::A64_Q.bits()", self._row("Sqadd_v"))
        # `asrv` (register shift) shares the `asr|asrv` db entry: no flags.
        self.assertEqual(self._row("Asrv"), "    (0, 0), // #12")

    def test_seen_flags_cover_all_variants(self):
        self.assertEqual(self.seen_read, {"A64_N", "A64_Z", "A64_C", "A64_V"})
        self.assertEqual(self.seen_write, {"A64_N", "A64_Z", "A64_C", "A64_V", "A64_Q"})


class A64GateTest(unittest.TestCase):
    def test_instdb_byte_exact(self):
        text = tablegen_a64.emit_a64(tablegen_a64.A64Db())
        expected = (REPO_ROOT / "src" / "aarch64" / "instdb.rs").read_text(encoding="utf-8")
        self.assertEqual(text, expected)

    def test_deterministic(self):
        a = tablegen_a64.emit_a64(tablegen_a64.A64Db())
        b = tablegen_a64.emit_a64(tablegen_a64.A64Db())
        self.assertEqual(a, b)

    def test_name_data_computed_not_copied(self):
        # The cross-check inside A64Db already proves parity; assert the
        # computed table sizes explicitly (776 ids, non-empty string blob).
        db = tablegen_a64.A64Db()
        self.assertEqual(len(db.name_data.primary_table), 776)
        self.assertEqual(db.name_data.max_name_length, 9)
        self.assertGreater(len(db.name_data.string_table), 100)


class X86GateTest(unittest.TestCase):
    def test_instdb_byte_exact(self):
        text = tablegen_x86.build()
        expected = (REPO_ROOT / "src" / "x86" / "instdb.rs").read_text(encoding="utf-8")
        self.assertEqual(text, expected)

    def test_deterministic(self):
        a = tablegen_x86.build()
        b = tablegen_x86.build()
        self.assertEqual(a, b)

    def test_db_shape(self):
        db = tablegen_x86.X86Db()
        self.assertEqual(len(db.inst_ids), 1648)
        self.assertEqual(len(db.aliases), 44)
        self.assertEqual(len(db.inst_name_data.primary_table), 1648)
        self.assertEqual(len(db.alias_name_data.primary_table), 44)
        self.assertEqual(db.inst_name_data.max_name_length, 17)


if __name__ == "__main__":
    unittest.main()
