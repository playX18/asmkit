pub trait X86AVX512VBMI2Emitter: Emitter {
    /// Emits `VPCOMPRESSB128MR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb128mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB128MR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb128mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB256MR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb256mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB256MR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb256mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB512MR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb512mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB512MR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb512mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB128RR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB128RR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB128RR_MASKZ` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB256RR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB256RR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB256RR_MASKZ` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB512RR` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB512RR_MASK` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSB512RR_MASKZ` (`VPCOMPRESSB`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSB](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressb512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW128MR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw128mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW128MR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw128mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW256MR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw256mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW256MR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw256mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW512MR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw512mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW512MR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw512mr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512MR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW128RR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW128RR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW128RR_MASKZ` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW256RR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW256RR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW256RR_MASKZ` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW512RR` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW512RR_MASK` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCOMPRESSW512RR_MASKZ` (`VPCOMPRESSW`). Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the destination operand (first operand), based on the active elements determined by the writemask operand. Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VPCOMPRESSW](https://www.felixcloutier.com/x86/VPCOMPRESSB%3AVCOMPRESSW.html)
    fn vpcompressw512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RM` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RM_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RM_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RM` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RM_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RM_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RM` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RM_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RM_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RR` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RR_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB128RR_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RR` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RR_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB256RR_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RR` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RR_MASK` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDB512RR_MASKZ` (`VPEXPANDB`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDB](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandb512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RM` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RM_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RM_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RM` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RM_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RM_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RM` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RM_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RM_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RR` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RR_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW128RR_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RR` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RR_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW256RR_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RR` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RR_MASK` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPEXPANDW512RR_MASKZ` (`VPEXPANDW`). Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.
    /// Reference: [Intel x86 docs for VPEXPANDW](https://www.felixcloutier.com/x86/VPEXPANDB%3AVPEXPANDW.html)
    fn vpexpandw512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPEXPANDW512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPSHLDW128RRRI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW128RRRI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW128RRRI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW128RRMI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW128RRMI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW128RRMI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRRI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRRI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRRI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRMI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRMI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW256RRMI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRRI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRRI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRRI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRMI` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRMI_MASK` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDW512RRMI_MASKZ` (`VPSHLDW`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDW](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldw512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDW512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRRI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRRI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRRI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRMI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRMI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRMI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRBI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRBI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD128RRBI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRRI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRRI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRRI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRMI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRMI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRMI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRBI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRBI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD256RRBI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRRI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRRI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRRI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRMI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRMI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRMI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRBI` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRBI_MASK` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDD512RRBI_MASKZ` (`VPSHLDD`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDD](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldd512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRRI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRRI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRRI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRMI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRMI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRMI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRBI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRBI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ128RRBI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRRI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRRI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRRI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRMI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRMI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRMI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRBI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRBI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ256RRBI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRRI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRRI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRRI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRMI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRMI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRMI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRBI` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRBI_MASK` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDQ512RRBI_MASKZ` (`VPSHLDQ`). Concatenate packed data, extract result shifted to the left by constant value.
    /// Reference: [Intel x86 docs for VPSHLDQ](https://www.felixcloutier.com/x86/VPSHLD.html)
    fn vpshldq512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHLDVW128RRR` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW128RRR_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW128RRR_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW128RRM` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW128RRM_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW128RRM_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRR` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRR_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRR_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRM` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRM_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW256RRM_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRR` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRR_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRR_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRM` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRM_MASK` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVW512RRM_MASKZ` (`VPSHLDVW`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVW](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvw512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVW512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRR` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRR_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRR_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRM` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRM_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRM_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRB` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRB_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD128RRB_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRR` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRR_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRR_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRM` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRM_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRM_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRB` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRB_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD256RRB_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRR` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRR_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRR_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRM` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRM_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRM_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRB` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRB_MASK` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVD512RRB_MASKZ` (`VPSHLDVD`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVD](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRR` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRR_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRR_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRM` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRM_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRM_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRB` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRB_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ128RRB_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRR` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRR_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRR_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRM` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRM_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRM_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRB` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRB_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ256RRB_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRR` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRR_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRR_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRM` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRM_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRM_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRB` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRB_MASK` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHLDVQ512RRB_MASKZ` (`VPSHLDVQ`). Concatenate packed data, extract result shifted to the left by variable value.
    /// Reference: [Intel x86 docs for VPSHLDVQ](https://www.felixcloutier.com/x86/VPSHLDV.html)
    fn vpshldvq512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDW128RRRI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW128RRRI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW128RRRI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW128RRMI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW128RRMI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW128RRMI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRRI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRRI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRRI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRMI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRMI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW256RRMI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRRI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRRI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRRI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRMI` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRMI_MASK` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDW512RRMI_MASKZ` (`VPSHRDW`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDW](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdw512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDW512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRRI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRRI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRRI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRMI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRMI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRMI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRBI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRBI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD128RRBI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRRI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRRI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRRI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRMI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRMI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRMI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRBI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRBI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD256RRBI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRRI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRRI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRRI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRMI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRMI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRMI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRBI` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRBI_MASK` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDD512RRBI_MASKZ` (`VPSHRDD`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDD](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdd512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRRI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRRI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRRI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRMI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRMI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRMI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRBI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRBI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ128RRBI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq128rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRRI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRRI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRRI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRMI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRMI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRMI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRBI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRBI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ256RRBI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq256rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRRI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRRI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrri_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRRI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRRI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrri_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRRI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRMI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRMI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrmi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRMI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRMI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrmi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRMI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRBI` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrbi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRBI_MASK` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrbi_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDQ512RRBI_MASKZ` (`VPSHRDQ`). Concatenate packed data, extract result shifted to the right by constant value.
    /// Reference: [Intel x86 docs for VPSHRDQ](https://www.felixcloutier.com/x86/VPSHRD.html)
    fn vpshrdq512rrbi_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPSHRDVW128RRR` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW128RRR_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW128RRR_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW128RRM` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW128RRM_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW128RRM_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRR` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRR_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRR_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRM` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRM_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW256RRM_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRR` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRR_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRR_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRM` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRM_MASK` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVW512RRM_MASKZ` (`VPSHRDVW`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVW](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvw512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVW512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRR` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRR_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRR_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRM` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRM_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRM_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRB` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRB_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD128RRB_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRR` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRR_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRR_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRM` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRM_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRM_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRB` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRB_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD256RRB_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRR` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRR_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRR_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRM` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRM_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRM_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRB` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRB_MASK` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVD512RRB_MASKZ` (`VPSHRDVD`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVD](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRR` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRR_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRR_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRM` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRM_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRM_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRB` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRB_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ128RRB_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRR` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRR_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRR_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRM` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRM_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRM_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRB` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRB_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ256RRB_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRR` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRR_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRR_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRM` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRM_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRM_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRB` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRB_MASK` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSHRDVQ512RRB_MASKZ` (`VPSHRDVQ`). Concatenate packed data, extract result shifted to the right by variable value.
    /// Reference: [Intel x86 docs for VPSHRDVQ](https://www.felixcloutier.com/x86/VPSHRDV.html)
    fn vpshrdvq512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
