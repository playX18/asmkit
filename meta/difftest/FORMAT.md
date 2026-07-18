# Differential-test corpus format

This document specifies the binary corpus format produced by `corpus_gen`
(the baseline-half generator of the x86 encoder differential test) and
consumed by the replay test (`tests/x86_differential.rs`) in the rewritten
tree.

A corpus file is a sequence of **records**. Each record describes one
concrete x86 instruction — mnemonic, operands, and encoder options (prefixes,
EVEX mask/rounding) — plus the exact machine-code bytes the pre-rewrite
("baseline") encoder produced for it. The replay test rebuilds every record
through the new encoder and requires byte-identical output.

All integers are little-endian. All offsets are in bytes.

## File header

| Size | Contents |
|------|----------|
| 8    | magic: ASCII `AKDFCOR1` |
| u32  | format flags (0) |
| u32  | reserved (0) |

Records follow until EOF; there is no record count in the header.

## Record

| Size | Contents |
|------|----------|
| u32  | payload length in bytes (everything after this field) |
| u16  | mnemonic length M |
| M    | mnemonic, lowercase ASCII (see below) |
| i64  | baseline packed opcode constant (debug identity only; replay must NOT need the baseline tables) |
| u8   | operand count N (0..=4) |
| u8   | opts0: bit0 = `lock`, bit1 = `rep` (F3), bit2 = `repnz` (F2) |
| u8   | opts1: bits0..2 = segment override id (0=none, 1..6 = es,cs,ss,ds,fs,gs); bits3..5 = ER/SAE mode (0=none, 1=sae, 2=rn, 3=rd, 4=ru, 5=rz); bit6 = EVEX mask present |
| u8   | EVEX mask register id (0..7; meaningful iff opts1 bit6 set) |
| u8   | label layout: 0xFF = no label; 0 = label bound at instruction offset; 1 = label bound at block start (padding before instruction); 2 = label bound after instruction + padding |
| i32  | label bind offset within the recorded block (−1 = no label) |
| u32  | instruction offset within the block |
| u32  | instruction length |
| —    | N operand descriptors (see below), in operand order |
| u32  | block length B |
| B    | block bytes (expected encoder output, label fixups already applied) |

### Mnemonic strings

Mnemonics are the strings accepted by the baseline's
`Instruction::from_mnem`, including its `_mask` / `_maskz` / `_er` /
`_sae` suffix convention for EVEX masking and embedded rounding (the
suffixes combine, e.g. `vcmppd_mask_sae`), the `sse_`/`sse2_`/`sse3_`/
`ssse3_`/`mmx_` ISA prefixes on legacy instruction names
(`sse_blendvpd`, `mmx_palignr`), and operand-source/direction infixes
(`vpbroadcastb_gp`, `vmovq_g2x`, `mov_s2g`). The replay side maps them
onto new-API instruction ids; the suffix is redundant with opts1 (mask
id / ER mode) but kept so the mapping is unambiguous.

Notes:

- Gather/scatter records (`vgather*`, `vpgather*`, `vpscatter*`,
  `vscatter*`) always carry an EVEX mask (opts1 bit6): the baseline
  encodes them as EVEX-only and errors without a mask, so no unmasked
  variants exist in the corpus.
- `_er`/`_sae` records always have a nonzero ER mode (2..5 = rn/rd/ru/
  rz); a plain variant would be byte-identical to rn-sae because the
  rounding consts carry EVEX.b already.

Synthetic size-qualified string-op mnemonics `movs16/32/64`, `stos16/32/64`,
`lods16/32/64`, `cmps16/32/64`, `scas16/32/64`, `ins16/32`, `outs16/32` also
appear: the baseline's `from_mnem("movs")` only yields the 8-bit form, so the
larger forms are recorded under these names (with opts0.rep/repnz as usual).
They denote the corresponding `movsw/movsd/movsq` (etc.) encodings.

### Operand descriptors

Each descriptor starts with a kind byte:

| Kind | Meaning |
|------|---------|
| 1    | register |
| 2    | memory |
| 3    | immediate |
| 4    | label (no further fields; see label layout) |

