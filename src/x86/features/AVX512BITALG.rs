pub trait X86AVX512BITALGEmitter: Emitter {
    /// Emits `VPOPCNTB128RR` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB128RR_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB128RR_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB128RM` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB128RM_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB128RM_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RR` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RR_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RR_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RM` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RM_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB256RM_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RR` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RR_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RR_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RM` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RM_MASK` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTB512RM_MASKZ` (`VPOPCNTB`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTB](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntb512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTB512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RR` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RR_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RR_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RM` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RM_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW128RM_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RR` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RR_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RR_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RM` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RM_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW256RM_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RR` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RR_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RR_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RM` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RM_MASK` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTW512RM_MASKZ` (`VPOPCNTW`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTW](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntw512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTW512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB128KRR` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb128krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB128KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB128KRR_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb128krr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB128KRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB128KRM` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb128krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB128KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB128KRM_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb128krm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB128KRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB256KRR` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb256krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB256KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB256KRR_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb256krr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB256KRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB256KRM` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb256krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB256KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB256KRM_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb256krm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB256KRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB512KRR` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb512krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB512KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB512KRR_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb512krr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB512KRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB512KRM` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb512krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB512KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHUFBITQMB512KRM_MASK` (`VPSHUFBITQMB`). The VPSHUFBITQMB instruction performs a bit gather select using second source as control and first source as data. Each bit uses 6 control bits (2nd source operand) to select which data bit is going to be gathered (first source operand). A given bit can only access 64 different bits of data (first 64 destination bits can access first 64 data bits, second 64 destination bits can access second 64 data bits, etc.).
    /// Reference: [Intel x86 docs for VPSHUFBITQMB](https://www.felixcloutier.com/x86/VPSHUFBITQMB.html)
    fn vpshufbitqmb512krm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHUFBITQMB512KRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
