pub trait X86AVX512CDEmitter: Emitter {
    /// Emits `VPBROADCASTMB2Q128RK` (`VPBROADCASTMB2Q`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMB2Q](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmb2q128rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q128RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPBROADCASTMB2Q256RK` (`VPBROADCASTMB2Q`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMB2Q](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmb2q256rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q256RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPBROADCASTMB2Q512RK` (`VPBROADCASTMB2Q`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMB2Q](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmb2q512rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q512RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPBROADCASTMW2D128RK` (`VPBROADCASTMW2D`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMW2D](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmw2d128rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D128RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPBROADCASTMW2D256RK` (`VPBROADCASTMW2D`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMW2D](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmw2d256rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D256RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPBROADCASTMW2D512RK` (`VPBROADCASTMW2D`). Broadcasts the zero-extended 64/32 bit value of the low byte/word of the source operand (the second operand) to each 64/32 bit element of the destination operand (the first operand). The source operand is an opmask register. The destination operand is a ZMM register (EVEX.512), YMM register (EVEX.256), or XMM register (EVEX.128).
    /// Reference: [Intel x86 docs for VPBROADCASTMW2D](https://www.felixcloutier.com/x86/VPBROADCASTM.html)
    fn vpbroadcastmw2d512rk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D512RK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RR` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RR_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RR_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RM` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RM_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RM_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RB` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RB_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD128RB_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RR` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RR_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RR_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RM` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RM_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RM_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RB` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RB_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD256RB_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RR` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RR_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RR_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RM` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RM_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RM_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RB` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RB_MASK` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTD512RB_MASKZ` (`VPCONFLICTD`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTD](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictd512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RR` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RR_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RR_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RM` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RM_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RM_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RB` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RB_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ128RB_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RR` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RR_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RR_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RM` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RM_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RM_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RB` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RB_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ256RB_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RR` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RR_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RR_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RM` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RM_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RM_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RB` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RB_MASK` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCONFLICTQ512RB_MASKZ` (`VPCONFLICTQ`). Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element. Each element’s comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.
    /// Reference: [Intel x86 docs for VPCONFLICTQ](https://www.felixcloutier.com/x86/VPCONFLICTD%3AVPCONFLICTQ.html)
    fn vpconflictq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RR` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RR_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RR_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RM` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RM_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RM_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RB` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RB_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD128RB_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RR` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RR_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RR_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RM` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RM_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RM_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RB` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RB_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD256RB_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RR` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RR_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RR_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RM` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RM_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RM_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RB` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RB_MASK` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTD512RB_MASKZ` (`VPLZCNTD`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTD](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntd512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RR` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RR_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RR_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RM` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RM_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RM_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RB` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RB_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ128RB_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RR` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RR_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RR_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RM` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RM_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RM_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RB` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RB_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ256RB_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RR` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RR_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RR_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RM` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RM_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RM_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RB` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RB_MASK` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPLZCNTQ512RB_MASKZ` (`VPLZCNTQ`). Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask. If an element is zero, the result for that element is the operand size of the element.
    /// Reference: [Intel x86 docs for VPLZCNTQ](https://www.felixcloutier.com/x86/VPLZCNTD%3AVPLZCNTQ.html)
    fn vplzcntq512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
