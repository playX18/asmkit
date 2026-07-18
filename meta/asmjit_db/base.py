# This file is part of asmkit.
#
# Port of asmjit's `db/base.js` (asmjit pinned at
# 0bd5787b54b575ed94bf32ac452153b34385c514, SPDX-License-Identifier: Zlib).

"""Instruction model (Operand / Instruction / InstructionGroup) and the ISA
database container.

Faithful port of asmjit's `db/base.js`. Method names are snake_case; data
fields keep their JS names (`opcodeString`, `aliasOf`, ...) because they
mirror the database schema. See README.md for behavior divergences.
"""

import enum
import re

from . import exp as exp  # Parity with base.js, which re-exports exp.


def _fail(msg):
    raise ValueError(f"[BASE] {msg}")


def dict_of(src=None):
    # JS `dict()` creates a null-prototype object; a plain dict is the
    # Python equivalent for this codebase's purposes.
    return dict(src) if src else {}


# asmdb.base.Symbols
# ==================

class Symbols:
    COMMUTATIVE = "~"


# asmdb.base.Parsing
# ==================

class Parsing:
    """Functions related to operand-string parsing."""

    @staticmethod
    def is_implicit(s):
        # Get whether the string `s` representing an operand is <implicit>.
        return s.startswith("<") and s.endswith(">")

    @staticmethod
    def clear_implicit(s):
        # Clear <implicit> attribute from the given operand string `s`.
        return s[1:-1]

    @staticmethod
    def is_optional(s):
        # Get whether the string `s` representing an operand is {optional}.
        return s.startswith("{") and s.endswith("}")

    @staticmethod
    def clear_optional(s):
        # Clear {optional} attribute from the given operand string `s`.
        return s[1:-1]

    @staticmethod
    def is_commutative(s):
        # Get whether the string `s` specifies commutativity.
        return len(s) > 0 and s[0] == Symbols.COMMUTATIVE

    @staticmethod
    def clear_commutative(s):
        # Clear commutative attribute from the given operand string `s`.
        return s[1:]

    @staticmethod
    def match_closing_char(s, from_):
        # Matches a closing bracket in string `s` starting `from_` the given
        # index. Behaves like `s.index()`, but uses a counter and skips all
        # nested matches.
        #
        # NOTE: the pinned JS maps '(' (40) to char code 31 instead of ')'
        # (41), so it never matches ')' and scans to the end of the string.
        # This port uses 41 so that `(...)` operands are skipped correctly.
        length = len(s)
        opening = ord(s[from_])
        closing = {
            40: 41,    # ().
            60: 62,    # <>.
            91: 93,    # [].
            123: 125,  # {}.
        }.get(opening, 0)

        i = from_
        pending = 1
        while True:
            i += 1
            if i >= length:
                break
            c = ord(s[i])
            if c == opening:
                pending += 1
            if c == closing:
                pending -= 1
            if not pending:
                break

        return i

    @staticmethod
    def split_operands(s):
        # Split instruction operands into a list of trimmed strings. Like
        # `s.split(",")`, but matches brackets inside operands and won't
        # blindly split on every ",".
        result = []

        s = s.strip()
        if not s:
            return result

        start = 0
        i = 0
        length = len(s)

        while True:
            if i == length or s[i] == ",":
                op = s[start:i].strip()
                if not op:
                    _fail(f"Found empty operand in '{s}'")

                result.append(op)
                if i == length:
                    return result

                i += 1
                start = i
                continue

            c = s[i]

            if (c == "<" or c == ">") and i != start:
                i += 1
                continue

            if c in "[{(<":
                i = Parsing.match_closing_char(s, i)
            else:
                i += 1


# asmdb.base.MapUtils
# ===================

class MapUtils:
    @staticmethod
    def clone_except(map_, except_):
        if isinstance(except_, str):
            except_ = {except_}
        return {k: v for k, v in map_.items() if k not in except_}

    @staticmethod
    def map_from_array(array):
        return {k: True for k in array}


# asmdb.base.Operand
# ==================

class OperandFlags(enum.IntFlag):
    Optional = 0x00000001
    Implicit = 0x00000002
    Commutative = 0x00000004
    ZExt = 0x00000008
    ReadAccess = 0x00000010
    WriteAccess = 0x00000020


