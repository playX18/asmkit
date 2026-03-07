pub trait X86BaseEmitter: Emitter {
    /// Emits `AADD32MR`.
    fn aadd32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AADD32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AADD64MR`.
    fn aadd64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AADD64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AAND32MR`.
    fn aand32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AAND32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AAND64MR`.
    fn aand64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AAND64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `ADC16`.
    fn adc16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADC16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADC16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADC16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADC16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADC16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADC16");
        }
    }
    /// Emits `ADC32`.
    fn adc32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADC32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADC32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADC32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADC32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADC32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADC32");
        }
    }
    /// Emits `ADC64`.
    fn adc64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADC64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADC64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADC64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADC64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADC64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADC64");
        }
    }
    /// Emits `ADC8`.
    fn adc8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADC8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADC8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADC8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADC8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADC8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADC8");
        }
    }
    /// Emits `ADD16`.
    fn add16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADD16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADD16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADD16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADD16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADD16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADD16");
        }
    }
    /// Emits `ADD32`.
    fn add32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADD32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADD32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADD32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADD32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADD32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADD32");
        }
    }
    /// Emits `ADD64`.
    fn add64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADD64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADD64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADD64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADD64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADD64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADD64");
        }
    }
    /// Emits `ADD8`.
    fn add8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADD8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ADD8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADD8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(ADD8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ADD8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ADD8");
        }
    }
    /// Emits `AND16`.
    fn and16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(AND16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(AND16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(AND16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(AND16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(AND16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AND16");
        }
    }
    /// Emits `AND32`.
    fn and32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(AND32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(AND32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(AND32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(AND32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(AND32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AND32");
        }
    }
    /// Emits `AND64`.
    fn and64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(AND64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(AND64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(AND64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(AND64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(AND64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AND64");
        }
    }
    /// Emits `AND8`.
    fn and8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(AND8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(AND8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(AND8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(AND8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(AND8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AND8");
        }
    }
    /// Emits `AOR32MR`.
    fn aor32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AOR32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AOR64MR`.
    fn aor64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AOR64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AXOR32MR`.
    fn axor32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AXOR32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AXOR64MR`.
    fn axor64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AXOR64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `BSF16`.
    fn bsf16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSF16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSF16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSF16");
        }
    }
    /// Emits `BSF32`.
    fn bsf32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSF32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSF32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSF32");
        }
    }
    /// Emits `BSF64`.
    fn bsf64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSF64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSF64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSF64");
        }
    }
    /// Emits `BSR16`.
    fn bsr16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSR16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSR16");
        }
    }
    /// Emits `BSR32`.
    fn bsr32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSR32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSR32");
        }
    }
    /// Emits `BSR64`.
    fn bsr64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BSR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BSR64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BSR64");
        }
    }
    /// Emits `BT16`.
    fn bt16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BT16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BT16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BT16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BT16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BT16");
        }
    }
    /// Emits `BT32`.
    fn bt32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BT32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BT32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BT32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BT32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BT32");
        }
    }
    /// Emits `BT64`.
    fn bt64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BT64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BT64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BT64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BT64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BT64");
        }
    }
    /// Emits `BTC16`.
    fn btc16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(BTC16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTC16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(BTC16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTC16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTC16");
        }
    }
    /// Emits `BTC32`.
    fn btc32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(BTC32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTC32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(BTC32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTC32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTC32");
        }
    }
    /// Emits `BTC64`.
    fn btc64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(BTC64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTC64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(BTC64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTC64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTC64");
        }
    }
    /// Emits `BTR16`.
    fn btr16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTR16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTR16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTR16");
        }
    }
    /// Emits `BTR32`.
    fn btr32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTR32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTR32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTR32");
        }
    }
    /// Emits `BTR64`.
    fn btr64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTR64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTR64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTR64");
        }
    }
    /// Emits `BTS16`.
    fn bts16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTS16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTS16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTS16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTS16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTS16");
        }
    }
    /// Emits `BTS32`.
    fn bts32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTS32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTS32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTS32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTS32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTS32");
        }
    }
    /// Emits `BTS64`.
    fn bts64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BTS64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(BTS64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(BTS64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(BTS64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BTS64");
        }
    }
    /// Emits `CALL`.
    fn call(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(CALLR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(CALLM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            self.emit(CALL, op0,&NOREG,&NOREG,&NOREG);
        }
    }
    /// Emits `CALLF16M`.
    fn callf16(&mut self,op0: impl OperandCast) -> () {
        self.emit(CALLF16M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CALLF32M`.
    fn callf32(&mut self,op0: impl OperandCast) -> () {
        self.emit(CALLF32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CALLF64M`.
    fn callf64(&mut self,op0: impl OperandCast) -> () {
        self.emit(CALLF64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CBW`.
    fn cbw(&mut self,) -> () {
        self.emit(CBW, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CDQ`.
    fn cdq(&mut self,) -> () {
        self.emit(CDQ, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CDQE`.
    fn cdqe(&mut self,) -> () {
        self.emit(CDQE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLC`.
    fn clc(&mut self,) -> () {
        self.emit(CLC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLD`.
    fn cld(&mut self,) -> () {
        self.emit(CLD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLFLUSHM`.
    fn clflush(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLFLUSHM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLI`.
    fn cli(&mut self,) -> () {
        self.emit(CLI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLTS`.
    fn clts(&mut self,) -> () {
        self.emit(CLTS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMC`.
    fn cmc(&mut self,) -> () {
        self.emit(CMC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMP16`.
    fn cmp16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMP16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMP16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMP16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(CMP16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(CMP16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMP16");
        }
    }
    /// Emits `CMP32`.
    fn cmp32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMP32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMP32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMP32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(CMP32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(CMP32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMP32");
        }
    }
    /// Emits `CMP64`.
    fn cmp64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMP64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMP64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMP64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(CMP64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(CMP64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMP64");
        }
    }
    /// Emits `CMP8`.
    fn cmp8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(CMP8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(CMP8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(CMP8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(CMP8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(CMP8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for CMP8");
        }
    }
    /// Emits `CMPS16`.
    fn cmps16(&mut self,) -> () {
        self.emit(CMPS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPS32`.
    fn cmps32(&mut self,) -> () {
        self.emit(CMPS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPS64`.
    fn cmps64(&mut self,) -> () {
        self.emit(CMPS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPS8`.
    fn cmps8(&mut self,) -> () {
        self.emit(CMPS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CQO`.
    fn cqo(&mut self,) -> () {
        self.emit(CQO, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CWD`.
    fn cwd(&mut self,) -> () {
        self.emit(CWD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CWDE`.
    fn cwde(&mut self,) -> () {
        self.emit(CWDE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_EX16`.
    fn c_ex16(&mut self,) -> () {
        self.emit(C_EX16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_EX32`.
    fn c_ex32(&mut self,) -> () {
        self.emit(C_EX32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_EX64`.
    fn c_ex64(&mut self,) -> () {
        self.emit(C_EX64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_SEP16`.
    fn c_sep16(&mut self,) -> () {
        self.emit(C_SEP16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_SEP32`.
    fn c_sep32(&mut self,) -> () {
        self.emit(C_SEP32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `C_SEP64`.
    fn c_sep64(&mut self,) -> () {
        self.emit(C_SEP64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `DEC16`.
    fn dec16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DEC16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DEC16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DEC16");
        }
    }
    /// Emits `DEC32`.
    fn dec32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DEC32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DEC32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DEC32");
        }
    }
    /// Emits `DEC64`.
    fn dec64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DEC64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DEC64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DEC64");
        }
    }
    /// Emits `DEC8`.
    fn dec8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DEC8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DEC8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DEC8");
        }
    }
    /// Emits `DIV16`.
    fn div16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DIV16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DIV16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DIV16");
        }
    }
    /// Emits `DIV32`.
    fn div32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DIV32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DIV32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DIV32");
        }
    }
    /// Emits `DIV64`.
    fn div64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DIV64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DIV64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DIV64");
        }
    }
    /// Emits `DIV8`.
    fn div8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(DIV8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(DIV8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for DIV8");
        }
    }
    /// Emits `ENTERI`.
    fn enter(&mut self,op0: impl OperandCast) -> () {
        self.emit(ENTERI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENTER16I`.
    fn enter16(&mut self,op0: impl OperandCast) -> () {
        self.emit(ENTER16I, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FWAIT`.
    fn fwait(&mut self,) -> () {
        self.emit(FWAIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `HLT`.
    fn hlt(&mut self,) -> () {
        self.emit(HLT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IDIV16`.
    fn idiv16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IDIV16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IDIV16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IDIV16");
        }
    }
    /// Emits `IDIV32`.
    fn idiv32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IDIV32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IDIV32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IDIV32");
        }
    }
    /// Emits `IDIV64`.
    fn idiv64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IDIV64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IDIV64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IDIV64");
        }
    }
    /// Emits `IDIV8`.
    fn idiv8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IDIV8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IDIV8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IDIV8");
        }
    }
    /// Emits `IMUL16`.
    fn imul16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IMUL16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IMUL16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL16");
        }
    }
    /// Emits `IMUL16`.
    fn imul16_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(IMUL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(IMUL16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL16");
        }
    }
    /// Emits `IMUL16`.
    fn imul16_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(IMUL16RRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_imm() {
            self.emit(IMUL16RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL16");
        }
    }
    /// Emits `IMUL32`.
    fn imul32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IMUL32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IMUL32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL32");
        }
    }
    /// Emits `IMUL32`.
    fn imul32_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(IMUL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(IMUL32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL32");
        }
    }
    /// Emits `IMUL32`.
    fn imul32_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(IMUL32RRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_imm() {
            self.emit(IMUL32RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL32");
        }
    }
    /// Emits `IMUL64`.
    fn imul64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IMUL64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IMUL64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL64");
        }
    }
    /// Emits `IMUL64`.
    fn imul64_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(IMUL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(IMUL64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL64");
        }
    }
    /// Emits `IMUL64`.
    fn imul64_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(IMUL64RRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_imm() {
            self.emit(IMUL64RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL64");
        }
    }
    /// Emits `IMUL8`.
    fn imul8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(IMUL8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(IMUL8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for IMUL8");
        }
    }
    /// Emits `IN16`.
    fn in16(&mut self,) -> () {
        self.emit(IN16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IN16RI`.
    fn in16_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(IN16RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `IN32`.
    fn in32(&mut self,) -> () {
        self.emit(IN32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IN32RI`.
    fn in32_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(IN32RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `IN64`.
    fn in64(&mut self,) -> () {
        self.emit(IN64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IN64RI`.
    fn in64_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(IN64RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `IN8`.
    fn in8(&mut self,) -> () {
        self.emit(IN8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IN8RI`.
    fn in8_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(IN8RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `INC16`.
    fn inc16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(INC16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(INC16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for INC16");
        }
    }
    /// Emits `INC32`.
    fn inc32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(INC32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(INC32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for INC32");
        }
    }
    /// Emits `INC64`.
    fn inc64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(INC64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(INC64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for INC64");
        }
    }
    /// Emits `INC8`.
    fn inc8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(INC8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(INC8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for INC8");
        }
    }
    /// Emits `INS16`.
    fn ins16(&mut self,) -> () {
        self.emit(INS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INS32`.
    fn ins32(&mut self,) -> () {
        self.emit(INS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INS64`.
    fn ins64(&mut self,) -> () {
        self.emit(INS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INS8`.
    fn ins8(&mut self,) -> () {
        self.emit(INS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INTI`.
    fn int(&mut self,op0: impl OperandCast) -> () {
        self.emit(INTI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INT1`.
    fn int1(&mut self,) -> () {
        self.emit(INT1, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INT3`.
    fn int3(&mut self,) -> () {
        self.emit(INT3, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IRET16`.
    fn iret16(&mut self,) -> () {
        self.emit(IRET16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IRET32`.
    fn iret32(&mut self,) -> () {
        self.emit(IRET32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `IRET64`.
    fn iret64(&mut self,) -> () {
        self.emit(IRET64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JA`.
    fn ja(&mut self,op0: impl OperandCast) -> () {
        self.emit(JA, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JBE`.
    fn jbe(&mut self,op0: impl OperandCast) -> () {
        self.emit(JBE, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JC`.
    fn jc(&mut self,op0: impl OperandCast) -> () {
        self.emit(JC, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JCXZ`.
    fn jcxz(&mut self,op0: impl OperandCast) -> () {
        self.emit(JCXZ, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JG`.
    fn jg(&mut self,op0: impl OperandCast) -> () {
        self.emit(JG, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JGE`.
    fn jge(&mut self,op0: impl OperandCast) -> () {
        self.emit(JGE, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JL`.
    fn jl(&mut self,op0: impl OperandCast) -> () {
        self.emit(JL, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JLE`.
    fn jle(&mut self,op0: impl OperandCast) -> () {
        self.emit(JLE, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JMP`.
    fn jmp(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(JMPR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(JMPM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            self.emit(JMP, op0,&NOREG,&NOREG,&NOREG);
        }
    }
    /// Emits `JMPF16M`.
    fn jmpf16(&mut self,op0: impl OperandCast) -> () {
        self.emit(JMPF16M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JMPF32M`.
    fn jmpf32(&mut self,op0: impl OperandCast) -> () {
        self.emit(JMPF32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JMPF64M`.
    fn jmpf64(&mut self,op0: impl OperandCast) -> () {
        self.emit(JMPF64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JNC`.
    fn jnc(&mut self,op0: impl OperandCast) -> () {
        self.emit(JNC, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JNO`.
    fn jno(&mut self,op0: impl OperandCast) -> () {
        self.emit(JNO, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JNP`.
    fn jnp(&mut self,op0: impl OperandCast) -> () {
        self.emit(JNP, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JNS`.
    fn jns(&mut self,op0: impl OperandCast) -> () {
        self.emit(JNS, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JNZ`.
    fn jnz(&mut self,op0: impl OperandCast) -> () {
        self.emit(JNZ, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JO`.
    fn jo(&mut self,op0: impl OperandCast) -> () {
        self.emit(JO, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JP`.
    fn jp(&mut self,op0: impl OperandCast) -> () {
        self.emit(JP, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JS`.
    fn js(&mut self,op0: impl OperandCast) -> () {
        self.emit(JS, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JZ`.
    fn jz(&mut self,op0: impl OperandCast) -> () {
        self.emit(JZ, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `JCC`.
    fn jcc(&mut self,op0: impl OperandCast) -> () {
        self.emit(JCC, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LAHF`.
    fn lahf(&mut self,) -> () {
        self.emit(LAHF, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LAR16`.
    fn lar16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LAR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LAR16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LAR16");
        }
    }
    /// Emits `LAR32`.
    fn lar32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LAR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LAR32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LAR32");
        }
    }
    /// Emits `LAR64`.
    fn lar64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LAR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LAR64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LAR64");
        }
    }
    /// Emits `LDTILECFGM`.
    fn ldtilecfg(&mut self,op0: impl OperandCast) -> () {
        self.emit(LDTILECFGM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LEA16RM`.
    fn lea16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LEA16RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LEA32RM`.
    fn lea32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LEA32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LEA64RM`.
    fn lea64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LEA64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LEAVE`.
    fn leave(&mut self,) -> () {
        self.emit(LEAVE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LEAVE16`.
    fn leave16(&mut self,) -> () {
        self.emit(LEAVE16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LFS16RM`.
    fn lfs16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LFS16RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LFS32RM`.
    fn lfs32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LFS32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LFS64RM`.
    fn lfs64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LFS64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LGDTM`.
    fn lgdt(&mut self,op0: impl OperandCast) -> () {
        self.emit(LGDTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LGS16RM`.
    fn lgs16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LGS16RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LGS32RM`.
    fn lgs32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LGS32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LGS64RM`.
    fn lgs64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LGS64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LIDTM`.
    fn lidt(&mut self,op0: impl OperandCast) -> () {
        self.emit(LIDTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LLDT`.
    fn lldt(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(LLDTR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(LLDTM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LLDT");
        }
    }
    /// Emits `LMSW`.
    fn lmsw(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(LMSWR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(LMSWM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LMSW");
        }
    }
    /// Emits `LODS16`.
    fn lods16(&mut self,) -> () {
        self.emit(LODS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LODS32`.
    fn lods32(&mut self,) -> () {
        self.emit(LODS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LODS64`.
    fn lods64(&mut self,) -> () {
        self.emit(LODS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LODS8`.
    fn lods8(&mut self,) -> () {
        self.emit(LODS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LOOP`.
    fn r#loop(&mut self,op0: impl OperandCast) -> () {
        self.emit(LOOP, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LOOPNZ`.
    fn loopnz(&mut self,op0: impl OperandCast) -> () {
        self.emit(LOOPNZ, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LOOPZ`.
    fn loopz(&mut self,op0: impl OperandCast) -> () {
        self.emit(LOOPZ, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LSL16`.
    fn lsl16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LSL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LSL16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LSL16");
        }
    }
    /// Emits `LSL32`.
    fn lsl32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LSL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LSL32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LSL32");
        }
    }
    /// Emits `LSL64`.
    fn lsl64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LSL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LSL64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LSL64");
        }
    }
    /// Emits `LSS16RM`.
    fn lss16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LSS16RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LSS32RM`.
    fn lss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LSS32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LSS64RM`.
    fn lss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LSS64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LTR`.
    fn ltr(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(LTRR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(LTRM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LTR");
        }
    }
    /// Emits `MOV16`.
    fn mov16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(MOV16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOV16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(MOV16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(MOV16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(MOV16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV16");
        }
    }
    /// Emits `MOV16A`.
    fn mov16a(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() {
            self.emit(MOV16RA, op0,op1,&NOREG,&NOREG);
        } else if op1.is_gp() {
            self.emit(MOV16AR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV16A");
        }
    }
    /// Emits `MOV32`.
    fn mov32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(MOV32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOV32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(MOV32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(MOV32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(MOV32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV32");
        }
    }
    /// Emits `MOV32A`.
    fn mov32a(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() {
            self.emit(MOV32RA, op0,op1,&NOREG,&NOREG);
        } else if op1.is_gp() {
            self.emit(MOV32AR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV32A");
        }
    }
    /// Emits `MOV64`.
    fn mov64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(MOV64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOV64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(MOV64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(MOV64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(MOV64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV64");
        }
    }
    /// Emits `MOV64A`.
    fn mov64a(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() {
            self.emit(MOV64RA, op0,op1,&NOREG,&NOREG);
        } else if op1.is_gp() {
            self.emit(MOV64AR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV64A");
        }
    }
    /// Emits `MOV8`.
    fn mov8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(MOV8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOV8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(MOV8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(MOV8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(MOV8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV8");
        }
    }
    /// Emits `MOV8A`.
    fn mov8a(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() {
            self.emit(MOV8RA, op0,op1,&NOREG,&NOREG);
        } else if op1.is_gp() {
            self.emit(MOV8AR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV8A");
        }
    }
    /// Emits `MOVS16`.
    fn movs16(&mut self,) -> () {
        self.emit(MOVS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MOVS32`.
    fn movs32(&mut self,) -> () {
        self.emit(MOVS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MOVS64`.
    fn movs64(&mut self,) -> () {
        self.emit(MOVS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MOVS8`.
    fn movs8(&mut self,) -> () {
        self.emit(MOVS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16M16`.
    fn movsxr16m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16M32`.
    fn movsxr16m32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16M32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16M8`.
    fn movsxr16m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16R16`.
    fn movsxr16r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16R32`.
    fn movsxr16r32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16R32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR16R8`.
    fn movsxr16r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR16R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32M16`.
    fn movsxr32m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32M32`.
    fn movsxr32m32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32M32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32M8`.
    fn movsxr32m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32R16`.
    fn movsxr32r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32R32`.
    fn movsxr32r32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32R32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR32R8`.
    fn movsxr32r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR32R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64M16`.
    fn movsxr64m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64M32`.
    fn movsxr64m32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64M32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64M8`.
    fn movsxr64m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64R16`.
    fn movsxr64r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64R32`.
    fn movsxr64r32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64R32, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVSXR64R8`.
    fn movsxr64r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVSXR64R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR16M16`.
    fn movzxr16m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR16M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR16M8`.
    fn movzxr16m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR16M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR16R16`.
    fn movzxr16r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR16R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR16R8`.
    fn movzxr16r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR16R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR32M16`.
    fn movzxr32m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR32M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR32M8`.
    fn movzxr32m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR32M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR32R16`.
    fn movzxr32r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR32R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR32R8`.
    fn movzxr32r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR32R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR64M16`.
    fn movzxr64m16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR64M16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR64M8`.
    fn movzxr64m8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR64M8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR64R16`.
    fn movzxr64r16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR64R16, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVZXR64R8`.
    fn movzxr64r8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVZXR64R8, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOV_CR2GRR`.
    fn mov_cr2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOV_CR2GRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOV_DR2GRR`.
    fn mov_dr2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOV_DR2GRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOV_G2CRRR`.
    fn mov_g2cr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOV_G2CRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOV_G2DRRR`.
    fn mov_g2dr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOV_G2DRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOV_G2S`.
    fn mov_g2s(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_reg_group_of(RegGroup::X86SReg) && op1.is_gp() {
            self.emit(MOV_G2SRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86SReg) && op1.is_mem() {
            self.emit(MOV_G2SRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV_G2S");
        }
    }
    /// Emits `MOV_S2G`.
    fn mov_s2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_reg_group_of(RegGroup::X86SReg) {
            self.emit(MOV_S2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_reg_group_of(RegGroup::X86SReg) {
            self.emit(MOV_S2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOV_S2G");
        }
    }
    /// Emits `MUL16`.
    fn mul16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(MUL16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(MUL16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MUL16");
        }
    }
    /// Emits `MUL32`.
    fn mul32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(MUL32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(MUL32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MUL32");
        }
    }
    /// Emits `MUL64`.
    fn mul64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(MUL64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(MUL64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MUL64");
        }
    }
    /// Emits `MUL8`.
    fn mul8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(MUL8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(MUL8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MUL8");
        }
    }
    /// Emits `NEG16`.
    fn neg16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NEG16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NEG16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NEG16");
        }
    }
    /// Emits `NEG32`.
    fn neg32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NEG32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NEG32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NEG32");
        }
    }
    /// Emits `NEG64`.
    fn neg64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NEG64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NEG64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NEG64");
        }
    }
    /// Emits `NEG8`.
    fn neg8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NEG8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NEG8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NEG8");
        }
    }
    /// Emits `NOP`.
    fn nop(&mut self,) -> () {
        self.emit(NOP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `NOP16`.
    fn nop16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOP16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOP16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOP16");
        }
    }
    /// Emits `NOP32`.
    fn nop32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOP32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOP32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOP32");
        }
    }
    /// Emits `NOP64`.
    fn nop64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOP64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOP64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOP64");
        }
    }
    /// Emits `NOT16`.
    fn not16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOT16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOT16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOT16");
        }
    }
    /// Emits `NOT32`.
    fn not32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOT32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOT32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOT32");
        }
    }
    /// Emits `NOT64`.
    fn not64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOT64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOT64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOT64");
        }
    }
    /// Emits `NOT8`.
    fn not8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(NOT8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(NOT8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for NOT8");
        }
    }
    /// Emits `OR16`.
    fn or16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(OR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(OR16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(OR16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(OR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(OR16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for OR16");
        }
    }
    /// Emits `OR32`.
    fn or32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(OR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(OR32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(OR32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(OR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(OR32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for OR32");
        }
    }
    /// Emits `OR64`.
    fn or64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(OR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(OR64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(OR64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(OR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(OR64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for OR64");
        }
    }
    /// Emits `OR8`.
    fn or8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(OR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(OR8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(OR8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(OR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(OR8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for OR8");
        }
    }
    /// Emits `OUT16`.
    fn out16(&mut self,) -> () {
        self.emit(OUT16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUT16RI`.
    fn out16_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(OUT16RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `OUT32`.
    fn out32(&mut self,) -> () {
        self.emit(OUT32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUT32RI`.
    fn out32_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(OUT32RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `OUT64`.
    fn out64(&mut self,) -> () {
        self.emit(OUT64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUT64RI`.
    fn out64_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(OUT64RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `OUT8`.
    fn out8(&mut self,) -> () {
        self.emit(OUT8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUT8RI`.
    fn out8_2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(OUT8RI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `OUTS16`.
    fn outs16(&mut self,) -> () {
        self.emit(OUTS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUTS32`.
    fn outs32(&mut self,) -> () {
        self.emit(OUTS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUTS64`.
    fn outs64(&mut self,) -> () {
        self.emit(OUTS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `OUTS8`.
    fn outs8(&mut self,) -> () {
        self.emit(OUTS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PAUSE`.
    fn pause(&mut self,) -> () {
        self.emit(PAUSE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `POP`.
    fn pop(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(POPR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(POPM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for POP");
        }
    }
    /// Emits `POP16`.
    fn pop16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(POP16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(POP16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for POP16");
        }
    }
    /// Emits `POPF`.
    fn popf(&mut self,) -> () {
        self.emit(POPF, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `POPF16`.
    fn popf16(&mut self,) -> () {
        self.emit(POPF16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `POP_SEGR`.
    fn pop_seg(&mut self,op0: impl OperandCast) -> () {
        self.emit(POP_SEGR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `POP_SEG16R`.
    fn pop_seg16(&mut self,op0: impl OperandCast) -> () {
        self.emit(POP_SEG16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PUSH`.
    fn push(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PUSHR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_imm() {
            self.emit(PUSHI, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PUSHM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for PUSH");
        }
    }
    /// Emits `PUSH16`.
    fn push16(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PUSH16R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_imm() {
            self.emit(PUSH16I, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PUSH16M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for PUSH16");
        }
    }
    /// Emits `PUSHF`.
    fn pushf(&mut self,) -> () {
        self.emit(PUSHF, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PUSHF16`.
    fn pushf16(&mut self,) -> () {
        self.emit(PUSHF16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PUSH_SEGR`.
    fn push_seg(&mut self,op0: impl OperandCast) -> () {
        self.emit(PUSH_SEGR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PUSH_SEG16R`.
    fn push_seg16(&mut self,op0: impl OperandCast) -> () {
        self.emit(PUSH_SEG16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RCL16`.
    fn rcl16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCL16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCL16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCL16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCL16");
        }
    }
    /// Emits `RCL32`.
    fn rcl32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCL32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCL32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCL32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCL32");
        }
    }
    /// Emits `RCL64`.
    fn rcl64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCL64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCL64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCL64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCL64");
        }
    }
    /// Emits `RCL8`.
    fn rcl8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCL8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCL8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCL8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCL8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCL8");
        }
    }
    /// Emits `RCR16`.
    fn rcr16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCR16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCR16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCR16");
        }
    }
    /// Emits `RCR32`.
    fn rcr32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCR32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCR32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCR32");
        }
    }
    /// Emits `RCR64`.
    fn rcr64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCR64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCR64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCR64");
        }
    }
    /// Emits `RCR8`.
    fn rcr8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(RCR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(RCR8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(RCR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(RCR8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for RCR8");
        }
    }
    /// Emits `RET`.
    fn ret(&mut self,) -> () {
        self.emit(RET, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETI`.
    fn ret_1(&mut self,op0: impl OperandCast) -> () {
        self.emit(RETI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF16`.
    fn retf16(&mut self,) -> () {
        self.emit(RETF16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF16I`.
    fn retf16_1(&mut self,op0: impl OperandCast) -> () {
        self.emit(RETF16I, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF32`.
    fn retf32(&mut self,) -> () {
        self.emit(RETF32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF32I`.
    fn retf32_1(&mut self,op0: impl OperandCast) -> () {
        self.emit(RETF32I, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF64`.
    fn retf64(&mut self,) -> () {
        self.emit(RETF64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RETF64I`.
    fn retf64_1(&mut self,op0: impl OperandCast) -> () {
        self.emit(RETF64I, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ROL16`.
    fn rol16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROL16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROL16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROL16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROL16");
        }
    }
    /// Emits `ROL32`.
    fn rol32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROL32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROL32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROL32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROL32");
        }
    }
    /// Emits `ROL64`.
    fn rol64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROL64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROL64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROL64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROL64");
        }
    }
    /// Emits `ROL8`.
    fn rol8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROL8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROL8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROL8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROL8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROL8");
        }
    }
    /// Emits `ROR16`.
    fn ror16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROR16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROR16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROR16");
        }
    }
    /// Emits `ROR32`.
    fn ror32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROR32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROR32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROR32");
        }
    }
    /// Emits `ROR64`.
    fn ror64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROR64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROR64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROR64");
        }
    }
    /// Emits `ROR8`.
    fn ror8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(ROR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(ROR8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(ROR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(ROR8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for ROR8");
        }
    }
    /// Emits `SAHF`.
    fn sahf(&mut self,) -> () {
        self.emit(SAHF, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SAR16`.
    fn sar16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SAR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SAR16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SAR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SAR16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SAR16");
        }
    }
    /// Emits `SAR32`.
    fn sar32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SAR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SAR32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SAR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SAR32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SAR32");
        }
    }
    /// Emits `SAR64`.
    fn sar64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SAR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SAR64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SAR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SAR64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SAR64");
        }
    }
    /// Emits `SAR8`.
    fn sar8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SAR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SAR8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SAR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SAR8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SAR8");
        }
    }
    /// Emits `SBB16`.
    fn sbb16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SBB16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SBB16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SBB16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SBB16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SBB16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SBB16");
        }
    }
    /// Emits `SBB32`.
    fn sbb32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SBB32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SBB32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SBB32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SBB32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SBB32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SBB32");
        }
    }
    /// Emits `SBB64`.
    fn sbb64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SBB64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SBB64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SBB64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SBB64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SBB64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SBB64");
        }
    }
    /// Emits `SBB8`.
    fn sbb8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SBB8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SBB8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SBB8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SBB8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SBB8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SBB8");
        }
    }
    /// Emits `SCAS16`.
    fn scas16(&mut self,) -> () {
        self.emit(SCAS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SCAS32`.
    fn scas32(&mut self,) -> () {
        self.emit(SCAS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SCAS64`.
    fn scas64(&mut self,) -> () {
        self.emit(SCAS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SCAS8`.
    fn scas8(&mut self,) -> () {
        self.emit(SCAS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SETA8`.
    fn seta8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETA8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETA8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETA8");
        }
    }
    /// Emits `SETBE8`.
    fn setbe8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETBE8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETBE8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETBE8");
        }
    }
    /// Emits `SETC8`.
    fn setc8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETC8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETC8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETC8");
        }
    }
    /// Emits `SETG8`.
    fn setg8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETG8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETG8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETG8");
        }
    }
    /// Emits `SETGE8`.
    fn setge8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETGE8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETGE8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETGE8");
        }
    }
    /// Emits `SETL8`.
    fn setl8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETL8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETL8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETL8");
        }
    }
    /// Emits `SETLE8`.
    fn setle8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETLE8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETLE8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETLE8");
        }
    }
    /// Emits `SETNC8`.
    fn setnc8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETNC8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETNC8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETNC8");
        }
    }
    /// Emits `SETNO8`.
    fn setno8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETNO8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETNO8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETNO8");
        }
    }
    /// Emits `SETNP8`.
    fn setnp8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETNP8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETNP8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETNP8");
        }
    }
    /// Emits `SETNS8`.
    fn setns8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETNS8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETNS8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETNS8");
        }
    }
    /// Emits `SETNZ8`.
    fn setnz8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETNZ8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETNZ8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETNZ8");
        }
    }
    /// Emits `SETO8`.
    fn seto8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETO8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETO8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETO8");
        }
    }
    /// Emits `SETP8`.
    fn setp8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETP8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETP8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETP8");
        }
    }
    /// Emits `SETS8`.
    fn sets8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETS8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETS8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETS8");
        }
    }
    /// Emits `SETZ8`.
    fn setz8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETZ8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETZ8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETZ8");
        }
    }
    /// Emits `SETCC8`.
    fn setcc8(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SETCC8R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SETCC8M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SETCC8");
        }
    }
    /// Emits `SGDTM`.
    fn sgdt(&mut self,op0: impl OperandCast) -> () {
        self.emit(SGDTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SHL16`.
    fn shl16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHL16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHL16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHL16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHL16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHL16");
        }
    }
    /// Emits `SHL32`.
    fn shl32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHL32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHL32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHL32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHL32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHL32");
        }
    }
    /// Emits `SHL64`.
    fn shl64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHL64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHL64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHL64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHL64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHL64");
        }
    }
    /// Emits `SHL8`.
    fn shl8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHL8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHL8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHL8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHL8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHL8");
        }
    }
    /// Emits `SHLD16`.
    fn shld16(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD16RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD16MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD16RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD16MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHLD16");
        }
    }
    /// Emits `SHLD32`.
    fn shld32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD32RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD32MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD32MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHLD32");
        }
    }
    /// Emits `SHLD64`.
    fn shld64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD64RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHLD64MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLD64MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHLD64");
        }
    }
    /// Emits `SHR16`.
    fn shr16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHR16MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHR16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHR16");
        }
    }
    /// Emits `SHR32`.
    fn shr32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHR32MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHR32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHR32");
        }
    }
    /// Emits `SHR64`.
    fn shr64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHR64MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHR64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHR64");
        }
    }
    /// Emits `SHR8`.
    fn shr8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_imm() {
            self.emit(SHR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SHR8MI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_gp() {
            self.emit(SHR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SHR8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHR8");
        }
    }
    /// Emits `SHRD16`.
    fn shrd16(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD16RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD16MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD16RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD16MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHRD16");
        }
    }
    /// Emits `SHRD32`.
    fn shrd32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD32RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD32MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD32MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHRD32");
        }
    }
    /// Emits `SHRD64`.
    fn shrd64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD64RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_imm() {
            self.emit(SHRD64MRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRD64MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHRD64");
        }
    }
    /// Emits `SIDTM`.
    fn sidt(&mut self,op0: impl OperandCast) -> () {
        self.emit(SIDTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SLDT`.
    fn sldt(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(SLDTR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(SLDTM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SLDT");
        }
    }
    /// Emits `SMSWM`.
    fn smsw(&mut self,op0: impl OperandCast) -> () {
        self.emit(SMSWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SMSW16R`.
    fn smsw16(&mut self,op0: impl OperandCast) -> () {
        self.emit(SMSW16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SMSW32R`.
    fn smsw32(&mut self,op0: impl OperandCast) -> () {
        self.emit(SMSW32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SMSW64R`.
    fn smsw64(&mut self,op0: impl OperandCast) -> () {
        self.emit(SMSW64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STC`.
    fn stc(&mut self,) -> () {
        self.emit(STC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STD`.
    fn std(&mut self,) -> () {
        self.emit(STD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STI`.
    fn sti(&mut self,) -> () {
        self.emit(STI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STOS16`.
    fn stos16(&mut self,) -> () {
        self.emit(STOS16, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STOS32`.
    fn stos32(&mut self,) -> () {
        self.emit(STOS32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STOS64`.
    fn stos64(&mut self,) -> () {
        self.emit(STOS64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STOS8`.
    fn stos8(&mut self,) -> () {
        self.emit(STOS8, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STR`.
    fn str(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(STRR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(STRM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for STR");
        }
    }
    /// Emits `STTILECFGM`.
    fn sttilecfg(&mut self,op0: impl OperandCast) -> () {
        self.emit(STTILECFGM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SUB16`.
    fn sub16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SUB16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SUB16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SUB16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SUB16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SUB16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SUB16");
        }
    }
    /// Emits `SUB32`.
    fn sub32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SUB32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SUB32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SUB32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SUB32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SUB32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SUB32");
        }
    }
    /// Emits `SUB64`.
    fn sub64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SUB64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SUB64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SUB64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SUB64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SUB64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SUB64");
        }
    }
    /// Emits `SUB8`.
    fn sub8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(SUB8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(SUB8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(SUB8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(SUB8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(SUB8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SUB8");
        }
    }
    /// Emits `SWAPGS`.
    fn swapgs(&mut self,) -> () {
        self.emit(SWAPGS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SYSCALL`.
    fn syscall(&mut self,) -> () {
        self.emit(SYSCALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SYSRET`.
    fn sysret(&mut self,) -> () {
        self.emit(SYSRET, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `TCMMIMFP16PSRRR`.
    fn tcmmimfp16ps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TCMMIMFP16PSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TCMMRLFP16PSRRR`.
    fn tcmmrlfp16ps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TCMMRLFP16PSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPBF16PSRRR`.
    fn tdpbf16ps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPBF16PSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPBSSDRRR`.
    fn tdpbssd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPBSSDRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPBSUDRRR`.
    fn tdpbsud(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPBSUDRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPBUSDRRR`.
    fn tdpbusd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPBUSDRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPBUUDRRR`.
    fn tdpbuud(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPBUUDRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TDPFP16PSRRR`.
    fn tdpfp16ps(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(TDPFP16PSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `TEST16`.
    fn test16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TEST16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(TEST16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(TEST16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(TEST16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TEST16");
        }
    }
    /// Emits `TEST32`.
    fn test32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TEST32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(TEST32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(TEST32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(TEST32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TEST32");
        }
    }
    /// Emits `TEST64`.
    fn test64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TEST64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(TEST64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(TEST64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(TEST64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TEST64");
        }
    }
    /// Emits `TEST8`.
    fn test8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TEST8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(TEST8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(TEST8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(TEST8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TEST8");
        }
    }
    /// Emits `TILELOADDRM`.
    fn tileloadd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TILELOADDRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `TILELOADDT1RM`.
    fn tileloaddt1(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TILELOADDT1RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `TILERELEASE`.
    fn tilerelease(&mut self,) -> () {
        self.emit(TILERELEASE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `TILESTOREDMR`.
    fn tilestored(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TILESTOREDMR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `TILEZEROR`.
    fn tilezero(&mut self,op0: impl OperandCast) -> () {
        self.emit(TILEZEROR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `UD0_16`.
    fn ud0_16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD0_16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD0_16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD0_16");
        }
    }
    /// Emits `UD0_32`.
    fn ud0_32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD0_32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD0_32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD0_32");
        }
    }
    /// Emits `UD0_64`.
    fn ud0_64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD0_64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD0_64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD0_64");
        }
    }
    /// Emits `UD1_16`.
    fn ud1_16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD1_16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD1_16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD1_16");
        }
    }
    /// Emits `UD1_32`.
    fn ud1_32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD1_32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD1_32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD1_32");
        }
    }
    /// Emits `UD1_64`.
    fn ud1_64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UD1_64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(UD1_64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UD1_64");
        }
    }
    /// Emits `UD2`.
    fn ud2(&mut self,) -> () {
        self.emit(UD2, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VADDPH128`.
    fn vaddph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH128");
        }
    }
    /// Emits `VADDPH128_MASK`.
    fn vaddph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH128_MASK");
        }
    }
    /// Emits `VADDPH128_MASKZ`.
    fn vaddph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH128_MASKZ");
        }
    }
    /// Emits `VADDPH128RRB`.
    fn vaddph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH128RRB_MASK`.
    fn vaddph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH128RRB_MASKZ`.
    fn vaddph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH256`.
    fn vaddph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH256");
        }
    }
    /// Emits `VADDPH256_MASK`.
    fn vaddph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH256_MASK");
        }
    }
    /// Emits `VADDPH256_MASKZ`.
    fn vaddph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH256_MASKZ");
        }
    }
    /// Emits `VADDPH256RRB`.
    fn vaddph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH256RRB_MASK`.
    fn vaddph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH256RRB_MASKZ`.
    fn vaddph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512`.
    fn vaddph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH512");
        }
    }
    /// Emits `VADDPH512RRR_ER`.
    fn vaddph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512_MASK`.
    fn vaddph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH512_MASK");
        }
    }
    /// Emits `VADDPH512RRR_MASK_ER`.
    fn vaddph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512_MASKZ`.
    fn vaddph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDPH512_MASKZ");
        }
    }
    /// Emits `VADDPH512RRR_MASKZ_ER`.
    fn vaddph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512RRB`.
    fn vaddph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512RRB_MASK`.
    fn vaddph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDPH512RRB_MASKZ`.
    fn vaddph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDSH`.
    fn vaddsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSH");
        }
    }
    /// Emits `VADDSHRRR_ER`.
    fn vaddsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDSH_MASK`.
    fn vaddsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSH_MASK");
        }
    }
    /// Emits `VADDSHRRR_MASK_ER`.
    fn vaddsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VADDSH_MASKZ`.
    fn vaddsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSH_MASKZ");
        }
    }
    /// Emits `VADDSHRRR_MASKZ_ER`.
    fn vaddsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VAESDEC128`.
    fn vaesdec128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDEC128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDEC128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDEC128");
        }
    }
    /// Emits `VAESDEC256`.
    fn vaesdec256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDEC256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDEC256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDEC256");
        }
    }
    /// Emits `VAESDEC512`.
    fn vaesdec512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDEC512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDEC512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDEC512");
        }
    }
    /// Emits `VAESDECLAST128`.
    fn vaesdeclast128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDECLAST128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDECLAST128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDECLAST128");
        }
    }
    /// Emits `VAESDECLAST256`.
    fn vaesdeclast256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDECLAST256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDECLAST256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDECLAST256");
        }
    }
    /// Emits `VAESDECLAST512`.
    fn vaesdeclast512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESDECLAST512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESDECLAST512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESDECLAST512");
        }
    }
    /// Emits `VAESENC128`.
    fn vaesenc128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENC128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENC128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENC128");
        }
    }
    /// Emits `VAESENC256`.
    fn vaesenc256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENC256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENC256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENC256");
        }
    }
    /// Emits `VAESENC512`.
    fn vaesenc512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENC512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENC512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENC512");
        }
    }
    /// Emits `VAESENCLAST128`.
    fn vaesenclast128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENCLAST128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENCLAST128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENCLAST128");
        }
    }
    /// Emits `VAESENCLAST256`.
    fn vaesenclast256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENCLAST256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENCLAST256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENCLAST256");
        }
    }
    /// Emits `VAESENCLAST512`.
    fn vaesenclast512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VAESENCLAST512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VAESENCLAST512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESENCLAST512");
        }
    }
    /// Emits `VAESIMC`.
    fn vaesimc(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VAESIMCRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VAESIMCRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESIMC");
        }
    }
    /// Emits `VAESKEYGENASSIST`.
    fn vaeskeygenassist(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VAESKEYGENASSISTRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VAESKEYGENASSISTRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VAESKEYGENASSIST");
        }
    }
    /// Emits `VBCSTNEBF162PS128RM`.
    fn vbcstnebf162ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBCSTNEBF162PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VBCSTNEBF162PS256RM`.
    fn vbcstnebf162ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBCSTNEBF162PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VBCSTNESH2PS128RM`.
    fn vbcstnesh2ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBCSTNESH2PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VBCSTNESH2PS256RM`.
    fn vbcstnesh2ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBCSTNESH2PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCMPPH128K`.
    fn vcmpph128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH128KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH128KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH128K");
        }
    }
    /// Emits `VCMPPH128K_MASK`.
    fn vcmpph128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH128KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH128KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH128K_MASK");
        }
    }
    /// Emits `VCMPPH128KRBI`.
    fn vcmpph128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH128KRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH128KRBI_MASK`.
    fn vcmpph128kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH128KRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH256K`.
    fn vcmpph256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH256KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH256KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH256K");
        }
    }
    /// Emits `VCMPPH256K_MASK`.
    fn vcmpph256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH256KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH256KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH256K_MASK");
        }
    }
    /// Emits `VCMPPH256KRBI`.
    fn vcmpph256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH256KRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH256KRBI_MASK`.
    fn vcmpph256kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH256KRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH512K`.
    fn vcmpph512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH512KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH512KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH512K");
        }
    }
    /// Emits `VCMPPH512K_MASK`.
    fn vcmpph512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPH512KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPH512KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPH512K_MASK");
        }
    }
    /// Emits `VCMPPH512KRRI_MASK_SAE`.
    fn vcmpph512k_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH512KRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH512KRRI_SAE`.
    fn vcmpph512k_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH512KRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH512KRBI`.
    fn vcmpph512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH512KRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPPH512KRBI_MASK`.
    fn vcmpph512kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPH512KRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPSHK`.
    fn vcmpshk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPSHKRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPSHKRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPSHK");
        }
    }
    /// Emits `VCMPSHK_MASK`.
    fn vcmpshk_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPSHKRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPSHKRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPSHK_MASK");
        }
    }
    /// Emits `VCMPSHKRRI_MASK_SAE`.
    fn vcmpshk_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSHKRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCMPSHKRRI_SAE`.
    fn vcmpshk_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSHKRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VCOMISH`.
    fn vcomish(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCOMISHRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCOMISHRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCOMISH");
        }
    }
    /// Emits `VCOMISHRR_SAE`.
    fn vcomish_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCOMISHRR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH128`.
    fn vcvtdq2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH128");
        }
    }
    /// Emits `VCVTDQ2PH128_MASK`.
    fn vcvtdq2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH128_MASK");
        }
    }
    /// Emits `VCVTDQ2PH128_MASKZ`.
    fn vcvtdq2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH128_MASKZ");
        }
    }
    /// Emits `VCVTDQ2PH128RB`.
    fn vcvtdq2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH128RB_MASK`.
    fn vcvtdq2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH128RB_MASKZ`.
    fn vcvtdq2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH256`.
    fn vcvtdq2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH256");
        }
    }
    /// Emits `VCVTDQ2PH256_MASK`.
    fn vcvtdq2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH256_MASK");
        }
    }
    /// Emits `VCVTDQ2PH256_MASKZ`.
    fn vcvtdq2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH256_MASKZ");
        }
    }
    /// Emits `VCVTDQ2PH256RB`.
    fn vcvtdq2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH256RB_MASK`.
    fn vcvtdq2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH256RB_MASKZ`.
    fn vcvtdq2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512`.
    fn vcvtdq2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH512");
        }
    }
    /// Emits `VCVTDQ2PH512RR_ER`.
    fn vcvtdq2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512_MASK`.
    fn vcvtdq2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH512_MASK");
        }
    }
    /// Emits `VCVTDQ2PH512RR_MASK_ER`.
    fn vcvtdq2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512_MASKZ`.
    fn vcvtdq2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTDQ2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTDQ2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTDQ2PH512_MASKZ");
        }
    }
    /// Emits `VCVTDQ2PH512RR_MASKZ_ER`.
    fn vcvtdq2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512RB`.
    fn vcvtdq2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512RB_MASK`.
    fn vcvtdq2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTDQ2PH512RB_MASKZ`.
    fn vcvtdq2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTDQ2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEEBF162PS128RM`.
    fn vcvtneebf162ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEEBF162PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEEBF162PS256RM`.
    fn vcvtneebf162ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEEBF162PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEEPH2PS128RM`.
    fn vcvtneeph2ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEEPH2PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEEPH2PS256RM`.
    fn vcvtneeph2ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEEPH2PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEOBF162PS128RM`.
    fn vcvtneobf162ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEOBF162PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEOBF162PS256RM`.
    fn vcvtneobf162ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEOBF162PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEOPH2PS128RM`.
    fn vcvtneoph2ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEOPH2PS128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEOPH2PS256RM`.
    fn vcvtneoph2ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEOPH2PS256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH128`.
    fn vcvtpd2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH128");
        }
    }
    /// Emits `VCVTPD2PH128_MASK`.
    fn vcvtpd2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH128_MASK");
        }
    }
    /// Emits `VCVTPD2PH128_MASKZ`.
    fn vcvtpd2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH128_MASKZ");
        }
    }
    /// Emits `VCVTPD2PH128RB`.
    fn vcvtpd2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH128RB_MASK`.
    fn vcvtpd2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH128RB_MASKZ`.
    fn vcvtpd2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH256`.
    fn vcvtpd2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH256");
        }
    }
    /// Emits `VCVTPD2PH256_MASK`.
    fn vcvtpd2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH256_MASK");
        }
    }
    /// Emits `VCVTPD2PH256_MASKZ`.
    fn vcvtpd2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH256_MASKZ");
        }
    }
    /// Emits `VCVTPD2PH256RB`.
    fn vcvtpd2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH256RB_MASK`.
    fn vcvtpd2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH256RB_MASKZ`.
    fn vcvtpd2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512`.
    fn vcvtpd2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH512");
        }
    }
    /// Emits `VCVTPD2PH512RR_ER`.
    fn vcvtpd2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512_MASK`.
    fn vcvtpd2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH512_MASK");
        }
    }
    /// Emits `VCVTPD2PH512RR_MASK_ER`.
    fn vcvtpd2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512_MASKZ`.
    fn vcvtpd2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPD2PH512_MASKZ");
        }
    }
    /// Emits `VCVTPD2PH512RR_MASKZ_ER`.
    fn vcvtpd2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512RB`.
    fn vcvtpd2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512RB_MASK`.
    fn vcvtpd2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPD2PH512RB_MASKZ`.
    fn vcvtpd2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ128`.
    fn vcvtph2dq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ128");
        }
    }
    /// Emits `VCVTPH2DQ128_MASK`.
    fn vcvtph2dq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ128_MASK");
        }
    }
    /// Emits `VCVTPH2DQ128_MASKZ`.
    fn vcvtph2dq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ128_MASKZ");
        }
    }
    /// Emits `VCVTPH2DQ128RB`.
    fn vcvtph2dq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ128RB_MASK`.
    fn vcvtph2dq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ128RB_MASKZ`.
    fn vcvtph2dq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ256`.
    fn vcvtph2dq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ256");
        }
    }
    /// Emits `VCVTPH2DQ256_MASK`.
    fn vcvtph2dq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ256_MASK");
        }
    }
    /// Emits `VCVTPH2DQ256_MASKZ`.
    fn vcvtph2dq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ256_MASKZ");
        }
    }
    /// Emits `VCVTPH2DQ256RB`.
    fn vcvtph2dq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ256RB_MASK`.
    fn vcvtph2dq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ256RB_MASKZ`.
    fn vcvtph2dq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512`.
    fn vcvtph2dq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ512");
        }
    }
    /// Emits `VCVTPH2DQ512RR_ER`.
    fn vcvtph2dq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512_MASK`.
    fn vcvtph2dq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ512_MASK");
        }
    }
    /// Emits `VCVTPH2DQ512RR_MASK_ER`.
    fn vcvtph2dq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512_MASKZ`.
    fn vcvtph2dq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2DQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2DQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2DQ512_MASKZ");
        }
    }
    /// Emits `VCVTPH2DQ512RR_MASKZ_ER`.
    fn vcvtph2dq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512RB`.
    fn vcvtph2dq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512RB_MASK`.
    fn vcvtph2dq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2DQ512RB_MASKZ`.
    fn vcvtph2dq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2DQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD128`.
    fn vcvtph2pd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD128");
        }
    }
    /// Emits `VCVTPH2PD128_MASK`.
    fn vcvtph2pd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD128_MASK");
        }
    }
    /// Emits `VCVTPH2PD128_MASKZ`.
    fn vcvtph2pd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD128_MASKZ");
        }
    }
    /// Emits `VCVTPH2PD128RB`.
    fn vcvtph2pd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD128RB_MASK`.
    fn vcvtph2pd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD128RB_MASKZ`.
    fn vcvtph2pd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD256`.
    fn vcvtph2pd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD256");
        }
    }
    /// Emits `VCVTPH2PD256_MASK`.
    fn vcvtph2pd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD256_MASK");
        }
    }
    /// Emits `VCVTPH2PD256_MASKZ`.
    fn vcvtph2pd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD256_MASKZ");
        }
    }
    /// Emits `VCVTPH2PD256RB`.
    fn vcvtph2pd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD256RB_MASK`.
    fn vcvtph2pd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD256RB_MASKZ`.
    fn vcvtph2pd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512`.
    fn vcvtph2pd512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD512");
        }
    }
    /// Emits `VCVTPH2PD512_MASK`.
    fn vcvtph2pd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD512_MASK");
        }
    }
    /// Emits `VCVTPH2PD512RR_MASK_SAE`.
    fn vcvtph2pd512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512_MASKZ`.
    fn vcvtph2pd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PD512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PD512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PD512_MASKZ");
        }
    }
    /// Emits `VCVTPH2PD512RR_MASKZ_SAE`.
    fn vcvtph2pd512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512RR_SAE`.
    fn vcvtph2pd512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512RB`.
    fn vcvtph2pd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512RB_MASK`.
    fn vcvtph2pd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PD512RB_MASKZ`.
    fn vcvtph2pd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX128`.
    fn vcvtph2psx128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX128");
        }
    }
    /// Emits `VCVTPH2PSX128_MASK`.
    fn vcvtph2psx128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX128_MASK");
        }
    }
    /// Emits `VCVTPH2PSX128_MASKZ`.
    fn vcvtph2psx128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX128_MASKZ");
        }
    }
    /// Emits `VCVTPH2PSX128RB`.
    fn vcvtph2psx128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX128RB_MASK`.
    fn vcvtph2psx128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX128RB_MASKZ`.
    fn vcvtph2psx128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX256`.
    fn vcvtph2psx256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX256");
        }
    }
    /// Emits `VCVTPH2PSX256_MASK`.
    fn vcvtph2psx256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX256_MASK");
        }
    }
    /// Emits `VCVTPH2PSX256_MASKZ`.
    fn vcvtph2psx256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX256_MASKZ");
        }
    }
    /// Emits `VCVTPH2PSX256RB`.
    fn vcvtph2psx256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX256RB_MASK`.
    fn vcvtph2psx256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX256RB_MASKZ`.
    fn vcvtph2psx256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512`.
    fn vcvtph2psx512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX512");
        }
    }
    /// Emits `VCVTPH2PSX512_MASK`.
    fn vcvtph2psx512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX512_MASK");
        }
    }
    /// Emits `VCVTPH2PSX512RR_MASK_SAE`.
    fn vcvtph2psx512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512_MASKZ`.
    fn vcvtph2psx512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2PSX512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2PSX512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2PSX512_MASKZ");
        }
    }
    /// Emits `VCVTPH2PSX512RR_MASKZ_SAE`.
    fn vcvtph2psx512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512RR_SAE`.
    fn vcvtph2psx512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512RB`.
    fn vcvtph2psx512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512RB_MASK`.
    fn vcvtph2psx512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2PSX512RB_MASKZ`.
    fn vcvtph2psx512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2PSX512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ128`.
    fn vcvtph2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ128");
        }
    }
    /// Emits `VCVTPH2QQ128_MASK`.
    fn vcvtph2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ128_MASK");
        }
    }
    /// Emits `VCVTPH2QQ128_MASKZ`.
    fn vcvtph2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ128_MASKZ");
        }
    }
    /// Emits `VCVTPH2QQ128RB`.
    fn vcvtph2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ128RB_MASK`.
    fn vcvtph2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ128RB_MASKZ`.
    fn vcvtph2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ256`.
    fn vcvtph2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ256");
        }
    }
    /// Emits `VCVTPH2QQ256_MASK`.
    fn vcvtph2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ256_MASK");
        }
    }
    /// Emits `VCVTPH2QQ256_MASKZ`.
    fn vcvtph2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ256_MASKZ");
        }
    }
    /// Emits `VCVTPH2QQ256RB`.
    fn vcvtph2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ256RB_MASK`.
    fn vcvtph2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ256RB_MASKZ`.
    fn vcvtph2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512`.
    fn vcvtph2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ512");
        }
    }
    /// Emits `VCVTPH2QQ512RR_ER`.
    fn vcvtph2qq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512_MASK`.
    fn vcvtph2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ512_MASK");
        }
    }
    /// Emits `VCVTPH2QQ512RR_MASK_ER`.
    fn vcvtph2qq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512_MASKZ`.
    fn vcvtph2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2QQ512_MASKZ");
        }
    }
    /// Emits `VCVTPH2QQ512RR_MASKZ_ER`.
    fn vcvtph2qq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512RB`.
    fn vcvtph2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512RB_MASK`.
    fn vcvtph2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2QQ512RB_MASKZ`.
    fn vcvtph2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ128`.
    fn vcvtph2udq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ128");
        }
    }
    /// Emits `VCVTPH2UDQ128_MASK`.
    fn vcvtph2udq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ128_MASK");
        }
    }
    /// Emits `VCVTPH2UDQ128_MASKZ`.
    fn vcvtph2udq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ128_MASKZ");
        }
    }
    /// Emits `VCVTPH2UDQ128RB`.
    fn vcvtph2udq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ128RB_MASK`.
    fn vcvtph2udq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ128RB_MASKZ`.
    fn vcvtph2udq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ256`.
    fn vcvtph2udq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ256");
        }
    }
    /// Emits `VCVTPH2UDQ256_MASK`.
    fn vcvtph2udq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ256_MASK");
        }
    }
    /// Emits `VCVTPH2UDQ256_MASKZ`.
    fn vcvtph2udq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ256_MASKZ");
        }
    }
    /// Emits `VCVTPH2UDQ256RB`.
    fn vcvtph2udq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ256RB_MASK`.
    fn vcvtph2udq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ256RB_MASKZ`.
    fn vcvtph2udq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512`.
    fn vcvtph2udq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ512");
        }
    }
    /// Emits `VCVTPH2UDQ512RR_ER`.
    fn vcvtph2udq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512_MASK`.
    fn vcvtph2udq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ512_MASK");
        }
    }
    /// Emits `VCVTPH2UDQ512RR_MASK_ER`.
    fn vcvtph2udq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512_MASKZ`.
    fn vcvtph2udq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UDQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UDQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UDQ512_MASKZ");
        }
    }
    /// Emits `VCVTPH2UDQ512RR_MASKZ_ER`.
    fn vcvtph2udq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512RB`.
    fn vcvtph2udq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512RB_MASK`.
    fn vcvtph2udq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UDQ512RB_MASKZ`.
    fn vcvtph2udq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UDQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ128`.
    fn vcvtph2uqq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ128");
        }
    }
    /// Emits `VCVTPH2UQQ128_MASK`.
    fn vcvtph2uqq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ128_MASK");
        }
    }
    /// Emits `VCVTPH2UQQ128_MASKZ`.
    fn vcvtph2uqq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ128_MASKZ");
        }
    }
    /// Emits `VCVTPH2UQQ128RB`.
    fn vcvtph2uqq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ128RB_MASK`.
    fn vcvtph2uqq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ128RB_MASKZ`.
    fn vcvtph2uqq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ256`.
    fn vcvtph2uqq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ256");
        }
    }
    /// Emits `VCVTPH2UQQ256_MASK`.
    fn vcvtph2uqq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ256_MASK");
        }
    }
    /// Emits `VCVTPH2UQQ256_MASKZ`.
    fn vcvtph2uqq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ256_MASKZ");
        }
    }
    /// Emits `VCVTPH2UQQ256RB`.
    fn vcvtph2uqq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ256RB_MASK`.
    fn vcvtph2uqq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ256RB_MASKZ`.
    fn vcvtph2uqq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512`.
    fn vcvtph2uqq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ512");
        }
    }
    /// Emits `VCVTPH2UQQ512RR_ER`.
    fn vcvtph2uqq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512_MASK`.
    fn vcvtph2uqq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ512_MASK");
        }
    }
    /// Emits `VCVTPH2UQQ512RR_MASK_ER`.
    fn vcvtph2uqq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512_MASKZ`.
    fn vcvtph2uqq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UQQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UQQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UQQ512_MASKZ");
        }
    }
    /// Emits `VCVTPH2UQQ512RR_MASKZ_ER`.
    fn vcvtph2uqq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512RB`.
    fn vcvtph2uqq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512RB_MASK`.
    fn vcvtph2uqq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UQQ512RB_MASKZ`.
    fn vcvtph2uqq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UQQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW128`.
    fn vcvtph2uw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW128");
        }
    }
    /// Emits `VCVTPH2UW128_MASK`.
    fn vcvtph2uw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW128_MASK");
        }
    }
    /// Emits `VCVTPH2UW128_MASKZ`.
    fn vcvtph2uw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW128_MASKZ");
        }
    }
    /// Emits `VCVTPH2UW128RB`.
    fn vcvtph2uw128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW128RB_MASK`.
    fn vcvtph2uw128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW128RB_MASKZ`.
    fn vcvtph2uw128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW256`.
    fn vcvtph2uw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW256");
        }
    }
    /// Emits `VCVTPH2UW256_MASK`.
    fn vcvtph2uw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW256_MASK");
        }
    }
    /// Emits `VCVTPH2UW256_MASKZ`.
    fn vcvtph2uw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW256_MASKZ");
        }
    }
    /// Emits `VCVTPH2UW256RB`.
    fn vcvtph2uw256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW256RB_MASK`.
    fn vcvtph2uw256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW256RB_MASKZ`.
    fn vcvtph2uw256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512`.
    fn vcvtph2uw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW512");
        }
    }
    /// Emits `VCVTPH2UW512RR_ER`.
    fn vcvtph2uw512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512_MASK`.
    fn vcvtph2uw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW512_MASK");
        }
    }
    /// Emits `VCVTPH2UW512RR_MASK_ER`.
    fn vcvtph2uw512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512_MASKZ`.
    fn vcvtph2uw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2UW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2UW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2UW512_MASKZ");
        }
    }
    /// Emits `VCVTPH2UW512RR_MASKZ_ER`.
    fn vcvtph2uw512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512RB`.
    fn vcvtph2uw512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512RB_MASK`.
    fn vcvtph2uw512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2UW512RB_MASKZ`.
    fn vcvtph2uw512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2UW512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W128`.
    fn vcvtph2w128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W128");
        }
    }
    /// Emits `VCVTPH2W128_MASK`.
    fn vcvtph2w128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W128_MASK");
        }
    }
    /// Emits `VCVTPH2W128_MASKZ`.
    fn vcvtph2w128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W128_MASKZ");
        }
    }
    /// Emits `VCVTPH2W128RB`.
    fn vcvtph2w128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W128RB_MASK`.
    fn vcvtph2w128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W128RB_MASKZ`.
    fn vcvtph2w128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W256`.
    fn vcvtph2w256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W256");
        }
    }
    /// Emits `VCVTPH2W256_MASK`.
    fn vcvtph2w256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W256_MASK");
        }
    }
    /// Emits `VCVTPH2W256_MASKZ`.
    fn vcvtph2w256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W256_MASKZ");
        }
    }
    /// Emits `VCVTPH2W256RB`.
    fn vcvtph2w256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W256RB_MASK`.
    fn vcvtph2w256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W256RB_MASKZ`.
    fn vcvtph2w256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512`.
    fn vcvtph2w512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W512");
        }
    }
    /// Emits `VCVTPH2W512RR_ER`.
    fn vcvtph2w512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512_MASK`.
    fn vcvtph2w512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W512_MASK");
        }
    }
    /// Emits `VCVTPH2W512RR_MASK_ER`.
    fn vcvtph2w512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512_MASKZ`.
    fn vcvtph2w512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPH2W512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPH2W512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPH2W512_MASKZ");
        }
    }
    /// Emits `VCVTPH2W512RR_MASKZ_ER`.
    fn vcvtph2w512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512RB`.
    fn vcvtph2w512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512RB_MASK`.
    fn vcvtph2w512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPH2W512RB_MASKZ`.
    fn vcvtph2w512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPH2W512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX128`.
    fn vcvtps2phx128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX128");
        }
    }
    /// Emits `VCVTPS2PHX128_MASK`.
    fn vcvtps2phx128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX128_MASK");
        }
    }
    /// Emits `VCVTPS2PHX128_MASKZ`.
    fn vcvtps2phx128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX128_MASKZ");
        }
    }
    /// Emits `VCVTPS2PHX128RB`.
    fn vcvtps2phx128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX128RB_MASK`.
    fn vcvtps2phx128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX128RB_MASKZ`.
    fn vcvtps2phx128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX256`.
    fn vcvtps2phx256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX256");
        }
    }
    /// Emits `VCVTPS2PHX256_MASK`.
    fn vcvtps2phx256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX256_MASK");
        }
    }
    /// Emits `VCVTPS2PHX256_MASKZ`.
    fn vcvtps2phx256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX256_MASKZ");
        }
    }
    /// Emits `VCVTPS2PHX256RB`.
    fn vcvtps2phx256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX256RB_MASK`.
    fn vcvtps2phx256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX256RB_MASKZ`.
    fn vcvtps2phx256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512`.
    fn vcvtps2phx512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX512");
        }
    }
    /// Emits `VCVTPS2PHX512RR_ER`.
    fn vcvtps2phx512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512_MASK`.
    fn vcvtps2phx512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX512_MASK");
        }
    }
    /// Emits `VCVTPS2PHX512RR_MASK_ER`.
    fn vcvtps2phx512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512_MASKZ`.
    fn vcvtps2phx512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2PHX512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2PHX512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTPS2PHX512_MASKZ");
        }
    }
    /// Emits `VCVTPS2PHX512RR_MASKZ_ER`.
    fn vcvtps2phx512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512RB`.
    fn vcvtps2phx512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512RB_MASK`.
    fn vcvtps2phx512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTPS2PHX512RB_MASKZ`.
    fn vcvtps2phx512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2PHX512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH128`.
    fn vcvtqq2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH128");
        }
    }
    /// Emits `VCVTQQ2PH128_MASK`.
    fn vcvtqq2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH128_MASK");
        }
    }
    /// Emits `VCVTQQ2PH128_MASKZ`.
    fn vcvtqq2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH128_MASKZ");
        }
    }
    /// Emits `VCVTQQ2PH128RB`.
    fn vcvtqq2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH128RB_MASK`.
    fn vcvtqq2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH128RB_MASKZ`.
    fn vcvtqq2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH256`.
    fn vcvtqq2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH256");
        }
    }
    /// Emits `VCVTQQ2PH256_MASK`.
    fn vcvtqq2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH256_MASK");
        }
    }
    /// Emits `VCVTQQ2PH256_MASKZ`.
    fn vcvtqq2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH256_MASKZ");
        }
    }
    /// Emits `VCVTQQ2PH256RB`.
    fn vcvtqq2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH256RB_MASK`.
    fn vcvtqq2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH256RB_MASKZ`.
    fn vcvtqq2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512`.
    fn vcvtqq2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH512");
        }
    }
    /// Emits `VCVTQQ2PH512RR_ER`.
    fn vcvtqq2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512_MASK`.
    fn vcvtqq2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH512_MASK");
        }
    }
    /// Emits `VCVTQQ2PH512RR_MASK_ER`.
    fn vcvtqq2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512_MASKZ`.
    fn vcvtqq2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTQQ2PH512_MASKZ");
        }
    }
    /// Emits `VCVTQQ2PH512RR_MASKZ_ER`.
    fn vcvtqq2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512RB`.
    fn vcvtqq2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512RB_MASK`.
    fn vcvtqq2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTQQ2PH512RB_MASKZ`.
    fn vcvtqq2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTSD2SH`.
    fn vcvtsd2sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSD2SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSD2SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSD2SH");
        }
    }
    /// Emits `VCVTSD2SHRRR_ER`.
    fn vcvtsd2sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSD2SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSD2SH_MASK`.
    fn vcvtsd2sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSD2SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSD2SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSD2SH_MASK");
        }
    }
    /// Emits `VCVTSD2SHRRR_MASK_ER`.
    fn vcvtsd2sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSD2SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSD2SH_MASKZ`.
    fn vcvtsd2sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSD2SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSD2SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSD2SH_MASKZ");
        }
    }
    /// Emits `VCVTSD2SHRRR_MASKZ_ER`.
    fn vcvtsd2sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSD2SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SD`.
    fn vcvtsh2sd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SDRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SDRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SD");
        }
    }
    /// Emits `VCVTSH2SD_MASK`.
    fn vcvtsh2sd_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SDRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SDRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SD_MASK");
        }
    }
    /// Emits `VCVTSH2SDRRR_MASK_SAE`.
    fn vcvtsh2sd_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SDRRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SD_MASKZ`.
    fn vcvtsh2sd_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SDRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SDRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SD_MASKZ");
        }
    }
    /// Emits `VCVTSH2SDRRR_MASKZ_SAE`.
    fn vcvtsh2sd_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SDRRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SDRRR_SAE`.
    fn vcvtsh2sd_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SDRRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SI32`.
    fn vcvtsh2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTSH2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTSH2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SI32");
        }
    }
    /// Emits `VCVTSH2SI32RR_ER`.
    fn vcvtsh2si32_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTSH2SI32RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTSH2SI64`.
    fn vcvtsh2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTSH2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTSH2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SI64");
        }
    }
    /// Emits `VCVTSH2SI64RR_ER`.
    fn vcvtsh2si64_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTSH2SI64RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTSH2SS`.
    fn vcvtsh2ss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SSRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SSRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SS");
        }
    }
    /// Emits `VCVTSH2SS_MASK`.
    fn vcvtsh2ss_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SSRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SSRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SS_MASK");
        }
    }
    /// Emits `VCVTSH2SSRRR_MASK_SAE`.
    fn vcvtsh2ss_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SSRRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SS_MASKZ`.
    fn vcvtsh2ss_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSH2SSRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSH2SSRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2SS_MASKZ");
        }
    }
    /// Emits `VCVTSH2SSRRR_MASKZ_SAE`.
    fn vcvtsh2ss_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SSRRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2SSRRR_SAE`.
    fn vcvtsh2ss_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSH2SSRRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSH2USI32`.
    fn vcvtsh2usi32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTSH2USI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTSH2USI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2USI32");
        }
    }
    /// Emits `VCVTSH2USI32RR_ER`.
    fn vcvtsh2usi32_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTSH2USI32RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTSH2USI64`.
    fn vcvtsh2usi64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTSH2USI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTSH2USI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSH2USI64");
        }
    }
    /// Emits `VCVTSH2USI64RR_ER`.
    fn vcvtsh2usi64_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTSH2USI64RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTSI2SH32`.
    fn vcvtsi2sh32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() {
            self.emit(VCVTSI2SH32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSI2SH32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSI2SH32");
        }
    }
    /// Emits `VCVTSI2SH32RRR_ER`.
    fn vcvtsi2sh32_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSI2SH32RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSI2SH64`.
    fn vcvtsi2sh64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() {
            self.emit(VCVTSI2SH64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSI2SH64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSI2SH64");
        }
    }
    /// Emits `VCVTSI2SH64RRR_ER`.
    fn vcvtsi2sh64_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSI2SH64RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSS2SH`.
    fn vcvtss2sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSS2SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSS2SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSS2SH");
        }
    }
    /// Emits `VCVTSS2SHRRR_ER`.
    fn vcvtss2sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSS2SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSS2SH_MASK`.
    fn vcvtss2sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSS2SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSS2SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSS2SH_MASK");
        }
    }
    /// Emits `VCVTSS2SHRRR_MASK_ER`.
    fn vcvtss2sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSS2SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTSS2SH_MASKZ`.
    fn vcvtss2sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTSS2SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTSS2SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTSS2SH_MASKZ");
        }
    }
    /// Emits `VCVTSS2SHRRR_MASKZ_ER`.
    fn vcvtss2sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTSS2SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTTPH2DQ128`.
    fn vcvttph2dq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ128");
        }
    }
    /// Emits `VCVTTPH2DQ128_MASK`.
    fn vcvttph2dq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ128_MASK");
        }
    }
    /// Emits `VCVTTPH2DQ128_MASKZ`.
    fn vcvttph2dq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2DQ128RB`.
    fn vcvttph2dq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ128RB_MASK`.
    fn vcvttph2dq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ128RB_MASKZ`.
    fn vcvttph2dq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ256`.
    fn vcvttph2dq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ256");
        }
    }
    /// Emits `VCVTTPH2DQ256_MASK`.
    fn vcvttph2dq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ256_MASK");
        }
    }
    /// Emits `VCVTTPH2DQ256_MASKZ`.
    fn vcvttph2dq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2DQ256RB`.
    fn vcvttph2dq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ256RB_MASK`.
    fn vcvttph2dq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ256RB_MASKZ`.
    fn vcvttph2dq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512`.
    fn vcvttph2dq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ512");
        }
    }
    /// Emits `VCVTTPH2DQ512_MASK`.
    fn vcvttph2dq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ512_MASK");
        }
    }
    /// Emits `VCVTTPH2DQ512RR_MASK_SAE`.
    fn vcvttph2dq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512_MASKZ`.
    fn vcvttph2dq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2DQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2DQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2DQ512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2DQ512RR_MASKZ_SAE`.
    fn vcvttph2dq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512RR_SAE`.
    fn vcvttph2dq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512RB`.
    fn vcvttph2dq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512RB_MASK`.
    fn vcvttph2dq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2DQ512RB_MASKZ`.
    fn vcvttph2dq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2DQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ128`.
    fn vcvttph2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ128");
        }
    }
    /// Emits `VCVTTPH2QQ128_MASK`.
    fn vcvttph2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ128_MASK");
        }
    }
    /// Emits `VCVTTPH2QQ128_MASKZ`.
    fn vcvttph2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2QQ128RB`.
    fn vcvttph2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ128RB_MASK`.
    fn vcvttph2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ128RB_MASKZ`.
    fn vcvttph2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ256`.
    fn vcvttph2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ256");
        }
    }
    /// Emits `VCVTTPH2QQ256_MASK`.
    fn vcvttph2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ256_MASK");
        }
    }
    /// Emits `VCVTTPH2QQ256_MASKZ`.
    fn vcvttph2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2QQ256RB`.
    fn vcvttph2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ256RB_MASK`.
    fn vcvttph2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ256RB_MASKZ`.
    fn vcvttph2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512`.
    fn vcvttph2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ512");
        }
    }
    /// Emits `VCVTTPH2QQ512_MASK`.
    fn vcvttph2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ512_MASK");
        }
    }
    /// Emits `VCVTTPH2QQ512RR_MASK_SAE`.
    fn vcvttph2qq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512_MASKZ`.
    fn vcvttph2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2QQ512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2QQ512RR_MASKZ_SAE`.
    fn vcvttph2qq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512RR_SAE`.
    fn vcvttph2qq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512RB`.
    fn vcvttph2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512RB_MASK`.
    fn vcvttph2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2QQ512RB_MASKZ`.
    fn vcvttph2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ128`.
    fn vcvttph2udq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ128");
        }
    }
    /// Emits `VCVTTPH2UDQ128_MASK`.
    fn vcvttph2udq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ128_MASK");
        }
    }
    /// Emits `VCVTTPH2UDQ128_MASKZ`.
    fn vcvttph2udq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UDQ128RB`.
    fn vcvttph2udq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ128RB_MASK`.
    fn vcvttph2udq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ128RB_MASKZ`.
    fn vcvttph2udq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ256`.
    fn vcvttph2udq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ256");
        }
    }
    /// Emits `VCVTTPH2UDQ256_MASK`.
    fn vcvttph2udq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ256_MASK");
        }
    }
    /// Emits `VCVTTPH2UDQ256_MASKZ`.
    fn vcvttph2udq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UDQ256RB`.
    fn vcvttph2udq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ256RB_MASK`.
    fn vcvttph2udq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ256RB_MASKZ`.
    fn vcvttph2udq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512`.
    fn vcvttph2udq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ512");
        }
    }
    /// Emits `VCVTTPH2UDQ512_MASK`.
    fn vcvttph2udq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ512_MASK");
        }
    }
    /// Emits `VCVTTPH2UDQ512RR_MASK_SAE`.
    fn vcvttph2udq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512_MASKZ`.
    fn vcvttph2udq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UDQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UDQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UDQ512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UDQ512RR_MASKZ_SAE`.
    fn vcvttph2udq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512RR_SAE`.
    fn vcvttph2udq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512RB`.
    fn vcvttph2udq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512RB_MASK`.
    fn vcvttph2udq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UDQ512RB_MASKZ`.
    fn vcvttph2udq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UDQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ128`.
    fn vcvttph2uqq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ128");
        }
    }
    /// Emits `VCVTTPH2UQQ128_MASK`.
    fn vcvttph2uqq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ128_MASK");
        }
    }
    /// Emits `VCVTTPH2UQQ128_MASKZ`.
    fn vcvttph2uqq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UQQ128RB`.
    fn vcvttph2uqq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ128RB_MASK`.
    fn vcvttph2uqq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ128RB_MASKZ`.
    fn vcvttph2uqq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ256`.
    fn vcvttph2uqq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ256");
        }
    }
    /// Emits `VCVTTPH2UQQ256_MASK`.
    fn vcvttph2uqq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ256_MASK");
        }
    }
    /// Emits `VCVTTPH2UQQ256_MASKZ`.
    fn vcvttph2uqq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UQQ256RB`.
    fn vcvttph2uqq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ256RB_MASK`.
    fn vcvttph2uqq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ256RB_MASKZ`.
    fn vcvttph2uqq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512`.
    fn vcvttph2uqq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ512");
        }
    }
    /// Emits `VCVTTPH2UQQ512_MASK`.
    fn vcvttph2uqq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ512_MASK");
        }
    }
    /// Emits `VCVTTPH2UQQ512RR_MASK_SAE`.
    fn vcvttph2uqq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512_MASKZ`.
    fn vcvttph2uqq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UQQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UQQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UQQ512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UQQ512RR_MASKZ_SAE`.
    fn vcvttph2uqq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512RR_SAE`.
    fn vcvttph2uqq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512RB`.
    fn vcvttph2uqq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512RB_MASK`.
    fn vcvttph2uqq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UQQ512RB_MASKZ`.
    fn vcvttph2uqq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UQQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW128`.
    fn vcvttph2uw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW128");
        }
    }
    /// Emits `VCVTTPH2UW128_MASK`.
    fn vcvttph2uw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW128_MASK");
        }
    }
    /// Emits `VCVTTPH2UW128_MASKZ`.
    fn vcvttph2uw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UW128RB`.
    fn vcvttph2uw128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW128RB_MASK`.
    fn vcvttph2uw128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW128RB_MASKZ`.
    fn vcvttph2uw128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW256`.
    fn vcvttph2uw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW256");
        }
    }
    /// Emits `VCVTTPH2UW256_MASK`.
    fn vcvttph2uw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW256_MASK");
        }
    }
    /// Emits `VCVTTPH2UW256_MASKZ`.
    fn vcvttph2uw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UW256RB`.
    fn vcvttph2uw256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW256RB_MASK`.
    fn vcvttph2uw256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW256RB_MASKZ`.
    fn vcvttph2uw256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512`.
    fn vcvttph2uw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW512");
        }
    }
    /// Emits `VCVTTPH2UW512_MASK`.
    fn vcvttph2uw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW512_MASK");
        }
    }
    /// Emits `VCVTTPH2UW512RR_MASK_SAE`.
    fn vcvttph2uw512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512_MASKZ`.
    fn vcvttph2uw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2UW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2UW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2UW512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2UW512RR_MASKZ_SAE`.
    fn vcvttph2uw512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512RR_SAE`.
    fn vcvttph2uw512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512RB`.
    fn vcvttph2uw512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512RB_MASK`.
    fn vcvttph2uw512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2UW512RB_MASKZ`.
    fn vcvttph2uw512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2UW512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W128`.
    fn vcvttph2w128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W128");
        }
    }
    /// Emits `VCVTTPH2W128_MASK`.
    fn vcvttph2w128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W128_MASK");
        }
    }
    /// Emits `VCVTTPH2W128_MASKZ`.
    fn vcvttph2w128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W128_MASKZ");
        }
    }
    /// Emits `VCVTTPH2W128RB`.
    fn vcvttph2w128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W128RB_MASK`.
    fn vcvttph2w128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W128RB_MASKZ`.
    fn vcvttph2w128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W256`.
    fn vcvttph2w256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W256");
        }
    }
    /// Emits `VCVTTPH2W256_MASK`.
    fn vcvttph2w256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W256_MASK");
        }
    }
    /// Emits `VCVTTPH2W256_MASKZ`.
    fn vcvttph2w256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W256_MASKZ");
        }
    }
    /// Emits `VCVTTPH2W256RB`.
    fn vcvttph2w256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W256RB_MASK`.
    fn vcvttph2w256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W256RB_MASKZ`.
    fn vcvttph2w256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512`.
    fn vcvttph2w512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W512");
        }
    }
    /// Emits `VCVTTPH2W512_MASK`.
    fn vcvttph2w512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W512_MASK");
        }
    }
    /// Emits `VCVTTPH2W512RR_MASK_SAE`.
    fn vcvttph2w512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512_MASKZ`.
    fn vcvttph2w512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPH2W512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPH2W512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTPH2W512_MASKZ");
        }
    }
    /// Emits `VCVTTPH2W512RR_MASKZ_SAE`.
    fn vcvttph2w512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512RR_SAE`.
    fn vcvttph2w512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512RB`.
    fn vcvttph2w512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512RB_MASK`.
    fn vcvttph2w512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTPH2W512RB_MASKZ`.
    fn vcvttph2w512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPH2W512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTSH2SI32`.
    fn vcvttsh2si32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTTSH2SI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTTSH2SI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTSH2SI32");
        }
    }
    /// Emits `VCVTTSH2SI32RR_SAE`.
    fn vcvttsh2si32_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTSH2SI32RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTSH2SI64`.
    fn vcvttsh2si64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTTSH2SI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTTSH2SI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTSH2SI64");
        }
    }
    /// Emits `VCVTTSH2SI64RR_SAE`.
    fn vcvttsh2si64_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTSH2SI64RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTSH2USI32`.
    fn vcvttsh2usi32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTTSH2USI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTTSH2USI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTSH2USI32");
        }
    }
    /// Emits `VCVTTSH2USI32RR_SAE`.
    fn vcvttsh2usi32_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTSH2USI32RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTTSH2USI64`.
    fn vcvttsh2usi64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VCVTTSH2USI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VCVTTSH2USI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTTSH2USI64");
        }
    }
    /// Emits `VCVTTSH2USI64RR_SAE`.
    fn vcvttsh2usi64_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTSH2USI64RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH128`.
    fn vcvtudq2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH128");
        }
    }
    /// Emits `VCVTUDQ2PH128_MASK`.
    fn vcvtudq2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH128_MASK");
        }
    }
    /// Emits `VCVTUDQ2PH128_MASKZ`.
    fn vcvtudq2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH128_MASKZ");
        }
    }
    /// Emits `VCVTUDQ2PH128RB`.
    fn vcvtudq2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH128RB_MASK`.
    fn vcvtudq2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH128RB_MASKZ`.
    fn vcvtudq2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH256`.
    fn vcvtudq2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH256");
        }
    }
    /// Emits `VCVTUDQ2PH256_MASK`.
    fn vcvtudq2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH256_MASK");
        }
    }
    /// Emits `VCVTUDQ2PH256_MASKZ`.
    fn vcvtudq2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH256_MASKZ");
        }
    }
    /// Emits `VCVTUDQ2PH256RB`.
    fn vcvtudq2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH256RB_MASK`.
    fn vcvtudq2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH256RB_MASKZ`.
    fn vcvtudq2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512`.
    fn vcvtudq2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH512");
        }
    }
    /// Emits `VCVTUDQ2PH512RR_ER`.
    fn vcvtudq2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512_MASK`.
    fn vcvtudq2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH512_MASK");
        }
    }
    /// Emits `VCVTUDQ2PH512RR_MASK_ER`.
    fn vcvtudq2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512_MASKZ`.
    fn vcvtudq2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUDQ2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUDQ2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUDQ2PH512_MASKZ");
        }
    }
    /// Emits `VCVTUDQ2PH512RR_MASKZ_ER`.
    fn vcvtudq2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512RB`.
    fn vcvtudq2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512RB_MASK`.
    fn vcvtudq2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUDQ2PH512RB_MASKZ`.
    fn vcvtudq2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUDQ2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH128`.
    fn vcvtuqq2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH128");
        }
    }
    /// Emits `VCVTUQQ2PH128_MASK`.
    fn vcvtuqq2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH128_MASK");
        }
    }
    /// Emits `VCVTUQQ2PH128_MASKZ`.
    fn vcvtuqq2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH128_MASKZ");
        }
    }
    /// Emits `VCVTUQQ2PH128RB`.
    fn vcvtuqq2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH128RB_MASK`.
    fn vcvtuqq2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH128RB_MASKZ`.
    fn vcvtuqq2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH256`.
    fn vcvtuqq2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH256");
        }
    }
    /// Emits `VCVTUQQ2PH256_MASK`.
    fn vcvtuqq2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH256_MASK");
        }
    }
    /// Emits `VCVTUQQ2PH256_MASKZ`.
    fn vcvtuqq2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH256_MASKZ");
        }
    }
    /// Emits `VCVTUQQ2PH256RB`.
    fn vcvtuqq2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH256RB_MASK`.
    fn vcvtuqq2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH256RB_MASKZ`.
    fn vcvtuqq2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512`.
    fn vcvtuqq2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH512");
        }
    }
    /// Emits `VCVTUQQ2PH512RR_ER`.
    fn vcvtuqq2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512_MASK`.
    fn vcvtuqq2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH512_MASK");
        }
    }
    /// Emits `VCVTUQQ2PH512RR_MASK_ER`.
    fn vcvtuqq2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512_MASKZ`.
    fn vcvtuqq2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUQQ2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUQQ2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUQQ2PH512_MASKZ");
        }
    }
    /// Emits `VCVTUQQ2PH512RR_MASKZ_ER`.
    fn vcvtuqq2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512RB`.
    fn vcvtuqq2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512RB_MASK`.
    fn vcvtuqq2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUQQ2PH512RB_MASKZ`.
    fn vcvtuqq2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUQQ2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUSI2SH32`.
    fn vcvtusi2sh32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() {
            self.emit(VCVTUSI2SH32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTUSI2SH32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUSI2SH32");
        }
    }
    /// Emits `VCVTUSI2SH32RRR_ER`.
    fn vcvtusi2sh32_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTUSI2SH32RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTUSI2SH64`.
    fn vcvtusi2sh64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() {
            self.emit(VCVTUSI2SH64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTUSI2SH64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUSI2SH64");
        }
    }
    /// Emits `VCVTUSI2SH64RRR_ER`.
    fn vcvtusi2sh64_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTUSI2SH64RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTUW2PH128`.
    fn vcvtuw2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH128");
        }
    }
    /// Emits `VCVTUW2PH128_MASK`.
    fn vcvtuw2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH128_MASK");
        }
    }
    /// Emits `VCVTUW2PH128_MASKZ`.
    fn vcvtuw2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH128_MASKZ");
        }
    }
    /// Emits `VCVTUW2PH128RB`.
    fn vcvtuw2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH128RB_MASK`.
    fn vcvtuw2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH128RB_MASKZ`.
    fn vcvtuw2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH256`.
    fn vcvtuw2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH256");
        }
    }
    /// Emits `VCVTUW2PH256_MASK`.
    fn vcvtuw2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH256_MASK");
        }
    }
    /// Emits `VCVTUW2PH256_MASKZ`.
    fn vcvtuw2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH256_MASKZ");
        }
    }
    /// Emits `VCVTUW2PH256RB`.
    fn vcvtuw2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH256RB_MASK`.
    fn vcvtuw2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH256RB_MASKZ`.
    fn vcvtuw2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512`.
    fn vcvtuw2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH512");
        }
    }
    /// Emits `VCVTUW2PH512RR_ER`.
    fn vcvtuw2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512_MASK`.
    fn vcvtuw2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH512_MASK");
        }
    }
    /// Emits `VCVTUW2PH512RR_MASK_ER`.
    fn vcvtuw2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512_MASKZ`.
    fn vcvtuw2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTUW2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTUW2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTUW2PH512_MASKZ");
        }
    }
    /// Emits `VCVTUW2PH512RR_MASKZ_ER`.
    fn vcvtuw2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512RB`.
    fn vcvtuw2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512RB_MASK`.
    fn vcvtuw2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTUW2PH512RB_MASKZ`.
    fn vcvtuw2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTUW2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH128`.
    fn vcvtw2ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH128");
        }
    }
    /// Emits `VCVTW2PH128_MASK`.
    fn vcvtw2ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH128_MASK");
        }
    }
    /// Emits `VCVTW2PH128_MASKZ`.
    fn vcvtw2ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH128_MASKZ");
        }
    }
    /// Emits `VCVTW2PH128RB`.
    fn vcvtw2ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH128RB_MASK`.
    fn vcvtw2ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH128RB_MASKZ`.
    fn vcvtw2ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH256`.
    fn vcvtw2ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH256");
        }
    }
    /// Emits `VCVTW2PH256_MASK`.
    fn vcvtw2ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH256_MASK");
        }
    }
    /// Emits `VCVTW2PH256_MASKZ`.
    fn vcvtw2ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH256_MASKZ");
        }
    }
    /// Emits `VCVTW2PH256RB`.
    fn vcvtw2ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH256RB_MASK`.
    fn vcvtw2ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH256RB_MASKZ`.
    fn vcvtw2ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512`.
    fn vcvtw2ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH512");
        }
    }
    /// Emits `VCVTW2PH512RR_ER`.
    fn vcvtw2ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512_MASK`.
    fn vcvtw2ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH512_MASK");
        }
    }
    /// Emits `VCVTW2PH512RR_MASK_ER`.
    fn vcvtw2ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512_MASKZ`.
    fn vcvtw2ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTW2PH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTW2PH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTW2PH512_MASKZ");
        }
    }
    /// Emits `VCVTW2PH512RR_MASKZ_ER`.
    fn vcvtw2ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512RB`.
    fn vcvtw2ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512RB_MASK`.
    fn vcvtw2ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTW2PH512RB_MASKZ`.
    fn vcvtw2ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTW2PH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VDIVPH128`.
    fn vdivph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH128");
        }
    }
    /// Emits `VDIVPH128_MASK`.
    fn vdivph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH128_MASK");
        }
    }
    /// Emits `VDIVPH128_MASKZ`.
    fn vdivph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH128_MASKZ");
        }
    }
    /// Emits `VDIVPH128RRB`.
    fn vdivph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH128RRB_MASK`.
    fn vdivph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH128RRB_MASKZ`.
    fn vdivph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH256`.
    fn vdivph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH256");
        }
    }
    /// Emits `VDIVPH256_MASK`.
    fn vdivph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH256_MASK");
        }
    }
    /// Emits `VDIVPH256_MASKZ`.
    fn vdivph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH256_MASKZ");
        }
    }
    /// Emits `VDIVPH256RRB`.
    fn vdivph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH256RRB_MASK`.
    fn vdivph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH256RRB_MASKZ`.
    fn vdivph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512`.
    fn vdivph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH512");
        }
    }
    /// Emits `VDIVPH512RRR_ER`.
    fn vdivph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512_MASK`.
    fn vdivph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH512_MASK");
        }
    }
    /// Emits `VDIVPH512RRR_MASK_ER`.
    fn vdivph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512_MASKZ`.
    fn vdivph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVPH512_MASKZ");
        }
    }
    /// Emits `VDIVPH512RRR_MASKZ_ER`.
    fn vdivph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512RRB`.
    fn vdivph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512RRB_MASK`.
    fn vdivph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVPH512RRB_MASKZ`.
    fn vdivph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVSH`.
    fn vdivsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVSH");
        }
    }
    /// Emits `VDIVSHRRR_ER`.
    fn vdivsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVSH_MASK`.
    fn vdivsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVSH_MASK");
        }
    }
    /// Emits `VDIVSHRRR_MASK_ER`.
    fn vdivsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDIVSH_MASKZ`.
    fn vdivsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDIVSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDIVSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDIVSH_MASKZ");
        }
    }
    /// Emits `VDIVSHRRR_MASKZ_ER`.
    fn vdivsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDIVSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VERR`.
    fn verr(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(VERRR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(VERRM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VERR");
        }
    }
    /// Emits `VERW`.
    fn verw(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(VERWR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(VERWM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VERW");
        }
    }
    /// Emits `VFCMADDCPH128`.
    fn vfcmaddcph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH128");
        }
    }
    /// Emits `VFCMADDCPH128_MASK`.
    fn vfcmaddcph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH128_MASK");
        }
    }
    /// Emits `VFCMADDCPH128_MASKZ`.
    fn vfcmaddcph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH128_MASKZ");
        }
    }
    /// Emits `VFCMADDCPH128RRB`.
    fn vfcmaddcph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH128RRB_MASK`.
    fn vfcmaddcph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH128RRB_MASKZ`.
    fn vfcmaddcph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH256`.
    fn vfcmaddcph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH256");
        }
    }
    /// Emits `VFCMADDCPH256_MASK`.
    fn vfcmaddcph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH256_MASK");
        }
    }
    /// Emits `VFCMADDCPH256_MASKZ`.
    fn vfcmaddcph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH256_MASKZ");
        }
    }
    /// Emits `VFCMADDCPH256RRB`.
    fn vfcmaddcph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH256RRB_MASK`.
    fn vfcmaddcph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH256RRB_MASKZ`.
    fn vfcmaddcph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512`.
    fn vfcmaddcph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH512");
        }
    }
    /// Emits `VFCMADDCPH512RRR_ER`.
    fn vfcmaddcph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512_MASK`.
    fn vfcmaddcph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH512_MASK");
        }
    }
    /// Emits `VFCMADDCPH512RRR_MASK_ER`.
    fn vfcmaddcph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512_MASKZ`.
    fn vfcmaddcph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCPH512_MASKZ");
        }
    }
    /// Emits `VFCMADDCPH512RRR_MASKZ_ER`.
    fn vfcmaddcph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512RRB`.
    fn vfcmaddcph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512RRB_MASK`.
    fn vfcmaddcph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCPH512RRB_MASKZ`.
    fn vfcmaddcph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCSH`.
    fn vfcmaddcsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCSH");
        }
    }
    /// Emits `VFCMADDCSHRRR_ER`.
    fn vfcmaddcsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCSH_MASK`.
    fn vfcmaddcsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCSH_MASK");
        }
    }
    /// Emits `VFCMADDCSHRRR_MASK_ER`.
    fn vfcmaddcsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMADDCSH_MASKZ`.
    fn vfcmaddcsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMADDCSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMADDCSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMADDCSH_MASKZ");
        }
    }
    /// Emits `VFCMADDCSHRRR_MASKZ_ER`.
    fn vfcmaddcsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMADDCSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH128`.
    fn vfcmulcph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH128");
        }
    }
    /// Emits `VFCMULCPH128_MASK`.
    fn vfcmulcph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH128_MASK");
        }
    }
    /// Emits `VFCMULCPH128_MASKZ`.
    fn vfcmulcph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH128_MASKZ");
        }
    }
    /// Emits `VFCMULCPH128RRB`.
    fn vfcmulcph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH128RRB_MASK`.
    fn vfcmulcph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH128RRB_MASKZ`.
    fn vfcmulcph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH256`.
    fn vfcmulcph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH256");
        }
    }
    /// Emits `VFCMULCPH256_MASK`.
    fn vfcmulcph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH256_MASK");
        }
    }
    /// Emits `VFCMULCPH256_MASKZ`.
    fn vfcmulcph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH256_MASKZ");
        }
    }
    /// Emits `VFCMULCPH256RRB`.
    fn vfcmulcph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH256RRB_MASK`.
    fn vfcmulcph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH256RRB_MASKZ`.
    fn vfcmulcph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512`.
    fn vfcmulcph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH512");
        }
    }
    /// Emits `VFCMULCPH512RRR_ER`.
    fn vfcmulcph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512_MASK`.
    fn vfcmulcph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH512_MASK");
        }
    }
    /// Emits `VFCMULCPH512RRR_MASK_ER`.
    fn vfcmulcph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512_MASKZ`.
    fn vfcmulcph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCPH512_MASKZ");
        }
    }
    /// Emits `VFCMULCPH512RRR_MASKZ_ER`.
    fn vfcmulcph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512RRB`.
    fn vfcmulcph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512RRB_MASK`.
    fn vfcmulcph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCPH512RRB_MASKZ`.
    fn vfcmulcph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCSH`.
    fn vfcmulcsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCSH");
        }
    }
    /// Emits `VFCMULCSHRRR_ER`.
    fn vfcmulcsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCSH_MASK`.
    fn vfcmulcsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCSH_MASK");
        }
    }
    /// Emits `VFCMULCSHRRR_MASK_ER`.
    fn vfcmulcsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFCMULCSH_MASKZ`.
    fn vfcmulcsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFCMULCSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFCMULCSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFCMULCSH_MASKZ");
        }
    }
    /// Emits `VFCMULCSHRRR_MASKZ_ER`.
    fn vfcmulcsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFCMULCSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH128`.
    fn vfmadd132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH128");
        }
    }
    /// Emits `VFMADD132PH128_MASK`.
    fn vfmadd132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH128_MASK");
        }
    }
    /// Emits `VFMADD132PH128_MASKZ`.
    fn vfmadd132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH128_MASKZ");
        }
    }
    /// Emits `VFMADD132PH128RRB`.
    fn vfmadd132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH128RRB_MASK`.
    fn vfmadd132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH128RRB_MASKZ`.
    fn vfmadd132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH256`.
    fn vfmadd132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH256");
        }
    }
    /// Emits `VFMADD132PH256_MASK`.
    fn vfmadd132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH256_MASK");
        }
    }
    /// Emits `VFMADD132PH256_MASKZ`.
    fn vfmadd132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH256_MASKZ");
        }
    }
    /// Emits `VFMADD132PH256RRB`.
    fn vfmadd132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH256RRB_MASK`.
    fn vfmadd132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH256RRB_MASKZ`.
    fn vfmadd132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512`.
    fn vfmadd132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH512");
        }
    }
    /// Emits `VFMADD132PH512RRR_ER`.
    fn vfmadd132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512_MASK`.
    fn vfmadd132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH512_MASK");
        }
    }
    /// Emits `VFMADD132PH512RRR_MASK_ER`.
    fn vfmadd132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512_MASKZ`.
    fn vfmadd132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132PH512_MASKZ");
        }
    }
    /// Emits `VFMADD132PH512RRR_MASKZ_ER`.
    fn vfmadd132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512RRB`.
    fn vfmadd132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512RRB_MASK`.
    fn vfmadd132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132PH512RRB_MASKZ`.
    fn vfmadd132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132SH`.
    fn vfmadd132sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132SH");
        }
    }
    /// Emits `VFMADD132SHRRR_ER`.
    fn vfmadd132sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132SH_MASK`.
    fn vfmadd132sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132SH_MASK");
        }
    }
    /// Emits `VFMADD132SHRRR_MASK_ER`.
    fn vfmadd132sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD132SH_MASKZ`.
    fn vfmadd132sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD132SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD132SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD132SH_MASKZ");
        }
    }
    /// Emits `VFMADD132SHRRR_MASKZ_ER`.
    fn vfmadd132sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD132SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH128`.
    fn vfmadd213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH128");
        }
    }
    /// Emits `VFMADD213PH128_MASK`.
    fn vfmadd213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH128_MASK");
        }
    }
    /// Emits `VFMADD213PH128_MASKZ`.
    fn vfmadd213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH128_MASKZ");
        }
    }
    /// Emits `VFMADD213PH128RRB`.
    fn vfmadd213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH128RRB_MASK`.
    fn vfmadd213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH128RRB_MASKZ`.
    fn vfmadd213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH256`.
    fn vfmadd213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH256");
        }
    }
    /// Emits `VFMADD213PH256_MASK`.
    fn vfmadd213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH256_MASK");
        }
    }
    /// Emits `VFMADD213PH256_MASKZ`.
    fn vfmadd213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH256_MASKZ");
        }
    }
    /// Emits `VFMADD213PH256RRB`.
    fn vfmadd213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH256RRB_MASK`.
    fn vfmadd213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH256RRB_MASKZ`.
    fn vfmadd213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512`.
    fn vfmadd213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH512");
        }
    }
    /// Emits `VFMADD213PH512RRR_ER`.
    fn vfmadd213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512_MASK`.
    fn vfmadd213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH512_MASK");
        }
    }
    /// Emits `VFMADD213PH512RRR_MASK_ER`.
    fn vfmadd213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512_MASKZ`.
    fn vfmadd213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213PH512_MASKZ");
        }
    }
    /// Emits `VFMADD213PH512RRR_MASKZ_ER`.
    fn vfmadd213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512RRB`.
    fn vfmadd213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512RRB_MASK`.
    fn vfmadd213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213PH512RRB_MASKZ`.
    fn vfmadd213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213SH`.
    fn vfmadd213sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213SH");
        }
    }
    /// Emits `VFMADD213SHRRR_ER`.
    fn vfmadd213sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213SH_MASK`.
    fn vfmadd213sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213SH_MASK");
        }
    }
    /// Emits `VFMADD213SHRRR_MASK_ER`.
    fn vfmadd213sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD213SH_MASKZ`.
    fn vfmadd213sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD213SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD213SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD213SH_MASKZ");
        }
    }
    /// Emits `VFMADD213SHRRR_MASKZ_ER`.
    fn vfmadd213sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD213SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH128`.
    fn vfmadd231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH128");
        }
    }
    /// Emits `VFMADD231PH128_MASK`.
    fn vfmadd231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH128_MASK");
        }
    }
    /// Emits `VFMADD231PH128_MASKZ`.
    fn vfmadd231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH128_MASKZ");
        }
    }
    /// Emits `VFMADD231PH128RRB`.
    fn vfmadd231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH128RRB_MASK`.
    fn vfmadd231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH128RRB_MASKZ`.
    fn vfmadd231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH256`.
    fn vfmadd231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH256");
        }
    }
    /// Emits `VFMADD231PH256_MASK`.
    fn vfmadd231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH256_MASK");
        }
    }
    /// Emits `VFMADD231PH256_MASKZ`.
    fn vfmadd231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH256_MASKZ");
        }
    }
    /// Emits `VFMADD231PH256RRB`.
    fn vfmadd231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH256RRB_MASK`.
    fn vfmadd231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH256RRB_MASKZ`.
    fn vfmadd231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512`.
    fn vfmadd231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH512");
        }
    }
    /// Emits `VFMADD231PH512RRR_ER`.
    fn vfmadd231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512_MASK`.
    fn vfmadd231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH512_MASK");
        }
    }
    /// Emits `VFMADD231PH512RRR_MASK_ER`.
    fn vfmadd231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512_MASKZ`.
    fn vfmadd231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231PH512_MASKZ");
        }
    }
    /// Emits `VFMADD231PH512RRR_MASKZ_ER`.
    fn vfmadd231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512RRB`.
    fn vfmadd231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512RRB_MASK`.
    fn vfmadd231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231PH512RRB_MASKZ`.
    fn vfmadd231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231SH`.
    fn vfmadd231sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231SH");
        }
    }
    /// Emits `VFMADD231SHRRR_ER`.
    fn vfmadd231sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231SH_MASK`.
    fn vfmadd231sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231SH_MASK");
        }
    }
    /// Emits `VFMADD231SHRRR_MASK_ER`.
    fn vfmadd231sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADD231SH_MASKZ`.
    fn vfmadd231sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADD231SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADD231SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADD231SH_MASKZ");
        }
    }
    /// Emits `VFMADD231SHRRR_MASKZ_ER`.
    fn vfmadd231sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADD231SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH128`.
    fn vfmaddcph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH128");
        }
    }
    /// Emits `VFMADDCPH128_MASK`.
    fn vfmaddcph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH128_MASK");
        }
    }
    /// Emits `VFMADDCPH128_MASKZ`.
    fn vfmaddcph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH128_MASKZ");
        }
    }
    /// Emits `VFMADDCPH128RRB`.
    fn vfmaddcph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH128RRB_MASK`.
    fn vfmaddcph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH128RRB_MASKZ`.
    fn vfmaddcph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH256`.
    fn vfmaddcph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH256");
        }
    }
    /// Emits `VFMADDCPH256_MASK`.
    fn vfmaddcph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH256_MASK");
        }
    }
    /// Emits `VFMADDCPH256_MASKZ`.
    fn vfmaddcph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH256_MASKZ");
        }
    }
    /// Emits `VFMADDCPH256RRB`.
    fn vfmaddcph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH256RRB_MASK`.
    fn vfmaddcph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH256RRB_MASKZ`.
    fn vfmaddcph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512`.
    fn vfmaddcph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH512");
        }
    }
    /// Emits `VFMADDCPH512RRR_ER`.
    fn vfmaddcph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512_MASK`.
    fn vfmaddcph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH512_MASK");
        }
    }
    /// Emits `VFMADDCPH512RRR_MASK_ER`.
    fn vfmaddcph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512_MASKZ`.
    fn vfmaddcph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCPH512_MASKZ");
        }
    }
    /// Emits `VFMADDCPH512RRR_MASKZ_ER`.
    fn vfmaddcph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512RRB`.
    fn vfmaddcph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512RRB_MASK`.
    fn vfmaddcph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCPH512RRB_MASKZ`.
    fn vfmaddcph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCSH`.
    fn vfmaddcsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCSH");
        }
    }
    /// Emits `VFMADDCSHRRR_ER`.
    fn vfmaddcsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCSH_MASK`.
    fn vfmaddcsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCSH_MASK");
        }
    }
    /// Emits `VFMADDCSHRRR_MASK_ER`.
    fn vfmaddcsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDCSH_MASKZ`.
    fn vfmaddcsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDCSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDCSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDCSH_MASKZ");
        }
    }
    /// Emits `VFMADDCSHRRR_MASKZ_ER`.
    fn vfmaddcsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDCSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH128`.
    fn vfmaddsub132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH128");
        }
    }
    /// Emits `VFMADDSUB132PH128_MASK`.
    fn vfmaddsub132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH128_MASK");
        }
    }
    /// Emits `VFMADDSUB132PH128_MASKZ`.
    fn vfmaddsub132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH128_MASKZ");
        }
    }
    /// Emits `VFMADDSUB132PH128RRB`.
    fn vfmaddsub132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH128RRB_MASK`.
    fn vfmaddsub132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH128RRB_MASKZ`.
    fn vfmaddsub132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH256`.
    fn vfmaddsub132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH256");
        }
    }
    /// Emits `VFMADDSUB132PH256_MASK`.
    fn vfmaddsub132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH256_MASK");
        }
    }
    /// Emits `VFMADDSUB132PH256_MASKZ`.
    fn vfmaddsub132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH256_MASKZ");
        }
    }
    /// Emits `VFMADDSUB132PH256RRB`.
    fn vfmaddsub132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH256RRB_MASK`.
    fn vfmaddsub132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH256RRB_MASKZ`.
    fn vfmaddsub132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512`.
    fn vfmaddsub132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH512");
        }
    }
    /// Emits `VFMADDSUB132PH512RRR_ER`.
    fn vfmaddsub132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512_MASK`.
    fn vfmaddsub132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH512_MASK");
        }
    }
    /// Emits `VFMADDSUB132PH512RRR_MASK_ER`.
    fn vfmaddsub132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512_MASKZ`.
    fn vfmaddsub132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB132PH512_MASKZ");
        }
    }
    /// Emits `VFMADDSUB132PH512RRR_MASKZ_ER`.
    fn vfmaddsub132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512RRB`.
    fn vfmaddsub132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512RRB_MASK`.
    fn vfmaddsub132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB132PH512RRB_MASKZ`.
    fn vfmaddsub132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH128`.
    fn vfmaddsub213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH128");
        }
    }
    /// Emits `VFMADDSUB213PH128_MASK`.
    fn vfmaddsub213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH128_MASK");
        }
    }
    /// Emits `VFMADDSUB213PH128_MASKZ`.
    fn vfmaddsub213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH128_MASKZ");
        }
    }
    /// Emits `VFMADDSUB213PH128RRB`.
    fn vfmaddsub213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH128RRB_MASK`.
    fn vfmaddsub213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH128RRB_MASKZ`.
    fn vfmaddsub213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH256`.
    fn vfmaddsub213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH256");
        }
    }
    /// Emits `VFMADDSUB213PH256_MASK`.
    fn vfmaddsub213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH256_MASK");
        }
    }
    /// Emits `VFMADDSUB213PH256_MASKZ`.
    fn vfmaddsub213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH256_MASKZ");
        }
    }
    /// Emits `VFMADDSUB213PH256RRB`.
    fn vfmaddsub213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH256RRB_MASK`.
    fn vfmaddsub213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH256RRB_MASKZ`.
    fn vfmaddsub213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512`.
    fn vfmaddsub213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH512");
        }
    }
    /// Emits `VFMADDSUB213PH512RRR_ER`.
    fn vfmaddsub213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512_MASK`.
    fn vfmaddsub213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH512_MASK");
        }
    }
    /// Emits `VFMADDSUB213PH512RRR_MASK_ER`.
    fn vfmaddsub213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512_MASKZ`.
    fn vfmaddsub213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB213PH512_MASKZ");
        }
    }
    /// Emits `VFMADDSUB213PH512RRR_MASKZ_ER`.
    fn vfmaddsub213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512RRB`.
    fn vfmaddsub213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512RRB_MASK`.
    fn vfmaddsub213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB213PH512RRB_MASKZ`.
    fn vfmaddsub213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH128`.
    fn vfmaddsub231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH128");
        }
    }
    /// Emits `VFMADDSUB231PH128_MASK`.
    fn vfmaddsub231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH128_MASK");
        }
    }
    /// Emits `VFMADDSUB231PH128_MASKZ`.
    fn vfmaddsub231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH128_MASKZ");
        }
    }
    /// Emits `VFMADDSUB231PH128RRB`.
    fn vfmaddsub231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH128RRB_MASK`.
    fn vfmaddsub231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH128RRB_MASKZ`.
    fn vfmaddsub231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH256`.
    fn vfmaddsub231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH256");
        }
    }
    /// Emits `VFMADDSUB231PH256_MASK`.
    fn vfmaddsub231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH256_MASK");
        }
    }
    /// Emits `VFMADDSUB231PH256_MASKZ`.
    fn vfmaddsub231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH256_MASKZ");
        }
    }
    /// Emits `VFMADDSUB231PH256RRB`.
    fn vfmaddsub231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH256RRB_MASK`.
    fn vfmaddsub231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH256RRB_MASKZ`.
    fn vfmaddsub231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512`.
    fn vfmaddsub231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH512");
        }
    }
    /// Emits `VFMADDSUB231PH512RRR_ER`.
    fn vfmaddsub231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512_MASK`.
    fn vfmaddsub231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH512_MASK");
        }
    }
    /// Emits `VFMADDSUB231PH512RRR_MASK_ER`.
    fn vfmaddsub231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512_MASKZ`.
    fn vfmaddsub231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMADDSUB231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMADDSUB231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMADDSUB231PH512_MASKZ");
        }
    }
    /// Emits `VFMADDSUB231PH512RRR_MASKZ_ER`.
    fn vfmaddsub231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512RRB`.
    fn vfmaddsub231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512RRB_MASK`.
    fn vfmaddsub231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMADDSUB231PH512RRB_MASKZ`.
    fn vfmaddsub231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMADDSUB231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH128`.
    fn vfmsub132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH128");
        }
    }
    /// Emits `VFMSUB132PH128_MASK`.
    fn vfmsub132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH128_MASK");
        }
    }
    /// Emits `VFMSUB132PH128_MASKZ`.
    fn vfmsub132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH128_MASKZ");
        }
    }
    /// Emits `VFMSUB132PH128RRB`.
    fn vfmsub132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH128RRB_MASK`.
    fn vfmsub132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH128RRB_MASKZ`.
    fn vfmsub132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH256`.
    fn vfmsub132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH256");
        }
    }
    /// Emits `VFMSUB132PH256_MASK`.
    fn vfmsub132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH256_MASK");
        }
    }
    /// Emits `VFMSUB132PH256_MASKZ`.
    fn vfmsub132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH256_MASKZ");
        }
    }
    /// Emits `VFMSUB132PH256RRB`.
    fn vfmsub132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH256RRB_MASK`.
    fn vfmsub132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH256RRB_MASKZ`.
    fn vfmsub132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512`.
    fn vfmsub132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH512");
        }
    }
    /// Emits `VFMSUB132PH512RRR_ER`.
    fn vfmsub132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512_MASK`.
    fn vfmsub132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH512_MASK");
        }
    }
    /// Emits `VFMSUB132PH512RRR_MASK_ER`.
    fn vfmsub132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512_MASKZ`.
    fn vfmsub132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132PH512_MASKZ");
        }
    }
    /// Emits `VFMSUB132PH512RRR_MASKZ_ER`.
    fn vfmsub132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512RRB`.
    fn vfmsub132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512RRB_MASK`.
    fn vfmsub132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132PH512RRB_MASKZ`.
    fn vfmsub132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132SH`.
    fn vfmsub132sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132SH");
        }
    }
    /// Emits `VFMSUB132SHRRR_ER`.
    fn vfmsub132sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132SH_MASK`.
    fn vfmsub132sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132SH_MASK");
        }
    }
    /// Emits `VFMSUB132SHRRR_MASK_ER`.
    fn vfmsub132sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB132SH_MASKZ`.
    fn vfmsub132sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB132SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB132SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB132SH_MASKZ");
        }
    }
    /// Emits `VFMSUB132SHRRR_MASKZ_ER`.
    fn vfmsub132sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB132SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH128`.
    fn vfmsub213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH128");
        }
    }
    /// Emits `VFMSUB213PH128_MASK`.
    fn vfmsub213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH128_MASK");
        }
    }
    /// Emits `VFMSUB213PH128_MASKZ`.
    fn vfmsub213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH128_MASKZ");
        }
    }
    /// Emits `VFMSUB213PH128RRB`.
    fn vfmsub213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH128RRB_MASK`.
    fn vfmsub213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH128RRB_MASKZ`.
    fn vfmsub213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH256`.
    fn vfmsub213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH256");
        }
    }
    /// Emits `VFMSUB213PH256_MASK`.
    fn vfmsub213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH256_MASK");
        }
    }
    /// Emits `VFMSUB213PH256_MASKZ`.
    fn vfmsub213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH256_MASKZ");
        }
    }
    /// Emits `VFMSUB213PH256RRB`.
    fn vfmsub213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH256RRB_MASK`.
    fn vfmsub213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH256RRB_MASKZ`.
    fn vfmsub213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512`.
    fn vfmsub213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH512");
        }
    }
    /// Emits `VFMSUB213PH512RRR_ER`.
    fn vfmsub213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512_MASK`.
    fn vfmsub213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH512_MASK");
        }
    }
    /// Emits `VFMSUB213PH512RRR_MASK_ER`.
    fn vfmsub213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512_MASKZ`.
    fn vfmsub213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213PH512_MASKZ");
        }
    }
    /// Emits `VFMSUB213PH512RRR_MASKZ_ER`.
    fn vfmsub213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512RRB`.
    fn vfmsub213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512RRB_MASK`.
    fn vfmsub213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213PH512RRB_MASKZ`.
    fn vfmsub213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213SH`.
    fn vfmsub213sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213SH");
        }
    }
    /// Emits `VFMSUB213SHRRR_ER`.
    fn vfmsub213sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213SH_MASK`.
    fn vfmsub213sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213SH_MASK");
        }
    }
    /// Emits `VFMSUB213SHRRR_MASK_ER`.
    fn vfmsub213sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB213SH_MASKZ`.
    fn vfmsub213sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB213SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB213SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB213SH_MASKZ");
        }
    }
    /// Emits `VFMSUB213SHRRR_MASKZ_ER`.
    fn vfmsub213sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB213SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH128`.
    fn vfmsub231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH128");
        }
    }
    /// Emits `VFMSUB231PH128_MASK`.
    fn vfmsub231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH128_MASK");
        }
    }
    /// Emits `VFMSUB231PH128_MASKZ`.
    fn vfmsub231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH128_MASKZ");
        }
    }
    /// Emits `VFMSUB231PH128RRB`.
    fn vfmsub231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH128RRB_MASK`.
    fn vfmsub231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH128RRB_MASKZ`.
    fn vfmsub231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH256`.
    fn vfmsub231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH256");
        }
    }
    /// Emits `VFMSUB231PH256_MASK`.
    fn vfmsub231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH256_MASK");
        }
    }
    /// Emits `VFMSUB231PH256_MASKZ`.
    fn vfmsub231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH256_MASKZ");
        }
    }
    /// Emits `VFMSUB231PH256RRB`.
    fn vfmsub231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH256RRB_MASK`.
    fn vfmsub231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH256RRB_MASKZ`.
    fn vfmsub231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512`.
    fn vfmsub231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH512");
        }
    }
    /// Emits `VFMSUB231PH512RRR_ER`.
    fn vfmsub231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512_MASK`.
    fn vfmsub231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH512_MASK");
        }
    }
    /// Emits `VFMSUB231PH512RRR_MASK_ER`.
    fn vfmsub231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512_MASKZ`.
    fn vfmsub231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231PH512_MASKZ");
        }
    }
    /// Emits `VFMSUB231PH512RRR_MASKZ_ER`.
    fn vfmsub231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512RRB`.
    fn vfmsub231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512RRB_MASK`.
    fn vfmsub231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231PH512RRB_MASKZ`.
    fn vfmsub231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231SH`.
    fn vfmsub231sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231SH");
        }
    }
    /// Emits `VFMSUB231SHRRR_ER`.
    fn vfmsub231sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231SH_MASK`.
    fn vfmsub231sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231SH_MASK");
        }
    }
    /// Emits `VFMSUB231SHRRR_MASK_ER`.
    fn vfmsub231sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUB231SH_MASKZ`.
    fn vfmsub231sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUB231SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUB231SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUB231SH_MASKZ");
        }
    }
    /// Emits `VFMSUB231SHRRR_MASKZ_ER`.
    fn vfmsub231sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUB231SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH128`.
    fn vfmsubadd132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH128");
        }
    }
    /// Emits `VFMSUBADD132PH128_MASK`.
    fn vfmsubadd132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH128_MASK");
        }
    }
    /// Emits `VFMSUBADD132PH128_MASKZ`.
    fn vfmsubadd132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH128_MASKZ");
        }
    }
    /// Emits `VFMSUBADD132PH128RRB`.
    fn vfmsubadd132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH128RRB_MASK`.
    fn vfmsubadd132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH128RRB_MASKZ`.
    fn vfmsubadd132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH256`.
    fn vfmsubadd132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH256");
        }
    }
    /// Emits `VFMSUBADD132PH256_MASK`.
    fn vfmsubadd132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH256_MASK");
        }
    }
    /// Emits `VFMSUBADD132PH256_MASKZ`.
    fn vfmsubadd132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH256_MASKZ");
        }
    }
    /// Emits `VFMSUBADD132PH256RRB`.
    fn vfmsubadd132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH256RRB_MASK`.
    fn vfmsubadd132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH256RRB_MASKZ`.
    fn vfmsubadd132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512`.
    fn vfmsubadd132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH512");
        }
    }
    /// Emits `VFMSUBADD132PH512RRR_ER`.
    fn vfmsubadd132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512_MASK`.
    fn vfmsubadd132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH512_MASK");
        }
    }
    /// Emits `VFMSUBADD132PH512RRR_MASK_ER`.
    fn vfmsubadd132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512_MASKZ`.
    fn vfmsubadd132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD132PH512_MASKZ");
        }
    }
    /// Emits `VFMSUBADD132PH512RRR_MASKZ_ER`.
    fn vfmsubadd132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512RRB`.
    fn vfmsubadd132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512RRB_MASK`.
    fn vfmsubadd132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD132PH512RRB_MASKZ`.
    fn vfmsubadd132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH128`.
    fn vfmsubadd213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH128");
        }
    }
    /// Emits `VFMSUBADD213PH128_MASK`.
    fn vfmsubadd213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH128_MASK");
        }
    }
    /// Emits `VFMSUBADD213PH128_MASKZ`.
    fn vfmsubadd213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH128_MASKZ");
        }
    }
    /// Emits `VFMSUBADD213PH128RRB`.
    fn vfmsubadd213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH128RRB_MASK`.
    fn vfmsubadd213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH128RRB_MASKZ`.
    fn vfmsubadd213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH256`.
    fn vfmsubadd213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH256");
        }
    }
    /// Emits `VFMSUBADD213PH256_MASK`.
    fn vfmsubadd213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH256_MASK");
        }
    }
    /// Emits `VFMSUBADD213PH256_MASKZ`.
    fn vfmsubadd213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH256_MASKZ");
        }
    }
    /// Emits `VFMSUBADD213PH256RRB`.
    fn vfmsubadd213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH256RRB_MASK`.
    fn vfmsubadd213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH256RRB_MASKZ`.
    fn vfmsubadd213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512`.
    fn vfmsubadd213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH512");
        }
    }
    /// Emits `VFMSUBADD213PH512RRR_ER`.
    fn vfmsubadd213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512_MASK`.
    fn vfmsubadd213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH512_MASK");
        }
    }
    /// Emits `VFMSUBADD213PH512RRR_MASK_ER`.
    fn vfmsubadd213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512_MASKZ`.
    fn vfmsubadd213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD213PH512_MASKZ");
        }
    }
    /// Emits `VFMSUBADD213PH512RRR_MASKZ_ER`.
    fn vfmsubadd213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512RRB`.
    fn vfmsubadd213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512RRB_MASK`.
    fn vfmsubadd213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD213PH512RRB_MASKZ`.
    fn vfmsubadd213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH128`.
    fn vfmsubadd231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH128");
        }
    }
    /// Emits `VFMSUBADD231PH128_MASK`.
    fn vfmsubadd231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH128_MASK");
        }
    }
    /// Emits `VFMSUBADD231PH128_MASKZ`.
    fn vfmsubadd231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH128_MASKZ");
        }
    }
    /// Emits `VFMSUBADD231PH128RRB`.
    fn vfmsubadd231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH128RRB_MASK`.
    fn vfmsubadd231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH128RRB_MASKZ`.
    fn vfmsubadd231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH256`.
    fn vfmsubadd231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH256");
        }
    }
    /// Emits `VFMSUBADD231PH256_MASK`.
    fn vfmsubadd231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH256_MASK");
        }
    }
    /// Emits `VFMSUBADD231PH256_MASKZ`.
    fn vfmsubadd231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH256_MASKZ");
        }
    }
    /// Emits `VFMSUBADD231PH256RRB`.
    fn vfmsubadd231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH256RRB_MASK`.
    fn vfmsubadd231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH256RRB_MASKZ`.
    fn vfmsubadd231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512`.
    fn vfmsubadd231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH512");
        }
    }
    /// Emits `VFMSUBADD231PH512RRR_ER`.
    fn vfmsubadd231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512_MASK`.
    fn vfmsubadd231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH512_MASK");
        }
    }
    /// Emits `VFMSUBADD231PH512RRR_MASK_ER`.
    fn vfmsubadd231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512_MASKZ`.
    fn vfmsubadd231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMSUBADD231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMSUBADD231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMSUBADD231PH512_MASKZ");
        }
    }
    /// Emits `VFMSUBADD231PH512RRR_MASKZ_ER`.
    fn vfmsubadd231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512RRB`.
    fn vfmsubadd231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512RRB_MASK`.
    fn vfmsubadd231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMSUBADD231PH512RRB_MASKZ`.
    fn vfmsubadd231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMSUBADD231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH128`.
    fn vfmulcph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH128");
        }
    }
    /// Emits `VFMULCPH128_MASK`.
    fn vfmulcph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH128_MASK");
        }
    }
    /// Emits `VFMULCPH128_MASKZ`.
    fn vfmulcph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH128_MASKZ");
        }
    }
    /// Emits `VFMULCPH128RRB`.
    fn vfmulcph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH128RRB_MASK`.
    fn vfmulcph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH128RRB_MASKZ`.
    fn vfmulcph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH256`.
    fn vfmulcph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH256");
        }
    }
    /// Emits `VFMULCPH256_MASK`.
    fn vfmulcph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH256_MASK");
        }
    }
    /// Emits `VFMULCPH256_MASKZ`.
    fn vfmulcph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH256_MASKZ");
        }
    }
    /// Emits `VFMULCPH256RRB`.
    fn vfmulcph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH256RRB_MASK`.
    fn vfmulcph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH256RRB_MASKZ`.
    fn vfmulcph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512`.
    fn vfmulcph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH512");
        }
    }
    /// Emits `VFMULCPH512RRR_ER`.
    fn vfmulcph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512_MASK`.
    fn vfmulcph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH512_MASK");
        }
    }
    /// Emits `VFMULCPH512RRR_MASK_ER`.
    fn vfmulcph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512_MASKZ`.
    fn vfmulcph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCPH512_MASKZ");
        }
    }
    /// Emits `VFMULCPH512RRR_MASKZ_ER`.
    fn vfmulcph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512RRB`.
    fn vfmulcph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512RRB_MASK`.
    fn vfmulcph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCPH512RRB_MASKZ`.
    fn vfmulcph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCSH`.
    fn vfmulcsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCSH");
        }
    }
    /// Emits `VFMULCSHRRR_ER`.
    fn vfmulcsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCSH_MASK`.
    fn vfmulcsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCSH_MASK");
        }
    }
    /// Emits `VFMULCSHRRR_MASK_ER`.
    fn vfmulcsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFMULCSH_MASKZ`.
    fn vfmulcsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFMULCSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFMULCSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFMULCSH_MASKZ");
        }
    }
    /// Emits `VFMULCSHRRR_MASKZ_ER`.
    fn vfmulcsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFMULCSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH128`.
    fn vfnmadd132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH128");
        }
    }
    /// Emits `VFNMADD132PH128_MASK`.
    fn vfnmadd132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH128_MASK");
        }
    }
    /// Emits `VFNMADD132PH128_MASKZ`.
    fn vfnmadd132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH128_MASKZ");
        }
    }
    /// Emits `VFNMADD132PH128RRB`.
    fn vfnmadd132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH128RRB_MASK`.
    fn vfnmadd132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH128RRB_MASKZ`.
    fn vfnmadd132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH256`.
    fn vfnmadd132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH256");
        }
    }
    /// Emits `VFNMADD132PH256_MASK`.
    fn vfnmadd132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH256_MASK");
        }
    }
    /// Emits `VFNMADD132PH256_MASKZ`.
    fn vfnmadd132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH256_MASKZ");
        }
    }
    /// Emits `VFNMADD132PH256RRB`.
    fn vfnmadd132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH256RRB_MASK`.
    fn vfnmadd132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH256RRB_MASKZ`.
    fn vfnmadd132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512`.
    fn vfnmadd132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH512");
        }
    }
    /// Emits `VFNMADD132PH512RRR_ER`.
    fn vfnmadd132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512_MASK`.
    fn vfnmadd132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH512_MASK");
        }
    }
    /// Emits `VFNMADD132PH512RRR_MASK_ER`.
    fn vfnmadd132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512_MASKZ`.
    fn vfnmadd132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132PH512_MASKZ");
        }
    }
    /// Emits `VFNMADD132PH512RRR_MASKZ_ER`.
    fn vfnmadd132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512RRB`.
    fn vfnmadd132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512RRB_MASK`.
    fn vfnmadd132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132PH512RRB_MASKZ`.
    fn vfnmadd132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132SH`.
    fn vfnmadd132sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132SH");
        }
    }
    /// Emits `VFNMADD132SHRRR_ER`.
    fn vfnmadd132sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132SH_MASK`.
    fn vfnmadd132sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132SH_MASK");
        }
    }
    /// Emits `VFNMADD132SHRRR_MASK_ER`.
    fn vfnmadd132sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD132SH_MASKZ`.
    fn vfnmadd132sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD132SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD132SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD132SH_MASKZ");
        }
    }
    /// Emits `VFNMADD132SHRRR_MASKZ_ER`.
    fn vfnmadd132sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD132SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH128`.
    fn vfnmadd213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH128");
        }
    }
    /// Emits `VFNMADD213PH128_MASK`.
    fn vfnmadd213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH128_MASK");
        }
    }
    /// Emits `VFNMADD213PH128_MASKZ`.
    fn vfnmadd213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH128_MASKZ");
        }
    }
    /// Emits `VFNMADD213PH128RRB`.
    fn vfnmadd213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH128RRB_MASK`.
    fn vfnmadd213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH128RRB_MASKZ`.
    fn vfnmadd213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH256`.
    fn vfnmadd213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH256");
        }
    }
    /// Emits `VFNMADD213PH256_MASK`.
    fn vfnmadd213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH256_MASK");
        }
    }
    /// Emits `VFNMADD213PH256_MASKZ`.
    fn vfnmadd213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH256_MASKZ");
        }
    }
    /// Emits `VFNMADD213PH256RRB`.
    fn vfnmadd213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH256RRB_MASK`.
    fn vfnmadd213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH256RRB_MASKZ`.
    fn vfnmadd213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512`.
    fn vfnmadd213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH512");
        }
    }
    /// Emits `VFNMADD213PH512RRR_ER`.
    fn vfnmadd213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512_MASK`.
    fn vfnmadd213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH512_MASK");
        }
    }
    /// Emits `VFNMADD213PH512RRR_MASK_ER`.
    fn vfnmadd213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512_MASKZ`.
    fn vfnmadd213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213PH512_MASKZ");
        }
    }
    /// Emits `VFNMADD213PH512RRR_MASKZ_ER`.
    fn vfnmadd213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512RRB`.
    fn vfnmadd213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512RRB_MASK`.
    fn vfnmadd213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213PH512RRB_MASKZ`.
    fn vfnmadd213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213SH`.
    fn vfnmadd213sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213SH");
        }
    }
    /// Emits `VFNMADD213SHRRR_ER`.
    fn vfnmadd213sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213SH_MASK`.
    fn vfnmadd213sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213SH_MASK");
        }
    }
    /// Emits `VFNMADD213SHRRR_MASK_ER`.
    fn vfnmadd213sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD213SH_MASKZ`.
    fn vfnmadd213sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD213SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD213SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD213SH_MASKZ");
        }
    }
    /// Emits `VFNMADD213SHRRR_MASKZ_ER`.
    fn vfnmadd213sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD213SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH128`.
    fn vfnmadd231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH128");
        }
    }
    /// Emits `VFNMADD231PH128_MASK`.
    fn vfnmadd231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH128_MASK");
        }
    }
    /// Emits `VFNMADD231PH128_MASKZ`.
    fn vfnmadd231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH128_MASKZ");
        }
    }
    /// Emits `VFNMADD231PH128RRB`.
    fn vfnmadd231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH128RRB_MASK`.
    fn vfnmadd231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH128RRB_MASKZ`.
    fn vfnmadd231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH256`.
    fn vfnmadd231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH256");
        }
    }
    /// Emits `VFNMADD231PH256_MASK`.
    fn vfnmadd231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH256_MASK");
        }
    }
    /// Emits `VFNMADD231PH256_MASKZ`.
    fn vfnmadd231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH256_MASKZ");
        }
    }
    /// Emits `VFNMADD231PH256RRB`.
    fn vfnmadd231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH256RRB_MASK`.
    fn vfnmadd231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH256RRB_MASKZ`.
    fn vfnmadd231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512`.
    fn vfnmadd231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH512");
        }
    }
    /// Emits `VFNMADD231PH512RRR_ER`.
    fn vfnmadd231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512_MASK`.
    fn vfnmadd231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH512_MASK");
        }
    }
    /// Emits `VFNMADD231PH512RRR_MASK_ER`.
    fn vfnmadd231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512_MASKZ`.
    fn vfnmadd231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231PH512_MASKZ");
        }
    }
    /// Emits `VFNMADD231PH512RRR_MASKZ_ER`.
    fn vfnmadd231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512RRB`.
    fn vfnmadd231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512RRB_MASK`.
    fn vfnmadd231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231PH512RRB_MASKZ`.
    fn vfnmadd231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231SH`.
    fn vfnmadd231sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231SH");
        }
    }
    /// Emits `VFNMADD231SHRRR_ER`.
    fn vfnmadd231sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231SH_MASK`.
    fn vfnmadd231sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231SH_MASK");
        }
    }
    /// Emits `VFNMADD231SHRRR_MASK_ER`.
    fn vfnmadd231sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMADD231SH_MASKZ`.
    fn vfnmadd231sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMADD231SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMADD231SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMADD231SH_MASKZ");
        }
    }
    /// Emits `VFNMADD231SHRRR_MASKZ_ER`.
    fn vfnmadd231sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMADD231SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH128`.
    fn vfnmsub132ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH128");
        }
    }
    /// Emits `VFNMSUB132PH128_MASK`.
    fn vfnmsub132ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH128_MASK");
        }
    }
    /// Emits `VFNMSUB132PH128_MASKZ`.
    fn vfnmsub132ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH128_MASKZ");
        }
    }
    /// Emits `VFNMSUB132PH128RRB`.
    fn vfnmsub132ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH128RRB_MASK`.
    fn vfnmsub132ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH128RRB_MASKZ`.
    fn vfnmsub132ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH256`.
    fn vfnmsub132ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH256");
        }
    }
    /// Emits `VFNMSUB132PH256_MASK`.
    fn vfnmsub132ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH256_MASK");
        }
    }
    /// Emits `VFNMSUB132PH256_MASKZ`.
    fn vfnmsub132ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH256_MASKZ");
        }
    }
    /// Emits `VFNMSUB132PH256RRB`.
    fn vfnmsub132ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH256RRB_MASK`.
    fn vfnmsub132ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH256RRB_MASKZ`.
    fn vfnmsub132ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512`.
    fn vfnmsub132ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH512");
        }
    }
    /// Emits `VFNMSUB132PH512RRR_ER`.
    fn vfnmsub132ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512_MASK`.
    fn vfnmsub132ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH512_MASK");
        }
    }
    /// Emits `VFNMSUB132PH512RRR_MASK_ER`.
    fn vfnmsub132ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512_MASKZ`.
    fn vfnmsub132ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132PH512_MASKZ");
        }
    }
    /// Emits `VFNMSUB132PH512RRR_MASKZ_ER`.
    fn vfnmsub132ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512RRB`.
    fn vfnmsub132ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512RRB_MASK`.
    fn vfnmsub132ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132PH512RRB_MASKZ`.
    fn vfnmsub132ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132SH`.
    fn vfnmsub132sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132SH");
        }
    }
    /// Emits `VFNMSUB132SHRRR_ER`.
    fn vfnmsub132sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132SH_MASK`.
    fn vfnmsub132sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132SH_MASK");
        }
    }
    /// Emits `VFNMSUB132SHRRR_MASK_ER`.
    fn vfnmsub132sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB132SH_MASKZ`.
    fn vfnmsub132sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB132SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB132SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB132SH_MASKZ");
        }
    }
    /// Emits `VFNMSUB132SHRRR_MASKZ_ER`.
    fn vfnmsub132sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB132SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH128`.
    fn vfnmsub213ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH128");
        }
    }
    /// Emits `VFNMSUB213PH128_MASK`.
    fn vfnmsub213ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH128_MASK");
        }
    }
    /// Emits `VFNMSUB213PH128_MASKZ`.
    fn vfnmsub213ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH128_MASKZ");
        }
    }
    /// Emits `VFNMSUB213PH128RRB`.
    fn vfnmsub213ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH128RRB_MASK`.
    fn vfnmsub213ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH128RRB_MASKZ`.
    fn vfnmsub213ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH256`.
    fn vfnmsub213ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH256");
        }
    }
    /// Emits `VFNMSUB213PH256_MASK`.
    fn vfnmsub213ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH256_MASK");
        }
    }
    /// Emits `VFNMSUB213PH256_MASKZ`.
    fn vfnmsub213ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH256_MASKZ");
        }
    }
    /// Emits `VFNMSUB213PH256RRB`.
    fn vfnmsub213ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH256RRB_MASK`.
    fn vfnmsub213ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH256RRB_MASKZ`.
    fn vfnmsub213ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512`.
    fn vfnmsub213ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH512");
        }
    }
    /// Emits `VFNMSUB213PH512RRR_ER`.
    fn vfnmsub213ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512_MASK`.
    fn vfnmsub213ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH512_MASK");
        }
    }
    /// Emits `VFNMSUB213PH512RRR_MASK_ER`.
    fn vfnmsub213ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512_MASKZ`.
    fn vfnmsub213ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213PH512_MASKZ");
        }
    }
    /// Emits `VFNMSUB213PH512RRR_MASKZ_ER`.
    fn vfnmsub213ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512RRB`.
    fn vfnmsub213ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512RRB_MASK`.
    fn vfnmsub213ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213PH512RRB_MASKZ`.
    fn vfnmsub213ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213SH`.
    fn vfnmsub213sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213SH");
        }
    }
    /// Emits `VFNMSUB213SHRRR_ER`.
    fn vfnmsub213sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213SH_MASK`.
    fn vfnmsub213sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213SH_MASK");
        }
    }
    /// Emits `VFNMSUB213SHRRR_MASK_ER`.
    fn vfnmsub213sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB213SH_MASKZ`.
    fn vfnmsub213sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB213SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB213SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB213SH_MASKZ");
        }
    }
    /// Emits `VFNMSUB213SHRRR_MASKZ_ER`.
    fn vfnmsub213sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB213SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH128`.
    fn vfnmsub231ph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH128");
        }
    }
    /// Emits `VFNMSUB231PH128_MASK`.
    fn vfnmsub231ph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH128_MASK");
        }
    }
    /// Emits `VFNMSUB231PH128_MASKZ`.
    fn vfnmsub231ph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH128_MASKZ");
        }
    }
    /// Emits `VFNMSUB231PH128RRB`.
    fn vfnmsub231ph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH128RRB_MASK`.
    fn vfnmsub231ph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH128RRB_MASKZ`.
    fn vfnmsub231ph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH256`.
    fn vfnmsub231ph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH256");
        }
    }
    /// Emits `VFNMSUB231PH256_MASK`.
    fn vfnmsub231ph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH256_MASK");
        }
    }
    /// Emits `VFNMSUB231PH256_MASKZ`.
    fn vfnmsub231ph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH256_MASKZ");
        }
    }
    /// Emits `VFNMSUB231PH256RRB`.
    fn vfnmsub231ph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH256RRB_MASK`.
    fn vfnmsub231ph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH256RRB_MASKZ`.
    fn vfnmsub231ph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512`.
    fn vfnmsub231ph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH512");
        }
    }
    /// Emits `VFNMSUB231PH512RRR_ER`.
    fn vfnmsub231ph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512_MASK`.
    fn vfnmsub231ph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH512_MASK");
        }
    }
    /// Emits `VFNMSUB231PH512RRR_MASK_ER`.
    fn vfnmsub231ph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512_MASKZ`.
    fn vfnmsub231ph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231PH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231PH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231PH512_MASKZ");
        }
    }
    /// Emits `VFNMSUB231PH512RRR_MASKZ_ER`.
    fn vfnmsub231ph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512RRB`.
    fn vfnmsub231ph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512RRB_MASK`.
    fn vfnmsub231ph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231PH512RRB_MASKZ`.
    fn vfnmsub231ph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231PH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231SH`.
    fn vfnmsub231sh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231SHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231SHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231SH");
        }
    }
    /// Emits `VFNMSUB231SHRRR_ER`.
    fn vfnmsub231sh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231SHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231SH_MASK`.
    fn vfnmsub231sh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231SHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231SHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231SH_MASK");
        }
    }
    /// Emits `VFNMSUB231SHRRR_MASK_ER`.
    fn vfnmsub231sh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231SHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFNMSUB231SH_MASKZ`.
    fn vfnmsub231sh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VFNMSUB231SHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VFNMSUB231SHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFNMSUB231SH_MASKZ");
        }
    }
    /// Emits `VFNMSUB231SHRRR_MASKZ_ER`.
    fn vfnmsub231sh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFNMSUB231SHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH128K`.
    fn vfpclassph128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH128KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH128KMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH128K");
        }
    }
    /// Emits `VFPCLASSPH128K_MASK`.
    fn vfpclassph128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH128KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH128KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH128K_MASK");
        }
    }
    /// Emits `VFPCLASSPH128KBI`.
    fn vfpclassph128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH128KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH128KBI_MASK`.
    fn vfpclassph128kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH128KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH256K`.
    fn vfpclassph256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH256KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH256KMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH256K");
        }
    }
    /// Emits `VFPCLASSPH256K_MASK`.
    fn vfpclassph256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH256KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH256KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH256K_MASK");
        }
    }
    /// Emits `VFPCLASSPH256KBI`.
    fn vfpclassph256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH256KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH256KBI_MASK`.
    fn vfpclassph256kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH256KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH512K`.
    fn vfpclassph512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH512KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH512KMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH512K");
        }
    }
    /// Emits `VFPCLASSPH512K_MASK`.
    fn vfpclassph512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPH512KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPH512KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSPH512K_MASK");
        }
    }
    /// Emits `VFPCLASSPH512KBI`.
    fn vfpclassph512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH512KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSPH512KBI_MASK`.
    fn vfpclassph512kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPH512KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VFPCLASSSHK`.
    fn vfpclassshk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSHKRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSHKMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSSHK");
        }
    }
    /// Emits `VFPCLASSSHK_MASK`.
    fn vfpclassshk_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSHKRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSHKMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VFPCLASSSHK_MASK");
        }
    }
    /// Emits `VGETEXPPH128`.
    fn vgetexpph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH128");
        }
    }
    /// Emits `VGETEXPPH128_MASK`.
    fn vgetexpph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH128_MASK");
        }
    }
    /// Emits `VGETEXPPH128_MASKZ`.
    fn vgetexpph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH128_MASKZ");
        }
    }
    /// Emits `VGETEXPPH128RB`.
    fn vgetexpph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH128RB_MASK`.
    fn vgetexpph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH128RB_MASKZ`.
    fn vgetexpph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH256`.
    fn vgetexpph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH256");
        }
    }
    /// Emits `VGETEXPPH256_MASK`.
    fn vgetexpph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH256_MASK");
        }
    }
    /// Emits `VGETEXPPH256_MASKZ`.
    fn vgetexpph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH256_MASKZ");
        }
    }
    /// Emits `VGETEXPPH256RB`.
    fn vgetexpph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH256RB_MASK`.
    fn vgetexpph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH256RB_MASKZ`.
    fn vgetexpph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512`.
    fn vgetexpph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH512");
        }
    }
    /// Emits `VGETEXPPH512_MASK`.
    fn vgetexpph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH512_MASK");
        }
    }
    /// Emits `VGETEXPPH512RR_MASK_SAE`.
    fn vgetexpph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512_MASKZ`.
    fn vgetexpph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VGETEXPPH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VGETEXPPH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPPH512_MASKZ");
        }
    }
    /// Emits `VGETEXPPH512RR_MASKZ_SAE`.
    fn vgetexpph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512RR_SAE`.
    fn vgetexpph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512RB`.
    fn vgetexpph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512RB_MASK`.
    fn vgetexpph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPPH512RB_MASKZ`.
    fn vgetexpph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VGETEXPPH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VGETEXPSH`.
    fn vgetexpsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGETEXPSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGETEXPSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPSH");
        }
    }
    /// Emits `VGETEXPSH_MASK`.
    fn vgetexpsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGETEXPSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGETEXPSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPSH_MASK");
        }
    }
    /// Emits `VGETEXPSHRRR_MASK_SAE`.
    fn vgetexpsh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETEXPSHRRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETEXPSH_MASKZ`.
    fn vgetexpsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGETEXPSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGETEXPSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETEXPSH_MASKZ");
        }
    }
    /// Emits `VGETEXPSHRRR_MASKZ_SAE`.
    fn vgetexpsh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETEXPSHRRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETEXPSHRRR_SAE`.
    fn vgetexpsh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETEXPSHRRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH128`.
    fn vgetmantph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH128");
        }
    }
    /// Emits `VGETMANTPH128_MASK`.
    fn vgetmantph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH128_MASK");
        }
    }
    /// Emits `VGETMANTPH128_MASKZ`.
    fn vgetmantph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH128_MASKZ");
        }
    }
    /// Emits `VGETMANTPH128RBI`.
    fn vgetmantph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH128RBI_MASK`.
    fn vgetmantph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH128RBI_MASKZ`.
    fn vgetmantph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH256`.
    fn vgetmantph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH256");
        }
    }
    /// Emits `VGETMANTPH256_MASK`.
    fn vgetmantph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH256_MASK");
        }
    }
    /// Emits `VGETMANTPH256_MASKZ`.
    fn vgetmantph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH256_MASKZ");
        }
    }
    /// Emits `VGETMANTPH256RBI`.
    fn vgetmantph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH256RBI_MASK`.
    fn vgetmantph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH256RBI_MASKZ`.
    fn vgetmantph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512`.
    fn vgetmantph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH512");
        }
    }
    /// Emits `VGETMANTPH512_MASK`.
    fn vgetmantph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH512_MASK");
        }
    }
    /// Emits `VGETMANTPH512RRI_MASK_SAE`.
    fn vgetmantph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512_MASKZ`.
    fn vgetmantph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VGETMANTPH512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VGETMANTPH512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGETMANTPH512_MASKZ");
        }
    }
    /// Emits `VGETMANTPH512RRI_MASKZ_SAE`.
    fn vgetmantph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512RRI_SAE`.
    fn vgetmantph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512RBI`.
    fn vgetmantph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512RBI_MASK`.
    fn vgetmantph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTPH512RBI_MASKZ`.
    fn vgetmantph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGETMANTPH512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGETMANTSH`.
    fn vgetmantsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGETMANTSHRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGETMANTSHRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGETMANTSH");
        }
    }
    /// Emits `VGETMANTSH_MASK`.
    fn vgetmantsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGETMANTSHRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGETMANTSHRRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGETMANTSH_MASK");
        }
    }
    /// Emits `VGETMANTSHRRRI_MASK_SAE`.
    fn vgetmantsh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGETMANTSHRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGETMANTSH_MASKZ`.
    fn vgetmantsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGETMANTSHRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGETMANTSHRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGETMANTSH_MASKZ");
        }
    }
    /// Emits `VGETMANTSHRRRI_MASKZ_SAE`.
    fn vgetmantsh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGETMANTSHRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGETMANTSHRRRI_SAE`.
    fn vgetmantsh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGETMANTSHRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB128`.
    fn vgf2p8affineinvqb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB128");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB128_MASK`.
    fn vgf2p8affineinvqb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB128_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB128_MASKZ`.
    fn vgf2p8affineinvqb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB128_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB128RRBI`.
    fn vgf2p8affineinvqb128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB128RRBI_MASK`.
    fn vgf2p8affineinvqb128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB128RRBI_MASKZ`.
    fn vgf2p8affineinvqb128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB256`.
    fn vgf2p8affineinvqb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB256");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB256_MASK`.
    fn vgf2p8affineinvqb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB256_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB256_MASKZ`.
    fn vgf2p8affineinvqb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB256_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB256RRBI`.
    fn vgf2p8affineinvqb256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB256RRBI_MASK`.
    fn vgf2p8affineinvqb256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB256RRBI_MASKZ`.
    fn vgf2p8affineinvqb256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB512`.
    fn vgf2p8affineinvqb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB512");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB512_MASK`.
    fn vgf2p8affineinvqb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB512_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB512_MASKZ`.
    fn vgf2p8affineinvqb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEINVQB512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEINVQB512_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEINVQB512RRBI`.
    fn vgf2p8affineinvqb512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB512RRBI_MASK`.
    fn vgf2p8affineinvqb512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEINVQB512RRBI_MASKZ`.
    fn vgf2p8affineinvqb512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEINVQB512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB128`.
    fn vgf2p8affineqb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB128");
        }
    }
    /// Emits `VGF2P8AFFINEQB128_MASK`.
    fn vgf2p8affineqb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB128_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEQB128_MASKZ`.
    fn vgf2p8affineqb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB128_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEQB128RRBI`.
    fn vgf2p8affineqb128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB128RRBI_MASK`.
    fn vgf2p8affineqb128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB128RRBI_MASKZ`.
    fn vgf2p8affineqb128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB256`.
    fn vgf2p8affineqb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB256");
        }
    }
    /// Emits `VGF2P8AFFINEQB256_MASK`.
    fn vgf2p8affineqb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB256_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEQB256_MASKZ`.
    fn vgf2p8affineqb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB256_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEQB256RRBI`.
    fn vgf2p8affineqb256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB256RRBI_MASK`.
    fn vgf2p8affineqb256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB256RRBI_MASKZ`.
    fn vgf2p8affineqb256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB512`.
    fn vgf2p8affineqb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB512");
        }
    }
    /// Emits `VGF2P8AFFINEQB512_MASK`.
    fn vgf2p8affineqb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB512_MASK");
        }
    }
    /// Emits `VGF2P8AFFINEQB512_MASKZ`.
    fn vgf2p8affineqb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VGF2P8AFFINEQB512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VGF2P8AFFINEQB512_MASKZ");
        }
    }
    /// Emits `VGF2P8AFFINEQB512RRBI`.
    fn vgf2p8affineqb512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB512RRBI_MASK`.
    fn vgf2p8affineqb512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8AFFINEQB512RRBI_MASKZ`.
    fn vgf2p8affineqb512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VGF2P8AFFINEQB512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VGF2P8MULB128`.
    fn vgf2p8mulb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB128");
        }
    }
    /// Emits `VGF2P8MULB128_MASK`.
    fn vgf2p8mulb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB128_MASK");
        }
    }
    /// Emits `VGF2P8MULB128_MASKZ`.
    fn vgf2p8mulb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB128_MASKZ");
        }
    }
    /// Emits `VGF2P8MULB256`.
    fn vgf2p8mulb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB256");
        }
    }
    /// Emits `VGF2P8MULB256_MASK`.
    fn vgf2p8mulb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB256_MASK");
        }
    }
    /// Emits `VGF2P8MULB256_MASKZ`.
    fn vgf2p8mulb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB256_MASKZ");
        }
    }
    /// Emits `VGF2P8MULB512`.
    fn vgf2p8mulb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB512");
        }
    }
    /// Emits `VGF2P8MULB512_MASK`.
    fn vgf2p8mulb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB512_MASK");
        }
    }
    /// Emits `VGF2P8MULB512_MASKZ`.
    fn vgf2p8mulb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VGF2P8MULB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VGF2P8MULB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VGF2P8MULB512_MASKZ");
        }
    }
    /// Emits `VMAXPH128`.
    fn vmaxph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH128");
        }
    }
    /// Emits `VMAXPH128_MASK`.
    fn vmaxph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH128_MASK");
        }
    }
    /// Emits `VMAXPH128_MASKZ`.
    fn vmaxph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH128_MASKZ");
        }
    }
    /// Emits `VMAXPH128RRB`.
    fn vmaxph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH128RRB_MASK`.
    fn vmaxph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH128RRB_MASKZ`.
    fn vmaxph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH256`.
    fn vmaxph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH256");
        }
    }
    /// Emits `VMAXPH256_MASK`.
    fn vmaxph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH256_MASK");
        }
    }
    /// Emits `VMAXPH256_MASKZ`.
    fn vmaxph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH256_MASKZ");
        }
    }
    /// Emits `VMAXPH256RRB`.
    fn vmaxph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH256RRB_MASK`.
    fn vmaxph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH256RRB_MASKZ`.
    fn vmaxph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512`.
    fn vmaxph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH512");
        }
    }
    /// Emits `VMAXPH512_MASK`.
    fn vmaxph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH512_MASK");
        }
    }
    /// Emits `VMAXPH512RRR_MASK_SAE`.
    fn vmaxph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512_MASKZ`.
    fn vmaxph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXPH512_MASKZ");
        }
    }
    /// Emits `VMAXPH512RRR_MASKZ_SAE`.
    fn vmaxph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512RRR_SAE`.
    fn vmaxph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512RRB`.
    fn vmaxph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512RRB_MASK`.
    fn vmaxph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXPH512RRB_MASKZ`.
    fn vmaxph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXSH`.
    fn vmaxsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXSH");
        }
    }
    /// Emits `VMAXSH_MASK`.
    fn vmaxsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXSH_MASK");
        }
    }
    /// Emits `VMAXSHRRR_MASK_SAE`.
    fn vmaxsh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXSHRRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXSH_MASKZ`.
    fn vmaxsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMAXSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMAXSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMAXSH_MASKZ");
        }
    }
    /// Emits `VMAXSHRRR_MASKZ_SAE`.
    fn vmaxsh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXSHRRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMAXSHRRR_SAE`.
    fn vmaxsh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMAXSHRRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH128`.
    fn vminph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH128");
        }
    }
    /// Emits `VMINPH128_MASK`.
    fn vminph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH128_MASK");
        }
    }
    /// Emits `VMINPH128_MASKZ`.
    fn vminph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH128_MASKZ");
        }
    }
    /// Emits `VMINPH128RRB`.
    fn vminph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH128RRB_MASK`.
    fn vminph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH128RRB_MASKZ`.
    fn vminph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH256`.
    fn vminph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH256");
        }
    }
    /// Emits `VMINPH256_MASK`.
    fn vminph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH256_MASK");
        }
    }
    /// Emits `VMINPH256_MASKZ`.
    fn vminph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH256_MASKZ");
        }
    }
    /// Emits `VMINPH256RRB`.
    fn vminph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH256RRB_MASK`.
    fn vminph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH256RRB_MASKZ`.
    fn vminph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512`.
    fn vminph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH512");
        }
    }
    /// Emits `VMINPH512_MASK`.
    fn vminph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH512_MASK");
        }
    }
    /// Emits `VMINPH512RRR_MASK_SAE`.
    fn vminph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512_MASKZ`.
    fn vminph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINPH512_MASKZ");
        }
    }
    /// Emits `VMINPH512RRR_MASKZ_SAE`.
    fn vminph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512RRR_SAE`.
    fn vminph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512RRB`.
    fn vminph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512RRB_MASK`.
    fn vminph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINPH512RRB_MASKZ`.
    fn vminph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINSH`.
    fn vminsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINSH");
        }
    }
    /// Emits `VMINSH_MASK`.
    fn vminsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINSH_MASK");
        }
    }
    /// Emits `VMINSHRRR_MASK_SAE`.
    fn vminsh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINSHRRR_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINSH_MASKZ`.
    fn vminsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMINSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMINSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMINSH_MASKZ");
        }
    }
    /// Emits `VMINSHRRR_MASKZ_SAE`.
    fn vminsh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINSHRRR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMINSHRRR_SAE`.
    fn vminsh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMINSHRRR_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMOVSH`.
    fn vmovsh(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVSHRM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVSHMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVSH");
        }
    }
    /// Emits `VMOVSHRRR`.
    fn vmovsh_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMOVSHRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMOVSH_MASK`.
    fn vmovsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVSHRM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVSHMR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVSH_MASK");
        }
    }
    /// Emits `VMOVSHRRR_MASK`.
    fn vmovsh_mask_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMOVSHRRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMOVSHRM_MASKZ`.
    fn vmovsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVSHRM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMOVSHRRR_MASKZ`.
    fn vmovsh_maskz_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMOVSHRRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMOVW_G2X`.
    fn vmovw_g2x(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_gp() {
            self.emit(VMOVW_G2XRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVW_G2XRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVW_G2X");
        }
    }
    /// Emits `VMOVW_X2G`.
    fn vmovw_x2g(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_vec() {
            self.emit(VMOVW_X2GRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVW_X2GMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVW_X2G");
        }
    }
    /// Emits `VMULPH128`.
    fn vmulph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH128");
        }
    }
    /// Emits `VMULPH128_MASK`.
    fn vmulph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH128_MASK");
        }
    }
    /// Emits `VMULPH128_MASKZ`.
    fn vmulph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH128_MASKZ");
        }
    }
    /// Emits `VMULPH128RRB`.
    fn vmulph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH128RRB_MASK`.
    fn vmulph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH128RRB_MASKZ`.
    fn vmulph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH256`.
    fn vmulph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH256");
        }
    }
    /// Emits `VMULPH256_MASK`.
    fn vmulph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH256_MASK");
        }
    }
    /// Emits `VMULPH256_MASKZ`.
    fn vmulph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH256_MASKZ");
        }
    }
    /// Emits `VMULPH256RRB`.
    fn vmulph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH256RRB_MASK`.
    fn vmulph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH256RRB_MASKZ`.
    fn vmulph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512`.
    fn vmulph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH512");
        }
    }
    /// Emits `VMULPH512RRR_ER`.
    fn vmulph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512_MASK`.
    fn vmulph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH512_MASK");
        }
    }
    /// Emits `VMULPH512RRR_MASK_ER`.
    fn vmulph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512_MASKZ`.
    fn vmulph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULPH512_MASKZ");
        }
    }
    /// Emits `VMULPH512RRR_MASKZ_ER`.
    fn vmulph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512RRB`.
    fn vmulph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512RRB_MASK`.
    fn vmulph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULPH512RRB_MASKZ`.
    fn vmulph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULSH`.
    fn vmulsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULSH");
        }
    }
    /// Emits `VMULSHRRR_ER`.
    fn vmulsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULSH_MASK`.
    fn vmulsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULSH_MASK");
        }
    }
    /// Emits `VMULSHRRR_MASK_ER`.
    fn vmulsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VMULSH_MASKZ`.
    fn vmulsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VMULSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMULSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMULSH_MASKZ");
        }
    }
    /// Emits `VMULSHRRR_MASKZ_ER`.
    fn vmulsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VMULSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPCLMULQDQ128`.
    fn vpclmulqdq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCLMULQDQ128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCLMULQDQ128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCLMULQDQ128");
        }
    }
    /// Emits `VPCLMULQDQ256`.
    fn vpclmulqdq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCLMULQDQ256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCLMULQDQ256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCLMULQDQ256");
        }
    }
    /// Emits `VPCLMULQDQ512`.
    fn vpclmulqdq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCLMULQDQ512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCLMULQDQ512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCLMULQDQ512");
        }
    }
    /// Emits `VPDPBSSD128`.
    fn vpdpbssd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSSD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSSD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSSD128");
        }
    }
    /// Emits `VPDPBSSD256`.
    fn vpdpbssd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSSD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSSD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSSD256");
        }
    }
    /// Emits `VPDPBSSDS128`.
    fn vpdpbssds128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSSDS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSSDS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSSDS128");
        }
    }
    /// Emits `VPDPBSSDS256`.
    fn vpdpbssds256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSSDS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSSDS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSSDS256");
        }
    }
    /// Emits `VPDPBSUD128`.
    fn vpdpbsud128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSUD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSUD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSUD128");
        }
    }
    /// Emits `VPDPBSUD256`.
    fn vpdpbsud256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSUD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSUD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSUD256");
        }
    }
    /// Emits `VPDPBSUDS128`.
    fn vpdpbsuds128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSUDS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSUDS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSUDS128");
        }
    }
    /// Emits `VPDPBSUDS256`.
    fn vpdpbsuds256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBSUDS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBSUDS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBSUDS256");
        }
    }
    /// Emits `VPDPBUUD128`.
    fn vpdpbuud128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBUUD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBUUD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBUUD128");
        }
    }
    /// Emits `VPDPBUUD256`.
    fn vpdpbuud256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBUUD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBUUD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBUUD256");
        }
    }
    /// Emits `VPDPBUUDS128`.
    fn vpdpbuuds128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBUUDS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBUUDS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBUUDS128");
        }
    }
    /// Emits `VPDPBUUDS256`.
    fn vpdpbuuds256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPDPBUUDS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPDPBUUDS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPDPBUUDS256");
        }
    }
    /// Emits `VRCPPH128`.
    fn vrcpph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH128");
        }
    }
    /// Emits `VRCPPH128_MASK`.
    fn vrcpph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH128_MASK");
        }
    }
    /// Emits `VRCPPH128_MASKZ`.
    fn vrcpph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH128_MASKZ");
        }
    }
    /// Emits `VRCPPH128RB`.
    fn vrcpph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH128RB_MASK`.
    fn vrcpph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH128RB_MASKZ`.
    fn vrcpph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH256`.
    fn vrcpph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH256");
        }
    }
    /// Emits `VRCPPH256_MASK`.
    fn vrcpph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH256_MASK");
        }
    }
    /// Emits `VRCPPH256_MASKZ`.
    fn vrcpph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH256_MASKZ");
        }
    }
    /// Emits `VRCPPH256RB`.
    fn vrcpph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH256RB_MASK`.
    fn vrcpph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH256RB_MASKZ`.
    fn vrcpph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH512`.
    fn vrcpph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH512");
        }
    }
    /// Emits `VRCPPH512_MASK`.
    fn vrcpph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH512_MASK");
        }
    }
    /// Emits `VRCPPH512_MASKZ`.
    fn vrcpph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPH512_MASKZ");
        }
    }
    /// Emits `VRCPPH512RB`.
    fn vrcpph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH512RB_MASK`.
    fn vrcpph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPPH512RB_MASKZ`.
    fn vrcpph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRCPSH`.
    fn vrcpsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRCPSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRCPSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPSH");
        }
    }
    /// Emits `VRCPSH_MASK`.
    fn vrcpsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRCPSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRCPSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPSH_MASK");
        }
    }
    /// Emits `VRCPSH_MASKZ`.
    fn vrcpsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRCPSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRCPSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPSH_MASKZ");
        }
    }
    /// Emits `VREDUCEPH128`.
    fn vreduceph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH128");
        }
    }
    /// Emits `VREDUCEPH128_MASK`.
    fn vreduceph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH128_MASK");
        }
    }
    /// Emits `VREDUCEPH128_MASKZ`.
    fn vreduceph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH128_MASKZ");
        }
    }
    /// Emits `VREDUCEPH128RBI`.
    fn vreduceph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH128RBI_MASK`.
    fn vreduceph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH128RBI_MASKZ`.
    fn vreduceph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH256`.
    fn vreduceph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH256");
        }
    }
    /// Emits `VREDUCEPH256_MASK`.
    fn vreduceph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH256_MASK");
        }
    }
    /// Emits `VREDUCEPH256_MASKZ`.
    fn vreduceph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH256_MASKZ");
        }
    }
    /// Emits `VREDUCEPH256RBI`.
    fn vreduceph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH256RBI_MASK`.
    fn vreduceph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH256RBI_MASKZ`.
    fn vreduceph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512`.
    fn vreduceph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH512");
        }
    }
    /// Emits `VREDUCEPH512_MASK`.
    fn vreduceph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH512_MASK");
        }
    }
    /// Emits `VREDUCEPH512RRI_MASK_SAE`.
    fn vreduceph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512_MASKZ`.
    fn vreduceph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPH512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPH512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VREDUCEPH512_MASKZ");
        }
    }
    /// Emits `VREDUCEPH512RRI_MASKZ_SAE`.
    fn vreduceph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512RRI_SAE`.
    fn vreduceph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512RBI`.
    fn vreduceph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512RBI_MASK`.
    fn vreduceph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCEPH512RBI_MASKZ`.
    fn vreduceph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPH512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VREDUCESH`.
    fn vreducesh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESHRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESHRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VREDUCESH");
        }
    }
    /// Emits `VREDUCESH_MASK`.
    fn vreducesh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESHRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESHRRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VREDUCESH_MASK");
        }
    }
    /// Emits `VREDUCESHRRRI_MASK_SAE`.
    fn vreducesh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESHRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VREDUCESH_MASKZ`.
    fn vreducesh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESHRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESHRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VREDUCESH_MASKZ");
        }
    }
    /// Emits `VREDUCESHRRRI_MASKZ_SAE`.
    fn vreducesh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESHRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VREDUCESHRRRI_SAE`.
    fn vreducesh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESHRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VRNDSCALEPH128`.
    fn vrndscaleph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH128");
        }
    }
    /// Emits `VRNDSCALEPH128_MASK`.
    fn vrndscaleph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH128_MASK");
        }
    }
    /// Emits `VRNDSCALEPH128_MASKZ`.
    fn vrndscaleph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH128_MASKZ");
        }
    }
    /// Emits `VRNDSCALEPH128RBI`.
    fn vrndscaleph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH128RBI_MASK`.
    fn vrndscaleph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH128RBI_MASKZ`.
    fn vrndscaleph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH256`.
    fn vrndscaleph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH256");
        }
    }
    /// Emits `VRNDSCALEPH256_MASK`.
    fn vrndscaleph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH256_MASK");
        }
    }
    /// Emits `VRNDSCALEPH256_MASKZ`.
    fn vrndscaleph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH256_MASKZ");
        }
    }
    /// Emits `VRNDSCALEPH256RBI`.
    fn vrndscaleph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH256RBI_MASK`.
    fn vrndscaleph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH256RBI_MASKZ`.
    fn vrndscaleph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512`.
    fn vrndscaleph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH512");
        }
    }
    /// Emits `VRNDSCALEPH512_MASK`.
    fn vrndscaleph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH512_MASK");
        }
    }
    /// Emits `VRNDSCALEPH512RRI_MASK_SAE`.
    fn vrndscaleph512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512_MASKZ`.
    fn vrndscaleph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VRNDSCALEPH512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRNDSCALEPH512_MASKZ");
        }
    }
    /// Emits `VRNDSCALEPH512RRI_MASKZ_SAE`.
    fn vrndscaleph512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512RRI_SAE`.
    fn vrndscaleph512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512RBI`.
    fn vrndscaleph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512RBI_MASK`.
    fn vrndscaleph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALEPH512RBI_MASKZ`.
    fn vrndscaleph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRNDSCALEPH512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VRNDSCALESH`.
    fn vrndscalesh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRNDSCALESHRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRNDSCALESHRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VRNDSCALESH");
        }
    }
    /// Emits `VRNDSCALESH_MASK`.
    fn vrndscalesh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRNDSCALESHRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRNDSCALESHRRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VRNDSCALESH_MASK");
        }
    }
    /// Emits `VRNDSCALESHRRRI_MASK_SAE`.
    fn vrndscalesh_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRNDSCALESHRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VRNDSCALESH_MASKZ`.
    fn vrndscalesh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRNDSCALESHRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRNDSCALESHRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VRNDSCALESH_MASKZ");
        }
    }
    /// Emits `VRNDSCALESHRRRI_MASKZ_SAE`.
    fn vrndscalesh_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRNDSCALESHRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VRNDSCALESHRRRI_SAE`.
    fn vrndscalesh_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRNDSCALESHRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VRSQRTPH128`.
    fn vrsqrtph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH128");
        }
    }
    /// Emits `VRSQRTPH128_MASK`.
    fn vrsqrtph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH128_MASK");
        }
    }
    /// Emits `VRSQRTPH128_MASKZ`.
    fn vrsqrtph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH128_MASKZ");
        }
    }
    /// Emits `VRSQRTPH128RB`.
    fn vrsqrtph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH128RB_MASK`.
    fn vrsqrtph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH128RB_MASKZ`.
    fn vrsqrtph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH256`.
    fn vrsqrtph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH256");
        }
    }
    /// Emits `VRSQRTPH256_MASK`.
    fn vrsqrtph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH256_MASK");
        }
    }
    /// Emits `VRSQRTPH256_MASKZ`.
    fn vrsqrtph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH256_MASKZ");
        }
    }
    /// Emits `VRSQRTPH256RB`.
    fn vrsqrtph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH256RB_MASK`.
    fn vrsqrtph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH256RB_MASKZ`.
    fn vrsqrtph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH512`.
    fn vrsqrtph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH512");
        }
    }
    /// Emits `VRSQRTPH512_MASK`.
    fn vrsqrtph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH512_MASK");
        }
    }
    /// Emits `VRSQRTPH512_MASKZ`.
    fn vrsqrtph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPH512_MASKZ");
        }
    }
    /// Emits `VRSQRTPH512RB`.
    fn vrsqrtph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH512RB_MASK`.
    fn vrsqrtph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTPH512RB_MASKZ`.
    fn vrsqrtph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VRSQRTSH`.
    fn vrsqrtsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRSQRTSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRSQRTSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTSH");
        }
    }
    /// Emits `VRSQRTSH_MASK`.
    fn vrsqrtsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRSQRTSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRSQRTSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTSH_MASK");
        }
    }
    /// Emits `VRSQRTSH_MASKZ`.
    fn vrsqrtsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRSQRTSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRSQRTSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTSH_MASKZ");
        }
    }
    /// Emits `VSCALEFPH128`.
    fn vscalefph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH128");
        }
    }
    /// Emits `VSCALEFPH128_MASK`.
    fn vscalefph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH128_MASK");
        }
    }
    /// Emits `VSCALEFPH128_MASKZ`.
    fn vscalefph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH128_MASKZ");
        }
    }
    /// Emits `VSCALEFPH128RRB`.
    fn vscalefph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH128RRB_MASK`.
    fn vscalefph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH128RRB_MASKZ`.
    fn vscalefph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH256`.
    fn vscalefph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH256");
        }
    }
    /// Emits `VSCALEFPH256_MASK`.
    fn vscalefph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH256_MASK");
        }
    }
    /// Emits `VSCALEFPH256_MASKZ`.
    fn vscalefph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH256_MASKZ");
        }
    }
    /// Emits `VSCALEFPH256RRB`.
    fn vscalefph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH256RRB_MASK`.
    fn vscalefph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH256RRB_MASKZ`.
    fn vscalefph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512`.
    fn vscalefph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH512");
        }
    }
    /// Emits `VSCALEFPH512RRR_ER`.
    fn vscalefph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512_MASK`.
    fn vscalefph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH512_MASK");
        }
    }
    /// Emits `VSCALEFPH512RRR_MASK_ER`.
    fn vscalefph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512_MASKZ`.
    fn vscalefph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFPH512_MASKZ");
        }
    }
    /// Emits `VSCALEFPH512RRR_MASKZ_ER`.
    fn vscalefph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512RRB`.
    fn vscalefph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512RRB_MASK`.
    fn vscalefph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFPH512RRB_MASKZ`.
    fn vscalefph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFSH`.
    fn vscalefsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFSH");
        }
    }
    /// Emits `VSCALEFSHRRR_ER`.
    fn vscalefsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFSH_MASK`.
    fn vscalefsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFSH_MASK");
        }
    }
    /// Emits `VSCALEFSHRRR_MASK_ER`.
    fn vscalefsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSCALEFSH_MASKZ`.
    fn vscalefsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSCALEFSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSCALEFSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSCALEFSH_MASKZ");
        }
    }
    /// Emits `VSCALEFSHRRR_MASKZ_ER`.
    fn vscalefsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSCALEFSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSM4KEY4_128`.
    fn vsm4key4_128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4KEY4_128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4KEY4_128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4KEY4_128");
        }
    }
    /// Emits `VSM4KEY4_256`.
    fn vsm4key4_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4KEY4_256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4KEY4_256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4KEY4_256");
        }
    }
    /// Emits `VSM4KEY4_512`.
    fn vsm4key4_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4KEY4_512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4KEY4_512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4KEY4_512");
        }
    }
    /// Emits `VSM4RNDS4_128`.
    fn vsm4rnds4_128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4RNDS4_128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4RNDS4_128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4RNDS4_128");
        }
    }
    /// Emits `VSM4RNDS4_256`.
    fn vsm4rnds4_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4RNDS4_256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4RNDS4_256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4RNDS4_256");
        }
    }
    /// Emits `VSM4RNDS4_512`.
    fn vsm4rnds4_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSM4RNDS4_512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSM4RNDS4_512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSM4RNDS4_512");
        }
    }
    /// Emits `VSQRTPH128`.
    fn vsqrtph128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH128");
        }
    }
    /// Emits `VSQRTPH128_MASK`.
    fn vsqrtph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH128_MASK");
        }
    }
    /// Emits `VSQRTPH128_MASKZ`.
    fn vsqrtph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH128_MASKZ");
        }
    }
    /// Emits `VSQRTPH128RB`.
    fn vsqrtph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH128RB_MASK`.
    fn vsqrtph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH128RB_MASKZ`.
    fn vsqrtph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH256`.
    fn vsqrtph256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH256");
        }
    }
    /// Emits `VSQRTPH256_MASK`.
    fn vsqrtph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH256_MASK");
        }
    }
    /// Emits `VSQRTPH256_MASKZ`.
    fn vsqrtph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH256_MASKZ");
        }
    }
    /// Emits `VSQRTPH256RB`.
    fn vsqrtph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH256RB_MASK`.
    fn vsqrtph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH256RB_MASKZ`.
    fn vsqrtph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512`.
    fn vsqrtph512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH512");
        }
    }
    /// Emits `VSQRTPH512RR_ER`.
    fn vsqrtph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512_MASK`.
    fn vsqrtph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH512_MASK");
        }
    }
    /// Emits `VSQRTPH512RR_MASK_ER`.
    fn vsqrtph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512_MASKZ`.
    fn vsqrtph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VSQRTPH512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VSQRTPH512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTPH512_MASKZ");
        }
    }
    /// Emits `VSQRTPH512RR_MASKZ_ER`.
    fn vsqrtph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512RB`.
    fn vsqrtph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512RB_MASK`.
    fn vsqrtph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTPH512RB_MASKZ`.
    fn vsqrtph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VSQRTPH512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VSQRTSH`.
    fn vsqrtsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSQRTSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSQRTSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTSH");
        }
    }
    /// Emits `VSQRTSHRRR_ER`.
    fn vsqrtsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSQRTSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSQRTSH_MASK`.
    fn vsqrtsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSQRTSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSQRTSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTSH_MASK");
        }
    }
    /// Emits `VSQRTSHRRR_MASK_ER`.
    fn vsqrtsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSQRTSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSQRTSH_MASKZ`.
    fn vsqrtsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSQRTSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSQRTSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSQRTSH_MASKZ");
        }
    }
    /// Emits `VSQRTSHRRR_MASKZ_ER`.
    fn vsqrtsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSQRTSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH128`.
    fn vsubph128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH128");
        }
    }
    /// Emits `VSUBPH128_MASK`.
    fn vsubph128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH128_MASK");
        }
    }
    /// Emits `VSUBPH128_MASKZ`.
    fn vsubph128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH128_MASKZ");
        }
    }
    /// Emits `VSUBPH128RRB`.
    fn vsubph128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH128RRB_MASK`.
    fn vsubph128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH128RRB_MASKZ`.
    fn vsubph128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH256`.
    fn vsubph256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH256");
        }
    }
    /// Emits `VSUBPH256_MASK`.
    fn vsubph256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH256_MASK");
        }
    }
    /// Emits `VSUBPH256_MASKZ`.
    fn vsubph256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH256_MASKZ");
        }
    }
    /// Emits `VSUBPH256RRB`.
    fn vsubph256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH256RRB_MASK`.
    fn vsubph256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH256RRB_MASKZ`.
    fn vsubph256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512`.
    fn vsubph512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH512");
        }
    }
    /// Emits `VSUBPH512RRR_ER`.
    fn vsubph512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512_MASK`.
    fn vsubph512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH512_MASK");
        }
    }
    /// Emits `VSUBPH512RRR_MASK_ER`.
    fn vsubph512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512_MASKZ`.
    fn vsubph512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBPH512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBPH512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBPH512_MASKZ");
        }
    }
    /// Emits `VSUBPH512RRR_MASKZ_ER`.
    fn vsubph512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512RRB`.
    fn vsubph512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512RRB_MASK`.
    fn vsubph512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBPH512RRB_MASKZ`.
    fn vsubph512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBPH512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBSH`.
    fn vsubsh(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBSHRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBSHRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBSH");
        }
    }
    /// Emits `VSUBSHRRR_ER`.
    fn vsubsh_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBSHRRR_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBSH_MASK`.
    fn vsubsh_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBSHRRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBSHRRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBSH_MASK");
        }
    }
    /// Emits `VSUBSHRRR_MASK_ER`.
    fn vsubsh_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBSHRRR_MASK_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VSUBSH_MASKZ`.
    fn vsubsh_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VSUBSHRRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VSUBSHRRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VSUBSH_MASKZ");
        }
    }
    /// Emits `VSUBSHRRR_MASKZ_ER`.
    fn vsubsh_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VSUBSHRRR_MASKZ_ER, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VUCOMISH`.
    fn vucomish(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VUCOMISHRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VUCOMISHRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VUCOMISH");
        }
    }
    /// Emits `VUCOMISHRR_SAE`.
    fn vucomish_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VUCOMISHRR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `XCHG16`.
    fn xchg16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XCHG16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XCHG16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XCHG16");
        }
    }
    /// Emits `XCHG32`.
    fn xchg32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XCHG32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XCHG32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XCHG32");
        }
    }
    /// Emits `XCHG64`.
    fn xchg64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XCHG64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XCHG64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XCHG64");
        }
    }
    /// Emits `XCHG8`.
    fn xchg8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XCHG8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XCHG8MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XCHG8");
        }
    }
    /// Emits `XLATB`.
    fn xlatb(&mut self,) -> () {
        self.emit(XLATB, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XOR16`.
    fn xor16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XOR16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XOR16MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(XOR16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(XOR16RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(XOR16MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XOR16");
        }
    }
    /// Emits `XOR32`.
    fn xor32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XOR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XOR32MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(XOR32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(XOR32RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(XOR32MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XOR32");
        }
    }
    /// Emits `XOR64`.
    fn xor64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XOR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XOR64MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(XOR64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(XOR64RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(XOR64MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XOR64");
        }
    }
    /// Emits `XOR8`.
    fn xor8(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(XOR8RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(XOR8MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(XOR8RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(XOR8RI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_imm() {
            self.emit(XOR8MI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for XOR8");
        }
    }
}
