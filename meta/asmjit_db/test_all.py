# This file is part of asmkit.
#
# Unit tests for the asmjit_db foundation layer (ports of asmjit's
# generator-commons.js, db/exp.js, and db/base.js).

"""Run from the repo root with `python3 -m meta.asmjit_db.test_all`, or via
unittest discovery (`python3 -m unittest discover -s meta/asmjit_db -t .`).
"""

import json
import os
import unittest
from functools import cmp_to_key

try:
    from . import base, exp, gen_common
except ImportError:  # Direct execution / non-package discovery.
    import sys

    sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
    from meta.asmjit_db import base, exp, gen_common


class ObjectUtilsTest(unittest.TestCase):
    def test_clone_is_shallow(self):
        src = {"a": {"b": 1}, "c": 2}
        out = gen_common.ObjectUtils.clone(src)
        self.assertEqual(out, src)
        self.assertIsNot(out, src)
        self.assertIs(out["a"], src["a"])  # Shallow, like JS Object.assign.

    def test_merge_recursive(self):
        a = {"x": {"p": 1}, "y": 1}
        b = {"x": {"q": 2}, "z": 3}
        out = gen_common.ObjectUtils.merge(a, b)
        self.assertIs(out, a)
        self.assertEqual(a, {"x": {"p": 1, "q": 2}, "y": 1, "z": 3})

    def test_merge_same_object(self):
        a = {"x": 1}
        self.assertIs(gen_common.ObjectUtils.merge(a, a), a)

    def test_equals(self):
        eq = gen_common.ObjectUtils.equals
        self.assertTrue(eq({"a": [1, {"b": 2}]}, {"a": [1, {"b": 2}]}))
        self.assertTrue(eq(1, 1.0))  # JS has only doubles.
        self.assertFalse(eq(True, 1))  # JS `true !== 1`.
        self.assertFalse(eq({"a": [1, 2]}, {"a": [1]}))
        self.assertFalse(eq({"a": 1}, {"a": 1, "b": 2}))
        self.assertFalse(eq({"a": 1}, [1]))
        self.assertFalse(eq("1", 1))
        self.assertTrue(eq(None, None))

    def test_equals_except(self):
        eq = gen_common.ObjectUtils.equals_except
        self.assertTrue(eq({"a": 1, "b": 2}, {"a": 1, "b": 3}, {"b"}))
        self.assertFalse(eq({"a": 1, "b": 2}, {"a": 1, "b": 3}, set()))
        self.assertTrue(eq([1, 2], [1, 2], {"anything"}))

    def test_find_key_and_has_any(self):
        ou = gen_common.ObjectUtils
        self.assertEqual(ou.find_key({"a": 1}, ["x", "a"]), "a")
        self.assertIsNone(ou.find_key({"a": 1}, ["x", "y"]))
        self.assertTrue(ou.has_any({"a": 1}, ["x", "a"]))
        self.assertFalse(ou.has_any({"a": 1}, ["x", "y"]))


class ArrayUtilsTest(unittest.TestCase):
    def test_min_max(self):
        au = gen_common.ArrayUtils
        self.assertIsNone(au.min([]))
        self.assertIsNone(au.max([]))
        self.assertEqual(au.min([3, 1, 2]), 1)
        self.assertEqual(au.max([3, 1, 2]), 3)
        self.assertEqual(au.min(["aa", "b"], fn=len), 1)

    def test_sorted(self):
        au = gen_common.ArrayUtils
        self.assertEqual(au.sorted({"b": 1, "a": 2}), ["a", "b"])
        arr = [3, 1, 2]
        self.assertEqual(au.sorted(arr), [1, 2, 3])
        self.assertEqual(arr, [3, 1, 2])  # Input not mutated.
        self.assertEqual(au.sorted([1, 2, 3], lambda a, b: b - a), [3, 2, 1])

    def test_deep_index_of(self):
        arr = [{"a": 1}, [1, 2], "x"]
        self.assertEqual(gen_common.ArrayUtils.deep_index_of(arr, [1, 2]), 1)
        self.assertEqual(gen_common.ArrayUtils.deep_index_of(arr, {"a": 2}), -1)

    def test_to_dict(self):
        au = gen_common.ArrayUtils
        self.assertEqual(au.to_dict(["a", "b"]), {"a": True, "b": True})
        self.assertEqual(au.to_dict(["a"], 5), {"a": 5})


