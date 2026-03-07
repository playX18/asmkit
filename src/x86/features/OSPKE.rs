pub trait X86OSPKEEmitter: Emitter {
    /// Emits `RDPKRU` (`RDPKRU`). Reads the value of PKRU into EAX and clears EDX. ECX must be 0 when RDPKRU is executed; otherwise, a general-protection exception (#GP) occurs.
    /// Reference: [Intel x86 docs for RDPKRU](https://www.felixcloutier.com/x86/RDPKRU.html)
    fn rdpkru(&mut self,) -> () {
        self.emit(RDPKRU, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRPKRU` (`WRPKRU`). Writes the value of EAX into PKRU. ECX and EDX must be 0 when WRPKRU is executed; otherwise, a general-protection exception (#GP) occurs.
    /// Reference: [Intel x86 docs for WRPKRU](https://www.felixcloutier.com/x86/WRPKRU.html)
    fn wrpkru(&mut self,) -> () {
        self.emit(WRPKRU, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
