# This file is part of asmkit.
#
# Unit tests and byte-exact acceptance tests for the tablegen port
# (meta/asmjit_db/tablegen.py, a port of asmjit's tools/tablegen.js).

"""The acceptance tests extract instruction names in `Inst::Id` enum order
from the globals headers (`kIdXxx, //!< Instruction 'name' {...}.` comments),
attach alias formats from `meta/asmjit/db/isa_x86.json`, run
`generate_name_data()`, and compare the result against the actual generated
tables in asmjit's own `x86instdb.cpp` / `a64instdb.cpp` — full-section
byte-exact plus structured per-table comparisons.

Run from the repo root with `python3 -m meta.asmjit_db.test_tablegen`, or via
unittest discovery (`python3 -m unittest discover -s meta/asmjit_db -t .`).
"""

import json
import os
import re
import unittest

try:
    from . import tablegen
    from .gen_common import StringUtils
except ImportError:  # Direct execution / non-package discovery.
    import sys

    sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
    from meta.asmjit_db import tablegen
    from meta.asmjit_db.gen_common import StringUtils

ASMJIT_ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), "..", "asmjit"))


def _read(rel_path):
    with open(os.path.join(ASMJIT_ROOT, rel_path)) as f:
        return f.read()


# ---------------------------------------------------------------------------
# C++ side parsing helpers.
# ---------------------------------------------------------------------------

def _extract_generated_section(cpp_text, key):
    # Text between the `// ${Key:Begin}` / `// ${Key:End}` inject markers.
    begin_marker = f"// ${{{key}:Begin}}\n"
    end_marker = f"// ${{{key}:End}}"
    start = cpp_text.index(begin_marker) + len(begin_marker)
    end = cpp_text.index(end_marker)
    return cpp_text[start:end]


def _parse_cpp_string_table(section, decl):
    start = section.index(f"const char {decl}[] =")
    end = section.index(";", start)
    data = bytearray()
    for m in re.finditer(r"\\x([0-9A-Fa-f]{2})", section[start:end]):
        data.append(int(m.group(1), 16))
    return bytes(data).decode("latin-1")


def _parse_cpp_index_table(section, decl):
    start = section.index(f"const uint32_t {decl}[] = {{")
    end = section.index("};", start)
    return [int(m.group(1), 16) for m in re.finditer(r"0x([0-9A-Fa-f]{8})", section[start:end])]


def _globals_inst_ids(rel_path):
    # Ordered (enum, comment name) pairs from the InstId inject region.
    text = _read(rel_path)
    begin = text.index("// ${InstId:Begin}")
    end = text.index("// ${InstId:End}")

    insts = []
    for line in text[begin:end].splitlines():
        if "_kIdCount" in line:
            break
        m = re.match(r"\s*kId([A-Za-z0-9_]+)", line)
        if not m:
            continue
        m_name = re.search(r"Instruction '([^']*)'", line)
        insts.append({
            "enum": m.group(1),
            "comment_name": m_name.group(1) if m_name else None,
        })
    return insts


def _x86_insts():
    insts = _globals_inst_ids("asmjit/x86/x86globals.h")
    alias_map = json.loads(_read("db/isa_x86.json"))["aliases"]

    out = []
    for entry in insts:
        enum = entry["enum"]
        name = "" if enum == "None" else enum.lower()

        # The generated comment embeds the raw instruction name
        # (`Instruction '<name>' ...`); kIdNone's comment has no name.
        comment = entry["comment_name"]
        assert comment is None or comment == name, (enum, comment, name)

        inst = {"name": name, "displayName": name, "enum": enum}
        alias_data = alias_map.get(name)
        if alias_data:
            inst["aliases"] = {
                "aliasNames": list(alias_data["aliases"]),
                "format": alias_data["format"],
            }
        out.append(inst)
    return out


def _a64_insts():
    insts = _globals_inst_ids("asmjit/arm/a64globals.h")

    out = []
    for entry in insts:
        enum = entry["enum"]
        name = "" if enum == "None" else enum.lower()

        # a64 displays ASIMD instructions without the `_v` suffix.
        display = name[:-2] if name.endswith("_v") else name
        assert entry["comment_name"] == display, (enum, entry["comment_name"], display)

        out.append({"name": name, "displayName": display, "enum": enum})
    return out


