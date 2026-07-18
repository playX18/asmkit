//! X86 opcode bit-pack model (port of AsmJit's `x86opcode_p.h`).
//!
//! The instruction database stores opcodes as packed `u32` values; the shifts and masks here
//! are defined exactly as in AsmJit so REX, VEX, and EVEX prefixes can be constructed with a
//! few arithmetic ops. Do not reorder or renumber any field — the encoder depends on the
//! exact layout, and `InstOptions` bits 24–27 are binary-compatible with the REX field.
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

use crate::core::globals::InstOptions;

/// Packed opcode value as stored in the X86 instruction database.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Opcode(pub u32);

#[allow(non_upper_case_globals)]
impl Opcode {
    // MM & VEX & EVEX & XOP
    // ---------------------

    // Two meanings: part of a legacy opcode (prefix bytes) or the `MMMMM` field of
    // VEX|EVEX|XOP instructions. [2:0] = 0F/0F38/0F3A + XOP/AVX/AVX512 MMM bits; [3] = XOP;
    // [4] = force EVEX.
    pub const MM_SHIFT: u32 = 8;
    pub const MM_MASK: u32 = 0x1F << Self::MM_SHIFT;
    pub const MM_00: u32 = 0x00 << Self::MM_SHIFT;
    pub const MM_0F: u32 = 0x01 << Self::MM_SHIFT;
    pub const MM_0F38: u32 = 0x02 << Self::MM_SHIFT;
    /// Described also as XOP.M3 in AMD manuals.
    pub const MM_0F3A: u32 = 0x03 << Self::MM_SHIFT;
    /// AsmJit way to describe 0F01 (never VEX/EVEX).
    pub const MM_0F01: u32 = 0x04 << Self::MM_SHIFT;
    pub const MM_MAP5: u32 = 0x05 << Self::MM_SHIFT;
    pub const MM_MAP6: u32 = 0x06 << Self::MM_SHIFT;

    pub const MM_XOP08: u32 = 0x08 << Self::MM_SHIFT;
    pub const MM_XOP09: u32 = 0x09 << Self::MM_SHIFT;
    pub const MM_XOP0A: u32 = 0x0A << Self::MM_SHIFT;

    pub const MM_IS_XOP_SHIFT: u32 = Self::MM_SHIFT + 3;
    pub const MM_IS_XOP: u32 = Self::MM_XOP08;

    /// Force 4-byte EVEX prefix.
    pub const MM_FORCE_EVEX: u32 = 0x10 << Self::MM_SHIFT;

    // FPU_2B — second opcode byte used by FPU
    // ---------------------------------------

    // Collides with 3 bits of MM and 5 bits of CDSHL/CDTT; fine as FPU and AVX-512 flags
    // are never used at the same time.
    pub const FPU_2B_SHIFT: u32 = 10;
    pub const FPU_2B_MASK: u32 = 0xFF << Self::FPU_2B_SHIFT;

    // CDSHL & CDTT — compressed displacement
    // --------------------------------------

    pub const CDSHL_SHIFT: u32 = 13;
    pub const CDSHL_MASK: u32 = 0x7 << Self::CDSHL_SHIFT;

    pub const CDSHL__: u32 = 0x0 << Self::CDSHL_SHIFT;
    pub const CDSHL_0: u32 = 0x0 << Self::CDSHL_SHIFT;
    pub const CDSHL_1: u32 = 0x1 << Self::CDSHL_SHIFT;
    pub const CDSHL_2: u32 = 0x2 << Self::CDSHL_SHIFT;
    pub const CDSHL_3: u32 = 0x3 << Self::CDSHL_SHIFT;
    pub const CDSHL_4: u32 = 0x4 << Self::CDSHL_SHIFT;
    pub const CDSHL_5: u32 = 0x5 << Self::CDSHL_SHIFT;

