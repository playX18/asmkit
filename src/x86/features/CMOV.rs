pub trait X86CMOVEmitter: Emitter {
    /// Emits `CMOVA16`.
    fn cmova16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVA16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVA16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVA16");
        }
    }
    /// Emits `CMOVA32`.
    fn cmova32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVA32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVA32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVA32");
        }
    }
    /// Emits `CMOVA64`.
    fn cmova64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVA64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVA64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVA64");
        }
    }
    /// Emits `CMOVBE16`.
    fn cmovbe16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVBE16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVBE16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVBE16");
        }
    }
    /// Emits `CMOVBE32`.
    fn cmovbe32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVBE32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVBE32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVBE32");
        }
    }
    /// Emits `CMOVBE64`.
    fn cmovbe64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVBE64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVBE64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVBE64");
        }
    }
    /// Emits `CMOVC16`.
    fn cmovc16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVC16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVC16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVC16");
        }
    }
    /// Emits `CMOVC32`.
    fn cmovc32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVC32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVC32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVC32");
        }
    }
    /// Emits `CMOVC64`.
    fn cmovc64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVC64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVC64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVC64");
        }
    }
    /// Emits `CMOVG16`.
    fn cmovg16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVG16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVG16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVG16");
        }
    }
    /// Emits `CMOVG32`.
    fn cmovg32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVG32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVG32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVG32");
        }
    }
    /// Emits `CMOVG64`.
    fn cmovg64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVG64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVG64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVG64");
        }
    }
    /// Emits `CMOVGE16`.
    fn cmovge16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVGE16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVGE16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVGE16");
        }
    }
    /// Emits `CMOVGE32`.
    fn cmovge32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVGE32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVGE32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVGE32");
        }
    }
    /// Emits `CMOVGE64`.
    fn cmovge64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVGE64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVGE64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVGE64");
        }
    }
    /// Emits `CMOVL16`.
    fn cmovl16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVL16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVL16");
        }
    }
    /// Emits `CMOVL32`.
    fn cmovl32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVL32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVL32");
        }
    }
    /// Emits `CMOVL64`.
    fn cmovl64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVL64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVL64");
        }
    }
    /// Emits `CMOVLE16`.
    fn cmovle16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVLE16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVLE16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVLE16");
        }
    }
    /// Emits `CMOVLE32`.
    fn cmovle32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVLE32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVLE32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVLE32");
        }
    }
    /// Emits `CMOVLE64`.
    fn cmovle64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVLE64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVLE64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVLE64");
        }
    }
    /// Emits `CMOVNC16`.
    fn cmovnc16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNC16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNC16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNC16");
        }
    }
    /// Emits `CMOVNC32`.
    fn cmovnc32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNC32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNC32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNC32");
        }
    }
    /// Emits `CMOVNC64`.
    fn cmovnc64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNC64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNC64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNC64");
        }
    }
    /// Emits `CMOVNO16`.
    fn cmovno16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNO16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNO16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNO16");
        }
    }
    /// Emits `CMOVNO32`.
    fn cmovno32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNO32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNO32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNO32");
        }
    }
    /// Emits `CMOVNO64`.
    fn cmovno64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNO64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNO64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNO64");
        }
    }
    /// Emits `CMOVNP16`.
    fn cmovnp16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNP16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNP16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNP16");
        }
    }
    /// Emits `CMOVNP32`.
    fn cmovnp32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNP32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNP32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNP32");
        }
    }
    /// Emits `CMOVNP64`.
    fn cmovnp64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNP64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNP64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNP64");
        }
    }
    /// Emits `CMOVNS16`.
    fn cmovns16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNS16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNS16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNS16");
        }
    }
    /// Emits `CMOVNS32`.
    fn cmovns32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNS32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNS32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNS32");
        }
    }
    /// Emits `CMOVNS64`.
    fn cmovns64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNS64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNS64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNS64");
        }
    }
    /// Emits `CMOVNZ16`.
    fn cmovnz16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNZ16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNZ16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNZ16");
        }
    }
    /// Emits `CMOVNZ32`.
    fn cmovnz32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNZ32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNZ32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNZ32");
        }
    }
    /// Emits `CMOVNZ64`.
    fn cmovnz64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVNZ64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVNZ64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVNZ64");
        }
    }
    /// Emits `CMOVO16`.
    fn cmovo16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVO16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVO16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVO16");
        }
    }
    /// Emits `CMOVO32`.
    fn cmovo32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVO32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVO32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVO32");
        }
    }
    /// Emits `CMOVO64`.
    fn cmovo64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVO64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVO64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVO64");
        }
    }
    /// Emits `CMOVP16`.
    fn cmovp16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVP16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVP16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVP16");
        }
    }
    /// Emits `CMOVP32`.
    fn cmovp32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVP32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVP32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVP32");
        }
    }
    /// Emits `CMOVP64`.
    fn cmovp64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVP64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVP64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVP64");
        }
    }
    /// Emits `CMOVS16`.
    fn cmovs16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVS16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVS16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVS16");
        }
    }
    /// Emits `CMOVS32`.
    fn cmovs32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVS32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVS32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVS32");
        }
    }
    /// Emits `CMOVS64`.
    fn cmovs64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVS64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVS64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVS64");
        }
    }
    /// Emits `CMOVZ16`.
    fn cmovz16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVZ16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVZ16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVZ16");
        }
    }
    /// Emits `CMOVZ32`.
    fn cmovz32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVZ32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVZ32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVZ32");
        }
    }
    /// Emits `CMOVZ64`.
    fn cmovz64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVZ64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVZ64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVZ64");
        }
    }
    /// Emits `CMOVCC16`.
    fn cmovcc16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVCC16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVCC16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVCC16");
        }
    }
    /// Emits `CMOVCC32`.
    fn cmovcc32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVCC32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVCC32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVCC32");
        }
    }
    /// Emits `CMOVCC64`.
    fn cmovcc64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMOVCC64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMOVCC64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMOVCC64");
        }
    }
}
