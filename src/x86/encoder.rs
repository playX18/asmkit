//! X86 encoder helpers, buffer-writer functions, and emit handlers (port of the
//! helper layer and the `EmitX86Op`..`EmitRel` emit handlers of AsmJit's
//! `x86assembler.cpp`: helpers, `X86BufferWriter`, `FIXUP_GPB`, `ENC_OPSn`, and the
//! emit-handler layer reached by the encoding arms).
//!
//! The emit handlers are 64-bit only: arms that only exist in 32-bit mode are kept
//! but stubbed (`IS_32BIT_MODE` is `false`).
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

use crate::X86Error;
use crate::core::buffer::{CodeBuffer, LabelUse, Reloc, RelocDistance, RelocTarget};
use crate::core::globals::{INVALID_ID, InstOptions};
use crate::core::operand::{Label, Operand, OperandCast, RegType, Sym};

use super::encoder_tables::{
    CDISP8_SHL_TABLE, LL_BY_REG_TYPE_TABLE, LL_BY_SIZE_DIV_16_TABLE, MEM_INFO_67H_X64,
    MEM_INFO_67H_X86, MEM_INFO_BASE_GP, MEM_INFO_BASE_LABEL, MEM_INFO_BASE_RIP, MEM_INFO_INDEX,
    MEM_INFO_TABLE, MOD16_BASE_INDEX_TABLE, MOD16_BASE_TABLE, OPCODE_MM_TABLE, OPCODE_PP_TABLE,
    SEGMENT_PREFIX_TABLE, VEX_PREFIX_TABLE, VEX_VVVVV_SHIFT, X86_BYTE_EVEX, X86_BYTE_INVALID_REX,
    X86_BYTE_REX, X86_BYTE_REX_W, X86_BYTE_VEX2, X86_BYTE_VEX3,
};
use super::instdb::{ALT_OPCODE_TABLE, Avx512Flags, CommonInfo, InstFlags, InstId, InstInfo};
use super::opcode::Opcode;
use super::operands::{AddrType, Gp, Mem, SReg};

/// Tests whether `op` is a memory operand with base register `base` and no offset
/// (AsmJit's `is_implicit_mem` — used by string ops' implicit `[zAX]` forms).
pub fn is_implicit_mem(op: &Operand, base: u32) -> bool {
    op.is_mem() && op.id() == base && !op.as_::<super::operands::Mem>().has_offset()
}

/// Combines `reg_id` and `vvvvv_id` into a single value (used by AVX and AVX-512).
pub const fn pack_reg_and_vvvvv(reg_id: u32, vvvvv_id: u32) -> u32 {
    reg_id + (vvvvv_id << VEX_VVVVV_SHIFT)
}

/// LL opcode field from a memory operand's index (vector) register type.
pub fn opcode_l_by_vmem(op: &Operand) -> u32 {
    LL_BY_REG_TYPE_TABLE[op.as_::<super::operands::Mem>().index_type() as usize]
}

/// LL opcode field from a register size in bytes.
pub fn opcode_l_by_size(size: u32) -> u32 {
    LL_BY_SIZE_DIV_16_TABLE[(size / 16) as usize]
}

/// Encodes a ModR/M byte.
pub const fn encode_mod(m: u32, o: u32, rm: u32) -> u32 {
    debug_assert!(m <= 3 && o <= 7 && rm <= 7);
    (m << 6) + (o << 3) + rm
}

/// Encodes a SIB byte.
pub const fn encode_sib(s: u32, i: u32, b: u32) -> u32 {
    debug_assert!(s <= 3 && i <= 7 && b <= 7);
    (s << 6) + (i << 3) + b
}

/// Validates the REX value: 0x00 ok (any mode), 0x40-0x4F ok (X64), 0x80 ok (X86),
/// 0x81-0xCF bad (REX prefix used in 32-bit mode).
pub const fn is_rex_invalid(rex: u32) -> bool {
    rex > X86_BYTE_INVALID_REX as u32
}

/// Moves the `X86_VEX3` option bit into the topmost bit (AsmJit's
/// `x86_get_force_evex3_mask_in_last_bit`).
pub const fn force_evex3_mask_in_last_bit(options: InstOptions) -> u32 {
    const VEX3_BIT: u32 = InstOptions::X86_VEX3.bits().trailing_zeros();
    (options.bits() & InstOptions::X86_VEX3.bits()) << (31 - VEX3_BIT)
}

/// Sign-extends the low 32 bits of `imm` to 64 bits.
pub const fn sign_extend_int32(imm: u64) -> u64 {
    (imm as u32 as i32) as i64 as u64
}

/// Tests whether the register is an MMX or XMM register.
pub fn is_mmx_or_xmm(reg_type: RegType) -> bool {
    reg_type == RegType::Extra || reg_type == RegType::Vec128
}

// Movabs heuristics (AsmJit's, simplified: asmkit has no base address at emit time, so the
// `is_absolute_location` path collapses to the raw-offset int32 check — 64-bit only).
// ------------------------------------------------------------------------------

/// Size of a 64-bit `movabs` instruction in bytes (AsmJit's
/// `x86_get_movabs_inst_size_64bit`).
pub fn movabs_inst_size_64bit(register_size: u32, options: InstOptions, rm_rel: &Mem) -> u32 {
    let segment_prefix_size = (rm_rel.segment_id() != 0) as u32;
    let x66h_prefix_size = (register_size == 2) as u32;
    let rex_prefix_size = (register_size == 8 || options.contains(InstOptions::X86_REX)) as u32;
    let opcode_byte_size = 1;
    let immediate_size = 8;

    segment_prefix_size + x66h_prefix_size + rex_prefix_size + opcode_byte_size + immediate_size
}

/// Decides whether to use the absolute (movabs) form for a memory operand (AsmJit's
/// `x86_should_use_movabs`, 64-bit simplified).
pub fn should_use_movabs(register_size: u32, options: InstOptions, rm_rel: &Mem) -> bool {
    let _ = register_size;
    // If the addressing type is REL or ModRM/ModMR was specified, absolute mov won't be used.
    if rm_rel.addr_type() == AddrType::Rel
        || options.intersects(InstOptions::X86_MOD_MR | InstOptions::X86_MOD_RM)
    {
        return false;
    }

    let addr_value = rm_rel.offset();
    // Relative addressing is always usable when the displacement fits int32.
    if i32::try_from(addr_value).is_ok() {
        return false;
    }

    addr_value as u64 > 0xFFFF_FFFF
}

/// `FIXUP_GPB` macro: if the operand is BPL|SPL|SIL|DIL|R8B-15B, force a REX prefix;
/// if it is AH|BH|CH|DH, patch its index from 0..3 to 4..7 and disallow REX.
pub fn fixup_gpb(options: &mut InstOptions, reg: &Gp, reg_id: &mut u32) {
    if !reg.is_gpb_hi() {
        if *reg_id >= 4 {
            *options |= InstOptions::X86_REX;
        }
    } else {
        *options |= InstOptions::X86_INVALID_REX;
        *reg_id += 4;
    }
}

/// `ENC_OPSn` — packs operand types into a 3-bit-per-operand signature (isign3/isign4).
#[allow(unused_macros)]
macro_rules! enc_ops {
    ($op0:expr) => {
        ($op0 as u32)
    };
    ($op0:expr, $op1:expr) => {
        ($op0 as u32) + (($op1 as u32) << 3)
    };
    ($op0:expr, $op1:expr, $op2:expr) => {
        ($op0 as u32) + (($op1 as u32) << 3) + (($op2 as u32) << 6)
    };
    ($op0:expr, $op1:expr, $op2:expr, $op3:expr) => {
        ($op0 as u32) + (($op1 as u32) << 3) + (($op2 as u32) << 6) + (($op3 as u32) << 9)
    };
    ($op0:expr, $op1:expr, $op2:expr, $op3:expr, $op4:expr) => {
        ($op0 as u32)
            + (($op1 as u32) << 3)
            + (($op2 as u32) << 6)
            + (($op3 as u32) << 9)
            + (($op4 as u32) << 12)
    };
    ($op0:expr, $op1:expr, $op2:expr, $op3:expr, $op4:expr, $op5:expr) => {
        ($op0 as u32)
            + (($op1 as u32) << 3)
            + (($op2 as u32) << 6)
            + (($op3 as u32) << 9)
            + (($op4 as u32) << 12)
            + (($op5 as u32) << 15)
    };
}

#[allow(unused_imports)]
pub(crate) use enc_ops;

// Buffer-writer functions (AsmJit's X86BufferWriter over asmkit's CodeBuffer)
// ---------------------------------------------------------------------------

/// Emits the mandatory prefix byte (66/F3/F2 or 9B) selected by the opcode's PP field.
pub fn emit_pp(buf: &mut CodeBuffer, opcode: Opcode) {
    let pp_index = (opcode.get() >> Opcode::PP_SHIFT) & (Opcode::PP_FPU_MASK >> Opcode::PP_SHIFT);
    if pp_index != 0 {
        buf.put1(OPCODE_PP_TABLE[pp_index as usize]);
    }
}

/// Emits the opcode's MM prefix bytes (0F/0F38/0F3A/0F01) followed by the opcode byte.
pub fn emit_mm_and_opcode(buf: &mut CodeBuffer, opcode: Opcode) {
    let mm_index = ((opcode.get() & Opcode::MM_MASK) >> Opcode::MM_SHIFT) as usize;
    let mm_code = &OPCODE_MM_TABLE[mm_index];

    if mm_code.size > 0 {
        buf.put1(mm_code.data[0]);
    }
    if mm_code.size > 1 {
        buf.put1(mm_code.data[1]);
    }
    buf.put1(opcode.get() as u8);
}

/// Emits a segment-override prefix byte if `segment_id` selects one.
pub fn emit_segment_override(buf: &mut CodeBuffer, segment_id: u32) {
    debug_assert!((segment_id as usize) < SEGMENT_PREFIX_TABLE.len());
    let prefix = SEGMENT_PREFIX_TABLE[segment_id as usize];
    if prefix != 0 {
        buf.put1(prefix);
    }
}

/// Emits the 0x67 address-size override byte if `condition` holds.
pub fn emit_address_override(buf: &mut CodeBuffer, condition: bool) {
    if condition {
        buf.put1(0x67);
    }
}

/// Emits optimized multi-byte NOPs to align the current offset to `alignment`
/// (port of AsmJit's `Assembler::align` code path with optimized align).
pub fn emit_code_align(buf: &mut CodeBuffer, alignment: u32) {
    debug_assert!(alignment.is_power_of_two());
    let mut i = (buf.cur_offset().wrapping_neg()) & (alignment - 1);
    while i > 0 {
        let n = i.min(9) as usize;
        for b in &super::encoder_tables::NOP_TABLE[n - 1][..n] {
            buf.put1(*b);
        }
        i -= n as u32;
    }
}

/// Emits a 1- or 4-byte immediate (VEX path — sizes other than 1/4 assert in debug).
pub fn emit_imm_byte_or_dword(buf: &mut CodeBuffer, imm_value: u64, imm_size: u8) {
    if imm_size == 0 {
        return;
    }
    debug_assert!(imm_size == 1 || imm_size == 4);

    let mut imm = imm_value;
    buf.put1(imm as u8);
    if imm_size == 1 {
        return;
    }
    imm >>= 8;
    buf.put1(imm as u8);
    imm >>= 8;
    buf.put1(imm as u8);
    imm >>= 8;
    buf.put1(imm as u8);
}

