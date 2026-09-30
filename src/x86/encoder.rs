//! X86 encoder helpers, buffer-writer functions, and emit handlers.
//!
//! The emit handlers cover both 64-bit and 32-bit modes; the mode is carried by
//! [`X86EmitState::is_32bit`].
//!
//! Derived from AsmJit (Zlib license): this file is an altered version; see LICENSE notices.

use crate::X86Error;
use crate::core::buffer::{CodeBuffer, LabelUse, Reloc, RelocDistance, RelocTarget};
use crate::core::globals::{INVALID_ID, InstOptions};
use crate::core::operand::{Label, Operand, OperandCast, RegType, Sym};
use crate::core::relax::RelaxableJump;

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
/// (AsmJit's `is_implicit_mem`: used by string ops' implicit `[zAX]` forms).
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

/// Decides whether to use the absolute (movabs) form for a memory operand (AsmJit's
/// `x86_should_use_movabs`, 64-bit simplified).
pub fn should_use_movabs(
    is_32bit: bool,
    register_size: u32,
    options: InstOptions,
    rm_rel: &Mem,
) -> bool {
    let _ = register_size;
    if is_32bit {
        // There is no relative addressing, just decide whether to use MOV encoded
        // with MOD R/M or absolute.
        return !options.intersects(InstOptions::X86_MOD_MR | InstOptions::X86_MOD_RM);
    }

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

/// `ENC_OPSn`: packs operand types into a 3-bit-per-operand signature (isign3/isign4).
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

/// Emits optimized multi-byte NOPs to align the current offset to `alignment`.
pub fn emit_code_align(buf: &mut CodeBuffer, alignment: u32) {
    debug_assert!(alignment.is_power_of_two());
    let len = (buf.cur_offset().wrapping_neg()) & (alignment - 1);
    let mut pad = smallvec::SmallVec::<[u8; 16]>::new();
    push_nops(&mut pad, len);
    for b in pad {
        buf.put1(b);
    }
}

/// Appends `len` bytes of NOPs, longest first.
pub(crate) fn push_nops<A: smallvec::Array<Item = u8>>(out: &mut smallvec::SmallVec<A>, len: u32) {
    let mut i = len;
    while i > 0 {
        let n = i.min(9) as usize;
        out.extend_from_slice(&super::encoder_tables::NOP_TABLE[n - 1][..n]);
        i -= n as u32;
    }
}

/// Emits a 1- or 4-byte immediate (VEX path: sizes other than 1/4 assert in debug).
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
    /// Target mode: 32-bit X86 (AsmJit's `Assembler::is_32bit()`).
    pub is_32bit: bool,
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
    /// {k} mask register (or rep-cx). Must be an id-0 operand when unused: NOT
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
            is_32bit: false,
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

impl X86EmitState {
    /// Address-override info mask for the target mode (AsmJit's
    /// `_address_override_mask()`): [`MEM_INFO_67H_X86`] in 32-bit mode,
    /// [`MEM_INFO_67H_X64`] in 64-bit mode.
    pub fn address_override_mask(&self) -> u8 {
        if self.is_32bit {
            MEM_INFO_67H_X86
        } else {
            MEM_INFO_67H_X64
        }
    }

    /// Native register size of the target mode (AsmJit's `register_size()`).
    pub fn register_size(&self) -> u32 {
        if self.is_32bit { 4 } else { 8 }
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

/// `EmitX86OpMovAbs`: movabs preamble: the address becomes an immediate of native
/// register size (4 bytes in 32-bit mode, 8 in 64-bit); falls through to
/// [`emit_x86_op`].
pub fn emit_x86_op_mov_abs(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    // AsmJit: `imm_size = FastUInt8(register_size())`: native register size.
    st.imm_size = st.register_size() as u8;
    emit_segment_override(buf, st.rm_rel.as_::<Mem>().segment_id());
    emit_x86_op(buf, st)
}

/// `EmitX86Op`: bare opcode (+ REX + immediate), no ModRM.
pub fn emit_x86_op(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);
    let rex = st.opcode.extract_rex(st.options);
    emit_rex(buf, rex)?;
    emit_mm_and_opcode(buf, st.opcode);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86OpReg`: opcode with the low 3 bits of a register id added to it (no ModRM).
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

/// `EmitX86OpImplicitMem`: opcode with an implicit memory operand (string ops).
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
    emit_address_override(buf, st.rm_info & st.address_override_mask() != 0);

    emit_mm_and_opcode(buf, st.opcode);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86R`: opcode /r with a register r/m (`MOD(3, reg, rm)`).
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

/// `EmitX86RFromM`: opcode /r where the r/m register comes from a memory operand's
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
        | (st.rb_reg >> 3); // REX.B (0x01): unmasked, as in AsmJit.
    emit_rex(buf, rex)?;
    st.op_reg &= 0x07;
    st.rb_reg &= 0x07;

    emit_segment_override(buf, mem.segment_id());
    emit_address_override(buf, st.rm_info & st.address_override_mask() != 0);

    emit_mm_and_opcode(buf, st.opcode);
    buf.put1(encode_mod(3, st.op_reg, st.rb_reg) as u8);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitX86M`: opcode /r with a memory r/m; tails into [`emit_mod_sib`].
pub fn emit_x86_m(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    debug_assert!(st.opcode.get() & Opcode::CDSHL_MASK == 0);
    let mem = st.rm_rel.as_::<Mem>();

    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    emit_segment_override(buf, mem.segment_id());

    st.mem_op_ao_mark = buf.cur_offset();
    emit_address_override(buf, st.rm_info & st.address_override_mask() != 0);

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

/// `EmitModSib`: ModRM + SIB + displacement for the memory operand in `st.rm_rel`.
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
            // asmkit extension: a Sym base is encoded with a relocation (AsmJit has no
            // Sym operands): rip-relative in 64-bit mode, absolute in 32-bit mode
            // (mirrors the old asmkit encoder).
            if mem.has_base_sym() {
                buf.put1(encode_mod(0, st.op_reg, 5) as u8);
                let disp_offset = buf.cur_offset();
                buf.put4(mem.offset_lo32() as u32);
                let sym = Sym::from_id(mem.base_id());
                if st.is_32bit {
                    buf.add_reloc_at_offset(
                        disp_offset,
                        Reloc::Abs4,
                        RelocTarget::Sym(sym),
                        mem.offset(),
                    );
                } else {
                    let distance = buf.symbol_distance(sym).ok_or(X86Error::InvalidOperand {
                        operand_index: 0,
                        reason: "symbol is not declared in this buffer",
                    })?;
                    let kind = if distance == RelocDistance::Near {
                        Reloc::X86PCRel4
                    } else {
                        Reloc::X86GOTPCRel4
                    };
                    buf.add_reloc_at_offset(disp_offset, kind, RelocTarget::Sym(sym), -4);
                }
                emit_immediate(buf, st.imm_value as u64, st.imm_size);
                return Ok(());
            }

            let mut addr_type = mem.addr_type();
            let rel_offset = mem.offset_lo32();

            if st.is_32bit {
                // Explicit relative addressing doesn't work in 32-bit mode.
                if addr_type == AddrType::Rel {
                    return Err(invalid_address(
                        &mem,
                        "relative addressing requires 64-bit mode",
                    ));
                }

                buf.put1(encode_mod(0, st.op_reg, 5) as u8);
                buf.put4(rel_offset as u32);
                emit_immediate(buf, st.imm_value as u64, st.imm_size);
                return Ok(());
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

            buf.put1(encode_mod(0, st.op_reg, 4) as u8);
            buf.put1(encode_sib(0, 4, 5) as u8);
            buf.put4(rel_offset as u32);
        } else {
            // ==========|> [LABEL|RIP + DISP32]
            buf.put1(encode_mod(0, st.op_reg, 5) as u8);

            if st.is_32bit {
                return emit_mod_sib_label_rip_x86(buf, st);
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
                    let at = buf.cur_offset();
                    buf.record_label_ref(at, label, LabelUse::X86JmpRel32, rel.wrapping_add(4));
                    let rel = rel.wrapping_add(buf.label_offset(label).wrapping_sub(at) as i32);
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
            buf.put1((st.op_reg | 0x06) as u8);
            buf.put2(rel_offset as u16);
        }
    }

    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}

/// `EmitModSib_LabelRip_X86`: 32-bit [LABEL|RIP + DISP32] tail: there is no
/// rip-relative addressing in 32-bit mode, so AsmJit turns the displacement into an
/// absolute address via a `kRelToAbs` relocation. asmkit models that with
/// [`Reloc::Abs4`]: a bound label is resolved in place (base-address-free, matching
/// the 64-bit arm's convention), an unbound label gets an Abs4 reloc against it, and
/// a rip base becomes an Abs4 reloc against an anonymous label bound at the end of
/// the instruction (AsmJit's `payload = source_offset + region_size + rel_offset`).
/// Shared by [`emit_mod_sib`] and [`emit_mod_v_sib`], mirroring AsmJit's shared label.
fn emit_mod_sib_label_rip_x86(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    let mem = st.rm_rel.as_::<Mem>();
    let rel_offset = mem.offset_lo32();

    if st.rm_info & MEM_INFO_BASE_LABEL != 0 {
        // [LABEL->ABS].
        let label_id = mem.base_id();
        if label_id >= buf.label_count() {
            return Err(X86Error::InvalidLabel {
                label_id,
                reason: "invalid label id",
            });
        }
        let label = Label::from_id(label_id);
        if buf.is_bound(label) {
            buf.put4(rel_offset.wrapping_add(buf.label_offset(label) as i32) as u32);
        } else {
            let disp_offset = buf.cur_offset();
            buf.put4(0);
            buf.add_reloc_at_offset(
                disp_offset,
                Reloc::Abs4,
                RelocTarget::Label(label),
                rel_offset as i64,
            );
        }
        emit_immediate(buf, st.imm_value as u64, st.imm_size);
        return Ok(());
    }

    // [RIP->ABS].
    let disp_offset = buf.cur_offset();
    buf.put4(0);
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    let end = buf.get_label();
    buf.bind_label(end);
    buf.add_reloc_at_offset(
        disp_offset,
        Reloc::Abs4,
        RelocTarget::Label(end),
        rel_offset as i64,
    );
    Ok(())
}

/// `EmitModVSib`: SIB (and VSIB) forms with an index register.
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
        if st.is_32bit {
            // 32-bit: absolute disp32, sharing the label/rip arm of `emit_mod_sib`.
            buf.put1(encode_mod(0, st.op_reg, 4) as u8);
            buf.put1(encode_sib(mem.shift(), rx, 5) as u8);
            return emit_mod_sib_label_rip_x86(buf, st);
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

/// `EmitFpuOp`: FPU opcode (two opcode bytes, plus optional 9B prefix via PP).
pub fn emit_fpu_op(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    emit_pp(buf, st.opcode);

    // FPU instructions consist of two opcodes.
    buf.put1((st.opcode.get() >> Opcode::FPU_2B_SHIFT) as u8);
    buf.put1(st.opcode.get() as u8);
    Ok(())
}

/// `EmitVexOp`: VEX opcode with no ModRM (only `vzeroall`/`vzeroupper`).
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

/// `EmitVexEvexR`: VEX|EVEX prefix + opcode /r with a register r/m.
pub fn emit_vex_evex_r(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    let opcode = st.opcode.get();

    // Construct `x` - a complete EVEX|VEX prefix.
    let mut x = ((st.op_reg << 4) & 0xF980) // [........|........|Vvvvv..R|R.......].
        | ((st.rb_reg << 2) & 0x0060) // [........|........|........|.BB.....].
        | st.opcode.extract_ll_mmmmm(st.options) // [........|.LL.....|Vvvvv..R|RBBmmmmm].
        | (st.extra_reg.id() << 16); // [........|.LL..aaa|Vvvvv..R|RBBmmmmm].
    let op_reg = st.op_reg & 0x7;

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

/// `EmitVexEvexM`: VEX|EVEX prefix + opcode /r with a memory r/m; tails into
/// [`emit_mod_sib`], or [`emit_mod_v_sib`] for VSIB instructions.
pub fn emit_vex_evex_m(buf: &mut CodeBuffer, st: &mut X86EmitState) -> Result<(), X86Error> {
    debug_assert!(st.rm_rel.is_mem());
    let mem = st.rm_rel.as_::<Mem>();

    st.rm_info = MEM_INFO_TABLE[mem.base_and_index_types() as usize];
    emit_segment_override(buf, mem.segment_id());

    st.mem_op_ao_mark = buf.cur_offset();
    emit_address_override(buf, st.rm_info & st.address_override_mask() != 0);

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

/// `EmitJmpCall`: jmp/jcc/call with a Label, Imm, or Sym target.
///
/// asmkit deviations from AsmJit, forced by the lack of a base address:
///
/// - Unbound labels use the long (rel32) form unless `SHORT_FORM` was requested
///   or the instruction is rel8-only (jecxz/loop); then the rel8 form is
///   emitted and the label must bind within 127 bytes.
/// - Bound labels use the short form when possible unless `LONG_FORM` was requested.
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
        let relaxable = relaxable_cc(st, opcode8);

        if buf.is_bound(label) {
            // Label bound to the current section.
            let rel32 = (buf.label_offset(label) as u64)
                .wrapping_sub(ip)
                .wrapping_sub(inst32_size as u64) as u32;
            let start = buf.cur_offset();
            emit_jmp_call_rel(buf, st, rel32, opcode8)?;
            let end = buf.cur_offset();
            if let Some(cc) = relaxable {
                buf.record_relaxable_jump(RelaxableJump {
                    start,
                    len: (end - start) as u8,
                    label,
                    cc,
                });
            } else if st.options.contains(InstOptions::SHORT_FORM) {
                buf.record_label_ref(end - 1, label, LabelUse::X86BranchRel8, 0);
            } else {
                buf.record_label_ref(end - 4, label, LabelUse::X86JmpRel32, 0);
            }
            return Ok(());
        }

        // Non-bound label with `SHORT_FORM` (or a rel8-only instruction): the
        // rel8 form, patched by a fixup. The label must bind within reach.
        if st.opcode.get() == 0 || st.options.contains(InstOptions::SHORT_FORM) {
            if opcode8 == 0 {
                return Err(X86Error::InvalidDisplacement {
                    value: 0,
                    size: 1,
                    reason: "instruction has no rel8 form",
                });
            }
            buf.put1(opcode8 as u8); // Emit opcode.
            let offset = buf.cur_offset();
            buf.put1(0); // Emit DISP8 (patched by the fixup).
            buf.use_label_at_offset(offset, label, LabelUse::X86BranchRel8);
            buf.record_label_ref(offset, label, LabelUse::X86BranchRel8, 0);
            return Ok(());
        }

        let start = buf.cur_offset();
        if st.opcode.get() & Opcode::MM_MASK != 0 {
            buf.put1(0x0F); // Emit 0F prefix.
        }
        buf.put1(st.opcode.get() as u8); // Emit opcode.
        if st.op_reg != 0 {
            buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
        }

        if let Some(cc) = relaxable {
            // `finish` shrinks it to rel8 when the label ends up in reach.
            let offset = buf.cur_offset();
            buf.put4(0);
            buf.use_label_at_offset(offset, label, LabelUse::X86JmpRel32);
            buf.record_relaxable_jump(RelaxableJump {
                start,
                len: (offset + 4 - start) as u8,
                label,
                cc,
            });
            return Ok(());
        }

        // Record DISP32 (non-bound label).
        st.rel_offset = (-4i32) as u32;
        st.rel_size = 4;
        return emit_rel(buf, st);
    }

    if st.rm_rel.is_imm() {
        // asmkit has no base address: a plain immediate is a raw displacement.
        // The rel8 form is used when it exists and the value fits (rel8-only
        // instructions like loop/jcxz require it); LONG_FORM forces rel32.
        let rel = st.rm_rel.as_::<crate::core::operand::Imm>().value();
        if opcode8 != 0 && !st.options.contains(InstOptions::LONG_FORM) {
            if let Ok(disp8) = i8::try_from(rel) {
                buf.put1(opcode8 as u8); // Emit opcode.
                buf.put1(disp8 as u8); // Emit DISP8.
                return Ok(());
            }
        }

        if st.opcode.get() == 0 || st.options.contains(InstOptions::SHORT_FORM) {
            return Err(X86Error::InvalidDisplacement {
                value: rel,
                size: 1,
                reason: "displacement does not fit the requested/available branch form",
            });
        }

        if st.opcode.get() & Opcode::MM_MASK != 0 {
            buf.put1(0x0F); // Emit 0F prefix.
        }
        buf.put1(st.opcode.get() as u8); // Emit opcode.
        if st.op_reg != 0 {
            buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
        }
        buf.put4(rel as u32); // Emit DISP32.
        return Ok(());
    }

    if st.rm_rel.is_sym() {
        // asmkit extension: jump/call to a symbol: long form + relocation.
        if st.opcode.get() == 0 {
            return Err(X86Error::InvalidDisplacement {
                value: 0,
                size: 1,
                reason: "symbol target requires the rel32 form",
            });
        }
        let sym = Sym::from_id(st.rm_rel.id());
        let distance = buf.symbol_distance(sym).ok_or(X86Error::InvalidOperand {
            operand_index: 0,
            reason: "symbol is not declared in this buffer",
        })?;

        if distance == RelocDistance::Far {
            // A far symbol is reached through its GOT slot, so the branch is
            // the indirect `FF /2` (call) or `FF /4` (jmp) form over
            // `[rip + slot]`. A rel32 to the slot would jump into data.
            let modrm = if st.inst_id == InstId::Call as u32 {
                encode_mod(0, 2, 5)
            } else if st.inst_id == InstId::Jmp as u32 {
                encode_mod(0, 4, 5)
            } else {
                return Err(invalid_instruction(
                    st,
                    "a far symbol is only a call or jmp target",
                ));
            };
            buf.put1(0xFF);
            buf.put1(modrm as u8);
            let disp_offset = buf.cur_offset();
            buf.put4(0); // Emit DISP32 (patched by the relocation).
            buf.add_reloc_at_offset(disp_offset, Reloc::X86GOTPCRel4, RelocTarget::Sym(sym), -4);
            return Ok(());
        }

        if st.opcode.get() & Opcode::MM_MASK != 0 {
            buf.put1(0x0F); // Emit 0F prefix.
        }
        buf.put1(st.opcode.get() as u8); // Emit opcode.
        if st.op_reg != 0 {
            buf.put1(encode_mod(3, st.op_reg, 0) as u8); // Emit MOD.
        }

        let disp_offset = buf.cur_offset();
        buf.put4(0); // Emit DISP32 (patched by the relocation).
        let kind = if st.inst_id == InstId::Call as u32 {
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

fn relaxable_cc(st: &X86EmitState, opcode8: u32) -> Option<Option<u8>> {
    if st.op_reg != 0
        || st
            .options
            .intersects(InstOptions::LONG_FORM | InstOptions::SHORT_FORM)
    {
        return None;
    }
    let opcode = st.opcode.get();
    let is_0f = opcode & Opcode::MM_MASK == Opcode::MM_0F;
    let cc = opcode8 as u8 & 0x0F;
    match opcode8 {
        0xEB if !is_0f && opcode as u8 == 0xE9 => Some(None),
        0x70..=0x7F if is_0f && opcode as u8 == 0x80 | cc => Some(Some(cc)),
        _ => None,
    }
}

/// `EmitJmpCallRel`: jmp/jcc/call with the relative displacement known at assembly
/// time. The short (rel8) form is selected whenever it fits unless `LONG_FORM`
/// was requested.
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
    if i8::try_from(disp8).is_ok() && opcode8 != 0 && !st.options.contains(InstOptions::LONG_FORM) {
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

/// `EmitRel`: records a label fixup for an unbound label and emits a placeholder
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
    let label = Label::from_id(st.label_id);
    let addend = st.rel_offset.wrapping_add(4);
    buf.put4(addend);
    buf.use_label_at_offset(offset, label, LabelUse::X86JmpRel32);
    buf.record_label_ref(offset, label, LabelUse::X86JmpRel32, addend as i32);

    // The displacement placeholder is patched once the label offset becomes known.
    emit_immediate(buf, st.imm_value as u64, st.imm_size);
    Ok(())
}
