# This file is part of asmkit.
#
# Port of asmjit's `tools/generator-commons.js` (asmjit pinned at
# 0bd5787b54b575ed94bf32ac452153b34385c514, SPDX-License-Identifier: Zlib).

"""Generator commons: object/array/string utilities and `IndexedArray`.

Only the pieces the Python pipeline needs are ported. Method names are
snake_case; see README.md for behavior divergences from the JS original.
"""

import functools
import re


class ObjectUtils:
    """Port of the JS `ObjectUtils` class."""

    @staticmethod
    def clone(map_):
        # Shallow copy, like JS `Object.assign(Object.create(null), map)`.
        return dict(map_)

    @staticmethod
    def merge(a, b):
        # Recursively merges dict values of `b` into `a`; anything else is
        # assigned. JS recurses on any pair of `typeof "object"` values; in
        # practice only plain objects occur, so the port recurses on dicts.
        if a is b:
            return a

        for k, bv in b.items():
            av = a.get(k)
            if isinstance(av, dict) and isinstance(bv, dict):
                ObjectUtils.merge(av, bv)
            else:
                a[k] = bv

        return a

    @staticmethod
    def equals(a, b):
        if a is b:
            return True

        # JS `typeof` separates booleans from numbers; Python does not.
        if isinstance(a, bool) or isinstance(b, bool):
            return isinstance(a, bool) and isinstance(b, bool) and a == b

        a_list = isinstance(a, (list, tuple))
        b_list = isinstance(b, (list, tuple))
        if a_list or b_list:
            if a_list != b_list or len(a) != len(b):
                return False
            return all(ObjectUtils.equals(x, y) for x, y in zip(a, b))

        if isinstance(a, dict) != isinstance(b, dict):
            return False

        if isinstance(a, dict):
            if len(a) != len(b):
                return False
            for k, v in a.items():
                if k not in b or not ObjectUtils.equals(v, b[k]):
                    return False
            return True

        return a == b

    @staticmethod
    def equals_except(a, b, except_keys):
        # `except_keys` is a set/dict of top-level keys skipped during the
        # comparison; skipped keys are ignored on both sides, as in JS.
        if a is b:
            return True

        if not isinstance(a, dict) or not isinstance(b, dict):
            return ObjectUtils.equals(a, b)

        for k, v in a.items():
            if k in except_keys:
                continue
            if k not in b or not ObjectUtils.equals(v, b[k]):
                return False

        for k in b:
            if k not in except_keys and k not in a:
                return False

        return True

    @staticmethod
    def find_key(map_, keys):
        # JS uses `for..in` over `keys`, which iterates array indices; all JS
        # callers pass plain objects, so the port iterates the keys/values
        # given, which matches every real call site.
        for key in keys:
            if key in map_:
                return key
        return None

    @staticmethod
    def has_any(map_, keys):
        for key in keys:
            if key in map_:
                return True
        return False


class ArrayUtils:
    """Port of the JS `ArrayUtils` class."""

    @staticmethod
    def min(arr, fn=None):
        if not arr:
            return None

        if fn is None:
            fn = _nop

        v = fn(arr[0])
        for i in range(1, len(arr)):
            x = fn(arr[i])
            if x < v:
                v = x
        return v

    @staticmethod
    def max(arr, fn=None):
        if not arr:
            return None

        if fn is None:
            fn = _nop

        v = fn(arr[0])
        for i in range(1, len(arr)):
            x = fn(arr[i])
            if x > v:
                v = x
        return v

    @staticmethod
    def sorted(obj, cmp=None):
        # Array input -> sorted copy; dict input -> sorted list of key names,
        # like JS `Object.getOwnPropertyNames(obj).sort(cmp)`.
        out = list(obj) if isinstance(obj, (list, tuple)) else list(obj.keys())
        if cmp is not None:
            out.sort(key=functools.cmp_to_key(cmp))
        else:
            out.sort()
        return out

    @staticmethod
    def deep_index_of(arr, what):
        for i, item in enumerate(arr):
            if ObjectUtils.equals(item, what):
                return i
        return -1

    @staticmethod
    def to_dict(arr, value=True):
        out = {}
        for k in arr:
            out[k] = value
        return out


def _nop(x):
    return x


