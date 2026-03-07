pub trait X86486Emitter: Emitter {
    /// Emits `BSWAP16R`.
    fn bswap16(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `BSWAP32R`.
    fn bswap32(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `BSWAP64R`.
    fn bswap64(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPXCHG16`.
    fn cmpxchg16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMPXCHG16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMPXCHG16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMPXCHG16");
        }
    }
    /// Emits `CMPXCHG32`.
    fn cmpxchg32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMPXCHG32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMPXCHG32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMPXCHG32");
        }
    }
    /// Emits `CMPXCHG64`.
    fn cmpxchg64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMPXCHG64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMPXCHG64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMPXCHG64");
        }
    }
    /// Emits `CMPXCHG8`.
    fn cmpxchg8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMPXCHG8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMPXCHG8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMPXCHG8");
        }
    }
    /// Emits `INVD`.
    fn invd(&mut self,) -> () {
        self.emit(INVD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INVLPG8M`.
    fn invlpg8(&mut self,op0: impl OperandCast) -> () {
        self.emit(INVLPG8M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WBINVD`.
    fn wbinvd(&mut self,) -> () {
        self.emit(WBINVD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XADD16`.
    fn xadd16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XADD16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XADD16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XADD16");
        }
    }
    /// Emits `XADD32`.
    fn xadd32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XADD32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XADD32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XADD32");
        }
    }
    /// Emits `XADD64`.
    fn xadd64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XADD64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XADD64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XADD64");
        }
    }
    /// Emits `XADD8`.
    fn xadd8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XADD8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XADD8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XADD8");
        }
    }
}
