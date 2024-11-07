use super::x86::*;

#[derive(Copy, Clone)]
pub enum Reg {
    X86Gp(Gp),
    X86GpLH(GpLH),
    X86GpH(GpH),
    X86Xmm(Xmm),
}