class StringUtilsTest(unittest.TestCase):
    def test_make_enum_name(self):
        su = gen_common.StringUtils
        self.assertEqual(su.make_enum_name("foo"), "Foo")
        self.assertEqual(su.make_enum_name(""), "")

    def test_count_of(self):
        su = gen_common.StringUtils
        self.assertEqual(su.count_of("aXbXcX", "X"), 3)
        self.assertEqual(su.count_of("aaaa", "aa"), 2)
        self.assertEqual(su.count_of("", "x"), 0)
        with self.assertRaises(ValueError):
            su.count_of("x", "")

    def test_trim(self):
        su = gen_common.StringUtils
        self.assertEqual(su.trim_left("  a  "), "a  ")
        self.assertEqual(su.trim_right("  a  "), "  a")
        self.assertEqual(su.trim("  a  "), "a")

    def test_up_first(self):
        su = gen_common.StringUtils
        self.assertEqual(su.up_first("foo"), "Foo")
        self.assertEqual(su.up_first(""), "")

    def test_dec_to_hex(self):
        su = gen_common.StringUtils
        self.assertEqual(su.dec_to_hex(-1), "0xFFFFFFFF")
        self.assertEqual(su.dec_to_hex(-2, 4), "0xFFFFFFFE")
        self.assertEqual(su.dec_to_hex(255, 8), "0x000000FF")
        self.assertEqual(su.dec_to_hex(10), "0xA")
        self.assertEqual(su.dec_to_hex(0), "0x0")

    def test_format_flow_wrap(self):
        su = gen_common.StringUtils
        # Short enough to stay on one line.
        self.assertEqual(su.format(["aaa", "bbb", "ccc"], "", -1), "aaa, bbb, ccc")
        # Long enough to wrap: the wrap check happens after appending an item.
        items = ["A" * 30, "B" * 30, "C" * 30, "D" * 30]
        expected = ("  " + "A" * 30 + ", " + "B" * 30 + ", " + "C" * 30 +
                    ",\n  " + "D" * 30)
        self.assertEqual(su.format(items, "  ", -1), expected)

    def test_format_comma_lines(self):
        su = gen_common.StringUtils
        self.assertEqual(su.format(["a", "b", "c"], "  ", 0), "  a,\n  b,\n  c")

    def test_format_with_index_and_ref_counts(self):
        su = gen_common.StringUtils
        arr = gen_common.IndexedArray(["x", "y"])
        arr.add_indexed("x")  # Bump x's ref count to 2.
        self.assertEqual(
            su.format(arr, "  ", 1),
            "  x, // #0 [ref=2x]\n  y  // #1 [ref=1x]")

    def test_format_map_fn(self):
        su = gen_common.StringUtils
        self.assertEqual(su.format([1, 2], "", 0, lambda x: f"<{x}>"), "<1>,\n<2>")

    def test_make_priority_compare(self):
        cmp = gen_common.StringUtils.make_priority_compare(["b", "a"])
        # Known items first (priority order), unknowns last, alphabetical.
        self.assertEqual(sorted(["x", "a", "b", "y"], key=cmp_to_key(cmp)),
                         ["b", "a", "x", "y"])


