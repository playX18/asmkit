#!/usr/bin/env bash
# Materialize the pre-rewrite baseline and build the differential-test corpus
# generator inside it.
#
# The baseline is the git commit pinned below (the last pre-rewrite x86
# encoder). It is extracted READ-ONLY via `git archive` into
# target/difftest-baseline/ — no git mutations (no checkout/worktree/stash).
# target/ is gitignored, so the extracted tree never touches the repo.
#
# Idempotent: re-running reuses an existing extraction; pass --force to
# re-extract from scratch.
#
# After this script completes, generate a corpus with e.g.:
#   cd target/difftest-baseline
#   ./target/debug/corpus_gen --out ../../meta/difftest/corpus_sample.bin --max-per-form 1
# (or use --release for the full corpus; see meta/difftest/FORMAT.md).

set -euo pipefail

BASELINE_COMMIT=35caa377d68c4ba3b4577f691f3673f91d4338aa
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BASELINE_DIR="$REPO_ROOT/target/difftest-baseline"
DIFFTEST_DIR="$REPO_ROOT/meta/difftest"

if [[ "${1:-}" == "--force" ]]; then
    rm -rf "$BASELINE_DIR"
fi

# 1. Extract the baseline tree (read-only).
if [[ -d "$BASELINE_DIR/src" ]]; then
    echo "materialize: reusing existing $BASELINE_DIR"
else
    echo "materialize: extracting $BASELINE_COMMIT into $BASELINE_DIR"
    mkdir -p "$BASELINE_DIR"
    git -C "$REPO_ROOT" archive "$BASELINE_COMMIT" | tar -x -C "$BASELINE_DIR"
fi

# 2. Add iced-x86 to the baseline's [dependencies] (the corpus generator's
#    verification pass decodes every record; src/bin targets cannot use
#    dev-dependencies, so the dev-dependency alone is not enough). The
#    baseline tree is a throwaway copy; the real repo is untouched.
if ! grep -q "# difftest: iced-x86 for corpus_gen" "$BASELINE_DIR/Cargo.toml"; then
    echo "materialize: patching baseline Cargo.toml (add iced-x86 dependency)"
    sed -i '/^smallvec = /a # difftest: iced-x86 for corpus_gen (mirrors the dev-dependency;\n# src/bin targets cannot use dev-dependencies).\niced-x86 = "1.21"' "$BASELINE_DIR/Cargo.toml"
fi

# The baseline release profile sets panic="abort"; the corpus generator relies
# on catch_unwind to skip encoder paths that panic, so force unwind in the
# throwaway tree.
if grep -q '^panic = "abort"$' "$BASELINE_DIR/Cargo.toml"; then
    echo "materialize: relaxing baseline release panic=abort -> unwind"
    sed -i 's/^panic = "abort"$/panic = "unwind" # difftest: corpus_gen uses catch_unwind/' "$BASELINE_DIR/Cargo.toml"
fi

# 3. Install the corpus generator and regenerate the recipe table from the
#    baseline's dynamic instruction API.
mkdir -p "$BASELINE_DIR/src/bin"
cp "$DIFFTEST_DIR/corpus_gen.rs" "$BASELINE_DIR/src/bin/corpus_gen.rs"
cp "$DIFFTEST_DIR/corpus_dump.rs" "$BASELINE_DIR/src/bin/corpus_dump.rs"
python3 "$DIFFTEST_DIR/extract_recipes.py" \
    "$BASELINE_DIR/src/x86/features/_DYN.rs" \
    "$BASELINE_DIR/src/bin/corpus_recipes.rs"

# 4. Build the generator (x86-dyn is the baseline's dynamic-instruction API).
echo "materialize: building corpus_gen"
(cd "$BASELINE_DIR" && cargo build --release --no-default-features --features x86-dyn --bin corpus_gen --bin corpus_dump)

echo "materialize: done. Generator: $BASELINE_DIR/target/release/corpus_gen"
