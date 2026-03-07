pub trait X86FSGSBASEEmitter: Emitter {
    /// Emits `RDFSBASE32R` (`RDFSBASE`). Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    /// Reference: [Intel x86 docs for RDFSBASE](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html)
    fn rdfsbase32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDFSBASE32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDFSBASE64R` (`RDFSBASE`). Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    /// Reference: [Intel x86 docs for RDFSBASE](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html)
    fn rdfsbase64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDFSBASE64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDGSBASE32R` (`RDGSBASE`). Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    /// Reference: [Intel x86 docs for RDGSBASE](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html)
    fn rdgsbase32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDGSBASE32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDGSBASE64R` (`RDGSBASE`). Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    /// Reference: [Intel x86 docs for RDGSBASE](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html)
    fn rdgsbase64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDGSBASE64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRFSBASE32R` (`WRFSBASE`). Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    /// Reference: [Intel x86 docs for WRFSBASE](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html)
    fn wrfsbase32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRFSBASE32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRFSBASE64R` (`WRFSBASE`). Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    /// Reference: [Intel x86 docs for WRFSBASE](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html)
    fn wrfsbase64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRFSBASE64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRGSBASE32R` (`WRGSBASE`). Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    /// Reference: [Intel x86 docs for WRGSBASE](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html)
    fn wrgsbase32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRGSBASE32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRGSBASE64R` (`WRGSBASE`). Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    /// Reference: [Intel x86 docs for WRGSBASE](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html)
    fn wrgsbase64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRGSBASE64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
