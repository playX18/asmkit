pub trait X86MOVDIR64BEmitter: Emitter {
    /// Emits `MOVDIR64BRM` (`MOVDIR64B`). Moves 64-bytes as direct-store with 64-byte write atomicity from source memory address to destination memory address. The source operand is a normal memory operand. The destination operand is a memory location specified in a general-purpose register. The register content is interpreted as an offset into ES segment without any segment override. In 64-bit mode, the register operand width is 64-bits (32-bits with 67H prefix). Outside of 64-bit mode, the register width is 32-bits when CS.D=1 (16-bits with 67H prefix), and 16-bits when CS.D=0 (32-bits with 67H prefix). MOVDIR64B requires the destination address to be 64-byte aligned. No alignment restriction is enforced for source operand.
    /// Reference: [Intel x86 docs for MOVDIR64B](https://www.felixcloutier.com/x86/MOVDIR64B.html)
    fn movdir64brm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIR64BRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
