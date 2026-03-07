pub trait X86AESNIEmitter: Emitter {
    /// Emits `AESIMCRR` (`AESIMC`). Perform the InvMixColumns transformation on the source operand and store the result in the destination operand. The destination operand is an XMM register. The source operand can be an XMM register or a 128-bit memory location.
    /// Reference: [Intel x86 docs for AESIMC](https://www.felixcloutier.com/x86/AESIMC.html)
    fn aesimcrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESIMCRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESIMCRM` (`AESIMC`). Perform the InvMixColumns transformation on the source operand and store the result in the destination operand. The destination operand is an XMM register. The source operand can be an XMM register or a 128-bit memory location.
    /// Reference: [Intel x86 docs for AESIMC](https://www.felixcloutier.com/x86/AESIMC.html)
    fn aesimcrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESIMCRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENCRR` (`AESENC`). This instruction performs a single round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESENC](https://www.felixcloutier.com/x86/AESENC.html)
    fn aesencrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENCRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENCRM` (`AESENC`). This instruction performs a single round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESENC](https://www.felixcloutier.com/x86/AESENC.html)
    fn aesencrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENCRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENCLASTRR` (`AESENCLAST`). This instruction performs the last round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESENCLAST](https://www.felixcloutier.com/x86/AESENCLAST.html)
    fn aesenclastrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENCLASTRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESENCLASTRM` (`AESENCLAST`). This instruction performs the last round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESENCLAST](https://www.felixcloutier.com/x86/AESENCLAST.html)
    fn aesenclastrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENCLASTRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECRR` (`AESDEC`). This instruction performs a single round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESDEC](https://www.felixcloutier.com/x86/AESDEC.html)
    fn aesdecrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDECRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECRM` (`AESDEC`). This instruction performs a single round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESDEC](https://www.felixcloutier.com/x86/AESDEC.html)
    fn aesdecrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDECRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECLASTRR` (`AESDECLAST`). This instruction performs the last round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESDECLAST](https://www.felixcloutier.com/x86/AESDECLAST.html)
    fn aesdeclastrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDECLASTRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESDECLASTRM` (`AESDECLAST`). This instruction performs the last round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for AESDECLAST](https://www.felixcloutier.com/x86/AESDECLAST.html)
    fn aesdeclastrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDECLASTRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `AESKEYGENASSISTRRI` (`AESKEYGENASSIST`). Assist in expanding the AES cipher key, by computing steps towards generating a round key for encryption, using 128-bit data specified in the source operand and an 8-bit round constant specified as an immediate, store the result in the destination operand.
    /// Reference: [Intel x86 docs for AESKEYGENASSIST](https://www.felixcloutier.com/x86/AESKEYGENASSIST.html)
    fn aeskeygenassistrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(AESKEYGENASSISTRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `AESKEYGENASSISTRMI` (`AESKEYGENASSIST`). Assist in expanding the AES cipher key, by computing steps towards generating a round key for encryption, using 128-bit data specified in the source operand and an 8-bit round constant specified as an immediate, store the result in the destination operand.
    /// Reference: [Intel x86 docs for AESKEYGENASSIST](https://www.felixcloutier.com/x86/AESKEYGENASSIST.html)
    fn aeskeygenassistrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(AESKEYGENASSISTRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
