pub trait X86SSSE3Emitter: Emitter {
    /// Emits `MMX_PABSB`.
    fn mmx_pabsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PABSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PABSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PABSB");
        }
    }
    /// Emits `MMX_PABSD`.
    fn mmx_pabsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PABSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PABSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PABSD");
        }
    }
    /// Emits `MMX_PABSW`.
    fn mmx_pabsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PABSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PABSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PABSW");
        }
    }
    /// Emits `MMX_PALIGNR`.
    fn mmx_palignr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) && op2.is_imm() {
            self.emit(MMX_PALIGNRRRI, op0,op1,op2,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() && op2.is_imm() {
            self.emit(MMX_PALIGNRRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PALIGNR");
        }
    }
    /// Emits `MMX_PHADDD`.
    fn mmx_phaddd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHADDDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHADDDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHADDD");
        }
    }
    /// Emits `MMX_PHADDSW`.
    fn mmx_phaddsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHADDSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHADDSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHADDSW");
        }
    }
    /// Emits `MMX_PHADDW`.
    fn mmx_phaddw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHADDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHADDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHADDW");
        }
    }
    /// Emits `MMX_PHSUBD`.
    fn mmx_phsubd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHSUBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHSUBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHSUBD");
        }
    }
    /// Emits `MMX_PHSUBSW`.
    fn mmx_phsubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHSUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHSUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHSUBSW");
        }
    }
    /// Emits `MMX_PHSUBW`.
    fn mmx_phsubw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PHSUBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PHSUBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PHSUBW");
        }
    }
    /// Emits `MMX_PMADDUBSW`.
    fn mmx_pmaddubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMADDUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMADDUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMADDUBSW");
        }
    }
    /// Emits `MMX_PMULHRSW`.
    fn mmx_pmulhrsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMULHRSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMULHRSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMULHRSW");
        }
    }
    /// Emits `MMX_PSHUFB`.
    fn mmx_pshufb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSHUFBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSHUFBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSHUFB");
        }
    }
    /// Emits `MMX_PSIGNB`.
    fn mmx_psignb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSIGNBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSIGNBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSIGNB");
        }
    }
    /// Emits `MMX_PSIGND`.
    fn mmx_psignd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSIGNDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSIGNDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSIGND");
        }
    }
    /// Emits `MMX_PSIGNW`.
    fn mmx_psignw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSIGNWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSIGNWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSIGNW");
        }
    }
    /// Emits `SSE_PABSB`.
    fn sse_pabsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PABSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PABSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PABSB");
        }
    }
    /// Emits `SSE_PABSD`.
    fn sse_pabsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PABSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PABSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PABSD");
        }
    }
    /// Emits `SSE_PABSW`.
    fn sse_pabsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PABSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PABSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PABSW");
        }
    }
    /// Emits `SSE_PALIGNR`.
    fn sse_palignr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PALIGNRRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PALIGNRRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PALIGNR");
        }
    }
    /// Emits `SSE_PHADDD`.
    fn sse_phaddd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHADDDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHADDDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHADDD");
        }
    }
    /// Emits `SSE_PHADDSW`.
    fn sse_phaddsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHADDSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHADDSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHADDSW");
        }
    }
    /// Emits `SSE_PHADDW`.
    fn sse_phaddw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHADDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHADDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHADDW");
        }
    }
    /// Emits `SSE_PHSUBD`.
    fn sse_phsubd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHSUBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHSUBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHSUBD");
        }
    }
    /// Emits `SSE_PHSUBSW`.
    fn sse_phsubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHSUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHSUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHSUBSW");
        }
    }
    /// Emits `SSE_PHSUBW`.
    fn sse_phsubw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHSUBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHSUBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHSUBW");
        }
    }
    /// Emits `SSE_PMADDUBSW`.
    fn sse_pmaddubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMADDUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMADDUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMADDUBSW");
        }
    }
    /// Emits `SSE_PMULHRSW`.
    fn sse_pmulhrsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULHRSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULHRSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULHRSW");
        }
    }
    /// Emits `SSE_PSHUFB`.
    fn sse_pshufb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSHUFBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSHUFBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSHUFB");
        }
    }
    /// Emits `SSE_PSIGNB`.
    fn sse_psignb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSIGNBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSIGNBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSIGNB");
        }
    }
    /// Emits `SSE_PSIGND`.
    fn sse_psignd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSIGNDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSIGNDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSIGND");
        }
    }
    /// Emits `SSE_PSIGNW`.
    fn sse_psignw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSIGNWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSIGNWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSIGNW");
        }
    }
}
