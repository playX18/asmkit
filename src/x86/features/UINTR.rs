pub trait X86UINTREmitter: Emitter {
    /// Emits `UIRET` (`UIRET`). UIRET returns from the handling of a user interrupt. It can be executed regardless of CPL.
    /// Reference: [Intel x86 docs for UIRET](https://www.felixcloutier.com/x86/UIRET.html)
    fn uiret(&mut self,) -> () {
        self.emit(UIRET, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TESTUI`.
    fn testui(&mut self,) -> () {
        self.emit(TESTUI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CLUI` (`CLUI`). CLUI clears the user interrupt flag (UIF). Its effect takes place immediately: a user interrupt cannot be delivered on the instruction boundary following CLUI.
    /// Reference: [Intel x86 docs for CLUI](https://www.felixcloutier.com/x86/CLUI.html)
    fn clui(&mut self,) -> () {
        self.emit(CLUI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `STUI` (`STUI`). STUI sets the user interrupt flag (UIF). Its effect takes place immediately; a user interrupt may be delivered on the instruction boundary following STUI. (This is in contrast with STI, whose effect is delayed by one instruction).
    /// Reference: [Intel x86 docs for STUI](https://www.felixcloutier.com/x86/STUI.html)
    fn stui(&mut self,) -> () {
        self.emit(STUI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SENDUIPIR` (`SENDUIPI`). The SENDUIPI instruction sends the user interprocessor interrupt (IPI) indicated by its register operand. (The operand always has 64 bits; operand-size overrides such as the prefix 66 are ignored.)
    /// Reference: [Intel x86 docs for SENDUIPI](https://www.felixcloutier.com/x86/SENDUIPI.html)
    fn senduipir(&mut self,op0: impl OperandCast) -> () {
        self.emit(SENDUIPIR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
