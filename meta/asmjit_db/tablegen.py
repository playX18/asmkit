# This file is part of asmkit.
#
# Port of asmjit's `tools/tablegen.js` (asmjit pinned at
# 0bd5787b54b575ed94bf32ac452153b34385c514, SPDX-License-Identifier: Zlib).

"""Core table-generation framework: `Task`/`TableGen`, `InstructionNameData`
(the 5-bit/packed instruction-name compression), `IdEnum`/`NameTable` tasks,
and `generate_name_data()`.

The JS `Injector` (which patches `// ${Key:Begin/End}` regions inside asmjit's
own C++ sources) is not ported: this pipeline emits whole files, so generated
sections are collected into `TableGen.outputs` (`dict[str, str]`) instead.

Faithfulness notes (see README.md for the full divergence list):

- The small-encoding regex is the JS `/^[a-z0-4]{0,6}$/` — the empty name of
  `kIdNone` IS small-encoded (first table entry `0x80000000`).
- `index()`'s `min_prefix_size` keeps JS floating-point semantics
  (`name.length / 2 + 1`, e.g. 5.5 for length 9) in the
  `longest_prefix >= min_prefix_size` comparison.
- The candidate sort is a pure stable sort by descending name length; the JS
  tiebreak compares two objects (always 0), which is dead code.
- Dead JS artifacts are not ported: the `size === -1` checks (`size` is never
  -1), `getIndex()` (reads a never-assigned `map`), the `aliasMem`/`aliasMap`
  typo, and a `console.log` debug leftover for "jz".
"""

import re

from .gen_common import StringUtils

# Hex formatting helpers (port of generator-cxx.js `Utils.toHexRaw/toHex`).

def _to_hex_raw(val, pad=None):
    if val < 0:
        val = 0xFFFFFFFF + val + 1
    s = format(val, "x")
    if pad is not None and len(s) < pad:
        s = "0" * (pad - len(s)) + s
    return s.upper()


def _to_hex(val, pad=None):
    return "0x" + _to_hex_raw(val, pad)


# ============================================================================
# [InstructionNameData]
# ============================================================================

_RE_SMALL_NAME = re.compile(r"^[a-z0-4]{0,6}$")


def char_to_5bit(c):
    if "a" <= c <= "z":
        return 1 + (ord(c) - ord("a"))
    if "0" <= c <= "4":
        return 1 + 26 + (ord(c) - ord("0"))
    raise ValueError(f"Character '{c}' cannot be encoded into a 5-bit string")


