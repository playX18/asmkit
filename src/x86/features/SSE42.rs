pub trait X86SSE42Emitter: Emitter {
    /// Emits `CRC32_16`.
    fn crc32_16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CRC32_16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CRC32_16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CRC32_16");
        }
    }
    /// Emits `CRC32_32`.
    fn crc32_32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CRC32_32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CRC32_32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CRC32_32");
        }
    }
    /// Emits `CRC32_64`.
    fn crc32_64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CRC32_64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CRC32_64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CRC32_64");
        }
    }
    /// Emits `CRC32_8`.
    fn crc32_8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CRC32_8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CRC32_8RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CRC32_8");
        }
    }
    /// Emits `SSE_PCMPESTRI`.
    fn sse_pcmpestri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PCMPESTRIRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PCMPESTRIRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPESTRI");
        }
    }
    /// Emits `SSE_PCMPESTRM`.
    fn sse_pcmpestrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PCMPESTRMRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PCMPESTRMRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPESTRM");
        }
    }
    /// Emits `SSE_PCMPISTRI`.
    fn sse_pcmpistri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PCMPISTRIRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PCMPISTRIRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPISTRI");
        }
    }
    /// Emits `SSE_PCMPISTRM`.
    fn sse_pcmpistrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PCMPISTRMRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PCMPISTRMRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SSE_PCMPISTRM");
        }
    }
}