Register (kind 1):

| Size | Contents |
|------|----------|
| u8   | register type id (see table below) |
| u8   | register id (0..31) |

Memory (kind 2):

| Size | Contents |
|------|----------|
| u8   | base register type id (0 = no base; 3 = rip) |
| u8   | base register id |
| u8   | index register type id (0 = none) |
| u8   | index register id |
| u8   | scale shift (0..4; scale = 1 << shift) |
| u8   | operand size in bytes (0 = unsized) |
| u8   | address type (0 = default, 1 = absolute/moffs, 2 = rip-relative) |
| u8   | EVEX broadcast (0 = none, 1..6 = 1to2,1to4,1to8,1to16,1to32,1to64) |
| i64  | displacement (full value; for absolute addresses the absolute address) |

Immediate (kind 3):

| Size | Contents |
|------|----------|
| i64  | immediate value (as passed to the encoder; only the low bits of the encoded immediate width may be significant — the replay should pass the same value) |

### Register type ids

These mirror the `RegType` discriminants in `src/core/operand.rs`
(`u32 as u8`), so the replay can rebuild registers without any baseline
tables:

| Id | Type | Registers |
|----|------|-----------|
| 0  | None | (no register) |
| 3  | PC   | rip |
| 4  | Gp8Lo | al,cl,dl,bl,spl,bpl,sil,dil,r8b..r15b |
| 5  | Gp8Hi | ah,ch,dh,bh (ids 0..3) |
| 6  | Gp16 | ax..r15w |
| 7  | Gp32 | eax..r15d |
| 8  | Gp64 | rax..r15 |
| 13 | Vec128 | xmm0..31 |
| 14 | Vec256 | ymm0..31 |
| 15 | Vec512 | zmm0..31 |
| 17 | Mask | k0..7 |
| 18 | Extra (X86Mm) | mm0..7 |
| 19 | X86SReg | es,cs,ss,ds,fs,gs (ids 1..6) |
| 20 | X86CReg | cr0..15 |
| 21 | X86DReg | dr0..15 |
| 22 | X86St | st0..7 |
| 24 | X86Tmm | tmm0..7 |

## Labels and blocks

For jump/call/loop records the recorded **block** contains more than the
instruction: the label is bound inside the same block, and the recorded bytes
have all fixups applied. Layout values:

- **0 (self):** label bound at the instruction's own offset. The block is
  just the instruction; displacement is `-instruction_length`.
- **1 (far back):** label bound at block offset 0; 200 bytes of `0x90`
  padding precede the instruction (so the displacement never fits rel8).
  `instruction offset` = 200.
- **2 (far forward):** instruction first, then 200 bytes of `0x90` padding,
  then the label. Exercises the unbound-label fixup path.

The baseline encoder *always* emits rel32 form for label operands (it has no
rel8 label path); the recorded bytes reflect that. For `jmp`/`jcc` with
**immediate** operands the baseline picks rel8 when the value fits, else
rel32 — those records are ordinary no-label records.

Replay procedure for a label record: create a buffer, write the block bytes
before `instruction offset` as raw data (they are padding), bind a label at
`label bind offset` (layouts 0/1: before emitting), emit the instruction with
the label as its operand, then for layout 2 write the remaining padding and
bind, apply fixups, and compare the full block.

Note: loop/jcxz with a label fail in the baseline encoder (rel8-only
instructions, no rel32 fallback) and therefore only appear with immediates.

## Guarantees

- Every record was accepted by the baseline dynamic API and emitted without
  encoder errors.