class InstructionNameData:
    def __init__(self):
        self.names = []
        self.primary_table = []
        self.string_table = ""
        self.index_comment = []
        self.max_name_length = 0

    def add(self, name, alt=None):
        if alt is None:
            alt = ""
        if name == alt:
            alt = ""

        if self.max_name_length < len(name):
            self.max_name_length = len(name)

        self.names.append(name)

        # First try to encode the string with 5-bit characters that fit into
        # a 32-bit int.
        if _RE_SMALL_NAME.match(name) and not alt:
            index = 0
            for i, c in enumerate(name):
                index |= char_to_5bit(c) << (i * 5)

            self.index_comment.append(f"Small '{name}'.")
            self.primary_table.append(index | (1 << 31))
        elif alt:
            prefix_index = self.add_or_reference_string(name + chr(len(alt)) + alt)

            self.index_comment.append(f"Large '{name}' + '{alt}'")
            self.primary_table.append(prefix_index | (len(name) << 12) | (0xFFF << 16))
        else:
            self.index_comment.append("")
            self.primary_table.append(0)

    def index(self):
        k_max_prefix_size = 15
        k_max_suffix_size = 6
        names = []

        for idx in range(len(self.primary_table)):
            if self.primary_table[idx] == 0:
                names.append({"name": self.names[idx], "index": idx})

        # Stable sort by descending name length (the JS tiebreak compares two
        # objects, which is always 0, so this is exactly equivalent).
        names.sort(key=lambda entry: -len(entry["name"]))

        for entry in names:
            idx = entry["index"]
            name = entry["name"]

            done = False
            longest_prefix = 0
            longest_suffix = 0

            prefix = ""
            suffix = ""

            for i in range(min(len(name), k_max_prefix_size), 0, -1):
                prefix = name[:i]
                suffix = name[i:]

                prefix_index = self.string_table.find(prefix)
                suffix_index = self.string_table.find(suffix)

                # Matched both parts?
                if prefix_index != -1 and suffix == "":
                    done = True
                    break

                if prefix_index != -1 and suffix_index != -1:
                    done = True
                    break

                if prefix_index != -1 and longest_prefix == 0:
                    longest_prefix = len(prefix)

                if suffix_index != -1 and len(suffix) > longest_suffix:
                    longest_suffix = len(suffix)

                if len(suffix) == k_max_suffix_size:
                    break

            if not done:
                # JS floating-point semantics: 5.5 for odd lengths >= 8.
                min_prefix_size = len(name) / 2 + 1 if len(name) >= 8 else len(name) - 2

                prefix = ""
                suffix = ""

                if longest_prefix >= min_prefix_size:
                    prefix = name[:longest_prefix]
                    suffix = name[longest_prefix:]
                elif longest_suffix:
                    split_at = min(len(name) - longest_suffix, k_max_prefix_size)
                    prefix = name[:split_at]
                    suffix = name[split_at:]
                elif len(name) > k_max_prefix_size:
                    prefix = name[:k_max_prefix_size]
                    suffix = name[k_max_prefix_size:]
                else:
                    prefix = name
                    suffix = ""

            if suffix:
                prefix_index = self.add_or_reference_string(prefix)
                suffix_index = self.add_or_reference_string(suffix)

                self.primary_table[idx] = (prefix_index | (len(prefix) << 12) |
                                           (suffix_index << 16) | (len(suffix) << 28))
                self.index_comment[idx] = f"Large '{prefix}|{suffix}'."
            else:
                prefix_index = self.add_or_reference_string(prefix)

                self.primary_table[idx] = prefix_index | (len(prefix) << 12)
                self.index_comment[idx] = f"Large '{prefix}'."

    def add_or_reference_string(self, s):
        index = self.string_table.find(s)
        if index == -1:
            index = len(self.string_table)
            self.string_table += s
        return index

    def format_index_table(self, table_name):
        s = ""
        for i in range(len(self.primary_table)):
            s += _to_hex(self.primary_table[i], 8)
            s += "," if i != len(self.primary_table) - 1 else " "
            s += " // " + self.index_comment[i] + "\n"

        return f"const uint32_t {table_name}[] = {{\n{StringUtils.indent(s, '  ')}}};\n"

    def format_string_table(self, table_name):
        s = ""
        line = ""

        for i, c in enumerate(self.string_table):
            line += "\\x" + _to_hex_raw(ord(c), 2)

            if len(line) >= 115 or i == len(self.string_table) - 1:
                if s:
                    s += "\n"
                s += f'"{line}"'
                line = ""

        s += ";\n"

        return f"const char {table_name}[] =\n{StringUtils.indent(s, '  ')}\n"

    def get_size(self):
        return len(self.primary_table) * 4 + len(self.string_table)


# ============================================================================
# [Task]
# ============================================================================

class Task:
    """A base runnable task that can access the TableGen through `self.ctx`."""

    def __init__(self, name="", deps=None):
        self.ctx = None
        self.name = name
        self.deps = deps if deps is not None else []

    def emit(self, key, content, size=0):
        self.ctx.add_output(key, content, size)
        return self

    def run(self):
        raise NotImplementedError("Task.run(): Must be reimplemented")


# ============================================================================
# [TableGen]
# ============================================================================

