//! Read/write information describing how an instruction accesses its operands and CPU flags.
//!
//! This is the asmkit port of AsmJit's effects model (`OpRWFlags`, `CpuRWFlags`, `OpRWInfo`,
//! `InstRWInfo`, `InstControlFlow`, `InstSameRegHint` — see `core/inst.h` in AsmJit). The data
//! model is shaped so that generated per-architecture tables (produced by `meta/`) are plain
//! `static` arrays and a future register-allocation pass can consume them uniformly across
//! architectures.

use bitflags::bitflags;

use super::globals::MAX_OP_COUNT;

/// Physical register id used when an operand is not tied to any specific physical register.
pub const INVALID_PHYS_ID: u8 = 0xFF;

bitflags! {
    /// CPU read/write flags used by [`InstRwInfo`].
    ///
    /// Common flags occupy the low byte. Architecture-specific aliases share the same bits
    /// where architectures do not overlap (X86 EFLAGS and AArch64 NZCV reuse the common bits).
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct CpuRwFlags: u32 {
        /// No flags.
        const NONE = 0x0000_0000;

        /// Signed overflow flag.
        const OF = 0x0000_0001;
        /// Carry flag.
        const CF = 0x0000_0002;
        /// Zero and/or equality flag (1 if zero/equal).
        const ZF = 0x0000_0004;
        /// Sign flag (negative/sign, if set).
        const SF = 0x0000_0008;

        /// Carry flag (X86/X64).
        const X86_CF = Self::CF.bits();
        /// Overflow flag (X86/X64).
        const X86_OF = Self::OF.bits();
        /// Sign flag (X86/X64).
        const X86_SF = Self::SF.bits();
        /// Zero flag (X86/X64).
        const X86_ZF = Self::ZF.bits();

        /// Adjust flag (X86/X64).
        const X86_AF = 0x0000_0100;
        /// Parity flag (X86/X64).
        const X86_PF = 0x0000_0200;
        /// Direction flag (X86/X64).
        const X86_DF = 0x0000_0400;
        /// Interrupt enable flag (X86/X64).
        const X86_IF = 0x0000_0800;
        /// Alignment check flag (X86/X64).
        const X86_AC = 0x0000_1000;

        /// FPU C0 status flag (X86/X64).
        const X86_C0 = 0x0001_0000;
        /// FPU C1 status flag (X86/X64).
        const X86_C1 = 0x0002_0000;
        /// FPU C2 status flag (X86/X64).
        const X86_C2 = 0x0004_0000;
        /// FPU C3 status flag (X86/X64).
        const X86_C3 = 0x0008_0000;

        /// Overflow flag (AArch64 PSTATE.V).
        const A64_V = Self::OF.bits();
        /// Carry flag (AArch64 PSTATE.C).
        const A64_C = Self::CF.bits();
        /// Zero flag (AArch64 PSTATE.Z).
        const A64_Z = Self::ZF.bits();
        /// Negative flag (AArch64 PSTATE.N).
        const A64_N = Self::SF.bits();
        /// Cumulative saturation flag (AArch64 PSTATE.Q).
        const A64_Q = 0x0000_0100;
        /// Greater-than-or-equal flags (AArch64 PSTATE.GE).
        const A64_GE = 0x0000_0200;

        /// Floating-point accrued exception flags (RISC-V `fflags` CSR field).
        const RISCV_FFLAGS = 0x0000_0100;
        /// Floating-point rounding mode (RISC-V `frm` CSR field).
        const RISCV_FRM = 0x0000_0200;
        /// Fixed-point saturation flag (RISC-V `vxsat` CSR field).
        const RISCV_VXSAT = 0x0000_0400;
    }
}