class StringUtils:
    """Port of the JS `StringUtils` class (subset)."""

    @staticmethod
    def as_string(x):
        return str(x)

    @staticmethod
    def make_enum_name(name):
        return name[0].upper() + name[1:] if name else ""

    @staticmethod
    def count_of(s, pattern):
        if not pattern:
            raise ValueError("Pattern cannot be empty")

        n = 0
        pos = 0

        while True:
            pos = s.find(pattern, pos)
            if pos < 0:
                return n
            n += 1
            pos += len(pattern)

    @staticmethod
    def trim_left(s):
        return re.sub(r"^\s+", "", s)

    @staticmethod
    def trim_right(s):
        return re.sub(r"\s+$", "", s)

    @staticmethod
    def trim(s):
        return re.sub(r"\s+$", "", re.sub(r"^\s+", "", s))

    @staticmethod
    def up_first(s):
        if not s:
            return ""
        return s[0].upper() + s[1:]

    @staticmethod
    def dec_to_hex(n, n_pad=0):
        # Like JS: a negative value is wrapped once into the u32 range and
        # the output is uppercase. Values >= 2**32 are not wrapped (as JS).
        if n < 0:
            n += 0x100000000
        hex_ = format(n, "x")
        while n_pad > len(hex_):
            hex_ = "0" + hex_
        return "0x" + hex_.upper()

    @staticmethod
    def format(array, indent, show_index, map_fn=None):
        # Exact port of the JS table-body formatter:
        #   show_index == -1: flow-wrapped at 80 columns.
        #   show_index > 0:   one row per line with `// #i` ordinals, plus
        #                     `[ref=Nx]` when the array provides ref counts.
        #   otherwise (0):    comma-joined lines, one item per line.
        if map_fn is None:
            map_fn = StringUtils.as_string

        s = ""
        threshold = 80

        if show_index == -1:
            s += indent

        for i in range(len(array)):
            item = array[i]
            last = i == len(array) - 1

            if show_index != -1:
                s += indent

            s += map_fn(item)
            if show_index > 0:
                s += (" " if last else ",") + f" // #{i}"
                ref_count_of = getattr(array, "ref_count_of", None)
                if callable(ref_count_of):
                    s += f" [ref={ref_count_of(item)}x]"
            elif not last:
                s += ","

            if show_index == -1:
                if len(s) >= threshold - 1 and not last:
                    s += "\n" + indent
                    threshold += 80
                else:
                    if not last:
                        s += " "
            else:
                if not last:
                    s += "\n"

        return s

    @staticmethod
    def disclaimer(s):
        return ("// ------------------- Automatically generated, do not edit -------------------\n" +
                s +
                "// ----------------------------------------------------------------------------\n")

    @staticmethod
    def indent(s, indentation):
        if isinstance(indentation, int):
            indentation = " " * indentation

        lines = re.split(r"\r?\n", s)
        if indentation:
            lines = [(indentation + line) if line else line for line in lines]

        return "\n".join(lines)

    @staticmethod
    def make_priority_compare(priority_array):
        # Items present in `priority_array` order first; unknown items sort
        # last, ties broken alphabetically (JS maps unknowns to Infinity).
        order = {}
        for index, name in enumerate(priority_array):
            order[name] = index

        def compare(a, b):
            a_known = a in order
            b_known = b in order
            if a_known and b_known:
                return order[a] - order[b]
            if a_known != b_known:
                return -1 if a_known else 1
            return (a > b) - (a < b)

        return compare


# Generator - Indexed Array
# =========================

def _freeze(value):
    # Canonical structural form used as the IndexedArray dedup key. Unlike
    # the JS `JSON.stringify(item)` key this is insensitive to dict key
    # insertion order and distinguishes booleans from numbers (Python
    # `True == 1`; JS `true !== 1`).
    if isinstance(value, bool):
        return ("#bool", value)
    if isinstance(value, dict):
        return ("#dict", tuple(sorted((k, _freeze(v)) for k, v in value.items())))
    if isinstance(value, (list, tuple)):
        return ("#list", tuple(_freeze(v) for v in value))
    return value


def _key_of(item):
    # The dedup key is the item itself if it is a string, otherwise a
    # canonical structural serialization (see the module README note).
    return item if isinstance(item, str) else _freeze(item)


class IndexedArray:
    """Port of the JS `IndexedArray` dedup workhorse.

    `add_indexed(item)` returns `(index, is_new)`; a duplicate bumps the
    item's ref count. Iteration yields `(index, item, ref_count)` triples
    in first-seen insertion order, exactly matching the JS dedup ORDER.
    """

    def __init__(self, iterable=None):
        self._items = []
        self._ref_counts = []  # Parallel to _items.
        self._index = {}  # Dedup key -> position in _items.
        if iterable is not None:
            for item in iterable:
                self.add_indexed(item)

    def __len__(self):
        return len(self._items)

    def __getitem__(self, index):
        return self._items[index]

    def __iter__(self):
        for i, item in enumerate(self._items):
            yield (i, item, self._ref_counts[i])

    def ref_count_of(self, item):
        pos = self._index.get(_key_of(item))
        return self._ref_counts[pos] if pos is not None else 0

    def add_indexed(self, item):
        key = _key_of(item)
        pos = self._index.get(key)

        if pos is not None:
            self._ref_counts[pos] += 1
            return (pos, False)

        pos = len(self._items)
        self._index[key] = pos
        self._items.append(item)
        self._ref_counts.append(1)
        return (pos, True)