class TableGen:
    """Main context used to run `Task`s with minimal dependency management.

    Replaces the JS `Injector`: generated sections are collected into
    `self.outputs` (with sizes in `self.table_sizes`) instead of being
    injected into asmjit's C++ sources.
    """

    def __init__(self, arch):
        self.arch = arch

        self.tasks = []
        self.task_map = {}

        self.insts = []
        self.inst_map = {}

        self.aliases = []
        self.alias_map = {}

        self.outputs = {}
        self.table_sizes = {}

    # --------------------------------------------------------------------------
    # [Task Management]
    # --------------------------------------------------------------------------

    def add_task(self, task):
        if not task.name:
            raise ValueError("TableGen.add_task(): Task must have a name")

        if task.name in self.task_map:
            raise ValueError(f"TableGen.add_task(): Task '{task.name}' already added")

        for dependency in task.deps:
            if dependency not in self.task_map:
                raise ValueError(
                    f"TableGen.add_task(): Dependency '{dependency}' of task '{task.name}' doesn't exist")

        self.tasks.append(task)
        self.task_map[task.name] = task

        task.ctx = self
        return self

    def run_tasks(self):
        tasks_done = {}
        pending = len(self.tasks)

        while pending:
            old_pending = pending
            arr_pending = []

            for task in self.tasks:
                if tasks_done.get(task.name):
                    continue

                if all(tasks_done.get(dependency) is True for dependency in task.deps):
                    task.run()
                    tasks_done[task.name] = True
                    pending -= 1
                else:
                    arr_pending.append(task.name)

            if old_pending == pending:
                raise RuntimeError(
                    f"TableGen.run_tasks(): Tasks '{'|'.join(arr_pending)}' stuck (cyclic dependency?)")

    # --------------------------------------------------------------------------
    # [Instruction Management]
    # --------------------------------------------------------------------------

    def add_instruction(self, inst):
        name = inst["name"]
        if name in self.inst_map:
            raise ValueError(f"TableGen.add_instruction(): Instruction '{name}' already added")

        inst["id"] = len(self.insts)
        self.insts.append(inst)
        self.inst_map[name] = inst

        return self

    def add_alias(self, alias, name):
        # JS assigns to `this.aliasMap` but initializes `this.aliasMem` (a
        # typo); the port keeps a single `alias_map`.
        self.aliases.append(alias)
        self.alias_map[alias] = name

        return self

    # --------------------------------------------------------------------------
    # [Outputs]
    # --------------------------------------------------------------------------

    def add_output(self, key, content, size=0):
        self.outputs[key] = content
        self.table_sizes[key] = size
        return self

    # --------------------------------------------------------------------------
    # [Run]
    # --------------------------------------------------------------------------

    def run(self):
        self.on_before_run()
        self.run_tasks()
        self.on_after_run()

    # --------------------------------------------------------------------------
    # [Hooks]
    # --------------------------------------------------------------------------

    def on_before_run(self):
        pass

    def on_after_run(self):
        pass


# ============================================================================
# [IdEnum]
# ============================================================================

class IdEnum(Task):
    def __init__(self, name="IdEnum", deps=None):
        super().__init__(name, deps)

    def comment(self, inst):
        raise NotImplementedError("IdEnum.comment(): Must be reimplemented")

    def run(self):
        insts = self.ctx.insts

        s = ""
        aliases = ""

        for i, inst in enumerate(insts):
            line = "kId" + inst["enum"] + ("" if i else " = 0") + ","
            text = self.comment(inst)

            if text:
                line = line.ljust(37) + "//!< " + text

            s += line + "\n"

            inst_aliases = inst.get("aliases")
            if inst_aliases:
                for alias_name in inst_aliases["aliasNames"]:
                    if aliases:
                        aliases += ",\n"
                    aliases += f"kId{StringUtils.make_enum_name(alias_name)} = kId{inst['enum']}"

        s += "_kIdCount"

        if aliases:
            s += ",\n\n" + "// Aliases.\n" + aliases + "\n"
        else:
            s += "\n"

        return self.emit("InstId", s)


