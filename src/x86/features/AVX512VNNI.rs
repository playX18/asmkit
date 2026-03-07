pub trait X86AVX512VNNIEmitter: Emitter {
    /// Emits `VPDPBUSD128RRR` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRM` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRR` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRM` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRR` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRM` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRR` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRM` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRR` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRM` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRR` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRM` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRR` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRM` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRR` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRM` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRR_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRR_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRM_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRM_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRB` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRB_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD128RRB_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRR_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRR_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRM_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRM_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRB` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRB_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD256RRB_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRR` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRR_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRR_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRM` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRM_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRM_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRB` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRB_MASK` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSD512RRB_MASKZ` (`VPDPBUSD`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    /// Reference: [Intel x86 docs for VPDPBUSD](https://www.felixcloutier.com/x86/VPDPBUSD.html)
    fn vpdpbusd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRR_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRR_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRM_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRM_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRB` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRB_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS128RRB_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRR_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRR_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRM_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRM_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRB` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRB_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS256RRB_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRR` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRR_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRR_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRM` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRM_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRM_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRB` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRB_MASK` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPBUSDS512RRB_MASKZ` (`VPDPBUSDS`). Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPBUSDS](https://www.felixcloutier.com/x86/VPDPBUSDS.html)
    fn vpdpbusds512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPBUSDS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRR_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRR_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRM_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRM_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRB` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRB_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD128RRB_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRR_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRR_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRM_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRM_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRB` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRB_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD256RRB_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRR` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRR_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRR_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRM` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRM_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRM_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRB` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRB_MASK` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSD512RRB_MASKZ` (`VPDPWSSD`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    /// Reference: [Intel x86 docs for VPDPWSSD](https://www.felixcloutier.com/x86/VPDPWSSD.html)
    fn vpdpwssd512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRR_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRR_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRM_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRM_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRB` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRB_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS128RRB_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRR_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRR_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRM_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRM_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRB` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRB_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS256RRB_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRR` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRR_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRR_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRM` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRM_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRM_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRB` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRB_MASK` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPDPWSSDS512RRB_MASKZ` (`VPDPWSSDS`). Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    /// Reference: [Intel x86 docs for VPDPWSSDS](https://www.felixcloutier.com/x86/VPDPWSSDS.html)
    fn vpdpwssds512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPDPWSSDS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