class IndexedArrayTest(unittest.TestCase):
    def test_dedup_order_and_ref_counts(self):
        arr = gen_common.IndexedArray()
        self.assertEqual(arr.add_indexed("a"), (0, True))
        self.assertEqual(arr.add_indexed("b"), (1, True))
        self.assertEqual(arr.add_indexed("a"), (0, False))
        self.assertEqual(arr.add_indexed("c"), (2, True))
        self.assertEqual(arr.add_indexed("b"), (1, False))

        self.assertEqual(arr.ref_count_of("a"), 2)
        self.assertEqual(arr.ref_count_of("b"), 2)
        self.assertEqual(arr.ref_count_of("c"), 1)
        self.assertEqual(arr.ref_count_of("missing"), 0)

        self.assertEqual(len(arr), 3)
        self.assertEqual(arr[0], "a")
        self.assertEqual(list(arr), [(0, "a", 2), (1, "b", 2), (2, "c", 1)])

    def test_structural_dedup(self):
        arr = gen_common.IndexedArray()
        d1 = {"op": "reg", "fields": ["a", "b"]}
        d2 = {"fields": ["a", "b"], "op": "reg"}  # Same content, other order.
        self.assertEqual(arr.add_indexed(d1), (0, True))
        self.assertEqual(arr.add_indexed(d2), (0, False))  # Canonical key.
        self.assertEqual(arr.add_indexed(["a", "b"]), (1, True))
        self.assertEqual(arr.add_indexed(["a", "b"]), (1, False))

    def test_bool_is_not_int(self):
        # Python `True == 1`; JS `true !== 1` — they must not dedup together.
        arr = gen_common.IndexedArray()
        self.assertEqual(arr.add_indexed(True), (0, True))
        self.assertEqual(arr.add_indexed(1), (1, True))


class _EvalCtx:
    """Minimal evaluation context used by the exp tests."""

    def __init__(self, variables=None):
        self._variables = variables or {}

    def variable(self, name):
        return self._variables[name]

    def function(self, name, args):
        if name == "$bit":
            return (args[0] >> args[1]) & 1
        raise exp.ExpressionError(f"Unknown function '{name}'")

    def unary(self, op, val):
        raise exp.ExpressionError(f"Unknown unary operator '{op}'")

    def binary(self, op, left, right):
        raise exp.ExpressionError(f"Unknown binary operator '{op}'")


class ExpEvaluateTest(unittest.TestCase):
    def test_shift(self):
        self.assertEqual(exp.parse("1 << 3").evaluate(), 8)

    def test_unary_minus(self):
        self.assertEqual(exp.parse("-5").evaluate(), -5)

    def test_parentheses(self):
        self.assertEqual(exp.parse("(2+3)*4").evaluate(), 20)

    def test_precedence_add_before_shift(self):
        # Like C: `+` binds tighter than `<<`.
        self.assertEqual(exp.parse("2 + 3 << 1").evaluate(), 10)

    def test_bit_call_with_ctx(self):
        node = exp.Call("$bit", [exp.Var("x"), exp.Imm(3)])
        self.assertEqual(node.evaluate(_EvalCtx({"x": 5})), 0)
        self.assertEqual(node.evaluate(_EvalCtx({"x": 8})), 1)

    def test_bit_access_sugar(self):
        node = exp.parse("x[2]")
        self.assertIsInstance(node, exp.CallNode)
        self.assertEqual(node.name, "$bit")
        self.assertEqual(node.evaluate(_EvalCtx({"x": 4})), 1)
        # Same sugar is available on VarNode directly.
        self.assertEqual(exp.Var("x")[2].to_string(), node.to_string())
        self.assertEqual(node.to_string(), "((x >> 2) & 1)")

    def test_hex_wraps_to_signed_int32(self):
        # JS parses hex through 32-bit bitwise ops, so this is -1...
        self.assertEqual(exp.parse("0xFFFFFFFF").evaluate(), -1)
        # ...and therefore this wraps to 0.
        self.assertEqual(exp.parse("0xFFFFFFFF + 1").evaluate(), 0)

    def test_division_truncates_toward_zero(self):
        self.assertEqual(exp.parse("-7 / 2").evaluate(), -3)
        self.assertEqual(exp.parse("7 / -2").evaluate(), -3)
        self.assertEqual(exp.parse("7 % -2").evaluate(), 1)  # JS: sign of dividend.
        self.assertEqual(exp.parse("-7 % 2").evaluate(), -1)

    def test_comparison_and_logic(self):
        self.assertEqual(exp.parse("1 < 2 == 3").evaluate(), 0)
        self.assertEqual(exp.parse("1 && 0 || 1").evaluate(), 1)
        self.assertEqual(exp.parse("~0").evaluate(), -1)
        self.assertEqual(exp.parse("!5").evaluate(), 0)
        self.assertEqual(exp.parse("!0").evaluate(), 1)

    def test_to_string(self):
        self.assertEqual(exp.parse("1+2*3").to_string(), "1 + (2 * 3)")
        self.assertEqual(exp.parse("-(2+3)").to_string(), "-(2 + 3)")

    def test_collectors(self):
        node = exp.parse("x + y[1] * z + x")
        self.assertEqual(exp.collect_vars(node), {"x": 2, "y": 1, "z": 1})
        self.assertEqual(exp.collect_calls(node), {"$bit": 1})

    def test_builders(self):
        self.assertEqual(exp.Shl(exp.Add(exp.Imm(2), exp.Imm(3)), exp.Imm(1)).evaluate(), 10)
        self.assertEqual(exp.Negate(exp.Imm(5)).evaluate(), -5)
        self.assertEqual(exp.BitNot(exp.Imm(0)).evaluate(), -1)
        self.assertEqual(exp.Or(exp.And(exp.Imm(1), exp.Imm(0)), exp.Imm(1)).evaluate(), 1)
        self.assertEqual(exp.Binary("-", exp.Imm(7), exp.Imm(2)).evaluate(), 5)
        self.assertEqual(exp.Unary("-", exp.Imm(1)).to_string(), "-1")

    def test_evaluate_variable_without_ctx_raises(self):
        with self.assertRaises(exp.ExpressionError):
            exp.parse("x").evaluate()

    def test_ternary_raises(self):
        # The JS ternary branch is broken dead code; the port rejects it.
        with self.assertRaises(exp.ExpressionError) as cm:
            exp.parse("1 ? 2 : 3")
        self.assertIn("Ternary", cm.exception.message)

    def test_errors(self):
        with self.assertRaises(exp.ExpressionError):
            exp.parse("")
        with self.assertRaises(exp.ExpressionError) as cm:
            exp.parse("0x")
        self.assertIn("Invalid number starting with 0x", cm.exception.message)
        with self.assertRaises(exp.ExpressionError) as cm:
            exp.parse("0xg1")
        self.assertIn("Invalid hex number 0xg", cm.exception.message)
        # Uppercase 'G' just terminates the hex (dead rejection clause in JS).
        with self.assertRaises(exp.ExpressionError) as cm:
            exp.parse("0xG")
        self.assertIn("Invalid number starting with 0x", cm.exception.message)
        with self.assertRaises(exp.ExpressionError):
            exp.parse("1 2")