bitflags! {
    /// Operand read/write flags describe how the operand is accessed and additional features.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct OpRwFlags: u32 {
        /// No flags.
        const NONE = 0;

        /// Operand is read.
        const READ = 0x0000_0001;
        /// Operand is written.
        const WRITE = 0x0000_0002;
        /// Operand is both read and written.
        const RW = Self::READ.bits() | Self::WRITE.bits();

        /// Register operand can be replaced by a memory operand.
        const REG_MEM = 0x0000_0004;

        /// The register must be allocated to the index of the previous register + 1.
        ///
        /// Used by instructions that use consecutive registers, where only the first one is
        /// encoded in the instruction, and the others are a sequence starting at the first
        /// one (X86 VP2INTERSECT*, AArch64 multi-vector loads/stores).
        const CONSECUTIVE = 0x0000_0008;

        /// The `extend_byte_mask` represents a zero extension.
        const ZEXT = 0x0000_0010;

        /// The register must have assigned a unique physical id, which cannot be assigned to
        /// any other register.
        const UNIQUE = 0x0000_0080;

        /// Register operand must use [`OpRwInfo::phys_id`].
        const REG_PHYS_ID = 0x0000_0100;
        /// Base register of a memory operand must use [`OpRwInfo::phys_id`].
        const MEM_PHYS_ID = 0x0000_0200;

        /// This memory operand is only used to encode registers and doesn't access memory.
        ///
        /// X86 specific: BNDLDX, BNDSTX, and LEA.
        const MEM_FAKE = 0x0000_0400;

        /// Base register of the memory operand will be read.
        const MEM_BASE_READ = 0x0000_1000;
        /// Base register of the memory operand will be written.
        const MEM_BASE_WRITE = 0x0000_2000;
        /// Base register of the memory operand will be read & written.
        const MEM_BASE_RW = Self::MEM_BASE_READ.bits() | Self::MEM_BASE_WRITE.bits();

        /// Index register of the memory operand will be read.
        const MEM_INDEX_READ = 0x0000_4000;
        /// Index register of the memory operand will be written.
        const MEM_INDEX_WRITE = 0x0000_8000;
        /// Index register of the memory operand will be read & written.
        const MEM_INDEX_RW = Self::MEM_INDEX_READ.bits() | Self::MEM_INDEX_WRITE.bits();

        /// Base register of the memory operand will be modified before the operation.
        const MEM_BASE_PRE_MODIFY = 0x0001_0000;
        /// Base register of the memory operand will be modified after the operation.
        const MEM_BASE_POST_MODIFY = 0x0002_0000;
    }
}

bitflags! {
    /// Flags used by [`InstRwInfo`].
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct InstRwFlags: u32 {
        /// No flags.
        const NONE = 0x0000_0000;

        /// Describes a move operation.
        ///
        /// Used by register allocation to eliminate moves that are guaranteed to be moves
        /// only.
        const MOV_OP = 0x0000_0001;
    }
}

/// Read/write information related to a single operand, used by [`InstRwInfo`].
///
/// `phys_id` is always [`INVALID_PHYS_ID`] unless a physical register is required — unlike
/// AsmJit (where value-initialization leaves it 0), asmkit keeps one "no phys id" sentinel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OpRwInfo {
    /// Read/write flags.
    pub op_flags: OpRwFlags,
    /// Physical register index, if required ([`INVALID_PHYS_ID`] otherwise).
    pub phys_id: u8,
    /// Size of a possible memory operand that can replace a register operand.
    pub rm_size: u8,
    /// If non-zero, this is a consecutive lead register and the value describes how many
    /// registers follow.
    pub consecutive_lead_count: u8,
    /// Read bit-mask where each bit represents one byte read from Reg/Mem.
    pub read_byte_mask: u64,
    /// Write bit-mask where each bit represents one byte written to Reg/Mem.
    pub write_byte_mask: u64,
    /// Zero/sign extend bit-mask where each bit represents one byte written to Reg/Mem.
    pub extend_byte_mask: u64,
}

impl Default for OpRwInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl OpRwInfo {
    /// Returns this operand info with all members reset to zero.
    pub const fn new() -> Self {
        Self {
            op_flags: OpRwFlags::NONE,
            phys_id: INVALID_PHYS_ID,
            rm_size: 0,
            consecutive_lead_count: 0,
            read_byte_mask: 0,
            write_byte_mask: 0,
            extend_byte_mask: 0,
        }
    }

    /// Resets this operand info to `op_flags`, `register_size`, and possibly `phys_id`,
    /// computing full byte masks from the flags (mirrors AsmJit's `OpRWInfo::reset`).
    pub fn reset(&mut self, op_flags: OpRwFlags, register_size: u32, phys_id: u8) {
        self.op_flags = op_flags;
        self.phys_id = phys_id;
        self.rm_size = if op_flags.contains(OpRwFlags::REG_MEM) {
            register_size as u8
        } else {
            0
        };
        self.consecutive_lead_count = 0;

        let mask = u64::MAX >> (64 - register_size.min(64));
        self.read_byte_mask = if op_flags.contains(OpRwFlags::READ) {
            mask
        } else {
            0
        };
        self.write_byte_mask = if op_flags.contains(OpRwFlags::WRITE) {
            mask
        } else {
            0
        };
        self.extend_byte_mask = 0;
    }

    /// Tests whether this operand is read from.
    pub const fn is_read(&self) -> bool {
        self.op_flags.contains(OpRwFlags::READ)
    }

    /// Tests whether this operand is written to.
    pub const fn is_write(&self) -> bool {
        self.op_flags.contains(OpRwFlags::WRITE)
    }