    /// Compressed displacement tuple-type (AsmJit-specific simplification).
    pub const CDTT_SHIFT: u32 = 16;
    pub const CDTT_MASK: u32 = 0x3 << Self::CDTT_SHIFT;
    /// Does nothing.
    pub const CDTT_NONE: u32 = 0x0 << Self::CDTT_SHIFT;
    /// Scales by LL (1x 2x 4x).
    pub const CDTT_BY_LL: u32 = 0x1 << Self::CDTT_SHIFT;
    /// Used to add 'W' to the shift.
    pub const CDTT_T1W: u32 = 0x2 << Self::CDTT_SHIFT;
    /// Special 'VMOVDDUP' case.
    pub const CDTT_DUP: u32 = 0x3 << Self::CDTT_SHIFT;

    // Aliases that match names used in instruction manuals.
    pub const CDTT__: u32 = Self::CDTT_NONE;
    pub const CDTT_FV: u32 = Self::CDTT_BY_LL;
    pub const CDTT_HV: u32 = Self::CDTT_BY_LL;
    pub const CDTT_QV: u32 = Self::CDTT_BY_LL;
    pub const CDTT_FVM: u32 = Self::CDTT_BY_LL;
    pub const CDTT_T1S: u32 = Self::CDTT_NONE;
    pub const CDTT_T1F: u32 = Self::CDTT_NONE;
    pub const CDTT_T2: u32 = Self::CDTT_NONE;
    pub const CDTT_T4: u32 = Self::CDTT_NONE;
    pub const CDTT_T8: u32 = Self::CDTT_NONE;
    pub const CDTT_HVM: u32 = Self::CDTT_BY_LL;
    pub const CDTT_QVM: u32 = Self::CDTT_BY_LL;
    pub const CDTT_OVM: u32 = Self::CDTT_BY_LL;
    pub const CDTT_128: u32 = Self::CDTT_NONE;

    // `O` field in ModR/M (??:xxx:???)
    // --------------------------------

    pub const MOD_O_SHIFT: u32 = 18;
    pub const MOD_O_MASK: u32 = 0x7 << Self::MOD_O_SHIFT;

    // `RM` field in ModR/M (??:???:xxx)
    // ---------------------------------

    // Only used by a few instructions where both ModR/M fields are part of the opcode.
    pub const MOD_RM_SHIFT: u32 = 13;
    pub const MOD_RM_MASK: u32 = 0x7 << Self::MOD_RM_SHIFT;

    // `PP` field
    // ----------

    // Two meanings: "PP" field in AVX/XOP/AVX-512 instructions, or a mandatory prefix in
    // legacy encodings. AsmJit extends storage by 1 bit used to emit the 9B FPU prefix.
    pub const PP_SHIFT: u32 = 21;
    /// PP field mask used by VEX/EVEX.
    pub const PP_VEX_MASK: u32 = 0x03 << Self::PP_SHIFT;
    /// Mask used by EMIT_PP, also includes '0x9B'.
    pub const PP_FPU_MASK: u32 = 0x07 << Self::PP_SHIFT;
    pub const PP_00: u32 = 0x00 << Self::PP_SHIFT;
    pub const PP_66: u32 = 0x01 << Self::PP_SHIFT;
    pub const PP_F3: u32 = 0x02 << Self::PP_SHIFT;
    pub const PP_F2: u32 = 0x03 << Self::PP_SHIFT;
    /// AsmJit specific to emit FPU's '9B' byte.
    pub const PP_9B: u32 = 0x07 << Self::PP_SHIFT;

    // REX|VEX|EVEX B|X|R|W bits
    // -------------------------

    // REX.[B|X|R] are never stored within the opcode itself; they are added dynamically by
    // the encoder. These must be binary compatible with `InstOptions` bits 24–27.
    pub const REX_SHIFT: u32 = 24;
    pub const REX_MASK: u32 = 0x0F << Self::REX_SHIFT;
    /// Never stored in DB, used by encoder.
    pub const B: u32 = 0x01 << Self::REX_SHIFT;
    /// Never stored in DB, used by encoder.
    pub const X: u32 = 0x02 << Self::REX_SHIFT;
    /// Never stored in DB, used by encoder.
    pub const R: u32 = 0x04 << Self::REX_SHIFT;
    pub const W: u32 = 0x08 << Self::REX_SHIFT;
    pub const W_SHIFT: u32 = Self::REX_SHIFT + 3;

    // EVEX.W field
    // ------------

    pub const EVEX_W_SHIFT: u32 = 28;
    pub const EVEX_W_MASK: u32 = 1 << Self::EVEX_W_SHIFT;

