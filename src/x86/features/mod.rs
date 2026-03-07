use super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();
include!("387.rs");
include!("3DNOW.rs");
include!("486.rs");
include!("586.rs");
include!("686.rs");
include!("ADX.rs");
include!("AESKLE.rs");
include!("AESNI.rs");
include!("AVX2.rs");
include!("AVX512BF16.rs");
include!("AVX512BITALG.rs");
include!("AVX512BW.rs");
include!("AVX512CD.rs");
include!("AVX512DQ.rs");
include!("AVX512F.rs");
include!("AVX512IFMA.rs");
include!("AVX512VBMI2.rs");
include!("AVX512VBMI.rs");
include!("AVX512VNNI.rs");
include!("AVX512VP2INTERSECT.rs");
include!("AVX512VPOPCNTDQ.rs");
include!("AVX.rs");
include!("BMI1.rs");
include!("BMI2.rs");
include!("Base.rs");
include!("CET.rs");
include!("CLDEMOTE.rs");
include!("CLFLUSHOPT.rs");
include!("CLWB.rs");
include!("CLZERO.rs");
include!("CMOV.rs");
include!("CMPCCXADD.rs");
include!("ENQCMD.rs");
include!("FRED.rs");
include!("FSGSBASE.rs");
include!("FXSR.rs");
include!("GFNI.rs");
include!("HLERTM.rs");
include!("HRESET.rs");
include!("INVLPGB.rs");
include!("INVPCID.rs");
include!("LZCNT.rs");
include!("MCOMMIT.rs");
include!("MMX.rs");
include!("MONITOR.rs");
include!("MONITORX.rs");
include!("MOVBE.rs");
include!("MOVDIR64B.rs");
include!("MOVDIRI.rs");
include!("MSRIMM.rs");
include!("MSRLIST.rs");
include!("OSPKE.rs");
include!("PADLOCK.rs");
include!("PBNDKB.rs");
include!("PCLMULQDQ.rs");
include!("PCONFIG.rs");
include!("POPCNT.rs");
include!("PREFETCH.rs");
include!("PREFETCHI.rs");
include!("PREFETCHW.rs");
include!("PREFETCHWT1.rs");
include!("PTWRITE.rs");
include!("RDPID.rs");
include!("RDPRU.rs");
include!("RDRAND.rs");
include!("RDSEED.rs");
include!("RDTSCP.rs");
include!("RMPQUERY.rs");
include!("RMPREAD.rs");
include!("SEAM.rs");
include!("SERIALIZE.rs");
include!("SEVES.rs");
include!("SGX.rs");
include!("SHA.rs");
include!("SKINIT.rs");
include!("SMAP.rs");
include!("SMX.rs");
include!("SNP.rs");
include!("SSE2.rs");
include!("SSE3.rs");
include!("SSE41.rs");
include!("SSE42.rs");
include!("SSE4A.rs");
include!("SSE.rs");
include!("SSSE3.rs");
include!("SVM.rs");
include!("TSXLDTRK.rs");
include!("UINTR.rs");
include!("USERMSR.rs");
include!("VMX.rs");
include!("WAITPKG.rs");
include!("WBNOINVD.rs");
include!("WRMSRNS.rs");
include!("XSAVEC.rs");
include!("XSAVE.rs");
include!("XSAVEOPT.rs");
include!("XSS.rs");

