pub trait X86GFNIEmitter: Emitter {
    /// Emits `GF2P8MULBRR`.
    fn gf2p8mulbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(GF2P8MULBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `GF2P8MULBRM`.
    fn gf2p8mulbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(GF2P8MULBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `GF2P8AFFINEQBRRI`.
    fn gf2p8affineqbrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(GF2P8AFFINEQBRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `GF2P8AFFINEQBRMI`.
    fn gf2p8affineqbrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(GF2P8AFFINEQBRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `GF2P8AFFINEINVQBRRI`.
    fn gf2p8affineinvqbrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(GF2P8AFFINEINVQBRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `GF2P8AFFINEINVQBRMI`.
    fn gf2p8affineinvqbrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(GF2P8AFFINEINVQBRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
