pub trait X86AVX512DQEmitter: Emitter {
    /// Emits `VANDPS128RRR` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRM` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRR` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRM` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRR` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRM` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRR` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRM` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRR` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRM` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRR` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRM` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRR` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRM` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRR` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRM` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRR` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRM` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRR` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRM` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRR` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRM` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRR` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRM` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRR` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRM` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRR` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRM` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRR` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRM` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRR` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRM` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRR` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRM` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRR` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRM` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPEXTRDRRI`.
    fn vpextrdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPEXTRDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPEXTRDMRI`.
    fn vpextrdmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPEXTRDMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPEXTRQRRI`.
    fn vpextrqrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPEXTRQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPEXTRQMRI`.
    fn vpextrqmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPEXTRQMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPINSRDRRRI`.
    fn vpinsrdrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPINSRDRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPINSRDRRMI`.
    fn vpinsrdrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPINSRDRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPINSRQRRRI`.
    fn vpinsrqrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPINSRQRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPINSRQRRMI`.
    fn vpinsrqrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPINSRQRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VANDPS128RRR_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRR_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRM_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRM_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRB` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRB_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS128RRB_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRR_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRR_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRM_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRM_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRB` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRB_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS256RRB_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRR` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRR_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRR_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRM` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRM_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRM_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRB` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRB_MASK` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPS512RRB_MASKZ` (`VANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn vandps512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRR_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRR_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRM_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRM_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRB` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRB_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD128RRB_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRR_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRR_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRM_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRM_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRB` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRB_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD256RRB_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRR` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRR_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRR_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRM` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRM_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRM_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRB` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRB_MASK` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDPD512RRB_MASKZ` (`VANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn vandpd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRR_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRR_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRM_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRM_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRB` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRB_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS128RRB_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRR_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRR_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRM_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRM_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRB` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRB_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS256RRB_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRR` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRR_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRR_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRM` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRM_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRM_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRB` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRB_MASK` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPS512RRB_MASKZ` (`VANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn vandnps512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRR_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRR_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRM_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRM_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRB` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRB_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD128RRB_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRR_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRR_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRM_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRM_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRB` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRB_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD256RRB_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRR` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRR_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRR_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRM` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRM_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRM_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRB` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRB_MASK` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VANDNPD512RRB_MASKZ` (`VANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn vandnpd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VANDNPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRR_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRR_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRM_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRM_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRB` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRB_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS128RRB_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRR_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRR_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRM_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRM_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRB` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRB_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS256RRB_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRR` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRR_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRR_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRM` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRM_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRM_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRB` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRB_MASK` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPS512RRB_MASKZ` (`VORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn vorps512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRR_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRR_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRM_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRM_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRB` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRB_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD128RRB_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRR_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRR_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRM_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRM_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRB` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRB_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD256RRB_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRR` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRR_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRR_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRM` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRM_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRM_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRB` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRB_MASK` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VORPD512RRB_MASKZ` (`VORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn vorpd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VORPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRR_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRR_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRM_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRM_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRB` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRB_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD128RRB_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRR_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRR_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRM_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRM_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRB` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRB_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD256RRB_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRR` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRR_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRR_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRM` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRM_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRM_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRB` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRB_MASK` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLD512RRB_MASKZ` (`VPMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmulld512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRR` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRR_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRR_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRM` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRM_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRM_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRB` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRB_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ128RRB_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRR` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRR_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRR_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRM` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRM_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRM_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRB` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRB_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ256RRB_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRR` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRR_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRR_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRM` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRM_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRM_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRB` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRB_MASK` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULLQ512RRB_MASKZ` (`VPMULLQ`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for VPMULLQ](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn vpmullq512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULLQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RR` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RR_MASK` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RR_MASKZ` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RM` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RM_MASK` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_256RM_MASKZ` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RR` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RR_MASK` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RR_MASKZ` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RM` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RM_MASK` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X2_512RM_MASKZ` (`VBROADCASTF32X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x2_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_256RM` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_256RM_MASK` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_256RM_MASKZ` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_512RM` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_512RM_MASK` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF64X2_512RM_MASKZ` (`VBROADCASTF64X2`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF64X2](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf64x2_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF64X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X8_512RM` (`VBROADCASTF32X8`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X8](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x8_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X8_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X8_512RM_MASK` (`VBROADCASTF32X8`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X8](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x8_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X8_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF32X8_512RM_MASKZ` (`VBROADCASTF32X8`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF32X8](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf32x8_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF32X8_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RR` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RR_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RR_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RM` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RM_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RM_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RB` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RB_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ128RB_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RR` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RR_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RR_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RM` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RM_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RM_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RB` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RB_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ256RB_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR_ER` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR_MASK_ER` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RR_MASKZ_ER` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rr_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RM` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RM_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RM_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RB` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RB_MASK` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPS2QQ512RB_MASKZ` (`VCVTPS2QQ`). Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTPS2QQ](https://www.felixcloutier.com/x86/VCVTPS2QQ.html)
    fn vcvtps2qq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPS2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RR` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RR_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RR_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RM` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RM_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RM_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RB` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RB_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ128RB_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RR` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RR_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RR_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RM` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RM_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RM_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RB` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RB_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ256RB_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR_ER` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR_MASK_ER` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RR_MASKZ_ER` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rr_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RM` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RM_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RM_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RB` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RB_MASK` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTPD2QQ512RB_MASKZ` (`VCVTPD2QQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTPD2QQ](https://www.felixcloutier.com/x86/VCVTPD2QQ.html)
    fn vcvtpd2qq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTPD2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RR` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RR_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RR_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RM` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RM_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RM_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RB` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RB_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD128RB_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RR` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RR_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RR_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RM` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RM_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RM_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RB` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RB_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD256RB_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR_ER` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR_MASK_ER` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RR_MASKZ_ER` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rr_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RM` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RM_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RM_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RB` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RB_MASK` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PD512RB_MASKZ` (`VCVTQQ2PD`). Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PD](https://www.felixcloutier.com/x86/VCVTQQ2PD.html)
    fn vcvtqq2pd512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RR` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RR_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RR_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RM` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RM_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RM_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RB` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RB_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS128RB_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RR` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RR_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RR_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RM` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RM_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RM_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RB` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RB_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS256RB_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR_ER` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR_MASK_ER` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr_mask_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR_MASK_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RR_MASKZ_ER` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rr_maskz_er(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RR_MASKZ_ER, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RM` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RM_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RM_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RB` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RB_MASK` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTQQ2PS512RB_MASKZ` (`VCVTQQ2PS`). Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTQQ2PS](https://www.felixcloutier.com/x86/VCVTQQ2PS.html)
    fn vcvtqq2ps512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTQQ2PS512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RR` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RR_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RR_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RM` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RM_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RM_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RB` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RB_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ128RB_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RR` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RR_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RR_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RM` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RM_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RM_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RB` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RB_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ256RB_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR_SAE` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR_MASK_SAE` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RR_MASKZ_SAE` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rr_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RM` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RM_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RM_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RB` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RB_MASK` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPS2QQ512RB_MASKZ` (`VCVTTPS2QQ`). Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    /// Reference: [Intel x86 docs for VCVTTPS2QQ](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html)
    fn vcvttps2qq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPS2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RR` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RR_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RR_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RM` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RM_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RM_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RB` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RB_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ128RB_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RR` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RR_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RR_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RM` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RM_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RM_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RB` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RB_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ256RB_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR_SAE` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR_MASK_SAE` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR_MASK_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RR_MASKZ_SAE` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rr_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RR_MASKZ_SAE, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RM` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RM_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RM_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RB` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RB_MASK` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTTPD2QQ512RB_MASKZ` (`VCVTTPD2QQ`). Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for VCVTTPD2QQ](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html)
    fn vcvttpd2qq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTTPD2QQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KRI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KRI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KMI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KMI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KBI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS128KBI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps128kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS128KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KRI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KRI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KMI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KMI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KBI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS256KBI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps256kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS256KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KRI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KRI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KMI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KMI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KBI` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPS512KBI_MASK` (`VFPCLASSPS`). The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPS](https://www.felixcloutier.com/x86/VFPCLASSPS.html)
    fn vfpclassps512kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPS512KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KRI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KRI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KMI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KMI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KBI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD128KBI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd128kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD128KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KRI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KRI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KMI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KMI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KBI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD256KBI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd256kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD256KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KRI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KRI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KMI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KMI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KBI` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSPD512KBI_MASK` (`VFPCLASSPD`). The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSPD](https://www.felixcloutier.com/x86/VFPCLASSPD.html)
    fn vfpclasspd512kbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSPD512KBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSSKRI` (`VFPCLASSSS`). The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSS](https://www.felixcloutier.com/x86/VFPCLASSSS.html)
    fn vfpclasssskri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSSKRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSSKRI_MASK` (`VFPCLASSSS`). The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSS](https://www.felixcloutier.com/x86/VFPCLASSSS.html)
    fn vfpclasssskri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSSKRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSSKMI` (`VFPCLASSSS`). The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSS](https://www.felixcloutier.com/x86/VFPCLASSSS.html)
    fn vfpclasssskmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSSKMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSSKMI_MASK` (`VFPCLASSSS`). The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSS](https://www.felixcloutier.com/x86/VFPCLASSSS.html)
    fn vfpclasssskmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSSKMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSDKRI` (`VFPCLASSSD`). The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSD](https://www.felixcloutier.com/x86/VFPCLASSSD.html)
    fn vfpclasssdkri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSDKRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSDKRI_MASK` (`VFPCLASSSD`). The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSD](https://www.felixcloutier.com/x86/VFPCLASSSD.html)
    fn vfpclasssdkri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSDKRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSDKMI` (`VFPCLASSSD`). The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSD](https://www.felixcloutier.com/x86/VFPCLASSSD.html)
    fn vfpclasssdkmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSDKMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VFPCLASSSDKMI_MASK` (`VFPCLASSSD`). The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    /// Reference: [Intel x86 docs for VFPCLASSSD](https://www.felixcloutier.com/x86/VFPCLASSSD.html)
    fn vfpclasssdkmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VFPCLASSSDKMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VINSERTF64X2_256RRRI` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_256RRRI_MASK` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_256RRRI_MASKZ` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_256RRMI` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_256RRMI_MASK` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_256RRMI_MASKZ` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRRI` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRRI_MASK` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRRI_MASKZ` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRMI` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRMI_MASK` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF64X2_512RRMI_MASKZ` (`VINSERTF64X2`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF64X2](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf64x2_512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF64X2_512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRRI` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRRI_MASK` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRRI_MASKZ` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRMI` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRMI_MASK` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF32X8_512RRMI_MASKZ` (`VINSERTF32X8`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF32X8](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf32x8_512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF32X8_512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRRI` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRRI_MASK` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRRI_MASKZ` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRMI` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRMI_MASK` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_256RRMI_MASKZ` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRRI` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRRI_MASK` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRRI_MASKZ` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRMI` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRMI_MASK` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI64X2_512RRMI_MASKZ` (`VINSERTI64X2`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI64X2](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti64x2_512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI64X2_512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRRI` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRRI_MASK` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRRI_MASKZ` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRMI` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRMI_MASK` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI32X8_512RRMI_MASKZ` (`VINSERTI32X8`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI32X8](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti32x8_512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI32X8_512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBROADCASTI32X2_128RR`.
    fn vbroadcasti32x2_128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_128RR_MASK`.
    fn vbroadcasti32x2_128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_128RR_MASKZ`.
    fn vbroadcasti32x2_128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_128RM`.
    fn vbroadcasti32x2_128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_128RM_MASK`.
    fn vbroadcasti32x2_128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_128RM_MASKZ`.
    fn vbroadcasti32x2_128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RR`.
    fn vbroadcasti32x2_256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RR_MASK`.
    fn vbroadcasti32x2_256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RR_MASKZ`.
    fn vbroadcasti32x2_256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RM`.
    fn vbroadcasti32x2_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RM_MASK`.
    fn vbroadcasti32x2_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_256RM_MASKZ`.
    fn vbroadcasti32x2_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RR`.
    fn vbroadcasti32x2_512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RR_MASK`.
    fn vbroadcasti32x2_512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RR_MASKZ`.
    fn vbroadcasti32x2_512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RM`.
    fn vbroadcasti32x2_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RM_MASK`.
    fn vbroadcasti32x2_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X2_512RM_MASKZ`.
    fn vbroadcasti32x2_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_256RM` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_256RM_MASK` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_256RM_MASKZ` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_512RM` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_512RM_MASK` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X4_512RM_MASKZ` (`VBROADCASTI32X4`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X4](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x4_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X4_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_256RM` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_256RM_MASK` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_256RM_MASKZ` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_512RM` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_512RM_MASK` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI64X2_512RM_MASKZ` (`VBROADCASTI64X2`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI64X2](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti64x2_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI64X2_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X8_512RM` (`VBROADCASTI32X8`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X8](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x8_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X8_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X8_512RM_MASK` (`VBROADCASTI32X8`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X8](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x8_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X8_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTI32X8_512RM_MASKZ` (`VBROADCASTI32X8`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI32X8](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti32x8_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI32X8_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVD2M128KR` (`VPMOVD2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVD2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovd2m128kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVD2M128KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVD2M256KR` (`VPMOVD2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVD2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovd2m256kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVD2M256KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVD2M512KR` (`VPMOVD2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVD2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovd2m512kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVD2M512KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVQ2M128KR` (`VPMOVQ2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVQ2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovq2m128kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVQ2M128KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVQ2M256KR` (`VPMOVQ2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVQ2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovq2m256kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVQ2M256KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVQ2M512KR` (`VPMOVQ2M`). Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    /// Reference: [Intel x86 docs for VPMOVQ2M](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html)
    fn vpmovq2m512kr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVQ2M512KR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2D128RK` (`VPMOVM2D`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2D](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2d128rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2D128RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2D256RK` (`VPMOVM2D`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2D](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2d256rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2D256RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2D512RK` (`VPMOVM2D`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2D](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2d512rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2D512RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2Q128RK` (`VPMOVM2Q`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2Q](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2q128rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2Q128RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2Q256RK` (`VPMOVM2Q`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2Q](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2q256rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2Q256RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVM2Q512RK` (`VPMOVM2Q`). Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    /// Reference: [Intel x86 docs for VPMOVM2Q](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html)
    fn vpmovm2q512rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2Q512RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRANGEPS128RRRI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRRI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRRI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRMI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRMI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRMI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRBI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRBI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS128RRBI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRRI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRRI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRRI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRMI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRMI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRMI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRBI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRBI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS256RRBI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI_SAE` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI_MASK_SAE` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRRI_MASKZ_SAE` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRMI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRMI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRMI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRBI` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRBI_MASK` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPS512RRBI_MASKZ` (`VRANGEPS`). This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPS](https://www.felixcloutier.com/x86/VRANGEPS.html)
    fn vrangeps512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPS512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRRI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRRI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRRI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRMI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRMI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRMI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRBI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRBI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD128RRBI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRRI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRRI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRRI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRMI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRMI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRMI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRBI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRBI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD256RRBI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI_SAE` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI_MASK_SAE` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRRI_MASKZ_SAE` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRMI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRMI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRMI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRBI` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRBI_MASK` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGEPD512RRBI_MASKZ` (`VRANGEPD`). This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGEPD](https://www.felixcloutier.com/x86/VRANGEPD.html)
    fn vrangepd512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGEPD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI_SAE` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI_MASK` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI_MASK_SAE` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI_MASKZ` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRRI_MASKZ_SAE` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRMI` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRMI_MASK` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESSRRMI_MASKZ` (`VRANGESS`). This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESS](https://www.felixcloutier.com/x86/VRANGESS.html)
    fn vrangessrrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESSRRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI_SAE` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI_MASK` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI_MASK_SAE` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI_MASKZ` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRRI_MASKZ_SAE` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRMI` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRMI_MASK` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VRANGESDRRMI_MASKZ` (`VRANGESD`). This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VRANGESD](https://www.felixcloutier.com/x86/VRANGESD.html)
    fn vrangesdrrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VRANGESDRRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCEPS128RRI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RRI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RRI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RMI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RMI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RMI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RBI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RBI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS128RBI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps128rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RRI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RRI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RRI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RMI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RMI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RMI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RBI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RBI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS256RBI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps256rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI_SAE` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI_MASK_SAE` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RRI_MASKZ_SAE` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RMI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RMI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RMI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RBI` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RBI_MASK` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPS512RBI_MASKZ` (`VREDUCEPS`). Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPS](https://www.felixcloutier.com/x86/VREDUCEPS.html)
    fn vreduceps512rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPS512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RRI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RRI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RRI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RMI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RMI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RMI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RBI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RBI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD128RBI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd128rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD128RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RRI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RRI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RRI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RMI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RMI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RMI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RBI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RBI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD256RBI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd256rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD256RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI_SAE` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI_MASK_SAE` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RRI_MASKZ_SAE` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RMI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RMI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RMI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RBI` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RBI_MASK` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCEPD512RBI_MASKZ` (`VREDUCEPD`). Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VREDUCEPD](https://www.felixcloutier.com/x86/VREDUCEPD.html)
    fn vreducepd512rbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VREDUCEPD512RBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VREDUCESSRRRI` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRRI_SAE` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRRI_MASK` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRRI_MASK_SAE` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRRI_MASKZ` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRRI_MASKZ_SAE` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRMI` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRMI_MASK` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESSRRMI_MASKZ` (`VREDUCESS`). Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESS](https://www.felixcloutier.com/x86/VREDUCESS.html)
    fn vreducessrrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESSRRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI_SAE` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI_MASK` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI_MASK_SAE` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri_mask_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI_MASK_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI_MASKZ` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRRI_MASKZ_SAE` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrri_maskz_sae(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRRI_MASKZ_SAE, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRMI` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRMI_MASK` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VREDUCESDRRMI_MASKZ` (`VREDUCESD`). Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    /// Reference: [Intel x86 docs for VREDUCESD](https://www.felixcloutier.com/x86/VREDUCESD.html)
    fn vreducesdrrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VREDUCESDRRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VXORPS128RRR_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRR_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRM_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRM_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRB` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRB_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS128RRB_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRR_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRR_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRM_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRM_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRB` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRB_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS256RRB_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRR` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRR_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRR_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRM` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRM_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRM_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRB` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRB_MASK` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPS512RRB_MASKZ` (`VXORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for VXORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn vxorps512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRR_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRR_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRM_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRM_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRB` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRB_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD128RRB_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRR_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRR_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRM_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRM_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRB` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRB_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD256RRB_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRR` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRR_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRR_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRM` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRM_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRM_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRB` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRB_MASK` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VXORPD512RRB_MASKZ` (`VXORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for VXORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn vxorpd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VXORPD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KANDBKKK` (`KANDB`). Performs a bitwise AND between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    /// Reference: [Intel x86 docs for KANDB](https://www.felixcloutier.com/x86/KANDW%3AKANDB%3AKANDQ%3AKANDD.html)
    fn kandbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KANDNBKKK` (`KANDNB`). Performs a bitwise AND NOT between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    /// Reference: [Intel x86 docs for KANDNB](https://www.felixcloutier.com/x86/KANDNW%3AKANDNB%3AKANDNQ%3AKANDND.html)
    fn kandnbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDNBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KNOTBKK` (`KNOTB`). Performs a bitwise NOT of vector mask k2 and writes the result into vector mask k1.
    /// Reference: [Intel x86 docs for KNOTB](https://www.felixcloutier.com/x86/KNOTW%3AKNOTB%3AKNOTQ%3AKNOTD.html)
    fn knotbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KNOTBKK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KORBKKK` (`KORB`). Performs a bitwise OR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    /// Reference: [Intel x86 docs for KORB](https://www.felixcloutier.com/x86/KORW%3AKORB%3AKORQ%3AKORD.html)
    fn korbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KXNORBKKK` (`KXNORB`). Performs a bitwise XNOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    /// Reference: [Intel x86 docs for KXNORB](https://www.felixcloutier.com/x86/KXNORW%3AKXNORB%3AKXNORQ%3AKXNORD.html)
    fn kxnorbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXNORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KXORBKKK` (`KXORB`). Performs a bitwise XOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    /// Reference: [Intel x86 docs for KXORB](https://www.felixcloutier.com/x86/KXORW%3AKXORB%3AKXORQ%3AKXORD.html)
    fn kxorbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXORBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KADDBKKK` (`KADDB`). Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    /// Reference: [Intel x86 docs for KADDB](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html)
    fn kaddbkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KADDBKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KADDWKKK` (`KADDW`). Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    /// Reference: [Intel x86 docs for KADDW](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html)
    fn kaddwkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KADDWKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KORTESTBKK` (`KORTESTB`). Performs a bitwise OR between the vector mask register k2, and the vector mask register k1, and sets CF and ZF based on the operation result.
    /// Reference: [Intel x86 docs for KORTESTB](https://www.felixcloutier.com/x86/KORTESTW%3AKORTESTB%3AKORTESTQ%3AKORTESTD.html)
    fn kortestbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KORTESTBKK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KMOVBKK` (`KMOVB`). Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    /// Reference: [Intel x86 docs for KMOVB](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html)
    fn kmovbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVBKK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KMOVBKM` (`KMOVB`). Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    /// Reference: [Intel x86 docs for KMOVB](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html)
    fn kmovbkm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVBKM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KMOVBMK` (`KMOVB`). Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    /// Reference: [Intel x86 docs for KMOVB](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html)
    fn kmovbmk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVBMK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KMOVBKR` (`KMOVB`). Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    /// Reference: [Intel x86 docs for KMOVB](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html)
    fn kmovbkr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVBKR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KMOVBRK` (`KMOVB`). Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    /// Reference: [Intel x86 docs for KMOVB](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html)
    fn kmovbrk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVBRK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KTESTBKK` (`KTESTB`). Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
    /// Reference: [Intel x86 docs for KTESTB](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html)
    fn ktestbkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KTESTBKK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KTESTWKK` (`KTESTW`). Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
    /// Reference: [Intel x86 docs for KTESTW](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html)
    fn ktestwkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KTESTWKK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `KSHIFTRBKKI` (`KSHIFTRB`). Shifts 8/16/32/64 bits in the second operand (source operand) right by the count specified in immediate and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
    /// Reference: [Intel x86 docs for KSHIFTRB](https://www.felixcloutier.com/x86/KSHIFTRW%3AKSHIFTRB%3AKSHIFTRQ%3AKSHIFTRD.html)
    fn kshiftrbkki(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTRBKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `KSHIFTLBKKI` (`KSHIFTLB`). Shifts 8/16/32/64 bits in the second operand (source operand) left by the count specified in immediate byte and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
    /// Reference: [Intel x86 docs for KSHIFTLB](https://www.felixcloutier.com/x86/KSHIFTLW%3AKSHIFTLB%3AKSHIFTLQ%3AKSHIFTLD.html)
    fn kshiftlbkki(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTLBKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
