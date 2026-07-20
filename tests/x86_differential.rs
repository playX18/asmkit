#![cfg(all(feature = "x86", feature = "validation"))]
//! Differential validation of the rewritten x86 encoder.
//!
//! Four layers:
//!
//! 1. `corpus_sample` / `corpus_full`: replay the binary corpus produced by the
//!    pre-rewrite ("baseline") encoder (format: `meta/difftest/FORMAT.md`,
//!    magic `AKDFCOR1`) through the new `Assembler::emit_n` API and require
//!    byte-identical output. The sample corpus runs by default; the full
//!    corpus is opt-in via `ASMKIT_X86_CORPUS_FULL=1` (same checks, ~6x data).
//!
//! 2. `golden_smoke`: replay the instruction list of `meta/golden/smoke.cpp`
//!    and compare against the asmjit-built oracle (`target/x86_golden_smoke`
//!    when present, otherwise the expected bytes documented in `smoke.cpp`).
//!
//! 3. `decode_roundtrip`: sweep instruction ids x operand-signature combos
//!    from the instdb signature tables, assemble each with the new encoder,
//!    decode with iced-x86, and verify mnemonic + operands match what was
//!    assembled. Capstone is used as a sampled secondary cross-check.
//!
//! 4. `x86_32`: 32-bit mode differential — assemble a representative spread
//!    with asmkit's 32-bit encoder and with iced-x86's 32-bit `CodeAssembler`,
//!    byte-identical where both pick the same encoding form and
//!    decode-equivalent for known divergences (prefix order, branch
//!    shortening, xchg accumulator forms). Includes mode-gating checks:
//!    X64-only instructions/registers must be rejected by both assemblers.
//!
//! The baseline-mnemonic -> InstId table lives in `tests/x86dif/mnem_map.rs`
//! (generated from the corpus mnemonics joined with `src/x86/instdb.rs` enum
//! names, plus the alias conventions described in FORMAT.md: `_mask`/`_maskz`/
//! `_er`/`_sae` suffixes, `sse_`/`sse2_`/`sse3_`/`ssse3_`/`mmx_` prefixes and
//! `_gp`/`_g2x`-style infixes dropped). Regenerate it by re-running that join
//! over freshly dumped corpus mnemonics; aliases and special cases that the
//! join cannot express are handled in [`resolve_mnemonic`] below.

#[path = "x86dif/mnem_map.rs"]
mod mnem_map;

use std::collections::BTreeMap;

use asmkit::CodeBuffer;
use asmkit::x86::InstId;
use asmkit::x86::assembler::Assembler;
use asmkit::x86::operands::{AddrType, Broadcast, KReg, Mem, Reg, SReg};
use asmkit::{Arch, Environment};
use asmkit::{Label, Operand, OperandCast, RegType, imm};

const SAMPLE_CORPUS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/meta/difftest/corpus_sample.bin"
);
const FULL_CORPUS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/meta/difftest/corpus_full.bin");
const GOLDEN_SMOKE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/x86_golden_smoke");

const MAGIC: &[u8; 8] = b"AKDFCOR1";
const LL_NONE: u8 = 0xFF;
const LL_FAR_FWD: u8 = 2;

// ============================================================================
// Corpus parsing (meta/difftest/FORMAT.md).
// ============================================================================

#[derive(Debug)]
enum OpRec {
    Reg {
        rt: u8,
        id: u8,
    },
    Mem {
        base_rt: u8,
        base_id: u8,
        index_rt: u8,
        index_id: u8,
        shift: u8,
        size: u8,
        addr: u8,
        bcast: u8,
        disp: i64,
    },
    Imm(i64),
    Label,
}

#[derive(Debug)]
struct Record {
    mnem: String,
    opts0: u8,
    opts1: u8,
    kmask: u8,
    label_layout: u8,
    bind_off: i32,
    inst_off: u32,
    inst_len: u32,
    ops: Vec<OpRec>,
    block: Vec<u8>,
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn u8(&mut self) -> u8 {
        let v = self.data[self.pos];
        self.pos += 1;
        v
    }
    fn u16(&mut self) -> u16 {
        let v = u16::from_le_bytes(self.data[self.pos..self.pos + 2].try_into().unwrap());
        self.pos += 2;
        v
    }
    fn u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
        self.pos += 4;
        v
    }
    fn i32(&mut self) -> i32 {
        self.u32() as i32
    }
    fn i64(&mut self) -> i64 {
        let v = i64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
        self.pos += 8;
        v
    }
    fn bytes(&mut self, n: usize) -> &'a [u8] {
        let v = &self.data[self.pos..self.pos + n];
        self.pos += n;
        v
    }
}

fn parse_corpus(data: &[u8]) -> Vec<Record> {
    assert_eq!(&data[..8], MAGIC, "bad corpus magic");
    let mut cur = Cursor { data, pos: 16 };
    let mut records = Vec::new();
    while cur.pos < data.len() {
        let payload_len = cur.u32() as usize;
        let end = cur.pos + payload_len;
        let mlen = cur.u16() as usize;
        let mnem = std::str::from_utf8(cur.bytes(mlen)).unwrap().to_string();
        let _baseline_opcode = cur.i64();
        let op_count = cur.u8();
        let opts0 = cur.u8();
        let opts1 = cur.u8();
        let kmask = cur.u8();
        let label_layout = cur.u8();
        let bind_off = cur.i32();
        let inst_off = cur.u32();
        let inst_len = cur.u32();
        let mut ops = Vec::new();
        for _ in 0..op_count {
            ops.push(match cur.u8() {
                1 => OpRec::Reg {
                    rt: cur.u8(),
                    id: cur.u8(),
                },
                2 => {
                    let (base_rt, base_id, index_rt, index_id, shift, size, addr, bcast) = (
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                        cur.u8(),
                    );
                    let disp = cur.i64();
                    OpRec::Mem {
                        base_rt,
                        base_id,
                        index_rt,
                        index_id,
                        shift,
                        size,
                        addr,
                        bcast,
                        disp,
                    }
                }
                3 => OpRec::Imm(cur.i64()),
                4 => OpRec::Label,
                k => panic!("bad operand kind {k}"),
            });
        }
        let block_len = cur.u32() as usize;
        let block = cur.bytes(block_len).to_vec();
        assert_eq!(cur.pos, end, "record payload length mismatch");
        records.push(Record {
            mnem,
            opts0,
            opts1,
            kmask,
            label_layout,
            bind_off,
            inst_off,
            inst_len,
            ops,
            block,
        });
    }
    records
}

// ============================================================================
// Mnemonic resolution: baseline corpus mnemonic -> new-API instruction id.
// ============================================================================

/// How a corpus record is replayed through the new API.
enum Replay {
    /// Emit the recorded operands under this instruction id.
    Id(u32),
    /// String instruction (movs/cmps/stos/lods/scas/ins/outs): the corpus
    /// records zero operands, the new API needs the implicit operands spelled
    /// out. Carries (instruction id, operand size in bytes).
    StringOp(u32, u32),
    /// No equivalent instruction id exists in the new encoder's instdb.
    Skip(&'static str),
}

/// Baseline mnemonics for instructions the new instdb does not have (it is
/// generated from an older asmjit): keylocker, ENCLV, and a few scattered
/// others. Records for these are counted as skipped, not failures.
const NO_INST_ID: &[&str] = &[
    "aesdec128kl",
    "aesdec256kl",
    "aesdecwide128kl",
    "aesdecwide256kl",
    "aesenc128kl",
    "aesenc256kl",
    "aesencwide128kl",
    "aesencwide256kl",
    "encodekey128",
    "encodekey256",
    "encls",
    "enclu",
    "enclv",
    "erets",
    "eretu",
    "int1",
    "lkgs",
    "loadiwkey",
    "pbndkb",
    "rdmsrlist",
    "rmpquery",
    "wrmsrlist",
    "wrmsrns",
    "wrpkru",
    "xstore",
];

/// Baseline generic-mnemonic aliases that always encode condition code 0
/// (`o`), plus a few one-off aliases the mechanical map cannot express.
fn alias_inst_id(mnem: &str) -> Option<u32> {
    Some(match mnem {
        "jcc" => InstId::Jo as u32,
        "cmovcc" => InstId::Cmovo as u32,
        "setcc" => InstId::Seto as u32,
        "cmpccxadd" => InstId::Cmpoxadd as u32,
        "callf" => InstId::Lcall as u32,
        "jmpf" => InstId::Ljmp as u32,
        // Junk names from recipe extraction: baseline `from_mnem` maps these
        // to the 16-bit convert forms (66 98 / 66 99).
        "c_ex" => InstId::Cbw as u32,
        "c_sep" => InstId::Cwd as u32,
        "jcxz" => InstId::Jecxz as u32,
        "movsx" => InstId::Movsx as u32,
        // Baseline name for cmpxchg8b (0F C7 /1).
        "cmpxchgd" => InstId::Cmpxchg8b as u32,
        _ => return None,
    })
}

/// Size-qualified baseline mnemonics that the new instdb splits into
/// dword/qword instruction ids; the variant is picked from the first register
/// operand's type.
fn sized_alias_inst_id(mnem: &str, ops: &[OpRec]) -> Option<Option<u32>> {
    let (d, q) = match mnem {
        "incssp" => (InstId::Incsspd as u32, InstId::Incsspq as u32),
        "rdssp" => (InstId::Rdsspd as u32, InstId::Rdsspq as u32),
        "wrss" => (InstId::Wrssd as u32, InstId::Wrssq as u32),
        "wruss" => (InstId::Wrussd as u32, InstId::Wrussq as u32),
        _ => return None,
    };
    let is_q = ops.iter().any(|op| matches!(op, OpRec::Reg { rt: 8, .. }));
    Some(Some(if is_q { q } else { d }))
}

/// String-op family: mnemonic -> (instruction id, implicit-operand kind,
/// operand size). The size-qualified synthetic names (`movs16/32/64`, ...)
/// denote the larger forms; the plain names are the 8-bit forms.
fn string_op_inst(mnem: &str) -> Option<(u32, StrKind, u32)> {
    let (base, size) = if let Some(base) = mnem.strip_suffix("16") {
        (base, 2)
    } else if let Some(base) = mnem.strip_suffix("32") {
        (base, 4)
    } else if let Some(base) = mnem.strip_suffix("64") {
        (base, 8)
    } else {
        (mnem, 1)
    };
    let (id, kind) = match base {
        "movs" => (InstId::Movs as u32, StrKind::MemMem),
        "cmps" => (InstId::Cmps as u32, StrKind::MemMem),
        "stos" => (InstId::Stos as u32, StrKind::MemReg),
        "scas" => (InstId::Scas as u32, StrKind::RegMem),
        "lods" => (InstId::Lods as u32, StrKind::RegMem),
        "ins" => (InstId::Ins as u32, StrKind::MemDx),
        "outs" => (InstId::Outs as u32, StrKind::DxMem),
        _ => return None,
    };
    Some((id, kind, size))
}

#[derive(Clone, Copy)]
enum StrKind {
    MemMem,
    MemReg,
    RegMem,
    MemDx,
    DxMem,
}

fn resolve_mnemonic(mnem: &str, ops: &[OpRec]) -> Replay {
    // String ops first: the corpus records them with zero operands and the
    // size lives in the mnemonic suffix (plain = 8-bit form).
    if let Some((id, _, size)) = string_op_inst(mnem) {
        return Replay::StringOp(id, size);
    }
    if let Some(id) = alias_inst_id(mnem) {
        return Replay::Id(id);
    }
    if let Some(Some(id)) = sized_alias_inst_id(mnem, ops) {
        return Replay::Id(id);
    }
    if NO_INST_ID.contains(&mnem) {
        return Replay::Skip("instruction not present in the new instdb");
    }
    match mnem_map::MNEM_MAP.binary_search_by_key(&mnem, |(m, _)| m) {
        Ok(i) => Replay::Id(mnem_map::MNEM_MAP[i].1),
        Err(_) => Replay::Skip("no mnemonic mapping"),
    }
}

// ============================================================================
// Operand reconstruction.
// ============================================================================

fn reg_type(rt: u8) -> RegType {
    RegType::try_from(rt as u32).unwrap_or_else(|_| panic!("bad register type id {rt}"))
}

fn gp_di_mem(size: u32, base_id: u32) -> Mem {
    let mut mem = Mem::new();
    mem.set_base_type(RegType::Gp64);
    mem.set_base_id(base_id);
    mem.set_size(size);
    mem
}

/// Accumulator-sized GP register for string ops: al/ax/eax/rax by size.
fn ax_reg(size: u32) -> Reg {
    let rt = match size {
        1 => RegType::Gp8Lo,
        2 => RegType::Gp16,
        4 => RegType::Gp32,
        8 => RegType::Gp64,
        s => panic!("bad string-op size {s}"),
    };
    Reg::from_type_and_id(rt, 0)
}

const GP_DI: u32 = 7;
const GP_SI: u32 = 6;
const GP_DX: u32 = 2;

#[allow(clippy::too_many_arguments)]
fn build_mem(
    base_rt: u8,
    base_id: u8,
    index_rt: u8,
    index_id: u8,
    shift: u8,
    size: u8,
    addr: u8,
    bcast: u8,
    disp: i64,
    size_override: Option<u32>,
) -> Mem {
    let mut mem = Mem::new();
    if base_rt != 0 {
        mem.set_base_type(reg_type(base_rt));
        mem.set_base_id(base_id as u32);
    }
    if index_rt != 0 {
        mem.set_index_type(reg_type(index_rt));
        mem.set_index_id(index_id as u32);
        mem.set_shift(shift as u32);
    }
    if size != 0 {
        mem.set_size(size as u32);
    }
    // The baseline encoder sized reg/mem forms by the register and defaulted
    // unsized mem+imm forms per group, so the recorded mem size is often
    // meaningless (FORMAT.md excludes it from verification); the replay
    // retries other sizes when the recorded one does not reproduce the bytes.
    if let Some(s) = size_override {
        if bcast == 0 {
            mem.set_size(s);
        }
    }
    match addr {
        1 => mem.set_addr_type(AddrType::Abs),
        2 => mem.set_addr_type(AddrType::Rel),
        _ => {}
    }
    if bcast != 0 {
        let b = match bcast {
            1 => Broadcast::B1To2,
            2 => Broadcast::B1To4,
            3 => Broadcast::B1To8,
            4 => Broadcast::B1To16,
            5 => Broadcast::B1To32,
            6 => Broadcast::B1To64,
            b => panic!("bad broadcast id {b}"),
        };
        mem.set_broadcast(b);
    }
    if addr == 1 {
        mem.set_offset(disp);
    } else {
        mem.set_offset_lo32(disp as i32);
    }
    mem
}

fn build_operands(rec: &Record, label: Option<Label>, size_override: Option<u32>) -> Vec<Operand> {
    match resolve_mnemonic(&rec.mnem, &rec.ops) {
        Replay::StringOp(_, size) => {
            let (_, kind, _) = string_op_inst(&rec.mnem).unwrap();
            let dst = *gp_di_mem(size, GP_DI).as_operand();
            let src = *gp_di_mem(size, GP_SI).as_operand();
            let ax = *ax_reg(size).as_operand();
            let dx = *Reg::from_type_and_id(RegType::Gp16, GP_DX).as_operand();
            match kind {
                StrKind::MemMem => vec![dst, src],
                StrKind::MemReg => vec![dst, ax],
                StrKind::RegMem => vec![ax, src],
                StrKind::MemDx => vec![dst, dx],
                StrKind::DxMem => vec![dx, src],
            }
        }
        _ => {
            // FPU pop/compare forms: the baseline accepted two st operands
            // (the other being st0); the asmjit-model API takes only the
            // varying st register.
            let fpu_keep = match rec.mnem.as_str() {
                "faddp" | "fmulp" | "fdivp" | "fdivrp" | "fsubp" | "fsubrp" => Some(0),
                "fcomip" | "fucomip" => Some(1),
                _ => None,
            };
            if let Some(keep) = fpu_keep {
                if rec.ops.len() == 2
                    && rec
                        .ops
                        .iter()
                        .all(|op| matches!(op, OpRec::Reg { rt: 22, .. }))
                {
                    let OpRec::Reg { rt, id } = rec.ops[keep] else {
                        unreachable!()
                    };
                    return vec![*Reg::from_type_and_id(reg_type(rt), id as u32).as_operand()];
                }
            }
            // in/out: the 8-bit DX forms are recorded with zero operands.
            if rec.ops.is_empty() && rec.mnem == "in" {
                return vec![
                    *Reg::from_type_and_id(RegType::Gp8Lo, 0).as_operand(),
                    *Reg::from_type_and_id(RegType::Gp16, GP_DX).as_operand(),
                ];
            }
            if rec.ops.is_empty() && rec.mnem == "out" {
                return vec![
                    *Reg::from_type_and_id(RegType::Gp16, GP_DX).as_operand(),
                    *Reg::from_type_and_id(RegType::Gp8Lo, 0).as_operand(),
                ];
            }
            let mut ops: Vec<Operand> = rec
                .ops
                .iter()
                .map(|op| match *op {
                    OpRec::Reg { rt, id } => {
                        *Reg::from_type_and_id(reg_type(rt), id as u32).as_operand()
                    }
                    OpRec::Mem {
                        base_rt,
                        base_id,
                        index_rt,
                        index_id,
                        shift,
                        size,
                        addr,
                        bcast,
                        disp,
                    } => *build_mem(
                        base_rt,
                        base_id,
                        index_rt,
                        index_id,
                        shift,
                        size,
                        addr,
                        bcast,
                        disp,
                        size_override,
                    )
                    .as_operand(),
                    OpRec::Imm(v) => *imm(v).as_operand(),
                    OpRec::Label => *label.expect("label operand without label").as_operand(),
                })
                .collect();
            // extrq takes two immediates; the baseline recorded only the first.
            if rec.mnem == "sse_extrq" && ops.len() == 2 {
                ops.push(*imm(0i64).as_operand());
            }
            ops
        }
    }
}

// ============================================================================
// Tier-2: decode-level equivalence (iced-x86).
//
// Some corpus records cannot match byte-identically because the baseline
// encoder picked a different *encoding form* than the asmjit-model encoder
// for the same instruction semantics:
//   - baseline forces EVEX for EVEX-capable encodings where asmjit prefers VEX
//     (records under `_mask`/`_maskz` names without a mask);
//   - baseline emits VEX for PREFER_EVEX instructions (vpdpbusd, vpmadd52*,
//     vcvtneps2bf16, ...) where asmjit prefers EVEX;
//   - xchg accumulator short forms (91) vs ModRM (87 /r), movq 6E/7E/D6 form
//     selection;
//   - legacy prefix order (66 F3 vs F3 66 on 16-bit string ops);
//   - baseline drops the FWAIT prefix on FPU wait-forms (fsave vs fnsave
//     style), asmjit emits it.
// For these, both byte strings are decoded with iced-x86 and required to
// agree on mnemonic (modulo the fn*/f* wait alias), operands, EVEX mask /
// zeroing / rounding / SAE attributes, and branch targets.
// ============================================================================

fn decode_stream(bytes: &[u8]) -> Option<Vec<iced_x86::Instruction>> {
    let mut dec = iced_x86::Decoder::new(64, bytes, 0);
    let mut out = Vec::new();
    while dec.position() < bytes.len() {
        let instr = dec.decode();
        if instr.is_invalid() {
            return None;
        }
        out.push(instr);
    }
    Some(out)
}

fn mnem_eq(a: iced_x86::Mnemonic, b: iced_x86::Mnemonic) -> bool {
    use iced_x86::Mnemonic::*;
    if a == b {
        return true;
    }
    // Baseline drops FWAIT on FPU wait-forms; iced splits the names.
    matches!(
        (a, b),
        (Fsave, Fnsave)
            | (Fnsave, Fsave)
            | (Fstcw, Fnstcw)
            | (Fnstcw, Fstcw)
            | (Fstsw, Fnstsw)
            | (Fnstsw, Fstsw)
            | (Fstenv, Fnstenv)
            | (Fnstenv, Fstenv)
            | (Fclex, Fnclex)
            | (Fnclex, Fclex)
            | (Finit, Fninit)
            | (Fninit, Finit)
    )
}

fn is_imm_kind(kind: iced_x86::OpKind) -> bool {
    matches!(
        kind,
        iced_x86::OpKind::Immediate8
            | iced_x86::OpKind::Immediate8_2nd
            | iced_x86::OpKind::Immediate8to16
            | iced_x86::OpKind::Immediate8to32
            | iced_x86::OpKind::Immediate8to64
            | iced_x86::OpKind::Immediate16
            | iced_x86::OpKind::Immediate32
            | iced_x86::OpKind::Immediate32to64
            | iced_x86::OpKind::Immediate64
    )
}

fn imm_width(kind: iced_x86::OpKind) -> u32 {
    use iced_x86::OpKind::*;
    match kind {
        Immediate8 | Immediate8to16 | Immediate8to32 | Immediate8to64 | Immediate8_2nd => 8,
        Immediate16 => 16,
        Immediate32 | Immediate32to64 => 32,
        _ => 64,
    }
}

fn instr_equivalent(e: &iced_x86::Instruction, a: &iced_x86::Instruction) -> bool {
    if !mnem_eq(e.mnemonic(), a.mnemonic()) {
        return false;
    }
    if e.op_count() != a.op_count() {
        return false;
    }
    // xchg is commutative: the 90+r accumulator form decodes with the
    // operands swapped relative to the ModRM form.
    if e.mnemonic() == iced_x86::Mnemonic::Xchg
        && a.mnemonic() == iced_x86::Mnemonic::Xchg
        && e.op_count() == 2
        && e.op_kind(0) == iced_x86::OpKind::Register
        && e.op_kind(1) == iced_x86::OpKind::Register
        && a.op_kind(0) == iced_x86::OpKind::Register
        && a.op_kind(1) == iced_x86::OpKind::Register
    {
        let (e0, e1) = (e.op_register(0), e.op_register(1));
        let (a0, a1) = (a.op_register(0), a.op_register(1));
        return (e0 == a0 && e1 == a1) || (e0 == a1 && e1 == a0);
    }
    // EVEX attributes must agree (catches dropped masks/rounding).
    if e.has_op_mask() != a.has_op_mask()
        || e.op_mask() != a.op_mask()
        || e.zeroing_masking() != a.zeroing_masking()
        || e.rounding_control() != a.rounding_control()
        || e.suppress_all_exceptions() != a.suppress_all_exceptions()
        || e.segment_prefix() != a.segment_prefix()
    {
        return false;
    }
    for i in 0..e.op_count() as usize {
        let (ke, ka) = (e.op_kind(i as u32), a.op_kind(i as u32));
        // Immediate widths may differ (e.g. baseline's 66-prefixed push).
        if is_imm_kind(ke) && is_imm_kind(ka) {
            let (ve, va) = (e.immediate(i as u32), a.immediate(i as u32));
            if ve != va {
                let w = imm_width(ke).min(imm_width(ka)).min(64);
                let mask = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };
                if ve & mask != va & mask {
                    return false;
                }
            }
            continue;
        }
        if ke != ka {
            return false;
        }
        match ke {
            iced_x86::OpKind::Register => {
                if e.op_register(i as u32) != a.op_register(i as u32) {
                    return false;
                }
            }
            iced_x86::OpKind::Memory => {
                if e.memory_base() != a.memory_base()
                    || e.memory_index() != a.memory_index()
                    || e.memory_index_scale() != a.memory_index_scale()
                    || e.memory_displacement64() != a.memory_displacement64()
                    || e.is_broadcast() != a.is_broadcast()
                {
                    return false;
                }
            }
            iced_x86::OpKind::NearBranch16
            | iced_x86::OpKind::NearBranch32
            | iced_x86::OpKind::NearBranch64
                if e.near_branch_target() != a.near_branch_target() =>
            {
                return false;
            }
            _ => {}
        }
    }
    true
}

