"""Python port of asmjit's JavaScript codegen database layer.

Stdlib-only reimplementation of the asmjit pipeline pieces pinned at
meta/asmjit (SHA 0bd5787b54b575ed94bf32ac452153b34385c514):

- `gen_common` - port of `tools/generator-commons.js`.
- `exp`        - port of `db/exp.js` (expression parsing / evaluation).
- `base`       - port of `db/base.js` (instruction model and ISA container).
- `tablegen`   - port of `tools/tablegen.js` (table-generation framework).
- `cxx_src`    - vendored-C++ parsing / Rust emission machinery shared by
                 the arch drivers (`tablegen_x86.py`, `tablegen_a64.py`).

See README.md in this package for the deliberate JS -> Python divergences.
"""

from . import base, cxx_src, exp, gen_common, tablegen

__all__ = ["base", "cxx_src", "exp", "gen_common", "tablegen"]
