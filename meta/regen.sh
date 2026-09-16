#!/usr/bin/env bash
# meta/regen.sh - regenerate asmkit's generated sources.

set -euo pipefail
cd "$(dirname "$0")/.."

PY=python3
CHECK=0
[[ "${1:-}" == "--check" ]] && CHECK=1

ASMJIT_REV=0bd5787b54b575ed94bf32ac452153b34385c514
RISCV_OPCODES_REV=c6edca7d8c3f92694963a0a0baeb511930fb2af4
RISCV_UNIFIED_DB_REV=92d67fdaad0cf314701179b879c8b1fb796ade6c

verify_revision() {
  local name=$1 path=$2 expected=$3 required=$4
  if [[ ! -e "$path/.git" ]]; then
    if [[ "$required" == "1" ]]; then
      echo "$name input is missing or is not a git checkout: $path" >&2
      exit 1
    fi
    return
  fi

  local actual
  actual=$(git -C "$path" rev-parse HEAD)
  if [[ "$actual" != "$expected" ]]; then
    echo "$name input revision mismatch: expected $expected, got $actual" >&2
    exit 1
  fi
}

ASMJIT_ROOT=${ASMJIT_SRC:-meta/asmjit}
RISCV_OPCODES_ROOT=${RISCV_OPCODES:-riscv-opcodes}
RISCV_UNIFIED_DB_ROOT=${RISCV_UNIFIED_DB:-riscv-unified-db}
if [[ "${ASMKIT_SKIP_REVISION_CHECK:-0}" != "1" ]]; then
  verify_revision AsmJit "$ASMJIT_ROOT" "$ASMJIT_REV" 1
  verify_revision riscv-opcodes "$RISCV_OPCODES_ROOT" "$RISCV_OPCODES_REV" 1
  verify_revision riscv-unified-db "$RISCV_UNIFIED_DB_ROOT" "$RISCV_UNIFIED_DB_REV" 0
fi

GENERATED_FILES=(
  src/x86/instdb.rs
  src/aarch64/instdb.rs
  src/aarch64/emit.rs
  src/aarch64/rwflags.rs
  src/x86/emitter.rs
  src/aarch64/emitter.rs
  src/riscv/opcodes.rs
  src/riscv/emitter.rs
  src/riscv/instdb.rs
)

if [[ "$CHECK" == "1" ]]; then
  source_root=$PWD
  temp_root=$(mktemp -d)
  trap 'rm -rf "$temp_root"' EXIT

  tar --exclude='./.git' --exclude='./target' --exclude='./.omo' -cf - . \
    | tar -xf - -C "$temp_root"
  (cd "$temp_root" && ASMKIT_SKIP_REVISION_CHECK=1 bash meta/regen.sh)

  status=0
  for file in "${GENERATED_FILES[@]}"; do
    if ! cmp -s "$source_root/$file" "$temp_root/$file"; then
      echo "generated file is stale: $file" >&2
      status=1
    fi
  done
  exit "$status"
fi

step() { echo "==> $*"; }

step "asmjit2rust.py (x86, aarch64)"
$PY meta/asmjit2rust.py all

step "asmjit_db tablegen (x86, aarch64)"
$PY -m meta.asmjit_db.tablegen_x86 --check
$PY -m meta.asmjit_db.tablegen_a64 --check
cp meta/asmjit_db/out/aarch64_rw_flags.rs src/aarch64/rwflags.rs

step "x86_emitter_gen.py"
$PY meta/x86_emitter_gen.py src/x86/emitter.rs

step "arm64.py"
$PY meta/arm64.py --features-output src/aarch64/emit.rs src/aarch64/emitter.rs

step "riscv.py"
RISCV_OPCODES="$RISCV_OPCODES_ROOT" RISCV_UNIFIED_DB="$RISCV_UNIFIED_DB_ROOT" \
  $PY meta/riscv.py 'rv*' -rust

step "rustfmt"
cargo fmt -- src/x86/instdb.rs src/aarch64/instdb.rs src/aarch64/rwflags.rs \
  src/x86/emitter.rs src/aarch64/emit.rs src/aarch64/emitter.rs src/riscv/opcodes.rs \
  src/riscv/emitter.rs src/riscv/instdb.rs 2>/dev/null

step "aarch64 selfcheck"
selfcheck_output=$(cargo test --test aarch64_selfcheck --features aarch64,validation -- --nocapture 2>&1)
echo "$selfcheck_output"
if [[ "$selfcheck_output" != *"0 encoder panics"* ]]; then
  echo "aarch64 selfcheck reported an encoder panic" >&2
  exit 1
fi

step "done"
