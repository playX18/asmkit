#!/usr/bin/env python3
"""Extracts per-instruction documentation from the riscv-unified-db clone.

Reads the flat instruction YAML files at `<unified-db>/arch/inst/<Ext>/<name>.yaml`
and returns `{instruction_name: {long_name, description, assembly}}` for use by
`meta/riscv.py`, which renders them as `///` doc comments on the generated
`Opcode` enum and emitter methods.

Input: riscv-unified-db @ v0.1.0 (see meta/README.md for the pin), BSD-3-Clause-Clear.
Located via the `RISCV_UNIFIED_DB` env var, defaulting to the repo-root
`riscv-unified-db/` clone.

Dependency note: PyYAML is intentionally NOT required — it is not available in the
build environment, and the instruction YAML files are flat and regular: the fields
we need (`name`, `long_name`, `description`, `assembly`) are top-level keys whose
values are either inline scalars or `|` block scalars indented by two spaces. A
line-based extractor is sufficient and keeps the generator dependency-free.

Run standalone to print extraction stats:

    python3 docenizer_riscv.py
"""

import glob
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

FIELDS = ("name", "long_name", "description", "assembly")


def unified_db_root():
    return os.environ.get("RISCV_UNIFIED_DB", os.path.join(REPO_ROOT, "riscv-unified-db"))


def extract_inst_doc(path):
    """Line-based extractor for the flat instruction YAML files (see module docstring).

    Returns None when the file has no `name` key (not an instruction file).
    """
    doc = {}
    with open(path, encoding="utf-8") as fp:
        lines = fp.read().splitlines()

    i = 0
    while i < len(lines):
        line = lines[i]
        i += 1
        if not line or line[0] in (" ", "#", "-"):
            continue
        key, sep, value = line.partition(":")
        if not sep or key not in FIELDS or key in doc:
            continue
        value = value.strip()
        if value == "|":
            # Block scalar: indented lines (or blanks) until the next top-level key.
            block = []
            while i < len(lines) and (lines[i].startswith("  ") or not lines[i].strip()):
                block.append(lines[i][2:] if lines[i].startswith("  ") else "")
                i += 1
            while block and not block[-1].strip():
                block.pop()
            value = "\n".join(block)
        doc[key] = value

    # riscv-unified-db fills undocumented instructions with placeholder strings;
    # treat them as missing so the generator falls back to a syntax line.
    for key, placeholder in (
        ("long_name", "No synopsis available."),
        ("description", "No description available."),
    ):
        if doc.get(key) == placeholder:
            del doc[key]

    return doc if "name" in doc else None


def load_inst_docs(root=None):
    """Loads `{name: {long_name, description, assembly}}` for every instruction YAML."""
    if root is None:
        root = unified_db_root()
    docs = {}
    for path in sorted(glob.glob(os.path.join(root, "arch", "inst", "*", "*.yaml"))):
        doc = extract_inst_doc(path)
        if doc is not None:
            docs[doc["name"]] = doc
    return docs


def main():
    docs = load_inst_docs()
    missing = {k: [f for f in ("long_name", "description", "assembly") if not d.get(f)] for k, d in docs.items()}
    incomplete = {k: v for k, v in missing.items() if v}
    print(f"loaded {len(docs)} instruction docs from {unified_db_root()}")
    print(f"{len(incomplete)} entries with missing fields")
    for name in ("add", "c.add", "fadd.d"):
        doc = docs.get(name)
        if doc is None:
            print(f"{name}: <missing>")
            continue
        print(f"{name}: long_name={doc.get('long_name')!r} assembly={doc.get('assembly')!r}")
        print(f"  description={doc.get('description', '')[:80]!r}")


if __name__ == "__main__":
    sys.exit(main())
