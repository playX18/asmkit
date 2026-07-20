//! Byte-writing emit handlers and opcode composition for the AArch64
//! assembler: the helper layer of AsmJit's `a64assembler.cpp` (opcode
//! composition, immediate encoders, operand checks) plus the `EmitOp` /
//! `EmitOp_DispImm` / `EmitOp_Rel` handler blocks reached from the encoding
//! arms in [`super::emit`].
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

#![allow(clippy::eq_op, clippy::erasing_op, dead_code, unused)]
use crate::AsmError;
use crate::aarch64::emit::Handler;
use crate::aarch64::encoder_tables::{SIZE_OP_MAP, SIZE_OP_TABLE};
use crate::aarch64::operands::*;
use crate::aarch64::{Assembler, instdb::*};
use crate::core::buffer::LabelUse;
use crate::core::operand::*;

macro_rules! B {
    ($e: expr) => {
        1 << $e
    };
}

macro_rules! check_signature {
    ($op0: expr, $op1: expr) => {
        $op0.signature() == $op1.signature()
    };
    ($op0: expr, $op1: expr, $op2: expr) => {
        $op0.signature() == $op1.signature() && $op1.signature() == $op2.signature()
    };

    ($op0: expr, $op1: expr, $op2: expr, $op3: expr) => {
        $op0.signature() == $op1.signature()
            && $op1.signature() == $op2.signature()
            && $op2.signature() == $op3.signature()
    };
}

/// Per-instruction emit state shared between the encoding arms in
/// [`super::emit`] and the byte-writing handlers here (AsmJit's `_emit`
/// locals).
pub(crate) struct A64EmitState {
    /// The opcode word being composed.
    pub opcode: Opc,
    /// Offset/displacement format of pc-relative and load/store forms.
    pub offset_format: OffsetFormat,
    /// Displacement value packed by `EmitOp_DispImm`.
    pub offset_value: i64,
    /// Words of a multi-instruction mov sequence.
    pub multiple_op_data: [u32; 4],
    /// Number of valid words in `multiple_op_data`.
    pub multiple_op_count: usize,
    /// The operand a pc-relative form refers to (label, label-based mem, or
    /// immediate).
    pub rm_rel: Operand,
}

impl A64EmitState {
    pub(crate) fn new() -> Self {
        Self {
            opcode: Opc(0),
            offset_format: OffsetFormat::new(OffsetType::SignedOffset, 0, 0, 0, 0, 0, 0, 0),
            offset_value: 0,
            multiple_op_data: [0; 4],
            multiple_op_count: 0,
            rm_rel: Operand::new(),
        }
    }
}