def _first_diff_offset(a, b):
    for i, (ca, cb) in enumerate(zip(a, b)):
        if ca != cb:
            return i
    return min(len(a), len(b))


# ---------------------------------------------------------------------------
# Unit tests.
# ---------------------------------------------------------------------------

class CharTo5BitTest(unittest.TestCase):
    def test_encoding(self):
        self.assertEqual(tablegen.char_to_5bit("a"), 1)
        self.assertEqual(tablegen.char_to_5bit("z"), 26)
        self.assertEqual(tablegen.char_to_5bit("0"), 27)
        self.assertEqual(tablegen.char_to_5bit("4"), 31)

    def test_rejects_unencodable(self):
        for c in ("5", "_", "A", "."):
            with self.assertRaises(ValueError):
                tablegen.char_to_5bit(c)


class InstructionNameDataTest(unittest.TestCase):
    def test_small_encoding(self):
        nd = tablegen.InstructionNameData()
        nd.add("abc")
        # 'a'=1, 'b'=2, 'c'=3 -> 1 | 2<<5 | 3<<10, bit 31 set.
        self.assertEqual(nd.primary_table, [0x80000000 | 1 | (2 << 5) | (3 << 10)])
        self.assertEqual(nd.index_comment, ["Small 'abc'."])

    def test_empty_name_is_small(self):
        # kIdNone's empty name is small-encoded (JS regex is {0,6}).
        nd = tablegen.InstructionNameData()
        nd.add("")
        self.assertEqual(nd.primary_table, [0x80000000])
        self.assertEqual(nd.index_comment, ["Small ''."])

    def test_six_chars_small_seven_is_placeholder(self):
        nd = tablegen.InstructionNameData()
        nd.add("abcdef")
        nd.add("abcdefg")
        self.assertEqual(nd.primary_table[0], 0x80000000 | 1 | (2 << 5) | (3 << 10) |
                         (4 << 15) | (5 << 20) | (6 << 25))
        self.assertEqual(nd.primary_table[1], 0)
        nd.index()
        self.assertEqual(nd.string_table, "abcdefg")
        self.assertEqual(nd.primary_table[1], 7 << 12)
        self.assertEqual(nd.index_comment[1], "Large 'abcdefg'.")

    def test_alt_encoding(self):
        nd = tablegen.InstructionNameData()
        nd.add("cmovb", "cmov.b|nae|c")
        self.assertEqual(nd.string_table, "cmovb\x0ccmov.b|nae|c")
        self.assertEqual(nd.primary_table, [0 | (5 << 12) | (0xFFF << 16)])
        self.assertEqual(nd.index_comment, ["Large 'cmovb' + 'cmov.b|nae|c'"])

    def test_alt_same_as_name_is_cleared(self):
        nd = tablegen.InstructionNameData()
        nd.add("abc", "abc")
        self.assertEqual(nd.primary_table, [0x80000000 | 1 | (2 << 5) | (3 << 10)])

    def test_add_or_reference_string_substring_dedup(self):
        nd = tablegen.InstructionNameData()
        self.assertEqual(nd.add_or_reference_string("hello"), 0)
        self.assertEqual(nd.add_or_reference_string("ell"), 1)  # Substring reuse.
        self.assertEqual(nd.add_or_reference_string("hello"), 0)
        self.assertEqual(nd.add_or_reference_string("xhello"), 5)
        self.assertEqual(nd.string_table, "helloxhello")

    def test_index_longest_suffix_path(self):
        nd = tablegen.InstructionNameData()
        nd.add("xxabcde")  # 7 chars -> placeholder.
        nd.add("yyabcde")
        nd.index()
        # "xxabcde" lands whole; "yyabcde" reuses suffix "abcde" at offset 2.
        self.assertEqual(nd.primary_table[0], 7 << 12)
        self.assertEqual(nd.string_table, "xxabcdeyy")
        self.assertEqual(nd.primary_table[1],
                         7 | (2 << 12) | (2 << 16) | (5 << 28))
        self.assertEqual(nd.index_comment[1], "Large 'yy|abcde'.")

    def test_index_longest_prefix_path(self):
        nd = tablegen.InstructionNameData()
        nd.add("aaaaa5bbbb")  # '5' -> placeholder.
        nd.add("aaaaa5cccc")
        nd.index()
        self.assertEqual(nd.primary_table[0], 10 << 12)
        # Longest prefix "aaaaa5" (6 >= 10/2+1) + new suffix "cccc".
        self.assertEqual(nd.string_table, "aaaaa5bbbbcccc")
        self.assertEqual(nd.primary_table[1],
                         0 | (6 << 12) | (10 << 16) | (4 << 28))
        self.assertEqual(nd.index_comment[1], "Large 'aaaaa5|cccc'.")

    def test_index_min_prefix_size_uses_js_float_semantics(self):
        # For length 9 the JS threshold is 9/2+1 = 5.5, so a longest prefix of
        # 5 does NOT qualify (floor-division semantics would split here).
        nd = tablegen.InstructionNameData()
        nd.add("aaaabbbbb")
        nd.add("aaaabcccc")
        nd.index()
        self.assertEqual(nd.primary_table[0], 9 << 12)
        self.assertEqual(nd.string_table, "aaaabbbbbaaaabcccc")
        self.assertEqual(nd.primary_table[1], 9 | (9 << 12))
        self.assertEqual(nd.index_comment[1], "Large 'aaaabcccc'.")

    def test_index_stable_length_order(self):
        # Equal lengths keep insertion order (JS tiebreak is dead code).
        nd = tablegen.InstructionNameData()
        nd.add("11111zz")  # '5' -> placeholder; first of length 7.
        nd.add("22222zz")
        nd.index()
        # First name indexed first: it gets the whole-word entry at offset 0.
        self.assertEqual(nd.index_comment[0], "Large '11111zz'.")
        self.assertEqual(nd.primary_table[0], 7 << 12)

    def test_format_index_table(self):
        nd = tablegen.InstructionNameData()
        nd.add("ab")
        self.assertEqual(nd.format_index_table("X"),
                         "const uint32_t X[] = {\n  0x80000041  // Small 'ab'.\n};\n")

    def test_format_string_table(self):
        nd = tablegen.InstructionNameData()
        nd.string_table = "abc"
        self.assertEqual(nd.format_string_table("Y"),
                         'const char Y[] =\n  "\\x61\\x62\\x63";\n\n')

    def test_get_size(self):
        nd = tablegen.InstructionNameData()
        nd.add("abc")
        self.assertEqual(nd.get_size(), 4)


