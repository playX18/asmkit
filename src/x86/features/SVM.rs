pub trait X86SVMEmitter: Emitter {
    /// Emits `VMRUN`.
    fn vmrun(&mut self,) -> () {
        self.emit(VMRUN, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMMCALL`.
    fn vmmcall(&mut self,) -> () {
        self.emit(VMMCALL, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMLOAD`.
    fn vmload(&mut self,) -> () {
        self.emit(VMLOAD, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMSAVE`.
    fn vmsave(&mut self,) -> () {
        self.emit(VMSAVE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `INVLPGA`.
    fn invlpga(&mut self,) -> () {
        self.emit(INVLPGA, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