class ParsingTest(unittest.TestCase):
    def test_predicates(self):
        p = base.Parsing
        self.assertTrue(p.is_implicit("<ax>"))
        self.assertFalse(p.is_implicit("ax"))
        self.assertEqual(p.clear_implicit("<ax>"), "ax")
        self.assertTrue(p.is_optional("{k}"))
        self.assertEqual(p.clear_optional("{k}"), "k")
        self.assertTrue(p.is_commutative("~eax"))
        self.assertFalse(p.is_commutative("eax"))
        self.assertEqual(p.clear_commutative("~eax"), "eax")
        self.assertEqual(base.Symbols.COMMUTATIVE, "~")

    def test_match_closing_char(self):
        self.assertEqual(base.Parsing.match_closing_char("a [b [c] d] e", 2), 10)

    def test_split_operands_basic(self):
        p = base.Parsing
        self.assertEqual(p.split_operands(""), [])
        self.assertEqual(p.split_operands("a, mem [X + Y], b"),
                         ["a", "mem [X + Y]", "b"])

    def test_split_operands_nested_brackets(self):
        self.assertEqual(base.Parsing.split_operands("mem [X + [Y - 1]], eax"),
                         ["mem [X + [Y - 1]]", "eax"])

    def test_split_operands_masks(self):
        p = base.Parsing
        self.assertEqual(p.split_operands("zmm0 {k1}, zmm1, zmm2"),
                         ["zmm0 {k1}", "zmm1", "zmm2"])
        self.assertEqual(p.split_operands("zmm {k1} {z}, zmm"),
                         ["zmm {k1} {z}", "zmm"])

    def test_split_operands_implicit(self):
        self.assertEqual(base.Parsing.split_operands("<ax>, ecx"), ["<ax>", "ecx"])

    def test_split_operands_parens(self):
        # The pinned JS never matches ')' (char code 31 bug); the port does.
        self.assertEqual(base.Parsing.split_operands("(a, b), c"), ["(a, b)", "c"])

    def test_split_operands_empty_raises(self):
        with self.assertRaises(ValueError):
            base.Parsing.split_operands("eax,, ebx")
        with self.assertRaises(ValueError):
            base.Parsing.split_operands("eax, ")