class TableGenFrameworkTest(unittest.TestCase):
    def test_task_deps_and_outputs(self):
        order = []

        class T(tablegen.Task):
            def run(self):
                order.append(self.name)
                self.emit(self.name, self.name + "-content", 1)

        tg = tablegen.TableGen("X86")
        tg.add_task(T("a", []))
        tg.add_task(T("b", ["a"]))
        tg.add_task(T("c", ["a"]))
        tg.run()

        self.assertEqual(order, ["a", "b", "c"])
        self.assertEqual(tg.outputs["b"], "b-content")
        self.assertEqual(tg.table_sizes["b"], 1)

    def test_add_task_errors(self):
        tg = tablegen.TableGen("X86")
        with self.assertRaises(ValueError):
            tg.add_task(tablegen.Task("", []))
        with self.assertRaises(ValueError):
            tg.add_task(tablegen.Task("x", ["missing"]))
        tg.add_task(tablegen.Task("y", []))
        with self.assertRaises(ValueError):
            tg.add_task(tablegen.Task("y", []))

    def test_cyclic_dependency_detected(self):
        # add_task() requires deps to pre-exist, so a cycle is only possible
        # by mutating deps afterwards (same as JS).
        tg = tablegen.TableGen("X86")
        task = tablegen.Task("z", [])
        tg.add_task(task)
        task.deps.append("z")
        with self.assertRaises(RuntimeError) as cm:
            tg.run()
        self.assertIn("cyclic dependency", str(cm.exception))

    def test_task_run_is_abstract(self):
        with self.assertRaises(NotImplementedError):
            tablegen.Task("t").run()

    def test_instruction_management(self):
        tg = tablegen.TableGen("X86")
        tg.add_instruction({"name": "add", "enum": "Add", "displayName": "add"})
        self.assertEqual(tg.insts[0]["id"], 0)
        self.assertIs(tg.inst_map["add"], tg.insts[0])
        with self.assertRaises(ValueError):
            tg.add_instruction({"name": "add", "enum": "Add", "displayName": "add"})
        tg.add_alias("aad", "add")
        self.assertEqual(tg.alias_map["aad"], "add")