impl Assembler<'_> {
    /// Dispatches to a byte-writing emit handler (AsmJit's `EmitOp` /
    /// `EmitOp_DispImm` / `EmitOp_Rel` blocks at the end of `_emit`).
    ///
    /// Returns `true` when the instruction was emitted or an error was
    /// recorded. `Handler::OpRel` returns `false` when `rm_rel` is not a
    /// relative operand and the encoding arm must continue.
    pub(crate) fn emit_handler(&mut self, handler: Handler, st: &mut A64EmitState) -> bool {
        match handler {
            Handler::Op => {
                self.buffer.write_u32(st.opcode.get());
                true
            }
            Handler::OpDispImm => {
                self.emit_disp_imm(st);
                true
            }
            Handler::OpRel => self.emit_rel(st),
            Handler::Multi => {
                for i in 0..st.multiple_op_count {
                    self.buffer.write_u32(st.multiple_op_data[i]);
                }
                true
            }
        }
    }

    /// AsmJit's `EmitOp_DispImm`: validates and packs the displacement
    /// immediate described by `st.offset_format` into the opcode, then emits
    /// the word.
    fn emit_disp_imm(&mut self, st: &mut A64EmitState) {
        if (st.offset_value & ((1 << st.offset_format.imm_discard_lsb()) - 1)) != 0 {
            self.last_error = Some(AsmError::InvalidOperand);
            return;
        }

        let disp_imm64 = (st.offset_value as i64) >> st.offset_format.imm_discard_lsb() as i64;
        let disp_imm32 = (disp_imm64 & ((1 << st.offset_format.imm_bit_count()) - 1)) as u32;

        match st.offset_format.typ() {
            OffsetType::SignedOffset => {
                st.opcode
                    .add_imm(disp_imm32 as _, st.offset_format.imm_bit_shift() as _);
                self.buffer.write_u32(st.opcode.get());
            }

            _ => {
                let imm_lo = disp_imm32 & 0x3;
                let imm_hi = disp_imm32 >> 2;
                st.opcode.add_imm(imm_lo, 29);
                st.opcode.add_imm(imm_hi, 5);
                self.buffer.write_u32(st.opcode.get());
            }
        }
    }

    /// AsmJit's `EmitOp_Rel`: resolves a label or immediate pc-relative
    /// operand into `st.offset_value` (recording a label use for unbound
    /// labels) and emits. Returns `false` when `st.rm_rel` is not relative.
    fn emit_rel(&mut self, st: &mut A64EmitState) -> bool {
        if st.rm_rel.is_label() || (st.rm_rel.is_mem() && st.rm_rel.as_::<Mem>().has_base_label()) {
            let label_id;
            let mut label_offset = 0;

            if st.rm_rel.is_label() {
                label_id = st.rm_rel.as_::<Label>().id();
            } else {
                label_id = st.rm_rel.as_::<Mem>().base_id();
                label_offset = st.rm_rel.as_::<Mem>().offset();
            }

            if self.buffer.is_bound(Label::from_id(label_id)) {
                st.offset_value = self.buffer.label_offset(Label::from_id(label_id)) as i64
                    + label_offset
                    - self.buffer.cur_offset() as i64;
                self.emit_disp_imm(st);
            } else {
                let offset = self.buffer.cur_offset();
                self.buffer.use_label_at_offset(
                    offset,
                    Label::from_id(label_id),
                    match st.offset_format.typ() {
                        OffsetType::Adrp => LabelUse::A64Adrp21,
                        OffsetType::Adr => LabelUse::A64Adr21,
                        OffsetType::Ldr => LabelUse::A64Ldr19,
                        OffsetType::SignedOffset => {
                            if st.offset_format.imm_bit_count() == 26 {
                                LabelUse::A64Branch26
                            } else if st.offset_format.imm_bit_count() == 19 {
                                LabelUse::A64Branch19
                            } else if st.offset_format.imm_bit_count() == 14 {
                                LabelUse::A64Branch14
                            } else {
                                panic!("Invalid offset format for label use")
                            }
                        }
                    },
                );

                self.buffer.write_u32(st.opcode.get());
            }

            return true;
        }

        if st.rm_rel.is_imm() {
            let target_offset = st.rm_rel.as_::<Imm>().value() as u64;
            let mut pc = self.buffer.cur_offset() as u64 + 4;
            if st.offset_format.typ() == OffsetType::Adrp {
                pc &= !(4096 - 1);
            }
            st.offset_value = target_offset as i64 - pc as i64;
            self.emit_disp_imm(st);
            return true;
        }

        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct Opc(pub(crate) u32);

impl Opc {
    const N: u32 = 1 << 2;
    const Q: u32 = 1 << 30;
    const X: u32 = 1 << 31;

    pub fn reset(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub const fn has_q(&self) -> bool {
        (self.0 & Self::Q) != 0
    }
    pub const fn has_x(&self) -> bool {
        (self.0 & Self::X) != 0
    }

    pub fn add_imm(&mut self, value: u32, bit_index: u32) -> &mut Self {
        self.0 |= value << bit_index;
        self
    }

    pub fn xor_imm(&mut self, value: u32, bit_index: u32) -> &mut Self {
        self.0 ^= value << bit_index;
        self
    }

    pub fn add_if(&mut self, condition: bool, value: u32, bit_index: u32) -> &mut Self {
        if condition {
            self.0 |= value << bit_index;
        }
        self
    }

    pub fn add_logical_imm(&mut self, logical_imm: &LogicalImm) -> &mut Self {
        self.add_imm(logical_imm.n, 22)
            .add_imm(logical_imm.s, 10)
            .add_imm(logical_imm.r, 16);
        self
    }

    pub fn add_reg(&mut self, id: u32, bit_index: u32) -> &mut Self {
        self.0 |= (id & 31) << bit_index;
        self
    }
}

impl core::ops::BitOr<u32> for Opc {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl core::ops::BitOrAssign<u32> for Opc {
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs;
    }
}

impl core::ops::BitAnd<u32> for Opc {
    type Output = Self;

    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl core::ops::BitAndAssign<u32> for Opc {
    fn bitand_assign(&mut self, rhs: u32) {
        self.0 &= rhs;
    }
}

impl core::ops::Not for Opc {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl core::ops::BitXor<u32> for Opc {
    type Output = Self;

    fn bitxor(self, rhs: u32) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl core::ops::BitXorAssign<u32> for Opc {
    fn bitxor_assign(&mut self, rhs: u32) {
        self.0 ^= rhs;
    }
}

impl core::ops::Shl<u32> for Opc {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl core::ops::ShlAssign<u32> for Opc {
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl core::ops::Shr<u32> for Opc {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl core::ops::ShrAssign<u32> for Opc {
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LogicalImm {
    pub n: u32,
    pub s: u32,
    pub r: u32,
}

pub(crate) fn check_gp_type(op: &Operand, allowed: u32) -> bool {
    let typ = op.as_::<Reg>().typ() as u32;
    let mask = allowed << RegType::Gp32 as u32;
    bit_test(mask, typ)
}

pub(crate) fn check_gp_typex(op: &Operand, allowed: u32, x: &mut u32) -> bool {
    let typ = op.as_::<Reg>().typ() as u32;
    *x = typ.wrapping_sub(RegType::Gp32 as u32) & allowed;
    bit_test(allowed << RegType::Gp32 as u32, typ)
}

pub(crate) fn check_gp_typex2(o0: &Operand, o1: &Operand, allowed: u32, x: &mut u32) -> bool {
    check_gp_typex(o0, allowed, x) && check_signature!(o0, o1)
}

pub(crate) fn check_gp_typex3(
    o0: &Operand,
    o1: &Operand,
    o2: &Operand,
    allowed: u32,
    x: &mut u32,
) -> bool {
    check_gp_typex(o0, allowed, x) && check_signature!(o0, o1, o2)
}

pub(crate) fn check_gp_id(op: &Operand, hi_id: u32) -> bool {
    op.id() < 31 || op.id() == hi_id
}

pub(crate) fn check_gp_id2(o0: &Operand, o1: &Operand, hi_id: u32) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    (id0 < 31 || id0 == hi_id) && (id1 < 31 || id1 == hi_id)
}

pub(crate) fn check_gp_id3(o0: &Operand, o1: &Operand, o2: &Operand, hi_id: u32) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    let id2 = o2.id();
    (id0 < 31 || id0 == hi_id) && (id1 < 31 || id1 == hi_id) && (id2 < 31 || id2 == hi_id)
}

pub(crate) fn check_vec_id(o0: &Operand) -> bool {
    let id = o0.id();
    id < 31
}

pub(crate) fn check_vec_id2(o0: &Operand, o1: &Operand) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    id0 < 31 && id1 < 31
}

pub(crate) fn check_vec_id3(o0: &Operand, o1: &Operand, o2: &Operand) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    let id2 = o2.id();
    id0 < 31 && id1 < 31 && id2 < 31
}

pub(crate) fn bit_test(value: u32, n: u32) -> bool {
    n < 32 && value & (1 << n) != 0
}

pub(crate) fn encode_mov_sequence64(out: &mut [u32; 4], mut imm: u64, rd: u32, x: u32) -> usize {
    const MOVZ: u32 = 0b11010010100000000000000000000000;
    const MOVN: u32 = 0b10010010100000000000000000000000;
    const MOVK: u32 = 0b11110010100000000000000000000000;

    if imm <= 0xFFFFFFFF {
        return encode_mov_sequence32(out, imm as u32, rd, x);
    }

    let zhw = count_zero_half_words_64(imm);
    let ohw = count_zero_half_words_64(!imm);

    if zhw >= ohw {
        let mut op = MOVZ;
        let mut count = 0;
        for hw_index in 0..4 {
            let hw_imm = (imm & 0xFFFF) as u32;
            if hw_imm == 0 {
                imm >>= 16;
                continue;
            }
            out[count] = op | (hw_index << 21) | (hw_imm << 5) | rd;
            op = MOVK;
            count += 1;

            imm >>= 16;
        }

        return count;
    }

    let mut op = MOVN;
    let mut count = 0;
    let mut neg_mask = 0xFFFF;

    for hw_index in 0..4 {
        let hw_imm = (imm & 0xFFFF) as u32;
        if hw_imm == 0xFFFF {
            imm >>= 16;
            continue;
        }

        out[count] = op | (hw_index << 21) | ((hw_imm ^ neg_mask) << 5) | rd;
        count += 1;
        op = MOVK;
        neg_mask = 0;
        imm >>= 16;
    }

    count
}

pub(crate) fn encode_mov_sequence32(out: &mut [u32], imm: u32, rd: u32, x: u32) -> usize {
    let movz = 0b11010010100000000000000000000000 | (x << 31);
    let movn = 0b10010010100000000000000000000000;
    let movk = 0b11110010100000000000000000000000;
    if (imm & 0xFFFF0000) == 0 {
        out[0] = movz | (0 << 21) | ((imm & 0xffff) << 5) | rd;
        return 1;
    }

    if (imm & 0xFFFF0000) == 0xFFFF0000 {
        out[0] = movn | (0 << 21) | ((!imm & 0xFFFF) << 5) | rd;
        return 1;
    }

    if (imm & 0x0000FFFF) == 0x00000000 {
        out[0] = movz | (1 << 21) | ((imm >> 16) << 5) | rd;
        return 1;
    }

    if (imm & 0x0000FFFF) == 0x0000FFFF {
        out[0] = movn | (1 << 21) | ((!imm >> 16) << 5) | rd;
        return 1;
    }

    out[0] = movz | (0 << 21) | ((imm & 0xFFFF) << 5) | rd;
    out[1] = movk | (1 << 21) | ((imm >> 16) << 5) | rd;
    return 2;
}

pub const fn count_zero_half_words_64(imm: u64) -> u32 {
    let mut count = 0;
    if (imm & 0x000000000000FFFF) == 0 {
        count += 1;
    }
    if (imm & 0x00000000FFFF0000) == 0 {
        count += 1;
    }
    if (imm & 0x0000FFFF00000000) == 0 {
        count += 1;
    }
    if (imm & 0xFFFF000000000000) == 0 {
        count += 1;
    }
    count
}

/// Encodes the given `imm` value of the given `width` to a logical immediate value represented as N, S, and R fields
/// and writes these fields to `out`.
///
/// Encoding Table:
///
/// ```text
/// +---+--------+--------+------+
/// | N |  ImmS  |  ImmR  | Size |
/// +---+--------+--------+------+
/// | 1 | ssssss | rrrrrr |  64  |
/// | 0 | 0sssss | .rrrrr |  32  |
/// | 0 | 10ssss | ..rrrr |  16  |
/// | 0 | 110sss | ...rrr |  8   |
/// | 0 | 1110ss | ....rr |  4   |
/// | 0 | 11110s | .....r |  2   |
/// +---+--------+--------+------+
/// ```
pub const fn encode_logical_imm(mut imm: u64, mut width: u32) -> Option<LogicalImm> {
    loop {
        width /= 2;
        let mask = (1u64 << width) - 1;
        if (imm & mask) != (imm >> width) & mask {
            width *= 2;
            break;
        }
        if width <= 2 {
            break;
        }
    }

    let width_mask = lsb_mask::<u64>(width);
    imm &= width_mask;

    // Patterns of all zeros and all ones are not encodable.
    if imm == 0 || width_mask == imm {
        return None;
    }

    // Inspect the pattern and get the most important bit indexes.
    //
    //         o_index <-+      +-> z_index
    //                  |      |
    // |..zeros..|o_count|z_count|..ones..|
    // |000000000|111111|000000|11111111|
    let z_index = (!imm).trailing_zeros();
    let z_imm = imm ^ ((1u64 << z_index) - 1);
    let z_count = (if z_imm != 0 {
        z_imm.trailing_zeros()
    } else {
        width
    })
    .wrapping_sub(z_index);

    let o_index = z_index + z_count;
    let o_imm = !(z_imm ^ lsb_mask::<u64>(o_index));
    let o_count = (if o_imm != 0 {
        o_imm.trailing_zeros()
    } else {
        width
    })
    .wrapping_sub(o_index);

    let must_be_zero = o_imm ^ !lsb_mask::<u64>((o_index + o_count) & 63);
    if must_be_zero != 0 || (z_index > 0 && width.wrapping_sub(o_index + o_count) != 0) {
        return None;
    }

    Some(LogicalImm {
        n: if width == 64 { 1 } else { 0 },
        s: (o_count + z_index).wrapping_sub(1) | 0u32.wrapping_sub(width * 2) & 0x3f,
        r: width.wrapping_sub(o_index),
    })
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub(crate) enum OffsetType {
    SignedOffset,
    Adr,
    Adrp,
    Ldr,
}

impl TryFrom<u8> for OffsetType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SignedOffset),
            1 => Ok(Self::Adr),
            2 => Ok(Self::Adrp),
            3 => Ok(Self::Ldr),
            _ => Err(()),
        }
    }
}

pub(crate) struct OffsetFormat {
    pub(crate) typ: OffsetType,
    pub(crate) flags: u8,
    pub(crate) region_size: u8,
    pub(crate) value_size: u8,
    pub(crate) value_offset: u8,
    pub(crate) imm_bit_count: u8,
    pub(crate) imm_bit_shift: u8,
    pub(crate) imm_discard_lsb: u8,
}

impl OffsetFormat {
    pub const fn new(
        typ: OffsetType,
        flags: u8,
        region_size: u8,
        value_size: u8,
        value_offset: u8,
        imm_bit_count: u8,
        imm_bit_shift: u8,
        imm_discard_lsb: u8,
    ) -> Self {
        Self {
            typ,
            flags,
            region_size,
            value_size,
            value_offset,
            imm_bit_count,
            imm_bit_shift,
            imm_discard_lsb,
        }
    }

    pub fn reset_to_imm_type(
        &mut self,
        typ: OffsetType,
        value_size: usize,
        imm_bit_shift: u32,
        imm_bit_count: u32,
        imm_discard_lsb: u32,
    ) {
        self.typ = typ;
        self.value_size = value_size as u8;
        self.region_size = value_size as u8;
        self.imm_bit_shift = imm_bit_shift as u8;
        self.imm_bit_count = imm_bit_count as u8;
        self.imm_discard_lsb = imm_discard_lsb as u8;
        self.flags = 0;
        self.value_offset = 0;
    }

    fn set_region(&mut self, region_size: usize, value_offset: usize) {
        self.region_size = region_size as u8;
        self.value_offset = value_offset as u8;
    }

    fn set_leading_and_trailing_size(&mut self, leading_size: usize, trailing_size: usize) {
        self.region_size = (leading_size + trailing_size + self.value_size as usize) as u8;
        self.value_offset = leading_size as u8;
    }

    fn typ(&self) -> OffsetType {
        self.typ
    }

    fn flags(&self) -> u8 {
        self.flags
    }

    fn region_size(&self) -> usize {
        self.region_size as usize
    }

    fn value_size(&self) -> usize {
        self.value_size as usize
    }

    fn value_offset(&self) -> usize {
        self.value_offset as usize
    }

    fn imm_bit_count(&self) -> usize {
        self.imm_bit_count as usize
    }

    fn imm_bit_shift(&self) -> usize {
        self.imm_bit_shift as usize
    }

    fn imm_discard_lsb(&self) -> usize {
        self.imm_discard_lsb as usize
    }
}

pub(crate) const fn lsb_mask<T>(n: u32) -> u64 {
    if size_of::<T>() < size_of::<u64>() {
        (1 << n) - 1
    } else {
        if n != 0 {
            (!0u64).wrapping_shr((size_of::<T>() as u32 * 8) - n)
        } else {
            0
        }
    }
}

pub(crate) const fn cond_code_to_opcode_field(cond: u32) -> u32 {
    (cond.wrapping_sub(2)) & 0xf
}

pub(crate) const fn is_byte_mask_imm(imm: u64) -> bool {
    let mask = 0x0101010101010101 & u64::MAX;
    imm == (imm & mask) * 255
}

pub(crate) const fn encode_imm64_byte_mask_to_imm8(imm: u64) -> u32 {
    (((imm >> (7  - 0)) & 0b00000011) | // [.......G|H.......]
     ((imm >> (23 - 2)) & 0b00001100) | // [.......E|F.......]
     ((imm >> (39 - 4)) & 0b00110000) | // [.......C|D.......]
     ((imm >> (55 - 6)) & 0b11000000)) as u32
}

macro_rules! is_fp_imm8_generic {
    ($t: ty: $val: expr, $num_b_bits: expr, $num_cdefgh_bits: expr, $num_zero_bits: expr) => {{
        let all_bs_mask = lsb_mask::<u32>($num_b_bits);
        let b0_pattern = 1u32 << ($num_b_bits - 1);
        let b1_pattern = all_bs_mask as u32 ^ b0_pattern;

        let imm_z = $val & lsb_mask::<$t>($num_zero_bits as _) as $t;
        let imm_b = ($val >> ($num_zero_bits + $num_cdefgh_bits)) as u32 & all_bs_mask as u32;
        imm_z == 0 && (imm_b == b0_pattern || imm_b == b1_pattern)
    }};
}

pub const fn is_fp16_imm8(val: u32) -> bool {
    is_fp_imm8_generic!(u32: val, 3, 6, 6)
}

pub const fn is_fp32_imm8(val: u32) -> bool {
    is_fp_imm8_generic!(u32: val, 6, 6, 19)
}

pub const fn is_fp64_imm8(val: u64) -> bool {
    is_fp_imm8_generic!(u64: val, 9, 6, 48)
}

macro_rules! encode_fp_to_imm8_generic {
    ($t: ty: $val: expr, $num_b_bits: expr, $num_cdefgh_bits: expr, $num_zero_bits: expr) => {{
        let bits = ($val >> $num_zero_bits) as u32;
        ((bits >> ($num_b_bits + $num_cdefgh_bits - 7)) & 0x80) | (bits & 0x7f)
    }};
}

pub const fn encode_fp64_to_imm8(val: u64) -> u32 {
    encode_fp_to_imm8_generic!(u64: val, 9, 6, 48)
}

pub(crate) fn pick_fp_opcode(
    reg: Vec,
    s_op: u32,
    s_hf: u32,
    v_op: u32,
    v_hf: u32,
    sz_out: &mut u32,
) -> Option<Opc> {
    const QBIT_INDEX: usize = 30;

    struct EncodeFpOpcodeBits {
        size_mask: u32,
        mask: [u32; 3],
    }

    static SZ_BITS_TABLE: [EncodeFpOpcodeBits; 6] = [
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1),
            mask: [0, 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [0, 0, 0],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 23 | 1 << 22, 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [(1 << 22) | (1 << 20) | (1 << 19), 0, 0],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 22 | (1 << 21) | (1 << 15) | (1 << 14), 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 23, 0, 1 << 22],
        },
    ];

    let mut op = Opc(0);
    if !reg.has_element_type() {
        // Scalar operation [HSD].
        let sz = (reg.typ() as u32).wrapping_sub(RegType::Vec16 as u32);
        if sz > 2 || !bit_test32(SZ_BITS_TABLE[s_hf as usize].size_mask, sz) {
            return None;
        }

        op.reset(SZ_BITS_TABLE[s_hf as usize].mask[sz as usize] ^ s_op);
        *sz_out = sz;

        return (s_op != 0).then_some(op);
    } else {
        // Vector operation [HSD].
        let q = (reg.typ() as u32).wrapping_sub(RegType::Vec64 as u32);
        let sz = (reg.element_type() as u32).wrapping_sub(VecElementType::H as u32);

        if q > 1 || sz > 2 || !bit_test32(SZ_BITS_TABLE[v_hf as usize].size_mask, sz) {
            return None;
        }

        op.reset(SZ_BITS_TABLE[v_hf as usize].mask[sz as usize] ^ (v_op | (q << QBIT_INDEX)));
        *sz_out = sz;
        return (v_op != 0).then_some(op);
    }
}