fn decode_equivalent(expected: &[u8], actual: &[u8]) -> bool {
    let (Some(mut e), Some(mut a)) = (decode_stream(expected), decode_stream(actual)) else {
        return false;
    };
    // The asmjit-model encoder emits FWAIT before FPU wait-forms, the
    // baseline did not; drop leading `wait` instructions from both streams.
    let not_wait = |i: &iced_x86::Instruction| i.mnemonic() != iced_x86::Mnemonic::Wait;
    e.retain(not_wait);
    a.retain(not_wait);
    e.len() == a.len() && e.iter().zip(a.iter()).all(|(e, a)| instr_equivalent(e, a))
}

// ============================================================================
// Replay.
// ============================================================================

fn apply_prefixes(asm: &mut Assembler, rec: &Record) {
    if rec.opts0 & 0x1 != 0 {
        asm.lock();
    }
    if rec.opts0 & 0x2 != 0 {
        asm.rep();
    }
    if rec.opts0 & 0x4 != 0 {
        asm.repnz();
    }
    let seg = rec.opts1 & 0x7;
    if seg != 0 {
        asm.seg(SReg::from_id(seg as u32));
    }
    match (rec.opts1 >> 3) & 0x7 {
        1 => {
            asm.sae();
        }
        2 => {
            asm.rn_sae();
        }
        3 => {
            asm.rd_sae();
        }
        4 => {
            asm.ru_sae();
        }
        5 => {
            asm.rz_sae();
        }
        _ => {}
    }
    if rec.opts1 & 0x40 != 0 {
        asm.k(KReg::from_id(rec.kmask as u32));
    }
    if rec.mnem.contains("maskz") {
        asm.z();
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Replays one record, returning the produced block bytes or an error string.
fn replay(rec: &Record, inst_id: u32, size_override: Option<u32>) -> Result<Vec<u8>, String> {
    let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
    let has_label = rec.label_layout != LL_NONE;
    let label = if has_label {
        Some(buf.get_label())
    } else {
        None
    };
    let ops = build_operands(rec, label, size_override);
    let refs: Vec<&Operand> = ops.iter().collect();

    if !has_label {
        {
            let mut asm = Assembler::new(&mut buf);
            apply_prefixes(&mut asm, rec);
            asm.emit_n(inst_id, &refs);
        }
    } else {
        let bind = rec.bind_off as usize;
        let ioff = rec.inst_off as usize;
        if rec.label_layout == LL_FAR_FWD {
            {
                let mut asm = Assembler::new(&mut buf);
                apply_prefixes(&mut asm, rec);
                asm.emit_n(inst_id, &refs);
            }
            if buf.error().is_none() {
                for &b in &rec.block[ioff + rec.inst_len as usize..bind] {
                    buf.put1(b);
                }
                buf.bind_label(label.unwrap());
            }
        } else {
            for &b in &rec.block[..bind] {
                buf.put1(b);
            }
            buf.bind_label(label.unwrap());
            for &b in &rec.block[bind..ioff] {
                buf.put1(b);
            }
            {
                let mut asm = Assembler::new(&mut buf);
                apply_prefixes(&mut asm, rec);
                asm.emit_n(inst_id, &refs);
            }
        }
    }
    if let Some(err) = buf.error() {
        return Err(format!("emit error: {err:?}"));
    }

    let out = if has_label {
        buf.finish().unwrap().data().to_vec()
    } else {
        buf.data().to_vec()
    };
    Ok(out)
}

#[derive(Default)]
struct ReplayStats {
    total: u64,
    ok: u64,
    size_recovered: u64,
    alt_encoding: u64,
    failed: u64,
    skipped: u64,
    failures_by_mnem: BTreeMap<String, u64>,
    alt_by_mnem: BTreeMap<String, u64>,
    skip_reasons: BTreeMap<&'static str, u64>,
    samples: Vec<String>,
}

/// Whether the record has a plain (non-broadcast) memory operand whose size
/// the baseline may have ignored.
fn has_resizable_mem(rec: &Record) -> bool {
    rec.ops.iter().any(|op| {
        matches!(
            op,
            OpRec::Mem {
                bcast: 0,
                addr: 0 | 2,
                ..
            }
        )
    })
}

/// Candidate mem sizes: the recorded one first (via `None` = as recorded),
/// then every architectural size, then unsized.
const SIZE_CANDIDATES: [u32; 8] = [1, 2, 4, 8, 16, 32, 64, 0];

fn run_corpus(path: &str, shard: Option<(usize, usize)>) -> ReplayStats {
    let data = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let records = parse_corpus(&data);
    let mut stats = ReplayStats::default();

    for (record_index, rec) in records.iter().enumerate() {
        if let Some((shard_index, shard_count)) = shard {
            if record_index % shard_count != shard_index {
                continue;
            }
        }
        stats.total += 1;
        let inst_id = match resolve_mnemonic(&rec.mnem, &rec.ops) {
            Replay::Id(id) => id,
            Replay::StringOp(id, _) => id,
            Replay::Skip(reason) => {
                stats.skipped += 1;
                *stats.skip_reasons.entry(reason).or_default() += 1;
                continue;
            }
        };

        // First attempt with the recorded operand sizes, then retry with
        // each candidate mem size (the recorded size is not reliable — the
        // baseline sized reg/mem forms by the register). Every successfully
        // emitted variant is a tier-2 (decode-equivalence) candidate.
        let first = replay(rec, inst_id, None);
        if matches!(&first, Ok(bytes) if *bytes == rec.block) {
            stats.ok += 1;
            continue;
        }

        let mut recovered = false;
        let mut alt_candidates: Vec<Vec<u8>> = Vec::new();
        if let Ok(bytes) = &first {
            alt_candidates.push(bytes.clone());
        }
        if has_resizable_mem(rec) {
            for &size in &SIZE_CANDIDATES {
                match replay(rec, inst_id, Some(size)) {
                    Ok(bytes) if bytes == rec.block => {
                        stats.ok += 1;
                        stats.size_recovered += 1;
                        recovered = true;
                        break;
                    }
                    Ok(bytes) => alt_candidates.push(bytes),
                    Err(_) => {}
                }
            }
        }
        if recovered {
            continue;
        }

        if alt_candidates
            .iter()
            .any(|bytes| decode_equivalent(&rec.block, bytes))
        {
            stats.alt_encoding += 1;
            *stats.alt_by_mnem.entry(rec.mnem.clone()).or_default() += 1;
            continue;
        }

        stats.failed += 1;
        *stats.failures_by_mnem.entry(rec.mnem.clone()).or_default() += 1;
        if stats.samples.len() < 100 {
            let detail = match &first {
                Ok(bytes) => format!("expected {}, got {}", hex(&rec.block), hex(bytes)),
                Err(e) => format!("{} (expected {})", e, hex(&rec.block)),
            };
            stats
                .samples
                .push(format!("FAIL {} {:?}: {}", rec.mnem, rec.ops, detail));
        }
    }

    eprintln!(
        "{path}: {} records, {} ok ({} size-recovered), {} alt-encoding, {} failed, {} skipped",
        stats.total,
        stats.ok,
        stats.size_recovered,
        stats.alt_encoding,
        stats.failed,
        stats.skipped
    );
    for (reason, n) in &stats.skip_reasons {
        eprintln!("  skipped: {n:6} {reason}");
    }
    eprintln!("  alt-encoding mnemonics: {}", stats.alt_by_mnem.len());
    let mut worst: Vec<_> = stats.failures_by_mnem.iter().collect();
    worst.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    for (mnem, n) in worst.iter() {
        eprintln!("  failures: {n:6} {mnem}");
    }
    for s in &stats.samples {
        eprintln!("  {s}");
    }
    stats
}

// ============================================================================
// Tier-3: expected differences (records the new encoder cannot reproduce).
//
// Every remaining baseline corpus record that fails both byte comparison and
// decode equivalence falls into one of these investigated classes. The table
// is a ratchet: the sample-corpus failure counts must match exactly, so any
// new failure (or a fix that makes an expected failure pass) turns the test
// red. Counts are asserted for corpus_sample; for corpus_full only the
// mnemonic/reason membership is asserted (counts scale with the grid).
// ============================================================================

const R_STRICT_SIG: &str = "strict signature validation: the instdb signature table rejects a form asmjit's emit arms accept opportunistically (asmjit runs no table validation by default)";
const R_BASELINE_FORM: &str = "baseline-only operand form: not expressible through the asmjit-model API (bogus width/prefix or register-for-memory operand)";
const R_SAE_RC: &str = "baseline emitted a rounding control ({rd/ru/rz-sae}) on an instruction whose tables only allow plain {sae}; asmjit validation rejects it";
const R_XCHG_LOCK: &str =
    "LOCK with xchg-mem: asmjit rejects (xchg is implicitly locked); baseline allowed it";

/// (mnemonic, expected failing records in corpus_sample, reason).
static EXPECTED_FAILURES: &[(&str, u32, &str)] = &[
    ("clzero", 3, R_BASELINE_FORM),
    ("enqcmd", 2, R_BASELINE_FORM),
    ("enqcmds", 2, R_BASELINE_FORM),
    ("enter", 2, R_BASELINE_FORM),
    ("invlpga", 1, R_STRICT_SIG),
    ("lar", 6, R_STRICT_SIG),
    ("lldt", 2, R_STRICT_SIG),
    ("lmsw", 2, R_STRICT_SIG),
    ("lsl", 4, R_STRICT_SIG),
    ("ltr", 2, R_STRICT_SIG),
    ("movdir64b", 2, R_BASELINE_FORM),
    ("movsx", 12, R_BASELINE_FORM),
    ("movzx", 2, R_STRICT_SIG),
    ("rsm", 1, R_STRICT_SIG),
    ("sse_extrq", 2, R_STRICT_SIG),
    ("ud0", 8, R_STRICT_SIG),
    ("ud1", 8, R_STRICT_SIG),
    ("umonitor", 2, R_BASELINE_FORM),
    ("vcmppd_sae", 1, R_SAE_RC),
    ("vcmpph_sae", 1, R_SAE_RC),
    ("vcmpps_sae", 1, R_SAE_RC),
    ("vcmpsd_sae", 1, R_SAE_RC),
    ("vcmpsh_sae", 1, R_SAE_RC),
    ("vcmpss_sae", 1, R_SAE_RC),
    ("vcomisd_sae", 1, R_SAE_RC),
    ("vcomish_sae", 1, R_SAE_RC),
    ("vcomiss_sae", 1, R_SAE_RC),
    ("vcvtdq2pd_mask_sae", 2, R_SAE_RC),
    ("vcvtdq2pd_maskz_sae", 2, R_SAE_RC),
    ("vcvtdq2pd_sae", 2, R_SAE_RC),
    ("vcvtph2pd_sae", 1, R_SAE_RC),
    ("vcvtph2ps_sae", 1, R_SAE_RC),
    ("vcvtph2psx_sae", 1, R_SAE_RC),
    ("vcvtps2ph_sae", 1, R_SAE_RC),
    ("vcvtsh2sd_sae", 1, R_SAE_RC),
    ("vcvtsh2ss_sae", 1, R_SAE_RC),
    ("vcvtss2sd_sae", 1, R_SAE_RC),
    ("vcvttpd2dq_sae", 1, R_SAE_RC),
    ("vcvttpd2qq_sae", 1, R_SAE_RC),
    ("vcvttpd2udq_sae", 1, R_SAE_RC),
    ("vcvttpd2uqq_sae", 1, R_SAE_RC),
    ("vcvttph2dq_sae", 1, R_SAE_RC),
    ("vcvttph2qq_sae", 1, R_SAE_RC),
    ("vcvttph2udq_sae", 1, R_SAE_RC),
    ("vcvttph2uqq_sae", 1, R_SAE_RC),
    ("vcvttph2uw_sae", 1, R_SAE_RC),
    ("vcvttph2w_sae", 1, R_SAE_RC),
    ("vcvttps2dq_sae", 1, R_SAE_RC),
    ("vcvttps2qq_sae", 1, R_SAE_RC),
    ("vcvttps2udq_sae", 1, R_SAE_RC),
    ("vcvttps2uqq_sae", 1, R_SAE_RC),
    ("vcvttsd2si_sae", 2, R_SAE_RC),
    ("vcvttsd2usi_sae", 2, R_SAE_RC),
    ("vcvttsh2si_sae", 2, R_SAE_RC),
    ("vcvttsh2usi_sae", 2, R_SAE_RC),
    ("vcvttss2si_sae", 2, R_SAE_RC),
    ("vcvttss2usi_sae", 2, R_SAE_RC),
    ("verr", 2, R_STRICT_SIG),
    ("verw", 2, R_STRICT_SIG),
    ("vfixupimmpd_sae", 1, R_SAE_RC),
    ("vfixupimmps_sae", 1, R_SAE_RC),
    ("vfixupimmsd_sae", 1, R_SAE_RC),
    ("vfixupimmss_sae", 1, R_SAE_RC),
    ("vgetexppd_sae", 1, R_SAE_RC),
    ("vgetexpph_sae", 1, R_SAE_RC),
    ("vgetexpps_sae", 1, R_SAE_RC),
    ("vgetexpsd_sae", 1, R_SAE_RC),
    ("vgetexpsh_sae", 1, R_SAE_RC),
    ("vgetexpss_sae", 1, R_SAE_RC),
    ("vgetmantpd_sae", 1, R_SAE_RC),
    ("vgetmantph_sae", 1, R_SAE_RC),
    ("vgetmantps_sae", 1, R_SAE_RC),
    ("vgetmantsd_sae", 1, R_SAE_RC),
    ("vgetmantsh_sae", 1, R_SAE_RC),
    ("vgetmantss_sae", 1, R_SAE_RC),
    ("vmaxpd_sae", 1, R_SAE_RC),
    ("vmaxph_sae", 1, R_SAE_RC),
    ("vmaxps_sae", 1, R_SAE_RC),
    ("vmaxsd_sae", 1, R_SAE_RC),
    ("vmaxsh_sae", 1, R_SAE_RC),
    ("vmaxss_sae", 1, R_SAE_RC),
    ("vminpd_sae", 1, R_SAE_RC),
    ("vminph_sae", 1, R_SAE_RC),
    ("vminps_sae", 1, R_SAE_RC),
    ("vminsd_sae", 1, R_SAE_RC),
    ("vminsh_sae", 1, R_SAE_RC),
    ("vminss_sae", 1, R_SAE_RC),
    ("vp2intersectd", 12, R_STRICT_SIG),
    ("vp2intersectq", 12, R_STRICT_SIG),
    ("vrangepd_sae", 1, R_SAE_RC),
    ("vrangeps_sae", 1, R_SAE_RC),
    ("vrangesd_sae", 1, R_SAE_RC),
    ("vrangess_sae", 1, R_SAE_RC),
    ("vreducepd_mask_sae", 2, R_SAE_RC),
    ("vreducepd_maskz_sae", 2, R_SAE_RC),
    ("vreducepd_sae", 2, R_SAE_RC),
    ("vreduceph_sae", 1, R_SAE_RC),
    ("vreduceps_mask_sae", 2, R_SAE_RC),
    ("vreduceps_maskz_sae", 2, R_SAE_RC),
    ("vreduceps_sae", 2, R_SAE_RC),
    ("vreducesd_mask_sae", 2, R_SAE_RC),
    ("vreducesd_maskz_sae", 2, R_SAE_RC),
    ("vreducesd_sae", 2, R_SAE_RC),
    ("vreducesh_sae", 1, R_SAE_RC),
    ("vreducess_mask_sae", 2, R_SAE_RC),
    ("vreducess_maskz_sae", 2, R_SAE_RC),
    ("vreducess_sae", 2, R_SAE_RC),
    ("vrndscalepd_sae", 1, R_SAE_RC),
    ("vrndscaleph_sae", 1, R_SAE_RC),
    ("vrndscaleps_sae", 1, R_SAE_RC),
    ("vrndscalesd_sae", 1, R_SAE_RC),
    ("vrndscalesh_sae", 1, R_SAE_RC),
    ("vrndscaless_sae", 1, R_SAE_RC),
    ("vucomisd_sae", 1, R_SAE_RC),
    ("vucomish_sae", 1, R_SAE_RC),
    ("vucomiss_sae", 1, R_SAE_RC),
    ("xchg", 4, R_XCHG_LOCK),
    // Masked {sae} forms only exercised by the full-corpus grid (count 0:
    // no failing records in corpus_sample).
    ("vcmppd_mask_sae", 0, R_SAE_RC),
    ("vcmpph_mask_sae", 0, R_SAE_RC),
    ("vcmpps_mask_sae", 0, R_SAE_RC),
    ("vcmpsd_mask_sae", 0, R_SAE_RC),
    ("vcmpsh_mask_sae", 0, R_SAE_RC),
    ("vcmpss_mask_sae", 0, R_SAE_RC),
    ("vcvtph2pd_mask_sae", 0, R_SAE_RC),
    ("vcvtph2pd_maskz_sae", 0, R_SAE_RC),
    ("vcvtph2ps_mask_sae", 0, R_SAE_RC),
    ("vcvtph2ps_maskz_sae", 0, R_SAE_RC),
    ("vcvtph2psx_mask_sae", 0, R_SAE_RC),
    ("vcvtph2psx_maskz_sae", 0, R_SAE_RC),
    ("vcvtps2ph_mask_sae", 0, R_SAE_RC),
    ("vcvtps2ph_maskz_sae", 0, R_SAE_RC),
    ("vcvtsh2sd_mask_sae", 0, R_SAE_RC),
    ("vcvtsh2sd_maskz_sae", 0, R_SAE_RC),
    ("vcvtsh2ss_mask_sae", 0, R_SAE_RC),
    ("vcvtsh2ss_maskz_sae", 0, R_SAE_RC),
    ("vcvtss2sd_mask_sae", 0, R_SAE_RC),
    ("vcvtss2sd_maskz_sae", 0, R_SAE_RC),
    ("vcvttpd2dq_mask_sae", 0, R_SAE_RC),
    ("vcvttpd2dq_maskz_sae", 0, R_SAE_RC),
    ("vcvttpd2qq_mask_sae", 0, R_SAE_RC),
    ("vcvttpd2qq_maskz_sae", 0, R_SAE_RC),
    ("vcvttpd2udq_mask_sae", 0, R_SAE_RC),
    ("vcvttpd2udq_maskz_sae", 0, R_SAE_RC),
    ("vcvttpd2uqq_mask_sae", 0, R_SAE_RC),
    ("vcvttpd2uqq_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2dq_mask_sae", 0, R_SAE_RC),
    ("vcvttph2dq_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2qq_mask_sae", 0, R_SAE_RC),
    ("vcvttph2qq_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2udq_mask_sae", 0, R_SAE_RC),
    ("vcvttph2udq_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2uqq_mask_sae", 0, R_SAE_RC),
    ("vcvttph2uqq_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2uw_mask_sae", 0, R_SAE_RC),
    ("vcvttph2uw_maskz_sae", 0, R_SAE_RC),
    ("vcvttph2w_mask_sae", 0, R_SAE_RC),
    ("vcvttph2w_maskz_sae", 0, R_SAE_RC),
    ("vcvttps2dq_mask_sae", 0, R_SAE_RC),
    ("vcvttps2dq_maskz_sae", 0, R_SAE_RC),
    ("vcvttps2qq_mask_sae", 0, R_SAE_RC),
    ("vcvttps2qq_maskz_sae", 0, R_SAE_RC),
    ("vcvttps2udq_mask_sae", 0, R_SAE_RC),
    ("vcvttps2udq_maskz_sae", 0, R_SAE_RC),
    ("vcvttps2uqq_mask_sae", 0, R_SAE_RC),
    ("vcvttps2uqq_maskz_sae", 0, R_SAE_RC),
    ("vfixupimmpd_mask_sae", 0, R_SAE_RC),
    ("vfixupimmpd_maskz_sae", 0, R_SAE_RC),
    ("vfixupimmps_mask_sae", 0, R_SAE_RC),
    ("vfixupimmps_maskz_sae", 0, R_SAE_RC),
    ("vfixupimmsd_mask_sae", 0, R_SAE_RC),
    ("vfixupimmsd_maskz_sae", 0, R_SAE_RC),
    ("vfixupimmss_mask_sae", 0, R_SAE_RC),
    ("vfixupimmss_maskz_sae", 0, R_SAE_RC),
    ("vgetexppd_mask_sae", 0, R_SAE_RC),
    ("vgetexppd_maskz_sae", 0, R_SAE_RC),
    ("vgetexpph_mask_sae", 0, R_SAE_RC),
    ("vgetexpph_maskz_sae", 0, R_SAE_RC),
    ("vgetexpps_mask_sae", 0, R_SAE_RC),
    ("vgetexpps_maskz_sae", 0, R_SAE_RC),
    ("vgetexpsd_mask_sae", 0, R_SAE_RC),
    ("vgetexpsd_maskz_sae", 0, R_SAE_RC),
    ("vgetexpsh_mask_sae", 0, R_SAE_RC),
    ("vgetexpsh_maskz_sae", 0, R_SAE_RC),
    ("vgetexpss_mask_sae", 0, R_SAE_RC),
    ("vgetexpss_maskz_sae", 0, R_SAE_RC),
    ("vgetmantpd_mask_sae", 0, R_SAE_RC),
    ("vgetmantpd_maskz_sae", 0, R_SAE_RC),
    ("vgetmantph_mask_sae", 0, R_SAE_RC),
    ("vgetmantph_maskz_sae", 0, R_SAE_RC),
    ("vgetmantps_mask_sae", 0, R_SAE_RC),
    ("vgetmantps_maskz_sae", 0, R_SAE_RC),
    ("vgetmantsd_mask_sae", 0, R_SAE_RC),
    ("vgetmantsd_maskz_sae", 0, R_SAE_RC),
    ("vgetmantsh_mask_sae", 0, R_SAE_RC),
    ("vgetmantsh_maskz_sae", 0, R_SAE_RC),
    ("vgetmantss_mask_sae", 0, R_SAE_RC),
    ("vgetmantss_maskz_sae", 0, R_SAE_RC),
    ("vmaxpd_mask_sae", 0, R_SAE_RC),
    ("vmaxpd_maskz_sae", 0, R_SAE_RC),
    ("vmaxph_mask_sae", 0, R_SAE_RC),
    ("vmaxph_maskz_sae", 0, R_SAE_RC),
    ("vmaxps_mask_sae", 0, R_SAE_RC),
    ("vmaxps_maskz_sae", 0, R_SAE_RC),
    ("vmaxsd_mask_sae", 0, R_SAE_RC),
    ("vmaxsd_maskz_sae", 0, R_SAE_RC),
    ("vmaxsh_mask_sae", 0, R_SAE_RC),
    ("vmaxsh_maskz_sae", 0, R_SAE_RC),
    ("vmaxss_mask_sae", 0, R_SAE_RC),
    ("vmaxss_maskz_sae", 0, R_SAE_RC),
    ("vminpd_mask_sae", 0, R_SAE_RC),
    ("vminpd_maskz_sae", 0, R_SAE_RC),
    ("vminph_mask_sae", 0, R_SAE_RC),
    ("vminph_maskz_sae", 0, R_SAE_RC),
    ("vminps_mask_sae", 0, R_SAE_RC),
    ("vminps_maskz_sae", 0, R_SAE_RC),
    ("vminsd_mask_sae", 0, R_SAE_RC),
    ("vminsd_maskz_sae", 0, R_SAE_RC),
    ("vminsh_mask_sae", 0, R_SAE_RC),
    ("vminsh_maskz_sae", 0, R_SAE_RC),
    ("vminss_mask_sae", 0, R_SAE_RC),
    ("vminss_maskz_sae", 0, R_SAE_RC),
    ("vrangepd_mask_sae", 0, R_SAE_RC),
    ("vrangepd_maskz_sae", 0, R_SAE_RC),
    ("vrangeps_mask_sae", 0, R_SAE_RC),
    ("vrangeps_maskz_sae", 0, R_SAE_RC),
    ("vrangesd_mask_sae", 0, R_SAE_RC),
    ("vrangesd_maskz_sae", 0, R_SAE_RC),
    ("vrangess_mask_sae", 0, R_SAE_RC),
    ("vrangess_maskz_sae", 0, R_SAE_RC),
    ("vreduceph_mask_sae", 0, R_SAE_RC),
    ("vreduceph_maskz_sae", 0, R_SAE_RC),
    ("vreducesh_mask_sae", 0, R_SAE_RC),
    ("vreducesh_maskz_sae", 0, R_SAE_RC),
    ("vrndscalepd_mask_sae", 0, R_SAE_RC),
    ("vrndscalepd_maskz_sae", 0, R_SAE_RC),
    ("vrndscaleph_mask_sae", 0, R_SAE_RC),
    ("vrndscaleph_maskz_sae", 0, R_SAE_RC),
    ("vrndscaleps_mask_sae", 0, R_SAE_RC),
    ("vrndscaleps_maskz_sae", 0, R_SAE_RC),
    ("vrndscalesd_mask_sae", 0, R_SAE_RC),
    ("vrndscalesd_maskz_sae", 0, R_SAE_RC),
    ("vrndscalesh_mask_sae", 0, R_SAE_RC),
    ("vrndscalesh_maskz_sae", 0, R_SAE_RC),
    ("vrndscaless_mask_sae", 0, R_SAE_RC),
    ("vrndscaless_maskz_sae", 0, R_SAE_RC),
];

fn check_expected_failures(stats: &ReplayStats, check_counts: bool) {
    let expected: BTreeMap<&str, (u32, &str)> = EXPECTED_FAILURES
        .iter()
        .map(|(m, n, r)| (*m, (*n, *r)))
        .collect();
    let mut problems = Vec::new();
    for (mnem, n) in &stats.failures_by_mnem {
        match expected.get(mnem.as_str()) {
            Some((want, _)) if !check_counts || *want == *n as u32 => {}
            Some((want, reason)) => {
                problems.push(format!("{mnem}: {n} failures, expected {want} ({reason})"))
            }
            None => problems.push(format!("{mnem}: {n} unexpected failures")),
        }
    }
    for (mnem, (want, _)) in &expected {
        if check_counts && *want != 0 && !stats.failures_by_mnem.contains_key(*mnem) {
            problems.push(format!(
                "{mnem}: expected failures but all records passed — update EXPECTED_FAILURES"
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "corpus failure set differs from EXPECTED_FAILURES:\n  {}",
        problems.join("\n  ")
    );
}

#[test]
fn corpus_sample() {
    let stats = run_corpus(SAMPLE_CORPUS, None);
    check_expected_failures(&stats, true);
}

#[test]
fn corpus_full() {
    if std::env::var("ASMKIT_X86_CORPUS_FULL").is_err() {
        eprintln!("corpus_full: skipped (set ASMKIT_X86_CORPUS_FULL=1 to enable)");
        return;
    }
    let shard = std::env::var("ASMKIT_X86_CORPUS_SHARD").ok().map(|value| {
        let (index, count) = value
            .split_once('/')
            .unwrap_or_else(|| panic!("ASMKIT_X86_CORPUS_SHARD must be index/count"));
        let index: usize = index.parse().expect("invalid corpus shard index");
        let count: usize = count.parse().expect("invalid corpus shard count");
        assert!(count > 0 && index < count, "invalid corpus shard {value}");
        (index, count)
    });
    let stats = run_corpus(FULL_CORPUS, shard);
    check_expected_failures(&stats, false);
}

// ============================================================================
// Golden smoke: replay meta/golden/smoke.cpp through the new encoder.
// ============================================================================

fn assemble(f: impl FnOnce(&mut Assembler)) -> Vec<u8> {
    let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
    {
        let mut asm = Assembler::new(&mut buf);
        f(&mut asm);
    }
    assert_eq!(buf.error(), None, "golden smoke emit error");
    buf.data().to_vec()
}

fn gp(rt: RegType, id: u32) -> Operand {
    *Reg::from_type_and_id(rt, id).as_operand()
}

fn golden_blocks() -> Vec<u8> {
    let mut out = assemble(|a| {
        a.emit_n(
            InstId::Add as u32,
            &[&gp(RegType::Gp64, 0), &gp(RegType::Gp64, 1)],
        );
        a.emit_n(
            InstId::Mov as u32,
            &[&gp(RegType::Gp32, 0), imm(42i64).as_operand()],
        );
        let x = |i: u32| gp(RegType::Vec128, i);
        a.emit_n(InstId::Vaddps as u32, &[&x(0), &x(1), &x(2)]);
        a.emit_n(InstId::Ret as u32, &[]);
    });
    out.extend(assemble(|a| {
        let z = |i: u32| gp(RegType::Vec512, i);
        a.emit_n(InstId::Vaddps as u32, &[&z(0), &z(1), &z(2)]);
    }));
    out
}

#[test]
fn golden_smoke() {
    let ours = golden_blocks();
    // Expected bytes as produced by the asmjit oracle (target/x86_golden_smoke;
    // note the vvvv field comments in meta/golden/smoke.cpp are stale):
    //   add rax, rcx                  48 01 c8
    //   mov eax, 42                   b8 2a 00 00 00
    //   vaddps xmm0, xmm1, xmm2       c5 f0 58 c2
    //   ret                           c3
    //   vaddps zmm0, zmm1, zmm2       62 f1 74 48 58 c2
    let expected: &[u8] = &[
        0x48, 0x01, 0xc8, 0xb8, 0x2a, 0x00, 0x00, 0x00, 0xc5, 0xf0, 0x58, 0xc2, 0xc3, 0x62, 0xf1,
        0x74, 0x48, 0x58, 0xc2,
    ];
    assert_eq!(hex(&ours), hex(expected), "golden smoke mismatch");

    // Cross-check against the asmjit-built oracle binary when available.
    if let Ok(out) = std::process::Command::new(GOLDEN_SMOKE).output() {
        assert!(out.status.success(), "golden oracle failed to run");
        let stdout = String::from_utf8(out.stdout).unwrap();
        let oracle: String = stdout
            .lines()
            .map(|l| l.split_whitespace().nth(1).unwrap_or("").to_string())
            .collect();
        assert_eq!(hex(&ours), oracle, "mismatch vs asmjit oracle output");
    } else {
        eprintln!("golden_smoke: oracle binary not found, used documented bytes");
    }
}

// ============================================================================
// Decode roundtrip: sweep InstIds x signature combos from the instdb tables,
// assemble with the new encoder, decode with iced-x86, and verify the decoded
// mnemonic + operands match what was assembled. Capstone is used as a sampled
// secondary cross-check.
// ============================================================================

mod roundtrip {
    use super::hex;
    use asmkit::CodeBuffer;
    use asmkit::x86::assembler::Assembler;
    use asmkit::x86::coverage::{
        Avx512Flags, INST_COMMON_INFO_TABLE, INST_INFO_TABLE, INST_NAME_INDEX_TABLE,
        INST_NAME_STRING_TABLE, INST_SIGNATURE_TABLE, Mode, OP_SIGNATURE_TABLE, OpFlags,
        OpSignature,
    };
    use asmkit::x86::operands::{AddrType, KReg, Mem, Reg};
    use asmkit::{Arch, Environment};
    use asmkit::{Label, Operand, OperandCast, RegType, imm};
    use capstone::arch::BuildsCapstone;
    use std::collections::{BTreeMap, BTreeSet};

    // ---- InstId -> name (asmjit instdb name encoding) ----

    fn inst_name(id: u32) -> String {
        let v = INST_NAME_INDEX_TABLE[id as usize];
        if v & 0x8000_0000 != 0 {
            // Small string: six 5-bit characters, a=1..z=26, 0=27..9=36? (asmjit
            // encodes digits as '0'-27+c).
            let mut s = String::new();
            let mut x = v;
            for _ in 0..6 {
                let c = x & 0x1F;
                if c == 0 {
                    break;
                }
                s.push(if c <= 26 {
                    (b'a' + c as u8 - 1) as char
                } else {
                    (b'0' + c as u8 - 27) as char
                });
                x >>= 5;
            }
            s
        } else {
            let pb = (v & 0xFFF) as usize;
            let ps = ((v >> 12) & 0xF) as usize;
            let sb = ((v >> 16) & 0xFFF) as usize;
            let ss = ((v >> 28) & 0x7) as usize;
            let t = INST_NAME_STRING_TABLE;
            let mut s = String::from_utf8_lossy(&t[pb..pb + ps]).into_owned();
            // Suffix base 0xFFF marks an alias-format entry (no suffix data).
            if ss != 0 {
                s.push_str(&String::from_utf8_lossy(&t[sb..sb + ss]));
            }
            s
        }
    }

    // ---- operand candidate generation ----

    #[derive(Clone, Copy, PartialEq)]
    enum Cand {
        Reg(RegType, u32),
        Mem {
            base: Option<(RegType, u32)>,
            index: Option<(RegType, u32, u32)>, // (type, id, shift)
            size: u32,
            rip: bool,
            disp: i32,
        },
        Imm(i64),
        Rel,
    }

    impl std::fmt::Debug for Cand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                Cand::Reg(rt, id) => write!(f, "reg({},{id})", rt as u32),
                Cand::Mem {
                    base,
                    index,
                    size,
                    rip,
                    disp,
                } => {
                    let b = base.map(|(rt, id)| (rt as u32, id));
                    let x = index.map(|(rt, id, sh)| (rt as u32, id, sh));
                    write!(f, "mem({b:?},{x:?},size={size},rip={rip},disp={disp})")
                }
                Cand::Imm(v) => write!(f, "imm({v})"),
                Cand::Rel => write!(f, "rel"),
            }
        }
    }

    fn reg_ids(rt: RegType, reg_mask: u8, vec_hi: u32) -> Vec<u32> {
        if reg_mask != 0 {
            let lo = reg_mask.trailing_zeros();
            let hi = 7 - reg_mask.leading_zeros();
            return if lo == hi { vec![lo] } else { vec![lo, hi] };
        }
        match rt {
            RegType::Gp8Hi => vec![0, 3],
            RegType::Gp8Lo | RegType::Gp16 | RegType::Gp32 | RegType::Gp64 => vec![0, 3],
            // Ids 16..=31 are only encodable with EVEX; legacy/VEX encodings
            // silently truncate them (asmjit behavior, verified against the
            // pinned asmjit clone).
            RegType::Vec128 | RegType::Vec256 | RegType::Vec512 => vec![1, vec_hi],
            RegType::Mask => vec![1, 7],
            RegType::X86SReg => vec![1, 6],
            RegType::X86CReg | RegType::X86DReg => vec![0, 15],
            RegType::X86St | RegType::Extra | RegType::X86Bnd | RegType::X86Tmm => vec![0, 3],
            _ => vec![0],
        }
    }

    fn mem_sizes(f: OpFlags) -> Vec<u32> {
        let mut out = Vec::new();
        if f.contains(OpFlags::MEM_UNSPECIFIED) {
            out.push(0);
        }
        for (flag, size) in [
            (OpFlags::MEM8, 1),
            (OpFlags::MEM16, 2),
            (OpFlags::MEM32, 4),
            (OpFlags::MEM48, 6),
            (OpFlags::MEM64, 8),
            (OpFlags::MEM80, 10),
            (OpFlags::MEM128, 16),
            (OpFlags::MEM256, 32),
            (OpFlags::MEM512, 64),
            (OpFlags::MEM1024, 128),
        ] {
            if f.contains(flag) {
                out.push(size);
            }
        }
        out
    }

    fn candidates(sig: &OpSignature, vec_hi: u32) -> Vec<Cand> {
        let f = OpFlags::from_bits_retain(sig.flags);
        let mut out: Vec<Cand> = Vec::new();

        for (flag, rt) in [
            (OpFlags::REG_GPB_LO, RegType::Gp8Lo),
            (OpFlags::REG_GPB_HI, RegType::Gp8Hi),
            (OpFlags::REG_GPW, RegType::Gp16),
            (OpFlags::REG_GPD, RegType::Gp32),
            (OpFlags::REG_GPQ, RegType::Gp64),
            (OpFlags::REG_XMM, RegType::Vec128),
            (OpFlags::REG_YMM, RegType::Vec256),
            (OpFlags::REG_ZMM, RegType::Vec512),
            (OpFlags::REG_MM, RegType::Extra),
            (OpFlags::REG_K_REG, RegType::Mask),
            (OpFlags::REG_S_REG, RegType::X86SReg),
            (OpFlags::REG_C_REG, RegType::X86CReg),
            (OpFlags::REG_D_REG, RegType::X86DReg),
            (OpFlags::REG_ST, RegType::X86St),
            (OpFlags::REG_BND, RegType::X86Bnd),
            (OpFlags::REG_TMM, RegType::X86Tmm),
        ] {
            if f.contains(flag) {
                out.extend(
                    reg_ids(rt, sig.reg_mask, vec_hi)
                        .into_iter()
                        .map(|id| Cand::Reg(rt, id)),
                );
            }
        }

        // VSIB (vector-index) memory, one per index kind, element-sized.
        for (flag, rt, esize) in [
            (OpFlags::VM32X, RegType::Vec128, 4u32),
            (OpFlags::VM32Y, RegType::Vec256, 4),
            (OpFlags::VM32Z, RegType::Vec512, 4),
            (OpFlags::VM64X, RegType::Vec128, 8),
            (OpFlags::VM64Y, RegType::Vec256, 8),
            (OpFlags::VM64Z, RegType::Vec512, 8),
        ] {
            if f.contains(flag) {
                // disp 0: iced-x86 1.21 rejects gather/scatter with a
                // displacement (the corpus grid likewise only has disp-0).
                out.push(Cand::Mem {
                    base: Some((RegType::Gp64, 0)),
                    index: Some((rt, 2, 0)),
                    size: esize,
                    rip: false,
                    disp: 0,
                });
            }
        }

        let base_only = f.contains(OpFlags::FLAG_MEM_BASE);
        // A nonzero reg_mask on a memory signature denotes the required
        // implicit base register (es:[zdi], ds:[zsi], ...).
        let base_id = if sig.reg_mask != 0 {
            sig.reg_mask.trailing_zeros()
        } else {
            0
        };
        for (i, size) in mem_sizes(f).into_iter().enumerate() {
            out.push(Cand::Mem {
                base: Some((RegType::Gp64, base_id)),
                index: None,
                size,
                rip: false,
                disp: 0,
            });
            if i == 0 && !base_only {
                out.push(Cand::Mem {
                    base: Some((RegType::Gp64, 1)),
                    index: Some((RegType::Gp64, 2, 1)),
                    size,
                    rip: false,
                    disp: 0x40,
                });
                out.push(Cand::Mem {
                    base: None,
                    index: None,
                    size,
                    rip: true,
                    disp: 0x10,
                });
            }
        }

        // Immediates: values for the smallest advertised width (edge values
        // exercise the imm8/imm32 form selection).
        let imms: &[i64] = if f.contains(OpFlags::IMM_I4) {
            &[0x7]
        } else if f.contains(OpFlags::IMM_U4) {
            &[0xF]
        } else if f.contains(OpFlags::IMM_I8) {
            &[0x7F, -1]
        } else if f.contains(OpFlags::IMM_U8) {
            &[0x80]
        } else if f.contains(OpFlags::IMM_I16) {
            &[0x1234, -2]
        } else if f.contains(OpFlags::IMM_U16) {
            &[0xFFFF]
        } else if f.contains(OpFlags::IMM_I32) {
            &[0x12345678, -3]
        } else if f.contains(OpFlags::IMM_U32) {
            &[0xFFFF_FFFF]
        } else if f.intersects(OpFlags::IMM_I64 | OpFlags::IMM_U64) {
            &[0x123456789ABCDEF0u64 as i64, -4]
        } else {
            &[]
        };
        out.extend(imms.iter().map(|v| Cand::Imm(*v)));

        if f.intersects(OpFlags::REL8 | OpFlags::REL32) {
            out.push(Cand::Rel);
        }
        if out.is_empty() {
            out.push(Cand::Reg(RegType::None, 0)); // placeholder, filtered later
        }
        out.truncate(4);
        out
    }

    fn build_operand(c: &Cand, label: Option<Label>) -> Operand {
        match *c {
            Cand::Reg(rt, id) => *Reg::from_type_and_id(rt, id).as_operand(),
            Cand::Mem {
                base,
                index,
                size,
                rip,
                disp,
            } => {
                let mut m = Mem::new();
                if rip {
                    m.set_base_type(RegType::PC);
                    m.set_addr_type(AddrType::Rel);
                } else if let Some((rt, id)) = base {
                    m.set_base_type(rt);
                    m.set_base_id(id);
                }
                if let Some((rt, id, shift)) = index {
                    m.set_index_type(rt);
                    m.set_index_id(id);
                    m.set_shift(shift);
                }
                if size != 0 {
                    m.set_size(size);
                }
                m.set_offset_lo32(disp);
                *m.as_operand()
            }
            Cand::Imm(v) => *imm(v).as_operand(),
            Cand::Rel => *label.expect("rel operand without label").as_operand(),
        }
    }

    /// iced-x86 register -> (RegType, id) for comparison.
    fn iced_reg(r: iced_x86::Register) -> Option<(RegType, u32)> {
        use iced_x86::Register as R;
        let v = r as u32;
        let out: (RegType, u32) = if (R::AL as u32..=R::BL as u32).contains(&v) {
            (RegType::Gp8Lo, v - R::AL as u32)
        } else if (R::AH as u32..=R::BH as u32).contains(&v) {
            (RegType::Gp8Hi, v - R::AH as u32)
        } else if (R::SPL as u32..=R::R15L as u32).contains(&v) {
            (RegType::Gp8Lo, v - R::SPL as u32 + 4)
        } else if (R::AX as u32..=R::R15W as u32).contains(&v) {
            (RegType::Gp16, v - R::AX as u32)
        } else if (R::EAX as u32..=R::R15D as u32).contains(&v) {
            (RegType::Gp32, v - R::EAX as u32)
        } else if (R::RAX as u32..=R::R15 as u32).contains(&v) {
            (RegType::Gp64, v - R::RAX as u32)
        } else if (R::XMM0 as u32..=R::XMM31 as u32).contains(&v) {
            (RegType::Vec128, v - R::XMM0 as u32)
        } else if (R::YMM0 as u32..=R::YMM31 as u32).contains(&v) {
            (RegType::Vec256, v - R::YMM0 as u32)
        } else if (R::ZMM0 as u32..=R::ZMM31 as u32).contains(&v) {
            (RegType::Vec512, v - R::ZMM0 as u32)
        } else if (R::K0 as u32..=R::K7 as u32).contains(&v) {
            (RegType::Mask, v - R::K0 as u32)
        } else if (R::MM0 as u32..=R::MM7 as u32).contains(&v) {
            (RegType::Extra, v - R::MM0 as u32)
        } else if (R::ES as u32..=R::GS as u32).contains(&v) {
            (RegType::X86SReg, v - R::ES as u32 + 1)
        } else if (R::CR0 as u32..=R::CR15 as u32).contains(&v) {
            (RegType::X86CReg, v - R::CR0 as u32)
        } else if (R::DR0 as u32..=R::DR15 as u32).contains(&v) {
            (RegType::X86DReg, v - R::DR0 as u32)
        } else if (R::ST0 as u32..=R::ST7 as u32).contains(&v) {
            (RegType::X86St, v - R::ST0 as u32)
        } else if (R::BND0 as u32..=R::BND3 as u32).contains(&v) {
            (RegType::X86Bnd, v - R::BND0 as u32)
        } else if (R::TMM0 as u32..=R::TMM7 as u32).contains(&v) {
            (RegType::X86Tmm, v - R::TMM0 as u32)
        } else if v == R::RIP as u32 {
            (RegType::PC, 0)
        } else {
            return None;
        };
        Some(out)
    }

    /// Mnemonic aliases between instdb names and iced-x86 names.
    fn mnem_alias(name: &str, decoded: &str) -> bool {
        if name == decoded {
            return true;
        }
        // Size-suffixed string operations: instdb `movs` vs iced `movsb` etc.
        let stringy = ["movs", "cmps", "stos", "lods", "scas", "ins", "outs"];
        if stringy.contains(&name) && decoded.starts_with(name) {
            return true;
        }
        matches!(
            (name, decoded),
            ("sal", "shl") | ("jecxz", "jrcxz") | ("xlatb", "xlat")
        )
    }

    /// Instructions iced-x86 1.21 cannot decode (or that the pinned asmjit
    /// itself emits undecodably, verified against meta/asmjit):
    ///  - tdp* AMX dot-products (missing from iced 1.21; asmjit emits VEX,
    ///    asmkit matches it byte-for-byte);
    ///  - the vfmaddc/vfcmulc complex-FP16 families: the instdb opcode tables
    ///    carry pp=F3 where the architecture requires 66/F2 (inherited asmjit
    ///    table bug, present in the baseline encoder too — see FORMAT.md).
    fn known_undecodable(name: &str) -> bool {
        name.starts_with("tdp")
            || name.starts_with("tcmm")
            || name.starts_with("tile")
            || matches!(name, "pfrcpv" | "pfrsqrtv" | "pfcpv")
            || matches!(
                name,
                "vfmulcph"
                    | "vfmulcsh"
                    | "vfmaddcph"
                    | "vfmaddcsh"
                    | "vfcmulcph"
                    | "vfcmulcsh"
                    | "vfcmaddcph"
                    | "vfcmaddcsh"
            )
    }

    /// Compares one assembled combo against its decoded form.
    /// Outcome: 1 = verified, 0 = problem (recorded), -1 = skipped (gap).
    fn check_case(
        name: &str,
        cands: &[Cand],
        bytes: &[u8],
        expect_mask: bool,
        problems: &mut Vec<String>,
        mnem_pairs: &mut BTreeSet<(String, String)>,
        gaps: &mut BTreeMap<String, u64>,
    ) -> i32 {
        // The encoder emits FWAIT before FPU wait-forms (asmjit behavior);
        // verify the FPU instruction itself.
        let bytes = if bytes.len() > 1 && bytes[0] == 0x9B {
            &bytes[1..]
        } else {
            bytes
        };
        let mut dec = iced_x86::Decoder::new(64, bytes, 0);
        let instr = dec.decode();
        if instr.is_invalid() || dec.position() != bytes.len() {
            if known_undecodable(name) {
                *gaps
                    .entry(format!("{name}: iced cannot decode"))
                    .or_default() += 1;
                return -1;
            }
            if expect_mask {
                // asmjit's tables claim {kz} on forms the architecture does
                // not mask (e.g. vpinsrb, vmovw r32): both encoders emit the
                // EVEX bytes, no decoder accepts them.
                *gaps
                    .entry(format!("{name}: masked form undecodable"))
                    .or_default() += 1;
                return -1;
            }
            if cands.iter().any(|c| {
                matches!(
                    c,
                    Cand::Mem {
                        index: Some((RegType::Vec128 | RegType::Vec256 | RegType::Vec512, ..)),
                        ..
                    }
                )
            }) {
                // EVEX gather/scatter requires a writemask; the unmasked
                // encoding is invalid (asmjit emits it anyway, GIGO).
                *gaps
                    .entry(format!("{name}: unmasked gather/scatter undecodable"))
                    .or_default() += 1;
                return -1;
            }
            problems.push(format!("{name} {cands:?}: undecodable {}", hex(bytes)));
            return 0;
        }
        // iced 1.21 decodes MPX (bnd*) as reserved-nop.
        if name.starts_with("bnd")
            && format!("{:?}", instr.mnemonic())
                .to_lowercase()
                .starts_with("reservednop")
        {
            *gaps.entry(format!("{name}: iced reservednop")).or_default() += 1;
            return -1;
        }
        // xchg with two equal accumulator registers assembles to 90+r, which
        // iced decodes as a sized nop.
        if name == "xchg"
            && instr.mnemonic() == iced_x86::Mnemonic::Nop
            && cands.len() == 2
            && cands[0] == cands[1]
            && matches!(
                cands[0],
                Cand::Reg(RegType::Gp16 | RegType::Gp32 | RegType::Gp64, 0)
            )
        {
            return 1;
        }
        let decoded_name = format!("{:?}", instr.mnemonic()).to_lowercase();
        if !mnem_alias(name, &decoded_name) {
            // Check operands anyway; collect the naming pair.
            if operands_match(&instr, cands, expect_mask) {
                mnem_pairs.insert((name.to_string(), decoded_name));
                return 1;
            }
            problems.push(format!(
                "{name} {cands:?}: decoded as {decoded_name} ({})",
                hex(bytes)
            ));
            return 0;
        }
        if !operands_match(&instr, cands, expect_mask) {
            problems.push(format!(
                "{name} {cands:?}: operand mismatch ({}, decoded {})",
                hex(bytes),
                instr
            ));
            return 0;
        }
        1
    }

    fn operands_match(instr: &iced_x86::Instruction, cands: &[Cand], expect_mask: bool) -> bool {
        if instr.has_op_mask() != expect_mask {
            return false;
        }
        if expect_mask && instr.op_mask() != iced_x86::Register::K1 {
            return false;
        }
        // xchg is commutative: operand order varies between the short form,
        // the ModRM form, and iced's operand model.
        if instr.mnemonic() == iced_x86::Mnemonic::Xchg && cands.len() == 2 {
            return (one_op_matches(instr, 0, &cands[0]) && one_op_matches(instr, 1, &cands[1]))
                || (one_op_matches(instr, 0, &cands[1]) && one_op_matches(instr, 1, &cands[0]));
        }
        // Zero explicit operands: nothing to check beyond the mnemonic (iced
        // models implicit operands for e.g. xlatb).
        if cands.is_empty() {
            return true;
        }
        // iced exposes the implicit es:[rdi] store operand of maskmovq/
        // (v)maskmovdqu; the instdb signature (and our combo) does not.
        let decoded_skip = if instr.op_count() as usize == cands.len() + 1
            && matches!(
                instr.op_kind(0),
                iced_x86::OpKind::MemorySegRDI | iced_x86::OpKind::MemoryESRDI
            ) {
            1
        } else {
            0
        };
        // rdmsr: the instdb signature's operands do not correspond to the
        // architectural (implicit) operands; only check the mnemonic.
        if matches!(instr.mnemonic(), iced_x86::Mnemonic::Rdmsr) {
            return true;
        }
        // FPU instructions: iced models the implicit st(0) operand
        // explicitly (faddp st(i),st / fcomip st,st(i) / fxch ...).
        if cands.len() == 1
            && matches!(cands[0], Cand::Reg(RegType::X86St, _))
            && instr.op_count() == 2
            && instr.op_kind(0) == iced_x86::OpKind::Register
            && instr.op_kind(1) == iced_x86::OpKind::Register
        {
            return one_op_matches(instr, 0, &cands[0]) || one_op_matches(instr, 1, &cands[0]);
        }
        // The nop signature's second operand is the ignored /0 field; iced
        // models only the r/m operand.
        let cands = if instr.mnemonic() == iced_x86::Mnemonic::Nop
            && cands.len() == 2
            && instr.op_count() == 1
        {
            &cands[..1]
        } else {
            cands
        };
        if instr.op_count() as usize != cands.len() + decoded_skip {
            return false;
        }
        for (i, c) in cands.iter().enumerate() {
            if !one_op_matches(instr, (i + decoded_skip) as u32, c) {
                return false;
            }
        }
        true
    }

    fn one_op_matches(instr: &iced_x86::Instruction, i: u32, c: &Cand) -> bool {
        {
            match (*c, instr.op_kind(i)) {
                (Cand::Reg(rt, id), iced_x86::OpKind::Register) => {
                    let got = iced_reg(instr.op_register(i));
                    if got != Some((rt, id)) {
                        // System instructions with an r/m16 operand decode as
                        // EAX in iced, and lar/lsl's r/m16 source decodes at
                        // the destination width; the encoding only carries
                        // the register id.
                        let width_lax = matches!(
                            instr.mnemonic(),
                            iced_x86::Mnemonic::Lar
                                | iced_x86::Mnemonic::Lsl
                                | iced_x86::Mnemonic::Lldt
                                | iced_x86::Mnemonic::Lmsw
                                | iced_x86::Mnemonic::Ltr
                                | iced_x86::Mnemonic::Verr
                                | iced_x86::Mnemonic::Verw
                                | iced_x86::Mnemonic::Smsw
                                | iced_x86::Mnemonic::Str
                                | iced_x86::Mnemonic::Sldt
                        ) && got.is_some_and(|(_, got_id)| got_id == id);
                        if !width_lax {
                            return false;
                        }
                    }
                }
                // Implicit string-op memory operands.
                (
                    Cand::Mem {
                        base: Some(base),
                        index: None,
                        rip: false,
                        disp: 0,
                        ..
                    },
                    kind @ (iced_x86::OpKind::MemoryESRDI | iced_x86::OpKind::MemorySegRSI),
                ) => {
                    let want = if kind == iced_x86::OpKind::MemoryESRDI {
                        (RegType::Gp64, 7)
                    } else {
                        (RegType::Gp64, 6)
                    };
                    if base != want {
                        return false;
                    }
                }
                // Some instructions address memory through the modrm.reg
                // field (movdir64b, enqcmd, umonitor): iced models that
                // operand as a register holding the address.
                (
                    Cand::Mem {
                        base: Some(base),
                        index: None,
                        rip: false,
                        disp: 0,
                        ..
                    },
                    iced_x86::OpKind::Register,
                ) => {
                    if iced_reg(instr.op_register(i)) != Some(base) {
                        return false;
                    }
                }
                (
                    Cand::Mem {
                        base,
                        index,
                        rip,
                        disp,
                        ..
                    },
                    iced_x86::OpKind::Memory,
                ) => {
                    let want_base = if rip { Some((RegType::PC, 0)) } else { base };
                    match want_base {
                        Some(w) if iced_reg(instr.memory_base()) != Some(w) => return false,
                        None if instr.memory_base() != iced_x86::Register::None => return false,
                        _ => {}
                    }
                    match index {
                        Some((rt, id, shift)) => {
                            if iced_reg(instr.memory_index()) != Some((rt, id))
                                || instr.memory_index_scale() != 1 << shift
                            {
                                return false;
                            }
                        }
                        None => {
                            if instr.memory_index() != iced_x86::Register::None {
                                return false;
                            }
                        }
                    }
                    if instr.memory_displacement64()
                        != if rip {
                            // iced resolves RIP-relative operands to the
                            // absolute target address (decoder ip = 0).
                            instr.len() as i64 + disp as i64
                        } else {
                            disp as i64
                        } as u64
                    {
                        return false;
                    }
                }
                (Cand::Imm(v), kind) => {
                    // iced models raw branch displacements (xbegin) as branch
                    // operands; the encoded immediate is the displacement.
                    if matches!(
                        kind,
                        iced_x86::OpKind::NearBranch16
                            | iced_x86::OpKind::NearBranch32
                            | iced_x86::OpKind::NearBranch64
                    ) {
                        return instr.near_branch_target()
                            == (instr.len() as i64).wrapping_add(v) as u64;
                    }
                    if !matches!(
                        kind,
                        iced_x86::OpKind::Immediate8
                            | iced_x86::OpKind::Immediate8_2nd
                            | iced_x86::OpKind::Immediate8to16
                            | iced_x86::OpKind::Immediate8to32
                            | iced_x86::OpKind::Immediate8to64
                            | iced_x86::OpKind::Immediate16
                            | iced_x86::OpKind::Immediate32
                            | iced_x86::OpKind::Immediate32to64
                            | iced_x86::OpKind::Immediate64
                    ) {
                        return false;
                    }
                    let got = instr.immediate(i);
                    if got != v as u64
                        && got != v as u8 as u64
                        && got != v as u16 as u64
                        && got != v as u32 as u64
                        && got != v as i8 as i64 as u64
                        && got != v as i16 as i64 as u64
                        && got != v as i32 as i64 as u64
                    {
                        return false;
                    }
                }
                (Cand::Rel, kind) => {
                    if !matches!(
                        kind,
                        iced_x86::OpKind::NearBranch16
                            | iced_x86::OpKind::NearBranch32
                            | iced_x86::OpKind::NearBranch64
                    ) {
                        return false;
                    }
                    // Label bound at offset 0, decoder ip 0.
                    if instr.near_branch_target() != 0 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Cartesian product over per-slot candidates, deterministically sampled
    /// down to `cap` combos.
    fn sample_combos(slots: &[Vec<Cand>], cap: usize) -> Vec<Vec<Cand>> {
        let total: usize = slots.iter().map(|s| s.len()).product();
        let stride = total.div_ceil(cap).max(1);
        let mut out = Vec::new();
        let mut idx = 0usize;
        while idx < total && out.len() < cap {
            let mut combo = Vec::with_capacity(slots.len());
            let mut x = idx;
            for slot in slots {
                combo.push(slot[x % slot.len()]);
                x /= slot.len();
            }
            out.push(combo);
            idx += stride;
        }
        out
    }

    struct SweepStats {
        insts: u64,
        signatures: u64,
        cases: u64,
        emit_ok: u64,
        emit_reject: u64,
        decode_fail: u64,
        rejects_by_inst: BTreeMap<String, u64>,
        gaps: BTreeMap<String, u64>,
        problems: Vec<String>,
        mnem_pairs: BTreeSet<(String, String)>,
        capstone_checked: u64,
        capstone_disagree: u64,
    }

    pub fn run() {
        let mut stats = SweepStats {
            insts: 0,
            signatures: 0,
            cases: 0,
            emit_ok: 0,
            emit_reject: 0,
            decode_fail: 0,
            rejects_by_inst: BTreeMap::new(),
            gaps: BTreeMap::new(),
            problems: Vec::new(),
            mnem_pairs: BTreeSet::new(),
            capstone_checked: 0,
            capstone_disagree: 0,
        };
        let capstone = capstone::Capstone::new()
            .x86()
            .mode(capstone::arch::x86::ArchMode::Mode64)
            .build();

        let n_insts = INST_INFO_TABLE.len();
        for id in 1..n_insts as u32 {
            let name = inst_name(id);
            if name.is_empty() {
                continue;
            }
            stats.insts += 1;
            let info = &INST_INFO_TABLE[id as usize];
            let common = &INST_COMMON_INFO_TABLE[info.common_info_index as usize];
            let sigs = &INST_SIGNATURE_TABLE[common.signature_index as usize
                ..common.signature_index as usize + common.signature_count as usize];

            let supports_k = common.has_avx512_flag(Avx512Flags::K);
            // True if some signature writes a mask register (vcmpp/vpcmpeq):
            // for those, adding {k} to a vector-destination signature would
            // silently reinterpret the destination as a k register.
            let has_k_dst = sigs.iter().any(|sig| {
                (0..sig.op_count as usize)
                    .map(|r| &OP_SIGNATURE_TABLE[sig.op_signature_indexes[r] as usize])
                    .find(|osig| osig.flags & OpFlags::FLAG_IMPLICIT.bits() == 0)
                    .is_some_and(|osig| osig.flags & OpFlags::REG_K_REG.bits() != 0)
            });

            let mut per_inst = 0usize;
            'sigs: for sig in sigs {
                if sig.mode & (Mode::X64 as u8) == 0 {
                    continue;
                }
                stats.signatures += 1;
                let mut slots: Vec<Vec<Cand>> = Vec::new();
                // Vec ids above 15 force EVEX. For compare-style instructions
                // whose destination can be a vector or a mask register
                // ({xmm|k} destination signatures), an EVEX upgrade silently
                // reinterprets a vector destination as a mask register, so
                // keep ids VEX-encodable there.
                let first_op = (0..sig.op_count as usize)
                    .map(|r| &OP_SIGNATURE_TABLE[sig.op_signature_indexes[r] as usize])
                    .find(|osig| osig.flags & OpFlags::FLAG_IMPLICIT.bits() == 0);
                let first_op_k_or_vec = first_op.is_some_and(|osig| {
                    let f = osig.flags;
                    f & OpFlags::REG_K_REG.bits() != 0
                        && f & (OpFlags::REG_XMM.bits()
                            | OpFlags::REG_YMM.bits()
                            | OpFlags::REG_ZMM.bits())
                            != 0
                });
                let vec_hi = if common.avx512_flags != 0 && !first_op_k_or_vec {
                    31
                } else {
                    15
                };
                let mut skip_sig = false;
                for r in 0..sig.op_count as usize {
                    let osig = &OP_SIGNATURE_TABLE[sig.op_signature_indexes[r] as usize];
                    if osig.flags & OpFlags::FLAG_IMPLICIT.bits() != 0 {
                        continue;
                    }
                    let cands = candidates(osig, vec_hi);
                    // The placeholder means "no representable operand".
                    if cands.len() == 1 && cands[0] == Cand::Reg(RegType::None, 0) {
                        skip_sig = true;
                        break;
                    }
                    slots.push(cands);
                }
                if skip_sig {
                    continue;
                }
                for combo in sample_combos(&slots, 16) {
                    if per_inst >= 48 {
                        break 'sigs;
                    }
                    per_inst += 1;
                    // Masking a vector-destination combo of a k-dst
                    // instruction would silently reinterpret the destination
                    // as a mask register.
                    let combo_op0_is_k = matches!(combo.first(), Some(Cand::Reg(RegType::Mask, _)));
                    let allow_mask = supports_k && (combo_op0_is_k || !has_k_dst);
                    for with_mask in [false, true] {
                        if with_mask && !allow_mask {
                            continue;
                        }
                        stats.cases += 1;
                        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
                        let label = if combo.iter().any(|c| matches!(c, Cand::Rel)) {
                            let l = buf.get_label();
                            buf.bind_label(l);
                            Some(l)
                        } else {
                            None
                        };
                        let ops: Vec<Operand> =
                            combo.iter().map(|c| build_operand(c, label)).collect();
                        let bytes = {
                            let mut a = Assembler::new(&mut buf);
                            if with_mask {
                                a.k(KReg::from_id(1));
                            }
                            let refs: Vec<&Operand> = ops.iter().collect();
                            a.emit_n(id, &refs);
                            if a.error().is_some() {
                                None
                            } else {
                                Some(a.data().to_vec())
                            }
                        };
                        let Some(bytes) = bytes else {
                            stats.emit_reject += 1;
                            *stats.rejects_by_inst.entry(name.clone()).or_default() += 1;
                            continue;
                        };
                        stats.emit_ok += 1;
                        let ok_case = check_case(
                            &name,
                            &combo,
                            &bytes,
                            with_mask,
                            &mut stats.problems,
                            &mut stats.mnem_pairs,
                            &mut stats.gaps,
                        );
                        if ok_case == 0 {
                            stats.decode_fail += 1;
                            continue;
                        } else if ok_case < 0 {
                            continue;
                        }
                        // Capstone secondary cross-check (sampled).
                        if stats.cases % 37 == 0 {
                            if let Ok(cs) = &capstone {
                                if let Ok(insns) = cs.disasm_all(&bytes, 0) {
                                    if let Some(one) = insns.first() {
                                        stats.capstone_checked += 1;
                                        let iced_name = format!(
                                            "{:?}",
                                            iced_x86::Decoder::new(64, &bytes, 0)
                                                .decode()
                                                .mnemonic()
                                        )
                                        .to_lowercase();
                                        if one.mnemonic().unwrap_or("") != iced_name {
                                            stats.capstone_disagree += 1;
                                            if stats.capstone_disagree <= 5 {
                                                eprintln!(
                                                    "  capstone-disagree: {name} {}: capstone={} iced={iced_name}",
                                                    hex(&bytes),
                                                    one.mnemonic().unwrap_or("")
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        eprintln!(
            "decode_roundtrip: {} instructions, {} signatures, {} cases: {} emitted, {} emit-rejected, {} decode mismatches",
            stats.insts,
            stats.signatures,
            stats.cases,
            stats.emit_ok,
            stats.emit_reject,
            stats.decode_fail
        );
        eprintln!(
            "  capstone cross-check: {} sampled, {} mnemonic disagreements vs iced",
            stats.capstone_checked, stats.capstone_disagree
        );
        let mut worst: Vec<_> = stats.rejects_by_inst.iter().collect();
        worst.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        for (name, n) in worst.iter().take(30) {
            eprintln!("  emit-reject: {n:5} {name}");
        }
        for (a, b) in &stats.mnem_pairs {
            eprintln!("  mnemonic-alias: {a} == {b}");
        }
        for (what, n) in &stats.gaps {
            eprintln!("  gap: {n:5} {what}");
        }
        for p in stats.problems.iter().take(2000) {
            eprintln!("  {p}");
        }
        assert!(
            stats.problems.is_empty(),
            "{} roundtrip problems",
            stats.problems.len()
        );
    }
}

#[test]
fn decode_roundtrip() {
    roundtrip::run();
}

// ============================================================================
// Layer 4: 32-bit mode differential against iced-x86's 32-bit CodeAssembler.
// ============================================================================

mod x86_32 {
    //! asmkit's 32-bit encoder (`Environment::new(Arch::X86)`) against
    //! iced-x86's 32-bit `CodeAssembler`. Comparisons are byte-identical
    //! except for known encoding-choice divergences, which are checked by
    //! decoding both outputs with iced's 32-bit decoder:
    //!
    //! - legacy prefix order on 16-bit 67h forms (asmjit order: 67 66,
    //!   iced order: 66 67);
    //! - jmp/jcc to a bound label (iced auto-shortens to rel8, asmkit keeps
    //!   rel32 unless SHORT_FORM is requested);
    //! - xchg accumulator short forms (asmjit: 90+r, iced: ModRM).

    use super::instr_equivalent;
    use asmkit::Arch;
    use asmkit::CodeBuffer;
    use asmkit::Environment;
    use asmkit::x86::InstId;
    use asmkit::x86::assembler::Assembler;
    use asmkit::x86::operands::regs::*;
    use asmkit::x86::operands::{
        byte_ptr, byte_ptr_index, dword_ptr, dword_ptr_index, dword_ptr_label, dword_ptr_u64,
        fword_ptr, word_ptr, word_ptr_index, word_ptr_u64,
    };
    use asmkit::{Label, OperandCast, imm};
    use iced_x86::code_asm;

    /// Assembles a block with asmkit's 32-bit encoder.
    fn asm32(f: impl FnOnce(&mut Assembler)) -> Vec<u8> {
        let environment = Environment::new(Arch::X86);
        let mut buf = CodeBuffer::new(environment);
        {
            let mut a = Assembler::new(&mut buf);
            f(&mut a);
        }
        assert!(buf.error().is_none(), "{:?}", buf.error());
        buf.finish().unwrap().data().to_vec()
    }

    /// Assembles with asmkit's 32-bit encoder, expecting rejection.
    fn asm32_is_err(f: impl FnOnce(&mut Assembler)) -> bool {
        let environment = Environment::new(Arch::X86);
        let mut buf = CodeBuffer::new(environment);
        {
            let mut a = Assembler::new(&mut buf);
            f(&mut a);
        }
        buf.error().is_some()
    }

    /// Assembles a block with iced-x86's 32-bit CodeAssembler.
    fn iced32(f: impl FnOnce(&mut code_asm::CodeAssembler)) -> Vec<u8> {
        let mut a = code_asm::CodeAssembler::new(32).unwrap();
        f(&mut a);
        a.assemble(0).unwrap()
    }

    /// Assembles with iced-x86's 32-bit CodeAssembler, expecting rejection
    /// (either at method call or at final assembly).
    fn iced32_is_err(
        f: impl FnOnce(&mut code_asm::CodeAssembler) -> Result<(), iced_x86::IcedError>,
    ) -> bool {
        let mut a = code_asm::CodeAssembler::new(32).unwrap();
        f(&mut a).is_err() || a.assemble(0).is_err()
    }

    /// Byte-identical comparison of one assembled block.
    #[track_caller]
    fn same_bytes(what: &str, ours: Vec<u8>, iced: Vec<u8>) {
        assert_eq!(ours, iced, "{what}: asmkit vs iced-x86 (32-bit)");
    }

    /// Decode-level equivalence at 32 bit (for the divergences listed above).
    #[track_caller]
    fn same_decode(what: &str, ours: &[u8], iced: &[u8]) {
        let decode = |bytes: &[u8]| -> Vec<iced_x86::Instruction> {
            let mut dec = iced_x86::Decoder::new(32, bytes, 0);
            let mut out = Vec::new();
            while dec.position() < bytes.len() {
                let instr = dec.decode();
                assert!(!instr.is_invalid(), "{what}: undecodable: {bytes:02X?}");
                out.push(instr);
            }
            out
        };
        let (e, a) = (decode(iced), decode(ours));
        assert_eq!(e.len(), a.len(), "{what}: instruction count");
        assert!(
            e.iter().zip(a.iter()).all(|(e, a)| instr_equivalent(e, a)),
            "{what}: {e:?} vs {a:?}"
        );
    }

    /// asmkit `emit_n` shorthand.
    fn emit1(a: &mut Assembler, id: InstId, op0: &asmkit::Operand) {
        a.emit_n(id as u32, &[op0]);
    }

    #[test]
    fn gp_reg_reg_all_sizes() {
        use code_asm::{ah, al, ax, bh, bl, bx, cl, cx, dx, eax, ebx, ecx, edx};
        same_bytes(
            "mov eax, ebx",
            asm32(|a| a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), EBX.as_operand()])),
            iced32(|a| a.mov(eax, ebx).unwrap()),
        );
        same_bytes(
            "mov ax, bx",
            asm32(|a| a.emit_n(InstId::Mov as u32, &[AX.as_operand(), BX.as_operand()])),
            iced32(|a| a.mov(ax, bx).unwrap()),
        );
        same_bytes(
            "mov al, bl",
            asm32(|a| a.emit_n(InstId::Mov as u32, &[AL.as_operand(), BL.as_operand()])),
            iced32(|a| a.mov(al, bl).unwrap()),
        );
        same_bytes(
            "mov ah, bh",
            asm32(|a| a.emit_n(InstId::Mov as u32, &[AH.as_operand(), BH.as_operand()])),
            iced32(|a| a.mov(ah, bh).unwrap()),
        );
        same_bytes(
            "add ecx, edx",
            asm32(|a| a.emit_n(InstId::Add as u32, &[ECX.as_operand(), EDX.as_operand()])),
            iced32(|a| a.add(ecx, edx).unwrap()),
        );
        same_bytes(
            "xor eax, ebx",
            asm32(|a| a.emit_n(InstId::Xor as u32, &[EAX.as_operand(), EBX.as_operand()])),
            iced32(|a| a.xor(eax, ebx).unwrap()),
        );
        same_bytes(
            "sub dx, cx",
            asm32(|a| a.emit_n(InstId::Sub as u32, &[DX.as_operand(), CX.as_operand()])),
            iced32(|a| a.sub(dx, cx).unwrap()),
        );
        same_bytes(
            "and cl, al",
            asm32(|a| a.emit_n(InstId::And as u32, &[CL.as_operand(), AL.as_operand()])),
            iced32(|a| a.and(cl, al).unwrap()),
        );
    }

    #[test]
    fn gp_imm_forms() {
        use code_asm::{ax, cl, eax, ecx};
        same_bytes(
            "mov eax, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[EAX.as_operand(), imm(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.mov(eax, 0x1234_5678u32).unwrap()),
        );
        same_bytes(
            "mov ax, imm16",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), imm(0x1234).as_operand()],
                )
            }),
            iced32(|a| a.mov(ax, 0x1234i32).unwrap()),
        );
        same_bytes(
            "mov cl, imm8",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[CL.as_operand(), imm(0x12).as_operand()],
                )
            }),
            iced32(|a| a.mov(cl, 0x12i32).unwrap()),
        );
        // Accumulator short forms.
        same_bytes(
            "add eax, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Add as u32,
                    &[EAX.as_operand(), imm(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.add(eax, 0x1234_5678i32).unwrap()),
        );
        same_bytes(
            "cmp eax, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Cmp as u32,
                    &[EAX.as_operand(), imm(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.cmp(eax, 0x1234_5678i32).unwrap()),
        );
        same_bytes(
            "test eax, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Test as u32,
                    &[EAX.as_operand(), imm(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.test(eax, 0x1234_5678u32).unwrap()),
        );
        // imm8 sign-extended forms.
        same_bytes(
            "add ecx, 1",
            asm32(|a| a.emit_n(InstId::Add as u32, &[ECX.as_operand(), imm(1).as_operand()])),
            iced32(|a| a.add(ecx, 1i32).unwrap()),
        );
        same_bytes(
            "or edx, -1",
            asm32(|a| a.emit_n(InstId::Or as u32, &[EDX.as_operand(), imm(-1).as_operand()])),
            iced32(|a| a.or(code_asm::edx, -1i32).unwrap()),
        );
        same_bytes(
            "test al, 1",
            asm32(|a| a.emit_n(InstId::Test as u32, &[AL.as_operand(), imm(1).as_operand()])),
            iced32(|a| a.test(code_asm::al, 1i32).unwrap()),
        );

        // A fixed seed keeps this independent boundary sweep reproducible.
        // MOV r32, imm32 has one unambiguous encoding; unlike ADD, it cannot
        // validly differ only because one assembler picked an imm8 form.
        let boundaries = [i32::MIN, -129, -128, -1, 0, 1, 127, 128, i32::MAX];
        let mut state = 0x5EED_C0DEu32;
        for _ in 0..64 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let value = boundaries[(state as usize) % boundaries.len()];
            same_bytes(
                "fixed-seed mov eax, imm32 boundary",
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), imm(value).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, value as u32).unwrap()),
            );
        }
    }

    #[test]
    fn gp_unary_shift_forms() {
        use code_asm::{cl, eax, ebx, ecx, edx};
        same_bytes(
            "neg/not/mul/imul/idiv",
            asm32(|a| {
                emit1(a, InstId::Neg, ECX.as_operand());
                emit1(a, InstId::Not, EDX.as_operand());
                emit1(a, InstId::Mul, EBX.as_operand());
                emit1(a, InstId::Idiv, ECX.as_operand());
            }),
            iced32(|a| {
                a.neg(ecx).unwrap();
                a.not(edx).unwrap();
                a.mul(ebx).unwrap();
                a.idiv(ecx).unwrap();
            }),
        );
        same_bytes(
            "imul ecx, edx",
            asm32(|a| a.emit_n(InstId::Imul as u32, &[ECX.as_operand(), EDX.as_operand()])),
            iced32(|a| a.imul_2(ecx, edx).unwrap()),
        );
        same_bytes(
            "imul ecx, edx, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Imul as u32,
                    &[ECX.as_operand(), EDX.as_operand(), imm(0x1234).as_operand()],
                )
            }),
            iced32(|a| a.imul_3(ecx, edx, 0x1234i32).unwrap()),
        );
        same_bytes(
            "imul ecx, edx, imm8",
            asm32(|a| {
                a.emit_n(
                    InstId::Imul as u32,
                    &[ECX.as_operand(), EDX.as_operand(), imm(4).as_operand()],
                )
            }),
            iced32(|a| a.imul_3(ecx, edx, 4i32).unwrap()),
        );
        same_bytes(
            "shl/shr/sar/rol",
            asm32(|a| {
                a.emit_n(InstId::Shl as u32, &[ECX.as_operand(), imm(2).as_operand()]);
                a.emit_n(InstId::Shl as u32, &[ECX.as_operand(), imm(1).as_operand()]);
                a.emit_n(InstId::Shr as u32, &[EDX.as_operand(), CL.as_operand()]);
                a.emit_n(InstId::Sar as u32, &[EAX.as_operand(), CL.as_operand()]);
                a.emit_n(InstId::Rol as u32, &[EAX.as_operand(), imm(1).as_operand()]);
            }),
            iced32(|a| {
                a.shl(ecx, 2i32).unwrap();
                a.shl(ecx, 1i32).unwrap();
                a.shr(edx, cl).unwrap();
                a.sar(eax, cl).unwrap();
                a.rol(eax, 1i32).unwrap();
            }),
        );
        same_bytes(
            "shld forms",
            asm32(|a| {
                a.emit_n(
                    InstId::Shld as u32,
                    &[ECX.as_operand(), EDX.as_operand(), imm(4).as_operand()],
                );
                a.emit_n(
                    InstId::Shld as u32,
                    &[ECX.as_operand(), EDX.as_operand(), CL.as_operand()],
                );
            }),
            iced32(|a| {
                a.shld(ecx, edx, 4i32).unwrap();
                a.shld(ecx, edx, cl).unwrap();
            }),
        );
        same_bytes(
            "bt/bts",
            asm32(|a| {
                a.emit_n(InstId::Bt as u32, &[ECX.as_operand(), EDX.as_operand()]);
                a.emit_n(InstId::Bts as u32, &[ECX.as_operand(), imm(3).as_operand()]);
            }),
            iced32(|a| {
                a.bt(ecx, edx).unwrap();
                a.bts(ecx, 3i32).unwrap();
            }),
        );
    }

    #[test]
    fn gp_misc_forms() {
        use code_asm::{al, bl, bx, eax, ebx, ecx};
        same_bytes(
            "movzx/movsx",
            asm32(|a| {
                a.emit_n(InstId::Movzx as u32, &[EAX.as_operand(), BL.as_operand()]);
                a.emit_n(InstId::Movsx as u32, &[EAX.as_operand(), BL.as_operand()]);
                a.emit_n(InstId::Movzx as u32, &[EAX.as_operand(), BX.as_operand()]);
                a.emit_n(
                    InstId::Movsx as u32,
                    &[EAX.as_operand(), word_ptr(ECX, 0).as_operand()],
                );
            }),
            iced32(|a| {
                a.movzx(eax, bl).unwrap();
                a.movsx(eax, bl).unwrap();
                a.movzx(eax, bx).unwrap();
                a.movsx(eax, code_asm::word_ptr(ecx)).unwrap();
            }),
        );
        same_bytes(
            "setcc/cmovcc",
            asm32(|a| {
                emit1(a, InstId::Setz, AL.as_operand());
                emit1(a, InstId::Setnz, byte_ptr(ECX, 0).as_operand());
                a.emit_n(InstId::Cmovz as u32, &[EAX.as_operand(), EBX.as_operand()]);
                a.emit_n(
                    InstId::Cmovz as u32,
                    &[EAX.as_operand(), dword_ptr(ECX, 0).as_operand()],
                );
            }),
            iced32(|a| {
                a.setz(al).unwrap();
                a.setnz(code_asm::byte_ptr(ecx)).unwrap();
                a.cmovz(eax, ebx).unwrap();
                a.cmovz(eax, code_asm::dword_ptr(ecx)).unwrap();
            }),
        );
        same_bytes(
            "xadd/cmpxchg",
            asm32(|a| {
                a.emit_n(InstId::Xadd as u32, &[ECX.as_operand(), EDX.as_operand()]);
                a.emit_n(
                    InstId::Cmpxchg as u32,
                    &[ECX.as_operand(), EDX.as_operand()],
                );
                a.emit_n(
                    InstId::Cmpxchg as u32,
                    &[byte_ptr(ECX, 0).as_operand(), DL.as_operand()],
                );
            }),
            iced32(|a| {
                a.xadd(ecx, code_asm::edx).unwrap();
                a.cmpxchg(ecx, code_asm::edx).unwrap();
                a.cmpxchg(code_asm::byte_ptr(ecx), code_asm::dl).unwrap();
            }),
        );
        same_bytes(
            "single-byte/misc",
            asm32(|a| {
                emit1(a, InstId::Bswap, ECX.as_operand());
                a.emit_n(InstId::Xlatb as u32, &[]);
                a.emit_n(InstId::Cbw as u32, &[]);
                a.emit_n(InstId::Cwde as u32, &[]);
                a.emit_n(InstId::Cdq as u32, &[]);
                a.emit_n(InstId::Nop as u32, &[]);
                a.emit_n(InstId::Leave as u32, &[]);
                a.emit_n(InstId::Cpuid as u32, &[]);
                a.emit_n(InstId::Rdtsc as u32, &[]);
                a.emit_n(InstId::Int3 as u32, &[]);
            }),
            iced32(|a| {
                a.bswap(ecx).unwrap();
                a.xlatb().unwrap();
                a.cbw().unwrap();
                a.cwde().unwrap();
                a.cdq().unwrap();
                a.nop().unwrap();
                a.leave().unwrap();
                a.cpuid().unwrap();
                a.rdtsc().unwrap();
                a.int3().unwrap();
            }),
        );
        same_bytes(
            "enter/ret",
            asm32(|a| {
                a.emit_n(
                    InstId::Enter as u32,
                    &[imm(0x100).as_operand(), imm(2).as_operand()],
                );
                a.emit_n(InstId::Ret as u32, &[]);
                a.emit_n(InstId::Ret as u32, &[imm(0x10).as_operand()]);
            }),
            iced32(|a| {
                a.enter(0x100i32, 2i32).unwrap();
                a.ret().unwrap();
                a.ret_1(0x10i32).unwrap();
            }),
        );
    }

    #[test]
    fn modrm_sib_forms() {
        use code_asm::{dword_ptr as dp, eax, ebp, ebx, ecx, edi, edx, esi, esp};
        let cases: [(Vec<u8>, Vec<u8>, &str); 8] = [
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), dword_ptr(ECX, 0).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, dp(ecx)).unwrap()),
                "mov eax, [ecx]",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), dword_ptr(EBP, 0).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, dp(ebp)).unwrap()),
                "mov eax, [ebp] (forced disp8)",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), dword_ptr(ESP, 8).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, dp(esp + 8)).unwrap()),
                "mov eax, [esp + 8] (forced SIB)",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), dword_ptr(ECX, 0x1234_5678).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, dp(ecx + 0x1234_5678)).unwrap()),
                "mov eax, [ecx + disp32]",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[EAX.as_operand(), dword_ptr(ECX, -8).as_operand()],
                    )
                }),
                iced32(|a| a.mov(eax, dp(ecx - 8)).unwrap()),
                "mov eax, [ecx - 8]",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[
                            EAX.as_operand(),
                            dword_ptr_index(ECX, EDX, 2, 0x20).as_operand(),
                        ],
                    )
                }),
                iced32(|a| a.mov(eax, dp(ecx + edx * 4 + 0x20)).unwrap()),
                "mov eax, [ecx + edx*4 + 0x20]",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[
                            EAX.as_operand(),
                            dword_ptr_index(ESI, EDI, 1, 0).as_operand(),
                        ],
                    )
                }),
                iced32(|a| a.mov(eax, dp(esi + edi * 2)).unwrap()),
                "mov eax, [esi + edi*2]",
            ),
            (
                asm32(|a| {
                    a.emit_n(
                        InstId::Mov as u32,
                        &[
                            dword_ptr_index(ECX, EDX, 3, 0x100).as_operand(),
                            EBX.as_operand(),
                        ],
                    )
                }),
                iced32(|a| a.mov(dp(ecx + edx * 8 + 0x100), ebx).unwrap()),
                "mov [ecx + edx*8 + 0x100], ebx",
            ),
        ];
        for (ours, iced, what) in cases {
            same_bytes(what, ours, iced);
        }
    }

    #[test]
    fn absolute_addressing() {
        use code_asm::{dword_ptr as dp, eax, ecx};
        // moffs forms (accumulator only).
        same_bytes(
            "mov eax, [abs]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[EAX.as_operand(), dword_ptr_u64(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.mov(eax, dp(0x1234_5678u64)).unwrap()),
        );
        same_bytes(
            "mov [abs], eax",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[dword_ptr_u64(0x1234_5678).as_operand(), EAX.as_operand()],
                )
            }),
            iced32(|a| a.mov(dp(0x1234_5678u64), eax).unwrap()),
        );
        same_bytes(
            "mov ax, [abs]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), word_ptr_u64(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| {
                a.mov(code_asm::ax, code_asm::word_ptr(0x1234_5678u64))
                    .unwrap()
            }),
        );
        // Non-accumulator registers use the mod/disp32 absolute form.
        same_bytes(
            "mov ecx, [abs]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[ECX.as_operand(), dword_ptr_u64(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.mov(ecx, dp(0x1234_5678u64)).unwrap()),
        );
        same_bytes(
            "mov [abs], ecx",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[dword_ptr_u64(0x1234_5678).as_operand(), ECX.as_operand()],
                )
            }),
            iced32(|a| a.mov(dp(0x1234_5678u64), ecx).unwrap()),
        );
        // lea with a 32-bit absolute address.
        same_bytes(
            "lea eax, [abs]",
            asm32(|a| {
                a.emit_n(
                    InstId::Lea as u32,
                    &[EAX.as_operand(), dword_ptr_u64(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.lea(eax, dp(0x1234_5678u64)).unwrap()),
        );
        // Segment override with an absolute address.
        same_bytes(
            "mov eax, fs:[0x40]",
            asm32(|a| {
                a.fs();
                a.emit_n(
                    InstId::Mov as u32,
                    &[EAX.as_operand(), dword_ptr_u64(0x40).as_operand()],
                );
            }),
            iced32(|a| a.mov(eax, dp(0x40u64).fs()).unwrap()),
        );
        // A 64-bit absolute address is rejected by asmkit (and meaningless to
        // iced's 32-bit assembler).
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[EAX.as_operand(), dword_ptr_u64(0x1_0000_0000).as_operand()]
        )));
    }

    #[test]
    fn mod16_addressing() {
        use code_asm::{al, ax, bp, bx, di, si, word_ptr as wp};
        // 8-bit forms carry no 66h prefix, so the prefix-order divergence does
        // not apply and the bytes are identical.
        same_bytes(
            "mov al, [bx]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AL.as_operand(), byte_ptr(BX, 0).as_operand()],
                )
            }),
            iced32(|a| a.mov(al, code_asm::byte_ptr(bx)).unwrap()),
        );
        same_bytes(
            "mov al, [bx + si + 0x10]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[
                        AL.as_operand(),
                        byte_ptr_index(BX, SI, 0, 0x10).as_operand(),
                    ],
                )
            }),
            iced32(|a| a.mov(al, code_asm::byte_ptr(bx + si + 0x10)).unwrap()),
        );
        same_bytes(
            "mov al, [di]",
            asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AL.as_operand(), byte_ptr(DI, 0).as_operand()],
                )
            }),
            iced32(|a| a.mov(al, code_asm::byte_ptr(di)).unwrap()),
        );
        // 16-bit forms: asmkit emits AsmJit's prefix order (67 66), iced emits
        // (66 67) — compare decode-equivalence.
        same_decode(
            "mov ax, [bx + si]",
            &asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), word_ptr_index(BX, SI, 0, 0).as_operand()],
                )
            }),
            &iced32(|a| a.mov(ax, wp(bx + si)).unwrap()),
        );
        same_decode(
            "mov ax, [bx + disp16]",
            &asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), word_ptr(BX, 0x1234).as_operand()],
                )
            }),
            &iced32(|a| a.mov(ax, wp(bx + 0x1234)).unwrap()),
        );
        same_decode(
            "mov ax, [bp] (forced disp8)",
            &asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), word_ptr(BP, 0).as_operand()],
                )
            }),
            &iced32(|a| a.mov(ax, wp(bp)).unwrap()),
        );
        same_decode(
            "mov ax, [si + 8]",
            &asm32(|a| {
                a.emit_n(
                    InstId::Mov as u32,
                    &[AX.as_operand(), word_ptr(SI, 8).as_operand()],
                )
            }),
            &iced32(|a| a.mov(ax, wp(si + 8)).unwrap()),
        );
        // Invalid 16-bit address combinations are rejected.
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[AX.as_operand(), word_ptr(AX, 0).as_operand()]
        )));
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[AX.as_operand(), word_ptr_index(BX, SI, 1, 0).as_operand()]
        )));
    }

    #[test]
    fn inc_dec_short_forms() {
        use code_asm::{ax, dx, eax, ebp, ebx, ecx, edi, edx, esi, esp};
        same_bytes(
            "inc r32 (all)",
            asm32(|a| {
                for r in [EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI] {
                    emit1(a, InstId::Inc, r.as_operand());
                }
            }),
            iced32(|a| {
                for r in [eax, ecx, edx, ebx, esp, ebp, esi, edi] {
                    a.inc(r).unwrap();
                }
            }),
        );
        same_bytes(
            "dec r32 (all)",
            asm32(|a| {
                for r in [EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI] {
                    emit1(a, InstId::Dec, r.as_operand());
                }
            }),
            iced32(|a| {
                for r in [eax, ecx, edx, ebx, esp, ebp, esi, edi] {
                    a.dec(r).unwrap();
                }
            }),
        );
        same_bytes(
            "inc ax / dec dx",
            asm32(|a| {
                emit1(a, InstId::Inc, AX.as_operand());
                emit1(a, InstId::Dec, DX.as_operand());
            }),
            iced32(|a| {
                a.inc(ax).unwrap();
                a.dec(dx).unwrap();
            }),
        );
    }

    #[test]
    fn push_pop_forms() {
        use code_asm::{ax, eax, ecx, edx};
        same_bytes(
            "push/pop r32+r16",
            asm32(|a| {
                emit1(a, InstId::Push, EAX.as_operand());
                emit1(a, InstId::Push, AX.as_operand());
                emit1(a, InstId::Pop, ECX.as_operand());
                emit1(a, InstId::Pop, CX.as_operand());
            }),
            iced32(|a| {
                a.push(eax).unwrap();
                a.push(ax).unwrap();
                a.pop(ecx).unwrap();
                a.pop(code_asm::cx).unwrap();
            }),
        );
        same_bytes(
            "push imm",
            asm32(|a| {
                a.emit_n(InstId::Push as u32, &[imm(0x12).as_operand()]);
                a.emit_n(InstId::Push as u32, &[imm(0x1234_5678).as_operand()]);
            }),
            iced32(|a| {
                a.push(0x12i32).unwrap();
                a.push(0x1234_5678u32).unwrap();
            }),
        );
        same_bytes(
            "push/pop m32+m16",
            asm32(|a| {
                emit1(a, InstId::Push, dword_ptr(ECX, 0).as_operand());
                emit1(a, InstId::Push, word_ptr(ECX, 0).as_operand());
                emit1(a, InstId::Pop, dword_ptr(EDX, 0).as_operand());
            }),
            iced32(|a| {
                a.push(code_asm::dword_ptr(ecx)).unwrap();
                a.push(code_asm::word_ptr(ecx)).unwrap();
                a.pop(code_asm::dword_ptr(edx)).unwrap();
            }),
        );
        // push/pop m64 is rejected in 32-bit mode.
        assert!(asm32_is_err(|a| emit1(
            a,
            InstId::Push,
            asmkit::x86::operands::qword_ptr(ECX, 0).as_operand()
        )));
        assert!(asm32_is_err(|a| emit1(
            a,
            InstId::Pop,
            asmkit::x86::operands::qword_ptr(ECX, 0).as_operand()
        )));
    }

    #[test]
    fn far_pointer_forms() {
        use code_asm::{ecx, fword_ptr as fp};
        same_bytes(
            "lcall imm16, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Lcall as u32,
                    &[imm(0x1234).as_operand(), imm(0x1234_5678).as_operand()],
                )
            }),
            iced32(|a| a.call_far(0x1234, 0x1234_5678).unwrap()),
        );
        same_bytes(
            "ljmp imm16, imm32",
            asm32(|a| {
                a.emit_n(
                    InstId::Ljmp as u32,
                    &[imm(0x10).as_operand(), imm(0x20).as_operand()],
                )
            }),
            iced32(|a| a.jmp_far(0x10, 0x20).unwrap()),
        );
        same_bytes(
            "lcall fword [ecx]",
            asm32(|a| emit1(a, InstId::Lcall, fword_ptr(ECX, 0).as_operand())),
            iced32(|a| a.call(fp(ecx)).unwrap()),
        );
        same_bytes(
            "ljmp fword [ecx]",
            asm32(|a| emit1(a, InstId::Ljmp, fword_ptr(ECX, 0).as_operand())),
            iced32(|a| a.jmp(fp(ecx)).unwrap()),
        );
        // Selector above 0xFFFF is rejected.
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Lcall as u32,
            &[imm(0x1_0000).as_operand(), imm(0).as_operand()]
        )));
    }

    #[test]
    fn branch_label_forms() {
        // call has no rel8 form: byte-identical forward and backward.
        same_bytes(
            "call forward",
            asm32(|a| {
                let l = a.get_label();
                a.emit_n(InstId::Call as u32, &[Label::from_id(l.id()).as_operand()]);
                a.bind_label(l);
                a.emit_n(InstId::Nop as u32, &[]);
            }),
            iced32(|a| {
                let l = a.create_label();
                a.call(l).unwrap();
                let mut l = l;
                a.set_label(&mut l).unwrap();
                a.nop().unwrap();
            }),
        );
        same_bytes(
            "call backward",
            asm32(|a| {
                let l = a.get_label();
                a.bind_label(l);
                a.emit_n(InstId::Call as u32, &[Label::from_id(l.id()).as_operand()]);
            }),
            iced32(|a| {
                let mut l = a.create_label();
                a.set_label(&mut l).unwrap();
                a.call(l).unwrap();
            }),
        );
        // iced auto-shortens jmp/jcc to a bound label (rel8); asmkit keeps
        // rel32 — decode-equivalence on the branch target.
        same_decode(
            "jmp backward",
            &asm32(|a| {
                let l = a.get_label();
                a.bind_label(l);
                a.emit_n(InstId::Jmp as u32, &[Label::from_id(l.id()).as_operand()]);
            }),
            &iced32(|a| {
                let mut l = a.create_label();
                a.set_label(&mut l).unwrap();
                a.jmp(l).unwrap();
            }),
        );
        same_decode(
            "jz backward",
            &asm32(|a| {
                let l = a.get_label();
                a.bind_label(l);
                a.emit_n(InstId::Jz as u32, &[Label::from_id(l.id()).as_operand()]);
            }),
            &iced32(|a| {
                let mut l = a.create_label();
                a.set_label(&mut l).unwrap();
                a.jz(l).unwrap();
            }),
        );
        // Label-based memory operands become absolute addresses (RelToAbs).
        // iced's CodeAsm cannot express absolute label addressing in 32-bit
        // mode, so compare against the expected bytes here; the unbound-label
        // Abs4 reloc path is covered by encoder unit tests.
        assert_eq!(
            asm32(|a| {
                let l = a.get_label();
                a.bind_label(l);
                a.emit_n(
                    InstId::Mov as u32,
                    &[EAX.as_operand(), dword_ptr_label(l, 8).as_operand()],
                );
            }),
            [0x8B, 0x05, 0x08, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn string_ops() {
        // Implicit-operand forms spelled out, [edi]/[esi] bases (no 67h).
        same_bytes(
            "movs/stos/lods/scas/cmps",
            asm32(|a| {
                a.emit_n(
                    InstId::Movs as u32,
                    &[byte_ptr(EDI, 0).as_operand(), byte_ptr(ESI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Movs as u32,
                    &[word_ptr(EDI, 0).as_operand(), word_ptr(ESI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Movs as u32,
                    &[
                        dword_ptr(EDI, 0).as_operand(),
                        dword_ptr(ESI, 0).as_operand(),
                    ],
                );
                a.emit_n(
                    InstId::Stos as u32,
                    &[byte_ptr(EDI, 0).as_operand(), AL.as_operand()],
                );
                a.emit_n(
                    InstId::Stos as u32,
                    &[dword_ptr(EDI, 0).as_operand(), EAX.as_operand()],
                );
                a.emit_n(
                    InstId::Lods as u32,
                    &[AL.as_operand(), byte_ptr(ESI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Lods as u32,
                    &[EAX.as_operand(), dword_ptr(ESI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Scas as u32,
                    &[AL.as_operand(), byte_ptr(EDI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Scas as u32,
                    &[EAX.as_operand(), dword_ptr(EDI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Cmps as u32,
                    &[byte_ptr(EDI, 0).as_operand(), byte_ptr(ESI, 0).as_operand()],
                );
                a.emit_n(
                    InstId::Cmps as u32,
                    &[
                        dword_ptr(EDI, 0).as_operand(),
                        dword_ptr(ESI, 0).as_operand(),
                    ],
                );
            }),
            iced32(|a| {
                a.movsb().unwrap();
                a.movsw().unwrap();
                a.movsd().unwrap();
                a.stosb().unwrap();
                a.stosd().unwrap();
                a.lodsb().unwrap();
                a.lodsd().unwrap();
                a.scasb().unwrap();
                a.scasd().unwrap();
                a.cmpsb().unwrap();
                a.cmpsd().unwrap();
            }),
        );
        same_bytes(
            "rep movsd",
            asm32(|a| {
                a.rep();
                a.emit_n(
                    InstId::Movs as u32,
                    &[
                        dword_ptr(EDI, 0).as_operand(),
                        dword_ptr(ESI, 0).as_operand(),
                    ],
                );
            }),
            iced32(|a| a.rep().movsd().unwrap()),
        );
        // Explicit jecxz cx: 67h in 32-bit mode (raw-disp8 immediate form).
        same_bytes(
            "jecxz cx, -2",
            asm32(|a| {
                a.emit_n(
                    InstId::Jecxz as u32,
                    &[CX.as_operand(), imm(-2).as_operand()],
                )
            }),
            vec![0x67, 0xE3, 0xFE],
        );
    }

    #[test]
    fn creg_dreg_forms() {
        use code_asm::{cr0, cr8, dr0, eax};
        same_bytes(
            "mov eax, cr0 / mov cr0, eax",
            asm32(|a| {
                a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), CR0.as_operand()]);
                a.emit_n(InstId::Mov as u32, &[CR0.as_operand(), EAX.as_operand()]);
            }),
            iced32(|a| {
                a.mov(eax, cr0).unwrap();
                a.mov(cr0, eax).unwrap();
            }),
        );
        // CR8+ uses the LOCK prefix in 32-bit mode (AMD extension).
        same_bytes(
            "mov eax, cr8 / mov cr8, eax",
            asm32(|a| {
                a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), CR8.as_operand()]);
                a.emit_n(InstId::Mov as u32, &[CR8.as_operand(), EAX.as_operand()]);
            }),
            iced32(|a| {
                a.mov(eax, cr8).unwrap();
                a.mov(cr8, eax).unwrap();
            }),
        );
        same_bytes(
            "mov eax, dr0 / mov dr0, eax",
            asm32(|a| {
                a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), DR0.as_operand()]);
                a.emit_n(InstId::Mov as u32, &[DR0.as_operand(), EAX.as_operand()]);
            }),
            iced32(|a| {
                a.mov(eax, dr0).unwrap();
                a.mov(dr0, eax).unwrap();
            }),
        );
    }

    #[test]
    fn fpu_forms() {
        use code_asm::{dword_ptr as dp, ecx, st0};
        same_bytes(
            "fadd st0, st0 / fadd m32 / fld m32 / fstp m32",
            asm32(|a| {
                a.emit_n(InstId::Fadd as u32, &[ST0.as_operand(), ST0.as_operand()]);
                a.emit_n(InstId::Fadd as u32, &[dword_ptr(ECX, 0).as_operand()]);
                a.emit_n(InstId::Fld as u32, &[dword_ptr(ECX, 0).as_operand()]);
                a.emit_n(InstId::Fstp as u32, &[dword_ptr(ECX, 0).as_operand()]);
            }),
            iced32(|a| {
                a.fadd_2(st0, st0).unwrap();
                a.fadd(dp(ecx)).unwrap();
                a.fld(dp(ecx)).unwrap();
                a.fstp(dp(ecx)).unwrap();
            }),
        );
    }

    #[test]
    fn sse_avx_forms() {
        use code_asm::{ecx, xmm0, xmm1, xmm2, xmm3, ymm1, ymm2, ymm3};
        same_bytes(
            "paddw xmm0, xmm1",
            asm32(|a| {
                a.emit_n(
                    InstId::Paddw as u32,
                    &[XMM0.as_operand(), XMM1.as_operand()],
                )
            }),
            iced32(|a| a.paddw(xmm0, xmm1).unwrap()),
        );
        same_bytes(
            "vaddps xmm/ymm",
            asm32(|a| {
                a.emit_n(
                    InstId::Vaddps as u32,
                    &[XMM1.as_operand(), XMM2.as_operand(), XMM3.as_operand()],
                );
                a.emit_n(
                    InstId::Vaddps as u32,
                    &[YMM1.as_operand(), YMM2.as_operand(), YMM3.as_operand()],
                );
            }),
            iced32(|a| {
                a.vaddps(xmm1, xmm2, xmm3).unwrap();
                a.vaddps(ymm1, ymm2, ymm3).unwrap();
            }),
        );
        same_bytes(
            "vmovdqu xmm0, [ecx]",
            asm32(|a| {
                a.emit_n(
                    InstId::Vmovdqu as u32,
                    &[
                        XMM0.as_operand(),
                        asmkit::x86::operands::oword_ptr(ECX, 0).as_operand(),
                    ],
                )
            }),
            iced32(|a| a.vmovdqu(xmm0, code_asm::xmmword_ptr(ecx)).unwrap()),
        );
        // EVEX is encodable in 32-bit mode (registers below 8 need no R'/X' bits).
        same_bytes(
            "vaddpd zmm1, zmm2, zmm3",
            asm32(|a| {
                a.emit_n(
                    InstId::Vaddpd as u32,
                    &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
                )
            }),
            iced32(|a| {
                a.vaddpd(code_asm::zmm1, code_asm::zmm2, code_asm::zmm3)
                    .unwrap()
            }),
        );
    }

    #[test]
    fn xchg_accumulator_decode() {
        // asmkit uses the 90+r short form (AsmJit), iced the ModRM form.
        same_decode(
            "xchg eax, ecx",
            &asm32(|a| a.emit_n(InstId::Xchg as u32, &[EAX.as_operand(), ECX.as_operand()])),
            &iced32(|a| a.xchg(code_asm::eax, code_asm::ecx).unwrap()),
        );
        same_decode(
            "xchg eax, eax (nop)",
            &asm32(|a| a.emit_n(InstId::Xchg as u32, &[EAX.as_operand(), EAX.as_operand()])),
            &iced32(|a| a.xchg(code_asm::eax, code_asm::eax).unwrap()),
        );
    }

    #[test]
    fn mode_gating() {
        // X64-only instructions are rejected by both 32-bit assemblers.
        // (iced's CodeAsm assembles `syscall` in 32-bit mode without
        // complaint, so only the asmkit side is checked here.)
        assert!(asm32_is_err(|a| a.emit_n(InstId::Syscall as u32, &[])));
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Movsxd as u32,
            &[EAX.as_operand(), EBX.as_operand()]
        )));
        assert!(iced32_is_err(|a| a.movsxd(code_asm::eax, code_asm::ebx)));
        // 64-bit registers are rejected by both.
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[RAX.as_operand(), RBX.as_operand()]
        )));
        assert!(iced32_is_err(|a| a.mov(code_asm::rax, code_asm::rbx)));
        assert!(asm32_is_err(|a| emit1(a, InstId::Push, RAX.as_operand())));
        // Register ids above 7 are rejected (asmkit side).
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[EAX.as_operand(), R8D.as_operand()]
        )));
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Paddw as u32,
            &[XMM0.as_operand(), XMM8.as_operand()]
        )));
        // 64-bit addressing registers are rejected in 32-bit mode.
        assert!(asm32_is_err(|a| a.emit_n(
            InstId::Mov as u32,
            &[EAX.as_operand(), dword_ptr(RBX, 0).as_operand()]
        )));
        // 16-bit addressing is rejected in 64-bit mode.
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut a64 = Assembler::new(&mut buf);
        a64.emit_n(
            InstId::Mov as u32,
            &[RAX.as_operand(), word_ptr(BX, 0).as_operand()],
        );
        assert!(buf.error().is_some());
    }
}
