pub trait X86SSEEmitter: Emitter {
    /// Emits `LDMXCSRM`.
    fn ldmxcsr(&mut self,op0: impl OperandCast) -> () {
        self.emit(LDMXCSRM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MMX_MASKMOVQRR`.
    fn mmx_maskmovq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MASKMOVQRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MMX_MOVDQ2QRR`.
    fn mmx_movdq2q(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVDQ2QRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MMX_MOVNTQMR`.
    fn mmx_movntq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVNTQMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MMX_MOVQ2DQRR`.
    fn mmx_movq2dq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ2DQRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MMX_PAVGB`.
    fn mmx_pavgb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PAVGBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PAVGBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PAVGB");
        }
    }
    /// Emits `MMX_PAVGW`.
    fn mmx_pavgw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PAVGWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PAVGWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PAVGW");
        }
    }
    /// Emits `MMX_PEXTRWRRI`.
    fn mmx_pextrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PEXTRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `MMX_PINSRW`.
    fn mmx_pinsrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_gp() && op2.is_imm() {
            self.emit(MMX_PINSRWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() && op2.is_imm() {
            self.emit(MMX_PINSRWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PINSRW");
        }
    }
    /// Emits `MMX_PMAXSW`.
    fn mmx_pmaxsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMAXSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMAXSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMAXSW");
        }
    }
    /// Emits `MMX_PMAXUB`.
    fn mmx_pmaxub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMAXUBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMAXUBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMAXUB");
        }
    }
    /// Emits `MMX_PMINSW`.
    fn mmx_pminsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMINSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMINSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMINSW");
        }
    }
    /// Emits `MMX_PMINUB`.
    fn mmx_pminub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMINUBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMINUBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMINUB");
        }
    }
    /// Emits `MMX_PMOVMSKBRR`.
    fn mmx_pmovmskb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMOVMSKBRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MMX_PMULHUW`.
    fn mmx_pmulhuw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PMULHUWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PMULHUWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PMULHUW");
        }
    }
    /// Emits `MMX_PSADBW`.
    fn mmx_psadbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_PSADBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_PSADBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSADBW");
        }
    }
    /// Emits `MMX_PSHUFW`.
    fn mmx_pshufw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) && op2.is_imm() {
            self.emit(MMX_PSHUFWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() && op2.is_imm() {
            self.emit(MMX_PSHUFWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_PSHUFW");
        }
    }
    /// Emits `PREFETCHNTAM`.
    fn prefetchnta(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHNTAM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PREFETCHT0M`.
    fn prefetcht0(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT0M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PREFETCHT1M`.
    fn prefetcht1(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT1M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PREFETCHT2M`.
    fn prefetcht2(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT2M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SFENCE`.
    fn sfence(&mut self,) -> () {
        self.emit(SFENCE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SSE_ADDPS`.
    fn sse_addps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDPS");
        }
    }
    /// Emits `SSE_ADDSS`.
    fn sse_addss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDSS");
        }
    }
    /// Emits `SSE_ANDNPS`.
    fn sse_andnps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ANDNPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ANDNPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ANDNPS");
        }
    }
    /// Emits `SSE_ANDPS`.
    fn sse_andps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ANDPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ANDPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ANDPS");
        }
    }
    /// Emits `SSE_CMPPS`.
    fn sse_cmpps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_CMPPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_CMPPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CMPPS");
        }
    }
    /// Emits `SSE_CMPSS`.
    fn sse_cmpss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_CMPSSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_CMPSSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CMPSS");
        }
    }
    /// Emits `SSE_COMISS`.
    fn sse_comiss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_COMISSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_COMISSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_COMISS");
        }
    }
    /// Emits `SSE_CVTSI2SS32`.
    fn sse_cvtsi2ss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_CVTSI2SS32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSI2SS32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSI2SS32");
        }
    }
    /// Emits `SSE_CVTSI2SS64`.
    fn sse_cvtsi2ss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_CVTSI2SS64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSI2SS64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSI2SS64");
        }
    }
    /// Emits `SSE_CVTSS2SI32`.
    fn sse_cvtss2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTSS2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTSS2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSS2SI32");
        }
    }
    /// Emits `SSE_CVTSS2SI64`.
    fn sse_cvtss2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTSS2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTSS2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSS2SI64");
        }
    }
    /// Emits `SSE_CVTTSS2SI32`.
    fn sse_cvttss2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTTSS2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTTSS2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTSS2SI32");
        }
    }
    /// Emits `SSE_CVTTSS2SI64`.
    fn sse_cvttss2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTTSS2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTTSS2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTSS2SI64");
        }
    }
    /// Emits `SSE_DIVPS`.
    fn sse_divps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_DIVPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_DIVPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DIVPS");
        }
    }
    /// Emits `SSE_DIVSS`.
    fn sse_divss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_DIVSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_DIVSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DIVSS");
        }
    }
    /// Emits `SSE_MAXPS`.
    fn sse_maxps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MAXPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MAXPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MAXPS");
        }
    }
    /// Emits `SSE_MAXSS`.
    fn sse_maxss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MAXSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MAXSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MAXSS");
        }
    }
    /// Emits `SSE_MINPS`.
    fn sse_minps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MINPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MINPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MINPS");
        }
    }
    /// Emits `SSE_MINSS`.
    fn sse_minss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MINSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MINSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MINSS");
        }
    }
    /// Emits `SSE_MOVAPS`.
    fn sse_movaps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVAPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVAPSRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVAPSMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVAPS");
        }
    }
    /// Emits `SSE_MOVHLPSRR`.
    fn sse_movhlps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHLPSRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVHPS`.
    fn sse_movhps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVHPSRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVHPSMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVHPS");
        }
    }
    /// Emits `SSE_MOVLHPSRR`.
    fn sse_movlhps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLHPSRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVLPS`.
    fn sse_movlps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVLPSRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVLPSMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVLPS");
        }
    }
    /// Emits `SSE_MOVMSKPSRR`.
    fn sse_movmskps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVMSKPSRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVNTPSMR`.
    fn sse_movntps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTPSMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVNTSSMR`.
    fn sse_movntss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTSSMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVSS`.
    fn sse_movss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVSSRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVSSMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVSS");
        }
    }
    /// Emits `SSE_MOVUPS`.
    fn sse_movups(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVUPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVUPSRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVUPSMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVUPS");
        }
    }
    /// Emits `SSE_MULPS`.
    fn sse_mulps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MULPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MULPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MULPS");
        }
    }
    /// Emits `SSE_MULSS`.
    fn sse_mulss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MULSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MULSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MULSS");
        }
    }
    /// Emits `SSE_ORPS`.
    fn sse_orps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ORPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ORPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ORPS");
        }
    }
    /// Emits `SSE_RCPPS`.
    fn sse_rcpps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_RCPPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_RCPPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_RCPPS");
        }
    }
    /// Emits `SSE_RCPSS`.
    fn sse_rcpss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_RCPSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_RCPSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_RCPSS");
        }
    }
    /// Emits `SSE_RSQRTPS`.
    fn sse_rsqrtps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_RSQRTPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_RSQRTPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_RSQRTPS");
        }
    }
    /// Emits `SSE_RSQRTSS`.
    fn sse_rsqrtss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_RSQRTSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_RSQRTSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_RSQRTSS");
        }
    }
    /// Emits `SSE_SHUFPS`.
    fn sse_shufps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_SHUFPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_SHUFPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SHUFPS");
        }
    }
    /// Emits `SSE_SQRTPS`.
    fn sse_sqrtps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SQRTPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SQRTPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SQRTPS");
        }
    }
    /// Emits `SSE_SQRTSS`.
    fn sse_sqrtss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SQRTSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SQRTSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SQRTSS");
        }
    }
    /// Emits `SSE_SUBPS`.
    fn sse_subps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SUBPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SUBPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SUBPS");
        }
    }
    /// Emits `SSE_SUBSS`.
    fn sse_subss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SUBSSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SUBSSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SUBSS");
        }
    }
    /// Emits `SSE_UCOMISS`.
    fn sse_ucomiss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UCOMISSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UCOMISSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UCOMISS");
        }
    }
    /// Emits `SSE_UNPCKHPS`.
    fn sse_unpckhps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UNPCKHPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UNPCKHPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UNPCKHPS");
        }
    }
    /// Emits `SSE_UNPCKLPS`.
    fn sse_unpcklps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UNPCKLPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UNPCKLPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UNPCKLPS");
        }
    }
    /// Emits `SSE_XORPS`.
    fn sse_xorps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_XORPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_XORPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_XORPS");
        }
    }
    /// Emits `STMXCSRM`.
    fn stmxcsr(&mut self,op0: impl OperandCast) -> () {
        self.emit(STMXCSRM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