pub(crate) const fn bit_test32(value: u32, n: u32) -> bool {
    n < 32 && value & (1 << n) != 0
}

pub(crate) struct SizeOpTable {
    pub(crate) array: [SizeOp; ((RegType::Vec128 as usize - RegType::Vec8 as usize + 1) + 1) * 40],
}

impl SizeOpTable {
    const fn len() -> usize {
        ((RegType::Vec128 as usize - RegType::Vec8 as usize + 1) + 1) * 40
    }
    pub(crate) const fn bin() -> Self {
        let mut i = 0;
        let mut array = [SizeOp::new(SizeOp::K_INVALID); Self::len()];
        while i < Self::len() {
            array[i] = Self::bin_at(i);
            i += 1;
        }
        Self { array }
    }

    pub(crate) const fn any() -> Self {
        let mut i = 0;
        let mut array = [SizeOp::new(SizeOp::K_INVALID); Self::len()];
        while i < Self::len() {
            array[i] = Self::any_at(i);
            i += 1;
        }
        Self { array }
    }

    const fn bin_at(x: usize) -> SizeOp {
        if x == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
            | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else {
            SizeOp::new(SizeOp::K_INVALID)
        }
    }

    const fn any_at(x: usize) -> SizeOp {
        if x == (((RegType::Vec8 as usize - RegType::Vec8 as usize) << 3)
            | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00_S)
        } else if x
            == (((RegType::Vec16 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K01_S)
        } else if x
            == (((RegType::Vec32 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K10_S)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K11_S)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::H as usize)
        {
            SizeOp::new(SizeOp::K01)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::H as usize)
        {
            SizeOp::new(SizeOp::K01_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::S as usize)
        {
            SizeOp::new(SizeOp::K10)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::S as usize)
        {
            SizeOp::new(SizeOp::K10_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::D as usize)
        {
            SizeOp::new(SizeOp::K11_S)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::D as usize)
        {
            SizeOp::new(SizeOp::K11_Q)
        } else {
            SizeOp::new(SizeOp::K_INVALID)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct SizeOp(u8);

impl SizeOp {
    pub const fn new(val: u8) -> Self {
        Self(val)
    }

    pub(crate) const K128_BIT_SHIFT: u8 = 0;
    pub(crate) const K_SCALAR_SHIFT: u8 = 1;
    pub(crate) const K_SIZE_SHIFT: u8 = 2;

    pub(crate) const K_Q: u8 = 1u8 << Self::K128_BIT_SHIFT;
    pub(crate) const K_S: u8 = 1u8 << Self::K_SCALAR_SHIFT;

    pub(crate) const K00: u8 = 0 << Self::K_SIZE_SHIFT;
    pub(crate) const K01: u8 = 1 << Self::K_SIZE_SHIFT;
    pub(crate) const K10: u8 = 2 << Self::K_SIZE_SHIFT;
    pub(crate) const K11: u8 = 3 << Self::K_SIZE_SHIFT;

    pub(crate) const K00_Q: u8 = Self::K00 | Self::K_Q;
    pub(crate) const K01_Q: u8 = Self::K01 | Self::K_Q;
    pub(crate) const K10_Q: u8 = Self::K10 | Self::K_Q;
    pub(crate) const K11_Q: u8 = Self::K11 | Self::K_Q;

    pub(crate) const K00_S: u8 = Self::K00 | Self::K_S;
    pub(crate) const K01_S: u8 = Self::K01 | Self::K_S;
    pub(crate) const K10_S: u8 = Self::K10 | Self::K_S;
    pub(crate) const K11_S: u8 = Self::K11 | Self::K_S;

    pub(crate) const K_INVALID: u8 = 0xFF;

    pub(crate) const K_SZ_Q: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_Q;
    pub(crate) const K_SZ_S: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_S;
    pub(crate) const K_SZ_QS: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_Q | Self::K_S;

    pub(crate) const fn is_valid(self) -> bool {
        self.0 != Self::K_INVALID
    }

    pub(crate) const fn make_invalid(&mut self) {
        self.0 = Self::K_INVALID;
    }

    pub(crate) const fn q(&self) -> u32 {
        (self.0 >> Self::K128_BIT_SHIFT) as u32 & 1
    }

    pub(crate) const fn qs(&self) -> u32 {
        (((self.0 >> Self::K128_BIT_SHIFT) as u32) | ((self.0 >> Self::K_SCALAR_SHIFT) as u32)) & 1
    }

    pub(crate) const fn scalar(&self) -> u32 {
        (self.0 >> Self::K_SCALAR_SHIFT) as u32 & 1
    }

    pub(crate) const fn size(&self) -> u32 {
        (self.0 >> Self::K_SIZE_SHIFT) as u32 & 0x3
    }

    pub(crate) const fn decrement_size(&mut self) {
        self.0 = (self.0 as u32 - (1u32 << Self::K_SIZE_SHIFT)) as u8;
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct SizeOpMap {
    pub(crate) table_id: u8,
    pub(crate) size_op_mask: u8,
    pub(crate) accept_mask: u16,
}

pub(crate) const fn significant_simd_op<'a>(
    o0: &'a Operand,
    o1: &'a Operand,
    inst_flags: u32,
) -> &'a Operand {
    if (inst_flags & InstFlag::Long as u32) == 0 {
        o0
    } else {
        o1
    }
}

/// AsmJit's `match_signature`: operands must share a signature, except the
/// first pair of Long/Narrow instructions (wide/narrow destination), which
/// AsmJit deliberately does not validate ("TODO: [ARM] Something smart to
/// validate this").
pub(crate) fn match_signature2(o0: &Operand, o1: &Operand, inst_flags: u32) -> bool {
    if inst_flags & (InstFlag::Long as u32 | InstFlag::Narrow as u32) == 0 {
        o0.signature() == o1.signature()
    } else {
        true
    }
}

/// AsmJit's 3-operand `match_signature`.
pub(crate) fn match_signature3(o0: &Operand, o1: &Operand, o2: &Operand, inst_flags: u32) -> bool {
    match_signature2(o0, o1, inst_flags) && o1.signature() == o2.signature()
}

/// AsmJit's 4-operand `match_signature`.
pub(crate) fn match_signature4(
    o0: &Operand,
    o1: &Operand,
    o2: &Operand,
    o3: &Operand,
    inst_flags: u32,
) -> bool {
    match_signature2(o0, o1, inst_flags)
        && o1.signature() == o2.signature()
        && o2.signature() == o3.signature()
}

pub(crate) const fn element_type_to_size_op(
    vec_op_type: u32,
    reg_type: RegType,
    element_type: VecElementType,
) -> SizeOp {
    let map = &SIZE_OP_MAP[vec_op_type as usize];
    let table = &SIZE_OP_TABLE[map.table_id as usize];

    // Mirrors AsmJit's `min(diff(reg_type, kVec8), diff(kVec128, kVec8) + 1)`:
    // out-of-range register types clamp to the invalid tail of the table.
    let a = (reg_type as usize).wrapping_sub(RegType::Vec8 as usize);
    let b = RegType::Vec128 as usize - RegType::Vec8 as usize;

    let clamped = if a < b + 1 { a } else { b + 1 };
    let index = (clamped << 3) | (element_type as usize);
    let op = table.array[index];
    let mut modified_op = SizeOp::new(op.0 & map.size_op_mask);

    if !bit_test32(map.accept_mask as u32, op.0 as u32) {
        modified_op.make_invalid();
    }

    modified_op
}

pub(crate) struct LMHImm {
    pub(crate) lm: u32,
    pub(crate) h: u32,
    pub(crate) max_rm_id: u32,
}

pub(crate) fn encode_lmh(size_field: u32, element_index: u32, out: &mut LMHImm) -> bool {
    if size_field != 1 && size_field != 2 {
        return false;
    }

    let h_shift = 3u32.saturating_sub(size_field);
    let lm_shift = size_field.saturating_sub(1u32);
    let max_element_index = 15u32 >> size_field;

    out.h = element_index >> h_shift;
    out.lm = (element_index << lm_shift) & 0x3u32;
    out.max_rm_id = (8u32 << size_field).saturating_sub(1);

    element_index <= max_element_index
}