# ============================================================================
# [NameTable]
# ============================================================================

class Output:
    def __init__(self):
        self.content = {}
        self.table_size = {}

    def add(self, id_, content, table_size=0):
        self.content[id_] = content
        self.table_size[id_] = table_size if isinstance(table_size, (int, float)) else 0


def generate_name_data(out, instructions, generate_aliases):
    none = "Inst::kIdNone"

    aliases = []
    alias_name_data = InstructionNameData()
    alias_link_data = []

    inst_first = [None] * 26
    inst_last = [None] * 26
    inst_name_data = InstructionNameData()

    for instruction in instructions:
        inst_aliases = instruction.get("aliases")

        if inst_aliases:
            inst_name_data.add(instruction["displayName"], inst_aliases["format"])
            for alias_name in inst_aliases["aliasNames"]:
                aliases.append({"name": instruction["name"], "alt": alias_name})
        else:
            inst_name_data.add(instruction["displayName"])

    # Stable sort by alias name (JS `cmp` on `alt`).
    aliases.sort(key=lambda alias: alias["alt"])

    for alias in aliases:
        alias_name_data.add(alias["alt"])
        alias_link_data.append(f"Inst::kId{StringUtils.make_enum_name(alias['name'])}")

    inst_name_data.index()
    alias_name_data.index()

    for inst in instructions:
        display_name = inst["displayName"]

        # JS computes NaN for an empty display name, which bypasses the range
        # check below and lands in a slot the 26-entry loop never reads.
        if not display_name:
            continue

        alpha_index = ord(display_name[0]) - ord("a")
        if alpha_index < 0 or alpha_index >= 26:
            raise ValueError(
                f"generate_name_data(): Invalid lookup character '{display_name[0]}' of '{display_name}'")

        if inst_first[alpha_index] is None:
            inst_first[alpha_index] = f"Inst::kId{inst['enum']}"
        inst_last[alpha_index] = f"Inst::kId{inst['enum']}"

    s = ""
    s += "const InstNameIndex InstDB::_inst_name_index = {{\n"
    for i in range(26):
        first_id = inst_first[i] or none
        last_id = inst_last[i] or none

        s += f"  {{ {first_id.ljust(22)}, {last_id.ljust(22)} + 1 }}"
        if i != 26 - 1:
            s += ","
        s += "\n"
    s += f"}}, uint16_t({inst_name_data.max_name_length})}};\n"
    s += "\n"
    s += inst_name_data.format_string_table("InstDB::_inst_name_string_table")
    s += "\n"
    s += inst_name_data.format_index_table("InstDB::_inst_name_index_table")

    data_size = inst_name_data.get_size() + 26 * 4

    if generate_aliases:
        s += "\n"
        s += alias_name_data.format_string_table("InstDB::alias_name_string_table")
        s += "\n"
        s += alias_name_data.format_index_table("InstDB::alias_name_index_table")
        s += "\n"
        s += ("const uint32_t InstDB::alias_index_to_inst_id_table[] = {\n" +
              StringUtils.format(alias_link_data, "  ", 1) + "\n};\n")

        data_size += alias_name_data.get_size()
        info = f"static constexpr uint32_t kAliasTableSize = {len(alias_link_data)};\n"
        out.add("NameDataInfo", StringUtils.disclaimer(info), 0)

    out.add("NameData", StringUtils.disclaimer(s), data_size)
    return out


class NameTable(Task):
    def __init__(self, name="NameTable", deps=None, generate_aliases=False):
        super().__init__(name, deps)
        self.generate_aliases = generate_aliases

    def run(self):
        output = Output()
        generate_name_data(output, self.ctx.insts, self.generate_aliases)

        self.emit("NameData", output.content["NameData"], output.table_size["NameData"])

        if self.generate_aliases:
            self.emit("NameDataInfo", output.content["NameDataInfo"], output.table_size["NameDataInfo"])
