pub trait X86SSE2Emitter: Emitter {
    /// Emits `LFENCE`.
    fn lfence(&mut self,) -> () {
        self.emit(LFENCE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MFENCE`.
    fn mfence(&mut self,) -> () {
        self.emit(MFENCE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MMX_CVTPD2PI`.
    fn mmx_cvtpd2pi(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_vec() {
            self.emit(MMX_CVTPD2PIRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_CVTPD2PIRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTPD2PI");
        }
    }
    /// Emits `MMX_CVTPI2PD`.
    fn mmx_cvtpi2pd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_CVTPI2PDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(MMX_CVTPI2PDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTPI2PD");
        }
    }
    /// Emits `MMX_CVTPI2PS`.
    fn mmx_cvtpi2ps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_reg_group_of(RegGroup::X86MM) {
            self.emit(MMX_CVTPI2PSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(MMX_CVTPI2PSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTPI2PS");
        }
    }
    /// Emits `MMX_CVTPS2PI`.
    fn mmx_cvtps2pi(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_vec() {
            self.emit(MMX_CVTPS2PIRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_CVTPS2PIRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTPS2PI");
        }
    }
    /// Emits `MMX_CVTTPD2PI`.
    fn mmx_cvttpd2pi(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_vec() {
            self.emit(MMX_CVTTPD2PIRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_CVTTPD2PIRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTTPD2PI");
        }
    }
    /// Emits `MMX_CVTTPS2PI`.
    fn mmx_cvttps2pi(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_vec() {
            self.emit(MMX_CVTTPS2PIRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() {
            self.emit(MMX_CVTTPS2PIRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MMX_CVTTPS2PI");
        }
    }
    /// Emits `MOVNTI32MR`.
    fn movnti32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVNTI32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVNTI64MR`.
    fn movnti64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVNTI64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_ADDPD`.
    fn sse_addpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDPD");
        }
    }
    /// Emits `SSE_ADDSD`.
    fn sse_addsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDSD");
        }
    }
    /// Emits `SSE_ANDNPD`.
    fn sse_andnpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ANDNPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ANDNPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ANDNPD");
        }
    }
    /// Emits `SSE_ANDPD`.
    fn sse_andpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ANDPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ANDPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ANDPD");
        }
    }
    /// Emits `SSE_CMPPD`.
    fn sse_cmppd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_CMPPDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_CMPPDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CMPPD");
        }
    }
    /// Emits `SSE_CMPSD`.
    fn sse_cmpsd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_CMPSDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_CMPSDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CMPSD");
        }
    }
    /// Emits `SSE_COMISD`.
    fn sse_comisd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_COMISDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_COMISDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_COMISD");
        }
    }
    /// Emits `SSE_CVTDQ2PD`.
    fn sse_cvtdq2pd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTDQ2PDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTDQ2PDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTDQ2PD");
        }
    }
    /// Emits `SSE_CVTDQ2PS`.
    fn sse_cvtdq2ps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTDQ2PSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTDQ2PSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTDQ2PS");
        }
    }
    /// Emits `SSE_CVTPD2DQ`.
    fn sse_cvtpd2dq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTPD2DQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTPD2DQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTPD2DQ");
        }
    }
    /// Emits `SSE_CVTPD2PS`.
    fn sse_cvtpd2ps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTPD2PSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTPD2PSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTPD2PS");
        }
    }
    /// Emits `SSE_CVTPS2DQ`.
    fn sse_cvtps2dq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTPS2DQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTPS2DQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTPS2DQ");
        }
    }
    /// Emits `SSE_CVTPS2PD`.
    fn sse_cvtps2pd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTPS2PDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTPS2PDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTPS2PD");
        }
    }
    /// Emits `SSE_CVTSD2SI32`.
    fn sse_cvtsd2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTSD2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTSD2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSD2SI32");
        }
    }
    /// Emits `SSE_CVTSD2SI64`.
    fn sse_cvtsd2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTSD2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTSD2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSD2SI64");
        }
    }
    /// Emits `SSE_CVTSD2SS`.
    fn sse_cvtsd2ss(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTSD2SSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSD2SSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSD2SS");
        }
    }
    /// Emits `SSE_CVTSI2SD32`.
    fn sse_cvtsi2sd32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_CVTSI2SD32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSI2SD32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSI2SD32");
        }
    }
    /// Emits `SSE_CVTSI2SD64`.
    fn sse_cvtsi2sd64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_CVTSI2SD64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSI2SD64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSI2SD64");
        }
    }
    /// Emits `SSE_CVTSS2SD`.
    fn sse_cvtss2sd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTSS2SDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTSS2SDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTSS2SD");
        }
    }
    /// Emits `SSE_CVTTPD2DQ`.
    fn sse_cvttpd2dq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTTPD2DQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTTPD2DQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTPD2DQ");
        }
    }
    /// Emits `SSE_CVTTPS2DQ`.
    fn sse_cvttps2dq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_CVTTPS2DQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_CVTTPS2DQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTPS2DQ");
        }
    }
    /// Emits `SSE_CVTTSD2SI32`.
    fn sse_cvttsd2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTTSD2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTTSD2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTSD2SI32");
        }
    }
    /// Emits `SSE_CVTTSD2SI64`.
    fn sse_cvttsd2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_CVTTSD2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SSE_CVTTSD2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_CVTTSD2SI64");
        }
    }
    /// Emits `SSE_DIVPD`.
    fn sse_divpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_DIVPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_DIVPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DIVPD");
        }
    }
    /// Emits `SSE_DIVSD`.
    fn sse_divsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_DIVSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_DIVSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DIVSD");
        }
    }
    /// Emits `SSE_MASKMOVDQURR`.
    fn sse_maskmovdqu(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MASKMOVDQURR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MAXPD`.
    fn sse_maxpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MAXPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MAXPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MAXPD");
        }
    }
    /// Emits `SSE_MAXSD`.
    fn sse_maxsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MAXSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MAXSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MAXSD");
        }
    }
    /// Emits `SSE_MINPD`.
    fn sse_minpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MINPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MINPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MINPD");
        }
    }
    /// Emits `SSE_MINSD`.
    fn sse_minsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MINSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MINSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MINSD");
        }
    }
    /// Emits `SSE_MOVAPD`.
    fn sse_movapd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVAPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVAPDRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVAPDMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVAPD");
        }
    }
    /// Emits `SSE_MOVDQA`.
    fn sse_movdqa(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVDQARR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVDQARM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVDQAMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVDQA");
        }
    }
    /// Emits `SSE_MOVDQU`.
    fn sse_movdqu(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVDQURR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVDQURM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVDQUMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVDQU");
        }
    }
    /// Emits `SSE_MOVD_G2X`.
    fn sse_movd_g2x(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_MOVD_G2XRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVD_G2XRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVD_G2X");
        }
    }
    /// Emits `SSE_MOVD_X2G`.
    fn sse_movd_x2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_MOVD_X2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVD_X2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVD_X2G");
        }
    }
    /// Emits `SSE_MOVHPD`.
    fn sse_movhpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVHPDRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVHPDMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVHPD");
        }
    }
    /// Emits `SSE_MOVLPD`.
    fn sse_movlpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVLPDRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVLPDMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVLPD");
        }
    }
    /// Emits `SSE_MOVMSKPDRR`.
    fn sse_movmskpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVMSKPDRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVNTDQMR`.
    fn sse_movntdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTDQMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVNTPDMR`.
    fn sse_movntpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTPDMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVNTSDMR`.
    fn sse_movntsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTSDMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVQ`.
    fn sse_movq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVQRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVQMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVQ");
        }
    }
    /// Emits `SSE_MOVQ_G2X`.
    fn sse_movq_g2x(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(SSE_MOVQ_G2XRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVQ_G2XRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVQ_G2X");
        }
    }
    /// Emits `SSE_MOVQ_X2G`.
    fn sse_movq_x2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(SSE_MOVQ_X2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVQ_X2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVQ_X2G");
        }
    }
    /// Emits `SSE_MOVSD`.
    fn sse_movsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVSDRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVSDMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVSD");
        }
    }
    /// Emits `SSE_MOVUPD`.
    fn sse_movupd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVUPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVUPDRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(SSE_MOVUPDMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVUPD");
        }
    }
    /// Emits `SSE_MULPD`.
    fn sse_mulpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MULPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MULPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MULPD");
        }
    }
    /// Emits `SSE_MULSD`.
    fn sse_mulsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MULSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MULSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MULSD");
        }
    }
    /// Emits `SSE_ORPD`.
    fn sse_orpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ORPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ORPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ORPD");
        }
    }
    /// Emits `SSE_PACKSSDW`.
    fn sse_packssdw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PACKSSDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PACKSSDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PACKSSDW");
        }
    }
    /// Emits `SSE_PACKSSWB`.
    fn sse_packsswb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PACKSSWBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PACKSSWBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PACKSSWB");
        }
    }
    /// Emits `SSE_PACKUSWB`.
    fn sse_packuswb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PACKUSWBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PACKUSWBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PACKUSWB");
        }
    }
    /// Emits `SSE_PADDB`.
    fn sse_paddb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDB");
        }
    }
    /// Emits `SSE_PADDD`.
    fn sse_paddd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDD");
        }
    }
    /// Emits `SSE_PADDQ`.
    fn sse_paddq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDQ");
        }
    }
    /// Emits `SSE_PADDSB`.
    fn sse_paddsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDSB");
        }
    }
    /// Emits `SSE_PADDSW`.
    fn sse_paddsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDSW");
        }
    }
    /// Emits `SSE_PADDUSB`.
    fn sse_paddusb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDUSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDUSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDUSB");
        }
    }
    /// Emits `SSE_PADDUSW`.
    fn sse_paddusw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDUSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDUSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDUSW");
        }
    }
    /// Emits `SSE_PADDW`.
    fn sse_paddw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PADDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PADDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PADDW");
        }
    }
    /// Emits `SSE_PAND`.
    fn sse_pand(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PANDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PANDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PAND");
        }
    }
    /// Emits `SSE_PANDN`.
    fn sse_pandn(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PANDNRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PANDNRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PANDN");
        }
    }
    /// Emits `SSE_PAVGB`.
    fn sse_pavgb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PAVGBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PAVGBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PAVGB");
        }
    }
    /// Emits `SSE_PAVGW`.
    fn sse_pavgw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PAVGWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PAVGWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PAVGW");
        }
    }
    /// Emits `SSE_PCMPEQB`.
    fn sse_pcmpeqb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPEQBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPEQBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPEQB");
        }
    }
    /// Emits `SSE_PCMPEQD`.
    fn sse_pcmpeqd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPEQDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPEQDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPEQD");
        }
    }
    /// Emits `SSE_PCMPEQW`.
    fn sse_pcmpeqw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPEQWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPEQWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPEQW");
        }
    }
    /// Emits `SSE_PCMPGTB`.
    fn sse_pcmpgtb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPGTBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPGTBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPGTB");
        }
    }
    /// Emits `SSE_PCMPGTD`.
    fn sse_pcmpgtd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPGTDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPGTDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPGTD");
        }
    }
    /// Emits `SSE_PCMPGTW`.
    fn sse_pcmpgtw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPGTWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPGTWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPGTW");
        }
    }
    /// Emits `SSE_PEXTRWRRI`.
    fn sse_pextrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `SSE_PINSRW`.
    fn sse_pinsrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_gp() && op2.is_imm() {
            self.emit(SSE_PINSRWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PINSRWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PINSRW");
        }
    }
    /// Emits `SSE_PMADDWD`.
    fn sse_pmaddwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMADDWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMADDWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMADDWD");
        }
    }
    /// Emits `SSE_PMAXSW`.
    fn sse_pmaxsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXSW");
        }
    }
    /// Emits `SSE_PMAXUB`.
    fn sse_pmaxub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXUBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXUBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXUB");
        }
    }
    /// Emits `SSE_PMINSW`.
    fn sse_pminsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINSW");
        }
    }
    /// Emits `SSE_PMINUB`.
    fn sse_pminub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINUBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINUBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINUB");
        }
    }
    /// Emits `SSE_PMOVMSKBRR`.
    fn sse_pmovmskb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVMSKBRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_PMULHUW`.
    fn sse_pmulhuw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULHUWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULHUWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULHUW");
        }
    }
    /// Emits `SSE_PMULHW`.
    fn sse_pmulhw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULHWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULHWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULHW");
        }
    }
    /// Emits `SSE_PMULLW`.
    fn sse_pmullw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULLW");
        }
    }
    /// Emits `SSE_PMULUDQ`.
    fn sse_pmuludq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULUDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULUDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULUDQ");
        }
    }
    /// Emits `SSE_POR`.
    fn sse_por(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PORRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PORRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_POR");
        }
    }
    /// Emits `SSE_PSADBW`.
    fn sse_psadbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSADBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSADBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSADBW");
        }
    }
    /// Emits `SSE_PSHUFD`.
    fn sse_pshufd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PSHUFDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PSHUFDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSHUFD");
        }
    }
    /// Emits `SSE_PSHUFHW`.
    fn sse_pshufhw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PSHUFHWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PSHUFHWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSHUFHW");
        }
    }
    /// Emits `SSE_PSHUFLW`.
    fn sse_pshuflw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PSHUFLWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PSHUFLWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSHUFLW");
        }
    }
    /// Emits `SSE_PSLLD`.
    fn sse_pslld(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSLLDRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSLLDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSLLDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSLLD");
        }
    }
    /// Emits `SSE_PSLLDQRI`.
    fn sse_pslldq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLDQRI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_PSLLQ`.
    fn sse_psllq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSLLQRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSLLQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSLLQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSLLQ");
        }
    }
    /// Emits `SSE_PSLLW`.
    fn sse_psllw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSLLWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSLLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSLLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSLLW");
        }
    }
    /// Emits `SSE_PSRAD`.
    fn sse_psrad(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSRADRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSRADRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSRADRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSRAD");
        }
    }
    /// Emits `SSE_PSRAW`.
    fn sse_psraw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSRAWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSRAWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSRAWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSRAW");
        }
    }
    /// Emits `SSE_PSRLD`.
    fn sse_psrld(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSRLDRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSRLDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSRLDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSRLD");
        }
    }
    /// Emits `SSE_PSRLDQRI`.
    fn sse_psrldq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLDQRI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_PSRLQ`.
    fn sse_psrlq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSRLQRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSRLQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSRLQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSRLQ");
        }
    }
    /// Emits `SSE_PSRLW`.
    fn sse_psrlw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_PSRLWRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSRLWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSRLWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSRLW");
        }
    }
    /// Emits `SSE_PSUBB`.
    fn sse_psubb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBB");
        }
    }
    /// Emits `SSE_PSUBD`.
    fn sse_psubd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBD");
        }
    }
    /// Emits `SSE_PSUBQ`.
    fn sse_psubq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBQ");
        }
    }
    /// Emits `SSE_PSUBSB`.
    fn sse_psubsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBSB");
        }
    }
    /// Emits `SSE_PSUBSW`.
    fn sse_psubsw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBSW");
        }
    }
    /// Emits `SSE_PSUBUSB`.
    fn sse_psubusb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBUSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBUSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBUSB");
        }
    }
    /// Emits `SSE_PSUBUSW`.
    fn sse_psubusw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBUSWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBUSWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBUSW");
        }
    }
    /// Emits `SSE_PSUBW`.
    fn sse_psubw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PSUBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PSUBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PSUBW");
        }
    }
    /// Emits `SSE_PUNPCKHBW`.
    fn sse_punpckhbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKHBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKHBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKHBW");
        }
    }
    /// Emits `SSE_PUNPCKHDQ`.
    fn sse_punpckhdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKHDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKHDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKHDQ");
        }
    }
    /// Emits `SSE_PUNPCKHQDQ`.
    fn sse_punpckhqdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKHQDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKHQDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKHQDQ");
        }
    }
    /// Emits `SSE_PUNPCKHWD`.
    fn sse_punpckhwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKHWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKHWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKHWD");
        }
    }
    /// Emits `SSE_PUNPCKLBW`.
    fn sse_punpcklbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKLBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKLBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKLBW");
        }
    }
    /// Emits `SSE_PUNPCKLDQ`.
    fn sse_punpckldq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKLDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKLDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKLDQ");
        }
    }
    /// Emits `SSE_PUNPCKLQDQ`.
    fn sse_punpcklqdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKLQDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKLQDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKLQDQ");
        }
    }
    /// Emits `SSE_PUNPCKLWD`.
    fn sse_punpcklwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PUNPCKLWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PUNPCKLWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PUNPCKLWD");
        }
    }
    /// Emits `SSE_PXOR`.
    fn sse_pxor(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PXORRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PXORRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PXOR");
        }
    }
    /// Emits `SSE_SHUFPD`.
    fn sse_shufpd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_SHUFPDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_SHUFPDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SHUFPD");
        }
    }
    /// Emits `SSE_SQRTPD`.
    fn sse_sqrtpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SQRTPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SQRTPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SQRTPD");
        }
    }
    /// Emits `SSE_SQRTSD`.
    fn sse_sqrtsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SQRTSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SQRTSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SQRTSD");
        }
    }
    /// Emits `SSE_SUBPD`.
    fn sse_subpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SUBPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SUBPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SUBPD");
        }
    }
    /// Emits `SSE_SUBSD`.
    fn sse_subsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_SUBSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_SUBSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_SUBSD");
        }
    }
    /// Emits `SSE_UCOMISD`.
    fn sse_ucomisd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UCOMISDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UCOMISDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UCOMISD");
        }
    }
    /// Emits `SSE_UNPCKHPD`.
    fn sse_unpckhpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UNPCKHPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UNPCKHPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UNPCKHPD");
        }
    }
    /// Emits `SSE_UNPCKLPD`.
    fn sse_unpcklpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_UNPCKLPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_UNPCKLPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_UNPCKLPD");
        }
    }
    /// Emits `SSE_XORPD`.
    fn sse_xorpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_XORPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_XORPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_XORPD");
        }
    }
}