/// Emits an immediate of up to 8 bytes, little-endian.
pub fn emit_immediate(buf: &mut CodeBuffer, imm_value: u64, imm_size: u8) {
    let mut imm = imm_value;
    let mut imm_size = imm_size;
    if imm_size >= 4 {
        buf.put4((imm & 0xFFFF_FFFF) as u32);
        imm >>= 32;
        imm_size -= 4;
    }

    if imm_size == 0 {
        return;
    }
    buf.put1(imm as u8);
    imm >>= 8;

    imm_size -= 1;
    if imm_size == 0 {
        return;
    }
    buf.put1(imm as u8);
    imm >>= 8;

    imm_size -= 1;
    if imm_size == 0 {
        return;
    }
    buf.put1(imm as u8);
    imm >>= 8;

    imm_size -= 1;
    if imm_size == 0 {
        return;
    }
    buf.put1(imm as u8);
}

// Encoder state and emit handlers (port of AsmJit's emit-handler layer)
// ---------------------------------------------------------------------

/// Address-override info mask in 64-bit mode (AsmJit's `_address_override_mask()`;
/// 32-bit mode would use [`MEM_INFO_67H_X86`] instead).
const ADDRESS_OVERRIDE_MASK: u8 = MEM_INFO_67H_X64;

/// asmkit currently only implements the 64-bit encoder. Arms that only exist in
/// 32-bit mode are kept (structurally 1:1 with AsmJit) but stub out behind this
/// constant with a `debug_assert!` and an error.
const IS_32BIT_MODE: bool = false;

// Shifts used to construct VEX/EVEX prefixes (AsmJit's `kVSHR_*`).
const VSHR_W: u32 = Opcode::W_SHIFT - 23;
const VSHR_PP: u32 = Opcode::PP_SHIFT - 16;
const VSHR_PP_EW: u32 = Opcode::PP_SHIFT - 16;

/// Combined `InstOptions` bits handled by the AVX-512 branch of the VEX/EVEX handlers
/// (AsmJit's `kAvx512Options`).
const AVX512_OPTIONS: u32 =
    InstOptions::X86_ZMASK.bits() | InstOptions::X86_ER.bits() | InstOptions::X86_SAE.bits();

/// Encoder state threaded through encoding arms and emit handlers
/// (mirrors the locals of AsmJit's `Assembler::_emit`).
///
/// `inst_id`, `inst_info` and `common_info` extend AsmJit's local set because the
/// fixed handler signature cannot take extra parameters: the emit handlers need the
/// instruction id (LEA/abs32 and jmp/call special cases), the alt opcode (jmp/call
/// short form), and the common flags (TSIB/VSIB/VEX/EVEX/broadcast queries).
#[derive(Clone, Copy, Debug)]
pub struct X86EmitState {
    pub opcode: Opcode,
    pub options: InstOptions,
    pub isign3: u32,
    /// The r/m operand (mem) or register-as-rm; also the Label|Imm|Sym of jmp/call.
    pub rm_rel: Operand,
    /// [`MEM_INFO_TABLE`] bits for `rm_rel` when it is a memory operand.
    pub rm_info: u8,
    /// Base register id (ModRM.rm field / SIB base).
    pub rb_reg: u32,
    /// Index register id (SIB index / VEX.vvvv where the arm packs it there).
    pub rx_reg: u32,
    /// ModRM.reg: register id or /r opcode extension. The VEX/EVEX handlers expect
    /// it pre-packed with the vvvvv id ([`pack_reg_and_vvvvv`]) and mask it to 3 bits.
    pub op_reg: u32,
    /// {k} mask register (or rep-cx). Must be an id-0 operand when unused — NOT
    /// `Operand::default()`, whose id is `INVALID_ID` and would corrupt the EVEX
    /// `aaa` field (AsmJit's "none" extra reg has id 0).
    pub extra_reg: Operand,
    /// Label id when operating on a label; [`INVALID_ID`] otherwise.
    pub label_id: u32,
    /// Displacement value (raw `i32` bits) consumed by [`emit_rel`] fixups
    /// (AsmJit's `rel_offset`).
    pub rel_offset: u32,
    pub rel_size: u8,
    pub imm_value: i64,
    pub imm_size: u8,
    /// Buffer offset of the position where an address-override prefix would be/has
    /// been emitted (AsmJit's `mem_op_ao_mark` pointer, as a [`CodeOffset`]).
    pub mem_op_ao_mark: u32,
    /// Instruction id (used by the LEA abs32 and jmp/call special cases).
    pub inst_id: u32,
    /// Instruction info (used for the alt-opcode lookup in jmp/call).
    pub inst_info: InstInfo,
    /// Common instruction flags (TSIB/VSIB/VEX/EVEX/broadcast queries).
    pub common_info: CommonInfo,
}

impl Default for X86EmitState {
    fn default() -> Self {
        Self {
            opcode: Opcode::default(),
            options: InstOptions::NONE,
            isign3: 0,
            rm_rel: Operand::default(),
            rm_info: 0,
            rb_reg: 0,
            rx_reg: 0,
            op_reg: 0,
            extra_reg: *super::operands::KReg::from_id(0).as_operand(),
            label_id: INVALID_ID,
            rel_offset: 0,
            rel_size: 0,
            imm_value: 0,
            imm_size: 0,
            mem_op_ao_mark: 0,
            inst_id: 0,
            inst_info: InstInfo::default(),
            common_info: CommonInfo::default(),
        }
    }
}

/// Validates and emits a REX prefix (AsmJit's repeated `rex` block): errors out on an
/// invalid REX, clears the INVALID_REX marker bit, and emits `0x40 | rex` if nonzero.
fn emit_rex(buf: &mut CodeBuffer, rex: u32) -> Result<(), X86Error> {
    if is_rex_invalid(rex) {
        return Err(X86Error::InvalidPrefix {
            prefix: rex as u64,
            reason: "invalid REX prefix (REX bits required together with AH|BH|CH|DH)",
        });
    }
    let rex = rex & !(X86_BYTE_INVALID_REX as u32) & 0xFF;
    if rex != 0 {
        buf.put1((rex | X86_BYTE_REX as u32) as u8);
    }
    Ok(())
}

fn invalid_instruction(st: &X86EmitState, reason: &'static str) -> X86Error {
    X86Error::InvalidInstruction {
        opcode: st.opcode.get() as u64,
        reason,
    }
}

fn invalid_address(mem: &Mem, reason: &'static str) -> X86Error {
    X86Error::InvalidMemoryOperand {
        base: mem.has_base().then(|| mem.base_id()),
        index: mem.has_index().then(|| mem.index_id()),
        scale: mem.shift() as u8,
        offset: mem.offset(),
        reason,
    }
}

fn unsupported_32bit(st: &X86EmitState) -> X86Error {
    debug_assert!(false, "32-bit x86 encoding not supported");
    invalid_instruction(st, "32-bit x86 encoding not supported")
}

/// `EmitX86OpMovAbs` — movabs preamble: the address becomes an immediate of native
/// register size (8 bytes in 64-bit mode); falls through to [`emit_x86_op`].
pub fn emit_x86_op_mov_abs(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    // AsmJit: `imm_size = FastUInt8(register_size())` — native register size.
    st.imm_size = 8;
    emit_segment_override(buf, st.rm_rel.as_::<Mem>().segment_id());
    emit_x86_op(buf, st)
}

/// `EmitX86Op` — bare opcode (+ REX + immediate), no ModRM.
pub fn emit_x86_op(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);
    let rex = st.opcode.extract_rex(st.options);
    emit_rex(buf, rex)?;
    emit_mm_and_opcode(buf, st.opcode);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86OpReg` — opcode with the low 3 bits of a register id added to it (no ModRM).
pub fn emit_x86_op_reg(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);
    let rex = st.opcode.extract_rex(st.options) | (st.op_reg >> 3); // Rex.B (0x01).
    emit_rex(buf, rex)?;
    st.op_reg &= 0x7;
    st.opcode.add(st.op_reg);
    emit_mm_and_opcode(buf, st.opcode);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86OpImplicitMem` — opcode with an implicit memory operand (string ops).
