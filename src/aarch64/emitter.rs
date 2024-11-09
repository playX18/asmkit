use super::opcodes::*;
use crate::{core::emitter::*, core::operand::*};
pub trait A64EmitterExplicit: Emitter {
    fn udf(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::UDF as _, &[imm16.as_operand()]);
    }
    fn adcw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADCw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn adcsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADCSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sbcw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SBCw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sbcsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SBCSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn adcx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADCx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn adcsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADCSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sbcx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SBCx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sbcsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SBCSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ngcw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NGCw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn ngcsw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NGCSw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn ngcx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NGCx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn ngcsx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NGCSx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn addw_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addw_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsw_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subw_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsw_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addx_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn addsx_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subx_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_uxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_uxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_uxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_uxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_uxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_uxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_uxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_uxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_sxtb(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_sxtb as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_sxth(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_sxth as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_sxtw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_sxtw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn subsx_sxtx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm3: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_sxtx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm3.as_operand(),
            ],
        );
    }
    fn cmnw_uxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_uxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_uxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_uxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_uxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_uxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_uxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_uxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_sxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_sxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_sxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_sxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_sxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_sxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnw_sxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_sxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_uxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_uxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_uxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_uxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_uxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_uxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_uxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_uxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_sxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_sxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_sxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_sxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_sxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_sxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpw_sxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_sxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_uxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_uxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_uxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_uxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_uxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_uxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_uxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_uxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_sxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_sxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_sxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_sxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_sxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_sxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmnx_sxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_sxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_uxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_uxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_uxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_uxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_uxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_uxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_uxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_uxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_sxtb(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_sxtb as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_sxth(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_sxth as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_sxtw(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_sxtw as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn cmpx_sxtx(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm3: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_sxtx as _,
            &[rn.as_operand(), rm.as_operand(), imm3.as_operand()],
        );
    }
    fn addwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn addswi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDSwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn subwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn subswi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBSwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn addxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn addsxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDSxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn subxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn subsxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBSxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn cmnwi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::CMNwi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn cmpwi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::CMPwi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn cmnxi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::CMNxi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn cmpxi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::CMPxi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn mov_spw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MOV_SPw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn mov_spx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MOV_SPx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addsx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDSx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn subsx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBSx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn addw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmnw_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnw_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnw_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNw_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpw_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpw_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpw_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPw_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnx_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnx_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnx_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMNx_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpx_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpx_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmpx_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::CMPx_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn cmnw(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::CMNw as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn cmpw(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::CMPw as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn cmnx(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::CMNx as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn cmpx(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::CMPx as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn negw_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGw_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negw_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGw_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negw_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGw_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsw_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSw_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsw_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSw_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsw_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSw_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negx_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGx_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negx_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGx_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negx_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGx_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsx_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSx_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsx_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSx_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negsx_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::NEGSx_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn negw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NEGw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn negsw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NEGSw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn negx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NEGx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn negsx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::NEGSx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn adr(
        &mut self,
        rd: impl OperandCast,
        immloimmhi: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADR as _,
            &[
                rd.as_operand(),
                immloimmhi.as_operand(),

            ],
        );
    }
    fn adrp(
        &mut self,
        rd: impl OperandCast,
        immloimmhi: impl OperandCast,

    ) {
        return self.emit_n(
            Opcode::ADRP as _,
            &[
                rd.as_operand(),
                immloimmhi.as_operand(),

            ],
        );
    }
    fn andwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn orrwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORRwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn eorwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::EORwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn andswi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDSwi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn andxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn orrxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORRxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn eorxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::EORxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn andsxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDSxi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn tstwi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::TSTwi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn tstxi(&mut self, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::TSTxi as _, &[rn.as_operand(), imm.as_operand()]);
    }
    fn andw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsw_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSw_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsw_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSw_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsw_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSw_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsw_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSw_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn orrx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORRx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn ornx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ORNx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eorx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EORx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn eonx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EONx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsx_lsl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSx_lsl as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsx_lsr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSx_lsr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsx_asr(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSx_asr as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andsx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ANDSx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn bicsx_ror(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BICSx_ror as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
    fn andw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bicw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BICw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orrw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORRw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ornw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORNw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eorw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EORw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eonw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EONw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn andsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bicsw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BICSw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn andx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bicx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BICx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orrx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORRx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ornx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORNx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eorx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EORx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eonx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EONx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn andsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ANDSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bicsx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BICSx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tstw_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTw_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstw_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTw_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstw_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTw_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstw_ror(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTw_ror as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstx_lsl(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTx_lsl as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstx_lsr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTx_lsr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstx_asr(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTx_asr as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstx_ror(&mut self, rn: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::TSTx_ror as _,
            &[rn.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn tstw(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::TSTw as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn tstx(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::TSTx as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn mvnw_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNw_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnw_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNw_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnw_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNw_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnw_ror(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNw_ror as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnx_lsl(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNx_lsl as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnx_lsr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNx_lsr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnx_asr(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNx_asr as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnx_ror(&mut self, rd: impl OperandCast, rm: impl OperandCast, imm6: impl OperandCast) {
        return self.emit_n(
            Opcode::MVNx_ror as _,
            &[rd.as_operand(), rm.as_operand(), imm6.as_operand()],
        );
    }
    fn mvnw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::MVNw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn mvnx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::MVNx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn movw(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::MOVw as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn movx(&mut self, rd: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::MOVx as _, &[rd.as_operand(), rm.as_operand()]);
    }
    fn lslvw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LSLVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn lsrvw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LSRVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn asrvw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ASRVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rorvw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RORVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn lslvx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LSLVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn lsrvx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LSRVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn asrvx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ASRVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rorvx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RORVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn maddw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MADDw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn msubw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MSUBw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn maddx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MADDx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn msubx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MSUBx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn mulw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MULw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mnegw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MNEGw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mulx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MULx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mnegx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MNEGx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaddl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMADDL as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn smsubl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMSUBL as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn umaddl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMADDL as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn umsubl(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMSUBL as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn smull(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smnegl(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMNEGL as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umnegl(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMNEGL as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smulh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULH as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umulh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULH as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bcond(&mut self, bcond: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::BCOND as _,
            &[bcond.as_operand(), imm19.as_operand()],
        );
    }
    fn bccond(&mut self, bcond: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::BCCOND as _,
            &[bcond.as_operand(), imm19.as_operand()],
        );
    }
    fn b(&mut self, imm26: impl OperandCast) {
        return self.emit_n(Opcode::B as _, &[imm26.as_operand()]);
    }
    fn bl(&mut self, imm26: impl OperandCast) {
        return self.emit_n(Opcode::BL as _, &[imm26.as_operand()]);
    }
    fn sbfmw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFMw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn bfmw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFMw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn ubfmw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFMw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn sbfmx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFMx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn bfmx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFMx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn ubfmx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        immr: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFMx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                immr.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn asrwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, immr: impl OperandCast) {
        return self.emit_n(
            Opcode::ASRwi as _,
            &[rd.as_operand(), rn.as_operand(), immr.as_operand()],
        );
    }
    fn lsrwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, immr: impl OperandCast) {
        return self.emit_n(
            Opcode::LSRwi as _,
            &[rd.as_operand(), rn.as_operand(), immr.as_operand()],
        );
    }
    fn asrxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, immr: impl OperandCast) {
        return self.emit_n(
            Opcode::ASRxi as _,
            &[rd.as_operand(), rn.as_operand(), immr.as_operand()],
        );
    }
    fn lsrxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, immr: impl OperandCast) {
        return self.emit_n(
            Opcode::LSRxi as _,
            &[rd.as_operand(), rn.as_operand(), immr.as_operand()],
        );
    }
    fn lslwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::LSLwi as _,
            &[rd.as_operand(), rn.as_operand(), lsl.as_operand()],
        );
    }
    fn lslxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::LSLxi as _,
            &[rd.as_operand(), rn.as_operand(), lsl.as_operand()],
        );
    }
    fn sbfxw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFXw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn bfxilw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFXILw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn ubfxw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFXw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn sbfxx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFXx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn bfxilx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFXILx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn ubfxx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFXx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn sbfizw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFIZw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn bfiw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFIw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn ubfizw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFIZw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn sbfizx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SBFIZx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn bfix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFIx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn ubfizx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        lsb: impl OperandCast,
        width: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UBFIZx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                lsb.as_operand(),
                width.as_operand(),
            ],
        );
    }
    fn bfcw(&mut self, rd: impl OperandCast, lsb: impl OperandCast, width: impl OperandCast) {
        return self.emit_n(
            Opcode::BFCw as _,
            &[rd.as_operand(), lsb.as_operand(), width.as_operand()],
        );
    }
    fn bfcx(&mut self, rd: impl OperandCast, lsb: impl OperandCast, width: impl OperandCast) {
        return self.emit_n(
            Opcode::BFCx as _,
            &[rd.as_operand(), lsb.as_operand(), width.as_operand()],
        );
    }
    fn sxtbw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTBw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtbw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTBw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtbx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTBx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxthw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTHw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxthw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTHw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxthx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTHx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtwx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTWx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn br(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BR as _, &[rn.as_operand()]);
    }
    fn braaz(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BRAAZ as _, &[rn.as_operand()]);
    }
    fn brabz(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BRABZ as _, &[rn.as_operand()]);
    }
    fn blr(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BLR as _, &[rn.as_operand()]);
    }
    fn blraaz(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BLRAAZ as _, &[rn.as_operand()]);
    }
    fn blrabz(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::BLRABZ as _, &[rn.as_operand()]);
    }
    fn ret(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::RET as _, &[rn.as_operand()]);
    }
    fn retaa(&mut self) {
        return self.emit_n(Opcode::RETAA as _, &[]);
    }
    fn retab(&mut self) {
        return self.emit_n(Opcode::RETAB as _, &[]);
    }
    fn braa(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::BRAA as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn brab(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::BRAB as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn blraa(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::BLRAA as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn blrab(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::BLRAB as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn brk(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::BRK as _, &[imm16.as_operand()]);
    }
    fn cbzw(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(Opcode::CBZw as _, &[rt.as_operand(), imm19.as_operand()]);
    }
    fn cbnzw(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(Opcode::CBNZw as _, &[rt.as_operand(), imm19.as_operand()]);
    }
    fn cbzx(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(Opcode::CBZx as _, &[rt.as_operand(), imm19.as_operand()]);
    }
    fn cbnzx(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(Opcode::CBNZx as _, &[rt.as_operand(), imm19.as_operand()]);
    }
    fn tbz(&mut self, rt: impl OperandCast, bit: impl OperandCast, imm14: impl OperandCast) {
        return self.emit_n(
            Opcode::TBZ as _,
            &[rt.as_operand(), bit.as_operand(), imm14.as_operand()],
        );
    }
    fn tbnz(&mut self, rt: impl OperandCast, bit: impl OperandCast, imm14: impl OperandCast) {
        return self.emit_n(
            Opcode::TBNZ as _,
            &[rt.as_operand(), bit.as_operand(), imm14.as_operand()],
        );
    }
    fn ccmnwi(
        &mut self,
        rn: impl OperandCast,
        imm5: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMNwi as _,
            &[
                rn.as_operand(),
                imm5.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmpwi(
        &mut self,
        rn: impl OperandCast,
        imm5: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMPwi as _,
            &[
                rn.as_operand(),
                imm5.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmnxi(
        &mut self,
        rn: impl OperandCast,
        imm5: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMNxi as _,
            &[
                rn.as_operand(),
                imm5.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmpxi(
        &mut self,
        rn: impl OperandCast,
        imm5: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMPxi as _,
            &[
                rn.as_operand(),
                imm5.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmnw(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMNw as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmpw(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMPw as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmnx(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMNx as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn ccmpx(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CCMPx as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn clrex(&mut self) {
        return self.emit_n(Opcode::CLREX as _, &[]);
    }
    fn dsb(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::DSB as _, &[crm.as_operand()]);
    }
    fn dmb(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::DMB as _, &[crm.as_operand()]);
    }
    fn isb(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::ISB as _, &[crm.as_operand()]);
    }
    fn ssbb(&mut self) {
        return self.emit_n(Opcode::SSBB as _, &[]);
    }
    fn hint(&mut self, imm: impl OperandCast) {
        return self.emit_n(Opcode::HINT as _, &[imm.as_operand()]);
    }
    fn nop(&mut self) {
        return self.emit_n(Opcode::NOP as _, &[]);
    }
    fn r#yield(&mut self) {
        return self.emit_n(Opcode::YIELD as _, &[]);
    }
    fn wfe(&mut self) {
        return self.emit_n(Opcode::WFE as _, &[]);
    }
    fn wfi(&mut self) {
        return self.emit_n(Opcode::WFI as _, &[]);
    }
    fn sev(&mut self) {
        return self.emit_n(Opcode::SEV as _, &[]);
    }
    fn sevl(&mut self) {
        return self.emit_n(Opcode::SEVL as _, &[]);
    }
    fn dgh(&mut self) {
        return self.emit_n(Opcode::DGH as _, &[]);
    }
    fn xpaclri(&mut self) {
        return self.emit_n(Opcode::XPACLRI as _, &[]);
    }
    fn pacia1716(&mut self) {
        return self.emit_n(Opcode::PACIA1716 as _, &[]);
    }
    fn pacib1716(&mut self) {
        return self.emit_n(Opcode::PACIB1716 as _, &[]);
    }
    fn autia1716(&mut self) {
        return self.emit_n(Opcode::AUTIA1716 as _, &[]);
    }
    fn autib1716(&mut self) {
        return self.emit_n(Opcode::AUTIB1716 as _, &[]);
    }
    fn esb(&mut self) {
        return self.emit_n(Opcode::ESB as _, &[]);
    }
    fn csdb(&mut self) {
        return self.emit_n(Opcode::CSDB as _, &[]);
    }
    fn clrbhb(&mut self) {
        return self.emit_n(Opcode::CLRBHB as _, &[]);
    }
    fn paciaz(&mut self) {
        return self.emit_n(Opcode::PACIAZ as _, &[]);
    }
    fn paciasp(&mut self) {
        return self.emit_n(Opcode::PACIASP as _, &[]);
    }
    fn pacibz(&mut self) {
        return self.emit_n(Opcode::PACIBZ as _, &[]);
    }
    fn pacibsp(&mut self) {
        return self.emit_n(Opcode::PACIBSP as _, &[]);
    }
    fn autiaz(&mut self) {
        return self.emit_n(Opcode::AUTIAZ as _, &[]);
    }
    fn autiasp(&mut self) {
        return self.emit_n(Opcode::AUTIASP as _, &[]);
    }
    fn autibz(&mut self) {
        return self.emit_n(Opcode::AUTIBZ as _, &[]);
    }
    fn autibsp(&mut self) {
        return self.emit_n(Opcode::AUTIBSP as _, &[]);
    }
    fn bti(&mut self) {
        return self.emit_n(Opcode::BTI as _, &[]);
    }
    fn btic(&mut self) {
        return self.emit_n(Opcode::BTIc as _, &[]);
    }
    fn btij(&mut self) {
        return self.emit_n(Opcode::BTIj as _, &[]);
    }
    fn btijc(&mut self) {
        return self.emit_n(Opcode::BTIjc as _, &[]);
    }
    fn chkfeat(&mut self) {
        return self.emit_n(Opcode::CHKFEAT as _, &[]);
    }
    fn hlt(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::HLT as _, &[imm16.as_operand()]);
    }
    fn svc(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::SVC as _, &[imm16.as_operand()]);
    }
    fn hvc(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::HVC as _, &[imm16.as_operand()]);
    }
    fn smc(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::SMC as _, &[imm16.as_operand()]);
    }
    fn dcps1(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::DCPS1 as _, &[imm16.as_operand()]);
    }
    fn dcps2(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::DCPS2 as _, &[imm16.as_operand()]);
    }
    fn dcps3(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::DCPS3 as _, &[imm16.as_operand()]);
    }
    fn eret(&mut self) {
        return self.emit_n(Opcode::ERET as _, &[]);
    }
    fn eretaa(&mut self) {
        return self.emit_n(Opcode::ERETAA as _, &[]);
    }
    fn eretab(&mut self) {
        return self.emit_n(Opcode::ERETAB as _, &[]);
    }
    fn drps(&mut self) {
        return self.emit_n(Opcode::DRPS as _, &[]);
    }
    fn clzw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clsw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLSw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ctzw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CTZw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cntw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CNTw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn absw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABSw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clzx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clsx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLSx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ctzx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CTZx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cntx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CNTx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn absx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABSx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXwi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn umaxwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXwi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn sminwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINwi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn uminwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINwi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn smaxxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXxi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn umaxxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXxi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn sminxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINxi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn uminxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm8: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINxi as _,
            &[rd.as_operand(), rn.as_operand(), imm8.as_operand()],
        );
    }
    fn smaxw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cselw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSELw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csincw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSINCw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csinvw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSINVw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csnegw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSNEGw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn cselx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSELx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csincx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSINCx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csinvx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSINVx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn csnegx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::CSNEGx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn cincw(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CINCw as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn cinvw(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CINVw as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn cnegw(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CNEGw as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn cincx(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CINCx as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn cinvx(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CINVx as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn cnegx(&mut self, rd: impl OperandCast, rn: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(
            Opcode::CNEGx as _,
            &[rd.as_operand(), rn.as_operand(), cond.as_operand()],
        );
    }
    fn csetw(&mut self, rd: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(Opcode::CSETw as _, &[rd.as_operand(), cond.as_operand()]);
    }
    fn csetmw(&mut self, rd: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(Opcode::CSETMw as _, &[rd.as_operand(), cond.as_operand()]);
    }
    fn csetx(&mut self, rd: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(Opcode::CSETx as _, &[rd.as_operand(), cond.as_operand()]);
    }
    fn csetmx(&mut self, rd: impl OperandCast, cond: impl OperandCast) {
        return self.emit_n(Opcode::CSETMx as _, &[rd.as_operand(), cond.as_operand()]);
    }
    fn extrw(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EXTRw as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn extrx(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imms: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EXTRx as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imms.as_operand(),
            ],
        );
    }
    fn rorwi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imms: impl OperandCast) {
        return self.emit_n(
            Opcode::RORwi as _,
            &[rd.as_operand(), rn.as_operand(), imms.as_operand()],
        );
    }
    fn rorxi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imms: impl OperandCast) {
        return self.emit_n(
            Opcode::RORxi as _,
            &[rd.as_operand(), rn.as_operand(), imms.as_operand()],
        );
    }
    fn movnw(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNw as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnw16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNw16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzw(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZw as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzw16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZw16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkw(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKw as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkw16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKw16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnx(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNx as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnx16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNx16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnx32(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNx32 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnx48(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVNx48 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzx(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZx as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzx16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZx16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzx32(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZx32 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movzx48(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVZx48 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkx(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKx as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkx16(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKx16 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkx32(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKx32 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movkx48(&mut self, rd: impl OperandCast, imm16: impl OperandCast) {
        return self.emit_n(Opcode::MOVKx48 as _, &[rd.as_operand(), imm16.as_operand()]);
    }
    fn movnw_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVNw_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn movzw_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVZw_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn movkw_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVKw_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn movnx_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVNx_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn movzx_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVZx_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn movkx_shift(&mut self, rd: impl OperandCast, imm16: impl OperandCast, hw: impl OperandCast) {
        return self.emit_n(
            Opcode::MOVKx_shift as _,
            &[rd.as_operand(), imm16.as_operand(), hw.as_operand()],
        );
    }
    fn sys(&mut self, sysreg: impl OperandCast, rt: impl OperandCast) {
        return self.emit_n(Opcode::SYS as _, &[sysreg.as_operand(), rt.as_operand()]);
    }
    fn sysl(&mut self, rt: impl OperandCast, sysreg: impl OperandCast) {
        return self.emit_n(Opcode::SYSL as _, &[rt.as_operand(), sysreg.as_operand()]);
    }
    fn at_s1e1r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E1R as _, &[rt.as_operand()]);
    }
    fn at_s1e1w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E1W as _, &[rt.as_operand()]);
    }
    fn at_s1e0r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E0R as _, &[rt.as_operand()]);
    }
    fn at_s1e0w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E0W as _, &[rt.as_operand()]);
    }
    fn at_s1e1rp(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E1RP as _, &[rt.as_operand()]);
    }
    fn at_s1e1wp(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E1WP as _, &[rt.as_operand()]);
    }
    fn at_s1e2r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E2R as _, &[rt.as_operand()]);
    }
    fn at_s1e2w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E2W as _, &[rt.as_operand()]);
    }
    fn at_s12e1r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S12E1R as _, &[rt.as_operand()]);
    }
    fn at_s12e1w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S12E1W as _, &[rt.as_operand()]);
    }
    fn at_s12e0r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S12E0R as _, &[rt.as_operand()]);
    }
    fn at_s12e0w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S12E0W as _, &[rt.as_operand()]);
    }
    fn at_s1e3r(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E3R as _, &[rt.as_operand()]);
    }
    fn at_s1e3w(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::AT_S1E3W as _, &[rt.as_operand()]);
    }
    fn dc_ivac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_IVAC as _, &[rt.as_operand()]);
    }
    fn dc_isw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_ISW as _, &[rt.as_operand()]);
    }
    fn dc_igvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_IGVAC as _, &[rt.as_operand()]);
    }
    fn dc_igsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_IGSW as _, &[rt.as_operand()]);
    }
    fn dc_igdvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_IGDVAC as _, &[rt.as_operand()]);
    }
    fn dc_igdsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_IGDSW as _, &[rt.as_operand()]);
    }
    fn dc_csw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CSW as _, &[rt.as_operand()]);
    }
    fn dc_cgsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGSW as _, &[rt.as_operand()]);
    }
    fn dc_cgdsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGDSW as _, &[rt.as_operand()]);
    }
    fn dc_cisw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CISW as _, &[rt.as_operand()]);
    }
    fn dc_cigsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CIGSW as _, &[rt.as_operand()]);
    }
    fn dc_cigdsw(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CIGDSW as _, &[rt.as_operand()]);
    }
    fn dc_zva(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_ZVA as _, &[rt.as_operand()]);
    }
    fn dc_gva(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_GVA as _, &[rt.as_operand()]);
    }
    fn dc_gzva(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_GZVA as _, &[rt.as_operand()]);
    }
    fn dc_cvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CVAC as _, &[rt.as_operand()]);
    }
    fn dc_cgvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGVAC as _, &[rt.as_operand()]);
    }
    fn dc_cgdvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGDVAC as _, &[rt.as_operand()]);
    }
    fn dc_cvau(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CVAU as _, &[rt.as_operand()]);
    }
    fn dc_cvap(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CVAP as _, &[rt.as_operand()]);
    }
    fn dc_cgvap(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGVAP as _, &[rt.as_operand()]);
    }
    fn dc_cgdvap(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGDVAP as _, &[rt.as_operand()]);
    }
    fn dc_cvadp(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CVADP as _, &[rt.as_operand()]);
    }
    fn dc_cgvadp(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGVADP as _, &[rt.as_operand()]);
    }
    fn dc_cgdvadp(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CGDVADP as _, &[rt.as_operand()]);
    }
    fn dc_civac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CIVAC as _, &[rt.as_operand()]);
    }
    fn dc_cigvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CIGVAC as _, &[rt.as_operand()]);
    }
    fn dc_cigdvac(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::DC_CIGDVAC as _, &[rt.as_operand()]);
    }
    fn ic_ialluis(&mut self) {
        return self.emit_n(Opcode::IC_IALLUIS as _, &[]);
    }
    fn ic_iallu(&mut self) {
        return self.emit_n(Opcode::IC_IALLU as _, &[]);
    }
    fn ic_ivau(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::IC_IVAU as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalle1is(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLE1IS as _, &[]);
    }
    fn tlbi_vae1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_aside1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_ASIDE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vaae1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAAE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vale1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vaale1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAALE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalle1(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLE1 as _, &[]);
    }
    fn tlbi_vae1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_aside1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_ASIDE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_vaae1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAAE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_vale1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_vaale1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAALE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_ipas2e1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2E1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_ipas2le1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2LE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle2is(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE2IS as _, &[]);
    }
    fn tlbi_vae2is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE2IS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle1is(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE1IS as _, &[]);
    }
    fn tlbi_vale2is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE2IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalls12e1is(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLS12E1IS as _, &[]);
    }
    fn tlbi_ipas2e1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2E1 as _, &[rt.as_operand()]);
    }
    fn tlbi_ipas2le1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2LE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_alle2(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE2 as _, &[]);
    }
    fn tlbi_vae2(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE2 as _, &[rt.as_operand()]);
    }
    fn tlbi_alle1(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE1 as _, &[]);
    }
    fn tlbi_vale2(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE2 as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalls12e1(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLS12E1 as _, &[]);
    }
    fn tlbi_alle3is(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE3IS as _, &[]);
    }
    fn tlbi_vae3is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE3IS as _, &[rt.as_operand()]);
    }
    fn tlbi_vale3is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE3IS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle3(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE3 as _, &[]);
    }
    fn tlbi_vae3(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE3 as _, &[rt.as_operand()]);
    }
    fn tlbi_vale3(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE3 as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalle1os(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLE1OS as _, &[]);
    }
    fn tlbi_vae1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_aside1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_ASIDE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_vaae1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAAE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_vale1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_vaale1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAALE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle2os(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE2OS as _, &[]);
    }
    fn tlbi_vae2os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE2OS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle1os(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE1OS as _, &[]);
    }
    fn tlbi_vale2os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE2OS as _, &[rt.as_operand()]);
    }
    fn tlbi_vmalls12e1os(&mut self) {
        return self.emit_n(Opcode::TLBI_VMALLS12E1OS as _, &[]);
    }
    fn tlbi_ipas2e1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2E1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_ipas2le1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_IPAS2LE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_alle3os(&mut self) {
        return self.emit_n(Opcode::TLBI_ALLE3OS as _, &[]);
    }
    fn tlbi_vae3os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VAE3OS as _, &[rt.as_operand()]);
    }
    fn tlbi_vale3os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_VALE3OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaae1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAAE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaale1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAALE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaae1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAAE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaale1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAALE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaae1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAAE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvaale1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAALE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2e1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2E1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2le1is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2LE1IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae2is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE2IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale2is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE2IS as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2e1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2E1 as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2e1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2E1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2le1(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2LE1 as _, &[rt.as_operand()]);
    }
    fn tlbi_ripas2le1os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RIPAS2LE1OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae2os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE2OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale2os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE2OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae2(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE2 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale2(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE2 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae3is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE3IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale3is(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE3IS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae3os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE3OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale3os(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE3OS as _, &[rt.as_operand()]);
    }
    fn tlbi_rvae3(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVAE3 as _, &[rt.as_operand()]);
    }
    fn tlbi_rvale3(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TLBI_RVALE3 as _, &[rt.as_operand()]);
    }
    fn msr(&mut self, sysreg: impl OperandCast, rt: impl OperandCast) {
        return self.emit_n(Opcode::MSR as _, &[sysreg.as_operand(), rt.as_operand()]);
    }
    fn mrs(&mut self, rt: impl OperandCast, sysreg: impl OperandCast) {
        return self.emit_n(Opcode::MRS as _, &[rt.as_operand(), sysreg.as_operand()]);
    }
    fn msri(&mut self, op1: impl OperandCast, op2: impl OperandCast, crm: impl OperandCast) {
        return self.emit_n(
            Opcode::MSRi as _,
            &[op1.as_operand(), op2.as_operand(), crm.as_operand()],
        );
    }
    fn msri_uao(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_UAO as _, &[crm.as_operand()]);
    }
    fn msri_pan(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_PAN as _, &[crm.as_operand()]);
    }
    fn msri_spsel(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_SPSel as _, &[crm.as_operand()]);
    }
    fn msri_ssbs(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_SSBS as _, &[crm.as_operand()]);
    }
    fn msri_dit(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_DIT as _, &[crm.as_operand()]);
    }
    fn msri_tco(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_TCO as _, &[crm.as_operand()]);
    }
    fn msri_daifset(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_DAIFSet as _, &[crm.as_operand()]);
    }
    fn msri_daifclr(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_DAIFClr as _, &[crm.as_operand()]);
    }
    fn msri_allint(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_ALLINT as _, &[crm.as_operand()]);
    }
    fn msri_pm(&mut self, crm: impl OperandCast) {
        return self.emit_n(Opcode::MSRi_PM as _, &[crm.as_operand()]);
    }
    fn rbitw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::RBITw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev16w(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV16w as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32w(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32w as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rbitx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::RBITx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev16x(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV16x as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32x(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32x as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64x(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64x as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn udivw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UDIVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sdivw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SDIVw as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn udivx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UDIVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sdivx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SDIVx as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn stllrb(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLLRB as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stlrb(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLRB as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldlarb(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDLARB as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldarb(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDARB as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stllrh(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLLRH as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stlrh(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLRH as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldlarh(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDLARH as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldarh(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDARH as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stllrw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLLRw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stlrw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLRw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldlarw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDLARw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldarw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDARw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stllrx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLLRx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stlrx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STLRx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldlarx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDLARx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldarx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDARx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stxrbw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STXRBw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stlxrbw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STLXRBw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldxrbw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDXRBw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaxrbw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAXRBw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stxrhw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STXRHw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stlxrhw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STLXRHw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldxrhw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDXRHw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaxrhw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAXRHw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stxrw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STXRw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stlxrw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STLXRw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stxpw(
        &mut self,
        rs: impl OperandCast,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STXPw as _,
            &[
                rs.as_operand(),
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
            ],
        );
    }
    fn stlxpw(
        &mut self,
        rs: impl OperandCast,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STLXPw as _,
            &[
                rs.as_operand(),
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
            ],
        );
    }
    fn ldxrw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDXRw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaxrw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAXRw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldxpw(&mut self, rt: impl OperandCast, rt2: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDXPw as _,
            &[rt.as_operand(), rt2.as_operand(), rn.as_operand()],
        );
    }
    fn ldaxpw(&mut self, rt: impl OperandCast, rt2: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAXPw as _,
            &[rt.as_operand(), rt2.as_operand(), rn.as_operand()],
        );
    }
    fn stxrx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STXRx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stlxrx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::STLXRx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stxpx(
        &mut self,
        rs: impl OperandCast,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STXPx as _,
            &[
                rs.as_operand(),
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
            ],
        );
    }
    fn stlxpx(
        &mut self,
        rs: impl OperandCast,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STLXPx as _,
            &[
                rs.as_operand(),
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
            ],
        );
    }
    fn ldxrx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDXRx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaxrx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAXRx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldxpx(&mut self, rt: impl OperandCast, rt2: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDXPx as _,
            &[rt.as_operand(), rt2.as_operand(), rn.as_operand()],
        );
    }
    fn ldaxpx(&mut self, rt: impl OperandCast, rt2: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAXPx as _,
            &[rt.as_operand(), rt2.as_operand(), rn.as_operand()],
        );
    }
    fn stnpw(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STNPw as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldnpw(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDNPw as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stnpx(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STNPx as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldnpx(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDNPx as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpw_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPw_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpw_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPw_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpw(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPw as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpw(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPw as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpw_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPw_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpw_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPw_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stgp_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STGP_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpsw_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPSW_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stgp(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STGP as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpsw(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPSW as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stgp_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STGP_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpsw_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPSW_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpx_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPx_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpx_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPx_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpx(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPx as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpx(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPx as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpx_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPx_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpx_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPx_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn sturb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strb_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRB_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sttrb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STTRB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strb_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRB_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrb_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRB_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrb_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRB_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldursbx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURSBx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrsbx_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBx_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrsbx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRSBx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrsbx_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBx_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldursbw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURSBw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrsbw_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBw_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrsbw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRSBw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrsbw_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBw_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strh_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRH_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sttrh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STTRH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strh_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRH_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrh_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRH_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrh_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRH_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurshx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURSHx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrshx_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHx_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrshx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRSHx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrshx_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHx_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurshw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURSHw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrshw_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHw_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrshw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRSHw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrshw_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHw_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strw_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRw_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sttrw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STTRw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strw_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRw_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrw_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRw_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrw_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRw_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurswx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURSWx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrswx_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSWx_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrswx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRSWx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrswx_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSWx_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strx_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRx_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sttrx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STTRx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strx_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRx_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrx_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRx_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtrx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTRx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrx_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRx_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn prfum(&mut self, prfop: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::PRFUM as _,
            &[prfop.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldm_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDM_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldtm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDTM as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldm_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDM_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strbu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRBu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrbu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRBu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrsbxu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBxu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrsbwu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBwu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strhu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRHu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrhu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRHu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrshxu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHxu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrshwu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHwu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strwu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRwu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrwu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRwu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrswxu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSWxu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strxu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRxu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrxu_imm(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRxu_imm as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn prfmu_imm(
        &mut self,
        prfop: impl OperandCast,
        rn: impl OperandCast,
        imm12: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::PRFMu_imm as _,
            &[prfop.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strbr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRBr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRBr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRBr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRBr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRBr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRBr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRBr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRBr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbxr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBxr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbxr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBxr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbxr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBxr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbxr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBxr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbwr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBwr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbwr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBwr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbwr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBwr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsbwr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSBwr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRHr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRHr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRHr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRHr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRHr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRHr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRHr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRHr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshxr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHxr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshxr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHxr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshxr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHxr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshxr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHxr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshwr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHwr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshwr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHwr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshwr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHwr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrshwr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSHwr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strwr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRwr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strwr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRwr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strwr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRwr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strwr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRwr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrwr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRwr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrwr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRwr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrwr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRwr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrwr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRwr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrswxr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSWxr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrswxr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSWxr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrswxr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSWxr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrswxr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRSWxr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strxr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRxr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strxr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRxr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strxr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRxr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strxr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRxr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrxr_uxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRxr_uxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrxr_lsl_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRxr_lsl_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrxr_sxtw_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRxr_sxtw_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrxr_sxtx_reg(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRxr_sxtx_reg as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn prfmr_uxtw_reg(
        &mut self,
        prfop: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::PRFMr_uxtw_reg as _,
            &[
                prfop.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn prfmr_lsl_reg(
        &mut self,
        prfop: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::PRFMr_lsl_reg as _,
            &[
                prfop.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn prfmr_sxtw_reg(
        &mut self,
        prfop: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::PRFMr_sxtw_reg as _,
            &[
                prfop.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn prfmr_sxtx_reg(
        &mut self,
        prfop: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::PRFMr_sxtx_reg as _,
            &[
                prfop.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRBr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrbr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRBr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrsbxr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBxr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrsbwr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSBwr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strhr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRHr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrhr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRHr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrshxr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHxr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrshwr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSHwr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strwr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRwr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrwr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRwr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrswxr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSWxr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strxr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRxr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrxr_reg(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRxr_reg as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn prfmr_reg(&mut self, prfop: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PRFMr_reg as _,
            &[prfop.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrw_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRw_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn ldrx_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRx_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn ldrswx_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRSWx_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn prfm_pcrel(&mut self, prfop: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::PRFM_pcrel as _,
            &[prfop.as_operand(), imm19.as_operand()],
        );
    }
    fn stnps(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STNPs as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldnps(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDNPs as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stps_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPs_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldps_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPs_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stps(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPs as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldps(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPs as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stps_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPs_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldps_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPs_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stnpd(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STNPd as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldnpd(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDNPd as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpd_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPd_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpd_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPd_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpd(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPd as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpd(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPd as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpd_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPd_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpd_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPd_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stnpq(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STNPq as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldnpq(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDNPq as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpq_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPq_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpq_post(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPq_post as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpq(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPq as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpq(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPq as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn stpq_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STPq_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn ldpq_pre(
        &mut self,
        rt: impl OperandCast,
        rt2: impl OperandCast,
        rn: impl OperandCast,
        imm7: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDPq_pre as _,
            &[
                rt.as_operand(),
                rt2.as_operand(),
                rn.as_operand(),
                imm7.as_operand(),
            ],
        );
    }
    fn sturb_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURb as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strb_post_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRb_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strb_pre_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRb_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurb_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURb as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrb_post_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRb_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrb_pre_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRb_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturq(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURq as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strq_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRq_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strq_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRq_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurq(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURq as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrq_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRq_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrq_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRq_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturh_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURh as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strh_post_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRh_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strh_pre_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRh_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurh_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURh as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrh_post_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRh_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrh_pre_(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRh_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturs(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURs as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strs_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRs_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strs_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRs_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurs(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURs as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrs_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRs_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrs_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRs_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn sturd(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STURd as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strd_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRd_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strd_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STRd_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldurd(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDURd as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrd_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRd_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldrd_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRd_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn strbu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRbu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrbu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRbu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strqu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRqu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrqu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRqu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strhu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRhu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrhu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRhu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strsu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRsu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrsu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRsu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strdu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::STRdu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn ldrdu(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm12: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRdu as _,
            &[rt.as_operand(), rn.as_operand(), imm12.as_operand()],
        );
    }
    fn strbr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRbr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRbr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRbr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRbr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRbr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRbr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRbr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrbr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRbr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strqr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRqr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strqr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRqr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strqr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRqr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strqr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRqr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrqr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRqr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrqr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRqr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrqr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRqr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrqr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRqr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRhr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRhr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRhr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strhr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRhr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRhr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRhr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRhr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrhr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRhr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strsr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRsr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strsr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRsr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strsr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRsr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strsr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRsr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRsr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRsr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRsr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrsr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRsr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strdr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRdr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strdr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRdr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strdr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRdr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strdr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::STRdr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrdr_uxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRdr_uxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrdr_lsl(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRdr_lsl as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrdr_sxtw(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRdr_sxtw as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn ldrdr_sxtx(
        &mut self,
        rt: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        sc: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LDRdr_sxtx as _,
            &[
                rt.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                sc.as_operand(),
            ],
        );
    }
    fn strbr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRbr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrbr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRbr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strqr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRqr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrqr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRqr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strhr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRhr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrhr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRhr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strsr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRsr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrsr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRsr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn strdr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::STRdr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrdr(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRdr as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ldrs_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRs_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn ldrd_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRd_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn ldrq_pcrel(&mut self, rt: impl OperandCast, imm19: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRq_pcrel as _,
            &[rt.as_operand(), imm19.as_operand()],
        );
    }
    fn st4_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST4_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_4_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_4_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st3_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST3_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_3_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_3_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_1_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_1_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st2_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST2_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st1_2_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST1_2_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_4_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_4_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_3_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_3_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_1_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_1_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1_2_2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1_2_2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st4_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_4_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st3_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_3_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_1_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st2_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st1_2_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_4_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_3_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_1_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1_2_2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn st4_8b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_8b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_4h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_4h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_2s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_2s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_8b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_8b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_4h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_4h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_2s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_2s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_1d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_1d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_8b_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_8b_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_4h_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_4h_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_2s_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_2s_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_8b_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_8b_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_4h_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_4h_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_2s_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_2s_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_1d_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_1d_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_8b_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_8b_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_4h_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_4h_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_2s_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_2s_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_1d_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_1d_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_8b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_8b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_4h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_4h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_2s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_2s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_8b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_8b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_4h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_4h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_2s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_2s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_1d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_1d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_8b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_8b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_4h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_4h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_2s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_2s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_8b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_8b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_4h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_4h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_2s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_2s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_1d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_1d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_8b_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_8b_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_4h_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_4h_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_2s_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_2s_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_8b_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_8b_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_4h_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_4h_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_2s_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_2s_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_1d_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_1d_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_8b_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_8b_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_4h_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_4h_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_2s_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_2s_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_1d_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_1d_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_8b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_8b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_4h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_4h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_2s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_2s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_8b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_8b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_4h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_4h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_2s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_2s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_1d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_1d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_16b_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_16b_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_8h_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_8h_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_4s_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_4s_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st4_2d_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4_2d_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_16b_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_16b_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_8h_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_8h_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_4s_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_4s_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_4_2d_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_4_2d_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_16b_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_16b_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_8h_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_8h_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_4s_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_4s_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st3_2d_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3_2d_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_16b_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_16b_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_8h_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_8h_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_4s_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_4s_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_3_2d_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_3_2d_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_16b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_16b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_8h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_8h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_4s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_4s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_1_2d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_1_2d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_16b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_16b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_8h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_8h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_4s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_4s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st2_2d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2_2d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_16b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_16b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_8h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_8h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_4s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_4s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1_2_2d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1_2_2d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_16b_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_16b_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_8h_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_8h_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_4s_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_4s_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4_2d_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4_2d_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_16b_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_16b_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_8h_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_8h_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_4s_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_4s_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_4_2d_post64(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_4_2d_post64 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_16b_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_16b_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_8h_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_8h_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_4s_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_4s_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3_2d_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3_2d_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_16b_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_16b_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_8h_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_8h_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_4s_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_4s_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_3_2d_post48(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_3_2d_post48 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_16b_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_16b_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_8h_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_8h_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_4s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_4s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_1_2d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_1_2d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_16b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_16b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_8h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_8h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_4s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_4s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2_2d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2_2d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_16b_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_16b_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_8h_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_8h_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_4s_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_4s_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1_2_2d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1_2_2d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn st1b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST1d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST3d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST4d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4b(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4b as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4h(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4h as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4s(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4s as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4d(&mut self, rt: impl OperandCast, elemidx: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4d as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st3b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st1h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st3h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st1s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st1d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st3s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st3d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st2b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st4b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st2h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st4h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st2s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st2d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st4s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st4d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld1b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld3b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld1h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld3h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld1s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld1d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld3s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld3d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld2b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld4b_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4b_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld2h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld4h_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4h_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld2s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld2d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld4s_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4s_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn ld4d_post(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4d_post as _,
            &[
                rt.as_operand(),
                elemidx.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
            ],
        );
    }
    fn st1b_post1(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1b_post1 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3b_post3(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3b_post3 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1h_post2(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1h_post2 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3h_post6(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3h_post6 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1s_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1s_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st1d_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST1d_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3s_post12(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3s_post12 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st3d_post24(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST3d_post24 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2b_post2(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2b_post2 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4b_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4b_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2h_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2h_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4h_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4h_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2s_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2s_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st2d_post16(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST2d_post16 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4s_post16(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4s_post16 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn st4d_post32(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ST4d_post32 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1b_post1(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1b_post1 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3b_post3(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3b_post3 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1h_post2(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1h_post2 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3h_post6(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3h_post6 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1s_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1s_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1d_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD1d_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3s_post12(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3s_post12 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld3d_post24(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD3d_post24 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2b_post2(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2b_post2 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4b_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4b_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2h_post4(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2h_post4 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4h_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4h_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2s_post8(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2s_post8 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld2d_post16(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD2d_post16 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4s_post16(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4s_post16 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld4d_post32(
        &mut self,
        rt: impl OperandCast,
        elemidx: impl OperandCast,
        rn: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::LD4d_post32 as _,
            &[rt.as_operand(), elemidx.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r8b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R8b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r4h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R4h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r2s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R2s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r1d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R1d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD1R2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld3r2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD3R2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld2r2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD2R2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r16b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R16b as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r8h(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R8h as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r4s(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R4s as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld4r2d(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD4R2d as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld1r8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r8b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R8b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r4h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R4h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r2s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R2s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r1d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R1d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld3r2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld2r2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r16b_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R16b_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r8h_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R8h_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r4s_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R4s_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld4r2d_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R2d_post as _,
            &[rt.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ld1r8b_post1(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R8b_post1 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r4h_post2(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R4h_post2 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r2s_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R2s_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r1d_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R1d_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r8b_post3(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R8b_post3 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r4h_post6(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R4h_post6 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r2s_post12(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R2s_post12 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r1d_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R1d_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r8b_post2(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R8b_post2 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r4h_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R4h_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r2s_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R2s_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r1d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R1d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r8b_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R8b_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r4h_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R4h_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r2s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R2s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r1d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R1d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r16b_post1(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R16b_post1 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r8h_post2(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R8h_post2 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r4s_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R4s_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld1r2d_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD1R2d_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r16b_post3(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R16b_post3 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r8h_post6(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R8h_post6 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r4s_post12(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R4s_post12 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld3r2d_post24(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD3R2d_post24 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r16b_post2(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R16b_post2 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r8h_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R8h_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r4s_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R4s_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld2r2d_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD2R2d_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r16b_post4(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R16b_post4 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r8h_post8(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R8h_post8 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r4s_post16(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R4s_post16 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn ld4r2d_post32(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LD4R2d_post32 as _,
            &[rt.as_operand(), rn.as_operand()],
        );
    }
    fn fcvtzsws_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSws_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuws_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUws_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzswd_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSwd_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuwd_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUwd_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzswh_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSwh_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuwh_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUwh_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzsxs_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSxs_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuxs_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUxs_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzsxd_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSxd_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuxd_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUxd_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzsxh_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZSxh_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtzuxh_fix(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        fbits: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCVTZUxh_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfsw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFsw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfsw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFsw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfdw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFdw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfdw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFdw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfhw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFhw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfhw_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFhw_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfsx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFsx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfsx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFsx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfdx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFdx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfdx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFdx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn scvtfhx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFhx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn ucvtfhx_fix(&mut self, rd: impl OperandCast, rn: impl OperandCast, fbits: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFhx_fix as _,
            &[rd.as_operand(), rn.as_operand(), fbits.as_operand()],
        );
    }
    fn fcvtnsws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtasws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpsws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuwd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtaswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauwd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuwd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuwd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuwd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnswh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtaswh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpswh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmswh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzswh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnsxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtasxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpsxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUxs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnsxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtasxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpsxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnsxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtasxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpsxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpuxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfsw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFsw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfsw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFsw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfdw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFdw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfdw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFdw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfhw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFhw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfhw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFhw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfsx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFsx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfsx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFsx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfdx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFdx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfdx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFdx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfhx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFhx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfhx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFhx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovws(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVws as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovwh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVwh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVxd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmov_highxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FMOV_HIGHxd as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fmovxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVxh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovsw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVsw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovhw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVhw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovdx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVdx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmov_highdx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FMOV_HIGHdx as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fmovhx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVhx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fjcvtzswd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FJCVTZSwd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtds(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTds as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvths(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVThs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTsd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn bfcvt(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::BFCVT as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvthd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVThd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTsh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtdh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTdh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintns(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTNs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintms(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTMs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintzs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintas(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTAs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTXs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintis(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTIs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintnd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTNd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintmd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTMd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintzd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintad(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTAd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTXd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintid(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTId as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintnh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTNh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintmh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTMh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintzh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintah(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTAh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTXh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintih(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTIh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32zs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Zs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32xs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Xs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64zs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Zs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64xs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Xs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32zd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Zd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32xd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Xd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64zd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Zd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64xd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Xd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fnegs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEGs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrts(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRTs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fnegd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEGd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrtd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRTd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMOVh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fnegh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEGh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrth(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRTh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmovsi(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOVsi as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmovdi(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOVdi as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmovhi(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOVhi as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmuls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdivs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIVs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadds(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsubs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUBs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmins(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnms(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnms(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fnmuls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FNMULs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmuld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdivd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIVd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsubd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUBd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmind(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fnmuld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FNMULd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdivh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIVh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsubh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUBh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fnmulh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FNMULh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmp_s(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_s as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmp_0s(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_0s as _, &[rn.as_operand()]);
    }
    fn fcmpe_s(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_s as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmpe_0s(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_0s as _, &[rn.as_operand()]);
    }
    fn fcmp_d(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_d as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmp_0d(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_0d as _, &[rn.as_operand()]);
    }
    fn fcmpe_d(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_d as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmpe_0d(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_0d as _, &[rn.as_operand()]);
    }
    fn fcmp_h(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_h as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmp_0h(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMP_0h as _, &[rn.as_operand()]);
    }
    fn fcmpe_h(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_h as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn fcmpe_0h(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCMPE_0h as _, &[rn.as_operand()]);
    }
    fn fccmps(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPs as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fccmpes(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPEs as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fccmpd(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPd as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fccmped(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPEd as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fccmph(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPh as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fccmpeh(
        &mut self,
        rn: impl OperandCast,
        rm: impl OperandCast,
        nzcv: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCCMPEh as _,
            &[
                rn.as_operand(),
                rm.as_operand(),
                nzcv.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fcsels(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCSELs as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fcseld(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCSELd as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fcselh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        cond: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCSELh as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                cond.as_operand(),
            ],
        );
    }
    fn fmadds(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMADDs as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fmsubs(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMSUBs as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmadds(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMADDs as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmsubs(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMSUBs as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fmaddd(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMADDd as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fmsubd(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMSUBd as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmaddd(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMADDd as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmsubd(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMSUBd as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fmaddh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMADDh as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fmsubh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMSUBh as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmaddh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMADDh as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn fnmsubh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FNMSUBh as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn dupb(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUPb as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn duph(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUPh as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dups(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUPs as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dupd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUPd as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP8b as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP4h as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP2s as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP16b as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP8h as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP4s as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::DUP2d as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn dup8bw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP8bw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup4hw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP4hw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup2sw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP2sw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup16bw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP16bw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup8hw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP8hw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup4sw(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP4sw as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn dup2dx(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::DUP2dx as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn insb(
        &mut self,
        rd: impl OperandCast,
        imm5: impl OperandCast,
        rn: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::INSb as _,
            &[
                rd.as_operand(),
                imm5.as_operand(),
                rn.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn insh(
        &mut self,
        rd: impl OperandCast,
        imm5: impl OperandCast,
        rn: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::INSh as _,
            &[
                rd.as_operand(),
                imm5.as_operand(),
                rn.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn inss(
        &mut self,
        rd: impl OperandCast,
        imm5: impl OperandCast,
        rn: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::INSs as _,
            &[
                rd.as_operand(),
                imm5.as_operand(),
                rn.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn insd(
        &mut self,
        rd: impl OperandCast,
        imm5: impl OperandCast,
        rn: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::INSd as _,
            &[
                rd.as_operand(),
                imm5.as_operand(),
                rn.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn insbw(&mut self, rd: impl OperandCast, imm5: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::INSbw as _,
            &[rd.as_operand(), imm5.as_operand(), rn.as_operand()],
        );
    }
    fn inshw(&mut self, rd: impl OperandCast, imm5: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::INShw as _,
            &[rd.as_operand(), imm5.as_operand(), rn.as_operand()],
        );
    }
    fn inssw(&mut self, rd: impl OperandCast, imm5: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::INSsw as _,
            &[rd.as_operand(), imm5.as_operand(), rn.as_operand()],
        );
    }
    fn insdx(&mut self, rd: impl OperandCast, imm5: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::INSdx as _,
            &[rd.as_operand(), imm5.as_operand(), rn.as_operand()],
        );
    }
    fn smovwb(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::SMOVwb as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn umovwb(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::UMOVwb as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn smovwh(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::SMOVwh as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn umovwh(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::UMOVwh as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn umovws(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::UMOVws as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn smovxb(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::SMOVxb as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn smovxh(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::SMOVxh as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn smovxs(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::SMOVxs as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn umovxd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm5: impl OperandCast) {
        return self.emit_n(
            Opcode::UMOVxd as _,
            &[rd.as_operand(), rn.as_operand(), imm5.as_operand()],
        );
    }
    fn tbl1_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL1_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx1_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX1_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl2_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL2_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx2_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX2_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl3_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL3_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx3_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX3_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl4_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL4_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx4_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX4_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl1_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL1_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx1_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX1_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl3_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL3_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx3_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX3_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbl4_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBL4_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn tbx4_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TBX4_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp1_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP1_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn1_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN1_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip1_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP1_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uzp2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UZP2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn trn2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::TRN2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn zip2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ZIP2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ext8b(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EXT8b as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn ext16b(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EXT16b as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm4.as_operand(),
            ],
        );
    }
    fn shadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srhadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRHADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn shsub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHSUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urhadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URHADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uhsub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UHSUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqaddb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADDb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsubb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUBb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqaddh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADDh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsubh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUBh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadds(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADDs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsubs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUBs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqaddd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADDd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsubd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUBd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqaddb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADDb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsubb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUBb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqaddh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADDh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsubh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUBh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadds(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADDs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsubs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUBs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqaddd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADDd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsubd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUBd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqadd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQADD2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqsub2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSUB2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqadd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQADD2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqsub2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSUB2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgtd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGTd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmged(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGEd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhid(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHId as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhsd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHSd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgt2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmge2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhi2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHI2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmhs2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMHS2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtstd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTSTd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeqd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmtst2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMTST2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmeq2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmgtd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CMGTd_zero as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cmeqd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CMEQd_zero as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cmltd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CMLTd_zero as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cmged_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CMGEd_zero as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cmled_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CMLEd_zero as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cmgt8b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT8b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq8b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ8b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt8b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT8b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge8b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE8b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle8b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE8b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt16b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT16b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq16b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ16b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt16b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT16b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmgt2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGT2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmeq2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMEQ2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmlt2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLT2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge16b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE16b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle16b_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE16b_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmge2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMGE2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn cmle2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CMLE2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn suqaddb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADDb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabsb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABSb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqaddh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADDh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadds(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADDs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqaddd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADDd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn absd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqaddb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADDb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqnegb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEGb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqaddh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADDh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqnegh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEGh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadds(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADDs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqnegs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEGs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqaddd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADDd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqnegd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEGd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn negd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEGd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn suqadd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SUQADD2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqabs2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQABS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn abs2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ABS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn usqadd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::USQADD2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqneg2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQNEG2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn neg2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NEG2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp1d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP1d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp1d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP1d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp1d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP1d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp1d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP1d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLP2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sadalp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADALP2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLP2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uadalp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADALP2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cls4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn clz4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CLZ4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cnt8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CNT8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn not8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NOT8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rbit8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::RBIT8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn cnt16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::CNT16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn not16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::NOT16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rbit16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::RBIT16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn mvn8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MVN8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn mvn16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MVN16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev16_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV16_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev16_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV16_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev64_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV64_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn rev32_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::REV32_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtnb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTNb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtnh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTNh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtns(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTNs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtunb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUNb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtnb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTNb as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtunh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUNh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtnh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTNh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtuns(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUNs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtns(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTNs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUN_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN_8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN2_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN2_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn xtn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::XTN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SQXTUN2_16b as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn uqxtn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN2_16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqxtun2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SQXTUN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uqxtn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UQXTN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqshlb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshlb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHLb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshlh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshlh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHLh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshlb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshlb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHLb as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshlh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshlh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHLh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn srshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ushl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn urshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uqrshl2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smax4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAX4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smin4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMIN4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smaxp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMAXP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sminp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMINP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umax4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAX4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umin4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMIN4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umaxp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMAXP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uminp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMINP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saba4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaba4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn add2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADD2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sub2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUB2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmul8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMUL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mul4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MUL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmul16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMUL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mls4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::MLS4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulhh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULHh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulhs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULHs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulhh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULHh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulhs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULHs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulh4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULH4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulh2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULH2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulh4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULH4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulh2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULH2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulh8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULH8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulh4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULH4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulh8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULH8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmulh4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMULH4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnm2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNM2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmax2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAX2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnm2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNM2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmin2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMIN2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnm4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNM4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmax4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAX4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnm2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNM2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmax2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAX2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnm4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNM4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmin4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMIN4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnm2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNM2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmin2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMIN2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeqs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeqd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmges(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facges(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGEs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmged(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facged(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGEd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgts(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgts(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGTs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgtd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgtd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGTd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgts_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTs_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeqs_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQs_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlts_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLTs_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmgtd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTd_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeqd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQd_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmltd_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLTd_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmges_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEs_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmles_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLEs_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmged_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEd_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmled_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLEd_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmge2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facge2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGE2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgt2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeq4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeq2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmge4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facge4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGE4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmge2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facge2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGE2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgt4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgt2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGT2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlt2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLT2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmge2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmle2s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLE2s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmgt4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlt4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLT4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmgt2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlt2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLT2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmge4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmle4s_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLE4s_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmge2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmle2d_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLE2d_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fabs2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fneg2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEG2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrt2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRT2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabs4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabs2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fneg4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEG4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrt4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRT4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fneg2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEG2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrt2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRT2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fabds(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABDs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabdd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABDd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsub2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUB2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDP2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabd2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABD2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADD2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsub4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUB4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsub2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUB2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDP4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDP2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabd4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABD4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabd2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABD2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecpes(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPEs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpxs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPXs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecped(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPEd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpxd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPXd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrtes(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTEs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrted(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTEd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn urecpe2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::URECPE2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpe2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPE2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ursqrte2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::URSQRTE2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrte2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTE2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn urecpe4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::URECPE4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpe4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPE4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpe2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPE2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ursqrte4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::URSQRTE4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrte4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTE4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrte2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTE2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpss(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPSs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecpsd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPSd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrtss(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTSs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrtsd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTSd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecps2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPS2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrts2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTS2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecps4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPS4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecps2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPS2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrts4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTS4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrts2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTS2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulxs(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULXs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulxd(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULXd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulx2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULX2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmul2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMUL2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulx4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULX4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulx2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULX2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmul4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMUL4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmul2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMUL2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmla2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLA2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmls2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLS2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmla2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLA2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmls4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLS4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmls2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLS2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdiv2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIV2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdiv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIV4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdiv2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIV2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulxh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULXh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeqh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecpsh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPSh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrtsh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTSh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgeh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgeh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGEh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabdh(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABDh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgth(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgth(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGTh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnm4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNM4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmla4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLA4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulx4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULX4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeq4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmax4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAX4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecps4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPS4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnm4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNM4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmls4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLS4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsub4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUB4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmin4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMIN4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrts4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTS4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmul4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMUL4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmge4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facge4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGE4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdiv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIV4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabd4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABD4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgt4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGT4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINP4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnm8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNM8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmla8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLA8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fadd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmulx8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMULX8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmeq8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmax8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAX8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frecps8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRECPS8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnm8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNM8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmls8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLS8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fsub8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FSUB8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmin8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMIN8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn frsqrts8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FRSQRTS8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxnmp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXNMP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn faddp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FADDP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmul8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMUL8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmge8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facge8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGE8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmaxp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMAXP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fdiv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FDIV8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminnmp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINNMP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fabd8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FABD8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcmgt8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn facgt8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FACGT8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fminp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMINP8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fcvtnsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtash(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmgth_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGTh_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeqh_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQh_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlth_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLTh_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcvtpsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpeh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPEh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpxh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPXh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnuh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmuh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtauh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmgeh_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGEh_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmleh_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLEh_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcvtpuh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzuh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrteh(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTEh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintn4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTN4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintm4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTM4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtns4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtms4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtas4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtf4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTF4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmgt4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlt4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLT4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fabs4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintp4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTP4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintz4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZ4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtps4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzs4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZS4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpe4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPE4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinta4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTA4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintx4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTX4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnu4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNU4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmu4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMU4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtau4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAU4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtf4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTF4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmge4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmle4h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLE4h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fneg4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEG4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinti4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTI4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpu4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPU4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzu4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZU4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrte4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTE4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrt4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRT4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintn8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTN8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintm8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTM8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtns8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtms8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtas8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtf8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTF8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmgt8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGT8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmeq8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMEQ8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmlt8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLT8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fabs8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FABS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintp8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTP8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintz8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZ8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtps8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzs8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZS8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frecpe8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRECPE8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinta8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTA8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintx8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTX8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnu8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNU8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmu8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMU8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtau8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAU8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtf8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTF8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcmge8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMGE8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fcmle8h_zero(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::FCMLE8h_zero as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn fneg8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FNEG8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinti8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTI8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpu8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPU8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzu8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZU8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frsqrte8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRSQRTE8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fsqrt8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FSQRT8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn and8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::AND8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bic8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orr8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orn8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORN8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eor8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EOR8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bsl8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BSL8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bit8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIT8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bif8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIF8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn and16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::AND16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bic16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orr16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn orn16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ORN16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn eor16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::EOR16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bsl16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BSL16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bit16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIT16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bif16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BIF16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn mov8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MOV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn mov16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::MOV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn saddw2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SADDW2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ssubw2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSUBW2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uaddw2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UADDW2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usubw2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USUBW2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn_8b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN_8b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN_4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addhn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::ADDHN2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subhn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBHN2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn2_16b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN2_16b as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn raddhn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RADDHN2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rsubhn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSUBHN2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabal2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABAL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sabdl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SABDL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabal2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABAL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn uabdl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UABDL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlal2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLAL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smlsl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMLSL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smull2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMULL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlal2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLAL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umlsl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMLSL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn umull2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMULL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlals(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLALs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulls(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULLs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlald(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLALd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmulld(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULLd as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmull_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlal_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLAL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmull_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULL_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmull2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlal2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLAL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmlsl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMLSL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqdmull2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQDMULL2_2d as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmull_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMULL_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmull_1q(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMULL_1q as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmull2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMULL2_8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn pmull2_1q(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PMULL2_1q as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah_scalarh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH_SCALARh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh_scalarh(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH_SCALARh as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah_scalars(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH_SCALARs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh_scalars(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH_SCALARs as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLAH4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh4h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLSH4h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLAH2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLSH2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLAH8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh8h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLSH8h as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlah4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLAH4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sqrdmlsh4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRDMLSH4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sshrd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHRd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssrad(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRAd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshrd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHRd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsrad(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRAd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushrd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHRd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usrad(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRAd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshrd(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHRd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursrad(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRAd as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshr2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHR2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ssra2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSRA2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srshr2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSHR2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn srsra2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRSRA2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushr2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHR2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn usra2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USRA2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn urshr2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSHR2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ursra2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::URSRA2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshldi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlubi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLUbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshlbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshluhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLUhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshlhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlusi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLUsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshlsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshludi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLUdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshldi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHLdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshl2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHL2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshlu2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHLU2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshl2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHL2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrnbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrnbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrnhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrnhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrnsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrnsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrunbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrunbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrnbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrnbi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRNbi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrunhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrunhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrnhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrnhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRNhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrunsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrunsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrnsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrnsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRNsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn_8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN_8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn_4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN_4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn_2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN_2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn rshrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::RSHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun2_16bi(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        imm: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRSHRUN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn2_16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN2_16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqshrun2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQSHRUN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sqrshrun2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SQRSHRUN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqshrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQSHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn uqrshrn2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UQRSHRN2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll_2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL_2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll_2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL_2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sshll2_2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SSHLL2_2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll2_8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL2_8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll2_4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL2_4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ushll2_2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::USHLL2_2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sxtl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sxtl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SXTL2_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uxtl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UXTL2_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn shll_8h_8(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHLL_8h_8 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn shll_4s_16(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHLL_4s_16 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn shll_2d_32(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHLL_2d_32 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn shll2_8h_8(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHLL2_8h_8 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn shll2_4s_16(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SHLL2_4s_16 as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn shll2_2d_32(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SHLL2_2d_32 as _,
            &[rd.as_operand(), rn.as_operand()],
        );
    }
    fn shldi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHLdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sridi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRIdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn slidi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLIdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli8bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI8bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn shl2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHL2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli16bi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI16bi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sri2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SRI2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn sli2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SLI2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtfhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzshi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZShi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtfsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzssi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZSsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtfdi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTFdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzsdi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZSdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtfhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzuhi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZUhi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtfsi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzusi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZUsi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtfdi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTFdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzudi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZUdi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtf4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTF4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzs4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZS4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtf2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTF2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzs2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZS2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtf4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTF4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzu4hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZU4hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtf2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTF2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzu2si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZU2si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtf8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTF8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzs8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZS8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtf4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTF4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzs4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZS4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn scvtf2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::SCVTF2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzs2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZS2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtf8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTF8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzu8hi(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZU8hi as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtf4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTF4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzu4si(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZU4si as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn ucvtf2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::UCVTF2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtzu2di(&mut self, rd: impl OperandCast, rn: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(
            Opcode::FCVTZU2di as _,
            &[rd.as_operand(), rn.as_operand(), imm.as_operand()],
        );
    }
    fn fcvtnss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtass(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtasd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTASd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzss(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzsd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZSd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnus(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmus(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtaus(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnud(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNUd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmud(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMUd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtaud(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAUd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpus(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzus(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpud(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPUd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzud(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZUd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtns2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtms2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtas2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtps2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzs2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZS2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnu2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNU2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmu2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMU2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtau2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAU2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpu2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPU2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzu2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZU2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtns4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtms4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtas4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtns2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtms2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtas2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtps4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzs4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZS4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtps2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzs2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZS2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnu4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNU4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmu4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMU4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtau4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAU4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtnu2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTNU2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtmu2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTMU2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtau2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTAU2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpu4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPU4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzu4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZU4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtpu2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTPU2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtzu2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTZU2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTL_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtl_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTL_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTL2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtl2_2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTL2_2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtfd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTFd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfs(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtfd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTFd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtf2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTF2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtf2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTF2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtf4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTF4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn scvtf2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SCVTF2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtf4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTF4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn ucvtf2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UCVTF2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtxns(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTXNs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn bfcvtn_4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::BFCVTN_4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtxn_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTXN_2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn bfcvtn2_8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::BFCVTN2_8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fcvtxn2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FCVTXN2_4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintn2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTN2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintm2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTM2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintp2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTP2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintz2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZ2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinta2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTA2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintx2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTX2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinti2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTI2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintn4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTN4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintm4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTM4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintn2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTN2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintm2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTM2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintp4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTP4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintz4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZ4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintp2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTP2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintz2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTZ2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinta4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTA4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintx4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTX4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinta2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTA2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frintx2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTX2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinti4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTI4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frinti2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINTI2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32z2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Z2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64z2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Z2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32x2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32X2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64x2s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64X2s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32z4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Z4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64z4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Z4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32z2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32Z2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64z2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64Z2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32x4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32X4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64x4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64X4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint32x2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT32X2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn frint64x2d(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FRINT64X2d as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMAXV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sminv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMINV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMAXV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sminv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMINV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn umaxv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMAXV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uminv8b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMINV8b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn umaxv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMAXV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uminv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMINV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMAXV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sminv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMINV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMAXV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sminv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMINV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn saddlv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SADDLV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn smaxv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMAXV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sminv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SMINV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn addv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ADDV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn umaxv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMAXV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uminv16b(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMINV16b as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn umaxv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMAXV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uminv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMINV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uaddlv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UADDLV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn umaxv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMAXV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn uminv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::UMINV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn faddph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FADDPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminph(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINPh as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn faddps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FADDPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn faddpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FADDPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminps(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINPs as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminpd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINPd as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminv4h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINV4h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminv8h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINV8h as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxnmv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXNMV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fmaxv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMAXV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminnmv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINNMV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn fminv4s(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::FMINV4s as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sqdmulhh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULHh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulhh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULHh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulhs_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULHs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulhs_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULHs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlahh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAHh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlshh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSHh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlahs_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAHs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlshs_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSHs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mul4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MUL4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulh4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULH4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulh4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULH4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mul2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MUL2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulh2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULH2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulh2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULH2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mla4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLA4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mls4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLS4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlah4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlsh4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mla2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLA2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mls2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLS2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlah2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlsh2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mul8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MUL8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulh8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULH8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulh8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULH8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mul4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MUL4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulh4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULH4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmulh4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMULH4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mla8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLA8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mls8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLS8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlah8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlsh8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mla4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLA4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn mls4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::MLS4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlah4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLAH4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqrdmlsh4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQRDMLSH4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlalh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLALh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlslh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSLh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmullh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULLh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlals_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLALs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlsls_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSLs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmulls_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULLs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlal_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLAL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlal_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLAL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlsl_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLSL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlsl_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smull_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMULL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmull_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlal_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLAL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlal_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLAL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlsl_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLSL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlsl_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smull_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMULL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmull_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlal_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLAL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlsl_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLSL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umull_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMULL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlal_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLAL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlsl_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLSL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umull_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMULL_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlal2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLAL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlal2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLAL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlsl2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLSL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlsl2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smull2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMULL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmull2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlal2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLAL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlal2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLAL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smlsl2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMLSL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmlsl2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMLSL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn smull2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SMULL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sqdmull2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SQDMULL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlal2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLAL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlsl2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLSL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umull2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMULL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlal2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLAL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umlsl2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMLSL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn umull2_2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UMULL2_2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlas_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlss_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmuls_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlad_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAd_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsd_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSd_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmuld_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULd_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulxs_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULXs_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulxd_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULXd_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlah_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulxh_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULXh_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmla2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLA2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmls2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLS2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmul2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMUL2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulx2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULX2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmla4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLA4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmls4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLS4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmul4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMUL4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmla2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLA2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmls2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLS2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmul2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMUL2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulx4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULX4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulx2d_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULX2d_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmla4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLA4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmls4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLS4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmul4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMUL4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulx4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULX4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmla8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLA8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmls8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLS8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmul8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMUL8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmulx8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMULX8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sdot2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SDOT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usdot2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USDOT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bfdot2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BFDOT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn udot2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UDOT2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sdot4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SDOT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usdot4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USDOT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn smmla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SMMLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn usmmla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::USMMLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bfmmla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BFMMLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bfdot4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BFDOT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn udot4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UDOT4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn ummla4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::UMMLA4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sudot2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUDOT2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn bfdot2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFDOT2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sdot2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SDOT2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn usdot2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::USDOT2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn udot2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UDOT2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sudot4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUDOT4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn bfdot4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFDOT4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn sdot4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SDOT4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn usdot4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::USDOT4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn udot4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::UDOT4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn bfmlalb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BFMLALB as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bfmlalt(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::BFMLALT as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn bfmlalb_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFMLALB_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn bfmlalt_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BFMLALT_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlal_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLAL_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlsl_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLSL_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlal2_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLAL2_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlsl2_2s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLSL2_2s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlal_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLAL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlsl_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLSL_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlal2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLAL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlsl2_4s(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::FMLSL2_4s as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn fmlal_2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAL_2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsl_2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSL_2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlal2_2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAL2_2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsl2_2s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSL2_2s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlal_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsl_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSL_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlal2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLAL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fmlsl2_4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FMLSL2_4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
            ],
        );
    }
    fn fcmla4h(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA4h as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcadd4h(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCADD4h as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla2s(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA2s as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcadd2s(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCADD2s as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla8h(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA8h as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcadd8h(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCADD8h as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla4s(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA4s as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcadd4s(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCADD4s as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla2d(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA2d as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcadd2d(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCADD2d as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla4h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA4h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla8h_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA8h_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn fcmla4s_elem(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        mrm: impl OperandCast,
        elemidx: impl OperandCast,
        rot: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::FCMLA4s_elem as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                mrm.as_operand(),
                elemidx.as_operand(),
                rot.as_operand(),
            ],
        );
    }
    fn movid(&mut self, rd: impl OperandCast, imm64: impl OperandCast) {
        return self.emit_n(Opcode::MOVId as _, &[rd.as_operand(), imm64.as_operand()]);
    }
    fn movi2d(&mut self, rd: impl OperandCast, imm64: impl OperandCast) {
        return self.emit_n(Opcode::MOVI2d as _, &[rd.as_operand(), imm64.as_operand()]);
    }
    fn orr2si(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR2si as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn bic2si(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC2si as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn orr4si(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR4si as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn bic4si(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC4si as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn orr4hi(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR4hi as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn bic4hi(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC4hi as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn orr8hi(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::ORR8hi as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn bic8hi(&mut self, rd: impl OperandCast, imm: impl OperandCast, lsl: impl OperandCast) {
        return self.emit_n(
            Opcode::BIC8hi as _,
            &[rd.as_operand(), imm.as_operand(), lsl.as_operand()],
        );
    }
    fn fmov2si(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOV2si as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmov4hi(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOV4hi as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmov4si(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOV4si as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmov8hi(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOV8hi as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn fmov2di(&mut self, rd: impl OperandCast, imm: impl OperandCast) {
        return self.emit_n(Opcode::FMOV2di as _, &[rd.as_operand(), imm.as_operand()]);
    }
    fn pacia(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::PACIA as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn pacib(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::PACIB as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn pacda(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::PACDA as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn pacdb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::PACDB as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn autia(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AUTIA as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn autib(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AUTIB as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn autda(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AUTDA as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn autdb(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AUTDB as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn paciza(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::PACIZA as _, &[rd.as_operand()]);
    }
    fn pacizb(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::PACIZB as _, &[rd.as_operand()]);
    }
    fn pacdza(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::PACDZA as _, &[rd.as_operand()]);
    }
    fn pacdzb(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::PACDZB as _, &[rd.as_operand()]);
    }
    fn autiza(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::AUTIZA as _, &[rd.as_operand()]);
    }
    fn autizb(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::AUTIZB as _, &[rd.as_operand()]);
    }
    fn autdza(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::AUTDZA as _, &[rd.as_operand()]);
    }
    fn autdzb(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::AUTDZB as _, &[rd.as_operand()]);
    }
    fn ldraa(&mut self, rt: impl OperandCast, rn: impl OperandCast, simm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRAA as _,
            &[rt.as_operand(), rn.as_operand(), simm9.as_operand()],
        );
    }
    fn ldraa_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, simm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRAA_pre as _,
            &[rt.as_operand(), rn.as_operand(), simm9.as_operand()],
        );
    }
    fn ldrab(&mut self, rt: impl OperandCast, rn: impl OperandCast, simm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRAB as _,
            &[rt.as_operand(), rn.as_operand(), simm9.as_operand()],
        );
    }
    fn ldrab_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, simm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDRAB_pre as _,
            &[rt.as_operand(), rn.as_operand(), simm9.as_operand()],
        );
    }
    fn xpaci(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::XPACI as _, &[rd.as_operand()]);
    }
    fn xpacd(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::XPACD as _, &[rd.as_operand()]);
    }
    fn pacga(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::PACGA as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn casb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caslb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn cash(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caslh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caslw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caslx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casplw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn casplx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn caspalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CASPALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swplb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swph(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swplh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swplw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swplx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn swpalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SWPALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminlb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINLB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminab(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINAB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclralb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeoralb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminalb(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINALB as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldseth(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminlh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINLH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminah(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINAH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclralh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeoralh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminalh(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINALH as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminlw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINLw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclraw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeoraw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminaw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINAw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclralw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeoralw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminalw(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINALw as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminlx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINLx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclrax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeorax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminax(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINAx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldaddalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDADDALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldclralx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDCLRALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldeoralx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDEORALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsetalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSETALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsmaxalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMAXALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldsminalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDSMINALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn ldumaxalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMAXALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn lduminalx(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::LDUMINALx as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn stlurb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STLURB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurb(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURB as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapursbx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURSBx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapursbw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURSBw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stlurh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STLURH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurh(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURH as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurshx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURSHx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurshw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURSHw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stlurw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STLURw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurw(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURw as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurswx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURSWx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stlurx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STLURx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldapurx(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::LDAPURx as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldaprb(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAPRB as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaprh(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAPRH as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaprw(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAPRw as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldaprx(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDAPRx as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn crc32b(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32B as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32H as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32w(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32W as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32cb(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32CB as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32ch(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32CH as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32cw(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32CW as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32x(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32X as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn crc32cx(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::CRC32CX as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn addg(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        uimm6: impl OperandCast,
        uimm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::ADDG as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                uimm6.as_operand(),
                uimm4.as_operand(),
            ],
        );
    }
    fn subg(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        uimm6: impl OperandCast,
        uimm4: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SUBG as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                uimm6.as_operand(),
                uimm4.as_operand(),
            ],
        );
    }
    fn irg(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::IRG as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn gmi(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::GMI as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subp(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBP as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn subps(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SUBPS as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn cmpp(&mut self, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(Opcode::CMPP as _, &[rn.as_operand(), rm.as_operand()]);
    }
    fn stg_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STG_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stg(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STG as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stg_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STG_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stzg_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZG_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stzg(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZG as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stzg_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZG_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn st2g_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2G_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn st2g(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2G as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn st2g_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::ST2G_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stz2g_post(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZ2G_post as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stz2g(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZ2G as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stz2g_pre(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::STZ2G_pre as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn ldg(&mut self, rt: impl OperandCast, rn: impl OperandCast, imm9: impl OperandCast) {
        return self.emit_n(
            Opcode::ldg as _,
            &[rt.as_operand(), rn.as_operand(), imm9.as_operand()],
        );
    }
    fn stzgm(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STZGM as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn stgm(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::STGM as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ldgm(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LDGM as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn cfinv(&mut self) {
        return self.emit_n(Opcode::CFINV as _, &[]);
    }
    fn xaflag(&mut self) {
        return self.emit_n(Opcode::XAFLAG as _, &[]);
    }
    fn axflag(&mut self) {
        return self.emit_n(Opcode::AXFLAG as _, &[]);
    }
    fn rmif(&mut self, rn: impl OperandCast, imm6: impl OperandCast, mask: impl OperandCast) {
        return self.emit_n(
            Opcode::RMIF as _,
            &[rn.as_operand(), imm6.as_operand(), mask.as_operand()],
        );
    }
    fn setf8(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::SETF8 as _, &[rn.as_operand()]);
    }
    fn setf16(&mut self, rn: impl OperandCast) {
        return self.emit_n(Opcode::SETF16 as _, &[rn.as_operand()]);
    }
    fn sb(&mut self) {
        return self.emit_n(Opcode::SB as _, &[]);
    }
    fn tcancel(&mut self, imm16: impl OperandCast) {
        return self.emit_n(Opcode::TCANCEL as _, &[imm16.as_operand()]);
    }
    fn tcommit(&mut self) {
        return self.emit_n(Opcode::TCOMMIT as _, &[]);
    }
    fn tstart(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TSTART as _, &[rt.as_operand()]);
    }
    fn ttest(&mut self, rt: impl OperandCast) {
        return self.emit_n(Opcode::TTEST as _, &[rt.as_operand()]);
    }
    fn wfet(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::WFET as _, &[rd.as_operand()]);
    }
    fn wfit(&mut self, rd: impl OperandCast) {
        return self.emit_n(Opcode::WFIT as _, &[rd.as_operand()]);
    }
    fn st64b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::ST64B as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn ld64b(&mut self, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::LD64B as _, &[rt.as_operand(), rn.as_operand()]);
    }
    fn st64bv0(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST64BV0 as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn st64bv(&mut self, rs: impl OperandCast, rt: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::ST64BV as _,
            &[rs.as_operand(), rt.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfp(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFP as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpwt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfprt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPRT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpwtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfprtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPRTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfptwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfprn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpwtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfprtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPRTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfptrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfpwtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfprtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPRTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfptn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFPTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfm(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFM as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmwt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmrt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMRT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmwtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmrtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMRTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmwtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmrtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMRTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmwtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmrtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMRTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfmtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFMTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfe(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFE as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfewt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfert(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFERT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfet(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFET as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfewn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfewtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfertwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFERTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfetwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFETWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfern(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFERN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfewtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfertrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFERTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfetrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFETRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfen(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfewtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFEWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfertn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFERTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyfetn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYFETN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyp(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYP as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypwt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyprt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPRT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypwtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyprtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPRTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyptwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyprn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypwtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyprtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPRTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyptrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpypwtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyprtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPRTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyptn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYPTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpym(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYM as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymwt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymrt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMRT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymwtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymrtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMRTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymwtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymrtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMRTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymwtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymrtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMRTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpymtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYMTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpye(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYE as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyewt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEWT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyert(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYERT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyet(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYET as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyewn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyewtwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEWTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyertwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYERTWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyetwn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYETWN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyern(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYERN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyewtrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEWTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyertrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYERTRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyetrn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYETRN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyen(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyewtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYEWTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyertn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYERTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn cpyetn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::CPYETN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setp(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETP as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setpt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETPT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setpn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETPN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setptn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETPTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setm(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETM as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setmt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETMT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setmn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETMN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setmtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETMTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn sete(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETE as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setet(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETET as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn seten(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETEN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setetn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETETN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgp(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGP as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgpt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGPT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgpn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGPN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgptn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGPTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgm(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGM as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgmt(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGMT as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgmn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGMN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgmtn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGMTN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setge(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGE as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setget(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGET as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgen(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGEN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn setgetn(&mut self, rd: impl OperandCast, rs: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(
            Opcode::SETGETN as _,
            &[rd.as_operand(), rs.as_operand(), rn.as_operand()],
        );
    }
    fn aese(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AESE as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn aesd(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AESD as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn aesmc(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AESMC as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn aesimc(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::AESIMC as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sha1c(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA1C as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha1p(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA1P as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha1m(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA1M as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha1su0(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA1SU0 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha256h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA256H as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha256h2(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA256H2 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha256su1(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA256SU1 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha1h(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHA1H as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sha1su1(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHA1SU1 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sha256su0(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHA256SU0 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sm3tt1a(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm2: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SM3TT1A as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm2.as_operand(),
            ],
        );
    }
    fn sm3tt1b(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm2: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SM3TT1B as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm2.as_operand(),
            ],
        );
    }
    fn sm3tt2a(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm2: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SM3TT2A as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm2.as_operand(),
            ],
        );
    }
    fn sm3tt2b(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm2: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SM3TT2B as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm2.as_operand(),
            ],
        );
    }
    fn eor3(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::EOR3 as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn bcax(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::BCAX as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn sm3ss1(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        ra: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::SM3SS1 as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                ra.as_operand(),
            ],
        );
    }
    fn sha512su0(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SHA512SU0 as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sm4e(&mut self, rd: impl OperandCast, rn: impl OperandCast) {
        return self.emit_n(Opcode::SM4E as _, &[rd.as_operand(), rn.as_operand()]);
    }
    fn sha512h(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA512H as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha512h2(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA512H2 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sha512su1(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SHA512SU1 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn rax1(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::RAX1 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sm3partw1(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SM3PARTW1 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sm3partw2(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SM3PARTW2 as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn sm4ekey(&mut self, rd: impl OperandCast, rn: impl OperandCast, rm: impl OperandCast) {
        return self.emit_n(
            Opcode::SM4EKEY as _,
            &[rd.as_operand(), rn.as_operand(), rm.as_operand()],
        );
    }
    fn xar(
        &mut self,
        rd: impl OperandCast,
        rn: impl OperandCast,
        rm: impl OperandCast,
        imm6: impl OperandCast,
    ) {
        return self.emit_n(
            Opcode::XAR as _,
            &[
                rd.as_operand(),
                rn.as_operand(),
                rm.as_operand(),
                imm6.as_operand(),
            ],
        );
    }
}