impl<'a> X86387Emitter for Assembler<'a> {}
impl<'a> X863DNOWEmitter for Assembler<'a> {}
impl<'a> X86486Emitter for Assembler<'a> {}
impl<'a> X86586Emitter for Assembler<'a> {}
impl<'a> X86686Emitter for Assembler<'a> {}
impl<'a> X86ADXEmitter for Assembler<'a> {}
impl<'a> X86AESKLEEmitter for Assembler<'a> {}
impl<'a> X86AESNIEmitter for Assembler<'a> {}
impl<'a> X86AVX2Emitter for Assembler<'a> {}
impl<'a> X86AVX512BF16Emitter for Assembler<'a> {}
impl<'a> X86AVX512BITALGEmitter for Assembler<'a> {}
impl<'a> X86AVX512BWEmitter for Assembler<'a> {}
impl<'a> X86AVX512CDEmitter for Assembler<'a> {}
impl<'a> X86AVX512DQEmitter for Assembler<'a> {}
impl<'a> X86AVX512FEmitter for Assembler<'a> {}
impl<'a> X86AVX512IFMAEmitter for Assembler<'a> {}
impl<'a> X86AVX512VBMI2Emitter for Assembler<'a> {}
impl<'a> X86AVX512VBMIEmitter for Assembler<'a> {}
impl<'a> X86AVX512VNNIEmitter for Assembler<'a> {}
impl<'a> X86AVX512VP2INTERSECTEmitter for Assembler<'a> {}
impl<'a> X86AVX512VPOPCNTDQEmitter for Assembler<'a> {}
impl<'a> X86AVXEmitter for Assembler<'a> {}
impl<'a> X86BMI1Emitter for Assembler<'a> {}
impl<'a> X86BMI2Emitter for Assembler<'a> {}
impl<'a> X86BaseEmitter for Assembler<'a> {}
impl<'a> X86CETEmitter for Assembler<'a> {}
impl<'a> X86CLDEMOTEEmitter for Assembler<'a> {}
impl<'a> X86CLFLUSHOPTEmitter for Assembler<'a> {}
impl<'a> X86CLWBEmitter for Assembler<'a> {}
impl<'a> X86CLZEROEmitter for Assembler<'a> {}
impl<'a> X86CMOVEmitter for Assembler<'a> {}
impl<'a> X86CMPCCXADDEmitter for Assembler<'a> {}
impl<'a> X86ENQCMDEmitter for Assembler<'a> {}
impl<'a> X86FREDEmitter for Assembler<'a> {}
impl<'a> X86FSGSBASEEmitter for Assembler<'a> {}
impl<'a> X86FXSREmitter for Assembler<'a> {}
impl<'a> X86GFNIEmitter for Assembler<'a> {}
impl<'a> X86HLERTMEmitter for Assembler<'a> {}
impl<'a> X86HRESETEmitter for Assembler<'a> {}
impl<'a> X86INVLPGBEmitter for Assembler<'a> {}
impl<'a> X86INVPCIDEmitter for Assembler<'a> {}
impl<'a> X86LZCNTEmitter for Assembler<'a> {}
impl<'a> X86MCOMMITEmitter for Assembler<'a> {}
impl<'a> X86MMXEmitter for Assembler<'a> {}
impl<'a> X86MONITOREmitter for Assembler<'a> {}
impl<'a> X86MONITORXEmitter for Assembler<'a> {}
impl<'a> X86MOVBEEmitter for Assembler<'a> {}
impl<'a> X86MOVDIR64BEmitter for Assembler<'a> {}
impl<'a> X86MOVDIRIEmitter for Assembler<'a> {}
impl<'a> X86MSRIMMEmitter for Assembler<'a> {}
impl<'a> X86MSRLISTEmitter for Assembler<'a> {}
impl<'a> X86OSPKEEmitter for Assembler<'a> {}
impl<'a> X86PADLOCKEmitter for Assembler<'a> {}
impl<'a> X86PBNDKBEmitter for Assembler<'a> {}
impl<'a> X86PCLMULQDQEmitter for Assembler<'a> {}
impl<'a> X86PCONFIGEmitter for Assembler<'a> {}
impl<'a> X86POPCNTEmitter for Assembler<'a> {}
impl<'a> X86PREFETCHEmitter for Assembler<'a> {}
impl<'a> X86PREFETCHIEmitter for Assembler<'a> {}
impl<'a> X86PREFETCHWEmitter for Assembler<'a> {}
impl<'a> X86PREFETCHWT1Emitter for Assembler<'a> {}
impl<'a> X86PTWRITEEmitter for Assembler<'a> {}
impl<'a> X86RDPIDEmitter for Assembler<'a> {}
impl<'a> X86RDPRUEmitter for Assembler<'a> {}
impl<'a> X86RDRANDEmitter for Assembler<'a> {}
impl<'a> X86RDSEEDEmitter for Assembler<'a> {}
impl<'a> X86RDTSCPEmitter for Assembler<'a> {}
impl<'a> X86RMPQUERYEmitter for Assembler<'a> {}
impl<'a> X86RMPREADEmitter for Assembler<'a> {}
impl<'a> X86SEAMEmitter for Assembler<'a> {}
impl<'a> X86SERIALIZEEmitter for Assembler<'a> {}
impl<'a> X86SEVESEmitter for Assembler<'a> {}
impl<'a> X86SGXEmitter for Assembler<'a> {}
impl<'a> X86SHAEmitter for Assembler<'a> {}
impl<'a> X86SKINITEmitter for Assembler<'a> {}
impl<'a> X86SMAPEmitter for Assembler<'a> {}
impl<'a> X86SMXEmitter for Assembler<'a> {}
impl<'a> X86SNPEmitter for Assembler<'a> {}
impl<'a> X86SSE2Emitter for Assembler<'a> {}
impl<'a> X86SSE3Emitter for Assembler<'a> {}
impl<'a> X86SSE41Emitter for Assembler<'a> {}
impl<'a> X86SSE42Emitter for Assembler<'a> {}
impl<'a> X86SSE4AEmitter for Assembler<'a> {}
impl<'a> X86SSEEmitter for Assembler<'a> {}
impl<'a> X86SSSE3Emitter for Assembler<'a> {}
impl<'a> X86SVMEmitter for Assembler<'a> {}
impl<'a> X86TSXLDTRKEmitter for Assembler<'a> {}
impl<'a> X86UINTREmitter for Assembler<'a> {}
impl<'a> X86USERMSREmitter for Assembler<'a> {}
impl<'a> X86VMXEmitter for Assembler<'a> {}
impl<'a> X86WAITPKGEmitter for Assembler<'a> {}
impl<'a> X86WBNOINVDEmitter for Assembler<'a> {}
impl<'a> X86WRMSRNSEmitter for Assembler<'a> {}
impl<'a> X86XSAVECEmitter for Assembler<'a> {}
impl<'a> X86XSAVEEmitter for Assembler<'a> {}
impl<'a> X86XSAVEOPTEmitter for Assembler<'a> {}
impl<'a> X86XSSEmitter for Assembler<'a> {}
