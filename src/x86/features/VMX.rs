pub trait X86VMXEmitter: Emitter {
    /// Emits `INVEPTRM`.
    fn inveptrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVEPTRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `INVVPIDRM`.
    fn invvpidrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVVPIDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMCALL`.
    fn vmcall(&mut self,) -> () {
        self.emit(VMCALL, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMCLEARM`.
    fn vmclearm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMCLEARM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMFUNC`.
    fn vmfunc(&mut self,) -> () {
        self.emit(VMFUNC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMLAUNCH`.
    fn vmlaunch(&mut self,) -> () {
        self.emit(VMLAUNCH, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMRESUME`.
    fn vmresume(&mut self,) -> () {
        self.emit(VMRESUME, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMPTRLDM`.
    fn vmptrldm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMPTRLDM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMPTRSTM`.
    fn vmptrstm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMPTRSTM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMREADRR`.
    fn vmreadrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMREADRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMREADMR`.
    fn vmreadmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMREADMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMWRITERR`.
    fn vmwriterr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMWRITERR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMWRITERM`.
    fn vmwriterm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMWRITERM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMXOFF`.
    fn vmxoff(&mut self,) -> () {
        self.emit(VMXOFF, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMXONM`.
    fn vmxonm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMXONM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