    /// Tests whether this operand is both read and written.
    pub const fn is_read_write(&self) -> bool {
        self.op_flags.contains(OpRwFlags::RW)
    }

    /// Tests whether this operand is read-only.
    pub const fn is_read_only(&self) -> bool {
        !self.is_write()
    }

    /// Tests whether this operand is write-only.
    pub const fn is_write_only(&self) -> bool {
        !self.is_read()
    }
}

/// Read/write information of an instruction.
#[derive(Clone, Copy, Debug)]
pub struct InstRwInfo {
    /// Instruction flags.
    pub inst_flags: InstRwFlags,
    /// CPU flags read.
    pub read_flags: CpuRwFlags,
    /// CPU flags written.
    pub write_flags: CpuRwFlags,
    /// Count of operands.
    pub op_count: u8,
    /// CPU feature required for replacing register operand with memory operand
    /// (architecture-specific feature id, 0 = none).
    pub rm_feature: u8,
    /// Read/write info of the extra register (rep{} or {k} selector).
    pub extra_reg: OpRwInfo,
    /// Read/write info of instruction operands.
    pub operands: [OpRwInfo; MAX_OP_COUNT],
}

impl Default for InstRwInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl InstRwInfo {
    /// Returns this instruction info with all members reset to zero.
    pub const fn new() -> Self {
        Self {
            inst_flags: InstRwFlags::NONE,
            read_flags: CpuRwFlags::NONE,
            write_flags: CpuRwFlags::NONE,
            op_count: 0,
            rm_feature: 0,
            extra_reg: OpRwInfo::new(),
            operands: [OpRwInfo::new(); MAX_OP_COUNT],
        }
    }

    /// Tests whether this instruction is a move operation (see [`InstRwFlags::MOV_OP`]).
    pub const fn is_mov_op(&self) -> bool {
        self.inst_flags.contains(InstRwFlags::MOV_OP)
    }

    /// Returns read/write info of operands as a slice, truncated to `op_count`.
    pub fn operands(&self) -> &[OpRwInfo] {
        &self.operands[..self.op_count as usize]
    }

    /// Returns read/write info of operands as a mutable slice, truncated to `op_count`.
    pub fn operands_mut(&mut self) -> &mut [OpRwInfo] {
        &mut self.operands[..self.op_count as usize]
    }

    /// Returns read/write info of the operand at `index`, or `None` if out of range.
    pub fn operand(&self, index: usize) -> Option<&OpRwInfo> {
        self.operands().get(index)
    }
}

/// Instruction control flow.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(u8)]
pub enum InstControlFlow {
    /// Regular instruction.
    #[default]
    Regular = 0,
    /// Unconditional jump.
    Jump = 1,
    /// Conditional jump (branch).
    Branch = 2,
    /// Function call.
    Call = 3,
    /// Function return.
    Return = 4,
}

/// Hint used when two or more operands of an instruction are the same register.
///
/// Influences the RW operations query: instructions such as `xor x, x` produce write-only
/// semantics, `and x, x` read-only.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(u8)]
pub enum InstSameRegHint {
    /// No special handling.
    #[default]
    None = 0,
    /// Operands become read-only, the operation doesn't change the content (`X & X` and
    /// similar).
    RO = 1,
    /// Operands become write-only, the content of the input(s) doesn't matter (`X ^ X`,
    /// `X - X`, and similar).
    WO = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_rw_flags_layout() {
        // Must stay compatible with AsmJit: RA code depends on these exact values.
        assert_eq!(OpRwFlags::READ.bits(), 0x1);
        assert_eq!(OpRwFlags::WRITE.bits(), 0x2);
        assert_eq!(OpRwFlags::RW.bits(), 0x3);
        assert_eq!(OpRwFlags::REG_MEM.bits(), 0x4);
    }

    #[test]
    fn reset_computes_masks() {
        let mut info = OpRwInfo::new();
        info.reset(OpRwFlags::RW, 8, INVALID_PHYS_ID);
        assert!(info.is_read_write());
        assert_eq!(info.read_byte_mask, 0xFF);
        assert_eq!(info.write_byte_mask, 0xFF);
        assert_eq!(info.extend_byte_mask, 0);

        info.reset(OpRwFlags::WRITE | OpRwFlags::REG_MEM, 16, 3);
        assert!(info.is_write_only());
        assert_eq!(info.rm_size, 16);
        assert_eq!(info.phys_id, 3);
        assert_eq!(info.write_byte_mask, 0xFFFF);
        assert_eq!(info.read_byte_mask, 0);
    }
}
