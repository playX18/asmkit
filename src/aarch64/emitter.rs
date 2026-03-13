#![allow(non_snake_case, non_camel_case_types)]
use super::{assembler::*, instdb::*, operands::*};
use crate::core::globals::CondCode;
use crate::core::operand::*;

pub trait AdcEmitter<T0, T1, T2> {
    fn adc(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AdcsEmitter<T0, T1, T2> {
    fn adcs(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AddEmitter<T0, T1, T2> {
    fn add(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Add4Emitter<T0, T1, T2, T3> {
    fn add_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait AddsEmitter<T0, T1, T2> {
    fn adds(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Adds4Emitter<T0, T1, T2, T3> {
    fn adds_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait AdrEmitter<T0, T1> {
    fn adr(&mut self, op0: T0, op1: T1);
}

pub trait AdrpEmitter<T0, T1> {
    fn adrp(&mut self, op0: T0, op1: T1);
}

pub trait AndEmitter<T0, T1, T2> {
    fn and_(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait And4Emitter<T0, T1, T2, T3> {
    fn and__4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait AndsEmitter<T0, T1, T2> {
    fn ands(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ands4Emitter<T0, T1, T2, T3> {
    fn ands_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait AsrEmitter<T0, T1, T2> {
    fn asr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AsrvEmitter<T0, T1, T2> {
    fn asrv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AtEmitter<T0, T1> {
    fn at(&mut self, op0: T0, op1: T1);
}

pub trait BfcEmitter<T0, T1, T2> {
    fn bfc(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BfiEmitter<T0, T1, T2, T3> {
    fn bfi(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait BfmEmitter<T0, T1, T2, T3> {
    fn bfm(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait BfxilEmitter<T0, T1, T2, T3> {
    fn bfxil(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait BicEmitter<T0, T1, T2> {
    fn bic(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Bic4Emitter<T0, T1, T2, T3> {
    fn bic_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait BicsEmitter<T0, T1, T2> {
    fn bics(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Bics4Emitter<T0, T1, T2, T3> {
    fn bics_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait BrkEmitter<T0> {
    fn brk(&mut self, op0: T0);
}

pub trait CcmnEmitter<T0, T1, T2, T3> {
    fn ccmn(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait CcmpEmitter<T0, T1, T2, T3> {
    fn ccmp(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait CincEmitter<T0, T1, T2> {
    fn cinc(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CinvEmitter<T0, T1, T2> {
    fn cinv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ClrexEmitter<T0> {
    fn clrex(&mut self, op0: T0);
}

pub trait ClsEmitter<T0, T1> {
    fn cls(&mut self, op0: T0, op1: T1);
}

pub trait ClzEmitter<T0, T1> {
    fn clz(&mut self, op0: T0, op1: T1);
}

pub trait CmnEmitter<T0, T1> {
    fn cmn(&mut self, op0: T0, op1: T1);
}

pub trait Cmn3Emitter<T0, T1, T2> {
    fn cmn_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmpEmitter<T0, T1> {
    fn cmp(&mut self, op0: T0, op1: T1);
}

pub trait Cmp3Emitter<T0, T1, T2> {
    fn cmp_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CnegEmitter<T0, T1, T2> {
    fn cneg(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CselEmitter<T0, T1, T2, T3> {
    fn csel(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait CsetEmitter<T0, T1> {
    fn cset(&mut self, op0: T0, op1: T1);
}

pub trait CsetmEmitter<T0, T1> {
    fn csetm(&mut self, op0: T0, op1: T1);
}

pub trait CsincEmitter<T0, T1, T2, T3> {
    fn csinc(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait CsinvEmitter<T0, T1, T2, T3> {
    fn csinv(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait CsnegEmitter<T0, T1, T2, T3> {
    fn csneg(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait DcEmitter<T0, T1> {
    fn dc(&mut self, op0: T0, op1: T1);
}

pub trait DmbEmitter<T0> {
    fn dmb(&mut self, op0: T0);
}

pub trait DsbEmitter<T0> {
    fn dsb(&mut self, op0: T0);
}

pub trait DrpsEmitter {
    fn drps(&mut self);
}

pub trait EonEmitter<T0, T1, T2> {
    fn eon(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Eon4Emitter<T0, T1, T2, T3> {
    fn eon_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait EorEmitter<T0, T1, T2> {
    fn eor(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Eor4Emitter<T0, T1, T2, T3> {
    fn eor_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait EretEmitter {
    fn eret(&mut self);
}

pub trait EsbEmitter {
    fn esb(&mut self);
}

pub trait ExtrEmitter<T0, T1, T2, T3> {
    fn extr(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait HltEmitter<T0> {
    fn hlt(&mut self, op0: T0);
}

pub trait HvcEmitter<T0> {
    fn hvc(&mut self, op0: T0);
}

pub trait IcEmitter<T0, T1> {
    fn ic(&mut self, op0: T0, op1: T1);
}

pub trait IsbEmitter<T0> {
    fn isb(&mut self, op0: T0);
}

pub trait LslEmitter<T0, T1, T2> {
    fn lsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LslvEmitter<T0, T1, T2> {
    fn lslv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LsrEmitter<T0, T1, T2> {
    fn lsr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LsrvEmitter<T0, T1, T2> {
    fn lsrv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MaddEmitter<T0, T1, T2, T3> {
    fn madd(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait MnegEmitter<T0, T1, T2> {
    fn mneg(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MovEmitter<T0, T1> {
    fn mov(&mut self, op0: T0, op1: T1);
}

pub trait MovkEmitter<T0, T1> {
    fn movk(&mut self, op0: T0, op1: T1);
}

pub trait Movk3Emitter<T0, T1, T2> {
    fn movk_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MovnEmitter<T0, T1> {
    fn movn(&mut self, op0: T0, op1: T1);
}

pub trait Movn3Emitter<T0, T1, T2> {
    fn movn_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MovzEmitter<T0, T1> {
    fn movz(&mut self, op0: T0, op1: T1);
}

pub trait Movz3Emitter<T0, T1, T2> {
    fn movz_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MrsEmitter<T0, T1> {
    fn mrs(&mut self, op0: T0, op1: T1);
}

pub trait MsrEmitter<T0, T1> {
    fn msr(&mut self, op0: T0, op1: T1);
}

pub trait MsubEmitter<T0, T1, T2, T3> {
    fn msub(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait MulEmitter<T0, T1, T2> {
    fn mul(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MvnEmitter<T0, T1> {
    fn mvn(&mut self, op0: T0, op1: T1);
}

pub trait Mvn3Emitter<T0, T1, T2> {
    fn mvn_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Mvn_Emitter<T0, T1> {
    fn mvn_(&mut self, op0: T0, op1: T1);
}

pub trait Mvn_3Emitter<T0, T1, T2> {
    fn mvn__3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait NegEmitter<T0, T1> {
    fn neg(&mut self, op0: T0, op1: T1);
}

pub trait Neg3Emitter<T0, T1, T2> {
    fn neg_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait NegsEmitter<T0, T1> {
    fn negs(&mut self, op0: T0, op1: T1);
}

pub trait Negs3Emitter<T0, T1, T2> {
    fn negs_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait NgcEmitter<T0, T1> {
    fn ngc(&mut self, op0: T0, op1: T1);
}

pub trait NgcsEmitter<T0, T1> {
    fn ngcs(&mut self, op0: T0, op1: T1);
}

pub trait OrnEmitter<T0, T1, T2> {
    fn orn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Orn4Emitter<T0, T1, T2, T3> {
    fn orn_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait OrrEmitter<T0, T1, T2> {
    fn orr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Orr4Emitter<T0, T1, T2, T3> {
    fn orr_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait RbitEmitter<T0, T1> {
    fn rbit(&mut self, op0: T0, op1: T1);
}

pub trait RetEmitter<T0> {
    fn ret(&mut self, op0: T0);
}

pub trait RevEmitter<T0, T1> {
    fn rev(&mut self, op0: T0, op1: T1);
}

pub trait Rev16Emitter<T0, T1> {
    fn rev16(&mut self, op0: T0, op1: T1);
}

pub trait Rev32Emitter<T0, T1> {
    fn rev32(&mut self, op0: T0, op1: T1);
}

pub trait Rev64Emitter<T0, T1> {
    fn rev64(&mut self, op0: T0, op1: T1);
}

pub trait RorEmitter<T0, T1, T2> {
    fn ror(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait RorvEmitter<T0, T1, T2> {
    fn rorv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SbcEmitter<T0, T1, T2> {
    fn sbc(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SbcsEmitter<T0, T1, T2> {
    fn sbcs(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SbfizEmitter<T0, T1, T2, T3> {
    fn sbfiz(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SbfmEmitter<T0, T1, T2, T3> {
    fn sbfm(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SbfxEmitter<T0, T1, T2, T3> {
    fn sbfx(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SdivEmitter<T0, T1, T2> {
    fn sdiv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmaddlEmitter<T0, T1, T2, T3> {
    fn smaddl(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SmcEmitter<T0> {
    fn smc(&mut self, op0: T0);
}

pub trait SmneglEmitter<T0, T1, T2> {
    fn smnegl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmsublEmitter<T0, T1, T2, T3> {
    fn smsubl(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SmulhEmitter<T0, T1, T2> {
    fn smulh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmullEmitter<T0, T1, T2> {
    fn smull(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SubEmitter<T0, T1, T2> {
    fn sub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sub4Emitter<T0, T1, T2, T3> {
    fn sub_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SubsEmitter<T0, T1, T2> {
    fn subs(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Subs4Emitter<T0, T1, T2, T3> {
    fn subs_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait SvcEmitter<T0> {
    fn svc(&mut self, op0: T0);
}

pub trait SxtbEmitter<T0, T1> {
    fn sxtb(&mut self, op0: T0, op1: T1);
}

pub trait SxthEmitter<T0, T1> {
    fn sxth(&mut self, op0: T0, op1: T1);
}

pub trait SxtwEmitter<T0, T1> {
    fn sxtw(&mut self, op0: T0, op1: T1);
}

pub trait SysEmitter<T0, T1, T2, T3> {
    fn sys(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Sys5Emitter<T0, T1, T2, T3, T4> {
    fn sys_5(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait TlbiEmitter<T0, T1> {
    fn tlbi(&mut self, op0: T0, op1: T1);
}

pub trait TstEmitter<T0, T1> {
    fn tst(&mut self, op0: T0, op1: T1);
}

pub trait Tst3Emitter<T0, T1, T2> {
    fn tst_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UdivEmitter<T0, T1, T2> {
    fn udiv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UbfizEmitter<T0, T1, T2, T3> {
    fn ubfiz(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait UbfmEmitter<T0, T1, T2, T3> {
    fn ubfm(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait UbfxEmitter<T0, T1, T2, T3> {
    fn ubfx(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait UmaddlEmitter<T0, T1, T2, T3> {
    fn umaddl(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait UmneglEmitter<T0, T1, T2> {
    fn umnegl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmsublEmitter<T0, T1, T2, T3> {
    fn umsubl(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait UmullEmitter<T0, T1, T2> {
    fn umull(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmulhEmitter<T0, T1, T2> {
    fn umulh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UxtbEmitter<T0, T1> {
    fn uxtb(&mut self, op0: T0, op1: T1);
}

pub trait UxthEmitter<T0, T1> {
    fn uxth(&mut self, op0: T0, op1: T1);
}

pub trait CsdbEmitter {
    fn csdb(&mut self);
}

pub trait Dcps1Emitter<T0> {
    fn dcps1(&mut self, op0: T0);
}

pub trait Dcps2Emitter<T0> {
    fn dcps2(&mut self, op0: T0);
}

pub trait Dcps3Emitter<T0> {
    fn dcps3(&mut self, op0: T0);
}

pub trait PssbbEmitter {
    fn pssbb(&mut self);
}

pub trait SsbbEmitter {
    fn ssbb(&mut self);
}

pub trait UdfEmitter<T0> {
    fn udf(&mut self, op0: T0);
}

pub trait BEmitter<T0> {
    fn b(&mut self, op0: T0);
    fn b_eq(&mut self, op0: T0);
    fn b_ne(&mut self, op0: T0);
    fn b_cs(&mut self, op0: T0);
    fn b_hs(&mut self, op0: T0);
    fn b_cc(&mut self, op0: T0);
    fn b_lo(&mut self, op0: T0);
    fn b_mi(&mut self, op0: T0);
    fn b_pl(&mut self, op0: T0);
    fn b_vs(&mut self, op0: T0);
    fn b_vc(&mut self, op0: T0);
    fn b_hi(&mut self, op0: T0);
    fn b_ls(&mut self, op0: T0);
    fn b_ge(&mut self, op0: T0);
    fn b_lt(&mut self, op0: T0);
    fn b_gt(&mut self, op0: T0);
    fn b_le(&mut self, op0: T0);
    fn b_al(&mut self, op0: T0);
}

pub trait BlEmitter<T0> {
    fn bl(&mut self, op0: T0);
}

pub trait BlrEmitter<T0> {
    fn blr(&mut self, op0: T0);
}

pub trait BrEmitter<T0> {
    fn br(&mut self, op0: T0);
}

pub trait CbzEmitter<T0, T1> {
    fn cbz(&mut self, op0: T0, op1: T1);
}

pub trait CbnzEmitter<T0, T1> {
    fn cbnz(&mut self, op0: T0, op1: T1);
}

pub trait TbnzEmitter<T0, T1, T2> {
    fn tbnz(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait TbzEmitter<T0, T1, T2> {
    fn tbz(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasEmitter<T0, T1, T2> {
    fn cas(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasaEmitter<T0, T1, T2> {
    fn casa(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasabEmitter<T0, T1, T2> {
    fn casab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasahEmitter<T0, T1, T2> {
    fn casah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasalEmitter<T0, T1, T2> {
    fn casal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasalbEmitter<T0, T1, T2> {
    fn casalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasalhEmitter<T0, T1, T2> {
    fn casalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CasbEmitter<T0, T1, T2> {
    fn casb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CashEmitter<T0, T1, T2> {
    fn cash(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CaslEmitter<T0, T1, T2> {
    fn casl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CaslbEmitter<T0, T1, T2> {
    fn caslb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CaslhEmitter<T0, T1, T2> {
    fn caslh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CaspEmitter<T0, T1, T2, T3, T4> {
    fn casp(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait CaspaEmitter<T0, T1, T2, T3, T4> {
    fn caspa(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait CaspalEmitter<T0, T1, T2, T3, T4> {
    fn caspal(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait CasplEmitter<T0, T1, T2, T3, T4> {
    fn caspl(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait LdaddEmitter<T0, T1, T2> {
    fn ldadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddaEmitter<T0, T1, T2> {
    fn ldadda(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddabEmitter<T0, T1, T2> {
    fn ldaddab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddahEmitter<T0, T1, T2> {
    fn ldaddah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddalEmitter<T0, T1, T2> {
    fn ldaddal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddalbEmitter<T0, T1, T2> {
    fn ldaddalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddalhEmitter<T0, T1, T2> {
    fn ldaddalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddbEmitter<T0, T1, T2> {
    fn ldaddb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddhEmitter<T0, T1, T2> {
    fn ldaddh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddlEmitter<T0, T1, T2> {
    fn ldaddl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddlbEmitter<T0, T1, T2> {
    fn ldaddlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaddlhEmitter<T0, T1, T2> {
    fn ldaddlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdarEmitter<T0, T1> {
    fn ldar(&mut self, op0: T0, op1: T1);
}

pub trait LdarbEmitter<T0, T1> {
    fn ldarb(&mut self, op0: T0, op1: T1);
}

pub trait LdarhEmitter<T0, T1> {
    fn ldarh(&mut self, op0: T0, op1: T1);
}

pub trait LdaxrEmitter<T0, T1> {
    fn ldaxr(&mut self, op0: T0, op1: T1);
}

pub trait LdaxrbEmitter<T0, T1> {
    fn ldaxrb(&mut self, op0: T0, op1: T1);
}

pub trait LdaxrhEmitter<T0, T1> {
    fn ldaxrh(&mut self, op0: T0, op1: T1);
}

pub trait LdclrEmitter<T0, T1, T2> {
    fn ldclr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclraEmitter<T0, T1, T2> {
    fn ldclra(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrabEmitter<T0, T1, T2> {
    fn ldclrab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrahEmitter<T0, T1, T2> {
    fn ldclrah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclralEmitter<T0, T1, T2> {
    fn ldclral(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclralbEmitter<T0, T1, T2> {
    fn ldclralb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclralhEmitter<T0, T1, T2> {
    fn ldclralh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrbEmitter<T0, T1, T2> {
    fn ldclrb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrhEmitter<T0, T1, T2> {
    fn ldclrh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrlEmitter<T0, T1, T2> {
    fn ldclrl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrlbEmitter<T0, T1, T2> {
    fn ldclrlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdclrlhEmitter<T0, T1, T2> {
    fn ldclrlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorEmitter<T0, T1, T2> {
    fn ldeor(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeoraEmitter<T0, T1, T2> {
    fn ldeora(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorabEmitter<T0, T1, T2> {
    fn ldeorab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorahEmitter<T0, T1, T2> {
    fn ldeorah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeoralEmitter<T0, T1, T2> {
    fn ldeoral(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeoralbEmitter<T0, T1, T2> {
    fn ldeoralb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeoralhEmitter<T0, T1, T2> {
    fn ldeoralh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorbEmitter<T0, T1, T2> {
    fn ldeorb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorhEmitter<T0, T1, T2> {
    fn ldeorh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorlEmitter<T0, T1, T2> {
    fn ldeorl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorlbEmitter<T0, T1, T2> {
    fn ldeorlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdeorlhEmitter<T0, T1, T2> {
    fn ldeorlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdlarEmitter<T0, T1> {
    fn ldlar(&mut self, op0: T0, op1: T1);
}

pub trait LdlarbEmitter<T0, T1> {
    fn ldlarb(&mut self, op0: T0, op1: T1);
}

pub trait LdlarhEmitter<T0, T1> {
    fn ldlarh(&mut self, op0: T0, op1: T1);
}

pub trait LdnpEmitter<T0, T1, T2> {
    fn ldnp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdpEmitter<T0, T1, T2> {
    fn ldp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdpswEmitter<T0, T1, T2> {
    fn ldpsw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdrEmitter<T0, T1> {
    fn ldr(&mut self, op0: T0, op1: T1);
}

pub trait LdrbEmitter<T0, T1> {
    fn ldrb(&mut self, op0: T0, op1: T1);
}

pub trait LdrhEmitter<T0, T1> {
    fn ldrh(&mut self, op0: T0, op1: T1);
}

pub trait LdrsbEmitter<T0, T1> {
    fn ldrsb(&mut self, op0: T0, op1: T1);
}

pub trait LdrshEmitter<T0, T1> {
    fn ldrsh(&mut self, op0: T0, op1: T1);
}

pub trait LdrswEmitter<T0, T1> {
    fn ldrsw(&mut self, op0: T0, op1: T1);
}

pub trait LdsetEmitter<T0, T1, T2> {
    fn ldset(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetaEmitter<T0, T1, T2> {
    fn ldseta(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetabEmitter<T0, T1, T2> {
    fn ldsetab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetahEmitter<T0, T1, T2> {
    fn ldsetah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetalEmitter<T0, T1, T2> {
    fn ldsetal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetalbEmitter<T0, T1, T2> {
    fn ldsetalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetalhEmitter<T0, T1, T2> {
    fn ldsetalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetbEmitter<T0, T1, T2> {
    fn ldsetb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsethEmitter<T0, T1, T2> {
    fn ldseth(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetlEmitter<T0, T1, T2> {
    fn ldsetl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetlbEmitter<T0, T1, T2> {
    fn ldsetlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsetlhEmitter<T0, T1, T2> {
    fn ldsetlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxEmitter<T0, T1, T2> {
    fn ldsmax(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxaEmitter<T0, T1, T2> {
    fn ldsmaxa(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxabEmitter<T0, T1, T2> {
    fn ldsmaxab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxahEmitter<T0, T1, T2> {
    fn ldsmaxah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxalEmitter<T0, T1, T2> {
    fn ldsmaxal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxalbEmitter<T0, T1, T2> {
    fn ldsmaxalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxalhEmitter<T0, T1, T2> {
    fn ldsmaxalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxbEmitter<T0, T1, T2> {
    fn ldsmaxb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxhEmitter<T0, T1, T2> {
    fn ldsmaxh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxlEmitter<T0, T1, T2> {
    fn ldsmaxl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxlbEmitter<T0, T1, T2> {
    fn ldsmaxlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsmaxlhEmitter<T0, T1, T2> {
    fn ldsmaxlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminEmitter<T0, T1, T2> {
    fn ldsmin(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminaEmitter<T0, T1, T2> {
    fn ldsmina(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminabEmitter<T0, T1, T2> {
    fn ldsminab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminahEmitter<T0, T1, T2> {
    fn ldsminah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminalEmitter<T0, T1, T2> {
    fn ldsminal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminalbEmitter<T0, T1, T2> {
    fn ldsminalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminalhEmitter<T0, T1, T2> {
    fn ldsminalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminbEmitter<T0, T1, T2> {
    fn ldsminb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminhEmitter<T0, T1, T2> {
    fn ldsminh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminlEmitter<T0, T1, T2> {
    fn ldsminl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminlbEmitter<T0, T1, T2> {
    fn ldsminlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdsminlhEmitter<T0, T1, T2> {
    fn ldsminlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdtrEmitter<T0, T1> {
    fn ldtr(&mut self, op0: T0, op1: T1);
}

pub trait LdtrbEmitter<T0, T1> {
    fn ldtrb(&mut self, op0: T0, op1: T1);
}

pub trait LdtrhEmitter<T0, T1> {
    fn ldtrh(&mut self, op0: T0, op1: T1);
}

pub trait LdtrsbEmitter<T0, T1> {
    fn ldtrsb(&mut self, op0: T0, op1: T1);
}

pub trait LdtrshEmitter<T0, T1> {
    fn ldtrsh(&mut self, op0: T0, op1: T1);
}

pub trait LdtrswEmitter<T0, T1> {
    fn ldtrsw(&mut self, op0: T0, op1: T1);
}

pub trait LdumaxEmitter<T0, T1, T2> {
    fn ldumax(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxaEmitter<T0, T1, T2> {
    fn ldumaxa(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxabEmitter<T0, T1, T2> {
    fn ldumaxab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxahEmitter<T0, T1, T2> {
    fn ldumaxah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxalEmitter<T0, T1, T2> {
    fn ldumaxal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxalbEmitter<T0, T1, T2> {
    fn ldumaxalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxalhEmitter<T0, T1, T2> {
    fn ldumaxalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxbEmitter<T0, T1, T2> {
    fn ldumaxb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxhEmitter<T0, T1, T2> {
    fn ldumaxh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxlEmitter<T0, T1, T2> {
    fn ldumaxl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxlbEmitter<T0, T1, T2> {
    fn ldumaxlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdumaxlhEmitter<T0, T1, T2> {
    fn ldumaxlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminEmitter<T0, T1, T2> {
    fn ldumin(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminaEmitter<T0, T1, T2> {
    fn ldumina(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminabEmitter<T0, T1, T2> {
    fn lduminab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminahEmitter<T0, T1, T2> {
    fn lduminah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminalEmitter<T0, T1, T2> {
    fn lduminal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminalbEmitter<T0, T1, T2> {
    fn lduminalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminalhEmitter<T0, T1, T2> {
    fn lduminalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminbEmitter<T0, T1, T2> {
    fn lduminb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminhEmitter<T0, T1, T2> {
    fn lduminh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminlEmitter<T0, T1, T2> {
    fn lduminl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminlbEmitter<T0, T1, T2> {
    fn lduminlb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LduminlhEmitter<T0, T1, T2> {
    fn lduminlh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdurEmitter<T0, T1> {
    fn ldur(&mut self, op0: T0, op1: T1);
}

pub trait LdurbEmitter<T0, T1> {
    fn ldurb(&mut self, op0: T0, op1: T1);
}

pub trait LdurhEmitter<T0, T1> {
    fn ldurh(&mut self, op0: T0, op1: T1);
}

pub trait LdursbEmitter<T0, T1> {
    fn ldursb(&mut self, op0: T0, op1: T1);
}

pub trait LdurshEmitter<T0, T1> {
    fn ldursh(&mut self, op0: T0, op1: T1);
}

pub trait LdurswEmitter<T0, T1> {
    fn ldursw(&mut self, op0: T0, op1: T1);
}

pub trait LdxpEmitter<T0, T1, T2> {
    fn ldxp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdaxpEmitter<T0, T1, T2> {
    fn ldaxp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait LdxrEmitter<T0, T1> {
    fn ldxr(&mut self, op0: T0, op1: T1);
}

pub trait LdxrbEmitter<T0, T1> {
    fn ldxrb(&mut self, op0: T0, op1: T1);
}

pub trait LdxrhEmitter<T0, T1> {
    fn ldxrh(&mut self, op0: T0, op1: T1);
}

pub trait PrfmEmitter<T0, T1> {
    fn prfm(&mut self, op0: T0, op1: T1);
}

pub trait StaddEmitter<T0, T1> {
    fn stadd(&mut self, op0: T0, op1: T1);
}

pub trait StaddbEmitter<T0, T1> {
    fn staddb(&mut self, op0: T0, op1: T1);
}

pub trait StaddhEmitter<T0, T1> {
    fn staddh(&mut self, op0: T0, op1: T1);
}

pub trait StaddlEmitter<T0, T1> {
    fn staddl(&mut self, op0: T0, op1: T1);
}

pub trait StaddlbEmitter<T0, T1> {
    fn staddlb(&mut self, op0: T0, op1: T1);
}

pub trait StaddlhEmitter<T0, T1> {
    fn staddlh(&mut self, op0: T0, op1: T1);
}

pub trait StclrEmitter<T0, T1> {
    fn stclr(&mut self, op0: T0, op1: T1);
}

pub trait StclrbEmitter<T0, T1> {
    fn stclrb(&mut self, op0: T0, op1: T1);
}

pub trait StclrhEmitter<T0, T1> {
    fn stclrh(&mut self, op0: T0, op1: T1);
}

pub trait StclrlEmitter<T0, T1> {
    fn stclrl(&mut self, op0: T0, op1: T1);
}

pub trait StclrlbEmitter<T0, T1> {
    fn stclrlb(&mut self, op0: T0, op1: T1);
}

pub trait StclrlhEmitter<T0, T1> {
    fn stclrlh(&mut self, op0: T0, op1: T1);
}

pub trait SteorEmitter<T0, T1> {
    fn steor(&mut self, op0: T0, op1: T1);
}

pub trait SteorbEmitter<T0, T1> {
    fn steorb(&mut self, op0: T0, op1: T1);
}

pub trait SteorhEmitter<T0, T1> {
    fn steorh(&mut self, op0: T0, op1: T1);
}

pub trait SteorlEmitter<T0, T1> {
    fn steorl(&mut self, op0: T0, op1: T1);
}

pub trait SteorlbEmitter<T0, T1> {
    fn steorlb(&mut self, op0: T0, op1: T1);
}

pub trait SteorlhEmitter<T0, T1> {
    fn steorlh(&mut self, op0: T0, op1: T1);
}

pub trait StllrEmitter<T0, T1> {
    fn stllr(&mut self, op0: T0, op1: T1);
}

pub trait StllrbEmitter<T0, T1> {
    fn stllrb(&mut self, op0: T0, op1: T1);
}

pub trait StllrhEmitter<T0, T1> {
    fn stllrh(&mut self, op0: T0, op1: T1);
}

pub trait StlrEmitter<T0, T1> {
    fn stlr(&mut self, op0: T0, op1: T1);
}

pub trait StlrbEmitter<T0, T1> {
    fn stlrb(&mut self, op0: T0, op1: T1);
}

pub trait StlrhEmitter<T0, T1> {
    fn stlrh(&mut self, op0: T0, op1: T1);
}

pub trait StlxrEmitter<T0, T1, T2> {
    fn stlxr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StlxrbEmitter<T0, T1, T2> {
    fn stlxrb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StlxrhEmitter<T0, T1, T2> {
    fn stlxrh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StnpEmitter<T0, T1, T2> {
    fn stnp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StpEmitter<T0, T1, T2> {
    fn stp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StrEmitter<T0, T1> {
    fn str(&mut self, op0: T0, op1: T1);
}

pub trait StrbEmitter<T0, T1> {
    fn strb(&mut self, op0: T0, op1: T1);
}

pub trait StrhEmitter<T0, T1> {
    fn strh(&mut self, op0: T0, op1: T1);
}

pub trait StsetEmitter<T0, T1> {
    fn stset(&mut self, op0: T0, op1: T1);
}

pub trait StsetbEmitter<T0, T1> {
    fn stsetb(&mut self, op0: T0, op1: T1);
}

pub trait StsethEmitter<T0, T1> {
    fn stseth(&mut self, op0: T0, op1: T1);
}

pub trait StsetlEmitter<T0, T1> {
    fn stsetl(&mut self, op0: T0, op1: T1);
}

pub trait StsetlbEmitter<T0, T1> {
    fn stsetlb(&mut self, op0: T0, op1: T1);
}

pub trait StsetlhEmitter<T0, T1> {
    fn stsetlh(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxEmitter<T0, T1> {
    fn stsmax(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxbEmitter<T0, T1> {
    fn stsmaxb(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxhEmitter<T0, T1> {
    fn stsmaxh(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxlEmitter<T0, T1> {
    fn stsmaxl(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxlbEmitter<T0, T1> {
    fn stsmaxlb(&mut self, op0: T0, op1: T1);
}

pub trait StsmaxlhEmitter<T0, T1> {
    fn stsmaxlh(&mut self, op0: T0, op1: T1);
}

pub trait StsminEmitter<T0, T1> {
    fn stsmin(&mut self, op0: T0, op1: T1);
}

pub trait StsminbEmitter<T0, T1> {
    fn stsminb(&mut self, op0: T0, op1: T1);
}

pub trait StsminhEmitter<T0, T1> {
    fn stsminh(&mut self, op0: T0, op1: T1);
}

pub trait StsminlEmitter<T0, T1> {
    fn stsminl(&mut self, op0: T0, op1: T1);
}

pub trait StsminlbEmitter<T0, T1> {
    fn stsminlb(&mut self, op0: T0, op1: T1);
}

pub trait StsminlhEmitter<T0, T1> {
    fn stsminlh(&mut self, op0: T0, op1: T1);
}

pub trait SttrEmitter<T0, T1> {
    fn sttr(&mut self, op0: T0, op1: T1);
}

pub trait SttrbEmitter<T0, T1> {
    fn sttrb(&mut self, op0: T0, op1: T1);
}

pub trait SttrhEmitter<T0, T1> {
    fn sttrh(&mut self, op0: T0, op1: T1);
}

pub trait StumaxEmitter<T0, T1> {
    fn stumax(&mut self, op0: T0, op1: T1);
}

pub trait StumaxbEmitter<T0, T1> {
    fn stumaxb(&mut self, op0: T0, op1: T1);
}

pub trait StumaxhEmitter<T0, T1> {
    fn stumaxh(&mut self, op0: T0, op1: T1);
}

pub trait StumaxlEmitter<T0, T1> {
    fn stumaxl(&mut self, op0: T0, op1: T1);
}

pub trait StumaxlbEmitter<T0, T1> {
    fn stumaxlb(&mut self, op0: T0, op1: T1);
}

pub trait StumaxlhEmitter<T0, T1> {
    fn stumaxlh(&mut self, op0: T0, op1: T1);
}

pub trait StuminEmitter<T0, T1> {
    fn stumin(&mut self, op0: T0, op1: T1);
}

pub trait StuminbEmitter<T0, T1> {
    fn stuminb(&mut self, op0: T0, op1: T1);
}

pub trait StuminhEmitter<T0, T1> {
    fn stuminh(&mut self, op0: T0, op1: T1);
}

pub trait StuminlEmitter<T0, T1> {
    fn stuminl(&mut self, op0: T0, op1: T1);
}

pub trait StuminlbEmitter<T0, T1> {
    fn stuminlb(&mut self, op0: T0, op1: T1);
}

pub trait StuminlhEmitter<T0, T1> {
    fn stuminlh(&mut self, op0: T0, op1: T1);
}

pub trait SturEmitter<T0, T1> {
    fn stur(&mut self, op0: T0, op1: T1);
}

pub trait SturbEmitter<T0, T1> {
    fn sturb(&mut self, op0: T0, op1: T1);
}

pub trait SturhEmitter<T0, T1> {
    fn sturh(&mut self, op0: T0, op1: T1);
}

pub trait StxpEmitter<T0, T1, T2, T3> {
    fn stxp(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait StlxpEmitter<T0, T1, T2, T3> {
    fn stlxp(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait StxrEmitter<T0, T1, T2> {
    fn stxr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StxrbEmitter<T0, T1, T2> {
    fn stxrb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StxrhEmitter<T0, T1, T2> {
    fn stxrh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpEmitter<T0, T1, T2> {
    fn swp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpaEmitter<T0, T1, T2> {
    fn swpa(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpabEmitter<T0, T1, T2> {
    fn swpab(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpahEmitter<T0, T1, T2> {
    fn swpah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpalEmitter<T0, T1, T2> {
    fn swpal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpalbEmitter<T0, T1, T2> {
    fn swpalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpalhEmitter<T0, T1, T2> {
    fn swpalh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwpbEmitter<T0, T1, T2> {
    fn swpb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwphEmitter<T0, T1, T2> {
    fn swph(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwplEmitter<T0, T1, T2> {
    fn swpl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwplbEmitter<T0, T1, T2> {
    fn swplb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SwplhEmitter<T0, T1, T2> {
    fn swplh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BtiEmitter<T0> {
    fn bti(&mut self, op0: T0);
}

pub trait ChkfeatEmitter<T0> {
    fn chkfeat(&mut self, op0: T0);
}

pub trait ClrbhbEmitter {
    fn clrbhb(&mut self);
}

pub trait Crc32bEmitter<T0, T1, T2> {
    fn crc32b(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32hEmitter<T0, T1, T2> {
    fn crc32h(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32wEmitter<T0, T1, T2> {
    fn crc32w(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32xEmitter<T0, T1, T2> {
    fn crc32x(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32cbEmitter<T0, T1, T2> {
    fn crc32cb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32chEmitter<T0, T1, T2> {
    fn crc32ch(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32cwEmitter<T0, T1, T2> {
    fn crc32cw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Crc32cxEmitter<T0, T1, T2> {
    fn crc32cx(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AbsEmitter<T0, T1> {
    fn abs(&mut self, op0: T0, op1: T1);
}

pub trait CntEmitter<T0, T1> {
    fn cnt(&mut self, op0: T0, op1: T1);
}

pub trait CtzEmitter<T0, T1> {
    fn ctz(&mut self, op0: T0, op1: T1);
}

pub trait SmaxEmitter<T0, T1, T2> {
    fn smax(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SminEmitter<T0, T1, T2> {
    fn smin(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmaxEmitter<T0, T1, T2> {
    fn umax(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UminEmitter<T0, T1, T2> {
    fn umin(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait DghEmitter {
    fn dgh(&mut self);
}

pub trait CfinvEmitter {
    fn cfinv(&mut self);
}

pub trait Setf8Emitter<T0> {
    fn setf8(&mut self, op0: T0);
}

pub trait Setf16Emitter<T0> {
    fn setf16(&mut self, op0: T0);
}

pub trait AxflagEmitter {
    fn axflag(&mut self);
}

pub trait XaflagEmitter {
    fn xaflag(&mut self);
}

pub trait BcEmitter<T0> {
    fn bc_eq(&mut self, op0: T0);
    fn bc_ne(&mut self, op0: T0);
    fn bc_cs(&mut self, op0: T0);
    fn bc_hs(&mut self, op0: T0);
    fn bc_cc(&mut self, op0: T0);
    fn bc_lo(&mut self, op0: T0);
    fn bc_mi(&mut self, op0: T0);
    fn bc_pl(&mut self, op0: T0);
    fn bc_vs(&mut self, op0: T0);
    fn bc_vc(&mut self, op0: T0);
    fn bc_hi(&mut self, op0: T0);
    fn bc_ls(&mut self, op0: T0);
    fn bc_ge(&mut self, op0: T0);
    fn bc_lt(&mut self, op0: T0);
    fn bc_gt(&mut self, op0: T0);
    fn bc_le(&mut self, op0: T0);
    fn bc_al(&mut self, op0: T0);
}

pub trait AutdaEmitter<T0, T1> {
    fn autda(&mut self, op0: T0, op1: T1);
}

pub trait AutdbEmitter<T0, T1> {
    fn autdb(&mut self, op0: T0, op1: T1);
}

pub trait AutdzaEmitter<T0> {
    fn autdza(&mut self, op0: T0);
}

pub trait AutdzbEmitter<T0> {
    fn autdzb(&mut self, op0: T0);
}

pub trait AutiaEmitter<T0, T1> {
    fn autia(&mut self, op0: T0, op1: T1);
}

pub trait Autia1716Emitter {
    fn autia1716(&mut self);
}

pub trait AutiaspEmitter {
    fn autiasp(&mut self);
}

pub trait AutiazEmitter {
    fn autiaz(&mut self);
}

pub trait AutibEmitter<T0, T1> {
    fn autib(&mut self, op0: T0, op1: T1);
}

pub trait Autib1716Emitter {
    fn autib1716(&mut self);
}

pub trait AutibspEmitter {
    fn autibsp(&mut self);
}

pub trait AutibzEmitter {
    fn autibz(&mut self);
}

pub trait AutizaEmitter<T0> {
    fn autiza(&mut self, op0: T0);
}

pub trait AutizbEmitter<T0> {
    fn autizb(&mut self, op0: T0);
}

pub trait GmiEmitter<T0, T1, T2> {
    fn gmi(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmppEmitter<T0, T1> {
    fn cmpp(&mut self, op0: T0, op1: T1);
}

pub trait AddgEmitter<T0, T1, T2, T3> {
    fn addg(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait LdgEmitter<T0, T1> {
    fn ldg(&mut self, op0: T0, op1: T1);
}

pub trait LdgmEmitter<T0, T1> {
    fn ldgm(&mut self, op0: T0, op1: T1);
}

pub trait LdraaEmitter<T0, T1> {
    fn ldraa(&mut self, op0: T0, op1: T1);
}

pub trait LdrabEmitter<T0, T1> {
    fn ldrab(&mut self, op0: T0, op1: T1);
}

pub trait PacdaEmitter<T0, T1> {
    fn pacda(&mut self, op0: T0, op1: T1);
}

pub trait PacdbEmitter<T0, T1> {
    fn pacdb(&mut self, op0: T0, op1: T1);
}

pub trait PacdzaEmitter<T0> {
    fn pacdza(&mut self, op0: T0);
}

pub trait PacdzbEmitter<T0> {
    fn pacdzb(&mut self, op0: T0);
}

pub trait PacgaEmitter<T0, T1, T2> {
    fn pacga(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SubpEmitter<T0, T1, T2> {
    fn subp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SubpsEmitter<T0, T1, T2> {
    fn subps(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SubgEmitter<T0, T1, T2, T3> {
    fn subg(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait St2gEmitter<T0, T1> {
    fn st2g(&mut self, op0: T0, op1: T1);
}

pub trait StgEmitter<T0, T1> {
    fn stg(&mut self, op0: T0, op1: T1);
}

pub trait StgpEmitter<T0, T1, T2> {
    fn stgp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait StgmEmitter<T0, T1> {
    fn stgm(&mut self, op0: T0, op1: T1);
}

pub trait StzgEmitter<T0, T1> {
    fn stzg(&mut self, op0: T0, op1: T1);
}

pub trait Stz2gEmitter<T0, T1> {
    fn stz2g(&mut self, op0: T0, op1: T1);
}

pub trait StzgmEmitter<T0, T1> {
    fn stzgm(&mut self, op0: T0, op1: T1);
}

pub trait XpacdEmitter<T0> {
    fn xpacd(&mut self, op0: T0);
}

pub trait XpaciEmitter<T0> {
    fn xpaci(&mut self, op0: T0);
}

pub trait XpaclriEmitter {
    fn xpaclri(&mut self);
}

pub trait HintEmitter<T0> {
    fn hint(&mut self, op0: T0);
}

pub trait NopEmitter {
    fn nop(&mut self);
}

pub trait SevEmitter {
    fn sev(&mut self);
}

pub trait SevlEmitter {
    fn sevl(&mut self);
}

pub trait WfeEmitter {
    fn wfe(&mut self);
}

pub trait WfiEmitter {
    fn wfi(&mut self);
}

pub trait YieldEmitter {
    fn r#yield(&mut self);
}

pub trait AddhnEmitter<T0, T1, T2> {
    fn addhn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Addhn2Emitter<T0, T1, T2> {
    fn addhn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AddpEmitter<T0, T1> {
    fn addp(&mut self, op0: T0, op1: T1);
}

pub trait Addp3Emitter<T0, T1, T2> {
    fn addp_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AddvEmitter<T0, T1> {
    fn addv(&mut self, op0: T0, op1: T1);
}

pub trait Bic2Emitter<T0, T1> {
    fn bic_2(&mut self, op0: T0, op1: T1);
}

pub trait BifEmitter<T0, T1, T2> {
    fn bif(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BitEmitter<T0, T1, T2> {
    fn bit(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BslEmitter<T0, T1, T2> {
    fn bsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmeqEmitter<T0, T1, T2> {
    fn cmeq(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmgeEmitter<T0, T1, T2> {
    fn cmge(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmgtEmitter<T0, T1, T2> {
    fn cmgt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmhiEmitter<T0, T1, T2> {
    fn cmhi(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmhsEmitter<T0, T1, T2> {
    fn cmhs(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmleEmitter<T0, T1, T2> {
    fn cmle(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmltEmitter<T0, T1, T2> {
    fn cmlt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait CmtstEmitter<T0, T1, T2> {
    fn cmtst(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait DupEmitter<T0, T1> {
    fn dup(&mut self, op0: T0, op1: T1);
}

pub trait ExtEmitter<T0, T1, T2, T3> {
    fn ext(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FabdEmitter<T0, T1, T2> {
    fn fabd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FabsEmitter<T0, T1> {
    fn fabs(&mut self, op0: T0, op1: T1);
}

pub trait FacgeEmitter<T0, T1, T2> {
    fn facge(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FacgtEmitter<T0, T1, T2> {
    fn facgt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FaddEmitter<T0, T1, T2> {
    fn fadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FaddpEmitter<T0, T1> {
    fn faddp(&mut self, op0: T0, op1: T1);
}

pub trait Faddp3Emitter<T0, T1, T2> {
    fn faddp_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FccmpEmitter<T0, T1, T2, T3> {
    fn fccmp(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FccmpeEmitter<T0, T1, T2, T3> {
    fn fccmpe(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FcmeqEmitter<T0, T1, T2> {
    fn fcmeq(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcmgeEmitter<T0, T1, T2> {
    fn fcmge(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcmgtEmitter<T0, T1, T2> {
    fn fcmgt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcmleEmitter<T0, T1, T2> {
    fn fcmle(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcmltEmitter<T0, T1, T2> {
    fn fcmlt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcmpEmitter<T0, T1> {
    fn fcmp(&mut self, op0: T0, op1: T1);
}

pub trait FcmpeEmitter<T0, T1> {
    fn fcmpe(&mut self, op0: T0, op1: T1);
}

pub trait FcselEmitter<T0, T1, T2, T3> {
    fn fcsel(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FcvtEmitter<T0, T1> {
    fn fcvt(&mut self, op0: T0, op1: T1);
}

pub trait FcvtasEmitter<T0, T1> {
    fn fcvtas(&mut self, op0: T0, op1: T1);
}

pub trait FcvtauEmitter<T0, T1> {
    fn fcvtau(&mut self, op0: T0, op1: T1);
}

pub trait FcvtlEmitter<T0, T1> {
    fn fcvtl(&mut self, op0: T0, op1: T1);
}

pub trait Fcvtl2Emitter<T0, T1> {
    fn fcvtl2(&mut self, op0: T0, op1: T1);
}

pub trait FcvtmsEmitter<T0, T1> {
    fn fcvtms(&mut self, op0: T0, op1: T1);
}

pub trait FcvtmuEmitter<T0, T1> {
    fn fcvtmu(&mut self, op0: T0, op1: T1);
}

pub trait FcvtnEmitter<T0, T1> {
    fn fcvtn(&mut self, op0: T0, op1: T1);
}

pub trait Fcvtn2Emitter<T0, T1> {
    fn fcvtn2(&mut self, op0: T0, op1: T1);
}

pub trait FcvtnsEmitter<T0, T1> {
    fn fcvtns(&mut self, op0: T0, op1: T1);
}

pub trait FcvtnuEmitter<T0, T1> {
    fn fcvtnu(&mut self, op0: T0, op1: T1);
}

pub trait FcvtpsEmitter<T0, T1> {
    fn fcvtps(&mut self, op0: T0, op1: T1);
}

pub trait FcvtpuEmitter<T0, T1> {
    fn fcvtpu(&mut self, op0: T0, op1: T1);
}

pub trait FcvtxnEmitter<T0, T1> {
    fn fcvtxn(&mut self, op0: T0, op1: T1);
}

pub trait Fcvtxn2Emitter<T0, T1> {
    fn fcvtxn2(&mut self, op0: T0, op1: T1);
}

pub trait FcvtzsEmitter<T0, T1> {
    fn fcvtzs(&mut self, op0: T0, op1: T1);
}

pub trait Fcvtzs3Emitter<T0, T1, T2> {
    fn fcvtzs_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcvtzuEmitter<T0, T1> {
    fn fcvtzu(&mut self, op0: T0, op1: T1);
}

pub trait Fcvtzu3Emitter<T0, T1, T2> {
    fn fcvtzu_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FdivEmitter<T0, T1, T2> {
    fn fdiv(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmaddEmitter<T0, T1, T2, T3> {
    fn fmadd(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FmaxEmitter<T0, T1, T2> {
    fn fmax(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmaxnmEmitter<T0, T1, T2> {
    fn fmaxnm(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmaxnmpEmitter<T0, T1, T2> {
    fn fmaxnmp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Fmaxnmp2Emitter<T0, T1> {
    fn fmaxnmp_2(&mut self, op0: T0, op1: T1);
}

pub trait FmaxnmvEmitter<T0, T1> {
    fn fmaxnmv(&mut self, op0: T0, op1: T1);
}

pub trait FmaxpEmitter<T0, T1, T2> {
    fn fmaxp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Fmaxp2Emitter<T0, T1> {
    fn fmaxp_2(&mut self, op0: T0, op1: T1);
}

pub trait FmaxvEmitter<T0, T1> {
    fn fmaxv(&mut self, op0: T0, op1: T1);
}

pub trait FminEmitter<T0, T1, T2> {
    fn fmin(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FminnmEmitter<T0, T1, T2> {
    fn fminnm(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FminnmvEmitter<T0, T1> {
    fn fminnmv(&mut self, op0: T0, op1: T1);
}

pub trait FminnmpEmitter<T0, T1, T2> {
    fn fminnmp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Fminnmp2Emitter<T0, T1> {
    fn fminnmp_2(&mut self, op0: T0, op1: T1);
}

pub trait FminpEmitter<T0, T1> {
    fn fminp(&mut self, op0: T0, op1: T1);
}

pub trait Fminp3Emitter<T0, T1, T2> {
    fn fminp_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FminvEmitter<T0, T1> {
    fn fminv(&mut self, op0: T0, op1: T1);
}

pub trait FmlaEmitter<T0, T1, T2> {
    fn fmla(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmlsEmitter<T0, T1, T2> {
    fn fmls(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmovEmitter<T0, T1> {
    fn fmov(&mut self, op0: T0, op1: T1);
}

pub trait FmsubEmitter<T0, T1, T2, T3> {
    fn fmsub(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FmulEmitter<T0, T1, T2> {
    fn fmul(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmulxEmitter<T0, T1, T2> {
    fn fmulx(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FnegEmitter<T0, T1> {
    fn fneg(&mut self, op0: T0, op1: T1);
}

pub trait FnmaddEmitter<T0, T1, T2, T3> {
    fn fnmadd(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FnmsubEmitter<T0, T1, T2, T3> {
    fn fnmsub(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FnmulEmitter<T0, T1, T2> {
    fn fnmul(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FrecpeEmitter<T0, T1> {
    fn frecpe(&mut self, op0: T0, op1: T1);
}

pub trait FrecpsEmitter<T0, T1, T2> {
    fn frecps(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FrecpxEmitter<T0, T1> {
    fn frecpx(&mut self, op0: T0, op1: T1);
}

pub trait Frint32xEmitter<T0, T1> {
    fn frint32x(&mut self, op0: T0, op1: T1);
}

pub trait Frint32zEmitter<T0, T1> {
    fn frint32z(&mut self, op0: T0, op1: T1);
}

pub trait Frint64xEmitter<T0, T1> {
    fn frint64x(&mut self, op0: T0, op1: T1);
}

pub trait Frint64zEmitter<T0, T1> {
    fn frint64z(&mut self, op0: T0, op1: T1);
}

pub trait FrintaEmitter<T0, T1> {
    fn frinta(&mut self, op0: T0, op1: T1);
}

pub trait FrintiEmitter<T0, T1> {
    fn frinti(&mut self, op0: T0, op1: T1);
}

pub trait FrintmEmitter<T0, T1> {
    fn frintm(&mut self, op0: T0, op1: T1);
}

pub trait FrintnEmitter<T0, T1> {
    fn frintn(&mut self, op0: T0, op1: T1);
}

pub trait FrintpEmitter<T0, T1> {
    fn frintp(&mut self, op0: T0, op1: T1);
}

pub trait FrintxEmitter<T0, T1> {
    fn frintx(&mut self, op0: T0, op1: T1);
}

pub trait FrintzEmitter<T0, T1> {
    fn frintz(&mut self, op0: T0, op1: T1);
}

pub trait FrsqrteEmitter<T0, T1> {
    fn frsqrte(&mut self, op0: T0, op1: T1);
}

pub trait FrsqrtsEmitter<T0, T1, T2> {
    fn frsqrts(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FsqrtEmitter<T0, T1> {
    fn fsqrt(&mut self, op0: T0, op1: T1);
}

pub trait FsubEmitter<T0, T1, T2> {
    fn fsub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait InsEmitter<T0, T1> {
    fn ins(&mut self, op0: T0, op1: T1);
}

pub trait Ld1Emitter<T0, T1> {
    fn ld1(&mut self, op0: T0, op1: T1);
}

pub trait Ld13Emitter<T0, T1, T2> {
    fn ld1_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ld14Emitter<T0, T1, T2, T3> {
    fn ld1_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Ld15Emitter<T0, T1, T2, T3, T4> {
    fn ld1_5(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait Ld1rEmitter<T0, T1> {
    fn ld1r(&mut self, op0: T0, op1: T1);
}

pub trait Ld2Emitter<T0, T1, T2> {
    fn ld2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ld2rEmitter<T0, T1, T2> {
    fn ld2r(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ld3Emitter<T0, T1, T2, T3> {
    fn ld3(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Ld3rEmitter<T0, T1, T2, T3> {
    fn ld3r(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Ld4Emitter<T0, T1, T2, T3, T4> {
    fn ld4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait Ld4rEmitter<T0, T1, T2, T3, T4> {
    fn ld4r(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait MlaEmitter<T0, T1, T2> {
    fn mla(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MlsEmitter<T0, T1, T2> {
    fn mls(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MoviEmitter<T0, T1> {
    fn movi(&mut self, op0: T0, op1: T1);
}

pub trait Movi3Emitter<T0, T1, T2> {
    fn movi_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait MvniEmitter<T0, T1> {
    fn mvni(&mut self, op0: T0, op1: T1);
}

pub trait Mvni3Emitter<T0, T1, T2> {
    fn mvni_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait NotEmitter<T0, T1> {
    fn not_(&mut self, op0: T0, op1: T1);
}

pub trait Orr2Emitter<T0, T1> {
    fn orr_2(&mut self, op0: T0, op1: T1);
}

pub trait PmulEmitter<T0, T1, T2> {
    fn pmul(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait PmullEmitter<T0, T1, T2> {
    fn pmull(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Pmull2Emitter<T0, T1, T2> {
    fn pmull2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait RaddhnEmitter<T0, T1, T2> {
    fn raddhn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Raddhn2Emitter<T0, T1, T2> {
    fn raddhn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait RshrnEmitter<T0, T1, T2> {
    fn rshrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Rshrn2Emitter<T0, T1, T2> {
    fn rshrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait RsubhnEmitter<T0, T1, T2> {
    fn rsubhn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Rsubhn2Emitter<T0, T1, T2> {
    fn rsubhn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SabaEmitter<T0, T1, T2> {
    fn saba(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SabalEmitter<T0, T1, T2> {
    fn sabal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sabal2Emitter<T0, T1, T2> {
    fn sabal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SabdEmitter<T0, T1, T2> {
    fn sabd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SabdlEmitter<T0, T1, T2> {
    fn sabdl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sabdl2Emitter<T0, T1, T2> {
    fn sabdl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SadalpEmitter<T0, T1> {
    fn sadalp(&mut self, op0: T0, op1: T1);
}

pub trait SaddlEmitter<T0, T1, T2> {
    fn saddl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Saddl2Emitter<T0, T1, T2> {
    fn saddl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SaddlpEmitter<T0, T1> {
    fn saddlp(&mut self, op0: T0, op1: T1);
}

pub trait SaddlvEmitter<T0, T1> {
    fn saddlv(&mut self, op0: T0, op1: T1);
}

pub trait SaddwEmitter<T0, T1, T2> {
    fn saddw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Saddw2Emitter<T0, T1, T2> {
    fn saddw2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ScvtfEmitter<T0, T1> {
    fn scvtf(&mut self, op0: T0, op1: T1);
}

pub trait Scvtf3Emitter<T0, T1, T2> {
    fn scvtf_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ShaddEmitter<T0, T1, T2> {
    fn shadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ShlEmitter<T0, T1, T2> {
    fn shl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ShllEmitter<T0, T1, T2> {
    fn shll(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Shll2Emitter<T0, T1, T2> {
    fn shll2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ShrnEmitter<T0, T1, T2> {
    fn shrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Shrn2Emitter<T0, T1, T2> {
    fn shrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait ShsubEmitter<T0, T1, T2> {
    fn shsub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SliEmitter<T0, T1, T2> {
    fn sli(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmaxpEmitter<T0, T1, T2> {
    fn smaxp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmaxvEmitter<T0, T1> {
    fn smaxv(&mut self, op0: T0, op1: T1);
}

pub trait SminpEmitter<T0, T1, T2> {
    fn sminp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SminvEmitter<T0, T1> {
    fn sminv(&mut self, op0: T0, op1: T1);
}

pub trait SmlalEmitter<T0, T1, T2> {
    fn smlal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Smlal2Emitter<T0, T1, T2> {
    fn smlal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmlslEmitter<T0, T1, T2> {
    fn smlsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Smlsl2Emitter<T0, T1, T2> {
    fn smlsl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmovEmitter<T0, T1> {
    fn smov(&mut self, op0: T0, op1: T1);
}

pub trait Smull2Emitter<T0, T1, T2> {
    fn smull2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqabsEmitter<T0, T1> {
    fn sqabs(&mut self, op0: T0, op1: T1);
}

pub trait SqaddEmitter<T0, T1, T2> {
    fn sqadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqdmlalEmitter<T0, T1, T2> {
    fn sqdmlal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqdmlal2Emitter<T0, T1, T2> {
    fn sqdmlal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqdmlslEmitter<T0, T1, T2> {
    fn sqdmlsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqdmlsl2Emitter<T0, T1, T2> {
    fn sqdmlsl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqdmulhEmitter<T0, T1, T2> {
    fn sqdmulh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqdmullEmitter<T0, T1, T2> {
    fn sqdmull(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqdmull2Emitter<T0, T1, T2> {
    fn sqdmull2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqnegEmitter<T0, T1> {
    fn sqneg(&mut self, op0: T0, op1: T1);
}

pub trait SqrdmulhEmitter<T0, T1, T2> {
    fn sqrdmulh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqrshlEmitter<T0, T1, T2> {
    fn sqrshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqrshrnEmitter<T0, T1, T2> {
    fn sqrshrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqrshrn2Emitter<T0, T1, T2> {
    fn sqrshrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqrshrunEmitter<T0, T1, T2> {
    fn sqrshrun(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqrshrun2Emitter<T0, T1, T2> {
    fn sqrshrun2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqshlEmitter<T0, T1, T2> {
    fn sqshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqshluEmitter<T0, T1, T2> {
    fn sqshlu(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqshrnEmitter<T0, T1, T2> {
    fn sqshrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqshrn2Emitter<T0, T1, T2> {
    fn sqshrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqshrunEmitter<T0, T1, T2> {
    fn sqshrun(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sqshrun2Emitter<T0, T1, T2> {
    fn sqshrun2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqsubEmitter<T0, T1, T2> {
    fn sqsub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqxtnEmitter<T0, T1> {
    fn sqxtn(&mut self, op0: T0, op1: T1);
}

pub trait Sqxtn2Emitter<T0, T1> {
    fn sqxtn2(&mut self, op0: T0, op1: T1);
}

pub trait SqxtunEmitter<T0, T1> {
    fn sqxtun(&mut self, op0: T0, op1: T1);
}

pub trait Sqxtun2Emitter<T0, T1> {
    fn sqxtun2(&mut self, op0: T0, op1: T1);
}

pub trait SrhaddEmitter<T0, T1, T2> {
    fn srhadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SriEmitter<T0, T1, T2> {
    fn sri(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SrshlEmitter<T0, T1, T2> {
    fn srshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SrshrEmitter<T0, T1, T2> {
    fn srshr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SrsraEmitter<T0, T1, T2> {
    fn srsra(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SshlEmitter<T0, T1, T2> {
    fn sshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SshllEmitter<T0, T1, T2> {
    fn sshll(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sshll2Emitter<T0, T1, T2> {
    fn sshll2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SshrEmitter<T0, T1, T2> {
    fn sshr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SsraEmitter<T0, T1, T2> {
    fn ssra(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SsublEmitter<T0, T1, T2> {
    fn ssubl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ssubl2Emitter<T0, T1, T2> {
    fn ssubl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SsubwEmitter<T0, T1, T2> {
    fn ssubw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ssubw2Emitter<T0, T1, T2> {
    fn ssubw2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait St1Emitter<T0, T1> {
    fn st1(&mut self, op0: T0, op1: T1);
}

pub trait St13Emitter<T0, T1, T2> {
    fn st1_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait St14Emitter<T0, T1, T2, T3> {
    fn st1_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait St15Emitter<T0, T1, T2, T3, T4> {
    fn st1_5(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait St2Emitter<T0, T1, T2> {
    fn st2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait St3Emitter<T0, T1, T2, T3> {
    fn st3(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait St4Emitter<T0, T1, T2, T3, T4> {
    fn st4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait SubhnEmitter<T0, T1, T2> {
    fn subhn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Subhn2Emitter<T0, T1, T2> {
    fn subhn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SuqaddEmitter<T0, T1> {
    fn suqadd(&mut self, op0: T0, op1: T1);
}

pub trait SxtlEmitter<T0, T1> {
    fn sxtl(&mut self, op0: T0, op1: T1);
}

pub trait Sxtl2Emitter<T0, T1> {
    fn sxtl2(&mut self, op0: T0, op1: T1);
}

pub trait TblEmitter<T0, T1, T2> {
    fn tbl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Tbl4Emitter<T0, T1, T2, T3> {
    fn tbl_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Tbl5Emitter<T0, T1, T2, T3, T4> {
    fn tbl_5(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait Tbl6Emitter<T0, T1, T2, T3, T4, T5> {
    fn tbl_6(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4, op5: T5);
}

pub trait TbxEmitter<T0, T1, T2> {
    fn tbx(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Tbx4Emitter<T0, T1, T2, T3> {
    fn tbx_4(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Tbx5Emitter<T0, T1, T2, T3, T4> {
    fn tbx_5(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4);
}

pub trait Tbx6Emitter<T0, T1, T2, T3, T4, T5> {
    fn tbx_6(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4, op5: T5);
}

pub trait Trn1Emitter<T0, T1, T2> {
    fn trn1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Trn2Emitter<T0, T1, T2> {
    fn trn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UabaEmitter<T0, T1, T2> {
    fn uaba(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UabalEmitter<T0, T1, T2> {
    fn uabal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uabal2Emitter<T0, T1, T2> {
    fn uabal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UabdEmitter<T0, T1, T2> {
    fn uabd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UabdlEmitter<T0, T1, T2> {
    fn uabdl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uabdl2Emitter<T0, T1, T2> {
    fn uabdl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UadalpEmitter<T0, T1> {
    fn uadalp(&mut self, op0: T0, op1: T1);
}

pub trait UaddlEmitter<T0, T1, T2> {
    fn uaddl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uaddl2Emitter<T0, T1, T2> {
    fn uaddl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UaddlpEmitter<T0, T1> {
    fn uaddlp(&mut self, op0: T0, op1: T1);
}

pub trait UaddlvEmitter<T0, T1> {
    fn uaddlv(&mut self, op0: T0, op1: T1);
}

pub trait UaddwEmitter<T0, T1, T2> {
    fn uaddw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uaddw2Emitter<T0, T1, T2> {
    fn uaddw2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UcvtfEmitter<T0, T1> {
    fn ucvtf(&mut self, op0: T0, op1: T1);
}

pub trait Ucvtf3Emitter<T0, T1, T2> {
    fn ucvtf_3(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UhaddEmitter<T0, T1, T2> {
    fn uhadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UhsubEmitter<T0, T1, T2> {
    fn uhsub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmaxpEmitter<T0, T1, T2> {
    fn umaxp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmaxvEmitter<T0, T1> {
    fn umaxv(&mut self, op0: T0, op1: T1);
}

pub trait UminpEmitter<T0, T1, T2> {
    fn uminp(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UminvEmitter<T0, T1> {
    fn uminv(&mut self, op0: T0, op1: T1);
}

pub trait UmlalEmitter<T0, T1, T2> {
    fn umlal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Umlal2Emitter<T0, T1, T2> {
    fn umlal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmlslEmitter<T0, T1, T2> {
    fn umlsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Umlsl2Emitter<T0, T1, T2> {
    fn umlsl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmovEmitter<T0, T1> {
    fn umov(&mut self, op0: T0, op1: T1);
}

pub trait Umull2Emitter<T0, T1, T2> {
    fn umull2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqaddEmitter<T0, T1, T2> {
    fn uqadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqrshlEmitter<T0, T1, T2> {
    fn uqrshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqrshrnEmitter<T0, T1, T2> {
    fn uqrshrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uqrshrn2Emitter<T0, T1, T2> {
    fn uqrshrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqshlEmitter<T0, T1, T2> {
    fn uqshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqshrnEmitter<T0, T1, T2> {
    fn uqshrn(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uqshrn2Emitter<T0, T1, T2> {
    fn uqshrn2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqsubEmitter<T0, T1, T2> {
    fn uqsub(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UqxtnEmitter<T0, T1> {
    fn uqxtn(&mut self, op0: T0, op1: T1);
}

pub trait Uqxtn2Emitter<T0, T1> {
    fn uqxtn2(&mut self, op0: T0, op1: T1);
}

pub trait UrecpeEmitter<T0, T1> {
    fn urecpe(&mut self, op0: T0, op1: T1);
}

pub trait UrhaddEmitter<T0, T1, T2> {
    fn urhadd(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UrshlEmitter<T0, T1, T2> {
    fn urshl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UrshrEmitter<T0, T1, T2> {
    fn urshr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UrsqrteEmitter<T0, T1> {
    fn ursqrte(&mut self, op0: T0, op1: T1);
}

pub trait UrsraEmitter<T0, T1, T2> {
    fn ursra(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UshlEmitter<T0, T1, T2> {
    fn ushl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UshllEmitter<T0, T1, T2> {
    fn ushll(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Ushll2Emitter<T0, T1, T2> {
    fn ushll2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UshrEmitter<T0, T1, T2> {
    fn ushr(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UsqaddEmitter<T0, T1> {
    fn usqadd(&mut self, op0: T0, op1: T1);
}

pub trait UsraEmitter<T0, T1, T2> {
    fn usra(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UsublEmitter<T0, T1, T2> {
    fn usubl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Usubl2Emitter<T0, T1, T2> {
    fn usubl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UsubwEmitter<T0, T1, T2> {
    fn usubw(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Usubw2Emitter<T0, T1, T2> {
    fn usubw2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UxtlEmitter<T0, T1> {
    fn uxtl(&mut self, op0: T0, op1: T1);
}

pub trait Uxtl2Emitter<T0, T1> {
    fn uxtl2(&mut self, op0: T0, op1: T1);
}

pub trait Uzp1Emitter<T0, T1, T2> {
    fn uzp1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Uzp2Emitter<T0, T1, T2> {
    fn uzp2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait XtnEmitter<T0, T1> {
    fn xtn(&mut self, op0: T0, op1: T1);
}

pub trait Xtn2Emitter<T0, T1> {
    fn xtn2(&mut self, op0: T0, op1: T1);
}

pub trait Zip1Emitter<T0, T1, T2> {
    fn zip1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Zip2Emitter<T0, T1, T2> {
    fn zip2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait AesdEmitter<T0, T1> {
    fn aesd(&mut self, op0: T0, op1: T1);
}

pub trait AeseEmitter<T0, T1> {
    fn aese(&mut self, op0: T0, op1: T1);
}

pub trait AesimcEmitter<T0, T1> {
    fn aesimc(&mut self, op0: T0, op1: T1);
}

pub trait AesmcEmitter<T0, T1> {
    fn aesmc(&mut self, op0: T0, op1: T1);
}

pub trait Sha1cEmitter<T0, T1, T2> {
    fn sha1c(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha1hEmitter<T0, T1> {
    fn sha1h(&mut self, op0: T0, op1: T1);
}

pub trait Sha1mEmitter<T0, T1, T2> {
    fn sha1m(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha1pEmitter<T0, T1, T2> {
    fn sha1p(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha1su0Emitter<T0, T1, T2> {
    fn sha1su0(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha1su1Emitter<T0, T1> {
    fn sha1su1(&mut self, op0: T0, op1: T1);
}

pub trait Sha256hEmitter<T0, T1, T2> {
    fn sha256h(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha256h2Emitter<T0, T1, T2> {
    fn sha256h2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha256su0Emitter<T0, T1> {
    fn sha256su0(&mut self, op0: T0, op1: T1);
}

pub trait Sha256su1Emitter<T0, T1, T2> {
    fn sha256su1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqrdmlahEmitter<T0, T1, T2> {
    fn sqrdmlah(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SqrdmlshEmitter<T0, T1, T2> {
    fn sqrdmlsh(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FcaddEmitter<T0, T1, T2, T3> {
    fn fcadd(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FcmlaEmitter<T0, T1, T2, T3> {
    fn fcmla(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait FjcvtzsEmitter<T0, T1> {
    fn fjcvtzs(&mut self, op0: T0, op1: T1);
}

pub trait FmlalEmitter<T0, T1, T2> {
    fn fmlal(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Fmlal2Emitter<T0, T1, T2> {
    fn fmlal2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait FmlslEmitter<T0, T1, T2> {
    fn fmlsl(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Fmlsl2Emitter<T0, T1, T2> {
    fn fmlsl2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BcaxEmitter<T0, T1, T2, T3> {
    fn bcax(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Eor3Emitter<T0, T1, T2, T3> {
    fn eor3(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Rax1Emitter<T0, T1, T2> {
    fn rax1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait XarEmitter<T0, T1, T2, T3> {
    fn xar(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Sha512hEmitter<T0, T1, T2> {
    fn sha512h(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha512h2Emitter<T0, T1, T2> {
    fn sha512h2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sha512su0Emitter<T0, T1> {
    fn sha512su0(&mut self, op0: T0, op1: T1);
}

pub trait Sha512su1Emitter<T0, T1, T2> {
    fn sha512su1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3partw1Emitter<T0, T1, T2> {
    fn sm3partw1(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3partw2Emitter<T0, T1, T2> {
    fn sm3partw2(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3ss1Emitter<T0, T1, T2, T3> {
    fn sm3ss1(&mut self, op0: T0, op1: T1, op2: T2, op3: T3);
}

pub trait Sm3tt1aEmitter<T0, T1, T2> {
    fn sm3tt1a(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3tt1bEmitter<T0, T1, T2> {
    fn sm3tt1b(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3tt2aEmitter<T0, T1, T2> {
    fn sm3tt2a(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm3tt2bEmitter<T0, T1, T2> {
    fn sm3tt2b(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait Sm4eEmitter<T0, T1> {
    fn sm4e(&mut self, op0: T0, op1: T1);
}

pub trait Sm4ekeyEmitter<T0, T1, T2> {
    fn sm4ekey(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SdotEmitter<T0, T1, T2> {
    fn sdot(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UdotEmitter<T0, T1, T2> {
    fn udot(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BfcvtEmitter<T0, T1> {
    fn bfcvt(&mut self, op0: T0, op1: T1);
}

pub trait BfcvtnEmitter<T0, T1> {
    fn bfcvtn(&mut self, op0: T0, op1: T1);
}

pub trait Bfcvtn2Emitter<T0, T1> {
    fn bfcvtn2(&mut self, op0: T0, op1: T1);
}

pub trait BfmlalbEmitter<T0, T1, T2> {
    fn bfmlalb(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BfmlaltEmitter<T0, T1, T2> {
    fn bfmlalt(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BfmmlaEmitter<T0, T1, T2> {
    fn bfmmla(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait BfdotEmitter<T0, T1, T2> {
    fn bfdot(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SmmlaEmitter<T0, T1, T2> {
    fn smmla(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait SudotEmitter<T0, T1, T2> {
    fn sudot(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UmmlaEmitter<T0, T1, T2> {
    fn ummla(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UsdotEmitter<T0, T1, T2> {
    fn usdot(&mut self, op0: T0, op1: T1, op2: T2);
}

pub trait UsmmlaEmitter<T0, T1, T2> {
    fn usmmla(&mut self, op0: T0, op1: T1, op2: T2);
}

impl AdcEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn adc(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Adc,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AdcsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn adcs(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Adcs,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn add(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Add,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn add(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Add,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn add(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Add_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Add4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn add_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Add,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Add4Emitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn add_4(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Add,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl AddsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn adds(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Adds,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddsEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn adds(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Adds,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Adds4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn adds_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Adds,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Adds4Emitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn adds_4(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Adds,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl AdrEmitter<Gp, Imm> for Assembler<'_> {
    fn adr(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Adr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AdrEmitter<Gp, Label> for Assembler<'_> {
    fn adr(&mut self, op0: Gp, op1: Label) {
        self.emit_n(InstId::Adr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AdrEmitter<Gp, Sym> for Assembler<'_> {
    fn adr(&mut self, op0: Gp, op1: Sym) {
        self.emit_n(InstId::Adr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AdrpEmitter<Gp, Imm> for Assembler<'_> {
    fn adrp(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Adrp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AdrpEmitter<Gp, Label> for Assembler<'_> {
    fn adrp(&mut self, op0: Gp, op1: Label) {
        self.emit_n(InstId::Adrp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AdrpEmitter<Gp, Sym> for Assembler<'_> {
    fn adrp(&mut self, op0: Gp, op1: Sym) {
        self.emit_n(InstId::Adrp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AndEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn and_(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::And,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AndEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn and_(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::And,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AndEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn and_(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::And_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl And4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn and__4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::And,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl AndsEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn ands(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Ands,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AndsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn ands(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Ands,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ands4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn ands_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Ands,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl AsrEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn asr(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Asr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AsrEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn asr(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Asr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AsrvEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn asrv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Asrv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AtEmitter<Imm, Gp> for Assembler<'_> {
    fn at(&mut self, op0: Imm, op1: Gp) {
        self.emit_n(InstId::At, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl BfcEmitter<Gp, Imm, Imm> for Assembler<'_> {
    fn bfc(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Bfc,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BfiEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn bfi(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Bfi,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl BfmEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn bfm(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Bfm,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl BfxilEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn bfxil(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Bfxil,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl BicEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn bic(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Bic,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BicEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn bic(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Bic,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BicEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bic(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bic_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BicEmitter<Vec, Imm, Imm> for Assembler<'_> {
    fn bic(&mut self, op0: Vec, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Bic_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Bic4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn bic_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Bic,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl BicsEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn bics(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Bics,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BicsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn bics(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Bics,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Bics4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn bics_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Bics,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl BrkEmitter<Imm> for Assembler<'_> {
    fn brk(&mut self, op0: Imm) {
        self.emit_n(InstId::Brk, &[op0.as_operand()]);
    }
}

impl CcmnEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn ccmn(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ccmn,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CcmnEmitter<Gp, Imm, Imm, Imm> for Assembler<'_> {
    fn ccmn(&mut self, op0: Gp, op1: Imm, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ccmn,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CcmpEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn ccmp(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ccmp,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CcmpEmitter<Gp, Imm, Imm, Imm> for Assembler<'_> {
    fn ccmp(&mut self, op0: Gp, op1: Imm, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ccmp,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CincEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn cinc(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Cinc,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CinvEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn cinv(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Cinv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ClrexEmitter<Imm> for Assembler<'_> {
    fn clrex(&mut self, op0: Imm) {
        self.emit_n(InstId::Clrex, &[op0.as_operand()]);
    }
}

impl ClsEmitter<Gp, Gp> for Assembler<'_> {
    fn cls(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Cls, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl ClsEmitter<Vec, Vec> for Assembler<'_> {
    fn cls(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Cls_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl ClzEmitter<Gp, Gp> for Assembler<'_> {
    fn clz(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Clz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl ClzEmitter<Vec, Vec> for Assembler<'_> {
    fn clz(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Clz_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CmnEmitter<Gp, Gp> for Assembler<'_> {
    fn cmn(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Cmn, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CmnEmitter<Gp, Imm> for Assembler<'_> {
    fn cmn(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Cmn, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Cmn3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn cmn_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Cmn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Cmn3Emitter<Gp, Imm, Imm> for Assembler<'_> {
    fn cmn_3(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Cmn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmpEmitter<Gp, Gp> for Assembler<'_> {
    fn cmp(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Cmp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CmpEmitter<Gp, Imm> for Assembler<'_> {
    fn cmp(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Cmp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Cmp3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn cmp_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Cmp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Cmp3Emitter<Gp, Imm, Imm> for Assembler<'_> {
    fn cmp_3(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Cmp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CnegEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn cneg(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Cneg,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CselEmitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn csel(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Csel,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CsetEmitter<Gp, Imm> for Assembler<'_> {
    fn cset(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Cset, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CsetmEmitter<Gp, Imm> for Assembler<'_> {
    fn csetm(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Csetm, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CsincEmitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn csinc(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Csinc,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CsinvEmitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn csinv(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Csinv,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl CsnegEmitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn csneg(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Csneg,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl DcEmitter<Imm, Gp> for Assembler<'_> {
    fn dc(&mut self, op0: Imm, op1: Gp) {
        self.emit_n(InstId::Dc, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl DmbEmitter<Imm> for Assembler<'_> {
    fn dmb(&mut self, op0: Imm) {
        self.emit_n(InstId::Dmb, &[op0.as_operand()]);
    }
}

impl DsbEmitter<Imm> for Assembler<'_> {
    fn dsb(&mut self, op0: Imm) {
        self.emit_n(InstId::Dsb, &[op0.as_operand()]);
    }
}

impl DrpsEmitter for Assembler<'_> {
    fn drps(&mut self) {
        self.emit_n(InstId::Drps, &[]);
    }
}

impl EonEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn eon(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Eon,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Eon4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn eon_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Eon,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl EorEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn eor(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Eor,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl EorEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn eor(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Eor,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl EorEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn eor(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Eor_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Eor4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn eor_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Eor,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl EretEmitter for Assembler<'_> {
    fn eret(&mut self) {
        self.emit_n(InstId::Eret, &[]);
    }
}

impl EsbEmitter for Assembler<'_> {
    fn esb(&mut self) {
        self.emit_n(InstId::Esb, &[]);
    }
}

impl ExtrEmitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn extr(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Extr,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl HltEmitter<Imm> for Assembler<'_> {
    fn hlt(&mut self, op0: Imm) {
        self.emit_n(InstId::Hlt, &[op0.as_operand()]);
    }
}

impl HvcEmitter<Imm> for Assembler<'_> {
    fn hvc(&mut self, op0: Imm) {
        self.emit_n(InstId::Hvc, &[op0.as_operand()]);
    }
}

impl IcEmitter<Imm, Gp> for Assembler<'_> {
    fn ic(&mut self, op0: Imm, op1: Gp) {
        self.emit_n(InstId::Ic, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl IsbEmitter<Imm> for Assembler<'_> {
    fn isb(&mut self, op0: Imm) {
        self.emit_n(InstId::Isb, &[op0.as_operand()]);
    }
}

impl LslEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn lsl(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Lsl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LslEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn lsl(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Lsl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LslvEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn lslv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Lslv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LsrEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn lsr(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Lsr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LsrEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn lsr(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Lsr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LsrvEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn lsrv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Lsrv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MaddEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn madd(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Madd,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl MnegEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn mneg(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Mneg,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MovEmitter<Gp, Gp> for Assembler<'_> {
    fn mov(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Mov, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MovEmitter<Gp, Imm> for Assembler<'_> {
    fn mov(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Mov, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MovEmitter<Vec, Vec> for Assembler<'_> {
    fn mov(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Mov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MovEmitter<Gp, Vec> for Assembler<'_> {
    fn mov(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Mov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MovEmitter<Vec, Gp> for Assembler<'_> {
    fn mov(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Mov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MovkEmitter<Gp, Imm> for Assembler<'_> {
    fn movk(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Movk, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Movk3Emitter<Gp, Imm, Imm> for Assembler<'_> {
    fn movk_3(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Movk,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MovnEmitter<Gp, Imm> for Assembler<'_> {
    fn movn(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Movn, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Movn3Emitter<Gp, Imm, Imm> for Assembler<'_> {
    fn movn_3(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Movn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MovzEmitter<Gp, Imm> for Assembler<'_> {
    fn movz(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Movz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Movz3Emitter<Gp, Imm, Imm> for Assembler<'_> {
    fn movz_3(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Movz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MrsEmitter<Gp, Imm> for Assembler<'_> {
    fn mrs(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Mrs, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MsrEmitter<Imm, Gp> for Assembler<'_> {
    fn msr(&mut self, op0: Imm, op1: Gp) {
        self.emit_n(InstId::Msr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MsrEmitter<Imm, Imm> for Assembler<'_> {
    fn msr(&mut self, op0: Imm, op1: Imm) {
        self.emit_n(InstId::Msr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MsubEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn msub(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Msub,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl MulEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn mul(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Mul,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MulEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn mul(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Mul_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MvnEmitter<Gp, Gp> for Assembler<'_> {
    fn mvn(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Mvn, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl MvnEmitter<Vec, Vec> for Assembler<'_> {
    fn mvn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Mvn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Mvn3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn mvn_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Mvn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Mvn_Emitter<Gp, Gp> for Assembler<'_> {
    fn mvn_(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Mvn, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Mvn_Emitter<Vec, Vec> for Assembler<'_> {
    fn mvn_(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Mvn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Mvn_3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn mvn__3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Mvn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl NegEmitter<Gp, Gp> for Assembler<'_> {
    fn neg(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Neg, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl NegEmitter<Vec, Vec> for Assembler<'_> {
    fn neg(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Neg_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Neg3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn neg_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Neg,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl NegsEmitter<Gp, Gp> for Assembler<'_> {
    fn negs(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Negs, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Negs3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn negs_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Negs,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl NgcEmitter<Gp, Gp> for Assembler<'_> {
    fn ngc(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Ngc, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl NgcsEmitter<Gp, Gp> for Assembler<'_> {
    fn ngcs(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Ngcs, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl OrnEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn orn(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Orn,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl OrnEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn orn(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Orn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Orn4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn orn_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Orn,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl OrrEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn orr(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Orr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl OrrEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn orr(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Orr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl OrrEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn orr(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Orr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl OrrEmitter<Vec, Imm, Imm> for Assembler<'_> {
    fn orr(&mut self, op0: Vec, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Orr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Orr4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn orr_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Orr,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl RbitEmitter<Gp, Gp> for Assembler<'_> {
    fn rbit(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Rbit, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl RbitEmitter<Vec, Vec> for Assembler<'_> {
    fn rbit(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Rbit_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl RetEmitter<Gp> for Assembler<'_> {
    fn ret(&mut self, op0: Gp) {
        self.emit_n(InstId::Ret, &[op0.as_operand()]);
    }
}

impl RevEmitter<Gp, Gp> for Assembler<'_> {
    fn rev(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Rev, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev16Emitter<Gp, Gp> for Assembler<'_> {
    fn rev16(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Rev16, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev16Emitter<Vec, Vec> for Assembler<'_> {
    fn rev16(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Rev16_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev32Emitter<Gp, Gp> for Assembler<'_> {
    fn rev32(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Rev32, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev32Emitter<Vec, Vec> for Assembler<'_> {
    fn rev32(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Rev32_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev64Emitter<Gp, Gp> for Assembler<'_> {
    fn rev64(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Rev64, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Rev64Emitter<Vec, Vec> for Assembler<'_> {
    fn rev64(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Rev64_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl RorEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn ror(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Ror,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl RorEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn ror(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Ror,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl RorvEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn rorv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Rorv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SbcEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn sbc(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Sbc,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SbcsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn sbcs(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Sbcs,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SbfizEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn sbfiz(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Sbfiz,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SbfmEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn sbfm(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Sbfm,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SbfxEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn sbfx(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Sbfx,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SdivEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn sdiv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Sdiv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmaddlEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn smaddl(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Smaddl,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SmcEmitter<Imm> for Assembler<'_> {
    fn smc(&mut self, op0: Imm) {
        self.emit_n(InstId::Smc, &[op0.as_operand()]);
    }
}

impl SmneglEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn smnegl(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Smnegl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmsublEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn smsubl(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Smsubl,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SmulhEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn smulh(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Smulh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmullEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn smull(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Smull,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmullEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smull(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smull_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn sub(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Sub,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn sub(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Sub,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sub4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn sub_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Sub,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Sub4Emitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn sub_4(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Sub,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SubsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn subs(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Subs,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubsEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn subs(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Subs,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Subs4Emitter<Gp, Gp, Gp, Imm> for Assembler<'_> {
    fn subs_4(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Imm) {
        self.emit_n(
            InstId::Subs,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Subs4Emitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn subs_4(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Subs,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl SvcEmitter<Imm> for Assembler<'_> {
    fn svc(&mut self, op0: Imm) {
        self.emit_n(InstId::Svc, &[op0.as_operand()]);
    }
}

impl SxtbEmitter<Gp, Gp> for Assembler<'_> {
    fn sxtb(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Sxtb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SxthEmitter<Gp, Gp> for Assembler<'_> {
    fn sxth(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Sxth, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SxtwEmitter<Gp, Gp> for Assembler<'_> {
    fn sxtw(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Sxtw, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SysEmitter<Imm, Imm, Imm, Imm> for Assembler<'_> {
    fn sys(&mut self, op0: Imm, op1: Imm, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Sys,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Sys5Emitter<Imm, Imm, Imm, Imm, Gp> for Assembler<'_> {
    fn sys_5(&mut self, op0: Imm, op1: Imm, op2: Imm, op3: Imm, op4: Gp) {
        self.emit_n(
            InstId::Sys,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl TlbiEmitter<Imm, Gp> for Assembler<'_> {
    fn tlbi(&mut self, op0: Imm, op1: Gp) {
        self.emit_n(InstId::Tlbi, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl TstEmitter<Gp, Imm> for Assembler<'_> {
    fn tst(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Tst, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl TstEmitter<Gp, Gp> for Assembler<'_> {
    fn tst(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Tst, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Tst3Emitter<Gp, Gp, Imm> for Assembler<'_> {
    fn tst_3(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Tst,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UdivEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn udiv(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Udiv,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UbfizEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn ubfiz(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ubfiz,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl UbfmEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn ubfm(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ubfm,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl UbfxEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn ubfx(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Ubfx,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl UmaddlEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn umaddl(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Umaddl,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl UmneglEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn umnegl(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Umnegl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmsublEmitter<Gp, Gp, Gp, Gp> for Assembler<'_> {
    fn umsubl(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp) {
        self.emit_n(
            InstId::Umsubl,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl UmullEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn umull(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Umull,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmullEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umull(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umull_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmulhEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn umulh(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Umulh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UxtbEmitter<Gp, Gp> for Assembler<'_> {
    fn uxtb(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Uxtb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UxthEmitter<Gp, Gp> for Assembler<'_> {
    fn uxth(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Uxth, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CsdbEmitter for Assembler<'_> {
    fn csdb(&mut self) {
        self.emit_n(InstId::Csdb, &[]);
    }
}

impl Dcps1Emitter<Imm> for Assembler<'_> {
    fn dcps1(&mut self, op0: Imm) {
        self.emit_n(InstId::Dcps1, &[op0.as_operand()]);
    }
}

impl Dcps2Emitter<Imm> for Assembler<'_> {
    fn dcps2(&mut self, op0: Imm) {
        self.emit_n(InstId::Dcps2, &[op0.as_operand()]);
    }
}

impl Dcps3Emitter<Imm> for Assembler<'_> {
    fn dcps3(&mut self, op0: Imm) {
        self.emit_n(InstId::Dcps3, &[op0.as_operand()]);
    }
}

impl PssbbEmitter for Assembler<'_> {
    fn pssbb(&mut self) {
        self.emit_n(InstId::Pssbb, &[]);
    }
}

impl SsbbEmitter for Assembler<'_> {
    fn ssbb(&mut self) {
        self.emit_n(InstId::Ssbb, &[]);
    }
}

impl UdfEmitter<Imm> for Assembler<'_> {
    fn udf(&mut self, op0: Imm) {
        self.emit_n(InstId::Udf, &[op0.as_operand()]);
    }
}

impl BEmitter<Imm> for Assembler<'_> {
    fn b(&mut self, op0: Imm) {
        self.emit_n(InstId::B, &[op0.as_operand()]);
    }

    fn b_eq(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::EQ), &[op0.as_operand()]);
    }
    fn b_ne(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::NE), &[op0.as_operand()]);
    }
    fn b_cs(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::CS), &[op0.as_operand()]);
    }
    fn b_hs(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::HS), &[op0.as_operand()]);
    }
    fn b_cc(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::CC), &[op0.as_operand()]);
    }
    fn b_lo(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::LO), &[op0.as_operand()]);
    }
    fn b_mi(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::MI), &[op0.as_operand()]);
    }
    fn b_pl(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::PL), &[op0.as_operand()]);
    }
    fn b_vs(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::VS), &[op0.as_operand()]);
    }
    fn b_vc(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::VC), &[op0.as_operand()]);
    }
    fn b_hi(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::HI), &[op0.as_operand()]);
    }
    fn b_ls(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::LS), &[op0.as_operand()]);
    }
    fn b_ge(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::GE), &[op0.as_operand()]);
    }
    fn b_lt(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::LT), &[op0.as_operand()]);
    }
    fn b_gt(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::GT), &[op0.as_operand()]);
    }
    fn b_le(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::LE), &[op0.as_operand()]);
    }
    fn b_al(&mut self, op0: Imm) {
        self.emit_n(InstId::B.with_cc(CondCode::AL), &[op0.as_operand()]);
    }
}

impl BEmitter<Label> for Assembler<'_> {
    fn b(&mut self, op0: Label) {
        self.emit_n(InstId::B, &[op0.as_operand()]);
    }

    fn b_eq(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::EQ), &[op0.as_operand()]);
    }
    fn b_ne(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::NE), &[op0.as_operand()]);
    }
    fn b_cs(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::CS), &[op0.as_operand()]);
    }
    fn b_hs(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::HS), &[op0.as_operand()]);
    }
    fn b_cc(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::CC), &[op0.as_operand()]);
    }
    fn b_lo(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::LO), &[op0.as_operand()]);
    }
    fn b_mi(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::MI), &[op0.as_operand()]);
    }
    fn b_pl(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::PL), &[op0.as_operand()]);
    }
    fn b_vs(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::VS), &[op0.as_operand()]);
    }
    fn b_vc(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::VC), &[op0.as_operand()]);
    }
    fn b_hi(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::HI), &[op0.as_operand()]);
    }
    fn b_ls(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::LS), &[op0.as_operand()]);
    }
    fn b_ge(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::GE), &[op0.as_operand()]);
    }
    fn b_lt(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::LT), &[op0.as_operand()]);
    }
    fn b_gt(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::GT), &[op0.as_operand()]);
    }
    fn b_le(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::LE), &[op0.as_operand()]);
    }
    fn b_al(&mut self, op0: Label) {
        self.emit_n(InstId::B.with_cc(CondCode::AL), &[op0.as_operand()]);
    }
}

impl BlEmitter<Imm> for Assembler<'_> {
    fn bl(&mut self, op0: Imm) {
        self.emit_n(InstId::Bl, &[op0.as_operand()]);
    }
}

impl BlEmitter<Label> for Assembler<'_> {
    fn bl(&mut self, op0: Label) {
        self.emit_n(InstId::Bl, &[op0.as_operand()]);
    }
}

impl BlEmitter<Sym> for Assembler<'_> {
    fn bl(&mut self, op0: Sym) {
        self.emit_n(InstId::Bl, &[op0.as_operand()]);
    }
}

impl BlrEmitter<Gp> for Assembler<'_> {
    fn blr(&mut self, op0: Gp) {
        self.emit_n(InstId::Blr, &[op0.as_operand()]);
    }
}

impl BrEmitter<Gp> for Assembler<'_> {
    fn br(&mut self, op0: Gp) {
        self.emit_n(InstId::Br, &[op0.as_operand()]);
    }
}

impl CbzEmitter<Gp, Imm> for Assembler<'_> {
    fn cbz(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Cbz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CbzEmitter<Gp, Label> for Assembler<'_> {
    fn cbz(&mut self, op0: Gp, op1: Label) {
        self.emit_n(InstId::Cbz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CbzEmitter<Gp, Sym> for Assembler<'_> {
    fn cbz(&mut self, op0: Gp, op1: Sym) {
        self.emit_n(InstId::Cbz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CbnzEmitter<Gp, Imm> for Assembler<'_> {
    fn cbnz(&mut self, op0: Gp, op1: Imm) {
        self.emit_n(InstId::Cbnz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CbnzEmitter<Gp, Label> for Assembler<'_> {
    fn cbnz(&mut self, op0: Gp, op1: Label) {
        self.emit_n(InstId::Cbnz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CbnzEmitter<Gp, Sym> for Assembler<'_> {
    fn cbnz(&mut self, op0: Gp, op1: Sym) {
        self.emit_n(InstId::Cbnz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl TbnzEmitter<Gp, Imm, Imm> for Assembler<'_> {
    fn tbnz(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Tbnz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl TbnzEmitter<Gp, Imm, Label> for Assembler<'_> {
    fn tbnz(&mut self, op0: Gp, op1: Imm, op2: Label) {
        self.emit_n(
            InstId::Tbnz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl TbnzEmitter<Gp, Imm, Sym> for Assembler<'_> {
    fn tbnz(&mut self, op0: Gp, op1: Imm, op2: Sym) {
        self.emit_n(
            InstId::Tbnz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl TbzEmitter<Gp, Imm, Imm> for Assembler<'_> {
    fn tbz(&mut self, op0: Gp, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Tbz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl TbzEmitter<Gp, Imm, Label> for Assembler<'_> {
    fn tbz(&mut self, op0: Gp, op1: Imm, op2: Label) {
        self.emit_n(
            InstId::Tbz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl TbzEmitter<Gp, Imm, Sym> for Assembler<'_> {
    fn tbz(&mut self, op0: Gp, op1: Imm, op2: Sym) {
        self.emit_n(
            InstId::Tbz,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn cas(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Cas,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casa(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casa,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CasbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CashEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn cash(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Cash,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CaslEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn casl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Casl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CaslbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn caslb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Caslb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CaslhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn caslh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Caslh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CaspEmitter<Gp, Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn casp(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp, op4: Mem) {
        self.emit_n(
            InstId::Casp,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl CaspaEmitter<Gp, Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn caspa(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp, op4: Mem) {
        self.emit_n(
            InstId::Caspa,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl CaspalEmitter<Gp, Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn caspal(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp, op4: Mem) {
        self.emit_n(
            InstId::Caspal,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl CasplEmitter<Gp, Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn caspl(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Gp, op4: Mem) {
        self.emit_n(
            InstId::Caspl,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl LdaddEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldadd(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldadd,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldadda(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldadda,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaddlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaddlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaddlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdarEmitter<Gp, Mem> for Assembler<'_> {
    fn ldar(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldar, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdarbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldarb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldarb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdarhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldarh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldarh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdaxrEmitter<Gp, Mem> for Assembler<'_> {
    fn ldaxr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldaxr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdaxrbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldaxrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldaxrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdaxrhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldaxrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldaxrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdclrEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclr(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclraEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclra(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclra,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclralEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclral(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclral,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclralbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclralb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclralb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclralhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclralh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclralh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdclrlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldclrlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldclrlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeor(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeor,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeoraEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeora(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeora,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeoralEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeoral(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeoral,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeoralbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeoralb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeoralb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeoralhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeoralh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeoralh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdeorlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldeorlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldeorlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdlarEmitter<Gp, Mem> for Assembler<'_> {
    fn ldlar(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldlar, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdlarbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldlarb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldlarb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdlarhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldlarh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldlarh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdnpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldnp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldnp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdnpEmitter<Vec, Vec, Mem> for Assembler<'_> {
    fn ldnp(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Ldnp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdpEmitter<Vec, Vec, Mem> for Assembler<'_> {
    fn ldp(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Ldp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdpswEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldpsw(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldpsw,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdrEmitter<Gp, Mem> for Assembler<'_> {
    fn ldr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrEmitter<Vec, Mem> for Assembler<'_> {
    fn ldr(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Ldr_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrsbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrsb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrsb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrshEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrsh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrsh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrswEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrsw(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrsw, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdsetEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldset(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldset,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldseta(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldseta,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsethEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldseth(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldseth,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsetlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsetlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsetlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmax(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxa(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxa,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsmaxlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmaxlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmaxlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmin(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsmina(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsmina,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdsminlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldsminlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldsminlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdtrEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdtrbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdtrhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdtrsbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtrsb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtrsb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdtrshEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtrsh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtrsh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdtrswEmitter<Gp, Mem> for Assembler<'_> {
    fn ldtrsw(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldtrsw, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdumaxEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumax(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxa(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxa,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdumaxlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumaxlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumaxlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumin(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldumina(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldumina,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminlEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminlbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminlb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminlb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LduminlhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn lduminlh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Lduminlh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdurEmitter<Gp, Mem> for Assembler<'_> {
    fn ldur(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldur, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdurEmitter<Vec, Mem> for Assembler<'_> {
    fn ldur(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Ldur_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdurbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldurb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldurb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdurhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldurh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldurh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdursbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldursb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldursb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdurshEmitter<Gp, Mem> for Assembler<'_> {
    fn ldursh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldursh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdurswEmitter<Gp, Mem> for Assembler<'_> {
    fn ldursw(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldursw, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdxpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldxp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldxp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdaxpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn ldaxp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Ldaxp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl LdxrEmitter<Gp, Mem> for Assembler<'_> {
    fn ldxr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldxr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdxrbEmitter<Gp, Mem> for Assembler<'_> {
    fn ldxrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldxrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdxrhEmitter<Gp, Mem> for Assembler<'_> {
    fn ldxrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldxrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl PrfmEmitter<Imm, Mem> for Assembler<'_> {
    fn prfm(&mut self, op0: Imm, op1: Mem) {
        self.emit_n(InstId::Prfm, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddEmitter<Gp, Mem> for Assembler<'_> {
    fn stadd(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stadd, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddbEmitter<Gp, Mem> for Assembler<'_> {
    fn staddb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Staddb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddhEmitter<Gp, Mem> for Assembler<'_> {
    fn staddh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Staddh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddlEmitter<Gp, Mem> for Assembler<'_> {
    fn staddl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Staddl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddlbEmitter<Gp, Mem> for Assembler<'_> {
    fn staddlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Staddlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StaddlhEmitter<Gp, Mem> for Assembler<'_> {
    fn staddlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Staddlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrEmitter<Gp, Mem> for Assembler<'_> {
    fn stclr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrbEmitter<Gp, Mem> for Assembler<'_> {
    fn stclrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrhEmitter<Gp, Mem> for Assembler<'_> {
    fn stclrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrlEmitter<Gp, Mem> for Assembler<'_> {
    fn stclrl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclrl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stclrlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclrlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StclrlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stclrlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stclrlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorEmitter<Gp, Mem> for Assembler<'_> {
    fn steor(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steor, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorbEmitter<Gp, Mem> for Assembler<'_> {
    fn steorb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steorb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorhEmitter<Gp, Mem> for Assembler<'_> {
    fn steorh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steorh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorlEmitter<Gp, Mem> for Assembler<'_> {
    fn steorl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steorl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorlbEmitter<Gp, Mem> for Assembler<'_> {
    fn steorlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steorlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SteorlhEmitter<Gp, Mem> for Assembler<'_> {
    fn steorlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Steorlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StllrEmitter<Gp, Mem> for Assembler<'_> {
    fn stllr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StllrbEmitter<Gp, Mem> for Assembler<'_> {
    fn stllrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StllrhEmitter<Gp, Mem> for Assembler<'_> {
    fn stllrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StlrEmitter<Gp, Mem> for Assembler<'_> {
    fn stlr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StlrbEmitter<Gp, Mem> for Assembler<'_> {
    fn stlrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StlrhEmitter<Gp, Mem> for Assembler<'_> {
    fn stlrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stllrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StlxrEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stlxr(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stlxr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StlxrbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stlxrb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stlxrb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StlxrhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stlxrh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stlxrh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StnpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stnp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stnp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StnpEmitter<Vec, Vec, Mem> for Assembler<'_> {
    fn stnp(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Stnp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StpEmitter<Vec, Vec, Mem> for Assembler<'_> {
    fn stp(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Stp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StrEmitter<Gp, Mem> for Assembler<'_> {
    fn str(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Str, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StrEmitter<Vec, Mem> for Assembler<'_> {
    fn str(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Str_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StrbEmitter<Gp, Mem> for Assembler<'_> {
    fn strb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Strb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StrhEmitter<Gp, Mem> for Assembler<'_> {
    fn strh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Strh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsetEmitter<Gp, Mem> for Assembler<'_> {
    fn stset(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stset, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsetbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsetb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsetb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsethEmitter<Gp, Mem> for Assembler<'_> {
    fn stseth(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stseth, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsetlEmitter<Gp, Mem> for Assembler<'_> {
    fn stsetl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsetl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsetlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsetlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsetlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsetlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stsetlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsetlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmax(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmax, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmaxb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmaxb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxhEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmaxh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmaxh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxlEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmaxl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmaxl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmaxlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmaxlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsmaxlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmaxlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmaxlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminEmitter<Gp, Mem> for Assembler<'_> {
    fn stsmin(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsmin, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsminb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsminb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminhEmitter<Gp, Mem> for Assembler<'_> {
    fn stsminh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsminh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminlEmitter<Gp, Mem> for Assembler<'_> {
    fn stsminl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsminl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stsminlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsminlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StsminlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stsminlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stsminlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SttrEmitter<Gp, Mem> for Assembler<'_> {
    fn sttr(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Sttr, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SttrbEmitter<Gp, Mem> for Assembler<'_> {
    fn sttrb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Sttrb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SttrhEmitter<Gp, Mem> for Assembler<'_> {
    fn sttrh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Sttrh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxEmitter<Gp, Mem> for Assembler<'_> {
    fn stumax(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumax, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxbEmitter<Gp, Mem> for Assembler<'_> {
    fn stumaxb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumaxb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxhEmitter<Gp, Mem> for Assembler<'_> {
    fn stumaxh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumaxh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxlEmitter<Gp, Mem> for Assembler<'_> {
    fn stumaxl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumaxl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stumaxlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumaxlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StumaxlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stumaxlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumaxlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminEmitter<Gp, Mem> for Assembler<'_> {
    fn stumin(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stumin, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminbEmitter<Gp, Mem> for Assembler<'_> {
    fn stuminb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stuminb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminhEmitter<Gp, Mem> for Assembler<'_> {
    fn stuminh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stuminh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminlEmitter<Gp, Mem> for Assembler<'_> {
    fn stuminl(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stuminl, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminlbEmitter<Gp, Mem> for Assembler<'_> {
    fn stuminlb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stuminlb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StuminlhEmitter<Gp, Mem> for Assembler<'_> {
    fn stuminlh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stuminlh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SturEmitter<Gp, Mem> for Assembler<'_> {
    fn stur(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stur, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SturEmitter<Vec, Mem> for Assembler<'_> {
    fn stur(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Stur_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SturbEmitter<Gp, Mem> for Assembler<'_> {
    fn sturb(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Sturb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SturhEmitter<Gp, Mem> for Assembler<'_> {
    fn sturh(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Sturh, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StxpEmitter<Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn stxp(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Mem) {
        self.emit_n(
            InstId::Stxp,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl StlxpEmitter<Gp, Gp, Gp, Mem> for Assembler<'_> {
    fn stlxp(&mut self, op0: Gp, op1: Gp, op2: Gp, op3: Mem) {
        self.emit_n(
            InstId::Stlxp,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl StxrEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stxr(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stxr,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StxrbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stxrb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stxrb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StxrhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stxrh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stxrh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpaEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpa(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpa,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpabEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpab(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpab,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpahEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpah(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpah,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpalEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpal(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpal,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpalbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpalb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpalb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpalhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpalh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpalh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwpbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwphEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swph(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swph,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwplEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swpl(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swpl,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwplbEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swplb(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swplb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SwplhEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn swplh(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Swplh,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BtiEmitter<Imm> for Assembler<'_> {
    fn bti(&mut self, op0: Imm) {
        self.emit_n(InstId::Bti, &[op0.as_operand()]);
    }
}

impl ChkfeatEmitter<Gp> for Assembler<'_> {
    fn chkfeat(&mut self, op0: Gp) {
        self.emit_n(InstId::Chkfeat, &[op0.as_operand()]);
    }
}

impl ClrbhbEmitter for Assembler<'_> {
    fn clrbhb(&mut self) {
        self.emit_n(InstId::Clrbhb, &[]);
    }
}

impl Crc32bEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32b(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32b,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32hEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32h(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32h,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32wEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32w(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32w,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32xEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32x(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32x,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32cbEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32cb(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32cb,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32chEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32ch(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32ch,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32cwEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32cw(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32cw,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Crc32cxEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn crc32cx(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Crc32cx,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AbsEmitter<Gp, Gp> for Assembler<'_> {
    fn abs(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Abs, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AbsEmitter<Vec, Vec> for Assembler<'_> {
    fn abs(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Abs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CntEmitter<Gp, Gp> for Assembler<'_> {
    fn cnt(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Cnt, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CntEmitter<Vec, Vec> for Assembler<'_> {
    fn cnt(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Cnt_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl CtzEmitter<Gp, Gp> for Assembler<'_> {
    fn ctz(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Ctz, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SmaxEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn smax(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Smax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmaxEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn smax(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Smax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmaxEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smax(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smax_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SminEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn smin(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Smin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SminEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn smin(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Smin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SminEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smin(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smin_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmaxEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn umax(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Umax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmaxEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn umax(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Umax,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmaxEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umax(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umax_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UminEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn umin(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Umin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UminEmitter<Gp, Gp, Imm> for Assembler<'_> {
    fn umin(&mut self, op0: Gp, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Umin,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UminEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umin(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umin_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl DghEmitter for Assembler<'_> {
    fn dgh(&mut self) {
        self.emit_n(InstId::Dgh, &[]);
    }
}

impl CfinvEmitter for Assembler<'_> {
    fn cfinv(&mut self) {
        self.emit_n(InstId::Cfinv, &[]);
    }
}

impl Setf8Emitter<Gp> for Assembler<'_> {
    fn setf8(&mut self, op0: Gp) {
        self.emit_n(InstId::Setf8, &[op0.as_operand()]);
    }
}

impl Setf16Emitter<Gp> for Assembler<'_> {
    fn setf16(&mut self, op0: Gp) {
        self.emit_n(InstId::Setf16, &[op0.as_operand()]);
    }
}

impl AxflagEmitter for Assembler<'_> {
    fn axflag(&mut self) {
        self.emit_n(InstId::Axflag, &[]);
    }
}

impl XaflagEmitter for Assembler<'_> {
    fn xaflag(&mut self) {
        self.emit_n(InstId::Xaflag, &[]);
    }
}

impl BcEmitter<Imm> for Assembler<'_> {
    fn bc_eq(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::EQ), &[op0.as_operand()]);
    }
    fn bc_ne(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::NE), &[op0.as_operand()]);
    }
    fn bc_cs(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CS), &[op0.as_operand()]);
    }
    fn bc_hs(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HS), &[op0.as_operand()]);
    }
    fn bc_cc(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CC), &[op0.as_operand()]);
    }
    fn bc_lo(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LO), &[op0.as_operand()]);
    }
    fn bc_mi(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::MI), &[op0.as_operand()]);
    }
    fn bc_pl(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::PL), &[op0.as_operand()]);
    }
    fn bc_vs(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VS), &[op0.as_operand()]);
    }
    fn bc_vc(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VC), &[op0.as_operand()]);
    }
    fn bc_hi(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HI), &[op0.as_operand()]);
    }
    fn bc_ls(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LS), &[op0.as_operand()]);
    }
    fn bc_ge(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GE), &[op0.as_operand()]);
    }
    fn bc_lt(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LT), &[op0.as_operand()]);
    }
    fn bc_gt(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GT), &[op0.as_operand()]);
    }
    fn bc_le(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LE), &[op0.as_operand()]);
    }
    fn bc_al(&mut self, op0: Imm) {
        self.emit_n(InstId::Bc.with_cc(CondCode::AL), &[op0.as_operand()]);
    }
}

impl BcEmitter<Label> for Assembler<'_> {
    fn bc_eq(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::EQ), &[op0.as_operand()]);
    }
    fn bc_ne(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::NE), &[op0.as_operand()]);
    }
    fn bc_cs(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CS), &[op0.as_operand()]);
    }
    fn bc_hs(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HS), &[op0.as_operand()]);
    }
    fn bc_cc(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CC), &[op0.as_operand()]);
    }
    fn bc_lo(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LO), &[op0.as_operand()]);
    }
    fn bc_mi(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::MI), &[op0.as_operand()]);
    }
    fn bc_pl(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::PL), &[op0.as_operand()]);
    }
    fn bc_vs(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VS), &[op0.as_operand()]);
    }
    fn bc_vc(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VC), &[op0.as_operand()]);
    }
    fn bc_hi(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HI), &[op0.as_operand()]);
    }
    fn bc_ls(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LS), &[op0.as_operand()]);
    }
    fn bc_ge(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GE), &[op0.as_operand()]);
    }
    fn bc_lt(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LT), &[op0.as_operand()]);
    }
    fn bc_gt(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GT), &[op0.as_operand()]);
    }
    fn bc_le(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LE), &[op0.as_operand()]);
    }
    fn bc_al(&mut self, op0: Label) {
        self.emit_n(InstId::Bc.with_cc(CondCode::AL), &[op0.as_operand()]);
    }
}

impl BcEmitter<Sym> for Assembler<'_> {
    fn bc_eq(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::EQ), &[op0.as_operand()]);
    }
    fn bc_ne(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::NE), &[op0.as_operand()]);
    }
    fn bc_cs(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CS), &[op0.as_operand()]);
    }
    fn bc_hs(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HS), &[op0.as_operand()]);
    }
    fn bc_cc(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::CC), &[op0.as_operand()]);
    }
    fn bc_lo(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LO), &[op0.as_operand()]);
    }
    fn bc_mi(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::MI), &[op0.as_operand()]);
    }
    fn bc_pl(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::PL), &[op0.as_operand()]);
    }
    fn bc_vs(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VS), &[op0.as_operand()]);
    }
    fn bc_vc(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::VC), &[op0.as_operand()]);
    }
    fn bc_hi(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::HI), &[op0.as_operand()]);
    }
    fn bc_ls(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LS), &[op0.as_operand()]);
    }
    fn bc_ge(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GE), &[op0.as_operand()]);
    }
    fn bc_lt(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LT), &[op0.as_operand()]);
    }
    fn bc_gt(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::GT), &[op0.as_operand()]);
    }
    fn bc_le(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::LE), &[op0.as_operand()]);
    }
    fn bc_al(&mut self, op0: Sym) {
        self.emit_n(InstId::Bc.with_cc(CondCode::AL), &[op0.as_operand()]);
    }
}

impl AutdaEmitter<Gp, Gp> for Assembler<'_> {
    fn autda(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Autda, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AutdbEmitter<Gp, Gp> for Assembler<'_> {
    fn autdb(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Autdb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AutdzaEmitter<Gp> for Assembler<'_> {
    fn autdza(&mut self, op0: Gp) {
        self.emit_n(InstId::Autdza, &[op0.as_operand()]);
    }
}

impl AutdzbEmitter<Gp> for Assembler<'_> {
    fn autdzb(&mut self, op0: Gp) {
        self.emit_n(InstId::Autdzb, &[op0.as_operand()]);
    }
}

impl AutiaEmitter<Gp, Gp> for Assembler<'_> {
    fn autia(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Autia, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Autia1716Emitter for Assembler<'_> {
    fn autia1716(&mut self) {
        self.emit_n(InstId::Autia1716, &[]);
    }
}

impl AutiaspEmitter for Assembler<'_> {
    fn autiasp(&mut self) {
        self.emit_n(InstId::Autiasp, &[]);
    }
}

impl AutiazEmitter for Assembler<'_> {
    fn autiaz(&mut self) {
        self.emit_n(InstId::Autiaz, &[]);
    }
}

impl AutibEmitter<Gp, Gp> for Assembler<'_> {
    fn autib(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Autib, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Autib1716Emitter for Assembler<'_> {
    fn autib1716(&mut self) {
        self.emit_n(InstId::Autib1716, &[]);
    }
}

impl AutibspEmitter for Assembler<'_> {
    fn autibsp(&mut self) {
        self.emit_n(InstId::Autibsp, &[]);
    }
}

impl AutibzEmitter for Assembler<'_> {
    fn autibz(&mut self) {
        self.emit_n(InstId::Autibz, &[]);
    }
}

impl AutizaEmitter<Gp> for Assembler<'_> {
    fn autiza(&mut self, op0: Gp) {
        self.emit_n(InstId::Autiza, &[op0.as_operand()]);
    }
}

impl AutizbEmitter<Gp> for Assembler<'_> {
    fn autizb(&mut self, op0: Gp) {
        self.emit_n(InstId::Autizb, &[op0.as_operand()]);
    }
}

impl GmiEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn gmi(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Gmi,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmppEmitter<Gp, Gp> for Assembler<'_> {
    fn cmpp(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Cmpp, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AddgEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn addg(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Addg,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl LdgEmitter<Gp, Mem> for Assembler<'_> {
    fn ldg(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldg, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdgmEmitter<Gp, Mem> for Assembler<'_> {
    fn ldgm(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldgm, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdraaEmitter<Gp, Mem> for Assembler<'_> {
    fn ldraa(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldraa, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl LdrabEmitter<Gp, Mem> for Assembler<'_> {
    fn ldrab(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Ldrab, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl PacdaEmitter<Gp, Gp> for Assembler<'_> {
    fn pacda(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Pacda, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl PacdbEmitter<Gp, Gp> for Assembler<'_> {
    fn pacdb(&mut self, op0: Gp, op1: Gp) {
        self.emit_n(InstId::Pacdb, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl PacdzaEmitter<Gp> for Assembler<'_> {
    fn pacdza(&mut self, op0: Gp) {
        self.emit_n(InstId::Pacdza, &[op0.as_operand()]);
    }
}

impl PacdzbEmitter<Gp> for Assembler<'_> {
    fn pacdzb(&mut self, op0: Gp) {
        self.emit_n(InstId::Pacdzb, &[op0.as_operand()]);
    }
}

impl PacgaEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn pacga(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Pacga,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubpEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn subp(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Subp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubpsEmitter<Gp, Gp, Gp> for Assembler<'_> {
    fn subps(&mut self, op0: Gp, op1: Gp, op2: Gp) {
        self.emit_n(
            InstId::Subps,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SubgEmitter<Gp, Gp, Imm, Imm> for Assembler<'_> {
    fn subg(&mut self, op0: Gp, op1: Gp, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Subg,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl St2gEmitter<Gp, Mem> for Assembler<'_> {
    fn st2g(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::St2g, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StgEmitter<Gp, Mem> for Assembler<'_> {
    fn stg(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stg, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StgpEmitter<Gp, Gp, Mem> for Assembler<'_> {
    fn stgp(&mut self, op0: Gp, op1: Gp, op2: Mem) {
        self.emit_n(
            InstId::Stgp,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl StgmEmitter<Gp, Mem> for Assembler<'_> {
    fn stgm(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stgm, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StzgEmitter<Gp, Mem> for Assembler<'_> {
    fn stzg(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stzg, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Stz2gEmitter<Gp, Mem> for Assembler<'_> {
    fn stz2g(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stz2g, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl StzgmEmitter<Gp, Mem> for Assembler<'_> {
    fn stzgm(&mut self, op0: Gp, op1: Mem) {
        self.emit_n(InstId::Stzgm, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl XpacdEmitter<Gp> for Assembler<'_> {
    fn xpacd(&mut self, op0: Gp) {
        self.emit_n(InstId::Xpacd, &[op0.as_operand()]);
    }
}

impl XpaciEmitter<Gp> for Assembler<'_> {
    fn xpaci(&mut self, op0: Gp) {
        self.emit_n(InstId::Xpaci, &[op0.as_operand()]);
    }
}

impl XpaclriEmitter for Assembler<'_> {
    fn xpaclri(&mut self) {
        self.emit_n(InstId::Xpaclri, &[]);
    }
}

impl HintEmitter<Imm> for Assembler<'_> {
    fn hint(&mut self, op0: Imm) {
        self.emit_n(InstId::Hint, &[op0.as_operand()]);
    }
}

impl NopEmitter for Assembler<'_> {
    fn nop(&mut self) {
        self.emit_n(InstId::Nop, &[]);
    }
}

impl SevEmitter for Assembler<'_> {
    fn sev(&mut self) {
        self.emit_n(InstId::Sev, &[]);
    }
}

impl SevlEmitter for Assembler<'_> {
    fn sevl(&mut self) {
        self.emit_n(InstId::Sevl, &[]);
    }
}

impl WfeEmitter for Assembler<'_> {
    fn wfe(&mut self) {
        self.emit_n(InstId::Wfe, &[]);
    }
}

impl WfiEmitter for Assembler<'_> {
    fn wfi(&mut self) {
        self.emit_n(InstId::Wfi, &[]);
    }
}

impl YieldEmitter for Assembler<'_> {
    fn r#yield(&mut self) {
        self.emit_n(InstId::Yield, &[]);
    }
}

impl AddhnEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn addhn(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Addhn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Addhn2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn addhn2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Addhn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddpEmitter<Vec, Vec> for Assembler<'_> {
    fn addp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Addp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Addp3Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn addp_3(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Addp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AddvEmitter<Vec, Vec> for Assembler<'_> {
    fn addv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Addv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Bic2Emitter<Vec, Imm> for Assembler<'_> {
    fn bic_2(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Bic_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl BifEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bif(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bif_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BitEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bit(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bit_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BslEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bsl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bsl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmeqEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmeq(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmeq_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmeqEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn cmeq(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Cmeq_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmgeEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmge(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmge_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmgeEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn cmge(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Cmge_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmgtEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmgt(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmgt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmgtEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn cmgt(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Cmgt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmhiEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmhi(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmhi_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmhsEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmhs(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmhs_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmleEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn cmle(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Cmle_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmltEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn cmlt(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Cmlt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl CmtstEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn cmtst(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Cmtst_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl DupEmitter<Vec, Gp> for Assembler<'_> {
    fn dup(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Dup_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl DupEmitter<Vec, Vec> for Assembler<'_> {
    fn dup(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Dup_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl ExtEmitter<Vec, Vec, Vec, Imm> for Assembler<'_> {
    fn ext(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Imm) {
        self.emit_n(
            InstId::Ext_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FabdEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fabd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fabd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FabsEmitter<Vec, Vec> for Assembler<'_> {
    fn fabs(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fabs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FacgeEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn facge(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Facge_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FacgtEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn facgt(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Facgt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FaddpEmitter<Vec, Vec> for Assembler<'_> {
    fn faddp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Faddp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Faddp3Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn faddp_3(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Faddp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FccmpEmitter<Vec, Vec, Imm, Imm> for Assembler<'_> {
    fn fccmp(&mut self, op0: Vec, op1: Vec, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Fccmp_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FccmpeEmitter<Vec, Vec, Imm, Imm> for Assembler<'_> {
    fn fccmpe(&mut self, op0: Vec, op1: Vec, op2: Imm, op3: Imm) {
        self.emit_n(
            InstId::Fccmpe_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FcmeqEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fcmeq(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fcmeq_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmeqEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcmeq(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcmeq_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmgeEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fcmge(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fcmge_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmgeEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcmge(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcmge_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmgtEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fcmgt(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fcmgt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmgtEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcmgt(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcmgt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmleEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcmle(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcmle_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmltEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcmlt(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcmlt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcmpEmitter<Vec, Vec> for Assembler<'_> {
    fn fcmp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcmp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcmpEmitter<Vec, Imm> for Assembler<'_> {
    fn fcmp(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Fcmp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcmpeEmitter<Vec, Vec> for Assembler<'_> {
    fn fcmpe(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcmpe_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcmpeEmitter<Vec, Imm> for Assembler<'_> {
    fn fcmpe(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Fcmpe_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcselEmitter<Vec, Vec, Vec, Imm> for Assembler<'_> {
    fn fcsel(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Imm) {
        self.emit_n(
            InstId::Fcsel_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FcvtEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvt(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvt_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtasEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtas(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtas_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtasEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtas(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtas_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtauEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtau(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtau_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtauEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtau(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtau_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtlEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtl(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtl_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fcvtl2Emitter<Vec, Vec> for Assembler<'_> {
    fn fcvtl2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtl2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtmsEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtms(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtms_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtmsEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtms(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtms_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtmuEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtmu(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtmu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtmuEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtmu(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtmu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtnEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fcvtn2Emitter<Vec, Vec> for Assembler<'_> {
    fn fcvtn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtnsEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtns(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtns_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtnsEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtns(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtns_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtnuEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtnu(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtnu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtnuEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtnu(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtnu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtpsEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtps(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtps_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtpsEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtps(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtps_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtpuEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtpu(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtpu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtpuEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtpu(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtpu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtxnEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtxn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtxn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fcvtxn2Emitter<Vec, Vec> for Assembler<'_> {
    fn fcvtxn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtxn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtzsEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtzs(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtzs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtzsEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtzs(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtzs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fcvtzs3Emitter<Gp, Vec, Imm> for Assembler<'_> {
    fn fcvtzs_3(&mut self, op0: Gp, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcvtzs_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fcvtzs3Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcvtzs_3(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcvtzs_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcvtzuEmitter<Gp, Vec> for Assembler<'_> {
    fn fcvtzu(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fcvtzu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FcvtzuEmitter<Vec, Vec> for Assembler<'_> {
    fn fcvtzu(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fcvtzu_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fcvtzu3Emitter<Gp, Vec, Imm> for Assembler<'_> {
    fn fcvtzu_3(&mut self, op0: Gp, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcvtzu_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fcvtzu3Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn fcvtzu_3(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Fcvtzu_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FdivEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fdiv(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fdiv_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmaddEmitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn fmadd(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Fmadd_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FmaxEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmax(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmax_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmaxnmEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmaxnm(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmaxnm_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmaxnmpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmaxnmp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmaxnmp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fmaxnmp2Emitter<Vec, Vec> for Assembler<'_> {
    fn fmaxnmp_2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fmaxnmp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmaxnmvEmitter<Vec, Vec> for Assembler<'_> {
    fn fmaxnmv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fmaxnmv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmaxpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmaxp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmaxp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fmaxp2Emitter<Vec, Vec> for Assembler<'_> {
    fn fmaxp_2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fmaxp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmaxvEmitter<Vec, Vec> for Assembler<'_> {
    fn fmaxv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fmaxv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FminEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmin(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmin_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FminnmEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fminnm(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fminnm_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FminnmvEmitter<Vec, Vec> for Assembler<'_> {
    fn fminnmv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fminnmv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FminnmpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fminnmp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fminnmp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fminnmp2Emitter<Vec, Vec> for Assembler<'_> {
    fn fminnmp_2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fminnmp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FminpEmitter<Vec, Vec> for Assembler<'_> {
    fn fminp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fminp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Fminp3Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fminp_3(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fminp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FminvEmitter<Vec, Vec> for Assembler<'_> {
    fn fminv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fminv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmlsEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmls(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmls_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmovEmitter<Gp, Vec> for Assembler<'_> {
    fn fmov(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fmov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmovEmitter<Vec, Gp> for Assembler<'_> {
    fn fmov(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Fmov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmovEmitter<Vec, Vec> for Assembler<'_> {
    fn fmov(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fmov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmovEmitter<Vec, Imm> for Assembler<'_> {
    fn fmov(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Fmov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmsubEmitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn fmsub(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Fmsub_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FmulEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmul(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmul_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmulxEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmulx(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmulx_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FnegEmitter<Vec, Vec> for Assembler<'_> {
    fn fneg(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fneg_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FnmaddEmitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn fnmadd(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Fnmadd_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FnmsubEmitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn fnmsub(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Fnmsub_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FnmulEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fnmul(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fnmul_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FrecpeEmitter<Vec, Vec> for Assembler<'_> {
    fn frecpe(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frecpe_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrecpsEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn frecps(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Frecps_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FrecpxEmitter<Vec, Vec> for Assembler<'_> {
    fn frecpx(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frecpx_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Frint32xEmitter<Vec, Vec> for Assembler<'_> {
    fn frint32x(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frint32x_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Frint32zEmitter<Vec, Vec> for Assembler<'_> {
    fn frint32z(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frint32z_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Frint64xEmitter<Vec, Vec> for Assembler<'_> {
    fn frint64x(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frint64x_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Frint64zEmitter<Vec, Vec> for Assembler<'_> {
    fn frint64z(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frint64z_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintaEmitter<Vec, Vec> for Assembler<'_> {
    fn frinta(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frinta_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintiEmitter<Vec, Vec> for Assembler<'_> {
    fn frinti(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frinti_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintmEmitter<Vec, Vec> for Assembler<'_> {
    fn frintm(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frintm_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintnEmitter<Vec, Vec> for Assembler<'_> {
    fn frintn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frintn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintpEmitter<Vec, Vec> for Assembler<'_> {
    fn frintp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frintp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintxEmitter<Vec, Vec> for Assembler<'_> {
    fn frintx(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frintx_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrintzEmitter<Vec, Vec> for Assembler<'_> {
    fn frintz(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frintz_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrsqrteEmitter<Vec, Vec> for Assembler<'_> {
    fn frsqrte(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Frsqrte_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FrsqrtsEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn frsqrts(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Frsqrts_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FsqrtEmitter<Vec, Vec> for Assembler<'_> {
    fn fsqrt(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Fsqrt_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FsubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fsub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fsub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl InsEmitter<Vec, Gp> for Assembler<'_> {
    fn ins(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Ins_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl InsEmitter<Vec, Vec> for Assembler<'_> {
    fn ins(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Ins_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Ld1Emitter<Vec, Mem> for Assembler<'_> {
    fn ld1(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Ld1_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Ld13Emitter<Vec, Vec, Mem> for Assembler<'_> {
    fn ld1_3(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Ld1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ld14Emitter<Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld1_4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Mem) {
        self.emit_n(
            InstId::Ld1_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Ld15Emitter<Vec, Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld1_5(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Mem) {
        self.emit_n(
            InstId::Ld1_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl Ld1rEmitter<Vec, Mem> for Assembler<'_> {
    fn ld1r(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::Ld1r_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Ld2Emitter<Vec, Vec, Mem> for Assembler<'_> {
    fn ld2(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Ld2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ld2rEmitter<Vec, Vec, Mem> for Assembler<'_> {
    fn ld2r(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::Ld2r_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ld3Emitter<Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld3(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Mem) {
        self.emit_n(
            InstId::Ld3_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Ld3rEmitter<Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld3r(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Mem) {
        self.emit_n(
            InstId::Ld3r_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Ld4Emitter<Vec, Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Mem) {
        self.emit_n(
            InstId::Ld4_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl Ld4rEmitter<Vec, Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn ld4r(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Mem) {
        self.emit_n(
            InstId::Ld4r_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl MlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn mla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Mla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MlsEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn mls(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Mls_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MoviEmitter<Vec, Imm> for Assembler<'_> {
    fn movi(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Movi_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Movi3Emitter<Vec, Imm, Imm> for Assembler<'_> {
    fn movi_3(&mut self, op0: Vec, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Movi_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl MvniEmitter<Vec, Imm> for Assembler<'_> {
    fn mvni(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Mvni_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Mvni3Emitter<Vec, Imm, Imm> for Assembler<'_> {
    fn mvni_3(&mut self, op0: Vec, op1: Imm, op2: Imm) {
        self.emit_n(
            InstId::Mvni_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl NotEmitter<Vec, Vec> for Assembler<'_> {
    fn not_(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Not_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Orr2Emitter<Vec, Imm> for Assembler<'_> {
    fn orr_2(&mut self, op0: Vec, op1: Imm) {
        self.emit_n(InstId::Orr_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl PmulEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn pmul(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Pmul_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl PmullEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn pmull(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Pmull_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Pmull2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn pmull2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Pmull2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl RaddhnEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn raddhn(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Raddhn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Raddhn2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn raddhn2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Raddhn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl RshrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn rshrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Rshrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Rshrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn rshrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Rshrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl RsubhnEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn rsubhn(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Rsubhn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Rsubhn2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn rsubhn2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Rsubhn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SabaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn saba(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Saba_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SabalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sabal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sabal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sabal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sabal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sabal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SabdEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sabd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sabd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SabdlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sabdl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sabdl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sabdl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sabdl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sabdl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SadalpEmitter<Vec, Vec> for Assembler<'_> {
    fn sadalp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sadalp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SaddlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn saddl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Saddl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Saddl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn saddl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Saddl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SaddlpEmitter<Vec, Vec> for Assembler<'_> {
    fn saddlp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Saddlp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SaddlvEmitter<Vec, Vec> for Assembler<'_> {
    fn saddlv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Saddlv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SaddwEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn saddw(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Saddw_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Saddw2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn saddw2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Saddw2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ScvtfEmitter<Vec, Gp> for Assembler<'_> {
    fn scvtf(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Scvtf_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl ScvtfEmitter<Vec, Vec> for Assembler<'_> {
    fn scvtf(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Scvtf_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Scvtf3Emitter<Vec, Gp, Imm> for Assembler<'_> {
    fn scvtf_3(&mut self, op0: Vec, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Scvtf_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Scvtf3Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn scvtf_3(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Scvtf_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ShaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn shadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Shadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ShlEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn shl(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Shl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ShllEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn shll(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Shll_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Shll2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn shll2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Shll2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ShrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn shrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Shrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Shrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn shrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Shrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl ShsubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn shsub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Shsub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SliEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sli(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sli_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmaxpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smaxp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smaxp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmaxvEmitter<Vec, Vec> for Assembler<'_> {
    fn smaxv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Smaxv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SminpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sminp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sminp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SminvEmitter<Vec, Vec> for Assembler<'_> {
    fn sminv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sminv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SmlalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smlal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smlal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Smlal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smlal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smlal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmlslEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smlsl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smlsl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Smlsl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smlsl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smlsl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmovEmitter<Gp, Vec> for Assembler<'_> {
    fn smov(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Smov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Smull2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smull2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smull2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqabsEmitter<Vec, Vec> for Assembler<'_> {
    fn sqabs(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqabs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SqaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqdmlalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmlal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmlal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqdmlal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmlal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmlal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqdmlslEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmlsl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmlsl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqdmlsl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmlsl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmlsl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqdmulhEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmulh(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmulh_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqdmullEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmull(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmull_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqdmull2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqdmull2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqdmull2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqnegEmitter<Vec, Vec> for Assembler<'_> {
    fn sqneg(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqneg_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SqrdmulhEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqrdmulh(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqrdmulh_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqrshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqrshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqrshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqrshrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqrshrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqrshrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqrshrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqrshrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqrshrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqrshrunEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqrshrun(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqrshrun_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqrshrun2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqrshrun2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqrshrun2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqshlEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshl(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqshluEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshlu(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshlu_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqshrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqshrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqshrunEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshrun(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshrun_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sqshrun2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sqshrun2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sqshrun2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqsubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqsub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqsub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqxtnEmitter<Vec, Vec> for Assembler<'_> {
    fn sqxtn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqxtn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sqxtn2Emitter<Vec, Vec> for Assembler<'_> {
    fn sqxtn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqxtn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SqxtunEmitter<Vec, Vec> for Assembler<'_> {
    fn sqxtun(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqxtun_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sqxtun2Emitter<Vec, Vec> for Assembler<'_> {
    fn sqxtun2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sqxtun2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SrhaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn srhadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Srhadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SriEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sri(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sri_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SrshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn srshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Srshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SrshrEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn srshr(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Srshr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SrsraEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn srsra(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Srsra_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SshllEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sshll(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sshll_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sshll2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sshll2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sshll2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SshrEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn sshr(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Sshr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SsraEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ssra(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ssra_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SsublEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ssubl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ssubl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ssubl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ssubl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ssubl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SsubwEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ssubw(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ssubw_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ssubw2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ssubw2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ssubw2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl St1Emitter<Vec, Mem> for Assembler<'_> {
    fn st1(&mut self, op0: Vec, op1: Mem) {
        self.emit_n(InstId::St1_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl St13Emitter<Vec, Vec, Mem> for Assembler<'_> {
    fn st1_3(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::St1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl St14Emitter<Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn st1_4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Mem) {
        self.emit_n(
            InstId::St1_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl St15Emitter<Vec, Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn st1_5(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Mem) {
        self.emit_n(
            InstId::St1_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl St2Emitter<Vec, Vec, Mem> for Assembler<'_> {
    fn st2(&mut self, op0: Vec, op1: Vec, op2: Mem) {
        self.emit_n(
            InstId::St2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl St3Emitter<Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn st3(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Mem) {
        self.emit_n(
            InstId::St3_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl St4Emitter<Vec, Vec, Vec, Vec, Mem> for Assembler<'_> {
    fn st4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Mem) {
        self.emit_n(
            InstId::St4_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl SubhnEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn subhn(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Subhn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Subhn2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn subhn2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Subhn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SuqaddEmitter<Vec, Vec> for Assembler<'_> {
    fn suqadd(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Suqadd_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl SxtlEmitter<Vec, Vec> for Assembler<'_> {
    fn sxtl(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sxtl_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sxtl2Emitter<Vec, Vec> for Assembler<'_> {
    fn sxtl2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sxtl2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl TblEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn tbl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Tbl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Tbl4Emitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbl_4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Tbl_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Tbl5Emitter<Vec, Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbl_5(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Vec) {
        self.emit_n(
            InstId::Tbl_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl Tbl6Emitter<Vec, Vec, Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbl_6(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Vec, op5: Vec) {
        self.emit_n(
            InstId::Tbl_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
                op5.as_operand(),
            ],
        );
    }
}

impl TbxEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn tbx(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Tbx_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Tbx4Emitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbx_4(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Tbx_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Tbx5Emitter<Vec, Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbx_5(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Vec) {
        self.emit_n(
            InstId::Tbx_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
            ],
        );
    }
}

impl Tbx6Emitter<Vec, Vec, Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn tbx_6(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec, op4: Vec, op5: Vec) {
        self.emit_n(
            InstId::Tbx_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
                op4.as_operand(),
                op5.as_operand(),
            ],
        );
    }
}

impl Trn1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn trn1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Trn1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Trn2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn trn2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Trn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UabaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uaba(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uaba_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UabalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uabal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uabal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uabal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uabal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uabal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UabdEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uabd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uabd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UabdlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uabdl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uabdl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uabdl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uabdl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uabdl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UadalpEmitter<Vec, Vec> for Assembler<'_> {
    fn uadalp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uadalp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UaddlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uaddl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uaddl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uaddl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uaddl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uaddl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UaddlpEmitter<Vec, Vec> for Assembler<'_> {
    fn uaddlp(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uaddlp_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UaddlvEmitter<Vec, Vec> for Assembler<'_> {
    fn uaddlv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uaddlv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UaddwEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uaddw(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uaddw_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uaddw2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uaddw2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uaddw2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UcvtfEmitter<Vec, Gp> for Assembler<'_> {
    fn ucvtf(&mut self, op0: Vec, op1: Gp) {
        self.emit_n(InstId::Ucvtf_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UcvtfEmitter<Vec, Vec> for Assembler<'_> {
    fn ucvtf(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Ucvtf_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Ucvtf3Emitter<Vec, Gp, Imm> for Assembler<'_> {
    fn ucvtf_3(&mut self, op0: Vec, op1: Gp, op2: Imm) {
        self.emit_n(
            InstId::Ucvtf_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ucvtf3Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ucvtf_3(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ucvtf_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UhaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uhadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uhadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UhsubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uhsub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uhsub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmaxpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umaxp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umaxp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmaxvEmitter<Vec, Vec> for Assembler<'_> {
    fn umaxv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Umaxv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UminpEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uminp(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uminp_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UminvEmitter<Vec, Vec> for Assembler<'_> {
    fn uminv(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uminv_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UmlalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umlal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umlal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Umlal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umlal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umlal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmlslEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umlsl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umlsl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Umlsl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umlsl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umlsl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmovEmitter<Gp, Vec> for Assembler<'_> {
    fn umov(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Umov_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Umull2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn umull2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Umull2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uqadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uqadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqrshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uqrshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uqrshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqrshlEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqrshl(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqrshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqrshrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqrshrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqrshrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uqrshrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqrshrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqrshrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uqshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uqshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqshlEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqshl(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqshrnEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqshrn(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqshrn_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uqshrn2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn uqshrn2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Uqshrn2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqsubEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uqsub(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uqsub_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UqxtnEmitter<Vec, Vec> for Assembler<'_> {
    fn uqxtn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uqxtn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Uqxtn2Emitter<Vec, Vec> for Assembler<'_> {
    fn uqxtn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uqxtn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UrecpeEmitter<Vec, Vec> for Assembler<'_> {
    fn urecpe(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Urecpe_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UrhaddEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn urhadd(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Urhadd_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UrshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn urshl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Urshl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UrshrEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn urshr(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Urshr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UrsqrteEmitter<Vec, Vec> for Assembler<'_> {
    fn ursqrte(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Ursqrte_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UrsraEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ursra(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ursra_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UshlEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ushl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ushl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UshllEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ushll(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ushll_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Ushll2Emitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ushll2(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ushll2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UshrEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn ushr(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Ushr_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UsqaddEmitter<Vec, Vec> for Assembler<'_> {
    fn usqadd(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Usqadd_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl UsraEmitter<Vec, Vec, Imm> for Assembler<'_> {
    fn usra(&mut self, op0: Vec, op1: Vec, op2: Imm) {
        self.emit_n(
            InstId::Usra_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UsublEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usubl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usubl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Usubl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usubl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usubl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UsubwEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usubw(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usubw_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Usubw2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usubw2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usubw2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UxtlEmitter<Vec, Vec> for Assembler<'_> {
    fn uxtl(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uxtl_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Uxtl2Emitter<Vec, Vec> for Assembler<'_> {
    fn uxtl2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Uxtl2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Uzp1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uzp1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uzp1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Uzp2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn uzp2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Uzp2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl XtnEmitter<Vec, Vec> for Assembler<'_> {
    fn xtn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Xtn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Xtn2Emitter<Vec, Vec> for Assembler<'_> {
    fn xtn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Xtn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Zip1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn zip1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Zip1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Zip2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn zip2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Zip2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl AesdEmitter<Vec, Vec> for Assembler<'_> {
    fn aesd(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Aesd_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AeseEmitter<Vec, Vec> for Assembler<'_> {
    fn aese(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Aese_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AesimcEmitter<Vec, Vec> for Assembler<'_> {
    fn aesimc(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Aesimc_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl AesmcEmitter<Vec, Vec> for Assembler<'_> {
    fn aesmc(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Aesmc_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sha1cEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha1c(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha1c_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha1hEmitter<Vec, Vec> for Assembler<'_> {
    fn sha1h(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sha1h_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sha1mEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha1m(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha1m_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha1pEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha1p(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha1p_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha1su0Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha1su0(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha1su0_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha1su1Emitter<Vec, Vec> for Assembler<'_> {
    fn sha1su1(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sha1su1_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sha256hEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha256h(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha256h_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha256h2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha256h2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha256h2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha256su0Emitter<Vec, Vec> for Assembler<'_> {
    fn sha256su0(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sha256su0_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sha256su1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha256su1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha256su1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqrdmlahEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqrdmlah(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqrdmlah_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SqrdmlshEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sqrdmlsh(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sqrdmlsh_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FcaddEmitter<Vec, Vec, Vec, Imm> for Assembler<'_> {
    fn fcadd(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Imm) {
        self.emit_n(
            InstId::Fcadd_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FcmlaEmitter<Vec, Vec, Vec, Imm> for Assembler<'_> {
    fn fcmla(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Imm) {
        self.emit_n(
            InstId::Fcmla_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl FjcvtzsEmitter<Gp, Vec> for Assembler<'_> {
    fn fjcvtzs(&mut self, op0: Gp, op1: Vec) {
        self.emit_n(InstId::Fjcvtzs_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl FmlalEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmlal(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmlal_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fmlal2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmlal2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmlal2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl FmlslEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmlsl(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmlsl_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Fmlsl2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn fmlsl2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Fmlsl2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BcaxEmitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn bcax(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Bcax_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Eor3Emitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn eor3(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Eor3_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Rax1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn rax1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Rax1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl XarEmitter<Vec, Vec, Vec, Imm> for Assembler<'_> {
    fn xar(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Imm) {
        self.emit_n(
            InstId::Xar_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Sha512hEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha512h(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha512h_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha512h2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha512h2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha512h2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sha512su0Emitter<Vec, Vec> for Assembler<'_> {
    fn sha512su0(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sha512su0_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sha512su1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sha512su1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sha512su1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3partw1Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3partw1(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3partw1_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3partw2Emitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3partw2(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3partw2_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3ss1Emitter<Vec, Vec, Vec, Vec> for Assembler<'_> {
    fn sm3ss1(&mut self, op0: Vec, op1: Vec, op2: Vec, op3: Vec) {
        self.emit_n(
            InstId::Sm3ss1_v,
            &[
                op0.as_operand(),
                op1.as_operand(),
                op2.as_operand(),
                op3.as_operand(),
            ],
        );
    }
}

impl Sm3tt1aEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3tt1a(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3tt1a_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3tt1bEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3tt1b(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3tt1b_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3tt2aEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3tt2a(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3tt2a_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm3tt2bEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm3tt2b(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm3tt2b_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Sm4eEmitter<Vec, Vec> for Assembler<'_> {
    fn sm4e(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Sm4e_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Sm4ekeyEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sm4ekey(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sm4ekey_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SdotEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sdot(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sdot_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UdotEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn udot(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Udot_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BfcvtEmitter<Vec, Vec> for Assembler<'_> {
    fn bfcvt(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Bfcvt_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl BfcvtnEmitter<Vec, Vec> for Assembler<'_> {
    fn bfcvtn(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Bfcvtn_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl Bfcvtn2Emitter<Vec, Vec> for Assembler<'_> {
    fn bfcvtn2(&mut self, op0: Vec, op1: Vec) {
        self.emit_n(InstId::Bfcvtn2_v, &[op0.as_operand(), op1.as_operand()]);
    }
}

impl BfmlalbEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bfmlalb(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bfmlalb_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BfmlaltEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bfmlalt(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bfmlalt_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BfmmlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bfmmla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bfmmla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl BfdotEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn bfdot(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Bfdot_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SmmlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn smmla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Smmla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl SudotEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn sudot(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Sudot_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UmmlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn ummla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Ummla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UsdotEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usdot(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usdot_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl UsmmlaEmitter<Vec, Vec, Vec> for Assembler<'_> {
    fn usmmla(&mut self, op0: Vec, op1: Vec, op2: Vec) {
        self.emit_n(
            InstId::Usmmla_v,
            &[op0.as_operand(), op1.as_operand(), op2.as_operand()],
        );
    }
}

impl Assembler<'_> {
    pub fn adc<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AdcEmitter<T0, T1, T2>,
    {
        <Self as AdcEmitter<T0, T1, T2>>::adc(self, op0, op1, op2);
    }
    pub fn adcs<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AdcsEmitter<T0, T1, T2>,
    {
        <Self as AdcsEmitter<T0, T1, T2>>::adcs(self, op0, op1, op2);
    }
    pub fn add<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AddEmitter<T0, T1, T2>,
    {
        <Self as AddEmitter<T0, T1, T2>>::add(self, op0, op1, op2);
    }
    pub fn add_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Add4Emitter<T0, T1, T2, T3>,
    {
        <Self as Add4Emitter<T0, T1, T2, T3>>::add_4(self, op0, op1, op2, op3);
    }
    pub fn adds<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AddsEmitter<T0, T1, T2>,
    {
        <Self as AddsEmitter<T0, T1, T2>>::adds(self, op0, op1, op2);
    }
    pub fn adds_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Adds4Emitter<T0, T1, T2, T3>,
    {
        <Self as Adds4Emitter<T0, T1, T2, T3>>::adds_4(self, op0, op1, op2, op3);
    }
    pub fn adr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AdrEmitter<T0, T1>,
    {
        <Self as AdrEmitter<T0, T1>>::adr(self, op0, op1);
    }
    pub fn adrp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AdrpEmitter<T0, T1>,
    {
        <Self as AdrpEmitter<T0, T1>>::adrp(self, op0, op1);
    }
    pub fn and_<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AndEmitter<T0, T1, T2>,
    {
        <Self as AndEmitter<T0, T1, T2>>::and_(self, op0, op1, op2);
    }
    pub fn and__4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: And4Emitter<T0, T1, T2, T3>,
    {
        <Self as And4Emitter<T0, T1, T2, T3>>::and__4(self, op0, op1, op2, op3);
    }
    pub fn ands<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AndsEmitter<T0, T1, T2>,
    {
        <Self as AndsEmitter<T0, T1, T2>>::ands(self, op0, op1, op2);
    }
    pub fn ands_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Ands4Emitter<T0, T1, T2, T3>,
    {
        <Self as Ands4Emitter<T0, T1, T2, T3>>::ands_4(self, op0, op1, op2, op3);
    }
    pub fn asr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AsrEmitter<T0, T1, T2>,
    {
        <Self as AsrEmitter<T0, T1, T2>>::asr(self, op0, op1, op2);
    }
    pub fn asrv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AsrvEmitter<T0, T1, T2>,
    {
        <Self as AsrvEmitter<T0, T1, T2>>::asrv(self, op0, op1, op2);
    }
    pub fn at<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AtEmitter<T0, T1>,
    {
        <Self as AtEmitter<T0, T1>>::at(self, op0, op1);
    }
    pub fn bfc<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BfcEmitter<T0, T1, T2>,
    {
        <Self as BfcEmitter<T0, T1, T2>>::bfc(self, op0, op1, op2);
    }
    pub fn bfi<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: BfiEmitter<T0, T1, T2, T3>,
    {
        <Self as BfiEmitter<T0, T1, T2, T3>>::bfi(self, op0, op1, op2, op3);
    }
    pub fn bfm<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: BfmEmitter<T0, T1, T2, T3>,
    {
        <Self as BfmEmitter<T0, T1, T2, T3>>::bfm(self, op0, op1, op2, op3);
    }
    pub fn bfxil<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: BfxilEmitter<T0, T1, T2, T3>,
    {
        <Self as BfxilEmitter<T0, T1, T2, T3>>::bfxil(self, op0, op1, op2, op3);
    }
    pub fn bic<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BicEmitter<T0, T1, T2>,
    {
        <Self as BicEmitter<T0, T1, T2>>::bic(self, op0, op1, op2);
    }
    pub fn bic_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Bic4Emitter<T0, T1, T2, T3>,
    {
        <Self as Bic4Emitter<T0, T1, T2, T3>>::bic_4(self, op0, op1, op2, op3);
    }
    pub fn bics<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BicsEmitter<T0, T1, T2>,
    {
        <Self as BicsEmitter<T0, T1, T2>>::bics(self, op0, op1, op2);
    }
    pub fn bics_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Bics4Emitter<T0, T1, T2, T3>,
    {
        <Self as Bics4Emitter<T0, T1, T2, T3>>::bics_4(self, op0, op1, op2, op3);
    }
    pub fn brk<T0>(&mut self, op0: T0)
    where
        Self: BrkEmitter<T0>,
    {
        <Self as BrkEmitter<T0>>::brk(self, op0);
    }
    pub fn ccmn<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CcmnEmitter<T0, T1, T2, T3>,
    {
        <Self as CcmnEmitter<T0, T1, T2, T3>>::ccmn(self, op0, op1, op2, op3);
    }
    pub fn ccmp<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CcmpEmitter<T0, T1, T2, T3>,
    {
        <Self as CcmpEmitter<T0, T1, T2, T3>>::ccmp(self, op0, op1, op2, op3);
    }
    pub fn cinc<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CincEmitter<T0, T1, T2>,
    {
        <Self as CincEmitter<T0, T1, T2>>::cinc(self, op0, op1, op2);
    }
    pub fn cinv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CinvEmitter<T0, T1, T2>,
    {
        <Self as CinvEmitter<T0, T1, T2>>::cinv(self, op0, op1, op2);
    }
    pub fn clrex<T0>(&mut self, op0: T0)
    where
        Self: ClrexEmitter<T0>,
    {
        <Self as ClrexEmitter<T0>>::clrex(self, op0);
    }
    pub fn cls<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: ClsEmitter<T0, T1>,
    {
        <Self as ClsEmitter<T0, T1>>::cls(self, op0, op1);
    }
    pub fn clz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: ClzEmitter<T0, T1>,
    {
        <Self as ClzEmitter<T0, T1>>::clz(self, op0, op1);
    }
    pub fn cmn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CmnEmitter<T0, T1>,
    {
        <Self as CmnEmitter<T0, T1>>::cmn(self, op0, op1);
    }
    pub fn cmn_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Cmn3Emitter<T0, T1, T2>,
    {
        <Self as Cmn3Emitter<T0, T1, T2>>::cmn_3(self, op0, op1, op2);
    }
    pub fn cmp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CmpEmitter<T0, T1>,
    {
        <Self as CmpEmitter<T0, T1>>::cmp(self, op0, op1);
    }
    pub fn cmp_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Cmp3Emitter<T0, T1, T2>,
    {
        <Self as Cmp3Emitter<T0, T1, T2>>::cmp_3(self, op0, op1, op2);
    }
    pub fn cneg<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CnegEmitter<T0, T1, T2>,
    {
        <Self as CnegEmitter<T0, T1, T2>>::cneg(self, op0, op1, op2);
    }
    pub fn csel<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CselEmitter<T0, T1, T2, T3>,
    {
        <Self as CselEmitter<T0, T1, T2, T3>>::csel(self, op0, op1, op2, op3);
    }
    pub fn cset<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CsetEmitter<T0, T1>,
    {
        <Self as CsetEmitter<T0, T1>>::cset(self, op0, op1);
    }
    pub fn csetm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CsetmEmitter<T0, T1>,
    {
        <Self as CsetmEmitter<T0, T1>>::csetm(self, op0, op1);
    }
    pub fn csinc<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CsincEmitter<T0, T1, T2, T3>,
    {
        <Self as CsincEmitter<T0, T1, T2, T3>>::csinc(self, op0, op1, op2, op3);
    }
    pub fn csinv<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CsinvEmitter<T0, T1, T2, T3>,
    {
        <Self as CsinvEmitter<T0, T1, T2, T3>>::csinv(self, op0, op1, op2, op3);
    }
    pub fn csneg<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: CsnegEmitter<T0, T1, T2, T3>,
    {
        <Self as CsnegEmitter<T0, T1, T2, T3>>::csneg(self, op0, op1, op2, op3);
    }
    pub fn dc<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: DcEmitter<T0, T1>,
    {
        <Self as DcEmitter<T0, T1>>::dc(self, op0, op1);
    }
    pub fn dmb<T0>(&mut self, op0: T0)
    where
        Self: DmbEmitter<T0>,
    {
        <Self as DmbEmitter<T0>>::dmb(self, op0);
    }
    pub fn dsb<T0>(&mut self, op0: T0)
    where
        Self: DsbEmitter<T0>,
    {
        <Self as DsbEmitter<T0>>::dsb(self, op0);
    }
    pub fn drps(&mut self)
    where
        Self: DrpsEmitter,
    {
        <Self as DrpsEmitter>::drps(self);
    }
    pub fn eon<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: EonEmitter<T0, T1, T2>,
    {
        <Self as EonEmitter<T0, T1, T2>>::eon(self, op0, op1, op2);
    }
    pub fn eon_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Eon4Emitter<T0, T1, T2, T3>,
    {
        <Self as Eon4Emitter<T0, T1, T2, T3>>::eon_4(self, op0, op1, op2, op3);
    }
    pub fn eor<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: EorEmitter<T0, T1, T2>,
    {
        <Self as EorEmitter<T0, T1, T2>>::eor(self, op0, op1, op2);
    }
    pub fn eor_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Eor4Emitter<T0, T1, T2, T3>,
    {
        <Self as Eor4Emitter<T0, T1, T2, T3>>::eor_4(self, op0, op1, op2, op3);
    }
    pub fn eret(&mut self)
    where
        Self: EretEmitter,
    {
        <Self as EretEmitter>::eret(self);
    }
    pub fn esb(&mut self)
    where
        Self: EsbEmitter,
    {
        <Self as EsbEmitter>::esb(self);
    }
    pub fn extr<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: ExtrEmitter<T0, T1, T2, T3>,
    {
        <Self as ExtrEmitter<T0, T1, T2, T3>>::extr(self, op0, op1, op2, op3);
    }
    pub fn hlt<T0>(&mut self, op0: T0)
    where
        Self: HltEmitter<T0>,
    {
        <Self as HltEmitter<T0>>::hlt(self, op0);
    }
    pub fn hvc<T0>(&mut self, op0: T0)
    where
        Self: HvcEmitter<T0>,
    {
        <Self as HvcEmitter<T0>>::hvc(self, op0);
    }
    pub fn ic<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: IcEmitter<T0, T1>,
    {
        <Self as IcEmitter<T0, T1>>::ic(self, op0, op1);
    }
    pub fn isb<T0>(&mut self, op0: T0)
    where
        Self: IsbEmitter<T0>,
    {
        <Self as IsbEmitter<T0>>::isb(self, op0);
    }
    pub fn lsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LslEmitter<T0, T1, T2>,
    {
        <Self as LslEmitter<T0, T1, T2>>::lsl(self, op0, op1, op2);
    }
    pub fn lslv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LslvEmitter<T0, T1, T2>,
    {
        <Self as LslvEmitter<T0, T1, T2>>::lslv(self, op0, op1, op2);
    }
    pub fn lsr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LsrEmitter<T0, T1, T2>,
    {
        <Self as LsrEmitter<T0, T1, T2>>::lsr(self, op0, op1, op2);
    }
    pub fn lsrv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LsrvEmitter<T0, T1, T2>,
    {
        <Self as LsrvEmitter<T0, T1, T2>>::lsrv(self, op0, op1, op2);
    }
    pub fn madd<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: MaddEmitter<T0, T1, T2, T3>,
    {
        <Self as MaddEmitter<T0, T1, T2, T3>>::madd(self, op0, op1, op2, op3);
    }
    pub fn mneg<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: MnegEmitter<T0, T1, T2>,
    {
        <Self as MnegEmitter<T0, T1, T2>>::mneg(self, op0, op1, op2);
    }
    pub fn mov<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MovEmitter<T0, T1>,
    {
        <Self as MovEmitter<T0, T1>>::mov(self, op0, op1);
    }
    pub fn movk<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MovkEmitter<T0, T1>,
    {
        <Self as MovkEmitter<T0, T1>>::movk(self, op0, op1);
    }
    pub fn movk_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Movk3Emitter<T0, T1, T2>,
    {
        <Self as Movk3Emitter<T0, T1, T2>>::movk_3(self, op0, op1, op2);
    }
    pub fn movn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MovnEmitter<T0, T1>,
    {
        <Self as MovnEmitter<T0, T1>>::movn(self, op0, op1);
    }
    pub fn movn_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Movn3Emitter<T0, T1, T2>,
    {
        <Self as Movn3Emitter<T0, T1, T2>>::movn_3(self, op0, op1, op2);
    }
    pub fn movz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MovzEmitter<T0, T1>,
    {
        <Self as MovzEmitter<T0, T1>>::movz(self, op0, op1);
    }
    pub fn movz_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Movz3Emitter<T0, T1, T2>,
    {
        <Self as Movz3Emitter<T0, T1, T2>>::movz_3(self, op0, op1, op2);
    }
    pub fn mrs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MrsEmitter<T0, T1>,
    {
        <Self as MrsEmitter<T0, T1>>::mrs(self, op0, op1);
    }
    pub fn msr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MsrEmitter<T0, T1>,
    {
        <Self as MsrEmitter<T0, T1>>::msr(self, op0, op1);
    }
    pub fn msub<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: MsubEmitter<T0, T1, T2, T3>,
    {
        <Self as MsubEmitter<T0, T1, T2, T3>>::msub(self, op0, op1, op2, op3);
    }
    pub fn mul<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: MulEmitter<T0, T1, T2>,
    {
        <Self as MulEmitter<T0, T1, T2>>::mul(self, op0, op1, op2);
    }
    pub fn mvn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MvnEmitter<T0, T1>,
    {
        <Self as MvnEmitter<T0, T1>>::mvn(self, op0, op1);
    }
    pub fn mvn_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Mvn3Emitter<T0, T1, T2>,
    {
        <Self as Mvn3Emitter<T0, T1, T2>>::mvn_3(self, op0, op1, op2);
    }
    pub fn mvn_<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Mvn_Emitter<T0, T1>,
    {
        <Self as Mvn_Emitter<T0, T1>>::mvn_(self, op0, op1);
    }
    pub fn mvn__3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Mvn_3Emitter<T0, T1, T2>,
    {
        <Self as Mvn_3Emitter<T0, T1, T2>>::mvn__3(self, op0, op1, op2);
    }
    pub fn neg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: NegEmitter<T0, T1>,
    {
        <Self as NegEmitter<T0, T1>>::neg(self, op0, op1);
    }
    pub fn neg_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Neg3Emitter<T0, T1, T2>,
    {
        <Self as Neg3Emitter<T0, T1, T2>>::neg_3(self, op0, op1, op2);
    }
    pub fn negs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: NegsEmitter<T0, T1>,
    {
        <Self as NegsEmitter<T0, T1>>::negs(self, op0, op1);
    }
    pub fn negs_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Negs3Emitter<T0, T1, T2>,
    {
        <Self as Negs3Emitter<T0, T1, T2>>::negs_3(self, op0, op1, op2);
    }
    pub fn ngc<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: NgcEmitter<T0, T1>,
    {
        <Self as NgcEmitter<T0, T1>>::ngc(self, op0, op1);
    }
    pub fn ngcs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: NgcsEmitter<T0, T1>,
    {
        <Self as NgcsEmitter<T0, T1>>::ngcs(self, op0, op1);
    }
    pub fn orn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: OrnEmitter<T0, T1, T2>,
    {
        <Self as OrnEmitter<T0, T1, T2>>::orn(self, op0, op1, op2);
    }
    pub fn orn_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Orn4Emitter<T0, T1, T2, T3>,
    {
        <Self as Orn4Emitter<T0, T1, T2, T3>>::orn_4(self, op0, op1, op2, op3);
    }
    pub fn orr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: OrrEmitter<T0, T1, T2>,
    {
        <Self as OrrEmitter<T0, T1, T2>>::orr(self, op0, op1, op2);
    }
    pub fn orr_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Orr4Emitter<T0, T1, T2, T3>,
    {
        <Self as Orr4Emitter<T0, T1, T2, T3>>::orr_4(self, op0, op1, op2, op3);
    }
    pub fn rbit<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: RbitEmitter<T0, T1>,
    {
        <Self as RbitEmitter<T0, T1>>::rbit(self, op0, op1);
    }
    pub fn ret<T0>(&mut self, op0: T0)
    where
        Self: RetEmitter<T0>,
    {
        <Self as RetEmitter<T0>>::ret(self, op0);
    }
    pub fn rev<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: RevEmitter<T0, T1>,
    {
        <Self as RevEmitter<T0, T1>>::rev(self, op0, op1);
    }
    pub fn rev16<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Rev16Emitter<T0, T1>,
    {
        <Self as Rev16Emitter<T0, T1>>::rev16(self, op0, op1);
    }
    pub fn rev32<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Rev32Emitter<T0, T1>,
    {
        <Self as Rev32Emitter<T0, T1>>::rev32(self, op0, op1);
    }
    pub fn rev64<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Rev64Emitter<T0, T1>,
    {
        <Self as Rev64Emitter<T0, T1>>::rev64(self, op0, op1);
    }
    pub fn ror<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: RorEmitter<T0, T1, T2>,
    {
        <Self as RorEmitter<T0, T1, T2>>::ror(self, op0, op1, op2);
    }
    pub fn rorv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: RorvEmitter<T0, T1, T2>,
    {
        <Self as RorvEmitter<T0, T1, T2>>::rorv(self, op0, op1, op2);
    }
    pub fn sbc<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SbcEmitter<T0, T1, T2>,
    {
        <Self as SbcEmitter<T0, T1, T2>>::sbc(self, op0, op1, op2);
    }
    pub fn sbcs<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SbcsEmitter<T0, T1, T2>,
    {
        <Self as SbcsEmitter<T0, T1, T2>>::sbcs(self, op0, op1, op2);
    }
    pub fn sbfiz<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SbfizEmitter<T0, T1, T2, T3>,
    {
        <Self as SbfizEmitter<T0, T1, T2, T3>>::sbfiz(self, op0, op1, op2, op3);
    }
    pub fn sbfm<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SbfmEmitter<T0, T1, T2, T3>,
    {
        <Self as SbfmEmitter<T0, T1, T2, T3>>::sbfm(self, op0, op1, op2, op3);
    }
    pub fn sbfx<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SbfxEmitter<T0, T1, T2, T3>,
    {
        <Self as SbfxEmitter<T0, T1, T2, T3>>::sbfx(self, op0, op1, op2, op3);
    }
    pub fn sdiv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SdivEmitter<T0, T1, T2>,
    {
        <Self as SdivEmitter<T0, T1, T2>>::sdiv(self, op0, op1, op2);
    }
    pub fn smaddl<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SmaddlEmitter<T0, T1, T2, T3>,
    {
        <Self as SmaddlEmitter<T0, T1, T2, T3>>::smaddl(self, op0, op1, op2, op3);
    }
    pub fn smc<T0>(&mut self, op0: T0)
    where
        Self: SmcEmitter<T0>,
    {
        <Self as SmcEmitter<T0>>::smc(self, op0);
    }
    pub fn smnegl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmneglEmitter<T0, T1, T2>,
    {
        <Self as SmneglEmitter<T0, T1, T2>>::smnegl(self, op0, op1, op2);
    }
    pub fn smsubl<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SmsublEmitter<T0, T1, T2, T3>,
    {
        <Self as SmsublEmitter<T0, T1, T2, T3>>::smsubl(self, op0, op1, op2, op3);
    }
    pub fn smulh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmulhEmitter<T0, T1, T2>,
    {
        <Self as SmulhEmitter<T0, T1, T2>>::smulh(self, op0, op1, op2);
    }
    pub fn smull<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmullEmitter<T0, T1, T2>,
    {
        <Self as SmullEmitter<T0, T1, T2>>::smull(self, op0, op1, op2);
    }
    pub fn sub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SubEmitter<T0, T1, T2>,
    {
        <Self as SubEmitter<T0, T1, T2>>::sub(self, op0, op1, op2);
    }
    pub fn sub_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Sub4Emitter<T0, T1, T2, T3>,
    {
        <Self as Sub4Emitter<T0, T1, T2, T3>>::sub_4(self, op0, op1, op2, op3);
    }
    pub fn subs<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SubsEmitter<T0, T1, T2>,
    {
        <Self as SubsEmitter<T0, T1, T2>>::subs(self, op0, op1, op2);
    }
    pub fn subs_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Subs4Emitter<T0, T1, T2, T3>,
    {
        <Self as Subs4Emitter<T0, T1, T2, T3>>::subs_4(self, op0, op1, op2, op3);
    }
    pub fn svc<T0>(&mut self, op0: T0)
    where
        Self: SvcEmitter<T0>,
    {
        <Self as SvcEmitter<T0>>::svc(self, op0);
    }
    pub fn sxtb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SxtbEmitter<T0, T1>,
    {
        <Self as SxtbEmitter<T0, T1>>::sxtb(self, op0, op1);
    }
    pub fn sxth<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SxthEmitter<T0, T1>,
    {
        <Self as SxthEmitter<T0, T1>>::sxth(self, op0, op1);
    }
    pub fn sxtw<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SxtwEmitter<T0, T1>,
    {
        <Self as SxtwEmitter<T0, T1>>::sxtw(self, op0, op1);
    }
    pub fn sys<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SysEmitter<T0, T1, T2, T3>,
    {
        <Self as SysEmitter<T0, T1, T2, T3>>::sys(self, op0, op1, op2, op3);
    }
    pub fn sys_5<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Sys5Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as Sys5Emitter<T0, T1, T2, T3, T4>>::sys_5(self, op0, op1, op2, op3, op4);
    }
    pub fn tlbi<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: TlbiEmitter<T0, T1>,
    {
        <Self as TlbiEmitter<T0, T1>>::tlbi(self, op0, op1);
    }
    pub fn tst<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: TstEmitter<T0, T1>,
    {
        <Self as TstEmitter<T0, T1>>::tst(self, op0, op1);
    }
    pub fn tst_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Tst3Emitter<T0, T1, T2>,
    {
        <Self as Tst3Emitter<T0, T1, T2>>::tst_3(self, op0, op1, op2);
    }
    pub fn udiv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UdivEmitter<T0, T1, T2>,
    {
        <Self as UdivEmitter<T0, T1, T2>>::udiv(self, op0, op1, op2);
    }
    pub fn ubfiz<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: UbfizEmitter<T0, T1, T2, T3>,
    {
        <Self as UbfizEmitter<T0, T1, T2, T3>>::ubfiz(self, op0, op1, op2, op3);
    }
    pub fn ubfm<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: UbfmEmitter<T0, T1, T2, T3>,
    {
        <Self as UbfmEmitter<T0, T1, T2, T3>>::ubfm(self, op0, op1, op2, op3);
    }
    pub fn ubfx<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: UbfxEmitter<T0, T1, T2, T3>,
    {
        <Self as UbfxEmitter<T0, T1, T2, T3>>::ubfx(self, op0, op1, op2, op3);
    }
    pub fn umaddl<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: UmaddlEmitter<T0, T1, T2, T3>,
    {
        <Self as UmaddlEmitter<T0, T1, T2, T3>>::umaddl(self, op0, op1, op2, op3);
    }
    pub fn umnegl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmneglEmitter<T0, T1, T2>,
    {
        <Self as UmneglEmitter<T0, T1, T2>>::umnegl(self, op0, op1, op2);
    }
    pub fn umsubl<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: UmsublEmitter<T0, T1, T2, T3>,
    {
        <Self as UmsublEmitter<T0, T1, T2, T3>>::umsubl(self, op0, op1, op2, op3);
    }
    pub fn umull<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmullEmitter<T0, T1, T2>,
    {
        <Self as UmullEmitter<T0, T1, T2>>::umull(self, op0, op1, op2);
    }
    pub fn umulh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmulhEmitter<T0, T1, T2>,
    {
        <Self as UmulhEmitter<T0, T1, T2>>::umulh(self, op0, op1, op2);
    }
    pub fn uxtb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UxtbEmitter<T0, T1>,
    {
        <Self as UxtbEmitter<T0, T1>>::uxtb(self, op0, op1);
    }
    pub fn uxth<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UxthEmitter<T0, T1>,
    {
        <Self as UxthEmitter<T0, T1>>::uxth(self, op0, op1);
    }
    pub fn csdb(&mut self)
    where
        Self: CsdbEmitter,
    {
        <Self as CsdbEmitter>::csdb(self);
    }
    pub fn dcps1<T0>(&mut self, op0: T0)
    where
        Self: Dcps1Emitter<T0>,
    {
        <Self as Dcps1Emitter<T0>>::dcps1(self, op0);
    }
    pub fn dcps2<T0>(&mut self, op0: T0)
    where
        Self: Dcps2Emitter<T0>,
    {
        <Self as Dcps2Emitter<T0>>::dcps2(self, op0);
    }
    pub fn dcps3<T0>(&mut self, op0: T0)
    where
        Self: Dcps3Emitter<T0>,
    {
        <Self as Dcps3Emitter<T0>>::dcps3(self, op0);
    }
    pub fn pssbb(&mut self)
    where
        Self: PssbbEmitter,
    {
        <Self as PssbbEmitter>::pssbb(self);
    }
    pub fn ssbb(&mut self)
    where
        Self: SsbbEmitter,
    {
        <Self as SsbbEmitter>::ssbb(self);
    }
    pub fn udf<T0>(&mut self, op0: T0)
    where
        Self: UdfEmitter<T0>,
    {
        <Self as UdfEmitter<T0>>::udf(self, op0);
    }

    pub fn b<T>(&mut self, op0: T)
    where
        Self: BEmitter<T>,
    {
        <Self as BEmitter<T>>::b(self, op0);
    }

    pub fn b_eq<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_eq(self, op0);
    }
    pub fn b_ne<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_ne(self, op0);
    }
    pub fn b_cs<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_cs(self, op0);
    }
    pub fn b_hs<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_hs(self, op0);
    }
    pub fn b_cc<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_cc(self, op0);
    }
    pub fn b_lo<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_lo(self, op0);
    }
    pub fn b_mi<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_mi(self, op0);
    }
    pub fn b_pl<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_pl(self, op0);
    }
    pub fn b_vs<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_vs(self, op0);
    }
    pub fn b_vc<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_vc(self, op0);
    }
    pub fn b_hi<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_hi(self, op0);
    }
    pub fn b_ls<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_ls(self, op0);
    }
    pub fn b_ge<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_ge(self, op0);
    }
    pub fn b_lt<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_lt(self, op0);
    }
    pub fn b_gt<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_gt(self, op0);
    }
    pub fn b_le<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_le(self, op0);
    }
    pub fn b_al<T0>(&mut self, op0: T0)
    where
        Self: BEmitter<T0>,
    {
        <Self as BEmitter<T0>>::b_al(self, op0);
    }
    pub fn bl<T0>(&mut self, op0: T0)
    where
        Self: BlEmitter<T0>,
    {
        <Self as BlEmitter<T0>>::bl(self, op0);
    }
    pub fn blr<T0>(&mut self, op0: T0)
    where
        Self: BlrEmitter<T0>,
    {
        <Self as BlrEmitter<T0>>::blr(self, op0);
    }
    pub fn br<T0>(&mut self, op0: T0)
    where
        Self: BrEmitter<T0>,
    {
        <Self as BrEmitter<T0>>::br(self, op0);
    }
    pub fn cbz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CbzEmitter<T0, T1>,
    {
        <Self as CbzEmitter<T0, T1>>::cbz(self, op0, op1);
    }
    pub fn cbnz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CbnzEmitter<T0, T1>,
    {
        <Self as CbnzEmitter<T0, T1>>::cbnz(self, op0, op1);
    }
    pub fn tbnz<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: TbnzEmitter<T0, T1, T2>,
    {
        <Self as TbnzEmitter<T0, T1, T2>>::tbnz(self, op0, op1, op2);
    }
    pub fn tbz<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: TbzEmitter<T0, T1, T2>,
    {
        <Self as TbzEmitter<T0, T1, T2>>::tbz(self, op0, op1, op2);
    }
    pub fn cas<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasEmitter<T0, T1, T2>,
    {
        <Self as CasEmitter<T0, T1, T2>>::cas(self, op0, op1, op2);
    }
    pub fn casa<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasaEmitter<T0, T1, T2>,
    {
        <Self as CasaEmitter<T0, T1, T2>>::casa(self, op0, op1, op2);
    }
    pub fn casab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasabEmitter<T0, T1, T2>,
    {
        <Self as CasabEmitter<T0, T1, T2>>::casab(self, op0, op1, op2);
    }
    pub fn casah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasahEmitter<T0, T1, T2>,
    {
        <Self as CasahEmitter<T0, T1, T2>>::casah(self, op0, op1, op2);
    }
    pub fn casal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasalEmitter<T0, T1, T2>,
    {
        <Self as CasalEmitter<T0, T1, T2>>::casal(self, op0, op1, op2);
    }
    pub fn casalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasalbEmitter<T0, T1, T2>,
    {
        <Self as CasalbEmitter<T0, T1, T2>>::casalb(self, op0, op1, op2);
    }
    pub fn casalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasalhEmitter<T0, T1, T2>,
    {
        <Self as CasalhEmitter<T0, T1, T2>>::casalh(self, op0, op1, op2);
    }
    pub fn casb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CasbEmitter<T0, T1, T2>,
    {
        <Self as CasbEmitter<T0, T1, T2>>::casb(self, op0, op1, op2);
    }
    pub fn cash<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CashEmitter<T0, T1, T2>,
    {
        <Self as CashEmitter<T0, T1, T2>>::cash(self, op0, op1, op2);
    }
    pub fn casl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CaslEmitter<T0, T1, T2>,
    {
        <Self as CaslEmitter<T0, T1, T2>>::casl(self, op0, op1, op2);
    }
    pub fn caslb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CaslbEmitter<T0, T1, T2>,
    {
        <Self as CaslbEmitter<T0, T1, T2>>::caslb(self, op0, op1, op2);
    }
    pub fn caslh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CaslhEmitter<T0, T1, T2>,
    {
        <Self as CaslhEmitter<T0, T1, T2>>::caslh(self, op0, op1, op2);
    }
    pub fn casp<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: CaspEmitter<T0, T1, T2, T3, T4>,
    {
        <Self as CaspEmitter<T0, T1, T2, T3, T4>>::casp(self, op0, op1, op2, op3, op4);
    }
    pub fn caspa<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: CaspaEmitter<T0, T1, T2, T3, T4>,
    {
        <Self as CaspaEmitter<T0, T1, T2, T3, T4>>::caspa(self, op0, op1, op2, op3, op4);
    }
    pub fn caspal<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: CaspalEmitter<T0, T1, T2, T3, T4>,
    {
        <Self as CaspalEmitter<T0, T1, T2, T3, T4>>::caspal(self, op0, op1, op2, op3, op4);
    }
    pub fn caspl<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: CasplEmitter<T0, T1, T2, T3, T4>,
    {
        <Self as CasplEmitter<T0, T1, T2, T3, T4>>::caspl(self, op0, op1, op2, op3, op4);
    }
    pub fn ldadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddEmitter<T0, T1, T2>,
    {
        <Self as LdaddEmitter<T0, T1, T2>>::ldadd(self, op0, op1, op2);
    }
    pub fn ldadda<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddaEmitter<T0, T1, T2>,
    {
        <Self as LdaddaEmitter<T0, T1, T2>>::ldadda(self, op0, op1, op2);
    }
    pub fn ldaddab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddabEmitter<T0, T1, T2>,
    {
        <Self as LdaddabEmitter<T0, T1, T2>>::ldaddab(self, op0, op1, op2);
    }
    pub fn ldaddah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddahEmitter<T0, T1, T2>,
    {
        <Self as LdaddahEmitter<T0, T1, T2>>::ldaddah(self, op0, op1, op2);
    }
    pub fn ldaddal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddalEmitter<T0, T1, T2>,
    {
        <Self as LdaddalEmitter<T0, T1, T2>>::ldaddal(self, op0, op1, op2);
    }
    pub fn ldaddalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddalbEmitter<T0, T1, T2>,
    {
        <Self as LdaddalbEmitter<T0, T1, T2>>::ldaddalb(self, op0, op1, op2);
    }
    pub fn ldaddalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddalhEmitter<T0, T1, T2>,
    {
        <Self as LdaddalhEmitter<T0, T1, T2>>::ldaddalh(self, op0, op1, op2);
    }
    pub fn ldaddb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddbEmitter<T0, T1, T2>,
    {
        <Self as LdaddbEmitter<T0, T1, T2>>::ldaddb(self, op0, op1, op2);
    }
    pub fn ldaddh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddhEmitter<T0, T1, T2>,
    {
        <Self as LdaddhEmitter<T0, T1, T2>>::ldaddh(self, op0, op1, op2);
    }
    pub fn ldaddl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddlEmitter<T0, T1, T2>,
    {
        <Self as LdaddlEmitter<T0, T1, T2>>::ldaddl(self, op0, op1, op2);
    }
    pub fn ldaddlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddlbEmitter<T0, T1, T2>,
    {
        <Self as LdaddlbEmitter<T0, T1, T2>>::ldaddlb(self, op0, op1, op2);
    }
    pub fn ldaddlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaddlhEmitter<T0, T1, T2>,
    {
        <Self as LdaddlhEmitter<T0, T1, T2>>::ldaddlh(self, op0, op1, op2);
    }
    pub fn ldar<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdarEmitter<T0, T1>,
    {
        <Self as LdarEmitter<T0, T1>>::ldar(self, op0, op1);
    }
    pub fn ldarb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdarbEmitter<T0, T1>,
    {
        <Self as LdarbEmitter<T0, T1>>::ldarb(self, op0, op1);
    }
    pub fn ldarh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdarhEmitter<T0, T1>,
    {
        <Self as LdarhEmitter<T0, T1>>::ldarh(self, op0, op1);
    }
    pub fn ldaxr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdaxrEmitter<T0, T1>,
    {
        <Self as LdaxrEmitter<T0, T1>>::ldaxr(self, op0, op1);
    }
    pub fn ldaxrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdaxrbEmitter<T0, T1>,
    {
        <Self as LdaxrbEmitter<T0, T1>>::ldaxrb(self, op0, op1);
    }
    pub fn ldaxrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdaxrhEmitter<T0, T1>,
    {
        <Self as LdaxrhEmitter<T0, T1>>::ldaxrh(self, op0, op1);
    }
    pub fn ldclr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrEmitter<T0, T1, T2>,
    {
        <Self as LdclrEmitter<T0, T1, T2>>::ldclr(self, op0, op1, op2);
    }
    pub fn ldclra<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclraEmitter<T0, T1, T2>,
    {
        <Self as LdclraEmitter<T0, T1, T2>>::ldclra(self, op0, op1, op2);
    }
    pub fn ldclrab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrabEmitter<T0, T1, T2>,
    {
        <Self as LdclrabEmitter<T0, T1, T2>>::ldclrab(self, op0, op1, op2);
    }
    pub fn ldclrah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrahEmitter<T0, T1, T2>,
    {
        <Self as LdclrahEmitter<T0, T1, T2>>::ldclrah(self, op0, op1, op2);
    }
    pub fn ldclral<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclralEmitter<T0, T1, T2>,
    {
        <Self as LdclralEmitter<T0, T1, T2>>::ldclral(self, op0, op1, op2);
    }
    pub fn ldclralb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclralbEmitter<T0, T1, T2>,
    {
        <Self as LdclralbEmitter<T0, T1, T2>>::ldclralb(self, op0, op1, op2);
    }
    pub fn ldclralh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclralhEmitter<T0, T1, T2>,
    {
        <Self as LdclralhEmitter<T0, T1, T2>>::ldclralh(self, op0, op1, op2);
    }
    pub fn ldclrb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrbEmitter<T0, T1, T2>,
    {
        <Self as LdclrbEmitter<T0, T1, T2>>::ldclrb(self, op0, op1, op2);
    }
    pub fn ldclrh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrhEmitter<T0, T1, T2>,
    {
        <Self as LdclrhEmitter<T0, T1, T2>>::ldclrh(self, op0, op1, op2);
    }
    pub fn ldclrl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrlEmitter<T0, T1, T2>,
    {
        <Self as LdclrlEmitter<T0, T1, T2>>::ldclrl(self, op0, op1, op2);
    }
    pub fn ldclrlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrlbEmitter<T0, T1, T2>,
    {
        <Self as LdclrlbEmitter<T0, T1, T2>>::ldclrlb(self, op0, op1, op2);
    }
    pub fn ldclrlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdclrlhEmitter<T0, T1, T2>,
    {
        <Self as LdclrlhEmitter<T0, T1, T2>>::ldclrlh(self, op0, op1, op2);
    }
    pub fn ldeor<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorEmitter<T0, T1, T2>,
    {
        <Self as LdeorEmitter<T0, T1, T2>>::ldeor(self, op0, op1, op2);
    }
    pub fn ldeora<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeoraEmitter<T0, T1, T2>,
    {
        <Self as LdeoraEmitter<T0, T1, T2>>::ldeora(self, op0, op1, op2);
    }
    pub fn ldeorab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorabEmitter<T0, T1, T2>,
    {
        <Self as LdeorabEmitter<T0, T1, T2>>::ldeorab(self, op0, op1, op2);
    }
    pub fn ldeorah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorahEmitter<T0, T1, T2>,
    {
        <Self as LdeorahEmitter<T0, T1, T2>>::ldeorah(self, op0, op1, op2);
    }
    pub fn ldeoral<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeoralEmitter<T0, T1, T2>,
    {
        <Self as LdeoralEmitter<T0, T1, T2>>::ldeoral(self, op0, op1, op2);
    }
    pub fn ldeoralb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeoralbEmitter<T0, T1, T2>,
    {
        <Self as LdeoralbEmitter<T0, T1, T2>>::ldeoralb(self, op0, op1, op2);
    }
    pub fn ldeoralh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeoralhEmitter<T0, T1, T2>,
    {
        <Self as LdeoralhEmitter<T0, T1, T2>>::ldeoralh(self, op0, op1, op2);
    }
    pub fn ldeorb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorbEmitter<T0, T1, T2>,
    {
        <Self as LdeorbEmitter<T0, T1, T2>>::ldeorb(self, op0, op1, op2);
    }
    pub fn ldeorh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorhEmitter<T0, T1, T2>,
    {
        <Self as LdeorhEmitter<T0, T1, T2>>::ldeorh(self, op0, op1, op2);
    }
    pub fn ldeorl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorlEmitter<T0, T1, T2>,
    {
        <Self as LdeorlEmitter<T0, T1, T2>>::ldeorl(self, op0, op1, op2);
    }
    pub fn ldeorlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorlbEmitter<T0, T1, T2>,
    {
        <Self as LdeorlbEmitter<T0, T1, T2>>::ldeorlb(self, op0, op1, op2);
    }
    pub fn ldeorlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdeorlhEmitter<T0, T1, T2>,
    {
        <Self as LdeorlhEmitter<T0, T1, T2>>::ldeorlh(self, op0, op1, op2);
    }
    pub fn ldlar<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdlarEmitter<T0, T1>,
    {
        <Self as LdlarEmitter<T0, T1>>::ldlar(self, op0, op1);
    }
    pub fn ldlarb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdlarbEmitter<T0, T1>,
    {
        <Self as LdlarbEmitter<T0, T1>>::ldlarb(self, op0, op1);
    }
    pub fn ldlarh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdlarhEmitter<T0, T1>,
    {
        <Self as LdlarhEmitter<T0, T1>>::ldlarh(self, op0, op1);
    }
    pub fn ldnp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdnpEmitter<T0, T1, T2>,
    {
        <Self as LdnpEmitter<T0, T1, T2>>::ldnp(self, op0, op1, op2);
    }
    pub fn ldp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdpEmitter<T0, T1, T2>,
    {
        <Self as LdpEmitter<T0, T1, T2>>::ldp(self, op0, op1, op2);
    }
    pub fn ldpsw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdpswEmitter<T0, T1, T2>,
    {
        <Self as LdpswEmitter<T0, T1, T2>>::ldpsw(self, op0, op1, op2);
    }
    pub fn ldr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrEmitter<T0, T1>,
    {
        <Self as LdrEmitter<T0, T1>>::ldr(self, op0, op1);
    }
    pub fn ldrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrbEmitter<T0, T1>,
    {
        <Self as LdrbEmitter<T0, T1>>::ldrb(self, op0, op1);
    }
    pub fn ldrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrhEmitter<T0, T1>,
    {
        <Self as LdrhEmitter<T0, T1>>::ldrh(self, op0, op1);
    }
    pub fn ldrsb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrsbEmitter<T0, T1>,
    {
        <Self as LdrsbEmitter<T0, T1>>::ldrsb(self, op0, op1);
    }
    pub fn ldrsh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrshEmitter<T0, T1>,
    {
        <Self as LdrshEmitter<T0, T1>>::ldrsh(self, op0, op1);
    }
    pub fn ldrsw<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrswEmitter<T0, T1>,
    {
        <Self as LdrswEmitter<T0, T1>>::ldrsw(self, op0, op1);
    }
    pub fn ldset<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetEmitter<T0, T1, T2>,
    {
        <Self as LdsetEmitter<T0, T1, T2>>::ldset(self, op0, op1, op2);
    }
    pub fn ldseta<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetaEmitter<T0, T1, T2>,
    {
        <Self as LdsetaEmitter<T0, T1, T2>>::ldseta(self, op0, op1, op2);
    }
    pub fn ldsetab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetabEmitter<T0, T1, T2>,
    {
        <Self as LdsetabEmitter<T0, T1, T2>>::ldsetab(self, op0, op1, op2);
    }
    pub fn ldsetah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetahEmitter<T0, T1, T2>,
    {
        <Self as LdsetahEmitter<T0, T1, T2>>::ldsetah(self, op0, op1, op2);
    }
    pub fn ldsetal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetalEmitter<T0, T1, T2>,
    {
        <Self as LdsetalEmitter<T0, T1, T2>>::ldsetal(self, op0, op1, op2);
    }
    pub fn ldsetalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetalbEmitter<T0, T1, T2>,
    {
        <Self as LdsetalbEmitter<T0, T1, T2>>::ldsetalb(self, op0, op1, op2);
    }
    pub fn ldsetalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetalhEmitter<T0, T1, T2>,
    {
        <Self as LdsetalhEmitter<T0, T1, T2>>::ldsetalh(self, op0, op1, op2);
    }
    pub fn ldsetb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetbEmitter<T0, T1, T2>,
    {
        <Self as LdsetbEmitter<T0, T1, T2>>::ldsetb(self, op0, op1, op2);
    }
    pub fn ldseth<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsethEmitter<T0, T1, T2>,
    {
        <Self as LdsethEmitter<T0, T1, T2>>::ldseth(self, op0, op1, op2);
    }
    pub fn ldsetl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetlEmitter<T0, T1, T2>,
    {
        <Self as LdsetlEmitter<T0, T1, T2>>::ldsetl(self, op0, op1, op2);
    }
    pub fn ldsetlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetlbEmitter<T0, T1, T2>,
    {
        <Self as LdsetlbEmitter<T0, T1, T2>>::ldsetlb(self, op0, op1, op2);
    }
    pub fn ldsetlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsetlhEmitter<T0, T1, T2>,
    {
        <Self as LdsetlhEmitter<T0, T1, T2>>::ldsetlh(self, op0, op1, op2);
    }
    pub fn ldsmax<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxEmitter<T0, T1, T2>>::ldsmax(self, op0, op1, op2);
    }
    pub fn ldsmaxa<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxaEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxaEmitter<T0, T1, T2>>::ldsmaxa(self, op0, op1, op2);
    }
    pub fn ldsmaxab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxabEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxabEmitter<T0, T1, T2>>::ldsmaxab(self, op0, op1, op2);
    }
    pub fn ldsmaxah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxahEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxahEmitter<T0, T1, T2>>::ldsmaxah(self, op0, op1, op2);
    }
    pub fn ldsmaxal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxalEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxalEmitter<T0, T1, T2>>::ldsmaxal(self, op0, op1, op2);
    }
    pub fn ldsmaxalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxalbEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxalbEmitter<T0, T1, T2>>::ldsmaxalb(self, op0, op1, op2);
    }
    pub fn ldsmaxalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxalhEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxalhEmitter<T0, T1, T2>>::ldsmaxalh(self, op0, op1, op2);
    }
    pub fn ldsmaxb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxbEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxbEmitter<T0, T1, T2>>::ldsmaxb(self, op0, op1, op2);
    }
    pub fn ldsmaxh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxhEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxhEmitter<T0, T1, T2>>::ldsmaxh(self, op0, op1, op2);
    }
    pub fn ldsmaxl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxlEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxlEmitter<T0, T1, T2>>::ldsmaxl(self, op0, op1, op2);
    }
    pub fn ldsmaxlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxlbEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxlbEmitter<T0, T1, T2>>::ldsmaxlb(self, op0, op1, op2);
    }
    pub fn ldsmaxlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsmaxlhEmitter<T0, T1, T2>,
    {
        <Self as LdsmaxlhEmitter<T0, T1, T2>>::ldsmaxlh(self, op0, op1, op2);
    }
    pub fn ldsmin<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminEmitter<T0, T1, T2>,
    {
        <Self as LdsminEmitter<T0, T1, T2>>::ldsmin(self, op0, op1, op2);
    }
    pub fn ldsmina<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminaEmitter<T0, T1, T2>,
    {
        <Self as LdsminaEmitter<T0, T1, T2>>::ldsmina(self, op0, op1, op2);
    }
    pub fn ldsminab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminabEmitter<T0, T1, T2>,
    {
        <Self as LdsminabEmitter<T0, T1, T2>>::ldsminab(self, op0, op1, op2);
    }
    pub fn ldsminah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminahEmitter<T0, T1, T2>,
    {
        <Self as LdsminahEmitter<T0, T1, T2>>::ldsminah(self, op0, op1, op2);
    }
    pub fn ldsminal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminalEmitter<T0, T1, T2>,
    {
        <Self as LdsminalEmitter<T0, T1, T2>>::ldsminal(self, op0, op1, op2);
    }
    pub fn ldsminalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminalbEmitter<T0, T1, T2>,
    {
        <Self as LdsminalbEmitter<T0, T1, T2>>::ldsminalb(self, op0, op1, op2);
    }
    pub fn ldsminalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminalhEmitter<T0, T1, T2>,
    {
        <Self as LdsminalhEmitter<T0, T1, T2>>::ldsminalh(self, op0, op1, op2);
    }
    pub fn ldsminb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminbEmitter<T0, T1, T2>,
    {
        <Self as LdsminbEmitter<T0, T1, T2>>::ldsminb(self, op0, op1, op2);
    }
    pub fn ldsminh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminhEmitter<T0, T1, T2>,
    {
        <Self as LdsminhEmitter<T0, T1, T2>>::ldsminh(self, op0, op1, op2);
    }
    pub fn ldsminl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminlEmitter<T0, T1, T2>,
    {
        <Self as LdsminlEmitter<T0, T1, T2>>::ldsminl(self, op0, op1, op2);
    }
    pub fn ldsminlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminlbEmitter<T0, T1, T2>,
    {
        <Self as LdsminlbEmitter<T0, T1, T2>>::ldsminlb(self, op0, op1, op2);
    }
    pub fn ldsminlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdsminlhEmitter<T0, T1, T2>,
    {
        <Self as LdsminlhEmitter<T0, T1, T2>>::ldsminlh(self, op0, op1, op2);
    }
    pub fn ldtr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrEmitter<T0, T1>,
    {
        <Self as LdtrEmitter<T0, T1>>::ldtr(self, op0, op1);
    }
    pub fn ldtrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrbEmitter<T0, T1>,
    {
        <Self as LdtrbEmitter<T0, T1>>::ldtrb(self, op0, op1);
    }
    pub fn ldtrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrhEmitter<T0, T1>,
    {
        <Self as LdtrhEmitter<T0, T1>>::ldtrh(self, op0, op1);
    }
    pub fn ldtrsb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrsbEmitter<T0, T1>,
    {
        <Self as LdtrsbEmitter<T0, T1>>::ldtrsb(self, op0, op1);
    }
    pub fn ldtrsh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrshEmitter<T0, T1>,
    {
        <Self as LdtrshEmitter<T0, T1>>::ldtrsh(self, op0, op1);
    }
    pub fn ldtrsw<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdtrswEmitter<T0, T1>,
    {
        <Self as LdtrswEmitter<T0, T1>>::ldtrsw(self, op0, op1);
    }
    pub fn ldumax<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxEmitter<T0, T1, T2>,
    {
        <Self as LdumaxEmitter<T0, T1, T2>>::ldumax(self, op0, op1, op2);
    }
    pub fn ldumaxa<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxaEmitter<T0, T1, T2>,
    {
        <Self as LdumaxaEmitter<T0, T1, T2>>::ldumaxa(self, op0, op1, op2);
    }
    pub fn ldumaxab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxabEmitter<T0, T1, T2>,
    {
        <Self as LdumaxabEmitter<T0, T1, T2>>::ldumaxab(self, op0, op1, op2);
    }
    pub fn ldumaxah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxahEmitter<T0, T1, T2>,
    {
        <Self as LdumaxahEmitter<T0, T1, T2>>::ldumaxah(self, op0, op1, op2);
    }
    pub fn ldumaxal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxalEmitter<T0, T1, T2>,
    {
        <Self as LdumaxalEmitter<T0, T1, T2>>::ldumaxal(self, op0, op1, op2);
    }
    pub fn ldumaxalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxalbEmitter<T0, T1, T2>,
    {
        <Self as LdumaxalbEmitter<T0, T1, T2>>::ldumaxalb(self, op0, op1, op2);
    }
    pub fn ldumaxalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxalhEmitter<T0, T1, T2>,
    {
        <Self as LdumaxalhEmitter<T0, T1, T2>>::ldumaxalh(self, op0, op1, op2);
    }
    pub fn ldumaxb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxbEmitter<T0, T1, T2>,
    {
        <Self as LdumaxbEmitter<T0, T1, T2>>::ldumaxb(self, op0, op1, op2);
    }
    pub fn ldumaxh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxhEmitter<T0, T1, T2>,
    {
        <Self as LdumaxhEmitter<T0, T1, T2>>::ldumaxh(self, op0, op1, op2);
    }
    pub fn ldumaxl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxlEmitter<T0, T1, T2>,
    {
        <Self as LdumaxlEmitter<T0, T1, T2>>::ldumaxl(self, op0, op1, op2);
    }
    pub fn ldumaxlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxlbEmitter<T0, T1, T2>,
    {
        <Self as LdumaxlbEmitter<T0, T1, T2>>::ldumaxlb(self, op0, op1, op2);
    }
    pub fn ldumaxlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdumaxlhEmitter<T0, T1, T2>,
    {
        <Self as LdumaxlhEmitter<T0, T1, T2>>::ldumaxlh(self, op0, op1, op2);
    }
    pub fn ldumin<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminEmitter<T0, T1, T2>,
    {
        <Self as LduminEmitter<T0, T1, T2>>::ldumin(self, op0, op1, op2);
    }
    pub fn ldumina<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminaEmitter<T0, T1, T2>,
    {
        <Self as LduminaEmitter<T0, T1, T2>>::ldumina(self, op0, op1, op2);
    }
    pub fn lduminab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminabEmitter<T0, T1, T2>,
    {
        <Self as LduminabEmitter<T0, T1, T2>>::lduminab(self, op0, op1, op2);
    }
    pub fn lduminah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminahEmitter<T0, T1, T2>,
    {
        <Self as LduminahEmitter<T0, T1, T2>>::lduminah(self, op0, op1, op2);
    }
    pub fn lduminal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminalEmitter<T0, T1, T2>,
    {
        <Self as LduminalEmitter<T0, T1, T2>>::lduminal(self, op0, op1, op2);
    }
    pub fn lduminalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminalbEmitter<T0, T1, T2>,
    {
        <Self as LduminalbEmitter<T0, T1, T2>>::lduminalb(self, op0, op1, op2);
    }
    pub fn lduminalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminalhEmitter<T0, T1, T2>,
    {
        <Self as LduminalhEmitter<T0, T1, T2>>::lduminalh(self, op0, op1, op2);
    }
    pub fn lduminb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminbEmitter<T0, T1, T2>,
    {
        <Self as LduminbEmitter<T0, T1, T2>>::lduminb(self, op0, op1, op2);
    }
    pub fn lduminh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminhEmitter<T0, T1, T2>,
    {
        <Self as LduminhEmitter<T0, T1, T2>>::lduminh(self, op0, op1, op2);
    }
    pub fn lduminl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminlEmitter<T0, T1, T2>,
    {
        <Self as LduminlEmitter<T0, T1, T2>>::lduminl(self, op0, op1, op2);
    }
    pub fn lduminlb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminlbEmitter<T0, T1, T2>,
    {
        <Self as LduminlbEmitter<T0, T1, T2>>::lduminlb(self, op0, op1, op2);
    }
    pub fn lduminlh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LduminlhEmitter<T0, T1, T2>,
    {
        <Self as LduminlhEmitter<T0, T1, T2>>::lduminlh(self, op0, op1, op2);
    }
    pub fn ldur<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdurEmitter<T0, T1>,
    {
        <Self as LdurEmitter<T0, T1>>::ldur(self, op0, op1);
    }
    pub fn ldurb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdurbEmitter<T0, T1>,
    {
        <Self as LdurbEmitter<T0, T1>>::ldurb(self, op0, op1);
    }
    pub fn ldurh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdurhEmitter<T0, T1>,
    {
        <Self as LdurhEmitter<T0, T1>>::ldurh(self, op0, op1);
    }
    pub fn ldursb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdursbEmitter<T0, T1>,
    {
        <Self as LdursbEmitter<T0, T1>>::ldursb(self, op0, op1);
    }
    pub fn ldursh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdurshEmitter<T0, T1>,
    {
        <Self as LdurshEmitter<T0, T1>>::ldursh(self, op0, op1);
    }
    pub fn ldursw<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdurswEmitter<T0, T1>,
    {
        <Self as LdurswEmitter<T0, T1>>::ldursw(self, op0, op1);
    }
    pub fn ldxp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdxpEmitter<T0, T1, T2>,
    {
        <Self as LdxpEmitter<T0, T1, T2>>::ldxp(self, op0, op1, op2);
    }
    pub fn ldaxp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: LdaxpEmitter<T0, T1, T2>,
    {
        <Self as LdaxpEmitter<T0, T1, T2>>::ldaxp(self, op0, op1, op2);
    }
    pub fn ldxr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdxrEmitter<T0, T1>,
    {
        <Self as LdxrEmitter<T0, T1>>::ldxr(self, op0, op1);
    }
    pub fn ldxrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdxrbEmitter<T0, T1>,
    {
        <Self as LdxrbEmitter<T0, T1>>::ldxrb(self, op0, op1);
    }
    pub fn ldxrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdxrhEmitter<T0, T1>,
    {
        <Self as LdxrhEmitter<T0, T1>>::ldxrh(self, op0, op1);
    }
    pub fn prfm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: PrfmEmitter<T0, T1>,
    {
        <Self as PrfmEmitter<T0, T1>>::prfm(self, op0, op1);
    }
    pub fn stadd<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddEmitter<T0, T1>,
    {
        <Self as StaddEmitter<T0, T1>>::stadd(self, op0, op1);
    }
    pub fn staddb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddbEmitter<T0, T1>,
    {
        <Self as StaddbEmitter<T0, T1>>::staddb(self, op0, op1);
    }
    pub fn staddh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddhEmitter<T0, T1>,
    {
        <Self as StaddhEmitter<T0, T1>>::staddh(self, op0, op1);
    }
    pub fn staddl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddlEmitter<T0, T1>,
    {
        <Self as StaddlEmitter<T0, T1>>::staddl(self, op0, op1);
    }
    pub fn staddlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddlbEmitter<T0, T1>,
    {
        <Self as StaddlbEmitter<T0, T1>>::staddlb(self, op0, op1);
    }
    pub fn staddlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StaddlhEmitter<T0, T1>,
    {
        <Self as StaddlhEmitter<T0, T1>>::staddlh(self, op0, op1);
    }
    pub fn stclr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrEmitter<T0, T1>,
    {
        <Self as StclrEmitter<T0, T1>>::stclr(self, op0, op1);
    }
    pub fn stclrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrbEmitter<T0, T1>,
    {
        <Self as StclrbEmitter<T0, T1>>::stclrb(self, op0, op1);
    }
    pub fn stclrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrhEmitter<T0, T1>,
    {
        <Self as StclrhEmitter<T0, T1>>::stclrh(self, op0, op1);
    }
    pub fn stclrl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrlEmitter<T0, T1>,
    {
        <Self as StclrlEmitter<T0, T1>>::stclrl(self, op0, op1);
    }
    pub fn stclrlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrlbEmitter<T0, T1>,
    {
        <Self as StclrlbEmitter<T0, T1>>::stclrlb(self, op0, op1);
    }
    pub fn stclrlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StclrlhEmitter<T0, T1>,
    {
        <Self as StclrlhEmitter<T0, T1>>::stclrlh(self, op0, op1);
    }
    pub fn steor<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorEmitter<T0, T1>,
    {
        <Self as SteorEmitter<T0, T1>>::steor(self, op0, op1);
    }
    pub fn steorb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorbEmitter<T0, T1>,
    {
        <Self as SteorbEmitter<T0, T1>>::steorb(self, op0, op1);
    }
    pub fn steorh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorhEmitter<T0, T1>,
    {
        <Self as SteorhEmitter<T0, T1>>::steorh(self, op0, op1);
    }
    pub fn steorl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorlEmitter<T0, T1>,
    {
        <Self as SteorlEmitter<T0, T1>>::steorl(self, op0, op1);
    }
    pub fn steorlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorlbEmitter<T0, T1>,
    {
        <Self as SteorlbEmitter<T0, T1>>::steorlb(self, op0, op1);
    }
    pub fn steorlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SteorlhEmitter<T0, T1>,
    {
        <Self as SteorlhEmitter<T0, T1>>::steorlh(self, op0, op1);
    }
    pub fn stllr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StllrEmitter<T0, T1>,
    {
        <Self as StllrEmitter<T0, T1>>::stllr(self, op0, op1);
    }
    pub fn stllrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StllrbEmitter<T0, T1>,
    {
        <Self as StllrbEmitter<T0, T1>>::stllrb(self, op0, op1);
    }
    pub fn stllrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StllrhEmitter<T0, T1>,
    {
        <Self as StllrhEmitter<T0, T1>>::stllrh(self, op0, op1);
    }
    pub fn stlr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StlrEmitter<T0, T1>,
    {
        <Self as StlrEmitter<T0, T1>>::stlr(self, op0, op1);
    }
    pub fn stlrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StlrbEmitter<T0, T1>,
    {
        <Self as StlrbEmitter<T0, T1>>::stlrb(self, op0, op1);
    }
    pub fn stlrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StlrhEmitter<T0, T1>,
    {
        <Self as StlrhEmitter<T0, T1>>::stlrh(self, op0, op1);
    }
    pub fn stlxr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StlxrEmitter<T0, T1, T2>,
    {
        <Self as StlxrEmitter<T0, T1, T2>>::stlxr(self, op0, op1, op2);
    }
    pub fn stlxrb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StlxrbEmitter<T0, T1, T2>,
    {
        <Self as StlxrbEmitter<T0, T1, T2>>::stlxrb(self, op0, op1, op2);
    }
    pub fn stlxrh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StlxrhEmitter<T0, T1, T2>,
    {
        <Self as StlxrhEmitter<T0, T1, T2>>::stlxrh(self, op0, op1, op2);
    }
    pub fn stnp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StnpEmitter<T0, T1, T2>,
    {
        <Self as StnpEmitter<T0, T1, T2>>::stnp(self, op0, op1, op2);
    }
    pub fn stp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StpEmitter<T0, T1, T2>,
    {
        <Self as StpEmitter<T0, T1, T2>>::stp(self, op0, op1, op2);
    }
    pub fn str<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StrEmitter<T0, T1>,
    {
        <Self as StrEmitter<T0, T1>>::str(self, op0, op1);
    }
    pub fn strb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StrbEmitter<T0, T1>,
    {
        <Self as StrbEmitter<T0, T1>>::strb(self, op0, op1);
    }
    pub fn strh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StrhEmitter<T0, T1>,
    {
        <Self as StrhEmitter<T0, T1>>::strh(self, op0, op1);
    }
    pub fn stset<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsetEmitter<T0, T1>,
    {
        <Self as StsetEmitter<T0, T1>>::stset(self, op0, op1);
    }
    pub fn stsetb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsetbEmitter<T0, T1>,
    {
        <Self as StsetbEmitter<T0, T1>>::stsetb(self, op0, op1);
    }
    pub fn stseth<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsethEmitter<T0, T1>,
    {
        <Self as StsethEmitter<T0, T1>>::stseth(self, op0, op1);
    }
    pub fn stsetl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsetlEmitter<T0, T1>,
    {
        <Self as StsetlEmitter<T0, T1>>::stsetl(self, op0, op1);
    }
    pub fn stsetlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsetlbEmitter<T0, T1>,
    {
        <Self as StsetlbEmitter<T0, T1>>::stsetlb(self, op0, op1);
    }
    pub fn stsetlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsetlhEmitter<T0, T1>,
    {
        <Self as StsetlhEmitter<T0, T1>>::stsetlh(self, op0, op1);
    }
    pub fn stsmax<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxEmitter<T0, T1>,
    {
        <Self as StsmaxEmitter<T0, T1>>::stsmax(self, op0, op1);
    }
    pub fn stsmaxb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxbEmitter<T0, T1>,
    {
        <Self as StsmaxbEmitter<T0, T1>>::stsmaxb(self, op0, op1);
    }
    pub fn stsmaxh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxhEmitter<T0, T1>,
    {
        <Self as StsmaxhEmitter<T0, T1>>::stsmaxh(self, op0, op1);
    }
    pub fn stsmaxl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxlEmitter<T0, T1>,
    {
        <Self as StsmaxlEmitter<T0, T1>>::stsmaxl(self, op0, op1);
    }
    pub fn stsmaxlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxlbEmitter<T0, T1>,
    {
        <Self as StsmaxlbEmitter<T0, T1>>::stsmaxlb(self, op0, op1);
    }
    pub fn stsmaxlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsmaxlhEmitter<T0, T1>,
    {
        <Self as StsmaxlhEmitter<T0, T1>>::stsmaxlh(self, op0, op1);
    }
    pub fn stsmin<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminEmitter<T0, T1>,
    {
        <Self as StsminEmitter<T0, T1>>::stsmin(self, op0, op1);
    }
    pub fn stsminb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminbEmitter<T0, T1>,
    {
        <Self as StsminbEmitter<T0, T1>>::stsminb(self, op0, op1);
    }
    pub fn stsminh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminhEmitter<T0, T1>,
    {
        <Self as StsminhEmitter<T0, T1>>::stsminh(self, op0, op1);
    }
    pub fn stsminl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminlEmitter<T0, T1>,
    {
        <Self as StsminlEmitter<T0, T1>>::stsminl(self, op0, op1);
    }
    pub fn stsminlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminlbEmitter<T0, T1>,
    {
        <Self as StsminlbEmitter<T0, T1>>::stsminlb(self, op0, op1);
    }
    pub fn stsminlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StsminlhEmitter<T0, T1>,
    {
        <Self as StsminlhEmitter<T0, T1>>::stsminlh(self, op0, op1);
    }
    pub fn sttr<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SttrEmitter<T0, T1>,
    {
        <Self as SttrEmitter<T0, T1>>::sttr(self, op0, op1);
    }
    pub fn sttrb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SttrbEmitter<T0, T1>,
    {
        <Self as SttrbEmitter<T0, T1>>::sttrb(self, op0, op1);
    }
    pub fn sttrh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SttrhEmitter<T0, T1>,
    {
        <Self as SttrhEmitter<T0, T1>>::sttrh(self, op0, op1);
    }
    pub fn stumax<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxEmitter<T0, T1>,
    {
        <Self as StumaxEmitter<T0, T1>>::stumax(self, op0, op1);
    }
    pub fn stumaxb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxbEmitter<T0, T1>,
    {
        <Self as StumaxbEmitter<T0, T1>>::stumaxb(self, op0, op1);
    }
    pub fn stumaxh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxhEmitter<T0, T1>,
    {
        <Self as StumaxhEmitter<T0, T1>>::stumaxh(self, op0, op1);
    }
    pub fn stumaxl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxlEmitter<T0, T1>,
    {
        <Self as StumaxlEmitter<T0, T1>>::stumaxl(self, op0, op1);
    }
    pub fn stumaxlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxlbEmitter<T0, T1>,
    {
        <Self as StumaxlbEmitter<T0, T1>>::stumaxlb(self, op0, op1);
    }
    pub fn stumaxlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StumaxlhEmitter<T0, T1>,
    {
        <Self as StumaxlhEmitter<T0, T1>>::stumaxlh(self, op0, op1);
    }
    pub fn stumin<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminEmitter<T0, T1>,
    {
        <Self as StuminEmitter<T0, T1>>::stumin(self, op0, op1);
    }
    pub fn stuminb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminbEmitter<T0, T1>,
    {
        <Self as StuminbEmitter<T0, T1>>::stuminb(self, op0, op1);
    }
    pub fn stuminh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminhEmitter<T0, T1>,
    {
        <Self as StuminhEmitter<T0, T1>>::stuminh(self, op0, op1);
    }
    pub fn stuminl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminlEmitter<T0, T1>,
    {
        <Self as StuminlEmitter<T0, T1>>::stuminl(self, op0, op1);
    }
    pub fn stuminlb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminlbEmitter<T0, T1>,
    {
        <Self as StuminlbEmitter<T0, T1>>::stuminlb(self, op0, op1);
    }
    pub fn stuminlh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StuminlhEmitter<T0, T1>,
    {
        <Self as StuminlhEmitter<T0, T1>>::stuminlh(self, op0, op1);
    }
    pub fn stur<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SturEmitter<T0, T1>,
    {
        <Self as SturEmitter<T0, T1>>::stur(self, op0, op1);
    }
    pub fn sturb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SturbEmitter<T0, T1>,
    {
        <Self as SturbEmitter<T0, T1>>::sturb(self, op0, op1);
    }
    pub fn sturh<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SturhEmitter<T0, T1>,
    {
        <Self as SturhEmitter<T0, T1>>::sturh(self, op0, op1);
    }
    pub fn stxp<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: StxpEmitter<T0, T1, T2, T3>,
    {
        <Self as StxpEmitter<T0, T1, T2, T3>>::stxp(self, op0, op1, op2, op3);
    }
    pub fn stlxp<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: StlxpEmitter<T0, T1, T2, T3>,
    {
        <Self as StlxpEmitter<T0, T1, T2, T3>>::stlxp(self, op0, op1, op2, op3);
    }
    pub fn stxr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StxrEmitter<T0, T1, T2>,
    {
        <Self as StxrEmitter<T0, T1, T2>>::stxr(self, op0, op1, op2);
    }
    pub fn stxrb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StxrbEmitter<T0, T1, T2>,
    {
        <Self as StxrbEmitter<T0, T1, T2>>::stxrb(self, op0, op1, op2);
    }
    pub fn stxrh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StxrhEmitter<T0, T1, T2>,
    {
        <Self as StxrhEmitter<T0, T1, T2>>::stxrh(self, op0, op1, op2);
    }
    pub fn swp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpEmitter<T0, T1, T2>,
    {
        <Self as SwpEmitter<T0, T1, T2>>::swp(self, op0, op1, op2);
    }
    pub fn swpa<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpaEmitter<T0, T1, T2>,
    {
        <Self as SwpaEmitter<T0, T1, T2>>::swpa(self, op0, op1, op2);
    }
    pub fn swpab<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpabEmitter<T0, T1, T2>,
    {
        <Self as SwpabEmitter<T0, T1, T2>>::swpab(self, op0, op1, op2);
    }
    pub fn swpah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpahEmitter<T0, T1, T2>,
    {
        <Self as SwpahEmitter<T0, T1, T2>>::swpah(self, op0, op1, op2);
    }
    pub fn swpal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpalEmitter<T0, T1, T2>,
    {
        <Self as SwpalEmitter<T0, T1, T2>>::swpal(self, op0, op1, op2);
    }
    pub fn swpalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpalbEmitter<T0, T1, T2>,
    {
        <Self as SwpalbEmitter<T0, T1, T2>>::swpalb(self, op0, op1, op2);
    }
    pub fn swpalh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpalhEmitter<T0, T1, T2>,
    {
        <Self as SwpalhEmitter<T0, T1, T2>>::swpalh(self, op0, op1, op2);
    }
    pub fn swpb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwpbEmitter<T0, T1, T2>,
    {
        <Self as SwpbEmitter<T0, T1, T2>>::swpb(self, op0, op1, op2);
    }
    pub fn swph<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwphEmitter<T0, T1, T2>,
    {
        <Self as SwphEmitter<T0, T1, T2>>::swph(self, op0, op1, op2);
    }
    pub fn swpl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwplEmitter<T0, T1, T2>,
    {
        <Self as SwplEmitter<T0, T1, T2>>::swpl(self, op0, op1, op2);
    }
    pub fn swplb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwplbEmitter<T0, T1, T2>,
    {
        <Self as SwplbEmitter<T0, T1, T2>>::swplb(self, op0, op1, op2);
    }
    pub fn swplh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SwplhEmitter<T0, T1, T2>,
    {
        <Self as SwplhEmitter<T0, T1, T2>>::swplh(self, op0, op1, op2);
    }
    pub fn bti<T0>(&mut self, op0: T0)
    where
        Self: BtiEmitter<T0>,
    {
        <Self as BtiEmitter<T0>>::bti(self, op0);
    }
    pub fn chkfeat<T0>(&mut self, op0: T0)
    where
        Self: ChkfeatEmitter<T0>,
    {
        <Self as ChkfeatEmitter<T0>>::chkfeat(self, op0);
    }
    pub fn clrbhb(&mut self)
    where
        Self: ClrbhbEmitter,
    {
        <Self as ClrbhbEmitter>::clrbhb(self);
    }
    pub fn crc32b<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32bEmitter<T0, T1, T2>,
    {
        <Self as Crc32bEmitter<T0, T1, T2>>::crc32b(self, op0, op1, op2);
    }
    pub fn crc32h<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32hEmitter<T0, T1, T2>,
    {
        <Self as Crc32hEmitter<T0, T1, T2>>::crc32h(self, op0, op1, op2);
    }
    pub fn crc32w<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32wEmitter<T0, T1, T2>,
    {
        <Self as Crc32wEmitter<T0, T1, T2>>::crc32w(self, op0, op1, op2);
    }
    pub fn crc32x<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32xEmitter<T0, T1, T2>,
    {
        <Self as Crc32xEmitter<T0, T1, T2>>::crc32x(self, op0, op1, op2);
    }
    pub fn crc32cb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32cbEmitter<T0, T1, T2>,
    {
        <Self as Crc32cbEmitter<T0, T1, T2>>::crc32cb(self, op0, op1, op2);
    }
    pub fn crc32ch<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32chEmitter<T0, T1, T2>,
    {
        <Self as Crc32chEmitter<T0, T1, T2>>::crc32ch(self, op0, op1, op2);
    }
    pub fn crc32cw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32cwEmitter<T0, T1, T2>,
    {
        <Self as Crc32cwEmitter<T0, T1, T2>>::crc32cw(self, op0, op1, op2);
    }
    pub fn crc32cx<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Crc32cxEmitter<T0, T1, T2>,
    {
        <Self as Crc32cxEmitter<T0, T1, T2>>::crc32cx(self, op0, op1, op2);
    }
    pub fn abs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AbsEmitter<T0, T1>,
    {
        <Self as AbsEmitter<T0, T1>>::abs(self, op0, op1);
    }
    pub fn cnt<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CntEmitter<T0, T1>,
    {
        <Self as CntEmitter<T0, T1>>::cnt(self, op0, op1);
    }
    pub fn ctz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CtzEmitter<T0, T1>,
    {
        <Self as CtzEmitter<T0, T1>>::ctz(self, op0, op1);
    }
    pub fn smax<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmaxEmitter<T0, T1, T2>,
    {
        <Self as SmaxEmitter<T0, T1, T2>>::smax(self, op0, op1, op2);
    }
    pub fn smin<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SminEmitter<T0, T1, T2>,
    {
        <Self as SminEmitter<T0, T1, T2>>::smin(self, op0, op1, op2);
    }
    pub fn umax<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmaxEmitter<T0, T1, T2>,
    {
        <Self as UmaxEmitter<T0, T1, T2>>::umax(self, op0, op1, op2);
    }
    pub fn umin<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UminEmitter<T0, T1, T2>,
    {
        <Self as UminEmitter<T0, T1, T2>>::umin(self, op0, op1, op2);
    }
    pub fn dgh(&mut self)
    where
        Self: DghEmitter,
    {
        <Self as DghEmitter>::dgh(self);
    }
    pub fn cfinv(&mut self)
    where
        Self: CfinvEmitter,
    {
        <Self as CfinvEmitter>::cfinv(self);
    }
    pub fn setf8<T0>(&mut self, op0: T0)
    where
        Self: Setf8Emitter<T0>,
    {
        <Self as Setf8Emitter<T0>>::setf8(self, op0);
    }
    pub fn setf16<T0>(&mut self, op0: T0)
    where
        Self: Setf16Emitter<T0>,
    {
        <Self as Setf16Emitter<T0>>::setf16(self, op0);
    }
    pub fn axflag(&mut self)
    where
        Self: AxflagEmitter,
    {
        <Self as AxflagEmitter>::axflag(self);
    }
    pub fn xaflag(&mut self)
    where
        Self: XaflagEmitter,
    {
        <Self as XaflagEmitter>::xaflag(self);
    }
    pub fn bc_eq<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_eq(self, op0);
    }
    pub fn bc_ne<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_ne(self, op0);
    }
    pub fn bc_cs<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_cs(self, op0);
    }
    pub fn bc_hs<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_hs(self, op0);
    }
    pub fn bc_cc<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_cc(self, op0);
    }
    pub fn bc_lo<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_lo(self, op0);
    }
    pub fn bc_mi<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_mi(self, op0);
    }
    pub fn bc_pl<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_pl(self, op0);
    }
    pub fn bc_vs<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_vs(self, op0);
    }
    pub fn bc_vc<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_vc(self, op0);
    }
    pub fn bc_hi<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_hi(self, op0);
    }
    pub fn bc_ls<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_ls(self, op0);
    }
    pub fn bc_ge<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_ge(self, op0);
    }
    pub fn bc_lt<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_lt(self, op0);
    }
    pub fn bc_gt<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_gt(self, op0);
    }
    pub fn bc_le<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_le(self, op0);
    }
    pub fn bc_al<T0>(&mut self, op0: T0)
    where
        Self: BcEmitter<T0>,
    {
        <Self as BcEmitter<T0>>::bc_al(self, op0);
    }
    pub fn autda<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AutdaEmitter<T0, T1>,
    {
        <Self as AutdaEmitter<T0, T1>>::autda(self, op0, op1);
    }
    pub fn autdb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AutdbEmitter<T0, T1>,
    {
        <Self as AutdbEmitter<T0, T1>>::autdb(self, op0, op1);
    }
    pub fn autdza<T0>(&mut self, op0: T0)
    where
        Self: AutdzaEmitter<T0>,
    {
        <Self as AutdzaEmitter<T0>>::autdza(self, op0);
    }
    pub fn autdzb<T0>(&mut self, op0: T0)
    where
        Self: AutdzbEmitter<T0>,
    {
        <Self as AutdzbEmitter<T0>>::autdzb(self, op0);
    }
    pub fn autia<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AutiaEmitter<T0, T1>,
    {
        <Self as AutiaEmitter<T0, T1>>::autia(self, op0, op1);
    }
    pub fn autia1716(&mut self)
    where
        Self: Autia1716Emitter,
    {
        <Self as Autia1716Emitter>::autia1716(self);
    }
    pub fn autiasp(&mut self)
    where
        Self: AutiaspEmitter,
    {
        <Self as AutiaspEmitter>::autiasp(self);
    }
    pub fn autiaz(&mut self)
    where
        Self: AutiazEmitter,
    {
        <Self as AutiazEmitter>::autiaz(self);
    }
    pub fn autib<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AutibEmitter<T0, T1>,
    {
        <Self as AutibEmitter<T0, T1>>::autib(self, op0, op1);
    }
    pub fn autib1716(&mut self)
    where
        Self: Autib1716Emitter,
    {
        <Self as Autib1716Emitter>::autib1716(self);
    }
    pub fn autibsp(&mut self)
    where
        Self: AutibspEmitter,
    {
        <Self as AutibspEmitter>::autibsp(self);
    }
    pub fn autibz(&mut self)
    where
        Self: AutibzEmitter,
    {
        <Self as AutibzEmitter>::autibz(self);
    }
    pub fn autiza<T0>(&mut self, op0: T0)
    where
        Self: AutizaEmitter<T0>,
    {
        <Self as AutizaEmitter<T0>>::autiza(self, op0);
    }
    pub fn autizb<T0>(&mut self, op0: T0)
    where
        Self: AutizbEmitter<T0>,
    {
        <Self as AutizbEmitter<T0>>::autizb(self, op0);
    }
    pub fn gmi<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: GmiEmitter<T0, T1, T2>,
    {
        <Self as GmiEmitter<T0, T1, T2>>::gmi(self, op0, op1, op2);
    }
    pub fn cmpp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: CmppEmitter<T0, T1>,
    {
        <Self as CmppEmitter<T0, T1>>::cmpp(self, op0, op1);
    }
    pub fn addg<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: AddgEmitter<T0, T1, T2, T3>,
    {
        <Self as AddgEmitter<T0, T1, T2, T3>>::addg(self, op0, op1, op2, op3);
    }
    pub fn ldg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdgEmitter<T0, T1>,
    {
        <Self as LdgEmitter<T0, T1>>::ldg(self, op0, op1);
    }
    pub fn ldgm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdgmEmitter<T0, T1>,
    {
        <Self as LdgmEmitter<T0, T1>>::ldgm(self, op0, op1);
    }
    pub fn ldraa<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdraaEmitter<T0, T1>,
    {
        <Self as LdraaEmitter<T0, T1>>::ldraa(self, op0, op1);
    }
    pub fn ldrab<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: LdrabEmitter<T0, T1>,
    {
        <Self as LdrabEmitter<T0, T1>>::ldrab(self, op0, op1);
    }
    pub fn pacda<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: PacdaEmitter<T0, T1>,
    {
        <Self as PacdaEmitter<T0, T1>>::pacda(self, op0, op1);
    }
    pub fn pacdb<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: PacdbEmitter<T0, T1>,
    {
        <Self as PacdbEmitter<T0, T1>>::pacdb(self, op0, op1);
    }
    pub fn pacdza<T0>(&mut self, op0: T0)
    where
        Self: PacdzaEmitter<T0>,
    {
        <Self as PacdzaEmitter<T0>>::pacdza(self, op0);
    }
    pub fn pacdzb<T0>(&mut self, op0: T0)
    where
        Self: PacdzbEmitter<T0>,
    {
        <Self as PacdzbEmitter<T0>>::pacdzb(self, op0);
    }
    pub fn pacga<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: PacgaEmitter<T0, T1, T2>,
    {
        <Self as PacgaEmitter<T0, T1, T2>>::pacga(self, op0, op1, op2);
    }
    pub fn subp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SubpEmitter<T0, T1, T2>,
    {
        <Self as SubpEmitter<T0, T1, T2>>::subp(self, op0, op1, op2);
    }
    pub fn subps<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SubpsEmitter<T0, T1, T2>,
    {
        <Self as SubpsEmitter<T0, T1, T2>>::subps(self, op0, op1, op2);
    }
    pub fn subg<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: SubgEmitter<T0, T1, T2, T3>,
    {
        <Self as SubgEmitter<T0, T1, T2, T3>>::subg(self, op0, op1, op2, op3);
    }
    pub fn st2g<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: St2gEmitter<T0, T1>,
    {
        <Self as St2gEmitter<T0, T1>>::st2g(self, op0, op1);
    }
    pub fn stg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StgEmitter<T0, T1>,
    {
        <Self as StgEmitter<T0, T1>>::stg(self, op0, op1);
    }
    pub fn stgp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: StgpEmitter<T0, T1, T2>,
    {
        <Self as StgpEmitter<T0, T1, T2>>::stgp(self, op0, op1, op2);
    }
    pub fn stgm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StgmEmitter<T0, T1>,
    {
        <Self as StgmEmitter<T0, T1>>::stgm(self, op0, op1);
    }
    pub fn stzg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StzgEmitter<T0, T1>,
    {
        <Self as StzgEmitter<T0, T1>>::stzg(self, op0, op1);
    }
    pub fn stz2g<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Stz2gEmitter<T0, T1>,
    {
        <Self as Stz2gEmitter<T0, T1>>::stz2g(self, op0, op1);
    }
    pub fn stzgm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: StzgmEmitter<T0, T1>,
    {
        <Self as StzgmEmitter<T0, T1>>::stzgm(self, op0, op1);
    }
    pub fn xpacd<T0>(&mut self, op0: T0)
    where
        Self: XpacdEmitter<T0>,
    {
        <Self as XpacdEmitter<T0>>::xpacd(self, op0);
    }
    pub fn xpaci<T0>(&mut self, op0: T0)
    where
        Self: XpaciEmitter<T0>,
    {
        <Self as XpaciEmitter<T0>>::xpaci(self, op0);
    }
    pub fn xpaclri(&mut self)
    where
        Self: XpaclriEmitter,
    {
        <Self as XpaclriEmitter>::xpaclri(self);
    }
    pub fn hint<T0>(&mut self, op0: T0)
    where
        Self: HintEmitter<T0>,
    {
        <Self as HintEmitter<T0>>::hint(self, op0);
    }
    pub fn nop(&mut self)
    where
        Self: NopEmitter,
    {
        <Self as NopEmitter>::nop(self);
    }
    pub fn sev(&mut self)
    where
        Self: SevEmitter,
    {
        <Self as SevEmitter>::sev(self);
    }
    pub fn sevl(&mut self)
    where
        Self: SevlEmitter,
    {
        <Self as SevlEmitter>::sevl(self);
    }
    pub fn wfe(&mut self)
    where
        Self: WfeEmitter,
    {
        <Self as WfeEmitter>::wfe(self);
    }
    pub fn wfi(&mut self)
    where
        Self: WfiEmitter,
    {
        <Self as WfiEmitter>::wfi(self);
    }
    pub fn r#yield(&mut self)
    where
        Self: YieldEmitter,
    {
        <Self as YieldEmitter>::r#yield(self);
    }
    pub fn addhn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: AddhnEmitter<T0, T1, T2>,
    {
        <Self as AddhnEmitter<T0, T1, T2>>::addhn(self, op0, op1, op2);
    }
    pub fn addhn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Addhn2Emitter<T0, T1, T2>,
    {
        <Self as Addhn2Emitter<T0, T1, T2>>::addhn2(self, op0, op1, op2);
    }
    pub fn addp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AddpEmitter<T0, T1>,
    {
        <Self as AddpEmitter<T0, T1>>::addp(self, op0, op1);
    }
    pub fn addp_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Addp3Emitter<T0, T1, T2>,
    {
        <Self as Addp3Emitter<T0, T1, T2>>::addp_3(self, op0, op1, op2);
    }
    pub fn addv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AddvEmitter<T0, T1>,
    {
        <Self as AddvEmitter<T0, T1>>::addv(self, op0, op1);
    }
    pub fn bic_2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Bic2Emitter<T0, T1>,
    {
        <Self as Bic2Emitter<T0, T1>>::bic_2(self, op0, op1);
    }
    pub fn bif<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BifEmitter<T0, T1, T2>,
    {
        <Self as BifEmitter<T0, T1, T2>>::bif(self, op0, op1, op2);
    }
    pub fn bit<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BitEmitter<T0, T1, T2>,
    {
        <Self as BitEmitter<T0, T1, T2>>::bit(self, op0, op1, op2);
    }
    pub fn bsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BslEmitter<T0, T1, T2>,
    {
        <Self as BslEmitter<T0, T1, T2>>::bsl(self, op0, op1, op2);
    }
    pub fn cmeq<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmeqEmitter<T0, T1, T2>,
    {
        <Self as CmeqEmitter<T0, T1, T2>>::cmeq(self, op0, op1, op2);
    }
    pub fn cmge<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmgeEmitter<T0, T1, T2>,
    {
        <Self as CmgeEmitter<T0, T1, T2>>::cmge(self, op0, op1, op2);
    }
    pub fn cmgt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmgtEmitter<T0, T1, T2>,
    {
        <Self as CmgtEmitter<T0, T1, T2>>::cmgt(self, op0, op1, op2);
    }
    pub fn cmhi<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmhiEmitter<T0, T1, T2>,
    {
        <Self as CmhiEmitter<T0, T1, T2>>::cmhi(self, op0, op1, op2);
    }
    pub fn cmhs<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmhsEmitter<T0, T1, T2>,
    {
        <Self as CmhsEmitter<T0, T1, T2>>::cmhs(self, op0, op1, op2);
    }
    pub fn cmle<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmleEmitter<T0, T1, T2>,
    {
        <Self as CmleEmitter<T0, T1, T2>>::cmle(self, op0, op1, op2);
    }
    pub fn cmlt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmltEmitter<T0, T1, T2>,
    {
        <Self as CmltEmitter<T0, T1, T2>>::cmlt(self, op0, op1, op2);
    }
    pub fn cmtst<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: CmtstEmitter<T0, T1, T2>,
    {
        <Self as CmtstEmitter<T0, T1, T2>>::cmtst(self, op0, op1, op2);
    }
    pub fn dup<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: DupEmitter<T0, T1>,
    {
        <Self as DupEmitter<T0, T1>>::dup(self, op0, op1);
    }
    pub fn ext<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: ExtEmitter<T0, T1, T2, T3>,
    {
        <Self as ExtEmitter<T0, T1, T2, T3>>::ext(self, op0, op1, op2, op3);
    }
    pub fn fabd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FabdEmitter<T0, T1, T2>,
    {
        <Self as FabdEmitter<T0, T1, T2>>::fabd(self, op0, op1, op2);
    }
    pub fn fabs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FabsEmitter<T0, T1>,
    {
        <Self as FabsEmitter<T0, T1>>::fabs(self, op0, op1);
    }
    pub fn facge<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FacgeEmitter<T0, T1, T2>,
    {
        <Self as FacgeEmitter<T0, T1, T2>>::facge(self, op0, op1, op2);
    }
    pub fn facgt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FacgtEmitter<T0, T1, T2>,
    {
        <Self as FacgtEmitter<T0, T1, T2>>::facgt(self, op0, op1, op2);
    }
    pub fn fadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FaddEmitter<T0, T1, T2>,
    {
        <Self as FaddEmitter<T0, T1, T2>>::fadd(self, op0, op1, op2);
    }
    pub fn faddp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FaddpEmitter<T0, T1>,
    {
        <Self as FaddpEmitter<T0, T1>>::faddp(self, op0, op1);
    }
    pub fn faddp_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Faddp3Emitter<T0, T1, T2>,
    {
        <Self as Faddp3Emitter<T0, T1, T2>>::faddp_3(self, op0, op1, op2);
    }
    pub fn fccmp<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FccmpEmitter<T0, T1, T2, T3>,
    {
        <Self as FccmpEmitter<T0, T1, T2, T3>>::fccmp(self, op0, op1, op2, op3);
    }
    pub fn fccmpe<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FccmpeEmitter<T0, T1, T2, T3>,
    {
        <Self as FccmpeEmitter<T0, T1, T2, T3>>::fccmpe(self, op0, op1, op2, op3);
    }
    pub fn fcmeq<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FcmeqEmitter<T0, T1, T2>,
    {
        <Self as FcmeqEmitter<T0, T1, T2>>::fcmeq(self, op0, op1, op2);
    }
    pub fn fcmge<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FcmgeEmitter<T0, T1, T2>,
    {
        <Self as FcmgeEmitter<T0, T1, T2>>::fcmge(self, op0, op1, op2);
    }
    pub fn fcmgt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FcmgtEmitter<T0, T1, T2>,
    {
        <Self as FcmgtEmitter<T0, T1, T2>>::fcmgt(self, op0, op1, op2);
    }
    pub fn fcmle<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FcmleEmitter<T0, T1, T2>,
    {
        <Self as FcmleEmitter<T0, T1, T2>>::fcmle(self, op0, op1, op2);
    }
    pub fn fcmlt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FcmltEmitter<T0, T1, T2>,
    {
        <Self as FcmltEmitter<T0, T1, T2>>::fcmlt(self, op0, op1, op2);
    }
    pub fn fcmp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcmpEmitter<T0, T1>,
    {
        <Self as FcmpEmitter<T0, T1>>::fcmp(self, op0, op1);
    }
    pub fn fcmpe<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcmpeEmitter<T0, T1>,
    {
        <Self as FcmpeEmitter<T0, T1>>::fcmpe(self, op0, op1);
    }
    pub fn fcsel<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FcselEmitter<T0, T1, T2, T3>,
    {
        <Self as FcselEmitter<T0, T1, T2, T3>>::fcsel(self, op0, op1, op2, op3);
    }
    pub fn fcvt<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtEmitter<T0, T1>,
    {
        <Self as FcvtEmitter<T0, T1>>::fcvt(self, op0, op1);
    }
    pub fn fcvtas<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtasEmitter<T0, T1>,
    {
        <Self as FcvtasEmitter<T0, T1>>::fcvtas(self, op0, op1);
    }
    pub fn fcvtau<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtauEmitter<T0, T1>,
    {
        <Self as FcvtauEmitter<T0, T1>>::fcvtau(self, op0, op1);
    }
    pub fn fcvtl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtlEmitter<T0, T1>,
    {
        <Self as FcvtlEmitter<T0, T1>>::fcvtl(self, op0, op1);
    }
    pub fn fcvtl2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fcvtl2Emitter<T0, T1>,
    {
        <Self as Fcvtl2Emitter<T0, T1>>::fcvtl2(self, op0, op1);
    }
    pub fn fcvtms<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtmsEmitter<T0, T1>,
    {
        <Self as FcvtmsEmitter<T0, T1>>::fcvtms(self, op0, op1);
    }
    pub fn fcvtmu<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtmuEmitter<T0, T1>,
    {
        <Self as FcvtmuEmitter<T0, T1>>::fcvtmu(self, op0, op1);
    }
    pub fn fcvtn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtnEmitter<T0, T1>,
    {
        <Self as FcvtnEmitter<T0, T1>>::fcvtn(self, op0, op1);
    }
    pub fn fcvtn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fcvtn2Emitter<T0, T1>,
    {
        <Self as Fcvtn2Emitter<T0, T1>>::fcvtn2(self, op0, op1);
    }
    pub fn fcvtns<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtnsEmitter<T0, T1>,
    {
        <Self as FcvtnsEmitter<T0, T1>>::fcvtns(self, op0, op1);
    }
    pub fn fcvtnu<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtnuEmitter<T0, T1>,
    {
        <Self as FcvtnuEmitter<T0, T1>>::fcvtnu(self, op0, op1);
    }
    pub fn fcvtps<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtpsEmitter<T0, T1>,
    {
        <Self as FcvtpsEmitter<T0, T1>>::fcvtps(self, op0, op1);
    }
    pub fn fcvtpu<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtpuEmitter<T0, T1>,
    {
        <Self as FcvtpuEmitter<T0, T1>>::fcvtpu(self, op0, op1);
    }
    pub fn fcvtxn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtxnEmitter<T0, T1>,
    {
        <Self as FcvtxnEmitter<T0, T1>>::fcvtxn(self, op0, op1);
    }
    pub fn fcvtxn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fcvtxn2Emitter<T0, T1>,
    {
        <Self as Fcvtxn2Emitter<T0, T1>>::fcvtxn2(self, op0, op1);
    }
    pub fn fcvtzs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtzsEmitter<T0, T1>,
    {
        <Self as FcvtzsEmitter<T0, T1>>::fcvtzs(self, op0, op1);
    }
    pub fn fcvtzs_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Fcvtzs3Emitter<T0, T1, T2>,
    {
        <Self as Fcvtzs3Emitter<T0, T1, T2>>::fcvtzs_3(self, op0, op1, op2);
    }
    pub fn fcvtzu<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FcvtzuEmitter<T0, T1>,
    {
        <Self as FcvtzuEmitter<T0, T1>>::fcvtzu(self, op0, op1);
    }
    pub fn fcvtzu_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Fcvtzu3Emitter<T0, T1, T2>,
    {
        <Self as Fcvtzu3Emitter<T0, T1, T2>>::fcvtzu_3(self, op0, op1, op2);
    }
    pub fn fdiv<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FdivEmitter<T0, T1, T2>,
    {
        <Self as FdivEmitter<T0, T1, T2>>::fdiv(self, op0, op1, op2);
    }
    pub fn fmadd<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FmaddEmitter<T0, T1, T2, T3>,
    {
        <Self as FmaddEmitter<T0, T1, T2, T3>>::fmadd(self, op0, op1, op2, op3);
    }
    pub fn fmax<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmaxEmitter<T0, T1, T2>,
    {
        <Self as FmaxEmitter<T0, T1, T2>>::fmax(self, op0, op1, op2);
    }
    pub fn fmaxnm<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmaxnmEmitter<T0, T1, T2>,
    {
        <Self as FmaxnmEmitter<T0, T1, T2>>::fmaxnm(self, op0, op1, op2);
    }
    pub fn fmaxnmp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmaxnmpEmitter<T0, T1, T2>,
    {
        <Self as FmaxnmpEmitter<T0, T1, T2>>::fmaxnmp(self, op0, op1, op2);
    }
    pub fn fmaxnmp_2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fmaxnmp2Emitter<T0, T1>,
    {
        <Self as Fmaxnmp2Emitter<T0, T1>>::fmaxnmp_2(self, op0, op1);
    }
    pub fn fmaxnmv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FmaxnmvEmitter<T0, T1>,
    {
        <Self as FmaxnmvEmitter<T0, T1>>::fmaxnmv(self, op0, op1);
    }
    pub fn fmaxp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmaxpEmitter<T0, T1, T2>,
    {
        <Self as FmaxpEmitter<T0, T1, T2>>::fmaxp(self, op0, op1, op2);
    }
    pub fn fmaxp_2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fmaxp2Emitter<T0, T1>,
    {
        <Self as Fmaxp2Emitter<T0, T1>>::fmaxp_2(self, op0, op1);
    }
    pub fn fmaxv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FmaxvEmitter<T0, T1>,
    {
        <Self as FmaxvEmitter<T0, T1>>::fmaxv(self, op0, op1);
    }
    pub fn fmin<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FminEmitter<T0, T1, T2>,
    {
        <Self as FminEmitter<T0, T1, T2>>::fmin(self, op0, op1, op2);
    }
    pub fn fminnm<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FminnmEmitter<T0, T1, T2>,
    {
        <Self as FminnmEmitter<T0, T1, T2>>::fminnm(self, op0, op1, op2);
    }
    pub fn fminnmv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FminnmvEmitter<T0, T1>,
    {
        <Self as FminnmvEmitter<T0, T1>>::fminnmv(self, op0, op1);
    }
    pub fn fminnmp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FminnmpEmitter<T0, T1, T2>,
    {
        <Self as FminnmpEmitter<T0, T1, T2>>::fminnmp(self, op0, op1, op2);
    }
    pub fn fminnmp_2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Fminnmp2Emitter<T0, T1>,
    {
        <Self as Fminnmp2Emitter<T0, T1>>::fminnmp_2(self, op0, op1);
    }
    pub fn fminp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FminpEmitter<T0, T1>,
    {
        <Self as FminpEmitter<T0, T1>>::fminp(self, op0, op1);
    }
    pub fn fminp_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Fminp3Emitter<T0, T1, T2>,
    {
        <Self as Fminp3Emitter<T0, T1, T2>>::fminp_3(self, op0, op1, op2);
    }
    pub fn fminv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FminvEmitter<T0, T1>,
    {
        <Self as FminvEmitter<T0, T1>>::fminv(self, op0, op1);
    }
    pub fn fmla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmlaEmitter<T0, T1, T2>,
    {
        <Self as FmlaEmitter<T0, T1, T2>>::fmla(self, op0, op1, op2);
    }
    pub fn fmls<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmlsEmitter<T0, T1, T2>,
    {
        <Self as FmlsEmitter<T0, T1, T2>>::fmls(self, op0, op1, op2);
    }
    pub fn fmov<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FmovEmitter<T0, T1>,
    {
        <Self as FmovEmitter<T0, T1>>::fmov(self, op0, op1);
    }
    pub fn fmsub<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FmsubEmitter<T0, T1, T2, T3>,
    {
        <Self as FmsubEmitter<T0, T1, T2, T3>>::fmsub(self, op0, op1, op2, op3);
    }
    pub fn fmul<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmulEmitter<T0, T1, T2>,
    {
        <Self as FmulEmitter<T0, T1, T2>>::fmul(self, op0, op1, op2);
    }
    pub fn fmulx<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmulxEmitter<T0, T1, T2>,
    {
        <Self as FmulxEmitter<T0, T1, T2>>::fmulx(self, op0, op1, op2);
    }
    pub fn fneg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FnegEmitter<T0, T1>,
    {
        <Self as FnegEmitter<T0, T1>>::fneg(self, op0, op1);
    }
    pub fn fnmadd<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FnmaddEmitter<T0, T1, T2, T3>,
    {
        <Self as FnmaddEmitter<T0, T1, T2, T3>>::fnmadd(self, op0, op1, op2, op3);
    }
    pub fn fnmsub<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FnmsubEmitter<T0, T1, T2, T3>,
    {
        <Self as FnmsubEmitter<T0, T1, T2, T3>>::fnmsub(self, op0, op1, op2, op3);
    }
    pub fn fnmul<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FnmulEmitter<T0, T1, T2>,
    {
        <Self as FnmulEmitter<T0, T1, T2>>::fnmul(self, op0, op1, op2);
    }
    pub fn frecpe<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrecpeEmitter<T0, T1>,
    {
        <Self as FrecpeEmitter<T0, T1>>::frecpe(self, op0, op1);
    }
    pub fn frecps<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FrecpsEmitter<T0, T1, T2>,
    {
        <Self as FrecpsEmitter<T0, T1, T2>>::frecps(self, op0, op1, op2);
    }
    pub fn frecpx<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrecpxEmitter<T0, T1>,
    {
        <Self as FrecpxEmitter<T0, T1>>::frecpx(self, op0, op1);
    }
    pub fn frint32x<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Frint32xEmitter<T0, T1>,
    {
        <Self as Frint32xEmitter<T0, T1>>::frint32x(self, op0, op1);
    }
    pub fn frint32z<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Frint32zEmitter<T0, T1>,
    {
        <Self as Frint32zEmitter<T0, T1>>::frint32z(self, op0, op1);
    }
    pub fn frint64x<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Frint64xEmitter<T0, T1>,
    {
        <Self as Frint64xEmitter<T0, T1>>::frint64x(self, op0, op1);
    }
    pub fn frint64z<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Frint64zEmitter<T0, T1>,
    {
        <Self as Frint64zEmitter<T0, T1>>::frint64z(self, op0, op1);
    }
    pub fn frinta<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintaEmitter<T0, T1>,
    {
        <Self as FrintaEmitter<T0, T1>>::frinta(self, op0, op1);
    }
    pub fn frinti<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintiEmitter<T0, T1>,
    {
        <Self as FrintiEmitter<T0, T1>>::frinti(self, op0, op1);
    }
    pub fn frintm<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintmEmitter<T0, T1>,
    {
        <Self as FrintmEmitter<T0, T1>>::frintm(self, op0, op1);
    }
    pub fn frintn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintnEmitter<T0, T1>,
    {
        <Self as FrintnEmitter<T0, T1>>::frintn(self, op0, op1);
    }
    pub fn frintp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintpEmitter<T0, T1>,
    {
        <Self as FrintpEmitter<T0, T1>>::frintp(self, op0, op1);
    }
    pub fn frintx<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintxEmitter<T0, T1>,
    {
        <Self as FrintxEmitter<T0, T1>>::frintx(self, op0, op1);
    }
    pub fn frintz<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrintzEmitter<T0, T1>,
    {
        <Self as FrintzEmitter<T0, T1>>::frintz(self, op0, op1);
    }
    pub fn frsqrte<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FrsqrteEmitter<T0, T1>,
    {
        <Self as FrsqrteEmitter<T0, T1>>::frsqrte(self, op0, op1);
    }
    pub fn frsqrts<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FrsqrtsEmitter<T0, T1, T2>,
    {
        <Self as FrsqrtsEmitter<T0, T1, T2>>::frsqrts(self, op0, op1, op2);
    }
    pub fn fsqrt<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FsqrtEmitter<T0, T1>,
    {
        <Self as FsqrtEmitter<T0, T1>>::fsqrt(self, op0, op1);
    }
    pub fn fsub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FsubEmitter<T0, T1, T2>,
    {
        <Self as FsubEmitter<T0, T1, T2>>::fsub(self, op0, op1, op2);
    }
    pub fn ins<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: InsEmitter<T0, T1>,
    {
        <Self as InsEmitter<T0, T1>>::ins(self, op0, op1);
    }
    pub fn ld1<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Ld1Emitter<T0, T1>,
    {
        <Self as Ld1Emitter<T0, T1>>::ld1(self, op0, op1);
    }
    pub fn ld1_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ld13Emitter<T0, T1, T2>,
    {
        <Self as Ld13Emitter<T0, T1, T2>>::ld1_3(self, op0, op1, op2);
    }
    pub fn ld1_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Ld14Emitter<T0, T1, T2, T3>,
    {
        <Self as Ld14Emitter<T0, T1, T2, T3>>::ld1_4(self, op0, op1, op2, op3);
    }
    pub fn ld1_5<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Ld15Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as Ld15Emitter<T0, T1, T2, T3, T4>>::ld1_5(self, op0, op1, op2, op3, op4);
    }
    pub fn ld1r<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Ld1rEmitter<T0, T1>,
    {
        <Self as Ld1rEmitter<T0, T1>>::ld1r(self, op0, op1);
    }
    pub fn ld2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ld2Emitter<T0, T1, T2>,
    {
        <Self as Ld2Emitter<T0, T1, T2>>::ld2(self, op0, op1, op2);
    }
    pub fn ld2r<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ld2rEmitter<T0, T1, T2>,
    {
        <Self as Ld2rEmitter<T0, T1, T2>>::ld2r(self, op0, op1, op2);
    }
    pub fn ld3<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Ld3Emitter<T0, T1, T2, T3>,
    {
        <Self as Ld3Emitter<T0, T1, T2, T3>>::ld3(self, op0, op1, op2, op3);
    }
    pub fn ld3r<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Ld3rEmitter<T0, T1, T2, T3>,
    {
        <Self as Ld3rEmitter<T0, T1, T2, T3>>::ld3r(self, op0, op1, op2, op3);
    }
    pub fn ld4<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Ld4Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as Ld4Emitter<T0, T1, T2, T3, T4>>::ld4(self, op0, op1, op2, op3, op4);
    }
    pub fn ld4r<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Ld4rEmitter<T0, T1, T2, T3, T4>,
    {
        <Self as Ld4rEmitter<T0, T1, T2, T3, T4>>::ld4r(self, op0, op1, op2, op3, op4);
    }
    pub fn mla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: MlaEmitter<T0, T1, T2>,
    {
        <Self as MlaEmitter<T0, T1, T2>>::mla(self, op0, op1, op2);
    }
    pub fn mls<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: MlsEmitter<T0, T1, T2>,
    {
        <Self as MlsEmitter<T0, T1, T2>>::mls(self, op0, op1, op2);
    }
    pub fn movi<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MoviEmitter<T0, T1>,
    {
        <Self as MoviEmitter<T0, T1>>::movi(self, op0, op1);
    }
    pub fn movi_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Movi3Emitter<T0, T1, T2>,
    {
        <Self as Movi3Emitter<T0, T1, T2>>::movi_3(self, op0, op1, op2);
    }
    pub fn mvni<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: MvniEmitter<T0, T1>,
    {
        <Self as MvniEmitter<T0, T1>>::mvni(self, op0, op1);
    }
    pub fn mvni_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Mvni3Emitter<T0, T1, T2>,
    {
        <Self as Mvni3Emitter<T0, T1, T2>>::mvni_3(self, op0, op1, op2);
    }
    pub fn not_<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: NotEmitter<T0, T1>,
    {
        <Self as NotEmitter<T0, T1>>::not_(self, op0, op1);
    }
    pub fn orr_2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Orr2Emitter<T0, T1>,
    {
        <Self as Orr2Emitter<T0, T1>>::orr_2(self, op0, op1);
    }
    pub fn pmul<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: PmulEmitter<T0, T1, T2>,
    {
        <Self as PmulEmitter<T0, T1, T2>>::pmul(self, op0, op1, op2);
    }
    pub fn pmull<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: PmullEmitter<T0, T1, T2>,
    {
        <Self as PmullEmitter<T0, T1, T2>>::pmull(self, op0, op1, op2);
    }
    pub fn pmull2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Pmull2Emitter<T0, T1, T2>,
    {
        <Self as Pmull2Emitter<T0, T1, T2>>::pmull2(self, op0, op1, op2);
    }
    pub fn raddhn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: RaddhnEmitter<T0, T1, T2>,
    {
        <Self as RaddhnEmitter<T0, T1, T2>>::raddhn(self, op0, op1, op2);
    }
    pub fn raddhn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Raddhn2Emitter<T0, T1, T2>,
    {
        <Self as Raddhn2Emitter<T0, T1, T2>>::raddhn2(self, op0, op1, op2);
    }
    pub fn rshrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: RshrnEmitter<T0, T1, T2>,
    {
        <Self as RshrnEmitter<T0, T1, T2>>::rshrn(self, op0, op1, op2);
    }
    pub fn rshrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Rshrn2Emitter<T0, T1, T2>,
    {
        <Self as Rshrn2Emitter<T0, T1, T2>>::rshrn2(self, op0, op1, op2);
    }
    pub fn rsubhn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: RsubhnEmitter<T0, T1, T2>,
    {
        <Self as RsubhnEmitter<T0, T1, T2>>::rsubhn(self, op0, op1, op2);
    }
    pub fn rsubhn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Rsubhn2Emitter<T0, T1, T2>,
    {
        <Self as Rsubhn2Emitter<T0, T1, T2>>::rsubhn2(self, op0, op1, op2);
    }
    pub fn saba<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SabaEmitter<T0, T1, T2>,
    {
        <Self as SabaEmitter<T0, T1, T2>>::saba(self, op0, op1, op2);
    }
    pub fn sabal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SabalEmitter<T0, T1, T2>,
    {
        <Self as SabalEmitter<T0, T1, T2>>::sabal(self, op0, op1, op2);
    }
    pub fn sabal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sabal2Emitter<T0, T1, T2>,
    {
        <Self as Sabal2Emitter<T0, T1, T2>>::sabal2(self, op0, op1, op2);
    }
    pub fn sabd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SabdEmitter<T0, T1, T2>,
    {
        <Self as SabdEmitter<T0, T1, T2>>::sabd(self, op0, op1, op2);
    }
    pub fn sabdl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SabdlEmitter<T0, T1, T2>,
    {
        <Self as SabdlEmitter<T0, T1, T2>>::sabdl(self, op0, op1, op2);
    }
    pub fn sabdl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sabdl2Emitter<T0, T1, T2>,
    {
        <Self as Sabdl2Emitter<T0, T1, T2>>::sabdl2(self, op0, op1, op2);
    }
    pub fn sadalp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SadalpEmitter<T0, T1>,
    {
        <Self as SadalpEmitter<T0, T1>>::sadalp(self, op0, op1);
    }
    pub fn saddl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SaddlEmitter<T0, T1, T2>,
    {
        <Self as SaddlEmitter<T0, T1, T2>>::saddl(self, op0, op1, op2);
    }
    pub fn saddl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Saddl2Emitter<T0, T1, T2>,
    {
        <Self as Saddl2Emitter<T0, T1, T2>>::saddl2(self, op0, op1, op2);
    }
    pub fn saddlp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SaddlpEmitter<T0, T1>,
    {
        <Self as SaddlpEmitter<T0, T1>>::saddlp(self, op0, op1);
    }
    pub fn saddlv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SaddlvEmitter<T0, T1>,
    {
        <Self as SaddlvEmitter<T0, T1>>::saddlv(self, op0, op1);
    }
    pub fn saddw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SaddwEmitter<T0, T1, T2>,
    {
        <Self as SaddwEmitter<T0, T1, T2>>::saddw(self, op0, op1, op2);
    }
    pub fn saddw2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Saddw2Emitter<T0, T1, T2>,
    {
        <Self as Saddw2Emitter<T0, T1, T2>>::saddw2(self, op0, op1, op2);
    }
    pub fn scvtf<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: ScvtfEmitter<T0, T1>,
    {
        <Self as ScvtfEmitter<T0, T1>>::scvtf(self, op0, op1);
    }
    pub fn scvtf_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Scvtf3Emitter<T0, T1, T2>,
    {
        <Self as Scvtf3Emitter<T0, T1, T2>>::scvtf_3(self, op0, op1, op2);
    }
    pub fn shadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: ShaddEmitter<T0, T1, T2>,
    {
        <Self as ShaddEmitter<T0, T1, T2>>::shadd(self, op0, op1, op2);
    }
    pub fn shl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: ShlEmitter<T0, T1, T2>,
    {
        <Self as ShlEmitter<T0, T1, T2>>::shl(self, op0, op1, op2);
    }
    pub fn shll<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: ShllEmitter<T0, T1, T2>,
    {
        <Self as ShllEmitter<T0, T1, T2>>::shll(self, op0, op1, op2);
    }
    pub fn shll2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Shll2Emitter<T0, T1, T2>,
    {
        <Self as Shll2Emitter<T0, T1, T2>>::shll2(self, op0, op1, op2);
    }
    pub fn shrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: ShrnEmitter<T0, T1, T2>,
    {
        <Self as ShrnEmitter<T0, T1, T2>>::shrn(self, op0, op1, op2);
    }
    pub fn shrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Shrn2Emitter<T0, T1, T2>,
    {
        <Self as Shrn2Emitter<T0, T1, T2>>::shrn2(self, op0, op1, op2);
    }
    pub fn shsub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: ShsubEmitter<T0, T1, T2>,
    {
        <Self as ShsubEmitter<T0, T1, T2>>::shsub(self, op0, op1, op2);
    }
    pub fn sli<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SliEmitter<T0, T1, T2>,
    {
        <Self as SliEmitter<T0, T1, T2>>::sli(self, op0, op1, op2);
    }
    pub fn smaxp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmaxpEmitter<T0, T1, T2>,
    {
        <Self as SmaxpEmitter<T0, T1, T2>>::smaxp(self, op0, op1, op2);
    }
    pub fn smaxv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SmaxvEmitter<T0, T1>,
    {
        <Self as SmaxvEmitter<T0, T1>>::smaxv(self, op0, op1);
    }
    pub fn sminp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SminpEmitter<T0, T1, T2>,
    {
        <Self as SminpEmitter<T0, T1, T2>>::sminp(self, op0, op1, op2);
    }
    pub fn sminv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SminvEmitter<T0, T1>,
    {
        <Self as SminvEmitter<T0, T1>>::sminv(self, op0, op1);
    }
    pub fn smlal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmlalEmitter<T0, T1, T2>,
    {
        <Self as SmlalEmitter<T0, T1, T2>>::smlal(self, op0, op1, op2);
    }
    pub fn smlal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Smlal2Emitter<T0, T1, T2>,
    {
        <Self as Smlal2Emitter<T0, T1, T2>>::smlal2(self, op0, op1, op2);
    }
    pub fn smlsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmlslEmitter<T0, T1, T2>,
    {
        <Self as SmlslEmitter<T0, T1, T2>>::smlsl(self, op0, op1, op2);
    }
    pub fn smlsl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Smlsl2Emitter<T0, T1, T2>,
    {
        <Self as Smlsl2Emitter<T0, T1, T2>>::smlsl2(self, op0, op1, op2);
    }
    pub fn smov<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SmovEmitter<T0, T1>,
    {
        <Self as SmovEmitter<T0, T1>>::smov(self, op0, op1);
    }
    pub fn smull2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Smull2Emitter<T0, T1, T2>,
    {
        <Self as Smull2Emitter<T0, T1, T2>>::smull2(self, op0, op1, op2);
    }
    pub fn sqabs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SqabsEmitter<T0, T1>,
    {
        <Self as SqabsEmitter<T0, T1>>::sqabs(self, op0, op1);
    }
    pub fn sqadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqaddEmitter<T0, T1, T2>,
    {
        <Self as SqaddEmitter<T0, T1, T2>>::sqadd(self, op0, op1, op2);
    }
    pub fn sqdmlal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqdmlalEmitter<T0, T1, T2>,
    {
        <Self as SqdmlalEmitter<T0, T1, T2>>::sqdmlal(self, op0, op1, op2);
    }
    pub fn sqdmlal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqdmlal2Emitter<T0, T1, T2>,
    {
        <Self as Sqdmlal2Emitter<T0, T1, T2>>::sqdmlal2(self, op0, op1, op2);
    }
    pub fn sqdmlsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqdmlslEmitter<T0, T1, T2>,
    {
        <Self as SqdmlslEmitter<T0, T1, T2>>::sqdmlsl(self, op0, op1, op2);
    }
    pub fn sqdmlsl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqdmlsl2Emitter<T0, T1, T2>,
    {
        <Self as Sqdmlsl2Emitter<T0, T1, T2>>::sqdmlsl2(self, op0, op1, op2);
    }
    pub fn sqdmulh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqdmulhEmitter<T0, T1, T2>,
    {
        <Self as SqdmulhEmitter<T0, T1, T2>>::sqdmulh(self, op0, op1, op2);
    }
    pub fn sqdmull<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqdmullEmitter<T0, T1, T2>,
    {
        <Self as SqdmullEmitter<T0, T1, T2>>::sqdmull(self, op0, op1, op2);
    }
    pub fn sqdmull2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqdmull2Emitter<T0, T1, T2>,
    {
        <Self as Sqdmull2Emitter<T0, T1, T2>>::sqdmull2(self, op0, op1, op2);
    }
    pub fn sqneg<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SqnegEmitter<T0, T1>,
    {
        <Self as SqnegEmitter<T0, T1>>::sqneg(self, op0, op1);
    }
    pub fn sqrdmulh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrdmulhEmitter<T0, T1, T2>,
    {
        <Self as SqrdmulhEmitter<T0, T1, T2>>::sqrdmulh(self, op0, op1, op2);
    }
    pub fn sqrshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrshlEmitter<T0, T1, T2>,
    {
        <Self as SqrshlEmitter<T0, T1, T2>>::sqrshl(self, op0, op1, op2);
    }
    pub fn sqrshrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrshrnEmitter<T0, T1, T2>,
    {
        <Self as SqrshrnEmitter<T0, T1, T2>>::sqrshrn(self, op0, op1, op2);
    }
    pub fn sqrshrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqrshrn2Emitter<T0, T1, T2>,
    {
        <Self as Sqrshrn2Emitter<T0, T1, T2>>::sqrshrn2(self, op0, op1, op2);
    }
    pub fn sqrshrun<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrshrunEmitter<T0, T1, T2>,
    {
        <Self as SqrshrunEmitter<T0, T1, T2>>::sqrshrun(self, op0, op1, op2);
    }
    pub fn sqrshrun2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqrshrun2Emitter<T0, T1, T2>,
    {
        <Self as Sqrshrun2Emitter<T0, T1, T2>>::sqrshrun2(self, op0, op1, op2);
    }
    pub fn sqshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqshlEmitter<T0, T1, T2>,
    {
        <Self as SqshlEmitter<T0, T1, T2>>::sqshl(self, op0, op1, op2);
    }
    pub fn sqshlu<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqshluEmitter<T0, T1, T2>,
    {
        <Self as SqshluEmitter<T0, T1, T2>>::sqshlu(self, op0, op1, op2);
    }
    pub fn sqshrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqshrnEmitter<T0, T1, T2>,
    {
        <Self as SqshrnEmitter<T0, T1, T2>>::sqshrn(self, op0, op1, op2);
    }
    pub fn sqshrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqshrn2Emitter<T0, T1, T2>,
    {
        <Self as Sqshrn2Emitter<T0, T1, T2>>::sqshrn2(self, op0, op1, op2);
    }
    pub fn sqshrun<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqshrunEmitter<T0, T1, T2>,
    {
        <Self as SqshrunEmitter<T0, T1, T2>>::sqshrun(self, op0, op1, op2);
    }
    pub fn sqshrun2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sqshrun2Emitter<T0, T1, T2>,
    {
        <Self as Sqshrun2Emitter<T0, T1, T2>>::sqshrun2(self, op0, op1, op2);
    }
    pub fn sqsub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqsubEmitter<T0, T1, T2>,
    {
        <Self as SqsubEmitter<T0, T1, T2>>::sqsub(self, op0, op1, op2);
    }
    pub fn sqxtn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SqxtnEmitter<T0, T1>,
    {
        <Self as SqxtnEmitter<T0, T1>>::sqxtn(self, op0, op1);
    }
    pub fn sqxtn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sqxtn2Emitter<T0, T1>,
    {
        <Self as Sqxtn2Emitter<T0, T1>>::sqxtn2(self, op0, op1);
    }
    pub fn sqxtun<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SqxtunEmitter<T0, T1>,
    {
        <Self as SqxtunEmitter<T0, T1>>::sqxtun(self, op0, op1);
    }
    pub fn sqxtun2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sqxtun2Emitter<T0, T1>,
    {
        <Self as Sqxtun2Emitter<T0, T1>>::sqxtun2(self, op0, op1);
    }
    pub fn srhadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SrhaddEmitter<T0, T1, T2>,
    {
        <Self as SrhaddEmitter<T0, T1, T2>>::srhadd(self, op0, op1, op2);
    }
    pub fn sri<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SriEmitter<T0, T1, T2>,
    {
        <Self as SriEmitter<T0, T1, T2>>::sri(self, op0, op1, op2);
    }
    pub fn srshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SrshlEmitter<T0, T1, T2>,
    {
        <Self as SrshlEmitter<T0, T1, T2>>::srshl(self, op0, op1, op2);
    }
    pub fn srshr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SrshrEmitter<T0, T1, T2>,
    {
        <Self as SrshrEmitter<T0, T1, T2>>::srshr(self, op0, op1, op2);
    }
    pub fn srsra<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SrsraEmitter<T0, T1, T2>,
    {
        <Self as SrsraEmitter<T0, T1, T2>>::srsra(self, op0, op1, op2);
    }
    pub fn sshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SshlEmitter<T0, T1, T2>,
    {
        <Self as SshlEmitter<T0, T1, T2>>::sshl(self, op0, op1, op2);
    }
    pub fn sshll<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SshllEmitter<T0, T1, T2>,
    {
        <Self as SshllEmitter<T0, T1, T2>>::sshll(self, op0, op1, op2);
    }
    pub fn sshll2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sshll2Emitter<T0, T1, T2>,
    {
        <Self as Sshll2Emitter<T0, T1, T2>>::sshll2(self, op0, op1, op2);
    }
    pub fn sshr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SshrEmitter<T0, T1, T2>,
    {
        <Self as SshrEmitter<T0, T1, T2>>::sshr(self, op0, op1, op2);
    }
    pub fn ssra<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SsraEmitter<T0, T1, T2>,
    {
        <Self as SsraEmitter<T0, T1, T2>>::ssra(self, op0, op1, op2);
    }
    pub fn ssubl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SsublEmitter<T0, T1, T2>,
    {
        <Self as SsublEmitter<T0, T1, T2>>::ssubl(self, op0, op1, op2);
    }
    pub fn ssubl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ssubl2Emitter<T0, T1, T2>,
    {
        <Self as Ssubl2Emitter<T0, T1, T2>>::ssubl2(self, op0, op1, op2);
    }
    pub fn ssubw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SsubwEmitter<T0, T1, T2>,
    {
        <Self as SsubwEmitter<T0, T1, T2>>::ssubw(self, op0, op1, op2);
    }
    pub fn ssubw2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ssubw2Emitter<T0, T1, T2>,
    {
        <Self as Ssubw2Emitter<T0, T1, T2>>::ssubw2(self, op0, op1, op2);
    }
    pub fn st1<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: St1Emitter<T0, T1>,
    {
        <Self as St1Emitter<T0, T1>>::st1(self, op0, op1);
    }
    pub fn st1_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: St13Emitter<T0, T1, T2>,
    {
        <Self as St13Emitter<T0, T1, T2>>::st1_3(self, op0, op1, op2);
    }
    pub fn st1_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: St14Emitter<T0, T1, T2, T3>,
    {
        <Self as St14Emitter<T0, T1, T2, T3>>::st1_4(self, op0, op1, op2, op3);
    }
    pub fn st1_5<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: St15Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as St15Emitter<T0, T1, T2, T3, T4>>::st1_5(self, op0, op1, op2, op3, op4);
    }
    pub fn st2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: St2Emitter<T0, T1, T2>,
    {
        <Self as St2Emitter<T0, T1, T2>>::st2(self, op0, op1, op2);
    }
    pub fn st3<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: St3Emitter<T0, T1, T2, T3>,
    {
        <Self as St3Emitter<T0, T1, T2, T3>>::st3(self, op0, op1, op2, op3);
    }
    pub fn st4<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: St4Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as St4Emitter<T0, T1, T2, T3, T4>>::st4(self, op0, op1, op2, op3, op4);
    }
    pub fn subhn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SubhnEmitter<T0, T1, T2>,
    {
        <Self as SubhnEmitter<T0, T1, T2>>::subhn(self, op0, op1, op2);
    }
    pub fn subhn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Subhn2Emitter<T0, T1, T2>,
    {
        <Self as Subhn2Emitter<T0, T1, T2>>::subhn2(self, op0, op1, op2);
    }
    pub fn suqadd<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SuqaddEmitter<T0, T1>,
    {
        <Self as SuqaddEmitter<T0, T1>>::suqadd(self, op0, op1);
    }
    pub fn sxtl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: SxtlEmitter<T0, T1>,
    {
        <Self as SxtlEmitter<T0, T1>>::sxtl(self, op0, op1);
    }
    pub fn sxtl2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sxtl2Emitter<T0, T1>,
    {
        <Self as Sxtl2Emitter<T0, T1>>::sxtl2(self, op0, op1);
    }
    pub fn tbl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: TblEmitter<T0, T1, T2>,
    {
        <Self as TblEmitter<T0, T1, T2>>::tbl(self, op0, op1, op2);
    }
    pub fn tbl_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Tbl4Emitter<T0, T1, T2, T3>,
    {
        <Self as Tbl4Emitter<T0, T1, T2, T3>>::tbl_4(self, op0, op1, op2, op3);
    }
    pub fn tbl_5<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Tbl5Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as Tbl5Emitter<T0, T1, T2, T3, T4>>::tbl_5(self, op0, op1, op2, op3, op4);
    }
    pub fn tbl_6<T0, T1, T2, T3, T4, T5>(
        &mut self,
        op0: T0,
        op1: T1,
        op2: T2,
        op3: T3,
        op4: T4,
        op5: T5,
    ) where
        Self: Tbl6Emitter<T0, T1, T2, T3, T4, T5>,
    {
        <Self as Tbl6Emitter<T0, T1, T2, T3, T4, T5>>::tbl_6(self, op0, op1, op2, op3, op4, op5);
    }
    pub fn tbx<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: TbxEmitter<T0, T1, T2>,
    {
        <Self as TbxEmitter<T0, T1, T2>>::tbx(self, op0, op1, op2);
    }
    pub fn tbx_4<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Tbx4Emitter<T0, T1, T2, T3>,
    {
        <Self as Tbx4Emitter<T0, T1, T2, T3>>::tbx_4(self, op0, op1, op2, op3);
    }
    pub fn tbx_5<T0, T1, T2, T3, T4>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3, op4: T4)
    where
        Self: Tbx5Emitter<T0, T1, T2, T3, T4>,
    {
        <Self as Tbx5Emitter<T0, T1, T2, T3, T4>>::tbx_5(self, op0, op1, op2, op3, op4);
    }
    pub fn tbx_6<T0, T1, T2, T3, T4, T5>(
        &mut self,
        op0: T0,
        op1: T1,
        op2: T2,
        op3: T3,
        op4: T4,
        op5: T5,
    ) where
        Self: Tbx6Emitter<T0, T1, T2, T3, T4, T5>,
    {
        <Self as Tbx6Emitter<T0, T1, T2, T3, T4, T5>>::tbx_6(self, op0, op1, op2, op3, op4, op5);
    }
    pub fn trn1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Trn1Emitter<T0, T1, T2>,
    {
        <Self as Trn1Emitter<T0, T1, T2>>::trn1(self, op0, op1, op2);
    }
    pub fn trn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Trn2Emitter<T0, T1, T2>,
    {
        <Self as Trn2Emitter<T0, T1, T2>>::trn2(self, op0, op1, op2);
    }
    pub fn uaba<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UabaEmitter<T0, T1, T2>,
    {
        <Self as UabaEmitter<T0, T1, T2>>::uaba(self, op0, op1, op2);
    }
    pub fn uabal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UabalEmitter<T0, T1, T2>,
    {
        <Self as UabalEmitter<T0, T1, T2>>::uabal(self, op0, op1, op2);
    }
    pub fn uabal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uabal2Emitter<T0, T1, T2>,
    {
        <Self as Uabal2Emitter<T0, T1, T2>>::uabal2(self, op0, op1, op2);
    }
    pub fn uabd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UabdEmitter<T0, T1, T2>,
    {
        <Self as UabdEmitter<T0, T1, T2>>::uabd(self, op0, op1, op2);
    }
    pub fn uabdl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UabdlEmitter<T0, T1, T2>,
    {
        <Self as UabdlEmitter<T0, T1, T2>>::uabdl(self, op0, op1, op2);
    }
    pub fn uabdl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uabdl2Emitter<T0, T1, T2>,
    {
        <Self as Uabdl2Emitter<T0, T1, T2>>::uabdl2(self, op0, op1, op2);
    }
    pub fn uadalp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UadalpEmitter<T0, T1>,
    {
        <Self as UadalpEmitter<T0, T1>>::uadalp(self, op0, op1);
    }
    pub fn uaddl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UaddlEmitter<T0, T1, T2>,
    {
        <Self as UaddlEmitter<T0, T1, T2>>::uaddl(self, op0, op1, op2);
    }
    pub fn uaddl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uaddl2Emitter<T0, T1, T2>,
    {
        <Self as Uaddl2Emitter<T0, T1, T2>>::uaddl2(self, op0, op1, op2);
    }
    pub fn uaddlp<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UaddlpEmitter<T0, T1>,
    {
        <Self as UaddlpEmitter<T0, T1>>::uaddlp(self, op0, op1);
    }
    pub fn uaddlv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UaddlvEmitter<T0, T1>,
    {
        <Self as UaddlvEmitter<T0, T1>>::uaddlv(self, op0, op1);
    }
    pub fn uaddw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UaddwEmitter<T0, T1, T2>,
    {
        <Self as UaddwEmitter<T0, T1, T2>>::uaddw(self, op0, op1, op2);
    }
    pub fn uaddw2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uaddw2Emitter<T0, T1, T2>,
    {
        <Self as Uaddw2Emitter<T0, T1, T2>>::uaddw2(self, op0, op1, op2);
    }
    pub fn ucvtf<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UcvtfEmitter<T0, T1>,
    {
        <Self as UcvtfEmitter<T0, T1>>::ucvtf(self, op0, op1);
    }
    pub fn ucvtf_3<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ucvtf3Emitter<T0, T1, T2>,
    {
        <Self as Ucvtf3Emitter<T0, T1, T2>>::ucvtf_3(self, op0, op1, op2);
    }
    pub fn uhadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UhaddEmitter<T0, T1, T2>,
    {
        <Self as UhaddEmitter<T0, T1, T2>>::uhadd(self, op0, op1, op2);
    }
    pub fn uhsub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UhsubEmitter<T0, T1, T2>,
    {
        <Self as UhsubEmitter<T0, T1, T2>>::uhsub(self, op0, op1, op2);
    }
    pub fn umaxp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmaxpEmitter<T0, T1, T2>,
    {
        <Self as UmaxpEmitter<T0, T1, T2>>::umaxp(self, op0, op1, op2);
    }
    pub fn umaxv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UmaxvEmitter<T0, T1>,
    {
        <Self as UmaxvEmitter<T0, T1>>::umaxv(self, op0, op1);
    }
    pub fn uminp<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UminpEmitter<T0, T1, T2>,
    {
        <Self as UminpEmitter<T0, T1, T2>>::uminp(self, op0, op1, op2);
    }
    pub fn uminv<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UminvEmitter<T0, T1>,
    {
        <Self as UminvEmitter<T0, T1>>::uminv(self, op0, op1);
    }
    pub fn umlal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmlalEmitter<T0, T1, T2>,
    {
        <Self as UmlalEmitter<T0, T1, T2>>::umlal(self, op0, op1, op2);
    }
    pub fn umlal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Umlal2Emitter<T0, T1, T2>,
    {
        <Self as Umlal2Emitter<T0, T1, T2>>::umlal2(self, op0, op1, op2);
    }
    pub fn umlsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmlslEmitter<T0, T1, T2>,
    {
        <Self as UmlslEmitter<T0, T1, T2>>::umlsl(self, op0, op1, op2);
    }
    pub fn umlsl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Umlsl2Emitter<T0, T1, T2>,
    {
        <Self as Umlsl2Emitter<T0, T1, T2>>::umlsl2(self, op0, op1, op2);
    }
    pub fn umov<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UmovEmitter<T0, T1>,
    {
        <Self as UmovEmitter<T0, T1>>::umov(self, op0, op1);
    }
    pub fn umull2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Umull2Emitter<T0, T1, T2>,
    {
        <Self as Umull2Emitter<T0, T1, T2>>::umull2(self, op0, op1, op2);
    }
    pub fn uqadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqaddEmitter<T0, T1, T2>,
    {
        <Self as UqaddEmitter<T0, T1, T2>>::uqadd(self, op0, op1, op2);
    }
    pub fn uqrshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqrshlEmitter<T0, T1, T2>,
    {
        <Self as UqrshlEmitter<T0, T1, T2>>::uqrshl(self, op0, op1, op2);
    }
    pub fn uqrshrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqrshrnEmitter<T0, T1, T2>,
    {
        <Self as UqrshrnEmitter<T0, T1, T2>>::uqrshrn(self, op0, op1, op2);
    }
    pub fn uqrshrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uqrshrn2Emitter<T0, T1, T2>,
    {
        <Self as Uqrshrn2Emitter<T0, T1, T2>>::uqrshrn2(self, op0, op1, op2);
    }
    pub fn uqshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqshlEmitter<T0, T1, T2>,
    {
        <Self as UqshlEmitter<T0, T1, T2>>::uqshl(self, op0, op1, op2);
    }
    pub fn uqshrn<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqshrnEmitter<T0, T1, T2>,
    {
        <Self as UqshrnEmitter<T0, T1, T2>>::uqshrn(self, op0, op1, op2);
    }
    pub fn uqshrn2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uqshrn2Emitter<T0, T1, T2>,
    {
        <Self as Uqshrn2Emitter<T0, T1, T2>>::uqshrn2(self, op0, op1, op2);
    }
    pub fn uqsub<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UqsubEmitter<T0, T1, T2>,
    {
        <Self as UqsubEmitter<T0, T1, T2>>::uqsub(self, op0, op1, op2);
    }
    pub fn uqxtn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UqxtnEmitter<T0, T1>,
    {
        <Self as UqxtnEmitter<T0, T1>>::uqxtn(self, op0, op1);
    }
    pub fn uqxtn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Uqxtn2Emitter<T0, T1>,
    {
        <Self as Uqxtn2Emitter<T0, T1>>::uqxtn2(self, op0, op1);
    }
    pub fn urecpe<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UrecpeEmitter<T0, T1>,
    {
        <Self as UrecpeEmitter<T0, T1>>::urecpe(self, op0, op1);
    }
    pub fn urhadd<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UrhaddEmitter<T0, T1, T2>,
    {
        <Self as UrhaddEmitter<T0, T1, T2>>::urhadd(self, op0, op1, op2);
    }
    pub fn urshl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UrshlEmitter<T0, T1, T2>,
    {
        <Self as UrshlEmitter<T0, T1, T2>>::urshl(self, op0, op1, op2);
    }
    pub fn urshr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UrshrEmitter<T0, T1, T2>,
    {
        <Self as UrshrEmitter<T0, T1, T2>>::urshr(self, op0, op1, op2);
    }
    pub fn ursqrte<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UrsqrteEmitter<T0, T1>,
    {
        <Self as UrsqrteEmitter<T0, T1>>::ursqrte(self, op0, op1);
    }
    pub fn ursra<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UrsraEmitter<T0, T1, T2>,
    {
        <Self as UrsraEmitter<T0, T1, T2>>::ursra(self, op0, op1, op2);
    }
    pub fn ushl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UshlEmitter<T0, T1, T2>,
    {
        <Self as UshlEmitter<T0, T1, T2>>::ushl(self, op0, op1, op2);
    }
    pub fn ushll<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UshllEmitter<T0, T1, T2>,
    {
        <Self as UshllEmitter<T0, T1, T2>>::ushll(self, op0, op1, op2);
    }
    pub fn ushll2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Ushll2Emitter<T0, T1, T2>,
    {
        <Self as Ushll2Emitter<T0, T1, T2>>::ushll2(self, op0, op1, op2);
    }
    pub fn ushr<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UshrEmitter<T0, T1, T2>,
    {
        <Self as UshrEmitter<T0, T1, T2>>::ushr(self, op0, op1, op2);
    }
    pub fn usqadd<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UsqaddEmitter<T0, T1>,
    {
        <Self as UsqaddEmitter<T0, T1>>::usqadd(self, op0, op1);
    }
    pub fn usra<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UsraEmitter<T0, T1, T2>,
    {
        <Self as UsraEmitter<T0, T1, T2>>::usra(self, op0, op1, op2);
    }
    pub fn usubl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UsublEmitter<T0, T1, T2>,
    {
        <Self as UsublEmitter<T0, T1, T2>>::usubl(self, op0, op1, op2);
    }
    pub fn usubl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Usubl2Emitter<T0, T1, T2>,
    {
        <Self as Usubl2Emitter<T0, T1, T2>>::usubl2(self, op0, op1, op2);
    }
    pub fn usubw<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UsubwEmitter<T0, T1, T2>,
    {
        <Self as UsubwEmitter<T0, T1, T2>>::usubw(self, op0, op1, op2);
    }
    pub fn usubw2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Usubw2Emitter<T0, T1, T2>,
    {
        <Self as Usubw2Emitter<T0, T1, T2>>::usubw2(self, op0, op1, op2);
    }
    pub fn uxtl<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: UxtlEmitter<T0, T1>,
    {
        <Self as UxtlEmitter<T0, T1>>::uxtl(self, op0, op1);
    }
    pub fn uxtl2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Uxtl2Emitter<T0, T1>,
    {
        <Self as Uxtl2Emitter<T0, T1>>::uxtl2(self, op0, op1);
    }
    pub fn uzp1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uzp1Emitter<T0, T1, T2>,
    {
        <Self as Uzp1Emitter<T0, T1, T2>>::uzp1(self, op0, op1, op2);
    }
    pub fn uzp2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Uzp2Emitter<T0, T1, T2>,
    {
        <Self as Uzp2Emitter<T0, T1, T2>>::uzp2(self, op0, op1, op2);
    }
    pub fn xtn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: XtnEmitter<T0, T1>,
    {
        <Self as XtnEmitter<T0, T1>>::xtn(self, op0, op1);
    }
    pub fn xtn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Xtn2Emitter<T0, T1>,
    {
        <Self as Xtn2Emitter<T0, T1>>::xtn2(self, op0, op1);
    }
    pub fn zip1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Zip1Emitter<T0, T1, T2>,
    {
        <Self as Zip1Emitter<T0, T1, T2>>::zip1(self, op0, op1, op2);
    }
    pub fn zip2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Zip2Emitter<T0, T1, T2>,
    {
        <Self as Zip2Emitter<T0, T1, T2>>::zip2(self, op0, op1, op2);
    }
    pub fn aesd<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AesdEmitter<T0, T1>,
    {
        <Self as AesdEmitter<T0, T1>>::aesd(self, op0, op1);
    }
    pub fn aese<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AeseEmitter<T0, T1>,
    {
        <Self as AeseEmitter<T0, T1>>::aese(self, op0, op1);
    }
    pub fn aesimc<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AesimcEmitter<T0, T1>,
    {
        <Self as AesimcEmitter<T0, T1>>::aesimc(self, op0, op1);
    }
    pub fn aesmc<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: AesmcEmitter<T0, T1>,
    {
        <Self as AesmcEmitter<T0, T1>>::aesmc(self, op0, op1);
    }
    pub fn sha1c<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha1cEmitter<T0, T1, T2>,
    {
        <Self as Sha1cEmitter<T0, T1, T2>>::sha1c(self, op0, op1, op2);
    }
    pub fn sha1h<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sha1hEmitter<T0, T1>,
    {
        <Self as Sha1hEmitter<T0, T1>>::sha1h(self, op0, op1);
    }
    pub fn sha1m<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha1mEmitter<T0, T1, T2>,
    {
        <Self as Sha1mEmitter<T0, T1, T2>>::sha1m(self, op0, op1, op2);
    }
    pub fn sha1p<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha1pEmitter<T0, T1, T2>,
    {
        <Self as Sha1pEmitter<T0, T1, T2>>::sha1p(self, op0, op1, op2);
    }
    pub fn sha1su0<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha1su0Emitter<T0, T1, T2>,
    {
        <Self as Sha1su0Emitter<T0, T1, T2>>::sha1su0(self, op0, op1, op2);
    }
    pub fn sha1su1<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sha1su1Emitter<T0, T1>,
    {
        <Self as Sha1su1Emitter<T0, T1>>::sha1su1(self, op0, op1);
    }
    pub fn sha256h<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha256hEmitter<T0, T1, T2>,
    {
        <Self as Sha256hEmitter<T0, T1, T2>>::sha256h(self, op0, op1, op2);
    }
    pub fn sha256h2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha256h2Emitter<T0, T1, T2>,
    {
        <Self as Sha256h2Emitter<T0, T1, T2>>::sha256h2(self, op0, op1, op2);
    }
    pub fn sha256su0<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sha256su0Emitter<T0, T1>,
    {
        <Self as Sha256su0Emitter<T0, T1>>::sha256su0(self, op0, op1);
    }
    pub fn sha256su1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha256su1Emitter<T0, T1, T2>,
    {
        <Self as Sha256su1Emitter<T0, T1, T2>>::sha256su1(self, op0, op1, op2);
    }
    pub fn sqrdmlah<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrdmlahEmitter<T0, T1, T2>,
    {
        <Self as SqrdmlahEmitter<T0, T1, T2>>::sqrdmlah(self, op0, op1, op2);
    }
    pub fn sqrdmlsh<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SqrdmlshEmitter<T0, T1, T2>,
    {
        <Self as SqrdmlshEmitter<T0, T1, T2>>::sqrdmlsh(self, op0, op1, op2);
    }
    pub fn fcadd<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FcaddEmitter<T0, T1, T2, T3>,
    {
        <Self as FcaddEmitter<T0, T1, T2, T3>>::fcadd(self, op0, op1, op2, op3);
    }
    pub fn fcmla<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: FcmlaEmitter<T0, T1, T2, T3>,
    {
        <Self as FcmlaEmitter<T0, T1, T2, T3>>::fcmla(self, op0, op1, op2, op3);
    }
    pub fn fjcvtzs<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: FjcvtzsEmitter<T0, T1>,
    {
        <Self as FjcvtzsEmitter<T0, T1>>::fjcvtzs(self, op0, op1);
    }
    pub fn fmlal<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmlalEmitter<T0, T1, T2>,
    {
        <Self as FmlalEmitter<T0, T1, T2>>::fmlal(self, op0, op1, op2);
    }
    pub fn fmlal2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Fmlal2Emitter<T0, T1, T2>,
    {
        <Self as Fmlal2Emitter<T0, T1, T2>>::fmlal2(self, op0, op1, op2);
    }
    pub fn fmlsl<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: FmlslEmitter<T0, T1, T2>,
    {
        <Self as FmlslEmitter<T0, T1, T2>>::fmlsl(self, op0, op1, op2);
    }
    pub fn fmlsl2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Fmlsl2Emitter<T0, T1, T2>,
    {
        <Self as Fmlsl2Emitter<T0, T1, T2>>::fmlsl2(self, op0, op1, op2);
    }
    pub fn bcax<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: BcaxEmitter<T0, T1, T2, T3>,
    {
        <Self as BcaxEmitter<T0, T1, T2, T3>>::bcax(self, op0, op1, op2, op3);
    }
    pub fn eor3<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Eor3Emitter<T0, T1, T2, T3>,
    {
        <Self as Eor3Emitter<T0, T1, T2, T3>>::eor3(self, op0, op1, op2, op3);
    }
    pub fn rax1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Rax1Emitter<T0, T1, T2>,
    {
        <Self as Rax1Emitter<T0, T1, T2>>::rax1(self, op0, op1, op2);
    }
    pub fn xar<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: XarEmitter<T0, T1, T2, T3>,
    {
        <Self as XarEmitter<T0, T1, T2, T3>>::xar(self, op0, op1, op2, op3);
    }
    pub fn sha512h<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha512hEmitter<T0, T1, T2>,
    {
        <Self as Sha512hEmitter<T0, T1, T2>>::sha512h(self, op0, op1, op2);
    }
    pub fn sha512h2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha512h2Emitter<T0, T1, T2>,
    {
        <Self as Sha512h2Emitter<T0, T1, T2>>::sha512h2(self, op0, op1, op2);
    }
    pub fn sha512su0<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sha512su0Emitter<T0, T1>,
    {
        <Self as Sha512su0Emitter<T0, T1>>::sha512su0(self, op0, op1);
    }
    pub fn sha512su1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sha512su1Emitter<T0, T1, T2>,
    {
        <Self as Sha512su1Emitter<T0, T1, T2>>::sha512su1(self, op0, op1, op2);
    }
    pub fn sm3partw1<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3partw1Emitter<T0, T1, T2>,
    {
        <Self as Sm3partw1Emitter<T0, T1, T2>>::sm3partw1(self, op0, op1, op2);
    }
    pub fn sm3partw2<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3partw2Emitter<T0, T1, T2>,
    {
        <Self as Sm3partw2Emitter<T0, T1, T2>>::sm3partw2(self, op0, op1, op2);
    }
    pub fn sm3ss1<T0, T1, T2, T3>(&mut self, op0: T0, op1: T1, op2: T2, op3: T3)
    where
        Self: Sm3ss1Emitter<T0, T1, T2, T3>,
    {
        <Self as Sm3ss1Emitter<T0, T1, T2, T3>>::sm3ss1(self, op0, op1, op2, op3);
    }
    pub fn sm3tt1a<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3tt1aEmitter<T0, T1, T2>,
    {
        <Self as Sm3tt1aEmitter<T0, T1, T2>>::sm3tt1a(self, op0, op1, op2);
    }
    pub fn sm3tt1b<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3tt1bEmitter<T0, T1, T2>,
    {
        <Self as Sm3tt1bEmitter<T0, T1, T2>>::sm3tt1b(self, op0, op1, op2);
    }
    pub fn sm3tt2a<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3tt2aEmitter<T0, T1, T2>,
    {
        <Self as Sm3tt2aEmitter<T0, T1, T2>>::sm3tt2a(self, op0, op1, op2);
    }
    pub fn sm3tt2b<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm3tt2bEmitter<T0, T1, T2>,
    {
        <Self as Sm3tt2bEmitter<T0, T1, T2>>::sm3tt2b(self, op0, op1, op2);
    }
    pub fn sm4e<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Sm4eEmitter<T0, T1>,
    {
        <Self as Sm4eEmitter<T0, T1>>::sm4e(self, op0, op1);
    }
    pub fn sm4ekey<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: Sm4ekeyEmitter<T0, T1, T2>,
    {
        <Self as Sm4ekeyEmitter<T0, T1, T2>>::sm4ekey(self, op0, op1, op2);
    }
    pub fn sdot<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SdotEmitter<T0, T1, T2>,
    {
        <Self as SdotEmitter<T0, T1, T2>>::sdot(self, op0, op1, op2);
    }
    pub fn udot<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UdotEmitter<T0, T1, T2>,
    {
        <Self as UdotEmitter<T0, T1, T2>>::udot(self, op0, op1, op2);
    }
    pub fn bfcvt<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: BfcvtEmitter<T0, T1>,
    {
        <Self as BfcvtEmitter<T0, T1>>::bfcvt(self, op0, op1);
    }
    pub fn bfcvtn<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: BfcvtnEmitter<T0, T1>,
    {
        <Self as BfcvtnEmitter<T0, T1>>::bfcvtn(self, op0, op1);
    }
    pub fn bfcvtn2<T0, T1>(&mut self, op0: T0, op1: T1)
    where
        Self: Bfcvtn2Emitter<T0, T1>,
    {
        <Self as Bfcvtn2Emitter<T0, T1>>::bfcvtn2(self, op0, op1);
    }
    pub fn bfmlalb<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BfmlalbEmitter<T0, T1, T2>,
    {
        <Self as BfmlalbEmitter<T0, T1, T2>>::bfmlalb(self, op0, op1, op2);
    }
    pub fn bfmlalt<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BfmlaltEmitter<T0, T1, T2>,
    {
        <Self as BfmlaltEmitter<T0, T1, T2>>::bfmlalt(self, op0, op1, op2);
    }
    pub fn bfmmla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BfmmlaEmitter<T0, T1, T2>,
    {
        <Self as BfmmlaEmitter<T0, T1, T2>>::bfmmla(self, op0, op1, op2);
    }
    pub fn bfdot<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: BfdotEmitter<T0, T1, T2>,
    {
        <Self as BfdotEmitter<T0, T1, T2>>::bfdot(self, op0, op1, op2);
    }
    pub fn smmla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SmmlaEmitter<T0, T1, T2>,
    {
        <Self as SmmlaEmitter<T0, T1, T2>>::smmla(self, op0, op1, op2);
    }
    pub fn sudot<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: SudotEmitter<T0, T1, T2>,
    {
        <Self as SudotEmitter<T0, T1, T2>>::sudot(self, op0, op1, op2);
    }
    pub fn ummla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UmmlaEmitter<T0, T1, T2>,
    {
        <Self as UmmlaEmitter<T0, T1, T2>>::ummla(self, op0, op1, op2);
    }
    pub fn usdot<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UsdotEmitter<T0, T1, T2>,
    {
        <Self as UsdotEmitter<T0, T1, T2>>::usdot(self, op0, op1, op2);
    }
    pub fn usmmla<T0, T1, T2>(&mut self, op0: T0, op1: T1, op2: T2)
    where
        Self: UsmmlaEmitter<T0, T1, T2>,
    {
        <Self as UsmmlaEmitter<T0, T1, T2>>::usmmla(self, op0, op1, op2);
    }
}