    // `L` or `LL` field in AVX/XOP/AVX-512
    // ------------------------------------

    pub const LL_SHIFT: u32 = 29;
    pub const LL_MASK: u32 = 0x3 << Self::LL_SHIFT;

    // Opcode combinations
    // -------------------

    pub const K0: u32 = 0;
    pub const K000000: u32 = Self::PP_00 | Self::MM_00;
    pub const K000F00: u32 = Self::PP_00 | Self::MM_0F;
    pub const K000F01: u32 = Self::PP_00 | Self::MM_0F01;
    /// '0F0F' - 3DNOW, equal to 0x0F, must have special encoding to take effect.
    pub const K000F0F: u32 = Self::PP_00 | Self::MM_0F;
    pub const K000F38: u32 = Self::PP_00 | Self::MM_0F38;
    pub const K000F3A: u32 = Self::PP_00 | Self::MM_0F3A;
    pub const K00MAP5: u32 = Self::PP_00 | Self::MM_MAP5;
    pub const K00MAP6: u32 = Self::PP_00 | Self::MM_MAP6;
    pub const K660000: u32 = Self::PP_66 | Self::MM_00;
    pub const K660F00: u32 = Self::PP_66 | Self::MM_0F;
    pub const K660F01: u32 = Self::PP_66 | Self::MM_0F01;
    pub const K660F38: u32 = Self::PP_66 | Self::MM_0F38;
    pub const K660F3A: u32 = Self::PP_66 | Self::MM_0F3A;
    pub const K66MAP5: u32 = Self::PP_66 | Self::MM_MAP5;
    pub const K66MAP6: u32 = Self::PP_66 | Self::MM_MAP6;
    pub const KF20000: u32 = Self::PP_F2 | Self::MM_00;
    pub const KF20F00: u32 = Self::PP_F2 | Self::MM_0F;
    pub const KF20F01: u32 = Self::PP_F2 | Self::MM_0F01;
    pub const KF20F38: u32 = Self::PP_F2 | Self::MM_0F38;
    pub const KF20F3A: u32 = Self::PP_F2 | Self::MM_0F3A;
    pub const KF2MAP5: u32 = Self::PP_F2 | Self::MM_MAP5;
    pub const KF2MAP6: u32 = Self::PP_F2 | Self::MM_MAP6;
    pub const KF30000: u32 = Self::PP_F3 | Self::MM_00;
    pub const KF30F00: u32 = Self::PP_F3 | Self::MM_0F;
    pub const KF30F01: u32 = Self::PP_F3 | Self::MM_0F01;
    pub const KF30F38: u32 = Self::PP_F3 | Self::MM_0F38;
    pub const KF30F3A: u32 = Self::PP_F3 | Self::MM_0F3A;
    pub const KF3MAP5: u32 = Self::PP_F3 | Self::MM_MAP5;
    pub const KF3MAP6: u32 = Self::PP_F3 | Self::MM_MAP6;
    pub const KFPU_00: u32 = Self::PP_00 | Self::MM_00;
    pub const KFPU_9B: u32 = Self::PP_9B | Self::MM_00;
    pub const KXOP_M8: u32 = Self::PP_00 | Self::MM_XOP08;
    pub const KXOP_M9: u32 = Self::PP_00 | Self::MM_XOP09;
    pub const KXOP_MA: u32 = Self::PP_00 | Self::MM_XOP0A;
}

impl Opcode {
    pub const fn get(self) -> u32 {
        self.0
    }

    pub const fn has_w(self) -> bool {
        (self.0 & Self::W) != 0
    }

    pub const fn has_66h(self) -> bool {
        (self.0 & Self::PP_66) != 0
    }

    pub fn add(&mut self, x: u32) {
        self.0 += x;
    }

    pub fn add_66h(&mut self) {
        self.0 |= Self::PP_66;
    }

    pub fn add_66h_if(&mut self, exp: bool) {
        self.0 |= (exp as u32) << Self::PP_SHIFT;
    }

    pub fn add_66h_by_size(&mut self, size: u32) {
        self.add_66h_if(size == 2);
    }

