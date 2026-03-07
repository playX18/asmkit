pub trait X86AVX512VP2INTERSECTEmitter: Emitter {
    /// Emits `VP2INTERSECTD128K`.
    fn vp2intersectd128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTD128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTD128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTD128K");
        }
    }
    /// Emits `VP2INTERSECTD128KRB`.
    fn vp2intersectd128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD128KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VP2INTERSECTD256K`.
    fn vp2intersectd256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTD256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTD256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTD256K");
        }
    }
    /// Emits `VP2INTERSECTD256KRB`.
    fn vp2intersectd256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD256KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VP2INTERSECTD512K`.
    fn vp2intersectd512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTD512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTD512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTD512K");
        }
    }
    /// Emits `VP2INTERSECTD512KRB`.
    fn vp2intersectd512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD512KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VP2INTERSECTQ128K`.
    fn vp2intersectq128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTQ128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTQ128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTQ128K");
        }
    }
    /// Emits `VP2INTERSECTQ128KRB`.
    fn vp2intersectq128kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ128KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VP2INTERSECTQ256K`.
    fn vp2intersectq256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTQ256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTQ256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTQ256K");
        }
    }
    /// Emits `VP2INTERSECTQ256KRB`.
    fn vp2intersectq256kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ256KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VP2INTERSECTQ512K`.
    fn vp2intersectq512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VP2INTERSECTQ512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VP2INTERSECTQ512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VP2INTERSECTQ512K");
        }
    }
    /// Emits `VP2INTERSECTQ512KRB`.
    fn vp2intersectq512kb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ512KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
}