class MapUtilsTest(unittest.TestCase):
    def test_clone_except(self):
        src = {"a": 1, "b": 2, "c": 3}
        self.assertEqual(base.MapUtils.clone_except(src, "b"), {"a": 1, "c": 3})
        self.assertEqual(base.MapUtils.clone_except(src, {"a", "c"}), {"b": 2})

    def test_map_from_array(self):
        self.assertEqual(base.MapUtils.map_from_array(["a", "b"]), {"a": True, "b": True})


class OperandTest(unittest.TestCase):
    def test_flag_accessors(self):
        op = base.Operand()
        self.assertEqual(op.flags, 0)
        op.optional = True
        op.implicit = True
        op.commutative = True
        op.zext = True
        self.assertEqual(op.flags, 0x0F)
        self.assertTrue(op.optional)
        self.assertTrue(op.implicit)
        self.assertTrue(op.commutative)
        self.assertTrue(op.zext)
        op.implicit = False
        self.assertFalse(op.implicit)
        self.assertEqual(op.flags, 0x0F & ~base.OperandFlags.Implicit)

    def test_flag_values(self):
        self.assertEqual(base.OperandFlags.Optional, 1)
        self.assertEqual(base.OperandFlags.Implicit, 2)
        self.assertEqual(base.OperandFlags.Commutative, 4)
        self.assertEqual(base.OperandFlags.ZExt, 8)
        self.assertEqual(base.OperandFlags.ReadAccess, 0x10)
        self.assertEqual(base.OperandFlags.WriteAccess, 0x20)

    def test_predicates(self):
        op = base.Operand()
        self.assertFalse(op.is_reg())
        self.assertFalse(op.is_partial_op())

        op.reg = "eax"
        self.assertTrue(op.is_reg())
        self.assertTrue(op.is_reg_or_mem())
        self.assertFalse(op.is_reg_mem())

        op.mem = "m32"
        self.assertTrue(op.is_mem())
        self.assertTrue(op.is_reg_mem())

        op.type = "reg-list"
        self.assertFalse(op.is_reg())
        self.assertTrue(op.is_reg_list())

        op2 = base.Operand()
        op2.imm = 32
        op2.rel = 8
        self.assertTrue(op2.is_imm())
        self.assertTrue(op2.is_rel())

    def test_str(self):
        op = base.Operand()
        op.data = "xmm0"
        self.assertEqual(str(op), "xmm0")