    pub fn add_w(&mut self) {
        self.0 |= Self::W;
    }

    pub fn add_w_if(&mut self, exp: bool) {
        self.0 |= (exp as u32) << Self::W_SHIFT;
    }

    pub fn add_w_by_size(&mut self, size: u32) {
        self.add_w_if(size == 8);
    }

    pub fn add_prefix_by_size(&mut self, size: u32) {
        const MASK: [u32; 16] = [
            0,
            0,
            Opcode::PP_66, // #2 -> 66H
            0,
            0,
            0,
            0,
            0,
            Opcode::W, // #8 -> REX.W
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        self.0 |= MASK[(size & 0xF) as usize];
    }

    pub fn add_arith_by_size(&mut self, size: u32) {
        const MASK: [u32; 16] = [
            0,
            0,
            1 | Opcode::PP_66, // #2 -> NOT_BYTE_OP(1) and 66H
            0,
            1, // #4 -> NOT_BYTE_OP(1)
            0,
            0,
            0,
            1 | Opcode::W, // #8 -> NOT_BYTE_OP(1) and REX.W
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        self.0 |= MASK[(size & 0xF) as usize];
    }

    pub fn force_evex(&mut self) {
        self.0 |= Self::MM_FORCE_EVEX;
    }

    pub fn force_evex_if(&mut self, exp: bool) {
        self.0 |= (exp as u32) << Self::MM_FORCE_EVEX.trailing_zeros();
    }

    /// Extracts the `O` field (R) from the opcode (specified as /0..7 in manuals).
    pub const fn extract_mod_o(self) -> u32 {
        (self.0 >> Self::MOD_O_SHIFT) & 0x07
    }

    /// Extracts the `RM` field (usually specified as another opcode value).
    pub const fn extract_mod_rm(self) -> u32 {
        (self.0 >> Self::MOD_RM_SHIFT) & 0x07
    }

    /// Extracts the REX prefix from the opcode combined with `options`.
    ///
    /// The REX field was designed so the shifted value forms a real REX prefix byte.
    pub const fn extract_rex(self, options: InstOptions) -> u32 {
        (self.0 | options.bits()) >> Self::REX_SHIFT
    }

    pub const fn extract_ll_mmmmm(self, options: InstOptions) -> u32 {
        let ll_mmmmm = self.0 & (Self::LL_MASK | Self::MM_MASK);
        let vex_evex = options.bits() & InstOptions::X86_EVEX.bits();
        (ll_mmmmm | vex_evex) >> Self::MM_SHIFT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_mod_fields() {
        // opcode with ModO=5, ModRM=2
        let opc = Opcode(0xFF | (5 << Opcode::MOD_O_SHIFT) | (2 << Opcode::MOD_RM_SHIFT));
        assert_eq!(opc.extract_mod_o(), 5);
        assert_eq!(opc.extract_mod_rm(), 2);
    }

    #[test]
    fn extract_rex_forms_prefix_byte() {
        // W bit set in opcode + R option from InstOptions → 0x4C.
        let opc = Opcode(Opcode::W);
        let options = InstOptions::X86_OP_CODE_R;
        assert_eq!(opc.extract_rex(options), 0x0C | 0x04);
        let empty = Opcode(0);
        assert_eq!(empty.extract_rex(InstOptions::NONE), 0);
    }

    #[test]
    fn by_size_builders() {
        let mut opc = Opcode(0x01);
        opc.add_arith_by_size(8);
        assert!(opc.has_w());
        assert_eq!(opc.0 & 1, 1);
        let mut opc = Opcode(0x01);
        opc.add_prefix_by_size(2);
        assert!(opc.has_66h());
    }

    #[test]
    fn extract_ll_mmmmm_includes_evex_force() {
        let opc = Opcode(Opcode::MM_0F38 | Opcode::LL_MASK);
        let ll_mm = opc.extract_ll_mmmmm(InstOptions::X86_EVEX);
        // Low 5 bits: MM (0F38 = 0x02) | EVEX force bit (0x10).
        assert_eq!(ll_mm & 0x1F, 0x12);
        // LL = 3 lands at bits [22:21] of the shifted value.
        assert_eq!((ll_mm >> 21) & 0x3, 0x3);
    }
}