class _CommentIdEnum(tablegen.IdEnum):
    def comment(self, inst):
        return f"Instruction '{inst['name']}'."


class IdEnumTest(unittest.TestCase):
    def test_id_enum_output(self):
        tg = tablegen.TableGen("X86")
        tg.add_instruction({"name": "", "enum": "None", "displayName": ""})
        tg.add_instruction({"name": "jb", "enum": "Jb", "displayName": "jb",
                            "aliases": {"aliasNames": ["jnae", "jc"], "format": "jb|nae|c"}})
        tg.add_task(_CommentIdEnum())
        tg.run()

        expected = (
            "kIdNone = 0,                         //!< Instruction ''.\n"
            "kIdJb,                               //!< Instruction 'jb'.\n"
            "_kIdCount,\n"
            "\n"
            "// Aliases.\n"
            "kIdJnae = kIdJb,\n"
            "kIdJc = kIdJb\n"
        )
        self.assertEqual(tg.outputs["InstId"], expected)

    def test_id_enum_without_aliases(self):
        tg = tablegen.TableGen("A64")
        tg.add_instruction({"name": "nop", "enum": "Nop", "displayName": "nop"})
        tg.add_task(_CommentIdEnum())
        tg.run()
        self.assertEqual(tg.outputs["InstId"],
                         "kIdNop = 0,                          //!< Instruction 'nop'.\n"
                         "_kIdCount\n")

    def test_comment_is_abstract(self):
        with self.assertRaises(NotImplementedError):
            tablegen.IdEnum().comment({})


class GenerateNameDataSyntheticTest(unittest.TestCase):
    def test_synthetic(self):
        insts = [
            {"name": "", "displayName": "", "enum": "None"},
            {"name": "add", "displayName": "add", "enum": "Add"},
            {"name": "jb", "displayName": "jb", "enum": "Jb",
             "aliases": {"aliasNames": ["jnae", "jc"], "format": "jb|nae|c"}},
            {"name": "mov", "displayName": "mov", "enum": "Mov"},
        ]
        out = tablegen.Output()
        tablegen.generate_name_data(out, insts, True)
        text = out.content["NameData"]

        self.assertIn("uint16_t(3)", text)

        def span(first, last):
            return f"{{ {first.ljust(22)}, {last.ljust(22)} + 1 }}"

        self.assertIn(span("Inst::kIdAdd", "Inst::kIdAdd"), text)
        self.assertIn(span("Inst::kIdJb", "Inst::kIdJb"), text)
        self.assertIn(span("Inst::kIdMov", "Inst::kIdMov"), text)
        # Letters without instructions map to kIdNone.
        self.assertIn(span("Inst::kIdNone", "Inst::kIdNone"), text)

        # Alias table is sorted by alias name: jc (#0) before jnae (#1).
        alias_idx = _parse_cpp_index_table(text, "InstDB::alias_name_index_table")
        self.assertEqual(len(alias_idx), 2)
        self.assertIn("Inst::kIdJb, // #0", text)
        self.assertIn("Inst::kIdJb  // #1", text)
        self.assertIn("static constexpr uint32_t kAliasTableSize = 2;\n",
                      out.content["NameDataInfo"])

    def test_invalid_lookup_character(self):
        insts = [{"name": "9x", "displayName": "9x", "enum": "X9"}]
        with self.assertRaises(ValueError):
            tablegen.generate_name_data(tablegen.Output(), insts, False)


class NameTableTaskTest(unittest.TestCase):
    def test_name_table_task_emits_outputs(self):
        tg = tablegen.TableGen("X86")
        tg.add_instruction({"name": "", "enum": "None", "displayName": ""})
        tg.add_instruction({"name": "add", "enum": "Add", "displayName": "add"})
        tg.add_task(tablegen.NameTable(generate_aliases=True))
        tg.run()
        self.assertIn("NameData", tg.outputs)
        self.assertIn("NameDataInfo", tg.outputs)
        self.assertGreater(tg.table_sizes["NameData"], 0)

    def test_name_table_task_without_aliases(self):
        tg = tablegen.TableGen("A64")
        tg.add_instruction({"name": "", "enum": "None", "displayName": ""})
        tg.add_task(tablegen.NameTable())
        tg.run()
        self.assertIn("NameData", tg.outputs)
        self.assertNotIn("NameDataInfo", tg.outputs)