class InstructionTest(unittest.TestCase):
    def test_io_string(self):
        inst = base.Instruction()
        inst._assign_attribute("io", "OF=W SF=W ZF=W AF=W PF=W CF=W")
        self.assertEqual(inst.io, {"OF": "W", "SF": "W", "ZF": "W",
                                   "AF": "W", "PF": "W", "CF": "W"})

    def test_io_multi_key(self):
        inst = base.Instruction()
        inst._assign_attribute("io", "CF|OF|SF=X")
        self.assertEqual(inst.io, {"CF": "X", "OF": "X", "SF": "X"})

    def test_io_bare_key_is_true(self):
        inst = base.Instruction()
        inst._assign_attribute("io", "OF")
        self.assertEqual(inst.io, {"OF": True})

    def test_ext_and_category(self):
        inst = base.Instruction()
        inst._assign_attribute("ext", ["SSE2", "AVX2"])
        self.assertEqual(inst.ext, {"SSE2": True, "AVX2": True})
        self.assertEqual(inst.ext_array, ["AVX2", "SSE2"])
        inst._assign_attribute("category", {"legacy": True})
        self.assertEqual(inst.category, {"legacy": True})

    def test_assign_plain_attribute(self):
        inst = base.Instruction()
        inst._assign_attribute("volatile", True)
        self.assertTrue(inst.volatile)
        with self.assertRaises(AttributeError):
            inst._assign_attribute("no_such_attribute", 1)

    def test_operands_info(self):
        inst = base.Instruction()
        for data, flags in (("eax", base.OperandFlags.Implicit),
                            ("ecx", base.OperandFlags.Commutative),
                            ("edx", 0)):
            op = base.Operand()
            op.data = data
            op.flags = flags
            inst.operands.append(op)
        inst._update_operands_info()
        self.assertEqual(inst.implicit, 0b001)
        self.assertEqual(inst.commutative, 0b010)
        self.assertTrue(inst.has_implicit())
        self.assertTrue(inst.is_commutative())
        self.assertFalse(inst.is_alias())
        self.assertEqual(inst.operand_count, 3)
        self.assertEqual(inst.minimum_operand_count, 3)
        self.assertEqual(str(inst), " eax, ecx, edx")

    def test_minimum_operand_count_with_optional(self):
        inst = base.Instruction()
        for data, optional in (("a", False), ("b", True), ("c", True)):
            op = base.Operand()
            op.data = data
            op.optional = optional
            inst.operands.append(op)
        self.assertEqual(inst.minimum_operand_count, 1)

    def test_has_attribute(self):
        inst = base.Instruction()
        self.assertTrue(inst.has_attribute("control"))
        self.assertTrue(inst.has_attribute("control", "none"))
        self.assertFalse(inst.has_attribute("control", "jmp"))
        self.assertFalse(inst.has_attribute("no_such_attribute"))


class _JsonISA(base.ISA):
    """Minimal ISA subclass that builds instructions from small JSON dicts."""

    def _add_instructions(self, items):
        for item in items:
            inst = base.Instruction(self)
            for k, v in item.items():
                if k == "operands":
                    for s in v:
                        op = base.Operand()
                        if base.Parsing.is_commutative(s):
                            op.commutative = True
                            s = base.Parsing.clear_commutative(s)
                        if base.Parsing.is_implicit(s):
                            op.implicit = True
                            s = base.Parsing.clear_implicit(s)
                        if base.Parsing.is_optional(s):
                            op.optional = True
                            s = base.Parsing.clear_optional(s)
                        op.data = s
                        inst.operands.append(op)
                    inst._update_operands_info()
                else:
                    inst._assign_attribute(k, v)
            self._add_instruction(inst)


def _make_isa():
    isa = _JsonISA()
    isa.add_data({
        "cpuLevels": [{"name": "X86"}, {"name": "X64"}],
        "specialRegs": [{"name": "mxcsr", "doc": "SSE control"}],
        "shortcuts": [{"name": "vfpu", "expand": "vec fp"}],
        "instructions": [
            {"name": "add", "encoding": "RM",
             "operands": ["~<eax>", "{ecx}"],
             "ext": "SSE2 SSE4_1", "io": "OF=W CF=W"},
            {"name": "add", "encoding": "MR"},
            {"name": "mov", "encoding": "OI"},
            {"name": "ccmp", "encoding": "X"},
        ],
        "aliases": {
            "ccmp": {"aliases": ["ccmp.b", "ccmp.nae", "ccmp.c"],
                     "format": "b|nae|c"},
        },
        "postproc": [
            {"group": "g1", "instructions": [
                {"name": "add mov", "volatile": True, "io": "ZF=W"},
            ]},
        ],
    })
    return isa


