pub trait X86AVX512DQEmitter: Emitter {
    /// Emits `KADDBKKK`.
    fn kaddbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KADDBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KADDWKKK`.
    fn kaddwkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KADDWKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KANDBKKK`.
    fn kandbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KANDBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KANDNBKKK`.
    fn kandnbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KANDNBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KMOVBK`.
    fn kmovbk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mask() && op1.is_mem() {
            self.emit(KMOVBKM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_mask() {
            self.emit(KMOVBMK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mask() && op1.is_gp() {
            self.emit(KMOVBKR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mask() {
            self.emit(KMOVBRK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "KMOVBK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KMOVBKK`.
    fn kmovbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KMOVBKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KNOTBKK`.
    fn knotbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KNOTBKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KORBKKK`.
    fn korbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KORTESTBKK`.
    fn kortestbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KORTESTBKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KSHIFTLBKKI`.
    fn kshiftlbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KSHIFTLBKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KSHIFTRBKKI`.
    fn kshiftrbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KSHIFTRBKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KTESTBKK`.
    fn ktestbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KTESTBKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KTESTWKK`.
    fn ktestwkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KTESTWKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KXNORBKKK`.
    fn kxnorbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KXNORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `KXORBKKK`.
    fn kxorbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(KXORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128`.
    fn vandnpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128_MASK`.
    fn vandnpd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128_MASKZ`.
    fn vandnpd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128RRB`.
    fn vandnpd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128RRB_MASK`.
    fn vandnpd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD128RRB_MASKZ`.
    fn vandnpd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256`.
    fn vandnpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256_MASK`.
    fn vandnpd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256_MASKZ`.
    fn vandnpd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256RRB`.
    fn vandnpd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256RRB_MASK`.
    fn vandnpd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD256RRB_MASKZ`.
    fn vandnpd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512`.
    fn vandnpd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512_MASK`.
    fn vandnpd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512_MASKZ`.
    fn vandnpd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512RRB`.
    fn vandnpd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512RRB_MASK`.
    fn vandnpd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPD512RRB_MASKZ`.
    fn vandnpd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128`.
    fn vandnps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128_MASK`.
    fn vandnps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128_MASKZ`.
    fn vandnps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128RRB`.
    fn vandnps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128RRB_MASK`.
    fn vandnps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS128RRB_MASKZ`.
    fn vandnps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256`.
    fn vandnps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256_MASK`.
    fn vandnps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256_MASKZ`.
    fn vandnps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256RRB`.
    fn vandnps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256RRB_MASK`.
    fn vandnps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS256RRB_MASKZ`.
    fn vandnps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512`.
    fn vandnps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512_MASK`.
    fn vandnps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512_MASKZ`.
    fn vandnps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDNPS512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDNPS512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDNPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512RRB`.
    fn vandnps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512RRB_MASK`.
    fn vandnps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDNPS512RRB_MASKZ`.
    fn vandnps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDNPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128`.
    fn vandpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128_MASK`.
    fn vandpd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128_MASKZ`.
    fn vandpd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128RRB`.
    fn vandpd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128RRB_MASK`.
    fn vandpd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD128RRB_MASKZ`.
    fn vandpd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256`.
    fn vandpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256_MASK`.
    fn vandpd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256_MASKZ`.
    fn vandpd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256RRB`.
    fn vandpd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256RRB_MASK`.
    fn vandpd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD256RRB_MASKZ`.
    fn vandpd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512`.
    fn vandpd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512_MASK`.
    fn vandpd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512_MASKZ`.
    fn vandpd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512RRB`.
    fn vandpd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512RRB_MASK`.
    fn vandpd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPD512RRB_MASKZ`.
    fn vandpd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128`.
    fn vandps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128_MASK`.
    fn vandps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128_MASKZ`.
    fn vandps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128RRB`.
    fn vandps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128RRB_MASK`.
    fn vandps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS128RRB_MASKZ`.
    fn vandps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256`.
    fn vandps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256_MASK`.
    fn vandps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256_MASKZ`.
    fn vandps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256RRB`.
    fn vandps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256RRB_MASK`.
    fn vandps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS256RRB_MASKZ`.
    fn vandps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512`.
    fn vandps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512_MASK`.
    fn vandps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512_MASKZ`.
    fn vandps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VANDPS512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VANDPS512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VANDPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512RRB`.
    fn vandps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512RRB_MASK`.
    fn vandps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VANDPS512RRB_MASKZ`.
    fn vandps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VANDPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_256`.
    fn vbroadcastf32x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_256_MASK`.
    fn vbroadcastf32x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_256_MASKZ`.
    fn vbroadcastf32x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_512`.
    fn vbroadcastf32x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_512_MASK`.
    fn vbroadcastf32x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X2_512_MASKZ`.
    fn vbroadcastf32x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF32X2_512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF32X2_512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTF32X2_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X8_512RM`.
    fn vbroadcastf32x8_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF32X8_512RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X8_512RM_MASK`.
    fn vbroadcastf32x8_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF32X8_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF32X8_512RM_MASKZ`.
    fn vbroadcastf32x8_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF32X8_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_256RM`.
    fn vbroadcastf64x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_256RM_MASK`.
    fn vbroadcastf64x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_256RM_MASKZ`.
    fn vbroadcastf64x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_512RM`.
    fn vbroadcastf64x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_512RM_MASK`.
    fn vbroadcastf64x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTF64X2_512RM_MASKZ`.
    fn vbroadcastf64x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTF64X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_128`.
    fn vbroadcasti32x2_128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_128_MASK`.
    fn vbroadcasti32x2_128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_128_MASKZ`.
    fn vbroadcasti32x2_128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_256`.
    fn vbroadcasti32x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_256_MASK`.
    fn vbroadcasti32x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_256_MASKZ`.
    fn vbroadcasti32x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_512`.
    fn vbroadcasti32x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_512_MASK`.
    fn vbroadcasti32x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X2_512_MASKZ`.
    fn vbroadcasti32x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTI32X2_512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTI32X2_512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VBROADCASTI32X2_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_256RM`.
    fn vbroadcasti32x4_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_256RM_MASK`.
    fn vbroadcasti32x4_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_256RM_MASKZ`.
    fn vbroadcasti32x4_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_512RM`.
    fn vbroadcasti32x4_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_512RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_512RM_MASK`.
    fn vbroadcasti32x4_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X4_512RM_MASKZ`.
    fn vbroadcasti32x4_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X4_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X8_512RM`.
    fn vbroadcasti32x8_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X8_512RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X8_512RM_MASK`.
    fn vbroadcasti32x8_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X8_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI32X8_512RM_MASKZ`.
    fn vbroadcasti32x8_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI32X8_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_256RM`.
    fn vbroadcasti64x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_256RM_MASK`.
    fn vbroadcasti64x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_256RM_MASKZ`.
    fn vbroadcasti64x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_512RM`.
    fn vbroadcasti64x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_512RM_MASK`.
    fn vbroadcasti64x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VBROADCASTI64X2_512RM_MASKZ`.
    fn vbroadcasti64x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VBROADCASTI64X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128`.
    fn vcvtpd2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128_MASK`.
    fn vcvtpd2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128_MASKZ`.
    fn vcvtpd2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128RB`.
    fn vcvtpd2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128RB_MASK`.
    fn vcvtpd2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ128RB_MASKZ`.
    fn vcvtpd2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256`.
    fn vcvtpd2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256_MASK`.
    fn vcvtpd2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256_MASKZ`.
    fn vcvtpd2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256RB`.
    fn vcvtpd2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256RB_MASK`.
    fn vcvtpd2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ256RB_MASKZ`.
    fn vcvtpd2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512`.
    fn vcvtpd2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RR_ER`.
    fn vcvtpd2qq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512_MASK`.
    fn vcvtpd2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RR_MASK_ER`.
    fn vcvtpd2qq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512_MASKZ`.
    fn vcvtpd2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPD2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPD2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPD2QQ512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RR_MASKZ_ER`.
    fn vcvtpd2qq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RB`.
    fn vcvtpd2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RB_MASK`.
    fn vcvtpd2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPD2QQ512RB_MASKZ`.
    fn vcvtpd2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPD2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128`.
    fn vcvtps2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128_MASK`.
    fn vcvtps2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128_MASKZ`.
    fn vcvtps2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128RB`.
    fn vcvtps2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128RB_MASK`.
    fn vcvtps2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ128RB_MASKZ`.
    fn vcvtps2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256`.
    fn vcvtps2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256_MASK`.
    fn vcvtps2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256_MASKZ`.
    fn vcvtps2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256RB`.
    fn vcvtps2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256RB_MASK`.
    fn vcvtps2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ256RB_MASKZ`.
    fn vcvtps2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512`.
    fn vcvtps2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RR_ER`.
    fn vcvtps2qq512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512_MASK`.
    fn vcvtps2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RR_MASK_ER`.
    fn vcvtps2qq512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512_MASKZ`.
    fn vcvtps2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTPS2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTPS2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTPS2QQ512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RR_MASKZ_ER`.
    fn vcvtps2qq512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RB`.
    fn vcvtps2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RB_MASK`.
    fn vcvtps2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTPS2QQ512RB_MASKZ`.
    fn vcvtps2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTPS2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128`.
    fn vcvtqq2pd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128_MASK`.
    fn vcvtqq2pd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128_MASKZ`.
    fn vcvtqq2pd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128RB`.
    fn vcvtqq2pd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128RB_MASK`.
    fn vcvtqq2pd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD128RB_MASKZ`.
    fn vcvtqq2pd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256`.
    fn vcvtqq2pd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256_MASK`.
    fn vcvtqq2pd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256_MASKZ`.
    fn vcvtqq2pd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256RB`.
    fn vcvtqq2pd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256RB_MASK`.
    fn vcvtqq2pd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD256RB_MASKZ`.
    fn vcvtqq2pd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512`.
    fn vcvtqq2pd512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RR_ER`.
    fn vcvtqq2pd512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512_MASK`.
    fn vcvtqq2pd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RR_MASK_ER`.
    fn vcvtqq2pd512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512_MASKZ`.
    fn vcvtqq2pd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PD512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PD512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RR_MASKZ_ER`.
    fn vcvtqq2pd512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RB`.
    fn vcvtqq2pd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RB_MASK`.
    fn vcvtqq2pd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PD512RB_MASKZ`.
    fn vcvtqq2pd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128`.
    fn vcvtqq2ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128_MASK`.
    fn vcvtqq2ps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128_MASKZ`.
    fn vcvtqq2ps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128RB`.
    fn vcvtqq2ps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128RB_MASK`.
    fn vcvtqq2ps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS128RB_MASKZ`.
    fn vcvtqq2ps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256`.
    fn vcvtqq2ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256_MASK`.
    fn vcvtqq2ps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256_MASKZ`.
    fn vcvtqq2ps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256RB`.
    fn vcvtqq2ps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256RB_MASK`.
    fn vcvtqq2ps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS256RB_MASKZ`.
    fn vcvtqq2ps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512`.
    fn vcvtqq2ps512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RR_ER`.
    fn vcvtqq2ps512_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512_MASK`.
    fn vcvtqq2ps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RR_MASK_ER`.
    fn vcvtqq2ps512_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512_MASKZ`.
    fn vcvtqq2ps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTQQ2PS512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTQQ2PS512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTQQ2PS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RR_MASKZ_ER`.
    fn vcvtqq2ps512_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RB`.
    fn vcvtqq2ps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RB_MASK`.
    fn vcvtqq2ps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTQQ2PS512RB_MASKZ`.
    fn vcvtqq2ps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTQQ2PS512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128`.
    fn vcvttpd2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128_MASK`.
    fn vcvttpd2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128_MASKZ`.
    fn vcvttpd2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128RB`.
    fn vcvttpd2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128RB_MASK`.
    fn vcvttpd2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ128RB_MASKZ`.
    fn vcvttpd2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256`.
    fn vcvttpd2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256_MASK`.
    fn vcvttpd2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256_MASKZ`.
    fn vcvttpd2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256RB`.
    fn vcvttpd2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256RB_MASK`.
    fn vcvttpd2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ256RB_MASKZ`.
    fn vcvttpd2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512`.
    fn vcvttpd2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512_MASK`.
    fn vcvttpd2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RR_MASK_SAE`.
    fn vcvttpd2qq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512_MASKZ`.
    fn vcvttpd2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPD2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPD2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPD2QQ512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RR_MASKZ_SAE`.
    fn vcvttpd2qq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RR_SAE`.
    fn vcvttpd2qq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RB`.
    fn vcvttpd2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RB_MASK`.
    fn vcvttpd2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPD2QQ512RB_MASKZ`.
    fn vcvttpd2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPD2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128`.
    fn vcvttps2qq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128_MASK`.
    fn vcvttps2qq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128_MASKZ`.
    fn vcvttps2qq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128RB`.
    fn vcvttps2qq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128RB_MASK`.
    fn vcvttps2qq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ128RB_MASKZ`.
    fn vcvttps2qq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256`.
    fn vcvttps2qq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256_MASK`.
    fn vcvttps2qq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256_MASKZ`.
    fn vcvttps2qq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256RB`.
    fn vcvttps2qq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256RB_MASK`.
    fn vcvttps2qq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ256RB_MASKZ`.
    fn vcvttps2qq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512`.
    fn vcvttps2qq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512_MASK`.
    fn vcvttps2qq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RR_MASK_SAE`.
    fn vcvttps2qq512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512_MASKZ`.
    fn vcvttps2qq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTTPS2QQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTTPS2QQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VCVTTPS2QQ512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RR_MASKZ_SAE`.
    fn vcvttps2qq512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RR_SAE`.
    fn vcvttps2qq512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RB`.
    fn vcvttps2qq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RB_MASK`.
    fn vcvttps2qq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VCVTTPS2QQ512RB_MASKZ`.
    fn vcvttps2qq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VCVTTPS2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD128K`.
    fn vfpclasspd128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD128KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD128KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD128K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD128K_MASK`.
    fn vfpclasspd128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD128KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD128KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD128K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD128KBI`.
    fn vfpclasspd128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD128KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD128KBI_MASK`.
    fn vfpclasspd128kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD128KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD256K`.
    fn vfpclasspd256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD256KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD256KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD256K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD256K_MASK`.
    fn vfpclasspd256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD256KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD256KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD256K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD256KBI`.
    fn vfpclasspd256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD256KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD256KBI_MASK`.
    fn vfpclasspd256kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD256KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD512K`.
    fn vfpclasspd512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD512KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD512KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD512K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD512K_MASK`.
    fn vfpclasspd512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPD512KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPD512KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPD512K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD512KBI`.
    fn vfpclasspd512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD512KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPD512KBI_MASK`.
    fn vfpclasspd512kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPD512KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS128K`.
    fn vfpclassps128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS128KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS128KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS128K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS128K_MASK`.
    fn vfpclassps128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS128KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS128KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS128K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS128KBI`.
    fn vfpclassps128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS128KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS128KBI_MASK`.
    fn vfpclassps128kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS128KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS256K`.
    fn vfpclassps256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS256KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS256KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS256K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS256K_MASK`.
    fn vfpclassps256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS256KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS256KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS256K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS256KBI`.
    fn vfpclassps256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS256KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS256KBI_MASK`.
    fn vfpclassps256kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS256KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS512K`.
    fn vfpclassps512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS512KRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS512KMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS512K" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS512K_MASK`.
    fn vfpclassps512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSPS512KRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSPS512KMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSPS512K_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS512KBI`.
    fn vfpclassps512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS512KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSPS512KBI_MASK`.
    fn vfpclassps512kb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VFPCLASSPS512KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSSDK`.
    fn vfpclasssdk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSDKRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSDKMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSSDK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSSDK_MASK`.
    fn vfpclasssdk_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSDKRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSDKMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSSDK_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSSSK`.
    fn vfpclassssk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSSKRI, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSSKMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSSSK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VFPCLASSSSK_MASK`.
    fn vfpclassssk_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_imm() {
            self.emit(VFPCLASSSSKRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_mem() && op2.is_imm() {
            self.emit(VFPCLASSSSKMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VFPCLASSSSK_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF32X8_512`.
    fn vinsertf32x8_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF32X8_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF32X8_512_MASK`.
    fn vinsertf32x8_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF32X8_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF32X8_512_MASKZ`.
    fn vinsertf32x8_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF32X8_512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF32X8_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_256`.
    fn vinsertf64x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_256_MASK`.
    fn vinsertf64x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_256_MASKZ`.
    fn vinsertf64x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_512`.
    fn vinsertf64x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_512_MASK`.
    fn vinsertf64x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTF64X2_512_MASKZ`.
    fn vinsertf64x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF64X2_512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTF64X2_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI32X8_512`.
    fn vinserti32x8_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI32X8_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI32X8_512_MASK`.
    fn vinserti32x8_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI32X8_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI32X8_512_MASKZ`.
    fn vinserti32x8_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI32X8_512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI32X8_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_256`.
    fn vinserti64x2_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_256_MASK`.
    fn vinserti64x2_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_256_MASKZ`.
    fn vinserti64x2_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_512`.
    fn vinserti64x2_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_512_MASK`.
    fn vinserti64x2_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VINSERTI64X2_512_MASKZ`.
    fn vinserti64x2_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI64X2_512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VINSERTI64X2_512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128`.
    fn vorpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128_MASK`.
    fn vorpd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128_MASKZ`.
    fn vorpd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128RRB`.
    fn vorpd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128RRB_MASK`.
    fn vorpd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD128RRB_MASKZ`.
    fn vorpd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256`.
    fn vorpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256_MASK`.
    fn vorpd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256_MASKZ`.
    fn vorpd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256RRB`.
    fn vorpd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256RRB_MASK`.
    fn vorpd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD256RRB_MASKZ`.
    fn vorpd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512`.
    fn vorpd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512_MASK`.
    fn vorpd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512_MASKZ`.
    fn vorpd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512RRB`.
    fn vorpd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512RRB_MASK`.
    fn vorpd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPD512RRB_MASKZ`.
    fn vorpd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128`.
    fn vorps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128_MASK`.
    fn vorps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128_MASKZ`.
    fn vorps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128RRB`.
    fn vorps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128RRB_MASK`.
    fn vorps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS128RRB_MASKZ`.
    fn vorps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256`.
    fn vorps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256_MASK`.
    fn vorps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256_MASKZ`.
    fn vorps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256RRB`.
    fn vorps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256RRB_MASK`.
    fn vorps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS256RRB_MASKZ`.
    fn vorps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512`.
    fn vorps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512_MASK`.
    fn vorps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512_MASKZ`.
    fn vorps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VORPS512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VORPS512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VORPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512RRB`.
    fn vorps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512RRB_MASK`.
    fn vorps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VORPS512RRB_MASKZ`.
    fn vorps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VORPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPEXTRD`.
    fn vpextrd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRDRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRDMRI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPEXTRD" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPEXTRQ`.
    fn vpextrq(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRQRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRQMRI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPEXTRQ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPINSRD`.
    fn vpinsrd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() && op3.is_imm() {
            self.emit(VPINSRDRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPINSRDRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPINSRD" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPINSRQ`.
    fn vpinsrq(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() && op3.is_imm() {
            self.emit(VPINSRQRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPINSRQRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPINSRQ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVD2M128KR`.
    fn vpmovd2m128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVD2M128KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVD2M256KR`.
    fn vpmovd2m256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVD2M256KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVD2M512KR`.
    fn vpmovd2m512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVD2M512KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2D128RK`.
    fn vpmovm2d128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2D128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2D256RK`.
    fn vpmovm2d256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2D256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2D512RK`.
    fn vpmovm2d512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2D512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2Q128RK`.
    fn vpmovm2q128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2Q128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2Q256RK`.
    fn vpmovm2q256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2Q256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVM2Q512RK`.
    fn vpmovm2q512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVM2Q512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVQ2M128KR`.
    fn vpmovq2m128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVQ2M128KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVQ2M256KR`.
    fn vpmovq2m256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVQ2M256KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMOVQ2M512KR`.
    fn vpmovq2m512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMOVQ2M512KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128`.
    fn vpmulld128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128_MASK`.
    fn vpmulld128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128_MASKZ`.
    fn vpmulld128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128RRB`.
    fn vpmulld128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128RRB_MASK`.
    fn vpmulld128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD128RRB_MASKZ`.
    fn vpmulld128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256`.
    fn vpmulld256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256_MASK`.
    fn vpmulld256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256_MASKZ`.
    fn vpmulld256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256RRB`.
    fn vpmulld256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256RRB_MASK`.
    fn vpmulld256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD256RRB_MASKZ`.
    fn vpmulld256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512`.
    fn vpmulld512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512_MASK`.
    fn vpmulld512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512_MASKZ`.
    fn vpmulld512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512RRB`.
    fn vpmulld512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512RRB_MASK`.
    fn vpmulld512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLD512RRB_MASKZ`.
    fn vpmulld512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128`.
    fn vpmullq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128_MASK`.
    fn vpmullq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128_MASKZ`.
    fn vpmullq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128RRB`.
    fn vpmullq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128RRB_MASK`.
    fn vpmullq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ128RRB_MASKZ`.
    fn vpmullq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256`.
    fn vpmullq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256_MASK`.
    fn vpmullq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256_MASKZ`.
    fn vpmullq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256RRB`.
    fn vpmullq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256RRB_MASK`.
    fn vpmullq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ256RRB_MASKZ`.
    fn vpmullq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512`.
    fn vpmullq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512_MASK`.
    fn vpmullq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512_MASKZ`.
    fn vpmullq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLQ512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLQ512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VPMULLQ512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512RRB`.
    fn vpmullq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512RRB_MASK`.
    fn vpmullq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VPMULLQ512RRB_MASKZ`.
    fn vpmullq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VPMULLQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128`.
    fn vrangepd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD128RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128_MASK`.
    fn vrangepd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128_MASKZ`.
    fn vrangepd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128RRBI`.
    fn vrangepd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128RRBI_MASK`.
    fn vrangepd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD128RRBI_MASKZ`.
    fn vrangepd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256`.
    fn vrangepd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD256RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256_MASK`.
    fn vrangepd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256_MASKZ`.
    fn vrangepd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256RRBI`.
    fn vrangepd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256RRBI_MASK`.
    fn vrangepd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD256RRBI_MASKZ`.
    fn vrangepd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512`.
    fn vrangepd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512_MASK`.
    fn vrangepd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRRI_MASK_SAE`.
    fn vrangepd512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512_MASKZ`.
    fn vrangepd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPD512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPD512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRRI_MASKZ_SAE`.
    fn vrangepd512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRRI_SAE`.
    fn vrangepd512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRBI`.
    fn vrangepd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRBI_MASK`.
    fn vrangepd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPD512RRBI_MASKZ`.
    fn vrangepd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128`.
    fn vrangeps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS128RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128_MASK`.
    fn vrangeps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128_MASKZ`.
    fn vrangeps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128RRBI`.
    fn vrangeps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128RRBI_MASK`.
    fn vrangeps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS128RRBI_MASKZ`.
    fn vrangeps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256`.
    fn vrangeps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS256RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256_MASK`.
    fn vrangeps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256_MASKZ`.
    fn vrangeps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256RRBI`.
    fn vrangeps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256RRBI_MASK`.
    fn vrangeps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS256RRBI_MASKZ`.
    fn vrangeps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512`.
    fn vrangeps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS512RRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512_MASK`.
    fn vrangeps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRRI_MASK_SAE`.
    fn vrangeps512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512_MASKZ`.
    fn vrangeps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGEPS512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGEPS512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGEPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRRI_MASKZ_SAE`.
    fn vrangeps512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRRI_SAE`.
    fn vrangeps512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRBI`.
    fn vrangeps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRBI_MASK`.
    fn vrangeps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGEPS512RRBI_MASKZ`.
    fn vrangeps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGEPS512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESD`.
    fn vrangesd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESDRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESDRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESD" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESD_MASK`.
    fn vrangesd_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESDRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESDRRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESD_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESDRRRI_MASK_SAE`.
    fn vrangesd_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESDRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESD_MASKZ`.
    fn vrangesd_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESDRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESDRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESD_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESDRRRI_MASKZ_SAE`.
    fn vrangesd_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESDRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESDRRRI_SAE`.
    fn vrangesd_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESDRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESS`.
    fn vrangess(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESSRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESSRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESS" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESS_MASK`.
    fn vrangess_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESSRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESSRRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESS_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESSRRRI_MASK_SAE`.
    fn vrangess_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESSRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESS_MASKZ`.
    fn vrangess_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VRANGESSRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VRANGESSRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VRANGESS_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESSRRRI_MASKZ_SAE`.
    fn vrangess_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESSRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VRANGESSRRRI_SAE`.
    fn vrangess_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VRANGESSRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128`.
    fn vreducepd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD128RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128_MASK`.
    fn vreducepd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128_MASKZ`.
    fn vreducepd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128RBI`.
    fn vreducepd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128RBI_MASK`.
    fn vreducepd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD128RBI_MASKZ`.
    fn vreducepd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256`.
    fn vreducepd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD256RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256_MASK`.
    fn vreducepd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256_MASKZ`.
    fn vreducepd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256RBI`.
    fn vreducepd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256RBI_MASK`.
    fn vreducepd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD256RBI_MASKZ`.
    fn vreducepd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512`.
    fn vreducepd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD512RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512_MASK`.
    fn vreducepd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RRI_MASK_SAE`.
    fn vreducepd512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512_MASKZ`.
    fn vreducepd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPD512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPD512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RRI_MASKZ_SAE`.
    fn vreducepd512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RRI_SAE`.
    fn vreducepd512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RBI`.
    fn vreducepd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RBI_MASK`.
    fn vreducepd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPD512RBI_MASKZ`.
    fn vreducepd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPD512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128`.
    fn vreduceps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS128RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128_MASK`.
    fn vreduceps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128_MASKZ`.
    fn vreduceps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128RBI`.
    fn vreduceps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128RBI_MASK`.
    fn vreduceps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS128RBI_MASKZ`.
    fn vreduceps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256`.
    fn vreduceps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS256RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256_MASK`.
    fn vreduceps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256_MASKZ`.
    fn vreduceps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256RBI`.
    fn vreduceps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256RBI_MASK`.
    fn vreduceps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS256RBI_MASKZ`.
    fn vreduceps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512`.
    fn vreduceps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS512RMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512_MASK`.
    fn vreduceps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RRI_MASK_SAE`.
    fn vreduceps512_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512_MASKZ`.
    fn vreduceps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VREDUCEPS512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VREDUCEPS512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCEPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RRI_MASKZ_SAE`.
    fn vreduceps512_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RRI_SAE`.
    fn vreduceps512_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RBI`.
    fn vreduceps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RBI_MASK`.
    fn vreduceps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCEPS512RBI_MASKZ`.
    fn vreduceps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCEPS512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESD`.
    fn vreducesd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESDRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESDRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESD" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESD_MASK`.
    fn vreducesd_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESDRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESDRRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESD_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESDRRRI_MASK_SAE`.
    fn vreducesd_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESDRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESD_MASKZ`.
    fn vreducesd_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESDRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESDRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESD_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESDRRRI_MASKZ_SAE`.
    fn vreducesd_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESDRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESDRRRI_SAE`.
    fn vreducesd_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESDRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESS`.
    fn vreducess(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESSRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESSRRMI, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESS" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESS_MASK`.
    fn vreducess_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESSRRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESSRRMI_MASK, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESS_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESSRRRI_MASK_SAE`.
    fn vreducess_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESSRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESS_MASKZ`.
    fn vreducess_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VREDUCESSRRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VREDUCESSRRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VREDUCESS_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESSRRRI_MASKZ_SAE`.
    fn vreducess_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESSRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VREDUCESSRRRI_SAE`.
    fn vreducess_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VREDUCESSRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128`.
    fn vxorpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128_MASK`.
    fn vxorpd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128_MASKZ`.
    fn vxorpd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128RRB`.
    fn vxorpd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128RRB_MASK`.
    fn vxorpd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD128RRB_MASKZ`.
    fn vxorpd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256`.
    fn vxorpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256_MASK`.
    fn vxorpd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256_MASKZ`.
    fn vxorpd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256RRB`.
    fn vxorpd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256RRB_MASK`.
    fn vxorpd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD256RRB_MASKZ`.
    fn vxorpd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512`.
    fn vxorpd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512_MASK`.
    fn vxorpd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512_MASKZ`.
    fn vxorpd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPD512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512RRB`.
    fn vxorpd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512RRB_MASK`.
    fn vxorpd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPD512RRB_MASKZ`.
    fn vxorpd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128`.
    fn vxorps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS128RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS128" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128_MASK`.
    fn vxorps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS128_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128_MASKZ`.
    fn vxorps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS128_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128RRB`.
    fn vxorps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128RRB_MASK`.
    fn vxorps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS128RRB_MASKZ`.
    fn vxorps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256`.
    fn vxorps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS256RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS256" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256_MASK`.
    fn vxorps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS256_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256_MASKZ`.
    fn vxorps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS256_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256RRB`.
    fn vxorps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256RRB_MASK`.
    fn vxorps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS256RRB_MASKZ`.
    fn vxorps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512`.
    fn vxorps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS512RRM, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS512" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512_MASK`.
    fn vxorps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS512_MASK" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512_MASKZ`.
    fn vxorps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VXORPS512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VXORPS512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VXORPS512_MASKZ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512RRB`.
    fn vxorps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512RRB_MASK`.
    fn vxorps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VXORPS512RRB_MASKZ`.
    fn vxorps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VXORPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