# ---------------------------------------------------------------------------
# Byte-exact acceptance tests against asmjit's own generated tables.
# ---------------------------------------------------------------------------

class X86NameDataAcceptanceTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.insts = _x86_insts()
        cls.out = tablegen.Output()
        tablegen.generate_name_data(cls.out, cls.insts, True)
        cls.generated = cls.out.content["NameData"]
        cls.expected = _extract_generated_section(_read("asmjit/x86/x86instdb.cpp"), "NameData")

    def test_instruction_count(self):
        self.assertEqual(len(self.insts), 1648)

    def test_name_data_section_byte_exact(self):
        if self.generated != self.expected:
            i = _first_diff_offset(self.generated, self.expected)
            self.fail(
                f"NameData section differs at offset {i}:\n"
                f"mine:     {self.generated[max(0, i - 60):i + 60]!r}\n"
                f"expected: {self.expected[max(0, i - 60):i + 60]!r}")

    def test_string_table_content(self):
        mine = _parse_cpp_string_table(self.generated, "InstDB::_inst_name_string_table")
        want = _parse_cpp_string_table(self.expected, "InstDB::_inst_name_string_table")
        self.assertEqual(mine, want)

    def test_index_table_content(self):
        mine = _parse_cpp_index_table(self.generated, "InstDB::_inst_name_index_table")
        want = _parse_cpp_index_table(self.expected, "InstDB::_inst_name_index_table")
        self.assertEqual(len(want), 1648)
        self.assertEqual(mine, want)

    def test_alias_tables_content(self):
        mine_str = _parse_cpp_string_table(self.generated, "InstDB::alias_name_string_table")
        want_str = _parse_cpp_string_table(self.expected, "InstDB::alias_name_string_table")
        self.assertEqual(mine_str, want_str)

        mine = _parse_cpp_index_table(self.generated, "InstDB::alias_name_index_table")
        want = _parse_cpp_index_table(self.expected, "InstDB::alias_name_index_table")
        self.assertEqual(len(want), 44)
        self.assertEqual(mine, want)

    def test_name_data_info(self):
        expected = _extract_generated_section(_read("asmjit/x86/x86instdb_p.h"), "NameDataInfo")
        self.assertEqual(self.out.content["NameDataInfo"], expected)


class A64NameDataAcceptanceTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.insts = _a64_insts()
        cls.out = tablegen.Output()
        tablegen.generate_name_data(cls.out, cls.insts, False)
        cls.generated = cls.out.content["NameData"]
        cls.expected = _extract_generated_section(_read("asmjit/arm/a64instdb.cpp"), "NameData")

    def test_instruction_count(self):
        self.assertEqual(len(self.insts), 776)

    def test_name_data_section_byte_exact(self):
        if self.generated != self.expected:
            i = _first_diff_offset(self.generated, self.expected)
            self.fail(
                f"NameData section differs at offset {i}:\n"
                f"mine:     {self.generated[max(0, i - 60):i + 60]!r}\n"
                f"expected: {self.expected[max(0, i - 60):i + 60]!r}")

    def test_string_table_content(self):
        mine = _parse_cpp_string_table(self.generated, "InstDB::_inst_name_string_table")
        want = _parse_cpp_string_table(self.expected, "InstDB::_inst_name_string_table")
        self.assertEqual(mine, want)

    def test_index_table_content(self):
        mine = _parse_cpp_index_table(self.generated, "InstDB::_inst_name_index_table")
        want = _parse_cpp_index_table(self.expected, "InstDB::_inst_name_index_table")
        self.assertEqual(len(want), 776)
        self.assertEqual(mine, want)

    def test_no_alias_outputs(self):
        self.assertNotIn("NameDataInfo", self.out.content)
        self.assertNotIn("alias_name_index_table", self.generated)


if __name__ == "__main__":
    unittest.main()
