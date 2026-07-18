#!/usr/bin/env bash
# meta/regen.sh — regenerate asmkit's generated sources.
#
# Pipeline order matters: the C++→Rust translator (Tool 1) is the ground truth; the
# Python tablegen (Tool 2) must reproduce its output; emitters/instdbs for each arch
# follow. Run from the repo root: `bash meta/regen.sh` (add `--check` for CI: fails if
# regeneration would change any tracked file).
#
# Required inputs (see meta/README.md for pins): meta/asmjit/ (ASMJIT_SRC override ok),
# riscv-opcodes/ (RISCV_OPCODES), riscv-unified-db/ (docs), asm-docs*/ (optional docs).
set -euo pipefail
cd "$(dirname "$0")/.."

PY=python3
CHECK=0
[[ "${1:-}" == "--check" ]] && CHECK=1

step() { echo "==> $*"; }

# 1. Tool 1: asmjit C++ instdb -> Rust (x86 + aarch64) + meta/*_rows.json
step "asmjit2rust.py (x86, aarch64)"
$PY meta/asmjit2rust.py all

# 2. Tool 2: Python tablegen (db JSON + rows -> Rust) — validates against Tool 1 output.
step "asmjit_db tablegen (x86, aarch64)"
$PY -m meta.asmjit_db.tablegen_x86 --check
$PY -m meta.asmjit_db.tablegen_a64 --check
# The a64 NZCV table is an asmkit extension with no Tool 1 counterpart: install it.
cp meta/asmjit_db/out/aarch64_rw_flags.rs src/aarch64/rwflags.rs

# 3. x86 emitter traits
step "x86_emitter_gen.py"
$PY meta/x86_emitter_gen.py src/x86/emitter.rs

# 4. RISC-V (opcodes, emitter, instdb with effects, docs)
step "riscv.py"
RISCV_OPCODES="${RISCV_OPCODES:-riscv-opcodes}" $PY meta/riscv.py 'rv*' -rust

# 5. Format generated Rust.
step "rustfmt"
cargo fmt -- src/x86/instdb.rs src/aarch64/instdb.rs src/aarch64/rwflags.rs \
  src/x86/emitter.rs src/riscv/opcodes.rs src/riscv/emitter.rs src/riscv/instdb.rs 2>/dev/null || true

if [[ "$CHECK" == "1" ]]; then
  step "git diff --exit-code (tracked generated files)"
  git diff --exit-code -- src/x86/instdb.rs src/aarch64/instdb.rs src/aarch64/rwflags.rs \
    src/x86/emitter.rs src/riscv/opcodes.rs src/riscv/emitter.rs src/riscv/instdb.rs
fi

step "done"