- Every record's bytes were decoded with iced-x86 1.21 (64-bit) and checked
  against the recorded mnemonic, registers, memory fields (base/index/scale/
  displacement/broadcast), immediates, and branch targets. Records that fail
  this check are dropped, so some instructions are absent (see the
  generator's drop report): `_3dnow`, `vsm4key4`/`vsm4rnds4`,
  `urdmsr`/`uwrmsr`, `rmpread` (iced-x86 1.21 cannot decode them), a set of
  baseline-encoder quirks (see below), and symbol-based forms (relocations
  cannot be replayed byte-identically).
- Records whose decoded mnemonic legitimately differs from the recorded one
  are kept only for an explicit alias list inside `corpus_gen.rs`
  (`MNEM_ALIASES` plus the `sse_`/`mmx_` prefix and `_g2x`-style infix
  normalization), e.g. `movsx`→`movsxd`, `xchg eax,eax`→`nop`,
  `movs`→`movsb`, `fsave`→`fnsave`, `fwait`→`wait`.

## Baseline-encoder quirks visible in the corpus

These are behaviors of the pre-rewrite encoder that the corpus either
records or deliberately omits. The replay side should expect them:

- The x86 `ArchTraits::regs_signature` table in the baseline is misindexed
  by one (missing `SymTag` slot); `Reg::from_type_and_id` therefore maps
  several `RegType`s to wrong signatures (Gp64→None, Vec128→Zmm, …). The
  generator uses typed constructors; the new encoder must not inherit this.
- Label operands on jumps/calls always produce rel32 (the only `LabelUse`
  is `X86JmpRel32`). `loop`/`jcxz` with a label *error out* in the baseline
  (rel8-only, no fallback), so they appear only with immediates.
- Immediate-form jumps (`jmp`/`jcc`/`call` with an `imm` operand) pick rel8
  when the value fits, else rel32, via the ALT_TAB retry chain. The same
  chain picks accumulator (al/ax/eax/rax) short forms and imm8 forms
  (83 /r) before wider forms, and turns `xchg eax,eax` into `90` (nop).
- High-byte registers (ah..bh) are unusable: the dyn API only accepts
  Gp8Lo, and the encoder has no Gp8Hi id+4 mapping (it would silently
  encode ah as al). No high-byte records exist.
- `Mem::segment` is ignored by the encoder; segment prefixes come only
  from `Assembler::seg()/fs()/gs()` (recorded in opts1).
- Segment registers *as instruction operands* (`mov_s2g`, `mov_g2s`,
  `push_seg`, `pop_seg`) are mis-encoded: the encoder writes the asmkit
  sreg id (es=1..gs=6) where hardware numbering (es=0..gs=5) belongs.
  These four mnemonics are omitted.
- Some consts are missing REX.W / operand-width bits, so the encoder
  emits a different width than the operands say: `sse_pmovmskb` gp64 dest
  (emits eax), `mmx_pmovmskb`, `mmx_pextrw`, `sse_movmskpd/ps` gp64 forms,
  `vpbroadcastq_gp*` gp32 source (emits r64), `vpinsrq`/`vpextrq`/
  `sse_pinsrq`/`sse_pextrq` gp32 form (emits r64), `enqcmd(s)` gp32 form,
  `crc32` (gp32,gp64) form. Records whose operand width disagrees with
  the encoding are dropped by the verifier.
- `vfmaddcph`/`vfcmaddcph`/`vfmulcph`/`vfcmulcph` (+csh) families are
  emitted with pp=F3 where the architecture requires 66/F2, so nothing
  can decode them; dropped.
- `tilezero` errors inside the old encoder; omitted.
- The old encoder silently *ignores* an EVEX broadcast decorator on
  non-EVEX instructions instead of erroring (records with a broadcast
  mem only verify for instructions where the b bit is real).
- Shift/rotate with a register count only encodes CL; other 8-bit
  registers are silently treated as CL (dropped by the verifier).
- `movs` (and `stos`/`lods`/`cmps`/`scas`/`ins`/`outs`) via `from_mnem`
  yields only the 8-bit form; the larger forms are recorded under the
  synthetic size-qualified names above.
- `movsx` r64←r/m32 is recorded under `movsx` (bytes are `movsxd`, 63 /r);
  the `movsx` r16←r/m32 form the old API accepts does not exist in the
  architecture and is dropped.
- `Mem::from_u64` truncates the low half of an absolute address to 28
  bits; absolute-moffs records use small addresses only.
