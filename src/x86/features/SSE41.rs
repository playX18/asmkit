pub trait X86SSE41Emitter: Emitter {
    /// Emits `SSE_BLENDPD`.
    fn sse_blendpd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_BLENDPDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_BLENDPDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_BLENDPD");
        }
    }
    /// Emits `SSE_BLENDPS`.
    fn sse_blendps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_BLENDPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_BLENDPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_BLENDPS");
        }
    }
    /// Emits `SSE_BLENDVPD`.
    fn sse_blendvpd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(SSE_BLENDVPDRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_vec() {
            self.emit(SSE_BLENDVPDRMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_BLENDVPD");
        }
    }
    /// Emits `SSE_BLENDVPS`.
    fn sse_blendvps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(SSE_BLENDVPSRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_vec() {
            self.emit(SSE_BLENDVPSRMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_BLENDVPS");
        }
    }
    /// Emits `SSE_DPPD`.
    fn sse_dppd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_DPPDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_DPPDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DPPD");
        }
    }
    /// Emits `SSE_DPPS`.
    fn sse_dpps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_DPPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_DPPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_DPPS");
        }
    }
    /// Emits `SSE_EXTRACTPS`.
    fn sse_extractps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_EXTRACTPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_EXTRACTPSMRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_EXTRACTPS");
        }
    }
    /// Emits `SSE_INSERTPS`.
    fn sse_insertps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_INSERTPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_INSERTPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_INSERTPS");
        }
    }
    /// Emits `SSE_MOVNTDQARM`.
    fn sse_movntdqa(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTDQARM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MPSADBW`.
    fn sse_mpsadbw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_MPSADBWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_MPSADBWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MPSADBW");
        }
    }
    /// Emits `SSE_PACKUSDW`.
    fn sse_packusdw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PACKUSDWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PACKUSDWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PACKUSDW");
        }
    }
    /// Emits `SSE_PBLENDVB`.
    fn sse_pblendvb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PBLENDVBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PBLENDVBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PBLENDVB");
        }
    }
    /// Emits `SSE_PBLENDW`.
    fn sse_pblendw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PBLENDWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PBLENDWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PBLENDW");
        }
    }
    /// Emits `SSE_PCMPEQQ`.
    fn sse_pcmpeqq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPEQQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPEQQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPEQQ");
        }
    }
    /// Emits `SSE_PCMPGTQ`.
    fn sse_pcmpgtq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PCMPGTQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PCMPGTQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPGTQ");
        }
    }
    /// Emits `SSE_PEXTRB`.
    fn sse_pextrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRBMRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRBRRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PEXTRB");
        }
    }
    /// Emits `SSE_PEXTRD`.
    fn sse_pextrd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRDMRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PEXTRD");
        }
    }
    /// Emits `SSE_PEXTRQ`.
    fn sse_pextrq(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRQRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PEXTRQMRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PEXTRQ");
        }
    }
    /// Emits `SSE_PEXTRWMRI`.
    fn sse_pextrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRWMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `SSE_PHMINPOSUW`.
    fn sse_phminposuw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PHMINPOSUWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PHMINPOSUWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PHMINPOSUW");
        }
    }
    /// Emits `SSE_PINSRB`.
    fn sse_pinsrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_gp() && op2.is_imm() {
            self.emit(SSE_PINSRBRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PINSRBRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PINSRB");
        }
    }
    /// Emits `SSE_PINSRD`.
    fn sse_pinsrd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_gp() && op2.is_imm() {
            self.emit(SSE_PINSRDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PINSRDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PINSRD");
        }
    }
    /// Emits `SSE_PINSRQ`.
    fn sse_pinsrq(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_gp() && op2.is_imm() {
            self.emit(SSE_PINSRQRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PINSRQRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PINSRQ");
        }
    }
    /// Emits `SSE_PMAXSB`.
    fn sse_pmaxsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXSB");
        }
    }
    /// Emits `SSE_PMAXSD`.
    fn sse_pmaxsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXSD");
        }
    }
    /// Emits `SSE_PMAXUD`.
    fn sse_pmaxud(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXUDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXUDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXUD");
        }
    }
    /// Emits `SSE_PMAXUW`.
    fn sse_pmaxuw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMAXUWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMAXUWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMAXUW");
        }
    }
    /// Emits `SSE_PMINSB`.
    fn sse_pminsb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINSBRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINSBRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINSB");
        }
    }
    /// Emits `SSE_PMINSD`.
    fn sse_pminsd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINSDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINSDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINSD");
        }
    }
    /// Emits `SSE_PMINUD`.
    fn sse_pminud(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINUDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINUDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINUD");
        }
    }
    /// Emits `SSE_PMINUW`.
    fn sse_pminuw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMINUWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMINUWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMINUW");
        }
    }
    /// Emits `SSE_PMOVSXBD`.
    fn sse_pmovsxbd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXBD");
        }
    }
    /// Emits `SSE_PMOVSXBQ`.
    fn sse_pmovsxbq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXBQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXBQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXBQ");
        }
    }
    /// Emits `SSE_PMOVSXBW`.
    fn sse_pmovsxbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXBW");
        }
    }
    /// Emits `SSE_PMOVSXDQ`.
    fn sse_pmovsxdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXDQ");
        }
    }
    /// Emits `SSE_PMOVSXWD`.
    fn sse_pmovsxwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXWD");
        }
    }
    /// Emits `SSE_PMOVSXWQ`.
    fn sse_pmovsxwq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVSXWQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVSXWQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVSXWQ");
        }
    }
    /// Emits `SSE_PMOVZXBD`.
    fn sse_pmovzxbd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXBDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXBDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXBD");
        }
    }
    /// Emits `SSE_PMOVZXBQ`.
    fn sse_pmovzxbq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXBQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXBQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXBQ");
        }
    }
    /// Emits `SSE_PMOVZXBW`.
    fn sse_pmovzxbw(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXBWRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXBWRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXBW");
        }
    }
    /// Emits `SSE_PMOVZXDQ`.
    fn sse_pmovzxdq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXDQ");
        }
    }
    /// Emits `SSE_PMOVZXWD`.
    fn sse_pmovzxwd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXWDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXWDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXWD");
        }
    }
    /// Emits `SSE_PMOVZXWQ`.
    fn sse_pmovzxwq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMOVZXWQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMOVZXWQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMOVZXWQ");
        }
    }
    /// Emits `SSE_PMULDQ`.
    fn sse_pmuldq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULDQRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULDQRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULDQ");
        }
    }
    /// Emits `SSE_PMULLD`.
    fn sse_pmulld(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PMULLDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PMULLDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PMULLD");
        }
    }
    /// Emits `SSE_PTEST`.
    fn sse_ptest(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_PTESTRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_PTESTRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PTEST");
        }
    }
    /// Emits `SSE_ROUNDPD`.
    fn sse_roundpd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_ROUNDPDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_ROUNDPDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ROUNDPD");
        }
    }
    /// Emits `SSE_ROUNDPS`.
    fn sse_roundps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_ROUNDPSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_ROUNDPSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ROUNDPS");
        }
    }
    /// Emits `SSE_ROUNDSD`.
    fn sse_roundsd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_ROUNDSDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_ROUNDSDRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ROUNDSD");
        }
    }
    /// Emits `SSE_ROUNDSS`.
    fn sse_roundss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_ROUNDSSRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_ROUNDSSRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ROUNDSS");
        }
    }
}
