pub trait X86MMXEmitter: Emitter {
    /// Emits `MMX_EMMS`.
    fn mmx_emms(&mut self,) -> () {
        self.emit(MMX_EMMS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MMX_MOVD_G2M`.
    fn mmx_movd_g2m(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_gp() {
            self.emit(MMX_MOVD_G2MRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_MOVD_G2MRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_MOVD_G2M");
        }
    }
    /// Emits `MMX_MOVD_M2G`.
    fn mmx_movd_m2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVD_M2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVD_M2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_MOVD_M2G");
        }
    }
    /// Emits `MMX_MOVQ`.
    fn mmx_movq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_MOVQRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVQMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_MOVQ");
        }
    }
    /// Emits `MMX_MOVQ_G2M`.
    fn mmx_movq_g2m(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_gp() {
            self.emit(MMX_MOVQ_G2MRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_MOVQ_G2MRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_MOVQ_G2M");
        }
    }
    /// Emits `MMX_MOVQ_M2G`.
    fn mmx_movq_m2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVQ_M2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_MOVQ_M2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_MOVQ_M2G");
        }
    }
    /// Emits `MMX_PACKSSDW`.
    fn mmx_packssdw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PACKSSDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PACKSSDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PACKSSDW");
        }
    }
    /// Emits `MMX_PACKSSWB`.
    fn mmx_packsswb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PACKSSWBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PACKSSWBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PACKSSWB");
        }
    }
    /// Emits `MMX_PACKUSWB`.
    fn mmx_packuswb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PACKUSWBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PACKUSWBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PACKUSWB");
        }
    }
    /// Emits `MMX_PADDB`.
    fn mmx_paddb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDB");
        }
    }
    /// Emits `MMX_PADDD`.
    fn mmx_paddd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDD");
        }
    }
    /// Emits `MMX_PADDQ`.
    fn mmx_paddq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDQ");
        }
    }
    /// Emits `MMX_PADDSB`.
    fn mmx_paddsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDSB");
        }
    }
    /// Emits `MMX_PADDSW`.
    fn mmx_paddsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDSW");
        }
    }
    /// Emits `MMX_PADDUSB`.
    fn mmx_paddusb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDUSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDUSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDUSB");
        }
    }
    /// Emits `MMX_PADDUSW`.
    fn mmx_paddusw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDUSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDUSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDUSW");
        }
    }
    /// Emits `MMX_PADDW`.
    fn mmx_paddw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PADDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PADDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PADDW");
        }
    }
    /// Emits `MMX_PAND`.
    fn mmx_pand(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PANDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PANDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PAND");
        }
    }
    /// Emits `MMX_PANDN`.
    fn mmx_pandn(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PANDNRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PANDNRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PANDN");
        }
    }
    /// Emits `MMX_PCMPEQB`.
    fn mmx_pcmpeqb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPEQBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPEQBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPEQB");
        }
    }
    /// Emits `MMX_PCMPEQD`.
    fn mmx_pcmpeqd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPEQDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPEQDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPEQD");
        }
    }
    /// Emits `MMX_PCMPEQW`.
    fn mmx_pcmpeqw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPEQWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPEQWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPEQW");
        }
    }
    /// Emits `MMX_PCMPGTB`.
    fn mmx_pcmpgtb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPGTBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPGTBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPGTB");
        }
    }
    /// Emits `MMX_PCMPGTD`.
    fn mmx_pcmpgtd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPGTDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPGTDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPGTD");
        }
    }
    /// Emits `MMX_PCMPGTW`.
    fn mmx_pcmpgtw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PCMPGTWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PCMPGTWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PCMPGTW");
        }
    }
    /// Emits `MMX_PMADDWD`.
    fn mmx_pmaddwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMADDWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMADDWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMADDWD");
        }
    }
    /// Emits `MMX_PMULHW`.
    fn mmx_pmulhw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMULHWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMULHWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMULHW");
        }
    }
    /// Emits `MMX_PMULLW`.
    fn mmx_pmullw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMULLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMULLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMULLW");
        }
    }
    /// Emits `MMX_PMULUDQ`.
    fn mmx_pmuludq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMULUDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMULUDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMULUDQ");
        }
    }
    /// Emits `MMX_POR`.
    fn mmx_por(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PORRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PORRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_POR");
        }
    }
    /// Emits `MMX_PSLLD`.
    fn mmx_pslld(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSLLDRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSLLDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSLLDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSLLD");
        }
    }
    /// Emits `MMX_PSLLQ`.
    fn mmx_psllq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSLLQRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSLLQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSLLQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSLLQ");
        }
    }
    /// Emits `MMX_PSLLW`.
    fn mmx_psllw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSLLWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSLLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSLLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSLLW");
        }
    }
    /// Emits `MMX_PSRAD`.
    fn mmx_psrad(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSRADRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSRADRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSRADRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSRAD");
        }
    }
    /// Emits `MMX_PSRAW`.
    fn mmx_psraw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSRAWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSRAWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSRAWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSRAW");
        }
    }
    /// Emits `MMX_PSRLD`.
    fn mmx_psrld(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSRLDRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSRLDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSRLDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSRLD");
        }
    }
    /// Emits `MMX_PSRLQ`.
    fn mmx_psrlq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSRLQRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSRLQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSRLQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSRLQ");
        }
    }
    /// Emits `MMX_PSRLW`.
    fn mmx_psrlw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_imm() {
            self.emit(MMX_PSRLWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSRLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSRLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSRLW");
        }
    }
    /// Emits `MMX_PSUBB`.
    fn mmx_psubb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBB");
        }
    }
    /// Emits `MMX_PSUBD`.
    fn mmx_psubd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBD");
        }
    }
    /// Emits `MMX_PSUBQ`.
    fn mmx_psubq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBQ");
        }
    }
    /// Emits `MMX_PSUBSB`.
    fn mmx_psubsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBSB");
        }
    }
    /// Emits `MMX_PSUBSW`.
    fn mmx_psubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBSW");
        }
    }
    /// Emits `MMX_PSUBUSB`.
    fn mmx_psubusb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBUSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBUSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBUSB");
        }
    }
    /// Emits `MMX_PSUBUSW`.
    fn mmx_psubusw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBUSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBUSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBUSW");
        }
    }
    /// Emits `MMX_PSUBW`.
    fn mmx_psubw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSUBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSUBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSUBW");
        }
    }
    /// Emits `MMX_PUNPCKHBW`.
    fn mmx_punpckhbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKHBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKHBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKHBW");
        }
    }
    /// Emits `MMX_PUNPCKHDQ`.
    fn mmx_punpckhdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKHDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKHDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKHDQ");
        }
    }
    /// Emits `MMX_PUNPCKHWD`.
    fn mmx_punpckhwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKHWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKHWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKHWD");
        }
    }
    /// Emits `MMX_PUNPCKLBW`.
    fn mmx_punpcklbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKLBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKLBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKLBW");
        }
    }
    /// Emits `MMX_PUNPCKLDQ`.
    fn mmx_punpckldq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKLDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKLDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKLDQ");
        }
    }
    /// Emits `MMX_PUNPCKLWD`.
    fn mmx_punpcklwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PUNPCKLWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PUNPCKLWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PUNPCKLWD");
        }
    }
    /// Emits `MMX_PXOR`.
    fn mmx_pxor(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PXORRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PXORRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PXOR");
        }
    }
}
