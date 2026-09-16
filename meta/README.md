# asmkit code generation (`meta/`)

Generates the checked-in Rust sources under `src/{x86,aarch64,riscv}/` from external
instruction databases. Python 3.12+, `beautifulsoup4` for the docenizers.

```sh
bash meta/regen.sh --check   # CI: fails if regeneration would change a tracked file
bash meta/regen.sh           # regenerate
```

## External inputs (pinned, gitignored)

| Input | Path / env var | Pinned commit | License |
|---|---|---|---|
| AsmJit | `meta/asmjit` (`ASMJIT_SRC`) | `0bd5787b54b575ed94bf32ac452153b34385c514` | Zlib |
| riscv-opcodes | `riscv-opcodes` (`RISCV_OPCODES`) | `c6edca7d8c3f92694963a0a0baeb511930fb2af4` | BSD-3-Clause |
| riscv-unified-db | `riscv-unified-db` (`RISCV_UNIFIED_DB`) | `v0.1.0` | BSD-3-Clause-Clear |

`git clone <url> <path> && git -C <path> checkout <commit>` for each. `regen.sh` checks these
revisions before generating. x86/ARM doc inputs (`asm-docs*`) are optional and unpinned.

## Licensing

Keep the Zlib/BSD notices on files derived from the inputs above. Never commit the fetched
inputs themselves (already gitignored).