class Operand:
    def __init__(self):
        self.type = ""         # Type of the operand ("reg", "reg-list", "mem", "reg/mem", "imm", "rel").
        self.data = ""         # The operand's data (possibly processed).
        self.flags = 0

        self.reg = ""          # Register operand's definition.
        self.mem = ""          # Memory operand's definition.
        self.imm = 0           # Immediate operand's size.
        self.rel = 0           # Relative displacement operand's size.

        self.restrict = ""     # Operand is restricted (specific register or immediate value).
        self.read = False      # True if the operand is a read-op from reg/mem.
        self.write = False     # True if the operand is a write-op to reg/mem.

        self.regType = ""      # Register operand's type.
        self.regIndexRel = 0   # Register index is relative to the previous register operand index (0 if not).
        self.memSize = -1      # Memory operand's size.
        self.immSign = ""      # Immediate sign (any / signed / unsigned).
        self.immValue = None   # Immediate value - `None` or `1` (only used by shift/rotate instructions).

        self.rwxIndex = -1     # Read/Write (RWX) index.
        self.rwxWidth = -1     # Read/Write (RWX) width.

    def _get_flag(self, flag):
        return (self.flags & flag) != 0

    def _set_flag(self, flag, value):
        self.flags = (self.flags & ~flag) | (flag if value else 0)
        return self

    @property
    def optional(self):
        return self._get_flag(OperandFlags.Optional)

    @optional.setter
    def optional(self, value):
        self._set_flag(OperandFlags.Optional, value)

    @property
    def implicit(self):
        return self._get_flag(OperandFlags.Implicit)

    @implicit.setter
    def implicit(self, value):
        self._set_flag(OperandFlags.Implicit, value)

    @property
    def commutative(self):
        return self._get_flag(OperandFlags.Commutative)

    @commutative.setter
    def commutative(self, value):
        self._set_flag(OperandFlags.Commutative, value)

    @property
    def zext(self):
        return self._get_flag(OperandFlags.ZExt)

    @zext.setter
    def zext(self, value):
        self._set_flag(OperandFlags.ZExt, value)

    def __str__(self):
        return self.data

    def is_reg(self):
        return bool(self.reg) and self.type != "reg-list"

    def is_mem(self):
        return bool(self.mem)

    def is_imm(self):
        return bool(self.imm)

    def is_rel(self):
        return bool(self.rel)

    def is_reg_mem(self):
        return bool(self.reg and self.mem)

    def is_reg_or_mem(self):
        return bool(self.reg or self.mem)

    def is_reg_list(self):
        return self.type == "reg-list"

    def is_partial_op(self):
        return False


# asmdb.base.Instruction
# ======================

_UNSET = object()


class Instruction:
    """Interface and properties that each architecture dependent instruction
    must provide even if that particular architecture doesn't use that
    feature(s)."""

    def __init__(self, db=None):
        self.db = db

        self.name = ""            # Instruction name.
        self.arch = "ANY"         # Architecture.
        self.encoding = ""        # Encoding type.
        self.operands = []        # Instruction operands.

        self.implicit = 0         # Indexes of all implicit operands (registers / memory).
        self.commutative = 0      # Indexes of all commutative operands.

        self.opcodeString = ""    # Instruction opcode as specified in manual.
        self.opcodeValue = 0      # Instruction opcode as number (arch dependent).
        self.fields = {}          # Information about each opcode field (arch dependent).
        self.operations = {}      # Operations the instruction performs.

        self.io = {}              # Instruction input / output (CPU flags, states, and other registers).
        self.ext = {}             # ISA extensions required by the instruction.
        self.category = {}        # Instruction categories.

        self.specialRegs = {}     # Information about read/write to special registers.

        self.alt = False          # This is an alternative form, not needed to create a signature.
        self.volatile = False     # Instruction is volatile and should not be reordered.
        self.control = "none"     # Control flow type (none by default).
        self.privilege = ""       # Privilege-level required to execute the instruction.
        self.aliasOf = ""         # Instruction is an alias of another instruction

    @property
    def ext_array(self):
        return sorted(self.ext.keys())

    @property
    def operand_count(self):
        return len(self.operands)

    @property
    def minimum_operand_count(self):
        for i, op in enumerate(self.operands):
            if op.optional:
                return i
        return len(self.operands)

    def _assign_attribute(self, key, value):
        if key in ("ext", "io", "category"):
            self._combine_attribute(key, value)
            return

        # JS intends to FAIL on unknown keys but `typeof this[key] ===
        # undefined` is always false (a JS bug); the port actually checks.
        if not hasattr(self, key):
            raise AttributeError(f"Cannot assign {key}={value}")
        setattr(self, key, value)

    def _combine_attribute(self, key, value):
        if isinstance(value, str):
            value = value.split(" ")

        target = getattr(self, key)
        if isinstance(value, (list, tuple)):
            for v in value:
                p_keys = v
                p_value = True

                i = v.find("=")
                if i != -1:
                    p_value = v[i + 1:]
                    p_keys = v[:i].strip()

                for pk in (s.strip() for s in p_keys.strip().split("|")):
                    target[pk] = p_value
        else:
            for k, v in value.items():
                target[k] = v

    def _update_operands_info(self):
        self.implicit = 0
        self.commutative = 0

        for i, op in enumerate(self.operands):
            if op.implicit:
                self.implicit |= (1 << i)
            if op.commutative:
                self.commutative |= (1 << i)

    def is_alias(self):
        return bool(self.aliasOf)

    def is_commutative(self):
        return self.commutative != 0

    def has_implicit(self):
        return self.implicit != 0

    def has_attribute(self, name, match_value=_UNSET):
        if not hasattr(self, name):
            return False
        if match_value is _UNSET:
            return True
        return getattr(self, name) == match_value

    def report(self, msg):
        print(f"{self}: {msg}")

    def __str__(self):
        return f"{self.name} {', '.join(str(op) for op in self.operands)}"


