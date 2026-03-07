pub trait X86AVX2Emitter: Emitter {
    /// Emits `VBROADCASTI128RM` (`VBROADCASTI128`). Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    /// Reference: [Intel x86 docs for VBROADCASTI128](https://www.felixcloutier.com/x86/VPBROADCAST.html)
    fn vbroadcasti128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPGATHERDD128RMR` (`VPGATHERDD`). A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERDD](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html)
    fn vpgatherdd128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERDD256RMR` (`VPGATHERDD`). A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERDD](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html)
    fn vpgatherdd256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERDQ128RMR` (`VPGATHERDQ`). A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERDQ](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html)
    fn vpgatherdq128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDQ128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERDQ256RMR` (`VPGATHERDQ`). A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERDQ](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html)
    fn vpgatherdq256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDQ256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERQD128RMR` (`VPGATHERQD`). A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERQD](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html)
    fn vpgatherqd128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERQD256RMR` (`VPGATHERQD`). A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERQD](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html)
    fn vpgatherqd256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERQQ128RMR` (`VPGATHERQQ`). A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERQQ](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html)
    fn vpgatherqq128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQQ128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPGATHERQQ256RMR` (`VPGATHERQQ`). A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    /// Reference: [Intel x86 docs for VPGATHERQQ](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html)
    fn vpgatherqq256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQQ256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERDPS128RMR` (`VGATHERDPS`). The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERDPS](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html)
    fn vgatherdps128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPS128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERDPS256RMR` (`VGATHERDPS`). The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERDPS](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html)
    fn vgatherdps256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPS256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERDPD128RMR` (`VGATHERDPD`). The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERDPD](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html)
    fn vgatherdpd128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERDPD256RMR` (`VGATHERDPD`). The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERDPD](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html)
    fn vgatherdpd256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERQPS128RMR` (`VGATHERQPS`). The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERQPS](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html)
    fn vgatherqps128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPS128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERQPS256RMR` (`VGATHERQPS`). The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERQPS](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html)
    fn vgatherqps256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPS256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERQPD128RMR` (`VGATHERQPD`). The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERQPD](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html)
    fn vgatherqpd128rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VGATHERQPD256RMR` (`VGATHERQPD`). The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    /// Reference: [Intel x86 docs for VGATHERQPD](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html)
    fn vgatherqpd256rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPBLENDD128RRRI` (`VPBLENDD`). Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDD](https://www.felixcloutier.com/x86/VPBLENDD.html)
    fn vpblendd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDD128RRMI` (`VPBLENDD`). Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDD](https://www.felixcloutier.com/x86/VPBLENDD.html)
    fn vpblendd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDD256RRRI` (`VPBLENDD`). Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDD](https://www.felixcloutier.com/x86/VPBLENDD.html)
    fn vpblendd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDD256RRMI` (`VPBLENDD`). Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDD](https://www.felixcloutier.com/x86/VPBLENDD.html)
    fn vpblendd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI128RRRI` (`VINSERTI128`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI128](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTI128RRMI` (`VINSERTI128`). VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    /// Reference: [Intel x86 docs for VINSERTI128](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html)
    fn vinserti128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTI128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VEXTRACTI128RRI` (`VEXTRACTI128`). VEXTRACTI128/VEXTRACTI32x4 and VEXTRACTI64x2 extract 128-bits of doubleword integer values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    /// Reference: [Intel x86 docs for VEXTRACTI128](https://www.felixcloutier.com/x86/VEXTRACTI128%3AVEXTRACTI32x4%3AVEXTRACTI64x2%3AVEXTRACTI32x8%3AVEXTRACTI64x4.html)
    fn vextracti128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VEXTRACTI128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VEXTRACTI128MRI` (`VEXTRACTI128`). VEXTRACTI128/VEXTRACTI32x4 and VEXTRACTI64x2 extract 128-bits of doubleword integer values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    /// Reference: [Intel x86 docs for VEXTRACTI128](https://www.felixcloutier.com/x86/VEXTRACTI128%3AVEXTRACTI32x4%3AVEXTRACTI64x2%3AVEXTRACTI32x8%3AVEXTRACTI64x4.html)
    fn vextracti128mri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VEXTRACTI128MRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERM2I128_256RRRI` (`VPERM2I128`). Permute 128 bit integer data from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    /// Reference: [Intel x86 docs for VPERM2I128](https://www.felixcloutier.com/x86/VPERM2I128.html)
    fn vperm2i128_256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPERM2I128_256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPERM2I128_256RRMI` (`VPERM2I128`). Permute 128 bit integer data from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    /// Reference: [Intel x86 docs for VPERM2I128](https://www.felixcloutier.com/x86/VPERM2I128.html)
    fn vperm2i128_256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPERM2I128_256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
}
