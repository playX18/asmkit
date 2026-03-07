pub trait X86AVX512VP2INTERSECTEmitter: Emitter {
    /// Emits `VP2INTERSECTD128KRR` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd128krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD128KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD128KRM` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd128krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD128KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD128KRB` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd128krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD128KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD256KRR` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd256krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD256KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD256KRM` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd256krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD256KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD256KRB` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd256krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD256KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD512KRR` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd512krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD512KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD512KRM` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd512krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD512KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTD512KRB` (`VP2INTERSECTD`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTD](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectd512krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTD512KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ128KRR` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq128krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ128KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ128KRM` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq128krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ128KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ128KRB` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq128krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ128KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ256KRR` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq256krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ256KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ256KRM` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq256krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ256KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ256KRB` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq256krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ256KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ512KRR` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq512krr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ512KRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ512KRM` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq512krm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ512KRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VP2INTERSECTQ512KRB` (`VP2INTERSECTQ`). This instruction writes an even/odd pair of mask registers. The mask register destination indicated in the MODRM.REG field is used to form the basis of the register pair. The low bit of that field is masked off (set to zero) to create the first register of the pair.
    /// Reference: [Intel x86 docs for VP2INTERSECTQ](https://www.felixcloutier.com/x86/VP2INTERSECTD%3AVP2INTERSECTQ.html)
    fn vp2intersectq512krb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VP2INTERSECTQ512KRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