# asmdb.base.InstructionGroup
# ===========================

class InstructionGroup(list):
    """A list of instructions with some additional functionality."""

    def union_cpu_features(self):
        # The JS version takes an unused `name` parameter; dropped here.
        result = {}
        for instruction in self:
            result.update(instruction.ext)
        return result

    def check_attribute(self, key, value):
        return sum(1 for instruction in self if getattr(instruction, key, None) == value)


# Stand-in for the JS frozen empty group; identity-checked internally.
_EMPTY_GROUP = InstructionGroup()


# asmdb.base.ISA
# ==============

class ISA:
    def __init__(self):
        self._instructions = None       # Instruction list (all instructions), lazy.
        self._instruction_names = None  # Instruction names (sorted), regenerated when needed.
        self._instruction_map = {}      # Instruction name to `InstructionGroup` mapping.
        self._aliases = {}              # Instruction aliases (per primary).
        self._alias_map = {}            # Alias name -> primary name.
        self._cpu_levels = {}           # Architecture versions.
        self._extensions = {}           # Architecture extensions.
        self._attributes = {}           # Instruction attributes.
        self._special_regs = {}         # Special registers.
        self._shortcuts = {}            # Shortcuts used by instructions metadata.
        self.stats = {
            "instructions": 0,          # Number of all instructions.
            "groups": 0,                # Number of grouped instructions (having unique name).
        }

    @property
    def instructions(self):
        if self._instructions is None:
            array = []
            for name in self.instruction_names:
                array.extend(self._instruction_map[name])
            self._instructions = array
        return self._instructions

    @property
    def instruction_names(self):
        if self._instruction_names is None:
            self._instruction_names = sorted(self._instruction_map.keys())
        return self._instruction_names

    @property
    def instruction_map(self):
        return self._instruction_map

    @property
    def aliases(self):
        return self._alias_map

    @property
    def cpu_levels(self):
        return self._cpu_levels

    @property
    def extensions(self):
        return self._extensions

    @property
    def attributes(self):
        return self._attributes

    @property
    def special_regs(self):
        return self._special_regs

    @property
    def shortcuts(self):
        return self._shortcuts

    def query(self, args=None, copy=False):
        if args is None or isinstance(args, (str, list, tuple)):
            return self._query_by_name(args, copy)

        filter_fn = args.get("filter")
        if filter_fn:
            copy = False

        result = self._query_by_name(args.get("name"), copy)
        if filter_fn:
            result = [instruction for instruction in result if filter_fn(instruction)]

        return result

    def alias_data(self, name):
        return self._aliases.get(name)

    def _query_by_name(self, name, copy):
        if isinstance(name, str):
            result = self._instruction_map.get(name, _EMPTY_GROUP)
            return list(result) if copy else result

        if isinstance(name, (list, tuple)):
            result = _EMPTY_GROUP
            for n in name:
                instructions = self._instruction_map.get(n)
                if not instructions:
                    continue
                if result is _EMPTY_GROUP:
                    result = InstructionGroup()
                result.extend(instructions)
            return result

        result = self.instructions
        return list(result) if copy else result

    def for_each_group(self, cb):
        for name in self.instruction_names:
            cb(name, self._instruction_map[name])
        return self

    def add_data(self, data):
        if not isinstance(data, dict):
            _fail("ISA.add_data(): data argument must be object")

        if data.get("cpuLevels"):
            self._add_cpu_levels(data["cpuLevels"])
        if data.get("specialRegs"):
            self._add_special_regs(data["specialRegs"])
        if data.get("shortcuts"):
            self._add_shortcuts(data["shortcuts"])
        if data.get("instructions"):
            self._add_instructions(data["instructions"])
        if data.get("aliases"):
            self._add_aliases(data["aliases"])
        if data.get("postproc"):
            self._post_proc(data["postproc"])

    def _post_proc(self, groups):
        for group in groups:
            for i_rule in group["instructions"]:
                names = i_rule["name"].split(" ")
                for name in names:
                    instructions = self._instruction_map.get(name)
                    if not instructions:
                        _fail(f"Instruction {name} referenced by '{group['group']}' group doesn't exist")

                    for k, v in i_rule.items():
                        if k in ("name", "data"):
                            continue
                        for instruction in instructions:
                            instruction._assign_attribute(k, v)

    def _add_cpu_levels(self, items):
        if not isinstance(items, list):
            _fail("Property 'cpuLevels' must be array")

        for item in items:
            name = item["name"]
            self._cpu_levels[name] = {"name": name}

    def _add_extensions(self, items):
        if not isinstance(items, list):
            _fail("Property 'extensions' must be array")

        for item in items:
            name = item["name"]
            self._extensions[name] = {
                "name": name,
                "from": item.get("from") or "",
            }

    _RE_ATTRIBUTE_TYPE = re.compile(r"^(?:flag|string|string\[\])$")

    def _add_attributes(self, items):
        if not isinstance(items, list):
            _fail("Property 'attributes' must be array")

        for item in items:
            name = item["name"]
            type_ = item["type"]

            if not self._RE_ATTRIBUTE_TYPE.match(type_):
                _fail(f"Unknown attribute type '{type_}'")

            self._attributes[name] = {
                "name": name,
                "type": type_,
                "doc": item.get("doc") or "",
            }

    def _add_special_regs(self, items):
        if not isinstance(items, list):
            _fail("Property 'specialRegs' must be array")

        for item in items:
            name = item["name"]
            self._special_regs[name] = {
                "name": name,
                "group": item.get("group") or name,
                "doc": item.get("doc") or "",
            }

    def _add_shortcuts(self, items):
        if not isinstance(items, list):
            _fail("Property 'shortcuts' must be array")

        for item in items:
            name = item.get("name")
            expand = item.get("expand")

            if not name or not expand:
                _fail("Shortcut must contain 'name' and 'expand' properties")

            self._shortcuts[name] = {
                "name": name,
                "expand": expand,
                "doc": item.get("doc") or "",
            }

    def _add_instructions(self, instructions):
        raise NotImplementedError("ISA._add_instructions() must be reimplemented")

    def _add_instruction(self, instruction):
        if instruction.name in self._instruction_map:
            group = self._instruction_map[instruction.name]
        else:
            group = InstructionGroup()
            self._instruction_names = None
            self._instruction_map[instruction.name] = group
            self.stats["groups"] += 1

        if instruction.aliasOf:
            self._add_alias(instruction.name, instruction.aliasOf)

        group.append(instruction)
        self.stats["instructions"] += 1
        self._instructions = None

        return self

    # Add aliases from instruction database - aliases must be a dict where
    # each key is a non-aliased instruction.
    def _add_aliases(self, aliases):
        for instruction_name, data in aliases.items():
            for alias_name in data["aliases"]:
                self._add_alias(instruction_name, alias_name, data.get("format") or "")

    def _add_alias(self, instruction_name, alias_name, alias_format=""):
        # NOTE: identity check (not truthiness): a just-created group is
        # still empty when `_add_instruction` registers an `aliasOf`, and JS
        # `!group` only fails on undefined.
        group = self._instruction_map.get(instruction_name)
        if group is None:
            _fail(f"Instruction {instruction_name} doesn't exist when processing alias ({alias_name})")

        alias = self._aliases.get(instruction_name)
        if alias is None:
            alias = {
                "primaryName": instruction_name,
                "aliasNames": [],
                "format": "",
            }
            self._aliases[instruction_name] = alias

        self._alias_map[alias_name] = instruction_name
        alias["aliasNames"].append(alias_name)
        alias["format"] = alias_format or ""
