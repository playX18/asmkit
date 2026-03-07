pub trait X86AVXEmitter: Emitter {
    /// Emits `VADDSUBPD128`.
    fn vaddsubpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSUBPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSUBPD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSUBPD128");
        }
    }
    /// Emits `VADDSUBPD256`.
    fn vaddsubpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSUBPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSUBPD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSUBPD256");
        }
    }
    /// Emits `VADDSUBPS128`.
    fn vaddsubps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSUBPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSUBPS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSUBPS128");
        }
    }
    /// Emits `VADDSUBPS256`.
    fn vaddsubps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VADDSUBPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VADDSUBPS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VADDSUBPS256");
        }
    }
    /// Emits `VBLENDPD128`.
    fn vblendpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VBLENDPD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VBLENDPD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDPD128");
        }
    }
    /// Emits `VBLENDPD256`.
    fn vblendpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VBLENDPD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VBLENDPD256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDPD256");
        }
    }
    /// Emits `VBLENDPS128`.
    fn vblendps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VBLENDPS128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VBLENDPS128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDPS128");
        }
    }
    /// Emits `VBLENDPS256`.
    fn vblendps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VBLENDPS256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VBLENDPS256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDPS256");
        }
    }
    /// Emits `VBLENDVPD128`.
    fn vblendvpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VBLENDVPD128RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VBLENDVPD128RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDVPD128");
        }
    }
    /// Emits `VBLENDVPD256`.
    fn vblendvpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VBLENDVPD256RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VBLENDVPD256RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDVPD256");
        }
    }
    /// Emits `VBLENDVPS128`.
    fn vblendvps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VBLENDVPS128RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VBLENDVPS128RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDVPS128");
        }
    }
    /// Emits `VBLENDVPS256`.
    fn vblendvps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VBLENDVPS256RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VBLENDVPS256RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VBLENDVPS256");
        }
    }
    /// Emits `VBROADCASTF128_256`.
    fn vbroadcastf128_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VBROADCASTF128_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VBROADCASTF128_256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VBROADCASTF128_256");
        }
    }
    /// Emits `VCMPPD128`.
    fn vcmppd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPD128");
        }
    }
    /// Emits `VCMPPD256`.
    fn vcmppd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPD256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPD256");
        }
    }
    /// Emits `VCMPPS128`.
    fn vcmpps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPS128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPS128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPS128");
        }
    }
    /// Emits `VCMPPS256`.
    fn vcmpps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPPS256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPPS256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPPS256");
        }
    }
    /// Emits `VCMPSD`.
    fn vcmpsd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPSDRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPSDRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPSD");
        }
    }
    /// Emits `VCMPSS`.
    fn vcmpss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VCMPSSRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VCMPSSRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VCMPSS");
        }
    }
    /// Emits `VDPPD128`.
    fn vdppd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDPPD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDPPD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDPPD128");
        }
    }
    /// Emits `VDPPS128`.
    fn vdpps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDPPS128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDPPS128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDPPS128");
        }
    }
    /// Emits `VDPPS256`.
    fn vdpps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDPPS256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDPPS256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDPPS256");
        }
    }
    /// Emits `VEXTRACTF128`.
    fn vextractf128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VEXTRACTF128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VEXTRACTF128MRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VEXTRACTF128");
        }
    }
    /// Emits `VHADDPD128`.
    fn vhaddpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHADDPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHADDPD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHADDPD128");
        }
    }
    /// Emits `VHADDPD256`.
    fn vhaddpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHADDPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHADDPD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHADDPD256");
        }
    }
    /// Emits `VHADDPS128`.
    fn vhaddps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHADDPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHADDPS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHADDPS128");
        }
    }
    /// Emits `VHADDPS256`.
    fn vhaddps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHADDPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHADDPS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHADDPS256");
        }
    }
    /// Emits `VHSUBPD128`.
    fn vhsubpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHSUBPD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHSUBPD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHSUBPD128");
        }
    }
    /// Emits `VHSUBPD256`.
    fn vhsubpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHSUBPD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHSUBPD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHSUBPD256");
        }
    }
    /// Emits `VHSUBPS128`.
    fn vhsubps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHSUBPS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHSUBPS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHSUBPS128");
        }
    }
    /// Emits `VHSUBPS256`.
    fn vhsubps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VHSUBPS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VHSUBPS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VHSUBPS256");
        }
    }
    /// Emits `VINSERTF128`.
    fn vinsertf128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTF128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTF128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VINSERTF128");
        }
    }
    /// Emits `VLDDQU128RM`.
    fn vlddqu128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VLDDQU128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VLDDQU256RM`.
    fn vlddqu256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VLDDQU256RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VLDMXCSRM`.
    fn vldmxcsr(&mut self,op0: impl OperandCast) -> () {
        self.emit(VLDMXCSRM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMASKMOVDQU128RR`.
    fn vmaskmovdqu128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMASKMOVDQU128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMASKMOVPD128`.
    fn vmaskmovpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMASKMOVPD128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VMASKMOVPD128MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMASKMOVPD128");
        }
    }
    /// Emits `VMASKMOVPD256`.
    fn vmaskmovpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMASKMOVPD256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VMASKMOVPD256MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMASKMOVPD256");
        }
    }
    /// Emits `VMASKMOVPS128`.
    fn vmaskmovps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMASKMOVPS128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VMASKMOVPS128MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMASKMOVPS128");
        }
    }
    /// Emits `VMASKMOVPS256`.
    fn vmaskmovps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VMASKMOVPS256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VMASKMOVPS256MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VMASKMOVPS256");
        }
    }
    /// Emits `VMOVDQA128`.
    fn vmovdqa128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQA128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQA128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQA128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQA128");
        }
    }
    /// Emits `VMOVDQA256`.
    fn vmovdqa256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQA256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQA256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQA256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQA256");
        }
    }
    /// Emits `VMOVDQU128`.
    fn vmovdqu128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU128");
        }
    }
    /// Emits `VMOVDQU256`.
    fn vmovdqu256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU256");
        }
    }
    /// Emits `VMOVMSKPD128RR`.
    fn vmovmskpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPD128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMOVMSKPD256RR`.
    fn vmovmskpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPD256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMOVMSKPS128RR`.
    fn vmovmskps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPS128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMOVMSKPS256RR`.
    fn vmovmskps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPS256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMPSADBW128`.
    fn vmpsadbw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VMPSADBW128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VMPSADBW128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VMPSADBW128");
        }
    }
    /// Emits `VMPSADBW256`.
    fn vmpsadbw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VMPSADBW256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VMPSADBW256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VMPSADBW256");
        }
    }
    /// Emits `VPAND128`.
    fn vpand128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAND128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAND128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAND128");
        }
    }
    /// Emits `VPAND256`.
    fn vpand256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAND256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAND256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAND256");
        }
    }
    /// Emits `VPANDN128`.
    fn vpandn128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPANDN128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPANDN128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPANDN128");
        }
    }
    /// Emits `VPANDN256`.
    fn vpandn256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPANDN256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPANDN256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPANDN256");
        }
    }
    /// Emits `VPBLENDVB128`.
    fn vpblendvb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VPBLENDVB128RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VPBLENDVB128RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDVB128");
        }
    }
    /// Emits `VPBLENDVB256`.
    fn vpblendvb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_vec() {
            self.emit(VPBLENDVB256RRRR, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_vec() {
            self.emit(VPBLENDVB256RRMR, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDVB256");
        }
    }
    /// Emits `VPBLENDW128`.
    fn vpblendw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPBLENDW128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPBLENDW128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDW128");
        }
    }
    /// Emits `VPBLENDW256`.
    fn vpblendw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPBLENDW256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPBLENDW256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDW256");
        }
    }
    /// Emits `VPCMPEQB128`.
    fn vpcmpeqb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQB128");
        }
    }
    /// Emits `VPCMPEQB256`.
    fn vpcmpeqb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQB256");
        }
    }
    /// Emits `VPCMPEQD128`.
    fn vpcmpeqd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQD128");
        }
    }
    /// Emits `VPCMPEQD256`.
    fn vpcmpeqd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQD256");
        }
    }
    /// Emits `VPCMPEQQ128`.
    fn vpcmpeqq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQQ128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQQ128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQQ128");
        }
    }
    /// Emits `VPCMPEQQ256`.
    fn vpcmpeqq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQQ256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQQ256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQQ256");
        }
    }
    /// Emits `VPCMPEQW128`.
    fn vpcmpeqw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQW128");
        }
    }
    /// Emits `VPCMPEQW256`.
    fn vpcmpeqw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPEQW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPEQW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPEQW256");
        }
    }
    /// Emits `VPCMPESTRI`.
    fn vpcmpestri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPCMPESTRIRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPCMPESTRIRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPESTRI");
        }
    }
    /// Emits `VPCMPESTRM`.
    fn vpcmpestrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPCMPESTRMRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPCMPESTRMRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPESTRM");
        }
    }
    /// Emits `VPCMPGTB128`.
    fn vpcmpgtb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTB128");
        }
    }
    /// Emits `VPCMPGTB256`.
    fn vpcmpgtb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTB256");
        }
    }
    /// Emits `VPCMPGTD128`.
    fn vpcmpgtd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTD128");
        }
    }
    /// Emits `VPCMPGTD256`.
    fn vpcmpgtd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTD256");
        }
    }
    /// Emits `VPCMPGTQ128`.
    fn vpcmpgtq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTQ128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTQ128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTQ128");
        }
    }
    /// Emits `VPCMPGTQ256`.
    fn vpcmpgtq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTQ256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTQ256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTQ256");
        }
    }
    /// Emits `VPCMPGTW128`.
    fn vpcmpgtw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTW128");
        }
    }
    /// Emits `VPCMPGTW256`.
    fn vpcmpgtw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPCMPGTW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPCMPGTW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPGTW256");
        }
    }
    /// Emits `VPCMPISTRI`.
    fn vpcmpistri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPCMPISTRIRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPCMPISTRIRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPISTRI");
        }
    }
    /// Emits `VPCMPISTRM`.
    fn vpcmpistrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPCMPISTRMRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPCMPISTRMRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCMPISTRM");
        }
    }
    /// Emits `VPERM2F128_256`.
    fn vperm2f128_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPERM2F128_256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPERM2F128_256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPERM2F128_256");
        }
    }
    /// Emits `VPHADDD128`.
    fn vphaddd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDD128");
        }
    }
    /// Emits `VPHADDD256`.
    fn vphaddd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDD256");
        }
    }
    /// Emits `VPHADDSW128`.
    fn vphaddsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDSW128");
        }
    }
    /// Emits `VPHADDSW256`.
    fn vphaddsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDSW256");
        }
    }
    /// Emits `VPHADDW128`.
    fn vphaddw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDW128");
        }
    }
    /// Emits `VPHADDW256`.
    fn vphaddw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHADDW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHADDW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHADDW256");
        }
    }
    /// Emits `VPHMINPOSUW128`.
    fn vphminposuw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPHMINPOSUW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPHMINPOSUW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHMINPOSUW128");
        }
    }
    /// Emits `VPHSUBD128`.
    fn vphsubd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBD128");
        }
    }
    /// Emits `VPHSUBD256`.
    fn vphsubd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBD256");
        }
    }
    /// Emits `VPHSUBSW128`.
    fn vphsubsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBSW128");
        }
    }
    /// Emits `VPHSUBSW256`.
    fn vphsubsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBSW256");
        }
    }
    /// Emits `VPHSUBW128`.
    fn vphsubw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBW128");
        }
    }
    /// Emits `VPHSUBW256`.
    fn vphsubw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPHSUBW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPHSUBW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPHSUBW256");
        }
    }
    /// Emits `VPMOVMSKB128RR`.
    fn vpmovmskb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVMSKB128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVMSKB256RR`.
    fn vpmovmskb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVMSKB256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOR128`.
    fn vpor128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPOR128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPOR128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOR128");
        }
    }
    /// Emits `VPOR256`.
    fn vpor256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPOR256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPOR256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOR256");
        }
    }
    /// Emits `VPSIGNB128`.
    fn vpsignb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGNB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGNB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGNB128");
        }
    }
    /// Emits `VPSIGNB256`.
    fn vpsignb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGNB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGNB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGNB256");
        }
    }
    /// Emits `VPSIGND128`.
    fn vpsignd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGND128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGND128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGND128");
        }
    }
    /// Emits `VPSIGND256`.
    fn vpsignd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGND256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGND256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGND256");
        }
    }
    /// Emits `VPSIGNW128`.
    fn vpsignw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGNW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGNW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGNW128");
        }
    }
    /// Emits `VPSIGNW256`.
    fn vpsignw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSIGNW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSIGNW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSIGNW256");
        }
    }
    /// Emits `VPTEST128`.
    fn vptest128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPTEST128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPTEST128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTEST128");
        }
    }
    /// Emits `VPTEST256`.
    fn vptest256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPTEST256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPTEST256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTEST256");
        }
    }
    /// Emits `VPXOR128`.
    fn vpxor128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPXOR128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPXOR128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPXOR128");
        }
    }
    /// Emits `VPXOR256`.
    fn vpxor256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPXOR256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPXOR256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPXOR256");
        }
    }
    /// Emits `VRCPPS128`.
    fn vrcpps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPS128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPS128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPS128");
        }
    }
    /// Emits `VRCPPS256`.
    fn vrcpps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRCPPS256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRCPPS256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPPS256");
        }
    }
    /// Emits `VRCPSS`.
    fn vrcpss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRCPSSRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRCPSSRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRCPSS");
        }
    }
    /// Emits `VROUNDPD128`.
    fn vroundpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VROUNDPD128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VROUNDPD128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VROUNDPD128");
        }
    }
    /// Emits `VROUNDPD256`.
    fn vroundpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VROUNDPD256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VROUNDPD256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VROUNDPD256");
        }
    }
    /// Emits `VROUNDPS128`.
    fn vroundps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VROUNDPS128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VROUNDPS128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VROUNDPS128");
        }
    }
    /// Emits `VROUNDPS256`.
    fn vroundps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VROUNDPS256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VROUNDPS256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VROUNDPS256");
        }
    }
    /// Emits `VROUNDSD`.
    fn vroundsd(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VROUNDSDRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VROUNDSDRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VROUNDSD");
        }
    }
    /// Emits `VROUNDSS`.
    fn vroundss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VROUNDSSRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VROUNDSSRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VROUNDSS");
        }
    }
    /// Emits `VRSQRTPS128`.
    fn vrsqrtps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPS128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPS128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPS128");
        }
    }
    /// Emits `VRSQRTPS256`.
    fn vrsqrtps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VRSQRTPS256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VRSQRTPS256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTPS256");
        }
    }
    /// Emits `VRSQRTSS`.
    fn vrsqrtss(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VRSQRTSSRRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VRSQRTSSRRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VRSQRTSS");
        }
    }
    /// Emits `VSTMXCSRM`.
    fn vstmxcsr(&mut self,op0: impl OperandCast) -> () {
        self.emit(VSTMXCSRM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VTESTPD128`.
    fn vtestpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VTESTPD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VTESTPD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VTESTPD128");
        }
    }
    /// Emits `VTESTPD256`.
    fn vtestpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VTESTPD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VTESTPD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VTESTPD256");
        }
    }
    /// Emits `VTESTPS128`.
    fn vtestps128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VTESTPS128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VTESTPS128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VTESTPS128");
        }
    }
    /// Emits `VTESTPS256`.
    fn vtestps256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VTESTPS256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VTESTPS256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VTESTPS256");
        }
    }
    /// Emits `VZEROALL`.
    fn vzeroall(&mut self,) -> () {
        self.emit(VZEROALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VZEROUPPER`.
    fn vzeroupper(&mut self,) -> () {
        self.emit(VZEROUPPER, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
