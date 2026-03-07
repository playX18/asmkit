pub trait X86TSXLDTRKEmitter: Emitter {
    /// Emits `XSUSLDTRK` (`XSUSLDTRK`). The instruction marks the start of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a transactional region, subsequent loads are not added to the read set of the transaction. If the instruction is used inside a suspend load address tracking region it will cause transaction abort.
    /// Reference: [Intel x86 docs for XSUSLDTRK](https://www.felixcloutier.com/x86/XSUSLDTRK.html)
    fn xsusldtrk(&mut self,) -> () {
        self.emit(XSUSLDTRK, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XRESLDTRK` (`XRESLDTRK`). The instruction marks the end of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a suspend load address tracking region it will end the suspend region and all following load addresses will be added to the transaction read set. If this instruction is used inside an active transaction but not in a suspend region it will cause transaction abort.
    /// Reference: [Intel x86 docs for XRESLDTRK](https://www.felixcloutier.com/x86/XRESLDTRK.html)
    fn xresldtrk(&mut self,) -> () {
        self.emit(XRESLDTRK, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