pub fn emit_x86_op_implicit_mem(
    buf: &mut CodeBuffer,
    st: &mut X86EmitState,
) -> Result<(), X86Error> {
    let mem = st.rm_rel.as_::<Mem>();
    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    if mem.has_offset() || st.rm_info & MEM_INFO_INDEX != 0 {
        return Err(invalid_instruction(
            st,
            "implicit memory operand must have no offset and no index",
        ));
    }

    emit_pp(buf, st.opcode);
    let rex = st.opcode.extract_rex(st.options);
    emit_rex(buf, rex)?;

    emit_segment_override(buf, mem.segment_id());
    emit_address_override(buf, st.rm_info & ADDRESS_OVERRIDE_MASK != 0);

    emit_mm_and_opcode(buf, st.opcode);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86R` — opcode /r with a register r/m (`MOD(3, reg, rm)`).
pub fn emit_x86_r(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);

    let rex = st.opcode.extract_rex(st.options)
        | ((st.op_reg & 0x08) >> 1) // REX.R (0x04).
        | ((st.rb_reg & 0x08) >> 3); // REX.B (0x01).
    emit_rex(buf, rex)?;
    st.op_reg &= 0x07;
    st.rb_reg &= 0x07;

    emit_mm_and_opcode(buf, st.opcode);
    buf.put1(encode_mod(3, st.op_reg, st.rb_reg) as u8);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86RFromM` — opcode /r where the r/m register comes from a memory operand's
/// base register (must be a plain base with no offset and no index).
pub fn emit_x86_r_from_m(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    let mem = st.rm_rel.as_::<Mem>();
    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    if mem.has_offset() || st.rm_info & MEM_INFO_INDEX != 0 {
        return Err(invalid_instruction(
            st,
            "expected a memory base register without offset and index",
        ));
    }

    emit_pp(buf, st.opcode);

    let rex = st.opcode.extract_rex(st.options)
        | ((st.op_reg & 0x08) >> 1) // REX.R (0x04).
        | (st.rb_reg >> 3); // REX.B (0x01) — unmasked, as in AsmJit.
    emit_rex(buf, rex)?;
    st.op_reg &= 0x07;
    st.rb_reg &= 0x07;

    emit_segment_override(buf, mem.segment_id());
    emit_address_override(buf, st.rm_info & ADDRESS_OVERRIDE_MASK != 0);

    emit_mm_and_opcode(buf, st.opcode);
    buf.put1(encode_mod(3, st.op_reg, st.rb_reg) as u8);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86M` — opcode /r with a memory r/m; tails into [`emit_mod_sib`].
pub fn emit_x86_m(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    debug_assert!(st.opcode.get() & Opcode::CDSHL_MASK == 0);
    let mem = st.rm_rel.as_::<Mem>();

    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    emit_segment_override(buf, mem.segment_id());

    st.mem_op_ao_mark = buf.cur_offset();
    emit_address_override(buf, st.rm_info & ADDRESS_OVERRIDE_MASK != 0);

    emit_pp(buf, st.opcode);

    st.rb_reg = mem.base_id();
    st.rx_reg = mem.index_id();

    let mut rex = (st.rb_reg >> 3) & 0x01; // REX.B (0x01).
    rex |= (st.rx_reg >> 2) & 0x02; // REX.X (0x02).
    rex |= (st.op_reg >> 1) & 0x04; // REX.R (0x04).
    rex &= st.rm_info as u32;
    rex |= st.opcode.extract_rex(st.options);
    emit_rex(buf, rex)?;
    st.op_reg &= 0x07;

    emit_mm_and_opcode(buf, st.opcode);
    emit_mod_sib(buf, st)
}

/// `EmitModSib` — ModRM + SIB + displacement for the memory operand in `st.rm_rel`.
pub fn emit_mod_sib(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    let mem = st.rm_rel.as_::<Mem>();

    if st.rm_info & (MEM_INFO_INDEX | MEM_INFO_67H_X86) == 0 {
        if st.rm_info & MEM_INFO_BASE_GP != 0 {
            // ==========|> [BASE + DISP8|DISP32].
            let rb = st.rb_reg & 0x7;
            let rel_offset = mem.offset_lo32();

            let mut mod_ = encode_mod(0, st.op_reg, rb);
            let force_sib = st.common_info.has_flag(InstFlags::TSIB);

            if rb == Gp::SP || force_sib {
                // TSIB or [XSP|R12].
                mod_ = (mod_ & 0xF8) | 0x04;
                if rb != Gp::BP && rel_offset == 0 {
                    buf.put1(mod_ as u8);
                    buf.put1(encode_sib(0, 4, rb) as u8);
                } else {
                    // TSIB or [XSP|R12 + DISP8|DISP32].
                    let cd_shift = (st.opcode.get() & Opcode::CDSHL_MASK) >> Opcode::CDSHL_SHIFT;
                    let cd_offset = rel_offset >> cd_shift;
                    if i8::try_from(cd_offset).is_ok()
                        && rel_offset == ((cd_offset as u32) << cd_shift) as i32
                    {
                        buf.put1((mod_ + 0x40) as u8); // <- MOD(1, op_reg, rb).
                        buf.put1(encode_sib(0, 4, rb) as u8);
                        buf.put1(cd_offset as u8);
                    } else {
                        buf.put1((mod_ + 0x80) as u8); // <- MOD(2, op_reg, rb).
                        buf.put1(encode_sib(0, 4, rb) as u8);
                        buf.put4(rel_offset as u32);
                    }
                }
            } else if rb != Gp::BP && rel_offset == 0 {
                // [BASE].
                buf.put1(mod_ as u8);
            } else {
                // [BASE + DISP8|DISP32].
                let cd_shift = (st.opcode.get() & Opcode::CDSHL_MASK) >> Opcode::CDSHL_SHIFT;
                let cd_offset = rel_offset >> cd_shift;
                if i8::try_from(cd_offset).is_ok()
                    && rel_offset == ((cd_offset as u32) << cd_shift) as i32
                {
                    buf.put1((mod_ + 0x40) as u8);
                    buf.put1(cd_offset as u8);
                } else {
                    buf.put1((mod_ + 0x80) as u8);
                    buf.put4(rel_offset as u32);
                }
            }
        } else if st.rm_info & (MEM_INFO_BASE_LABEL | MEM_INFO_BASE_RIP) == 0 {
            // ==========|> [ABSOLUTE | DISP32].
            //
            // asmkit extension: a Sym base is encoded as rip-relative disp32 with a
            // relocation (AsmJit has no Sym operands; mirrors the old asmkit encoder).
            if mem.has_base_sym() {
                buf.put1(encode_mod(0, st.op_reg, 5) as u8);
                let disp_offset = buf.cur_offset();
                buf.put4(mem.offset_lo32() as u32);
                let sym = Sym::from_id(mem.base_id());
                let kind = if buf.symbol_distance(sym) == RelocDistance::Near {
                    Reloc::X86PCRel4
                } else {
                    Reloc::X86GOTPCRel4
                };
                buf.add_reloc_at_offset(disp_offset, kind, RelocTarget::Sym(sym), -4);
                emit_immediate(buf, st.imm_value as u64, st.imm_size);
                return Ok(());
            }

            let mut addr_type = mem.addr_type();
            let rel_offset = mem.offset_lo32();

            if IS_32BIT_MODE {
                // Explicit relative addressing doesn't work in 32-bit mode; the whole
                // 32-bit arm (raw disp32 without REX handling) is stubbed.
                return Err(unsupported_32bit(st));
            }

            let is_offset_int32 = mem.offset_hi32() == (rel_offset >> 31);
            let is_offset_uint32 = mem.offset_hi32() == 0;

            // asmkit never has a base address at emit time, so this is always AsmJit's
            // "not an absolute location" guess: prefer absolute addressing with an FS|GS
            // segment override or for LEA with a 32-bit immediate, relative otherwise.
            if addr_type == AddrType::Default {
                let has_fs_gs = mem.segment_id() >= SReg::FS;
                let is_lea_32 =
                    st.inst_id == InstId::Lea as u32 && (is_offset_int32 || is_offset_uint32);
                addr_type = if has_fs_gs || is_lea_32 {
                    AddrType::Abs
                } else {
                    AddrType::Rel
                };
            }

            if addr_type == AddrType::Rel {
                // AsmJit would create a kAbsToRel relocation against the raw address;
                // asmkit cannot relocate raw addresses (only Sym/Label bases), so an
                // explicitly relative raw address is unencodable, and a guessed-relative
                // one falls back to the absolute form below (old-encoder behavior).
                if mem.is_rel() {
                    return Err(X86Error::InvalidRIPRelative {
                        offset: mem.offset(),
                        reason: "relative raw address requires a Sym or Label base",
                    });
                }
            }

            // Handle an unsigned 32-bit address that doesn't work with sign extension
            // (see the long comment in AsmJit): patch in an address-size override,
            // or remove REX.W for LEA, unless the override is already present.
            if !is_offset_int32 {
                // 64-bit absolute address is unencodable.
                if !is_offset_uint32 {
                    return Err(invalid_address(
                        &mem,
                        "64-bit absolute address is not encodable",
                    ));
                }

                if buf.byte_at(st.mem_op_ao_mark) != 0x67 {
                    if st.inst_id == InstId::Lea as u32 {
                        // LEA: remove REX.W, if present (lea uses no PP prefix, so a REX
                        // prefix would be exactly at `mem_op_ao_mark`).
                        let mut rex = buf.byte_at(st.mem_op_ao_mark) as u32;
                        if rex & X86_BYTE_REX as u32 != 0 {
                            rex &= !(X86_BYTE_REX_W as u32) & 0xFF;
                            buf.set_byte_at(st.mem_op_ao_mark, rex as u8);

                            // Remove the REX prefix completely if it was not forced.
                            if rex == X86_BYTE_REX as u32
                                && !st.options.contains(InstOptions::X86_REX)
                            {
                                buf.remove_at(st.mem_op_ao_mark);
                            }
                        }
                    } else {
                        // Any other instruction: insert the address-size override prefix.
                        buf.insert_at(st.mem_op_ao_mark, 0x67);
                    }
                }
            }

            // Emit 32-bit absolute address.
            buf.put1(encode_mod(0, st.op_reg, 4) as u8);
            buf.put1(encode_sib(0, 4, 5) as u8);
            buf.put4(rel_offset as u32);
        } else {
            // ==========|> [LABEL|RIP + DISP32]
            buf.put1(encode_mod(0, st.op_reg, 5) as u8);

            if IS_32BIT_MODE {
                // 32-bit label/rip arm (`EmitModSib_LabelRip_X86`, RelToAbs relocs).
                return Err(unsupported_32bit(st));
            }

            let rel_offset = mem.offset_lo32();
            if st.rm_info & MEM_INFO_BASE_LABEL != 0 {
                // [RIP] with a label base.
                let label_id = mem.base_id();
                if label_id >= buf.label_count() {
                    return Err(X86Error::InvalidLabel {
                        label_id,
                        reason: "invalid label id",
                    });
                }
                let label = Label::from_id(label_id);
                let rel = rel_offset.wrapping_sub(4 + st.imm_size as i32);

                if buf.is_bound(label) {
                    // Label bound to the current section.
                    let rel =
                        rel.wrapping_add(
                            buf.label_offset(label).wrapping_sub(buf.cur_offset()) as i32
                        );
                    buf.put4(rel as u32);
                } else {
                    // Non-bound label.
                    st.label_id = label_id;
                    st.rel_offset = rel as u32;
                    st.rel_size = 4;
                    return emit_rel(buf, st);
                }
            } else {
                // [RIP + disp32].
                buf.put4(rel_offset as u32);
            }
        }
    } else if st.rm_info & MEM_INFO_67H_X86 == 0 {
        // ESP|RSP can't be used as INDEX in pure SIB mode, however, VSIB mode allows
        // XMM4|YMM4|ZMM4 (that's why the check is before the VSIB handler).
        if st.rx_reg == Gp::SP {
            return Err(X86Error::InvalidSIB {
                sib: 0,
                reason: "ESP/RSP cannot be used as an index register",
            });
        }
        return emit_mod_v_sib(buf, st);
    } else {
        // 16-bit address mode (32-bit mode with 67 override prefix).
        //
        // NOTE: 16-bit addresses don't use SIB byte and their encoding differs. A
        // table-based approach computes the MOD byte; not all BASE [+ INDEX]
        // combinations are supported in 16-bit mode, so this may fail.
        let rel_offset = (mem.offset_lo32() << 16) >> 16;
        const BASE_GP_IDX: u8 = MEM_INFO_BASE_GP | MEM_INFO_INDEX;

        if st.rm_info & BASE_GP_IDX != 0 {
            // ==========|> [BASE + INDEX + DISP16].
            let mut rb = st.rb_reg & 0x7;
            let rx = st.rx_reg & 0x7;

            let mut mod_;
            if st.rm_info & BASE_GP_IDX == BASE_GP_IDX {
                if mem.shift() != 0 {
                    return Err(invalid_address(
                        &mem,
                        "16-bit addressing cannot use a scaled index",
                    ));
                }
                mod_ = MOD16_BASE_INDEX_TABLE[((rb << 3) + rx) as usize] as u32;
            } else {
                if st.rm_info & MEM_INFO_INDEX != 0 {
                    rb = rx;
                }
                mod_ = MOD16_BASE_TABLE[rb as usize] as u32;
            }

            if mod_ == 0xFF {
                return Err(invalid_address(
                    &mem,
                    "invalid 16-bit address register combination",
                ));
            }

            mod_ += st.op_reg << 3;
            if rel_offset == 0 && mod_ != 0x06 {
                buf.put1(mod_ as u8);
            } else if i8::try_from(rel_offset).is_ok() {
                buf.put1((mod_ + 0x40) as u8);
                buf.put1(rel_offset as u8);
            } else {
                buf.put1((mod_ + 0x80) as u8);
                buf.put2(rel_offset as u16);
            }
        } else {
            // Not supported in 16-bit addresses.
            if st.rm_info & (MEM_INFO_BASE_RIP | MEM_INFO_BASE_LABEL) != 0 {
                return Err(invalid_address(
                    &mem,
                    "16-bit addressing cannot be rip or label based",
                ));
            }

            // ==========|> [DISP16].
            //
            // NOTE: `op_reg | 0x06` mirrors AsmJit verbatim (the reg field is not
            // shifted into bits 3-5 there — suspect but kept 1:1).
            buf.put1((st.op_reg | 0x06) as u8);
            buf.put2(rel_offset as u16);
        }
    }

    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitModVSib` — SIB (and VSIB) forms with an index register.
pub fn emit_mod_v_sib(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    let mem = st.rm_rel.as_::<Mem>();
    let rx = st.rx_reg & 0x7;

    if st.rm_info & MEM_INFO_BASE_GP != 0 {
        // ==========|> [BASE + INDEX + DISP8|DISP32].
        let rb = st.rb_reg & 0x7;
        let rel_offset = mem.offset_lo32();

        let mod_ = encode_mod(0, st.op_reg, 4);
        let sib = encode_sib(mem.shift(), rx, rb);

        if rel_offset == 0 && rb != Gp::BP {
            // [BASE + INDEX << SHIFT].
            buf.put1(mod_ as u8);
            buf.put1(sib as u8);
        } else {
            let cd_shift = (st.opcode.get() & Opcode::CDSHL_MASK) >> Opcode::CDSHL_SHIFT;
            let cd_offset = rel_offset >> cd_shift;
            if i8::try_from(cd_offset).is_ok()
                && rel_offset == ((cd_offset as u32) << cd_shift) as i32
            {
                // [BASE + INDEX << SHIFT + DISP8].
                buf.put1((mod_ + 0x40) as u8); // <- MOD(1, op_reg, 4).
                buf.put1(sib as u8);
                buf.put1(cd_offset as u8);
            } else {
                // [BASE + INDEX << SHIFT + DISP32].
                buf.put1((mod_ + 0x80) as u8); // <- MOD(2, op_reg, 4).
                buf.put1(sib as u8);
                buf.put4(rel_offset as u32);
            }
        }
    } else if st.rm_info & (MEM_INFO_BASE_LABEL | MEM_INFO_BASE_RIP) == 0 {
        // ==========|> [INDEX + DISP32].
        buf.put1(encode_mod(0, st.op_reg, 4) as u8);
        buf.put1(encode_sib(mem.shift(), rx, 5) as u8);
        buf.put4(mem.offset_lo32() as u32);
    } else {
        // ==========|> [LABEL|RIP + INDEX + DISP32].
        if IS_32BIT_MODE {
            // 32-bit: shares the label/rip arm of `emit_mod_sib`.
            return Err(unsupported_32bit(st));
        }
        // This also covers VSIB+RIP, which is not allowed in 64-bit mode.
        return Err(invalid_address(
            &mem,
            "rip or label base cannot be used with an index register in 64-bit mode",
        ));
    }

    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitFpuOp` — FPU opcode (two opcode bytes, plus optional 9B prefix via PP).
pub fn emit_fpu_op(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);

    // FPU instructions consist of two opcodes.
    buf.put1((st.opcode.get() >> Opcode::FPU_2B_SHIFT) as u8);
    buf.put1(st.opcode.get() as u8);
    Ok(())
}

/// `EmitVexOp` — VEX opcode with no ModRM (only `vzeroall`/`vzeroupper`).
pub fn emit_vex_op(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    // These don't use immediate.
    debug_assert!(st.imm_size == 0);
    // Both instructions can be encoded by VEX2; VEX3 is only used on request, and
    // they don't define 'W' to be '1' so only the 'mmmmm' field decides.
    debug_assert!(st.opcode.get() & Opcode::W == 0);

    let opcode = st.opcode.get();
    let mut x = ((opcode & Opcode::MM_MASK) >> Opcode::MM_SHIFT)
        | ((opcode & Opcode::LL_MASK) >> (Opcode::LL_SHIFT - 10))
        | ((opcode & Opcode::PP_VEX_MASK) >> (Opcode::PP_SHIFT - 8));

    if st.options.contains(InstOptions::X86_VEX3) {
        x = (x & 0xFFFF) << 8; // [00000000|00000Lpp|000mmmmm|00000000].
        x ^= (X86_BYTE_VEX3 as u32) // [........|00000Lpp|000mmmmm|__VEX3__].
            | (0x07 << 13) // [........|00000Lpp|111mmmmm|__VEX3__].
            | (0x0F << 19) // [........|01111Lpp|111mmmmm|__VEX3__].
            | (opcode << 24); // [_OPCODE_|01111Lpp|111mmmmm|__VEX3__].
        buf.put4(x);
    } else {
        x = ((x >> 8) ^ x) ^ 0xF9;
        buf.put1(X86_BYTE_VEX2);
        buf.put1(x as u8);
        buf.put1(opcode as u8);
    }
    Ok(())
}

/// `EmitVexEvexR` — VEX|EVEX prefix + opcode /r with a register r/m.
pub fn emit_vex_evex_r(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    let opcode = st.opcode.get();

    // Construct `x` - a complete EVEX|VEX prefix.
    let mut x = ((st.op_reg << 4) & 0xF980) // [........|........|Vvvvv..R|R.......].
        | ((st.rb_reg << 2) & 0x0060) // [........|........|........|.BB.....].
        | st.opcode.extract_ll_mmmmm(st.options) // [........|.LL.....|Vvvvv..R|RBBmmmmm].
        | (st.extra_reg.id() << 16); // [........|.LL..aaa|Vvvvv..R|RBBmmmmm].
    let op_reg = st.op_reg & 0x7;

    // Handle AVX512 options by a single branch.
    if st.options.bits() & AVX512_OPTIONS != 0 {
        const BCST_MASK: u32 = 0x1 << 20;
        const LL_MASK_10: u32 = 0x2 << 21;
        const LL_MASK_11: u32 = 0x3 << 21;

        // {rz-sae} is encoded as {11}, so it must match the mask.
        const _: () = assert!(InstOptions::X86_RZ_SAE.bits() == LL_MASK_11);

        x |= st.options.bits() & InstOptions::X86_ZMASK.bits(); // [........|zLLb.aaa|Vvvvv..R|RBBmmmmm].

        // Support embedded-rounding {er} and suppress-all-exceptions {sae}.
        if st
            .options
            .intersects(InstOptions::X86_ER | InstOptions::X86_SAE)
        {
            // Embedded rounding is only encodable if the instruction is either scalar
            // or a 512-bit operation as the {er} rounding predicate collides with the
            // LL part of the instruction.
            if x & LL_MASK_11 != LL_MASK_10 {
                // LL is not 10, thus the instruction must be scalar. Scalar instructions
                // don't support broadcast, so if this instruction supports it neither
                // {er} nor {sae} would be encodable.
                if st
                    .common_info
                    .has_avx512_flag(Avx512Flags::B16 | Avx512Flags::B32 | Avx512Flags::B64)
                {
                    return Err(X86Error::InvalidRoundingControl {
                        rc: st.options.bits() as u64,
                        reason: "{er}/{sae} is not encodable for this instruction",
                    });
                }
            }

            if st.options.contains(InstOptions::X86_ER) {
                if !st.common_info.has_avx512_flag(Avx512Flags::ER) {
                    return Err(X86Error::InvalidRoundingControl {
                        rc: st.options.bits() as u64,
                        reason: "instruction does not support embedded rounding {er}",
                    });
                }
                x &= !LL_MASK_11; // [........|.00..aaa|Vvvvv..R|RBBmmmmm].
                x |= BCST_MASK | (st.options.bits() & LL_MASK_11); // [........|.LLb.aaa|Vvvvv..R|RBBmmmmm].
            } else {
                if !st.common_info.has_avx512_flag(Avx512Flags::SAE) {
                    return Err(X86Error::InvalidRoundingControl {
                        rc: st.options.bits() as u64,
                        reason: "instruction does not support suppress-all-exceptions {sae}",
                    });
                }
                x &= !LL_MASK_11; // [........|.00..aaa|Vvvvv..R|RBBmmmmm].
                x |= BCST_MASK; // [........|.00b.aaa|Vvvvv..R|RBBmmmmm].
            }
        }
    }

    // These bits would force EVEX prefix.
    const EVEX_FORCE: u32 = 0x00000010; // [........|........|........|...x....].
    const EVEX_BITS: u32 = 0x00D78150; // [........|xx.x.xxx|x......x|.x.x....].

    // Force EVEX prefix even in case the instruction has VEX encoding, because EVEX
    // encoding is preferred (AVX_VNNI added after AVX512_VNNI).
    if st.common_info.has_flag(InstFlags::PREFER_EVEX)
        && x & EVEX_BITS == 0
        && !st
            .options
            .intersects(InstOptions::X86_VEX | InstOptions::X86_VEX3)
    {
        x |= EVEX_FORCE;
    }

    // Check if EVEX is required by checking bits in `x`.
    if x & EVEX_BITS != 0 {
        let y = ((x << 4) & 0x00080000) // [........|...bV...|........|........].
            | ((x >> 4) & 0x00000010); // [........|...bV...|........|...R....].
        x = (x & 0x00FF78EF) | y; // [........|zLLbVaaa|0vvvv000|RBBRmmmm].
        x <<= 8; // [zLLbVaaa|0vvvv000|RBBRmmmm|00000000].
        x |= (opcode >> VSHR_W) & 0x00800000; // [zLLbVaaa|Wvvvv000|RBBRmmmm|00000000].
        x |= (opcode >> VSHR_PP_EW) & 0x00830000; // [zLLbVaaa|Wvvvv0pp|RBBRmmmm|00000000] (PP and EVEX.W).
        x ^= 0x087CF000 | X86_BYTE_EVEX as u32; // [zLLbVaaa|Wvvvv1pp|RBBRmmmm|01100010].

        buf.put4(x);
        buf.put1(opcode as u8);
        buf.put1(encode_mod(3, op_reg, st.rb_reg & 0x7) as u8);
        emit_imm_byte_or_dword(buf, st.imm_value as u64, st.imm_size);
        return Ok(());
    }

    // Not EVEX, prepare `x` for VEX2 or VEX3:   x = [........|00L00000|0vvvv000|R0Bmmmmm].
    x |= ((opcode >> (VSHR_W + 8)) & 0x8000) // [00000000|00L00000|Wvvvv000|R0Bmmmmm].
        | ((opcode >> (VSHR_PP + 8)) & 0x0300) // [00000000|00L00000|0vvvv0pp|R0Bmmmmm].
        | ((x >> 11) & 0x0400); // [00000000|00L00000|WvvvvLpp|R0Bmmmmm].
    x |= force_evex3_mask_in_last_bit(st.options); // [x0000000|00L00000|WvvvvLpp|R0Bmmmmm].

    // Check if VEX3 is required / forced:         [x.......|........|x.......|..xxxxx.].
    if x & 0x8000803E != 0 {
        let xor_mask = VEX_PREFIX_TABLE[(x & 0xF) as usize] | (opcode << 24);

        // Clear all high bits.
        x = (x & 0xFFFF) << 8; // [00000000|WvvvvLpp|R0Bmmmmm|00000000].
        x ^= xor_mask; // [_OPCODE_|WvvvvLpp|R1Bmmmmm|VEX3|XOP].
        buf.put4(x);
        buf.put1(encode_mod(3, op_reg, st.rb_reg & 0x7) as u8);
        emit_imm_byte_or_dword(buf, st.imm_value as u64, st.imm_size);
        return Ok(());
    }

    // 'mmmmm' must be '00001'.
    debug_assert!(x & 0x1F == 0x01);

    x = ((x >> 8) ^ x) ^ 0xF9;
    buf.put1(X86_BYTE_VEX2);
    buf.put1(x as u8);
    buf.put1(opcode as u8);
    buf.put1(encode_mod(3, op_reg, st.rb_reg & 0x7) as u8);
    emit_imm_byte_or_dword(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitVexEvexM` — VEX|EVEX prefix + opcode /r with a memory r/m; tails into
/// [`emit_mod_sib`], or [`emit_mod_v_sib`] for VSIB instructions.
pub fn emit_vex_evex_m(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    let mem = st.rm_rel.as_::<Mem>();

    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    emit_segment_override(buf, mem.segment_id());

    st.mem_op_ao_mark = buf.cur_offset();
    emit_address_override(buf, st.rm_info & ADDRESS_OVERRIDE_MASK != 0);

    st.rb_reg = if mem.has_base_reg() { mem.base_id() } else { 0 };
    st.rx_reg = if mem.has_index_reg() {
        mem.index_id()
    } else {
        0
    };

    let mut opcode = st.opcode.get();
    let broadcast_bit = mem.has_broadcast() as u32;

    // Construct `x` - a complete EVEX|VEX prefix.
    let mut x = ((st.op_reg << 4) & 0x0000F980) // [........|........|Vvvvv..R|R.......].
        | ((st.rx_reg << 3) & 0x00000040) // [........|........|........|.X......].
        | ((st.rx_reg << 15) & 0x00080000) // [........|....X...|........|........].
        | ((st.rb_reg << 2) & 0x00000020) // [........|........|........|..B.....].
        | st.opcode.extract_ll_mmmmm(st.options) // [........|.LL.X...|Vvvvv..R|RXBmmmmm].
        | (st.extra_reg.id() << 16) // [........|.LL.Xaaa|Vvvvv..R|RXBmmmmm].
        | (broadcast_bit << 20); // [........|.LLbXaaa|Vvvvv..R|RXBmmmmm].
    st.op_reg &= 0x07;

    // Mark invalid VEX (force EVEX) case:         [@.......|.LLbXaaa|Vvvvv..R|RXBmmmmm].
    x |= (!st.common_info.flags & InstFlags::VEX.bits())
        << (31 - InstFlags::VEX.bits().trailing_zeros());

    // Handle AVX512 options by a single branch.
    if st.options.bits() & AVX512_OPTIONS != 0 {
        // {er} and {sae} are both invalid if a memory operand is used.
        if st
            .options
            .intersects(InstOptions::X86_ER | InstOptions::X86_SAE)
        {
            return Err(X86Error::InvalidRoundingControl {
                rc: st.options.bits() as u64,
                reason: "{er}/{sae} is not encodable with a memory operand",
            });
        }

        x |= st.options.bits() & InstOptions::X86_ZMASK.bits(); // [@.......|zLLbXaaa|Vvvvv..R|RXBmmmmm].
    }

    // If these bits are used then EVEX prefix is required.
    const EVEX_FORCE: u32 = 0x00000010; // [........|........|........|...x....].
    const EVEX_BITS: u32 = 0x80DF8110; // [@.......|xx.xxxxx|x......x|...x....].

    // Force EVEX prefix even in case the instruction has VEX encoding, because EVEX
    // encoding is preferred (AVX_VNNI added after AVX512_VNNI).
    if st.common_info.has_flag(InstFlags::PREFER_EVEX)
        && x & EVEX_BITS == 0
        && !st
            .options
            .intersects(InstOptions::X86_VEX | InstOptions::X86_VEX3)
    {
        x |= EVEX_FORCE;
    }

    // Check if EVEX is required by checking bits in `x`.
    if x & EVEX_BITS != 0 {
        let y = ((x << 4) & 0x00080000) // [@.......|....V...|........|........].
            | ((x >> 4) & 0x00000010); // [@.......|....V...|........|...R....].
        x = (x & 0x00FF78EF) | y; // [........|zLLbVaaa|0vvvv000|RXBRmmmm].
        x <<= 8; // [zLLbVaaa|0vvvv000|RBBRmmmm|00000000].
        x |= (opcode >> VSHR_W) & 0x00800000; // [zLLbVaaa|Wvvvv000|RBBRmmmm|00000000].
        x |= (opcode >> VSHR_PP_EW) & 0x00830000; // [zLLbVaaa|Wvvvv0pp|RBBRmmmm|00000000] (PP and EVEX.W).
        x ^= 0x087CF000 | X86_BYTE_EVEX as u32; // [zLLbVaaa|Wvvvv1pp|RBBRmmmm|01100010].

        if x & 0x10000000 != 0 {
            // Broadcast support.
            //
            // 1. Verify the LL field is correct as broadcast changes the "size" of the
            //    source operand.
            // 2. Change the compressed displacement scale to x2|x4|x8 depending on the
            //    broadcast unit/element size.
            let avx512_flags = st.common_info.avx512_flags;
            let broadcast_unit_size = (avx512_flags
                & (Avx512Flags::B16 | Avx512Flags::B32 | Avx512Flags::B64).bits())
                >> (Avx512Flags::B16.bits().trailing_zeros() - 1);
            let broadcast_vector_size = broadcast_unit_size << (mem.get_broadcast() as u32);

            if broadcast_unit_size == 0 {
                return Err(X86Error::InvalidBroadcast {
                    reason: "instruction does not support broadcast",
                });
            }

            // LL was already shifted 8 bits right.
            const LL_SHIFT_OUT: u32 = 21 + 8;

            let current_ll = x & (0x3 << LL_SHIFT_OUT);
            let broadcast_ll = (broadcast_vector_size.trailing_zeros().max(4) - 4) << LL_SHIFT_OUT;

            if broadcast_ll > (2 << LL_SHIFT_OUT) {
                return Err(X86Error::InvalidBroadcast {
                    reason: "broadcast size is invalid for this instruction",
                });
            }

            let new_ll = current_ll.max(broadcast_ll);
            x = (x & !(0x3 << LL_SHIFT_OUT)) | new_ll;

            opcode &= !Opcode::CDSHL_MASK;
            opcode |= broadcast_unit_size.trailing_zeros() << Opcode::CDSHL_SHIFT;
        } else {
            // Add the compressed displacement 'SHF' to the opcode based on 'TTWLL'.
            // The index to `CDISP8_SHL_TABLE` is composed as `CDTT[4:3] | W[2] | LL[1:0]`.
            let tt_w_ll = ((opcode >> (Opcode::CDTT_SHIFT - 3)) & 0x18)
                | ((opcode >> (Opcode::W_SHIFT - 2)) & 0x04)
                | ((x >> 29) & 0x3);
            opcode = opcode.wrapping_add(CDISP8_SHL_TABLE[tt_w_ll as usize]);
        }

        buf.put4(x);
        buf.put1(opcode as u8);
    } else {
        // Not EVEX, prepare `x` for VEX2 or VEX3: x = [........|00L00000|0vvvv000|RXBmmmmm].
        x |= ((opcode >> (VSHR_W + 8)) & 0x8000) // [00000000|00L00000|Wvvvv000|RXBmmmmm].
            | ((opcode >> (VSHR_PP + 8)) & 0x0300) // [00000000|00L00000|0vvvv0pp|RXBmmmmm].
            | ((x >> 11) & 0x0400); // [00000000|00L00000|WvvvvLpp|RXBmmmmm].
        x |= force_evex3_mask_in_last_bit(st.options); // [x0000000|00L00000|WvvvvLpp|RXBmmmmm].

        // Clear a possible CDisp specified by EVEX.
        opcode &= !Opcode::CDSHL_MASK;

        // Check if VEX3 is required / forced:       [x.......|........|x.......|.xxxxxx.].
        if x & 0x8000807E != 0 {
            let xor_mask = VEX_PREFIX_TABLE[(x & 0xF) as usize] | (opcode << 24);

            // Clear all high bits.
            x = (x & 0xFFFF) << 8; // [00000000|WvvvvLpp|RXBmmmmm|00000000].
            x ^= xor_mask; // [_OPCODE_|WvvvvLpp|RXBmmmmm|VEX3_XOP].
            buf.put4(x);
        } else {
            // 'mmmmm' must be '00001'.
            debug_assert!(x & 0x1F == 0x01);

            x = ((x >> 8) ^ x) ^ 0xF9;
            buf.put1(X86_BYTE_VEX2);
            buf.put1(x as u8);
            buf.put1(opcode as u8);
        }
    }

    st.opcode = Opcode(opcode);

    // MOD|SIB address.
    if !st.common_info.has_flag(InstFlags::VSIB) {
        return emit_mod_sib(buf, st);
    }

    // MOD|VSIB address without INDEX is invalid.
    if st.rm_info & MEM_INFO_INDEX != 0 {
        return emit_mod_v_sib(buf, st);
    }
    Err(invalid_instruction(
        st,
        "VSIB instruction requires a vector index register",
    ))
}

/// `EmitJmpCall` — jmp/jcc/call with a Label, Imm, or Sym target.
///
/// asmkit deviations from AsmJit, forced by asmkit's fixup model (a rel32 site cannot
/// be shrunk to rel8 after the fact) and by the lack of a base address:
///
/// - Unbound labels always use the long (rel32) form; AsmJit's optimistic rel8
///   emission is not supported. Requesting `SHORT_FORM` for an unbound label, or
///   targeting an unbound label from a rel8-only instruction (jecxz/loop), is an
///   `InvalidDisplacement` error — same as the old asmkit encoder.
/// - The short form is only used for a bound label when `SHORT_FORM` was explicitly
///   requested; AsmJit also auto-selects it unless `LONG_FORM`.
/// - A plain immediate target is emitted as a raw displacement (old-encoder
///   semantics); use a Sym operand for targets resolved at load time.
/// - A Sym target (asmkit extension) always uses the long form plus a relocation —
///   this maps AsmJit's `kAbsToRel` onto asmkit's Sym relocs.
pub fn emit_jmp_call(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    // Emit REX prefix if asked for (64-bit only).
    let rex = st.opcode.extract_rex(st.options);
    emit_rex(buf, rex)?;

    let ip = buf.cur_offset() as u64;
    let opcode8 = ALT_OPCODE_TABLE[st.inst_info.alt_opcode_index as usize];

    // Jcc instructions with 32-bit displacement use the 0x0F prefix, other
    // instructions don't. No other prefixes are used by X86.
    debug_assert!(opcode8 & Opcode::MM_MASK == 0);
    debug_assert!(
        st.opcode.get() & Opcode::MM_MASK == 0
            || st.opcode.get() & Opcode::MM_MASK == Opcode::MM_0F
    );

    // inst8_size  = 1 + 1:          OPCODE + REL8 .
    // inst32_size = 1 + 4: [PREFIX] OPCODE + REL32.
    // Only one of the two adjustments should apply at the same time.
    let inst32_size =
        5 + (st.op_reg != 0) as u32 + ((st.opcode.get() & Opcode::MM_MASK) == Opcode::MM_0F) as u32;

    if st.rm_rel.is_label() {
        let label_id = st.label_id;
        if label_id >= buf.label_count() {
            return Err(X86Error::InvalidLabel {
                label_id,
                reason: "invalid label id",
            });
        }
        let label = Label::from_id(label_id);

        if buf.is_bound(label) {
            // Label bound to the current section.
            let rel32 = (buf.label_offset(label) as u64)
                .wrapping_sub(ip)
                .wrapping_sub(inst32_size as u64) as u32;
            return emit_jmp_call_rel(buf, st, rel32, opcode8);
        }

        // Non-bound label — always the rel32 long form (asmkit deviation, see above).
        if st.opcode.get() == 0 || st.options.contains(InstOptions::SHORT_FORM) {
            return Err(X86Error::InvalidDisplacement {
                value: 0,
                size: 1,
                reason: "unbound label requires the rel32 form (rel8 fixups are not supported)",
            });
        }

        if st.opcode.get() & Opcode::MM_MASK != 0 {
            buf.put1(0x0F); // Emit 0F prefix.
        }
        buf.put1(st.opcode.get() as u8); // Emit opcode.
        if st.op_reg != 0 {
            buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
        }

        // Record DISP32 (non-bound label).
        st.rel_offset = (-4i32) as u32;
        st.rel_size = 4;
        return emit_rel(buf, st);
    }

    if st.rm_rel.is_imm() {
        // asmkit has no base address: a plain immediate is a raw displacement (old
        // encoder semantics), encoded like a bound label.
        return emit_jmp_call_rel(buf, st, st.imm_value as u32, opcode8);
    }

    if st.rm_rel.is_sym() {
        // asmkit extension: jump/call to a symbol — long form + relocation.
        if st.opcode.get() == 0 {
            return Err(X86Error::InvalidDisplacement {
                value: 0,
                size: 1,
                reason: "symbol target requires the rel32 form",
            });
        }
        let sym = Sym::from_id(st.rm_rel.id());

        if st.opcode.get() & Opcode::MM_MASK != 0 {
            buf.put1(0x0F); // Emit 0F prefix.
        }
        buf.put1(st.opcode.get() as u8); // Emit opcode.
        if st.op_reg != 0 {
            buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
        }

        let disp_offset = buf.cur_offset();
        buf.put4(0); // Emit DISP32 (patched by the relocation).
        let kind = if buf.symbol_distance(sym) == RelocDistance::Far {
            Reloc::X86GOTPCRel4
        } else if st.inst_id == InstId::Call as u32 {
            Reloc::X86CallPCRel4
        } else {
            Reloc::X86PCRel4
        };
        buf.add_reloc_at_offset(disp_offset, kind, RelocTarget::Sym(sym), -4);
        return Ok(());
    }

    // Not Label|Imm|Sym -> Invalid.
    Err(invalid_instruction(
        st,
        "jmp/call target must be a label, immediate, or symbol",
    ))
}

/// `EmitJmpCallRel` — jmp/jcc/call with the relative displacement known at assembly
/// time. The short (rel8) form is only used when explicitly requested with
/// `SHORT_FORM` (asmkit deviation: AsmJit also auto-selects it unless `LONG_FORM`).
pub fn emit_jmp_call_rel(
    buf: &mut CodeBuffer,
    st: &mut X86EmitState,
    rel32: u32,
    opcode8: u32,
) -> Result<(), X86Error> {
    // Recomputed from the opcode (same values `emit_jmp_call` computes).
    let inst8_size = 2;
    let inst32_size =
        5 + (st.op_reg != 0) as u32 + ((st.opcode.get() & Opcode::MM_MASK) == Opcode::MM_0F) as u32;

    let disp8 = rel32.wrapping_add(inst32_size - inst8_size) as i32;
    if i8::try_from(disp8).is_ok() && opcode8 != 0 && st.options.contains(InstOptions::SHORT_FORM) {
        st.options |= InstOptions::SHORT_FORM;
        buf.put1(opcode8 as u8); // Emit opcode.
        buf.put1(disp8 as u8); // Emit DISP8.
        return Ok(());
    }

    if st.opcode.get() == 0 || st.options.contains(InstOptions::SHORT_FORM) {
        return Err(X86Error::InvalidDisplacement {
            value: rel32 as i32 as i64,
            size: 1,
            reason: "displacement does not fit the requested/available branch form",
        });
    }

    st.options &= !InstOptions::SHORT_FORM;
    if st.opcode.get() & Opcode::MM_MASK != 0 {
        buf.put1(0x0F); // Emit 0x0F prefix.
    }
    buf.put1(st.opcode.get() as u8); // Emit Opcode.
    if st.op_reg != 0 {
        buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
    }
    buf.put4(rel32); // Emit DISP32.
    Ok(())
}

/// `EmitRel` — records a label fixup for an unbound label and emits a placeholder
/// displacement plus the trailing immediate.
///
/// asmkit's [`LabelUse::X86JmpRel32`] patch reads the placeholder as the addend, so
/// the placeholder carries AsmJit's `rel_offset + 4` (AsmJit stores the addend in the
/// fixup and emits zeros instead). Only rel32 fixups exist in asmkit.
pub fn emit_rel(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rel_size == 1 || st.rel_size == 4);
    if st.rel_size != 4 {
        debug_assert!(false, "rel8 fixups are not supported");
        return Err(X86Error::InvalidDisplacement {
            value: st.rel_offset as i32 as i64,
            size: st.rel_size as usize,
            reason: "8-bit fixups for unbound labels are not supported",
        });
    }

    // Chain with the label.
    let offset = buf.cur_offset();
    buf.put4(st.rel_offset.wrapping_add(4));
    buf.use_label_at_offset(offset, Label::from_id(st.label_id), LabelUse::X86JmpRel32);

    // The displacement placeholder is patched once the label offset becomes known.
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_mod_sib() {
        assert_eq!(encode_mod(3, 0, 1), 0xC1);
        assert_eq!(encode_mod(0, 4, 4), 0x24);
        assert_eq!(encode_sib(2, 1, 2), 0x8A);
    }

    #[test]
    fn rex_validation() {
        assert!(!is_rex_invalid(0x00));
        assert!(!is_rex_invalid(0x4F));
        assert!(!is_rex_invalid(0x80));
        assert!(is_rex_invalid(0x81));
    }

    #[test]
    fn writer_prefixes() {
        let mut buf = CodeBuffer::new();
        emit_pp(&mut buf, Opcode(Opcode::PP_66));
        emit_pp(&mut buf, Opcode(Opcode::PP_F2));
        emit_mm_and_opcode(&mut buf, Opcode(Opcode::MM_0F38 | 0x1A));
        assert_eq!(buf.data(), &[0x66, 0xF2, 0x0F, 0x38, 0x1A]);
    }

    #[test]
    fn writer_immediates() {
        let mut buf = CodeBuffer::new();
        emit_immediate(&mut buf, 0x1122_3344_5566_7788, 8);
        emit_imm_byte_or_dword(&mut buf, 0xAABB_CCDD, 4);
        emit_immediate(&mut buf, 0x1234, 2);
        assert_eq!(
            buf.data(),
            &[
                0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0xDD, 0xCC, 0xBB, 0xAA, 0x34, 0x12
            ]
        );
    }

    #[test]
    fn code_align() {
        let mut buf = CodeBuffer::new();
        buf.put1(0xCC);
        emit_code_align(&mut buf, 4);
        assert_eq!(buf.data(), &[0xCC, 0x0F, 0x1F, 0x00]);

        let mut buf = CodeBuffer::new();
        buf.put1(0xCC);
        emit_code_align(&mut buf, 16);
        let mut expected = vec![0xCC];
        expected.extend_from_slice(&super::super::encoder_tables::NOP_TABLE[8]);
        expected.extend_from_slice(&super::super::encoder_tables::NOP_TABLE[5][..6]);
        assert_eq!(buf.data(), &expected[..]);
    }

    // Emit-handler tests. Goldens cross-checked against AsmJit's own test suite
    // (meta/asmjit/asmjit-testing/tests/asmjit_test_assembler_x64.cpp).
    // --------------------------------------------------------------------------

    use crate::core::buffer::ExternalName;
    use crate::core::operand::{OperandCast, Sym};
    use crate::x86::instdb::{
        ALT_OPCODE_TABLE, INST_COMMON_INFO_TABLE, INST_INFO_TABLE, InstId, MAIN_OPCODE_TABLE,
    };
    use crate::x86::operands::regs::*;
    use crate::x86::operands::{
        dword_ptr, dword_ptr_u64, qword_ptr, qword_ptr_index, qword_ptr_label, qword_ptr_rip,
        qword_ptr_sym, qword_ptr_u64, qword_ptr_u64_rel,
    };

    fn db(inst: InstId) -> (Opcode, InstInfo, CommonInfo) {
        let info = INST_INFO_TABLE[inst as usize];
        (
            // AsmJit `_emit`: `opcode = main_opcode_table[i]; opcode |= main_opcode_value`.
            Opcode(
                MAIN_OPCODE_TABLE[info.main_opcode_index as usize] | info.main_opcode_value as u32,
            ),
            info,
            INST_COMMON_INFO_TABLE[info.common_info_index as usize],
        )
    }

    fn st_for(opcode: Opcode, rm_rel: Operand) -> X86EmitState {
        X86EmitState {
            opcode,
            rm_rel,
            ..Default::default()
        }
    }

    fn mem_op(mem: Mem) -> Operand {
        *mem.as_operand()
    }

    /// Prepares the mem-related state fields the way `emit_x86_m` does before tailing
    /// into `emit_mod_sib` (without emitting any prefixes).
    fn prep_mem(st: &mut X86EmitState) {
        let mem = st.rm_rel.as_::<Mem>();
        st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
        st.rb_reg = mem.base_id();
        st.rx_reg = mem.index_id();
        st.mem_op_ao_mark = 0;
    }

    fn run(buf: &mut CodeBuffer, st: &mut X86EmitState, f: EmitFn) {
        f(buf, st).expect("emit handler failed");
    }

    type EmitFn = fn(&mut CodeBuffer, &mut X86EmitState) -> Result<(), X86Error>;

    #[test]
    fn emit_x86_r_forms() {
        // add(rcx, rdx) — AsmJit golden "4801D1".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x01), Operand::default());
        st.op_reg = 2; // rdx
        st.rb_reg = 1; // rcx
        run(&mut buf, &mut st, emit_x86_r);
        assert_eq!(buf.data(), &[0x48, 0x01, 0xD1]);

        // adc(rcx, 1) — AsmJit golden "4883D101" (/2 opcode extension + imm8).
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x83), Operand::default());
        st.op_reg = 2;
        st.rb_reg = 1;
        st.imm_value = 1;
        st.imm_size = 1;
        run(&mut buf, &mut st, emit_x86_r);
        assert_eq!(buf.data(), &[0x48, 0x83, 0xD1, 0x01]);

        // add(r8, r9) — REX.R + REX.B: "4D03C1".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x03), Operand::default());
        st.op_reg = 8;
        st.rb_reg = 9;
        run(&mut buf, &mut st, emit_x86_r);
        assert_eq!(buf.data(), &[0x4D, 0x03, 0xC1]);
    }

    #[test]
    fn emit_x86_op_reg_movabs() {
        // movabs(rcx, 1) — AsmJit golden "48B90100000000000000".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0xB8), Operand::default());
        st.op_reg = 1;
        st.imm_value = 1;
        st.imm_size = 8;
        run(&mut buf, &mut st, emit_x86_op_reg);
        assert_eq!(buf.data(), &[0x48, 0xB9, 1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn emit_x86_op_mov_abs_form() {
        // mov rax, [0x1122334455667788] (moffs A1 form chosen by the arm).
        let mut buf = CodeBuffer::new();
        let mem = qword_ptr_u64(0x1122_3344_5566_7788);
        let mut st = st_for(Opcode(Opcode::W | 0xA1), mem_op(mem));
        st.imm_value = mem.offset();
        run(&mut buf, &mut st, emit_x86_op_mov_abs);
        assert_eq!(
            buf.data(),
            &[0x48, 0xA1, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]
        );
    }

    #[test]
    fn emit_x86_m_base_disp() {
        // mov rax, [rbx].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr(RBX, 0)));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x03]);

        // mov rax, [rbp] — BP requires disp8(0).
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr(RBP, 0)));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x45, 0x00]);

        // mov rax, [rbx + 64] — disp8.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr(RBX, 64)));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x43, 0x40]);

        // mov rax, [rbx + 0x12345678] — disp32.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(
            Opcode(Opcode::W | 0x8B),
            mem_op(qword_ptr(RBX, 0x1234_5678)),
        );
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x83, 0x78, 0x56, 0x34, 0x12]);

        // mov rax, [rsp + 16] — SP forces SIB.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr(RSP, 16)));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x44, 0x24, 0x10]);

        // add(rcx, qword_ptr(rdx, rbx, 0, 128)) — AsmJit golden "48038C1A80000000".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(
            Opcode(Opcode::W | 0x03),
            mem_op(qword_ptr_index(RDX, RBX, 0, 128)),
        );
        st.op_reg = 1;
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(
            buf.data(),
            &[0x48, 0x03, 0x8C, 0x1A, 0x80, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn emit_x86_m_cdisp8_compression() {
        // NOTE: plain X86M never carries a compressed displacement (AsmJit asserts
        // CDSHL==0 there); EVEX paths reach EmitModSib with CDSHL set, so these
        // drive `emit_mod_sib` directly.
        // Synthetic opcode with compressed-disp8 scale 4 (CDSHL_2), mem [rax + 16].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0x58 | Opcode::CDSHL_2), mem_op(qword_ptr(RAX, 16)));
        prep_mem(&mut st);
        run(&mut buf, &mut st, emit_mod_sib);
        assert_eq!(buf.data(), &[0x40, 0x04]);

        // Same but disp not a multiple of the scale: falls back to disp32.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0x58 | Opcode::CDSHL_2), mem_op(qword_ptr(RAX, 17)));
        prep_mem(&mut st);
        run(&mut buf, &mut st, emit_mod_sib);
        assert_eq!(buf.data(), &[0x80, 0x11, 0x00, 0x00, 0x00]);

        // Forced SIB (SP base) with compressed disp8, mem [rsp + 16].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0x58 | Opcode::CDSHL_2), mem_op(qword_ptr(RSP, 16)));
        prep_mem(&mut st);
        run(&mut buf, &mut st, emit_mod_sib);
        assert_eq!(buf.data(), &[0x44, 0x24, 0x04]);

        // TSIB flag forces SIB even for a non-SP base, mem [rax + 16].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0x58 | Opcode::CDSHL_2), mem_op(qword_ptr(RAX, 16)));
        prep_mem(&mut st);
        st.common_info.flags = InstFlags::TSIB.bits();
        run(&mut buf, &mut st, emit_mod_sib);
        assert_eq!(buf.data(), &[0x44, 0x20, 0x04]);
    }

    #[test]
    fn emit_x86_m_rip_disp32() {
        // mov rax, [rip + 0x1234].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr_rip(0x1234)));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x05, 0x34, 0x12, 0x00, 0x00]);
    }

    #[test]
    fn emit_x86_m_abs32() {
        // lea rax, [0xFFFFFFFF] — LEA drops REX.W instead of adding 0x67.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8D), mem_op(qword_ptr_u64(0xFFFF_FFFF)));
        st.inst_id = InstId::Lea as u32;
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x8D, 0x04, 0x25, 0xFF, 0xFF, 0xFF, 0xFF]);

        // mov eax, [0x80000000] — inserts the address-size override.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0x8B), mem_op(dword_ptr_u64(0x8000_0000)));
        st.inst_id = InstId::Mov as u32;
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(
            buf.data(),
            &[0x67, 0x8B, 0x04, 0x25, 0x00, 0x00, 0x00, 0x80]
        );

        // mov rax, fs:[0x40] — FS override prefers absolute, disp fits int32.
        let mut buf = CodeBuffer::new();
        let mut mem = qword_ptr_u64(0x40);
        mem.set_segment(FS);
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(mem));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(
            buf.data(),
            &[0x64, 0x48, 0x8B, 0x04, 0x25, 0x40, 0x00, 0x00, 0x00]
        );

        // Explicit rel addressing of a raw address: unencodable without a base address.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(qword_ptr_u64_rel(0x40)));
        let err = emit_x86_m(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidRIPRelative { .. }));

        // A true 64-bit absolute address is unencodable via disp32.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(
            Opcode(Opcode::W | 0x8B),
            mem_op(qword_ptr_u64(0x1_0000_0001)),
        );
        st.inst_id = InstId::Mov as u32;
        let err = emit_x86_m(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidMemoryOperand { .. }));
    }

    #[test]
    fn emit_x86_op_implicit_mem_form() {
        // stosb-like: opcode 0xAA with implicit [rdi].
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0xAA), mem_op(qword_ptr(RDI, 0)));
        run(&mut buf, &mut st, emit_x86_op_implicit_mem);
        assert_eq!(buf.data(), &[0xAA]);

        // 32-bit base emits the address-size override.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0xAA), mem_op(dword_ptr(EDI, 0)));
        run(&mut buf, &mut st, emit_x86_op_implicit_mem);

        // An implicit mem with an offset is rejected.
        let mut buf2 = CodeBuffer::new();
        let mut st2 = st_for(Opcode(0xAA), mem_op(qword_ptr(RDI, 8)));
        assert!(emit_x86_op_implicit_mem(&mut buf2, &mut st2).is_err());
        assert_eq!(buf.data(), &[0x67, 0xAA]);
    }

    #[test]
    fn emit_x86_r_from_m_form() {
        // umonitor rax: F3 0F AE /6 with the base register as the "address" operand —
        // AsmJit golden "F30FAEF0".
        let (opcode, _info, _common) = db(InstId::Umonitor);
        let mut buf = CodeBuffer::new();
        let mut st = st_for(opcode, mem_op(qword_ptr(RAX, 0)));
        st.op_reg = opcode.extract_mod_o();
        run(&mut buf, &mut st, emit_x86_r_from_m);
        assert_eq!(buf.data(), &[0xF3, 0x0F, 0xAE, 0xF0]);
    }

    #[test]
    fn emit_fpu_op_form() {
        // fadd st(0), st(0) = D8 C0.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(
            Opcode((0xD8 << Opcode::FPU_2B_SHIFT) | 0xC0),
            Operand::default(),
        );
        run(&mut buf, &mut st, emit_fpu_op);
        assert_eq!(buf.data(), &[0xD8, 0xC0]);

        // With the 9B (fwait) prefix via the PP field.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(
            Opcode(Opcode::PP_9B | (0xD8 << Opcode::FPU_2B_SHIFT) | 0xC0),
            Operand::default(),
        );
        run(&mut buf, &mut st, emit_fpu_op);
        assert_eq!(buf.data(), &[0x9B, 0xD8, 0xC0]);
    }

    #[test]
    fn emit_vex_op_vzeroupper() {
        let (opcode, _info, _common) = db(InstId::Vzeroupper);

        // AsmJit golden "C5F877".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(opcode, Operand::default());
        run(&mut buf, &mut st, emit_vex_op);
        assert_eq!(buf.data(), &[0xC5, 0xF8, 0x77]);

        // Forced VEX3: C4 E1 78 77 (hand-computed).
        let mut buf = CodeBuffer::new();
        let mut st = st_for(opcode, Operand::default());
        st.options = InstOptions::X86_VEX3;
        run(&mut buf, &mut st, emit_vex_op);
        assert_eq!(buf.data(), &[0xC4, 0xE1, 0x78, 0x77]);
    }

    #[test]
    fn emit_vex_evex_r_forms() {
        let (base, _info, _common) = db(InstId::Vaddps);

        // vaddps(xmm1, xmm2, xmm3) — AsmJit golden "C5E858CB".
        let mut buf = CodeBuffer::new();
        let mut st = st_for(base, Operand::default());
        st.op_reg = pack_reg_and_vvvvv(1, 2);
        st.rb_reg = 3;
        run(&mut buf, &mut st, emit_vex_evex_r);
        assert_eq!(buf.data(), &[0xC5, 0xE8, 0x58, 0xCB]);

        // vaddps(ymm1, ymm2, ymm3) — AsmJit golden "C5EC58CB".
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(32));
        let mut st = st_for(opcode, Operand::default());
        st.op_reg = pack_reg_and_vvvvv(1, 2);
        st.rb_reg = 3;
        run(&mut buf, &mut st, emit_vex_evex_r);
        assert_eq!(buf.data(), &[0xC5, 0xEC, 0x58, 0xCB]);

        // vaddps(zmm1, zmm2, zmm3) — AsmJit golden "62F16C4858CB".
        // LL=2 (512-bit) forces EVEX via kEvexBits; no X86_EVEX option needed.
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, Operand::default());
        st.op_reg = pack_reg_and_vvvvv(1, 2);
        st.rb_reg = 3;
        run(&mut buf, &mut st, emit_vex_evex_r);
        assert_eq!(buf.data(), &[0x62, 0xF1, 0x6C, 0x48, 0x58, 0xCB]);
    }

    #[test]
    fn emit_vex_evex_r_mask_er_sae() {
        let (base, _info, common) = db(InstId::Vaddpd);
        let zmm_opcode = |o: &mut Opcode| o.add(opcode_l_by_size(64));

        // k(k5).z().vaddpd(zmm1, zmm1, zmm2) — AsmJit golden "62F1F5CD58CA".
        // The {k}{z} bits force EVEX via kEvexBits; no X86_EVEX option needed.
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        zmm_opcode(&mut opcode);
        let mut st = st_for(opcode, Operand::default());
        st.options = InstOptions::X86_ZMASK;
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(1, 1);
        st.rb_reg = 2;
        st.extra_reg = *K5.as_operand();
        run(&mut buf, &mut st, emit_vex_evex_r);
        assert_eq!(buf.data(), &[0x62, 0xF1, 0xF5, 0xCD, 0x58, 0xCA]);

        // {x-sae} rounding forms — AsmJit goldens (vaddpd zmm1, zmm1, zmm2).
        let cases: [(InstOptions, u8); 4] = [
            (InstOptions::X86_ER, 0x18),                           // rn_sae
            (InstOptions::X86_ER | InstOptions::X86_RD_SAE, 0x38), // rd_sae
            (InstOptions::X86_ER | InstOptions::X86_RU_SAE, 0x58), // ru_sae
            (InstOptions::X86_ER | InstOptions::X86_RZ_SAE, 0x78), // rz_sae
        ];
        for (rounding, p3) in cases {
            let mut buf = CodeBuffer::new();
            let mut opcode = base;
            zmm_opcode(&mut opcode);
            let mut st = st_for(opcode, Operand::default());
            st.options = rounding;
            st.common_info = common;
            st.op_reg = pack_reg_and_vvvvv(1, 1);
            st.rb_reg = 2;
            run(&mut buf, &mut st, emit_vex_evex_r);
            assert_eq!(buf.data(), &[0x62, 0xF1, 0xF5, p3, 0x58, 0xCA]);
        }

        // {er} is only encodable for 512-bit/scalar forms; the xmm (LL=0) form of a
        // broadcast-capable instruction is rejected.
        let (vaddps_base, _, vaddps_common) = db(InstId::Vaddps);
        let mut buf = CodeBuffer::new();
        let mut opcode = vaddps_base;
        opcode.add(opcode_l_by_size(16));
        let mut st = st_for(opcode, Operand::default());
        st.options = InstOptions::X86_ER;
        st.common_info = vaddps_common;
        st.op_reg = pack_reg_and_vvvvv(1, 1);
        st.rb_reg = 2;
        let err = emit_vex_evex_r(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidRoundingControl { .. }));
    }

    #[test]
    fn emit_vex_evex_m_forms() {
        let (base, _info, common) = db(InstId::Vaddps);

        // vaddps(zmm1, zmm2, ptr(rbx, rbp, 0, 128)) — AsmJit golden "62F16C48584C2B02".
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, mem_op(qword_ptr_index(RBX, RBP, 0, 128)));
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(1, 2);
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(
            buf.data(),
            &[0x62, 0xF1, 0x6C, 0x48, 0x58, 0x4C, 0x2B, 0x02]
        );
    }

    #[test]
    fn emit_vex_evex_m_broadcast() {
        let (base, _info, common) = db(InstId::Vcmppd);

        // vcmppd(k2, zmm12, qword_ptr(rcx)._1to8(), 123) — AsmJit golden "62F19D58C2117B".
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, mem_op(qword_ptr(RCX, 0)._1to8()));
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(2, 12);
        st.imm_value = 123;
        st.imm_size = 1;
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(buf.data(), &[0x62, 0xF1, 0x9D, 0x58, 0xC2, 0x11, 0x7B]);

        // Broadcast compressed disp8: disp 1016 / 8 = 127 fits — "62F19D58C2527F7B".
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, mem_op(qword_ptr(RDX, 1016)._1to8()));
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(2, 12);
        st.imm_value = 123;
        st.imm_size = 1;
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(
            buf.data(),
            &[0x62, 0xF1, 0x9D, 0x58, 0xC2, 0x52, 0x7F, 0x7B]
        );

        // disp 1024 / 8 = 128 doesn't fit: disp32 — "62F19D58C292000400007B".
        let mut buf = CodeBuffer::new();
        let mut opcode = base;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, mem_op(qword_ptr(RDX, 1024)._1to8()));
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(2, 12);
        st.imm_value = 123;
        st.imm_size = 1;
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(
            buf.data(),
            &[
                0x62, 0xF1, 0x9D, 0x58, 0xC2, 0x92, 0x00, 0x04, 0x00, 0x00, 0x7B
            ]
        );

        // vcmpps(k2, zmm17, dword_ptr(rcx)._1to16(), 123) — "62F17450C2117B".
        // Exercises the V' bit from vvvvv bit 4 (zmm17).
        let (base_ps, _info_ps, common_ps) = db(InstId::Vcmpps);
        let mut buf = CodeBuffer::new();
        let mut opcode = base_ps;
        opcode.add(opcode_l_by_size(64));
        let mut st = st_for(opcode, mem_op(dword_ptr(RCX, 0)._1to16()));
        st.common_info = common_ps;
        st.op_reg = pack_reg_and_vvvvv(2, 17);
        st.imm_value = 123;
        st.imm_size = 1;
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(buf.data(), &[0x62, 0xF1, 0x74, 0x50, 0xC2, 0x11, 0x7B]);
    }

    #[test]
    fn emit_vex_evex_m_vsib() {
        let (_base, info, common) = db(InstId::Vgatherdpd);

        // k(k1).vgatherdpd(ymm1, ptr(rdx, xmm3, 0, 128)) — golden "62F2FD29924C1A10".
        // The VexRmvRm_VM arm uses the alt (EVEX) opcode, which carries FORCE_EVEX.
        let mut buf = CodeBuffer::new();
        let mut opcode = Opcode(ALT_OPCODE_TABLE[info.alt_opcode_index as usize]);
        opcode.add(opcode_l_by_size(32));
        let mem = Mem::from_base_and_index_shift_disp(&RDX, &XMM3, 0, 128, 8, 0.into());
        let mut st = st_for(opcode, mem_op(mem));
        st.common_info = common;
        st.op_reg = pack_reg_and_vvvvv(1, 0);
        st.extra_reg = *K1.as_operand();
        run(&mut buf, &mut st, emit_vex_evex_m);
        assert_eq!(
            buf.data(),
            &[0x62, 0xF2, 0xFD, 0x29, 0x92, 0x4C, 0x1A, 0x10]
        );
    }

    #[test]
    fn emit_jmp_call_bound() {
        let (_j, jmp_info, _c) = db(InstId::Jmp);
        let (_j, jz_info, _c) = db(InstId::Jz);

        // Backward jmp to a bound label: rel32 = 0 - 0 - 5 = -5.
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        buf.bind_label(label);
        let mut st = st_for(Opcode(0xE9), Label::from_id(label.id()).0);
        st.inst_info = jmp_info;
        st.label_id = label.id();
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.data(), &[0xE9, 0xFB, 0xFF, 0xFF, 0xFF]);

        // Same with SHORT_FORM: disp8 = -5 + 3 = -2.
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        buf.bind_label(label);
        let mut st = st_for(Opcode(0xE9), Label::from_id(label.id()).0);
        st.inst_info = jmp_info;
        st.label_id = label.id();
        st.options = InstOptions::SHORT_FORM;
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.data(), &[0xEB, 0xFE]);

        // Backward jz (0F 84): rel32 = 0 - 0 - 6 = -6.
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        buf.bind_label(label);
        let mut st = st_for(Opcode(Opcode::MM_0F | 0x84), Label::from_id(label.id()).0);
        st.inst_info = jz_info;
        st.label_id = label.id();
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.data(), &[0x0F, 0x84, 0xFA, 0xFF, 0xFF, 0xFF]);

        // jz short form: disp8 = -6 + 4 = -2.
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        buf.bind_label(label);
        let mut st = st_for(Opcode(Opcode::MM_0F | 0x84), Label::from_id(label.id()).0);
        st.inst_info = jz_info;
        st.label_id = label.id();
        st.options = InstOptions::SHORT_FORM;
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.data(), &[0x74, 0xFE]);
    }

    #[test]
    fn emit_jmp_call_unbound_fixup() {
        let (_j, jmp_info, _c) = db(InstId::Jmp);

        // Forward jmp: long-form placeholder patched by the fixup at finish().
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        let mut st = st_for(Opcode(0xE9), Label::from_id(label.id()).0);
        st.inst_info = jmp_info;
        st.label_id = label.id();
        run(&mut buf, &mut st, emit_jmp_call);
        buf.put1(0x90); // nop
        buf.bind_label(label);
        let out = buf.finish();
        // disp = 6 - 1 - 4 = 1.
        assert_eq!(out.data(), &[0xE9, 0x01, 0x00, 0x00, 0x00, 0x90]);

        // Unbound + SHORT_FORM: error (asmkit cannot relax rel8 fixups).
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        let mut st = st_for(Opcode(0xE9), Label::from_id(label.id()).0);
        st.inst_info = jmp_info;
        st.label_id = label.id();
        st.options = InstOptions::SHORT_FORM;
        let err = emit_jmp_call(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidDisplacement { .. }));

        // rel8-only instruction (jecxz) with an unbound label: error.
        let (_j, jecxz_info, _c) = db(InstId::Jecxz);
        assert_eq!(ALT_OPCODE_TABLE[jecxz_info.alt_opcode_index as usize], 0xE3);
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        let mut st = st_for(Opcode(0), Label::from_id(label.id()).0);
        st.inst_info = jecxz_info;
        st.label_id = label.id();
        let err = emit_jmp_call(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidDisplacement { .. }));

        // Invalid label id.
        let mut buf = CodeBuffer::new();
        let mut st = st_for(Opcode(0xE9), Label::from_id(123).0);
        st.inst_info = jmp_info;
        st.label_id = 123;
        let err = emit_jmp_call(&mut buf, &mut st).unwrap_err();
        assert!(matches!(err, X86Error::InvalidLabel { .. }));
    }

    #[test]
    fn emit_jmp_call_sym_reloc() {
        let (_j, call_info, _c) = db(InstId::Call);

        // call sym — long form + X86CallPCRel4 reloc with addend -4.
        let mut buf = CodeBuffer::new();
        let sym = buf.add_symbol(
            ExternalName::Symbol("target_fn".into()),
            RelocDistance::Near,
        );
        let mut st = st_for(Opcode(0xE8), Sym::from_id(sym.id()).0);
        st.inst_info = call_info;
        st.inst_id = InstId::Call as u32;
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.data(), &[0xE8, 0x00, 0x00, 0x00, 0x00]);
        let relocs = buf.relocs();
        assert_eq!(relocs.len(), 1);
        assert_eq!(relocs[0].offset, 1);
        assert_eq!(relocs[0].kind, Reloc::X86CallPCRel4);
        assert_eq!(relocs[0].addend, -4);

        // jmp far-sym — X86GOTPCRel4.
        let (_j, jmp_info, _c) = db(InstId::Jmp);
        let mut buf = CodeBuffer::new();
        let sym = buf.add_symbol(
            ExternalName::Symbol("far_target".into()),
            RelocDistance::Far,
        );
        let mut st = st_for(Opcode(0xE9), Sym::from_id(sym.id()).0);
        st.inst_info = jmp_info;
        st.inst_id = InstId::Jmp as u32;
        run(&mut buf, &mut st, emit_jmp_call);
        assert_eq!(buf.relocs()[0].kind, Reloc::X86GOTPCRel4);
    }

    #[test]
    fn emit_mod_sib_rip_label() {
        // mov rax, [rip + label + 8], label already bound at offset 0.
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        buf.bind_label(label);
        let mem = qword_ptr_label(label, 8);
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(mem));
        run(&mut buf, &mut st, emit_x86_m);
        // disp = 8 - 4 + (0 - 3) = 1.
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x05, 0x01, 0x00, 0x00, 0x00]);

        // Unbound label: placeholder patched by the fixup at finish().
        let mut buf = CodeBuffer::new();
        let label = buf.get_label();
        let mem = qword_ptr_label(label, 8);
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(mem));
        run(&mut buf, &mut st, emit_x86_m);
        buf.put1(0x90); // nop
        buf.bind_label(label);
        let out = buf.finish();
        // disp = 8 - 3 + 8 - 4 = 9.
        assert_eq!(
            out.data(),
            &[0x48, 0x8B, 0x05, 0x09, 0x00, 0x00, 0x00, 0x90]
        );
    }

    #[test]
    fn emit_mod_sib_sym_reloc() {
        // mov rax, [rip + sym]: rip-relative disp32 with an X86PCRel4 reloc.
        let mut buf = CodeBuffer::new();
        let sym = buf.add_symbol(ExternalName::Symbol("data_sym".into()), RelocDistance::Near);
        let mem = qword_ptr_sym(sym, 0);
        let mut st = st_for(Opcode(Opcode::W | 0x8B), mem_op(mem));
        run(&mut buf, &mut st, emit_x86_m);
        assert_eq!(buf.data(), &[0x48, 0x8B, 0x05, 0x00, 0x00, 0x00, 0x00]);
        let relocs = buf.relocs();
        assert_eq!(relocs.len(), 1);
        assert_eq!(relocs[0].offset, 3);
        assert_eq!(relocs[0].kind, Reloc::X86PCRel4);
        assert_eq!(relocs[0].addend, -4);
    }
}
