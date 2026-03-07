pub trait X86CMOVEmitter: Emitter {
    /// Emits `CMOVO16RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC16RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVO16RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC16RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVO32RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC32RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVO32RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC32RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVO64RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC64RR` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVO64RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovo64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVO64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVCC64RM` (`CMOVO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovcc64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVCC64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO16RR` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO16RM` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO32RR` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO32RM` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO64RR` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNO64RM` (`CMOVNO`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNO](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovno64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNO64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC16RR` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC16RM` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC32RR` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC32RM` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC64RR` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVC64RM` (`CMOVC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovc64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVC64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC16RR` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC16RM` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC32RR` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC32RM` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC64RR` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNC64RM` (`CMOVNC`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNC](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnc64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNC64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ16RR` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ16RM` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ32RR` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ32RM` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ64RR` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVZ64RM` (`CMOVZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovz64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVZ64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ16RR` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ16RM` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ32RR` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ32RM` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ64RR` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNZ64RM` (`CMOVNZ`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNZ](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnz64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNZ64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE16RR` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE16RM` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE32RR` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE32RM` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE64RR` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVBE64RM` (`CMOVBE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVBE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovbe64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVBE64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA16RR` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA16RM` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA32RR` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA32RM` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA64RR` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVA64RM` (`CMOVA`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVA](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmova64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVA64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS16RR` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS16RM` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS32RR` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS32RM` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS64RR` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVS64RM` (`CMOVS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovs64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVS64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS16RR` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS16RM` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS32RR` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS32RM` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS64RR` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNS64RM` (`CMOVNS`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNS](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovns64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNS64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP16RR` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP16RM` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP32RR` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP32RM` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP64RR` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVP64RM` (`CMOVP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovp64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVP64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP16RR` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP16RM` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP32RR` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP32RM` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP64RR` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVNP64RM` (`CMOVNP`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVNP](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovnp64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVNP64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL16RR` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL16RM` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL32RR` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL32RM` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL64RR` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVL64RM` (`CMOVL`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVL](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovl64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVL64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE16RR` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE16RM` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE32RR` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE32RM` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE64RR` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVGE64RM` (`CMOVGE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVGE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovge64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVGE64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE16RR` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE16RM` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE32RR` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE32RM` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE64RR` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVLE64RM` (`CMOVLE`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVLE](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovle64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVLE64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG16RR` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG16RM` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG32RR` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG32RM` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG64RR` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMOVG64RM` (`CMOVG`). Each of the CMOVcc instructions performs a move operation if the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) are in a specified state (or condition). A condition code (cc) is associated with each instruction to indicate the condition being tested for. If the condition is not satisfied, a move is not performed and execution continues with the instruction following the CMOVcc instruction.
    /// Reference: [Intel x86 docs for CMOVG](https://www.felixcloutier.com/x86/CMOVcc.html)
    fn cmovg64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMOVG64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
