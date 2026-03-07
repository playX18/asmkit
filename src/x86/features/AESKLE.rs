pub trait X86AESKLEEmitter: Emitter {
    /// Emits `AESENCWIDE128KLM`.
    fn aesencwide128klm(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESENCWIDE128KLM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECWIDE128KLM`.
    fn aesdecwide128klm(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESDECWIDE128KLM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENCWIDE256KLM`.
    fn aesencwide256klm(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESENCWIDE256KLM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECWIDE256KLM`.
    fn aesdecwide256klm(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESDECWIDE256KLM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENC128KLRM`.
    fn aesenc128klrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENC128KLRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LOADIWKEYRR`.
    fn loadiwkeyrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LOADIWKEYRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDEC128KLRM`.
    fn aesdec128klrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDEC128KLRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENC256KLRM`.
    fn aesenc256klrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENC256KLRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDEC256KLRM`.
    fn aesdec256klrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDEC256KLRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENCODEKEY128RR`.
    fn encodekey128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENCODEKEY128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENCODEKEY256RR`.
    fn encodekey256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENCODEKEY256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