class ISATest(unittest.TestCase):
    def test_add_data_round_trip(self):
        isa = _make_isa()
        self.assertEqual(isa.stats, {"instructions": 4, "groups": 3})
        self.assertEqual(isa.instruction_names, ["add", "ccmp", "mov"])
        self.assertEqual(sorted(isa.cpu_levels), ["X64", "X86"])
        self.assertEqual(isa.special_regs["mxcsr"]["group"], "mxcsr")
        self.assertEqual(isa.shortcuts["vfpu"]["expand"], "vec fp")

    def test_instruction_groups(self):
        isa = _make_isa()
        group = isa.query("add")
        self.assertIsInstance(group, base.InstructionGroup)
        self.assertEqual(len(group), 2)
        self.assertEqual(group.check_attribute("encoding", "MR"), 1)
        self.assertEqual(group.union_cpu_features(), {"SSE2": True, "SSE4_1": True})

    def test_operands_parsed(self):
        isa = _make_isa()
        inst = isa.query("add")[0]
        self.assertEqual([op.data for op in inst.operands], ["eax", "ecx"])
        self.assertEqual(inst.implicit, 0b01)
        self.assertEqual(inst.commutative, 0b01)
        self.assertTrue(inst.has_implicit())
        self.assertTrue(inst.is_commutative())
        self.assertEqual(inst.minimum_operand_count, 1)
        self.assertEqual(inst.ext, {"SSE2": True, "SSE4_1": True})
        self.assertEqual(inst.ext_array, ["SSE2", "SSE4_1"])

    def test_postproc_patching(self):
        isa = _make_isa()
        add0 = isa.query("add")[0]
        self.assertEqual(add0.io, {"OF": "W", "CF": "W", "ZF": "W"})
        self.assertTrue(add0.volatile)
        mov = isa.query("mov")[0]
        self.assertEqual(mov.io, {"ZF": "W"})
        self.assertTrue(mov.volatile)

    def test_postproc_missing_instruction_raises(self):
        isa = _JsonISA()
        with self.assertRaises(ValueError):
            isa.add_data({
                "instructions": [{"name": "mov"}],
                "postproc": [{"group": "g", "instructions": [{"name": "nope", "volatile": True}]}],
            })

    def test_aliases(self):
        isa = _make_isa()
        self.assertEqual(isa.aliases, {"ccmp.b": "ccmp", "ccmp.nae": "ccmp",
                                       "ccmp.c": "ccmp"})
        data = isa.alias_data("ccmp")
        self.assertEqual(data["primaryName"], "ccmp")
        self.assertEqual(data["aliasNames"], ["ccmp.b", "ccmp.nae", "ccmp.c"])
        self.assertEqual(data["format"], "b|nae|c")
        self.assertIsNone(isa.alias_data("mov"))

    def test_alias_of_missing_primary_raises(self):
        isa = _JsonISA()
        with self.assertRaises(ValueError):
            isa.add_data({"aliases": {"nope": {"aliases": ["nope.x"]}}})

    def test_query_variants(self):
        isa = _make_isa()
        self.assertEqual(len(isa.query("missing")), 0)
        self.assertEqual(len(isa.query(["add", "mov", "missing"])), 3)
        self.assertEqual(len(isa.query()), 4)
        self.assertEqual([i.name for i in isa.query()], ["add", "add", "ccmp", "mov"])

        filtered = isa.query({"name": "add", "filter": lambda i: i.encoding == "MR"})
        self.assertEqual(len(filtered), 1)
        self.assertEqual(filtered[0].encoding, "MR")

        self.assertIs(isa.query("add"), isa.query("add"))
        copied = isa.query("add", copy=True)
        self.assertIsNot(copied, isa.query("add"))
        self.assertEqual(copied, isa.query("add"))

    def test_alias_of_registration(self):
        isa = _JsonISA()
        isa.add_data({"instructions": [{"name": "jc", "aliasOf": "jb"}]})
        self.assertTrue(isa.query("jc")[0].is_alias())
        self.assertEqual(isa.aliases["jb"], "jc")

    def test_abstract_add_instructions(self):
        with self.assertRaises(NotImplementedError):
            base.ISA().add_data({"instructions": [{"name": "x"}]})

    def test_for_each_group(self):
        isa = _make_isa()
        seen = []
        isa.for_each_group(lambda name, group: seen.append((name, len(group))))
        self.assertEqual(seen, [("add", 2), ("ccmp", 1), ("mov", 1)])


class ISAX86SmokeTest(unittest.TestCase):
    def test_isa_x86_json_parses(self):
        path = os.path.join(os.path.dirname(__file__), "..", "asmjit", "db", "isa_x86.json")
        with open(path) as f:
            data = json.load(f)
        self.assertIn("instructions", data)
        self.assertIn("aliases", data)
        self.assertIn("postproc", data)
        self.assertIsInstance(data["instructions"], list)
        self.assertGreater(len(data["instructions"]), 0)


if __name__ == "__main__":
    unittest.main()
