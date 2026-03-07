pub trait X86SMAPEmitter: Emitter {
    /// Emits `CLAC` (`CLAC`). Clears the AC flag bit in EFLAGS register. This disables any alignment checking of user-mode data accesses. Ifthe SMAP bit is set in the CR4 register, this disallows explicit supervisor-mode data accesses to user-mode pages.
    /// Reference: [Intel x86 docs for CLAC](https://www.felixcloutier.com/x86/CLAC.html)
    fn clac(&mut self,) -> () {
        self.emit(CLAC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `STAC` (`STAC`). Sets the AC flag bit in EFLAGS register. This may enable alignment checking of user-mode data accesses. This allows explicit supervisor-mode data accesses to user-mode pages even if the SMAP bit is set in the CR4 register.
    /// Reference: [Intel x86 docs for STAC](https://www.felixcloutier.com/x86/STAC.html)
    fn stac(&mut self,) -> () {
        self.emit(STAC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
