pub trait X86AVX512VPOPCNTDQEmitter: Emitter {
    /// Emits `VPOPCNTD128RR` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RR_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RR_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RM` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RM_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RM_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RB` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RB_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD128RB_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RR` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RR_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RR_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RM` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RM_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RM_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RB` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RB_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD256RB_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RR` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RR_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RR_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RM` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RM_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RM_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RB` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RB_MASK` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTD512RB_MASKZ` (`VPOPCNTD`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTD](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntd512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RR` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RR_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RR_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RM` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RM_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RM_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RB` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RB_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ128RB_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RR` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RR_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RR_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RM` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RM_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RM_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RB` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RB_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ256RB_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RR` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RR_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RR_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RM` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RM_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RM_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RB` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RB_MASK` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPOPCNTQ512RB_MASKZ` (`VPOPCNTQ`). This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    /// Reference: [Intel x86 docs for VPOPCNTQ](https://www.felixcloutier.com/x86/VPOPCNT.html)
    fn vpopcntq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
