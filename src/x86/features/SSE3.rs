pub trait X86SSE3Emitter: Emitter {
    /// Emits `FISTTPM32`.
    fn fisttpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SSE_ADDSUBPD`.
    fn sse_addsubpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDSUBPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDSUBPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDSUBPD");
        }
    }
    /// Emits `SSE_ADDSUBPS`.
    fn sse_addsubps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_ADDSUBPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_ADDSUBPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_ADDSUBPS");
        }
    }
    /// Emits `SSE_HADDPD`.
    fn sse_haddpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_HADDPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_HADDPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_HADDPD");
        }
    }
    /// Emits `SSE_HADDPS`.
    fn sse_haddps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_HADDPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_HADDPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_HADDPS");
        }
    }
    /// Emits `SSE_HSUBPD`.
    fn sse_hsubpd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_HSUBPDRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_HSUBPDRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_HSUBPD");
        }
    }
    /// Emits `SSE_HSUBPS`.
    fn sse_hsubps(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_HSUBPSRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_HSUBPSRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_HSUBPS");
        }
    }
    /// Emits `SSE_LDDQURM`.
    fn sse_lddqu(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_LDDQURM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `SSE_MOVDDUP`.
    fn sse_movddup(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVDDUPRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVDDUPRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVDDUP");
        }
    }
    /// Emits `SSE_MOVSHDUP`.
    fn sse_movshdup(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVSHDUPRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVSHDUPRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVSHDUP");
        }
    }
    /// Emits `SSE_MOVSLDUP`.
    fn sse_movsldup(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_MOVSLDUPRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SSE_MOVSLDUPRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_MOVSLDUP");
        }
    }
}
