/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

// [AsmKit] This file is a derived work: it was translated from AsmJit's generated
// instruction-database tables (asmjit/x86/x86globals.h, asmjit/x86/x86instdb.h,
// asmjit/x86/x86instdb_p.h, asmjit/x86/x86instdb.cpp, asmjit/core/cpuinfo.h)
// by meta/asmjit2rust.py. Do not edit manually; fix the translator instead.

use bitflags::bitflags;

use crate::core::rwinfo::{CpuRwFlags, InstControlFlow, InstRwFlags, InstSameRegHint, OpRwFlags};

/// X86 CPU feature identifiers (port of AsmJit's `CpuFeatures::X86`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum CpuFeature {
    /// No feature (never set, used internally).
    None,
    /// CPU has multi-threading capabilities.
    MT,
    /// CPU has Not-Execute-Bit aka DEP (data-execution prevention).
    NX,
    /// CPU has ADX              (multi-precision add-carry instruction extensions).
    ADX,
    /// CPU has LOCK MOV R<->CR0 (supports `MOV R<->CR8` via `LOCK MOV R<->CR0` in 32-bit mode) {AMD}.
    ALTMOVCR8,
    /// CPU has APX_F            (advanced performance extensions - 32 GP registers, REX2 prefix, ...) {X86_64}.
    APX_F,
    /// CPU has BMI              (bit manipulation instructions #1).
    BMI,
    /// CPU has BMI2             (bit manipulation instructions #2).
    BMI2,
    /// CPU has CET-IBT          (indirect branch tracking).
    CET_IBT,
    /// CPU has CET-SS.
    CET_SS,
    /// CPU has CET-SSS.
    CET_SSS,
    /// CPU has CLDEMOTE         (cache line demote).
    CLDEMOTE,
    /// CPU has CLFUSH           (cache Line flush).
    CLFLUSH,
    /// CPU has CLFUSHOPT        (cache Line flush - optimized).
    CLFLUSHOPT,
    /// CPU has CLWB.
    CLWB,
    /// CPU has CLZERO.
    CLZERO,
    /// CPU has CMOV             (CMOV and FCMOV instructions).
    CMOV,
    /// CPU has CMPCCXADD.
    CMPCCXADD,
    /// CPU has CMPXCHG16B       (compare-exchange 16 bytes) {X86_64}.
    CMPXCHG16B,
    /// CPU has CMPXCHG8B        (compare-exchange 8 bytes).
    CMPXCHG8B,
    /// CPU has ENCLV.
    ENCLV,
    /// CPU has ENQCMD           (enqueue stores).
    ENQCMD,
    /// CPU has ERMS             (enhanced REP MOVSB/STOSB).
    ERMS,
    /// CPU has FSGSBASE.
    FSGSBASE,
    /// CPU has FSRM             (fast short REP MOVSB).
    FSRM,
    /// CPU has FSRC             (fast short REP CMPSB|SCASB).
    FSRC,
    /// CPU has FSRS             (fast short REP STOSB)
    FSRS,
    /// CPU has FXSR             (FXSAVE/FXRSTOR instructions).
    FXSR,
    /// CPU has FXSROTP          (FXSAVE/FXRSTOR is optimized).
    FXSROPT,
    /// CPU has FZRM             (fast zero-length REP MOVSB).
    FZRM,
    /// CPU has HRESET.
    HRESET,
    /// CPU has I486 features    (I486+ support).
    I486,
    /// CPU has INVLPGB.
    INVLPGB,
    /// CPU has LAHF/SAHF        (LAHF/SAHF in 64-bit mode) {X86_64}.
    LAHFSAHF,
    /// CPU has LAM              (linear address masking) {X86_64}.
    LAM,
    /// CPU has LWP              (lightweight profiling) {AMD}.
    LWP,
    /// CPU has LZCNT            (LZCNT instruction).
    LZCNT,
    /// CPU has MCOMMIT          (MCOMMIT instruction).
    MCOMMIT,
    /// CPU has MONITOR          (MONITOR/MWAIT instructions).
    MONITOR,
    /// CPU has MONITORX         (MONITORX/MWAITX instructions).
    MONITORX,
    /// CPU has MOVBE            (move with byte-order swap).
    MOVBE,
    /// CPU has MOVDIR64B        (move 64 bytes as direct store).
    MOVDIR64B,
    /// CPU has MOVDIRI          (move dword/qword as direct store).
    MOVDIRI,
    /// CPU has MOVRS            (move from shared memory).
    MOVRS,
    /// CPU has MPX              (memory protection extensions).
    MPX,
    /// CPU has MSR              (RDMSR/WRMSR instructions).
    MSR,
    /// CPU has MSRLIST.
    MSRLIST,
    /// CPU has MSR_IMM          (RDMSR/WRMSR immediate encoding).
    MSR_IMM,
    /// CPU has MSSE             (misaligned SSE support).
    MSSE,
    /// CPU has OSXSAVE          (XSAVE enabled by OS).
    OSXSAVE,
    /// CPU has OSPKE            (PKE enabled by OS).
    OSPKE,
    /// CPU has PCONFIG          (PCONFIG instruction).
    PCONFIG,
    /// CPU has POPCNT           (POPCNT instruction).
    POPCNT,
    /// CPU has PREFETCHI.
    PREFETCHI,
    /// CPU has PREFETCHW.
    PREFETCHW,
    /// CPU has PREFETCHWT1.
    PREFETCHWT1,
    /// CPU has PTWRITE.
    PTWRITE,
    /// CPU has RAO_INT          (AADD, AAND, AOR, AXOR instructions).
    RAO_INT,
    /// CPU has RMPQUERY         (RMPQUERY instruction).
    RMPQUERY,
    /// CPU has RDPID            (RDPID instruction).
    RDPID,
    /// CPU has RDPRU            (RDPRU instruction).
    RDPRU,
    /// CPU has RDRAND           (RDRAND instruction).
    RDRAND,
    /// CPU has RDSEED           (RDSEED instruction).
    RDSEED,
    /// CPU has RDTSC.
    RDTSC,
    /// CPU has RDTSCP.
    RDTSCP,
    /// CPU has RTM              (RTM instructions - deprecated).
    RTM,
    /// CPU has SEAM.
    SEAM,
    /// CPU has SERIALIZE.
    SERIALIZE,
    /// CPU has SEV              (secure encrypted virtualization).
    SEV,
    /// CPU has SEV_ES           (SEV encrypted state).
    SEV_ES,
    /// CPU has SEV_SNP          (SEV secure nested paging).
    SEV_SNP,
    /// CPU has SKINIT           (SKINIT/STGI instructions) {AMD}.
    SKINIT,
    /// CPU has SMAP             (supervisor-mode access prevention).
    SMAP,
    /// CPU has SME              (secure memory encryption).
    SME,
    /// CPU has SMEP             (supervisor-mode execution prevention).
    SMEP,
    /// CPU has SMX              (safer mode extensions).
    SMX,
    /// CPU has SVM              (virtualization) {AMD}.
    SVM,
    /// CPU has TBM              (trailing bit manipulation) {AMD}.
    TBM,
    /// CPU has TSE.
    TSE,
    /// CPU has TSXLDTRK.
    TSXLDTRK,
    /// CPU has UINTR            (user interrupts).
    UINTR,
    /// CPU has VMX              (virtualization) {INTEL}.
    VMX,
    /// CPU has WAITPKG          (UMONITOR, UMWAIT, TPAUSE).
    WAITPKG,
    /// CPU has WBNOINVD.
    WBNOINVD,
    /// CPU has WRMSRNS.
    WRMSRNS,
    /// CPU has XSAVE.
    XSAVE,
    /// CPU has XSAVEC.
    XSAVEC,
    /// CPU has XSAVEOPT.
    XSAVEOPT,
    /// CPU has XSAVES.
    XSAVES,
    /// CPU has FPU              (FPU support).
    FPU,
    /// CPU has MMX              (MMX base instructions) (deprecated).
    MMX,
    /// CPU has MMX2             (MMX2 extensions or initial SSE extensions) (deprecated).
    MMX2,
    /// CPU has 3DNOW            (3DNOW base instructions) {AMD} (deprecated).
    _3DNOW,
    /// CPU has 3DNOW2           (enhanced 3DNOW) {AMD} (deprecated).
    _3DNOW2,
    /// CPU has GEODE extensions (GEODE 3DNOW additions) (deprecated).
    GEODE,
    /// CPU has SSE              (SSE instructions).
    SSE,
    /// CPU has SSE2             (SSE2 instructions).
    SSE2,
    /// CPU has SSE3             (SSE3 instructions).
    SSE3,
    /// CPU has SSSE3            (SSSE3 instructions).
    SSSE3,
    /// CPU has SSE4.1           (SSE4.1 instructions).
    SSE4_1,
    /// CPU has SSE4.2           (SSE4.2 instructions).
    SSE4_2,
    /// CPU has SSE4A            (SSE4.A instructions) {AMD} (deprecated).
    SSE4A,
    /// CPU has PCLMULQDQ        (packed carry-less multiplication).
    PCLMULQDQ,
    /// CPU has AVX              (advanced vector extensions).
    AVX,
    /// CPU has AVX2             (advanced vector extensions 2).
    AVX2,
    /// CPU has AVX_IFMA         (AVX/VEX encoding of vpmadd52huq/vpmadd52luq).
    AVX_IFMA,
    /// CPU has AVX_NE_CONVERT.
    AVX_NE_CONVERT,
    /// CPU has AVX_VNNI         (AVX/VEX encoding of vpdpbusd/vpdpbusds/vpdpwssd/vpdpwssds).
    AVX_VNNI,
    /// CPU has AVX_VNNI_INT16.
    AVX_VNNI_INT16,
    /// CPU has AVX_VNNI_INT8.
    AVX_VNNI_INT8,
    /// CPU has F16C             (AVX FP16 conversion instructions).
    F16C,
    /// CPU has FMA              (AVX fused-multiply-add - 3 operand form).
    FMA,
    /// CPU has FMA4             (AVX fused-multiply-add - 4 operand form) (deprecated).
    FMA4,
    /// CPU has XOP              (XOP instructions) {AMD} (deprecated).
    XOP,
    /// CPU has AVX512_BF16      (AVX512 BFLOAT16 support instructions).
    AVX512_BF16,
    /// CPU has AVX512_BITALG    (AVX512 VPOPCNT[B|W] and VPSHUFBITQMB instructions).
    AVX512_BITALG,
    /// CPU has AVX512_BW        (AVX512 integer BYTE|WORD instructions).
    AVX512_BW,
    /// CPU has AVX512_CD        (AVX512 conflict detection DWORD|QWORD instructions).
    AVX512_CD,
    /// CPU has AVX512_DQ        (AVX512 integer DWORD|QWORD instructions).
    AVX512_DQ,
    /// CPU has AVX512_F         (AVX512 foundation).
    AVX512_F,
    /// CPU has AVX512_FP16      (AVX512 FP16 instructions).
    AVX512_FP16,
    /// CPU has AVX512_IFMA      (AVX512 integer fused-multiply-add using 52-bit precision).
    AVX512_IFMA,
    /// CPU has AVX512_VBMI      (AVX512 vector byte manipulation instructions).
    AVX512_VBMI,
    /// CPU has AVX512_VBMI2     (AVX512 vector byte manipulation instructions v2).
    AVX512_VBMI2,
    /// CPU has AVX512_VL        (AVX512 vector length extensions).
    AVX512_VL,
    /// CPU has AVX512_VNNI      (AVX512 vector neural network instructions).
    AVX512_VNNI,
    /// CPU has AVX512_VP2INTERSECT
    AVX512_VP2INTERSECT,
    /// CPU has AVX512_VPOPCNTDQ (AVX512 VPOPCNT[D|Q] instructions).
    AVX512_VPOPCNTDQ,
    /// CPU has AESNI            (AES encode/decode instructions).
    AESNI,
    /// CPU has GFNI             (galois field instructions).
    GFNI,
    /// CPU has SHA              (SHA-1 and SHA-256 instructions).
    SHA,
    /// CPU has SHA512           (SHA-512 instructions).
    SHA512,
    /// CPU has SM3              (SM3 hash extensions).
    SM3,
    /// CPU has SM4              (SM4 cipher extensions).
    SM4,
    /// CPU has VAES             (vector AES 256|512 bit support).
    VAES,
    /// CPU has VPCLMULQDQ       (vector PCLMULQDQ 256|512-bit support).
    VPCLMULQDQ,
    /// CPU has KL               (Key Locker).
    KL,
    /// CPU has AESKLE           (AESKLE).
    AESKLE,
    /// CPU has AESKLE+WIDEKL+KL (AESKLE & WIDEKL instructions and KL enabled)
    AESKLEWIDE_KL,
    /// CPU has AVX10.1/512      (AVX10.1 with 512-bit vectors).
    AVX10_1,
    /// CPU has AVX10.2/512      (AVX10.2 with 512-bit vectors).
    AVX10_2,
    /// CPU has AMX_AVX512       (AMX-AVX512 instructions).
    AMX_AVX512,
    /// CPU has AMX_BF16         (AMX-BF16 instructions).
    AMX_BF16,
    /// CPU has AMX_COMPLEX      (AMX-COMPLEX instructions).
    AMX_COMPLEX,
    /// CPU has AMX_FP16         (AMX-FP16 instructions).
    AMX_FP16,
    /// CPU has AMX_FP8          (AMX-FP8 instructions).
    AMX_FP8,
    /// CPU has AMX_INT8         (AMX-INT8 instructions).
    AMX_INT8,
    /// CPU has AMX_MOVRS        (AMX-MOVRS instructions).
    AMX_MOVRS,
    /// CPU has AMX_TF32         (AMX-TF32 instructions).
    AMX_TF32,
    /// CPU has AMX_TILE         (advanced matrix extensions).
    AMX_TILE,
    /// CPU has AMX_TRANSPOSE    (AMX-TRANSPOSE instructions).
    AMX_TRANSPOSE,
}

pub const CPU_FEATURE_COUNT: usize = 151;
pub static CPU_FEATURE_NAMES: &[&str] = &["None", "MT", "NX", "ADX", "ALTMOVCR8", "APX_F", "BMI", "BMI2", "CET_IBT", "CET_SS", "CET_SSS", "CLDEMOTE", "CLFLUSH", "CLFLUSHOPT", "CLWB", "CLZERO", "CMOV", "CMPCCXADD", "CMPXCHG16B", "CMPXCHG8B", "ENCLV", "ENQCMD", "ERMS", "FSGSBASE", "FSRM", "FSRC", "FSRS", "FXSR", "FXSROPT", "FZRM", "HRESET", "I486", "INVLPGB", "LAHFSAHF", "LAM", "LWP", "LZCNT", "MCOMMIT", "MONITOR", "MONITORX", "MOVBE", "MOVDIR64B", "MOVDIRI", "MOVRS", "MPX", "MSR", "MSRLIST", "MSR_IMM", "MSSE", "OSXSAVE", "OSPKE", "PCONFIG", "POPCNT", "PREFETCHI", "PREFETCHW", "PREFETCHWT1", "PTWRITE", "RAO_INT", "RMPQUERY", "RDPID", "RDPRU", "RDRAND", "RDSEED", "RDTSC", "RDTSCP", "RTM", "SEAM", "SERIALIZE", "SEV", "SEV_ES", "SEV_SNP", "SKINIT", "SMAP", "SME", "SMEP", "SMX", "SVM", "TBM", "TSE", "TSXLDTRK", "UINTR", "VMX", "WAITPKG", "WBNOINVD", "WRMSRNS", "XSAVE", "XSAVEC", "XSAVEOPT", "XSAVES", "FPU", "MMX", "MMX2", "_3DNOW", "_3DNOW2", "GEODE", "SSE", "SSE2", "SSE3", "SSSE3", "SSE4_1", "SSE4_2", "SSE4A", "PCLMULQDQ", "AVX", "AVX2", "AVX_IFMA", "AVX_NE_CONVERT", "AVX_VNNI", "AVX_VNNI_INT16", "AVX_VNNI_INT8", "F16C", "FMA", "FMA4", "XOP", "AVX512_BF16", "AVX512_BITALG", "AVX512_BW", "AVX512_CD", "AVX512_DQ", "AVX512_F", "AVX512_FP16", "AVX512_IFMA", "AVX512_VBMI", "AVX512_VBMI2", "AVX512_VL", "AVX512_VNNI", "AVX512_VP2INTERSECT", "AVX512_VPOPCNTDQ", "AESNI", "GFNI", "SHA", "SHA512", "SM3", "SM4", "VAES", "VPCLMULQDQ", "KL", "AESKLE", "AESKLEWIDE_KL", "AVX10_1", "AVX10_2", "AMX_AVX512", "AMX_BF16", "AMX_COMPLEX", "AMX_FP16", "AMX_FP8", "AMX_INT8", "AMX_MOVRS", "AMX_TF32", "AMX_TILE", "AMX_TRANSPOSE"];
pub const DEFAULT_X86_FEATURES: &[CpuFeature] = &[CpuFeature::MT, CpuFeature::NX, CpuFeature::ADX, CpuFeature::ALTMOVCR8, CpuFeature::APX_F, CpuFeature::BMI, CpuFeature::BMI2, CpuFeature::CET_IBT, CpuFeature::CET_SS, CpuFeature::CET_SSS, CpuFeature::CLDEMOTE, CpuFeature::CLFLUSH, CpuFeature::CLFLUSHOPT, CpuFeature::CLWB, CpuFeature::CLZERO, CpuFeature::CMOV, CpuFeature::CMPCCXADD, CpuFeature::CMPXCHG16B, CpuFeature::CMPXCHG8B, CpuFeature::ENCLV, CpuFeature::ENQCMD, CpuFeature::ERMS, CpuFeature::FSGSBASE, CpuFeature::FSRM, CpuFeature::FSRC, CpuFeature::FSRS, CpuFeature::FXSR, CpuFeature::FXSROPT, CpuFeature::FZRM, CpuFeature::HRESET, CpuFeature::I486, CpuFeature::INVLPGB, CpuFeature::LAHFSAHF, CpuFeature::LAM, CpuFeature::LWP, CpuFeature::LZCNT, CpuFeature::MCOMMIT, CpuFeature::MONITOR, CpuFeature::MONITORX, CpuFeature::MOVBE, CpuFeature::MOVDIR64B, CpuFeature::MOVDIRI, CpuFeature::MOVRS, CpuFeature::MPX, CpuFeature::MSR, CpuFeature::MSRLIST, CpuFeature::MSR_IMM, CpuFeature::MSSE, CpuFeature::OSXSAVE, CpuFeature::OSPKE, CpuFeature::PCONFIG, CpuFeature::POPCNT, CpuFeature::PREFETCHI, CpuFeature::PREFETCHW, CpuFeature::PREFETCHWT1, CpuFeature::PTWRITE, CpuFeature::RAO_INT, CpuFeature::RMPQUERY, CpuFeature::RDPID, CpuFeature::RDPRU, CpuFeature::RDRAND, CpuFeature::RDSEED, CpuFeature::RDTSC, CpuFeature::RDTSCP, CpuFeature::RTM, CpuFeature::SEAM, CpuFeature::SERIALIZE, CpuFeature::SEV, CpuFeature::SEV_ES, CpuFeature::SEV_SNP, CpuFeature::SKINIT, CpuFeature::SMAP, CpuFeature::SME, CpuFeature::SMEP, CpuFeature::SMX, CpuFeature::SVM, CpuFeature::TBM, CpuFeature::TSE, CpuFeature::TSXLDTRK, CpuFeature::UINTR, CpuFeature::VMX, CpuFeature::WAITPKG, CpuFeature::WBNOINVD, CpuFeature::WRMSRNS, CpuFeature::XSAVE, CpuFeature::XSAVEC, CpuFeature::XSAVEOPT, CpuFeature::XSAVES, CpuFeature::FPU, CpuFeature::MMX, CpuFeature::MMX2, CpuFeature::_3DNOW, CpuFeature::_3DNOW2, CpuFeature::GEODE, CpuFeature::SSE, CpuFeature::SSE2, CpuFeature::SSE3, CpuFeature::SSSE3, CpuFeature::SSE4_1, CpuFeature::SSE4_2, CpuFeature::SSE4A, CpuFeature::PCLMULQDQ, CpuFeature::AVX, CpuFeature::AVX2, CpuFeature::AVX_IFMA, CpuFeature::AVX_NE_CONVERT, CpuFeature::AVX_VNNI, CpuFeature::AVX_VNNI_INT16, CpuFeature::AVX_VNNI_INT8, CpuFeature::F16C, CpuFeature::FMA, CpuFeature::FMA4, CpuFeature::XOP, CpuFeature::AVX512_BF16, CpuFeature::AVX512_BITALG, CpuFeature::AVX512_BW, CpuFeature::AVX512_CD, CpuFeature::AVX512_DQ, CpuFeature::AVX512_F, CpuFeature::AVX512_FP16, CpuFeature::AVX512_IFMA, CpuFeature::AVX512_VBMI, CpuFeature::AVX512_VBMI2, CpuFeature::AVX512_VL, CpuFeature::AVX512_VNNI, CpuFeature::AVX512_VP2INTERSECT, CpuFeature::AVX512_VPOPCNTDQ, CpuFeature::AESNI, CpuFeature::GFNI, CpuFeature::SHA, CpuFeature::SHA512, CpuFeature::SM3, CpuFeature::SM4, CpuFeature::VAES, CpuFeature::VPCLMULQDQ, CpuFeature::KL, CpuFeature::AESKLE, CpuFeature::AESKLEWIDE_KL, CpuFeature::AVX10_1, CpuFeature::AVX10_2, CpuFeature::AMX_AVX512, CpuFeature::AMX_BF16, CpuFeature::AMX_COMPLEX, CpuFeature::AMX_FP16, CpuFeature::AMX_FP8, CpuFeature::AMX_INT8, CpuFeature::AMX_MOVRS, CpuFeature::AMX_TF32, CpuFeature::AMX_TILE, CpuFeature::AMX_TRANSPOSE];

impl CpuFeature {
    pub const AVX512F: Self = Self::AVX512_F;
    pub const AVX512BW: Self = Self::AVX512_BW;
    pub const AVX512CD: Self = Self::AVX512_CD;
    pub const AVX512DQ: Self = Self::AVX512_DQ;
    pub const AVX512VL: Self = Self::AVX512_VL;
}

/// X86 instruction id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InstId {
    /// Invalid instruction id.
    None = 0,
    /// Instruction 'aaa' (X86).
    Aaa,
    /// Instruction 'aad' (X86).
    Aad,
    /// Instruction 'aadd' {RAO_INT}.
    Aadd,
    /// Instruction 'aam' (X86).
    Aam,
    /// Instruction 'aand' {RAO_INT}.
    Aand,
    /// Instruction 'aas' (X86).
    Aas,
    /// Instruction 'adc'.
    Adc,
    /// Instruction 'adcx' {ADX}.
    Adcx,
    /// Instruction 'add'.
    Add,
    /// Instruction 'addpd' {SSE2}.
    Addpd,
    /// Instruction 'addps' {SSE}.
    Addps,
    /// Instruction 'addsd' {SSE2}.
    Addsd,
    /// Instruction 'addss' {SSE}.
    Addss,
    /// Instruction 'addsubpd' {SSE3}.
    Addsubpd,
    /// Instruction 'addsubps' {SSE3}.
    Addsubps,
    /// Instruction 'adox' {ADX}.
    Adox,
    /// Instruction 'aesdec' {AESNI}.
    Aesdec,
    /// Instruction 'aesdeclast' {AESNI}.
    Aesdeclast,
    /// Instruction 'aesenc' {AESNI}.
    Aesenc,
    /// Instruction 'aesenclast' {AESNI}.
    Aesenclast,
    /// Instruction 'aesimc' {AESNI}.
    Aesimc,
    /// Instruction 'aeskeygenassist' {AESNI}.
    Aeskeygenassist,
    /// Instruction 'and'.
    And,
    /// Instruction 'andn' {BMI}.
    Andn,
    /// Instruction 'andnpd' {SSE2}.
    Andnpd,
    /// Instruction 'andnps' {SSE}.
    Andnps,
    /// Instruction 'andpd' {SSE2}.
    Andpd,
    /// Instruction 'andps' {SSE}.
    Andps,
    /// Instruction 'aor' {RAO_INT}.
    Aor,
    /// Instruction 'arpl' (X86).
    Arpl,
    /// Instruction 'axor' {RAO_INT}.
    Axor,
    /// Instruction 'bextr' {BMI}.
    Bextr,
    /// Instruction 'blcfill' {TBM}.
    Blcfill,
    /// Instruction 'blci' {TBM}.
    Blci,
    /// Instruction 'blcic' {TBM}.
    Blcic,
    /// Instruction 'blcmsk' {TBM}.
    Blcmsk,
    /// Instruction 'blcs' {TBM}.
    Blcs,
    /// Instruction 'blendpd' {SSE4_1}.
    Blendpd,
    /// Instruction 'blendps' {SSE4_1}.
    Blendps,
    /// Instruction 'blendvpd' {SSE4_1}.
    Blendvpd,
    /// Instruction 'blendvps' {SSE4_1}.
    Blendvps,
    /// Instruction 'blsfill' {TBM}.
    Blsfill,
    /// Instruction 'blsi' {BMI}.
    Blsi,
    /// Instruction 'blsic' {TBM}.
    Blsic,
    /// Instruction 'blsmsk' {BMI}.
    Blsmsk,
    /// Instruction 'blsr' {BMI}.
    Blsr,
    /// Instruction 'bndcl' {MPX}.
    Bndcl,
    /// Instruction 'bndcn' {MPX}.
    Bndcn,
    /// Instruction 'bndcu' {MPX}.
    Bndcu,
    /// Instruction 'bndldx' {MPX}.
    Bndldx,
    /// Instruction 'bndmk' {MPX}.
    Bndmk,
    /// Instruction 'bndmov' {MPX}.
    Bndmov,
    /// Instruction 'bndstx' {MPX}.
    Bndstx,
    /// Instruction 'bound' (X86).
    Bound,
    /// Instruction 'bsf'.
    Bsf,
    /// Instruction 'bsr'.
    Bsr,
    /// Instruction 'bswap'.
    Bswap,
    /// Instruction 'bt'.
    Bt,
    /// Instruction 'btc'.
    Btc,
    /// Instruction 'btr'.
    Btr,
    /// Instruction 'bts'.
    Bts,
    /// Instruction 'bzhi' {BMI2}.
    Bzhi,
    /// Instruction 'call'.
    Call,
    /// Instruction 'cbw'.
    Cbw,
    /// Instruction 'cdq'.
    Cdq,
    /// Instruction 'cdqe' (X64).
    Cdqe,
    /// Instruction 'clac' {SMAP}.
    Clac,
    /// Instruction 'clc'.
    Clc,
    /// Instruction 'cld'.
    Cld,
    /// Instruction 'cldemote' {CLDEMOTE}.
    Cldemote,
    /// Instruction 'clflush' {CLFLUSH}.
    Clflush,
    /// Instruction 'clflushopt' {CLFLUSHOPT}.
    Clflushopt,
    /// Instruction 'clgi' {SVM}.
    Clgi,
    /// Instruction 'cli'.
    Cli,
    /// Instruction 'clrssbsy' {CET_SS}.
    Clrssbsy,
    /// Instruction 'clts'.
    Clts,
    /// Instruction 'clui' {UINTR} (X64).
    Clui,
    /// Instruction 'clwb' {CLWB}.
    Clwb,
    /// Instruction 'clzero' {CLZERO}.
    Clzero,
    /// Instruction 'cmc'.
    Cmc,
    /// Instruction 'cmovb' {CMOV}.
    Cmovb,
    /// Instruction 'cmovbe' {CMOV}.
    Cmovbe,
    /// Instruction 'cmovl' {CMOV}.
    Cmovl,
    /// Instruction 'cmovle' {CMOV}.
    Cmovle,
    /// Instruction 'cmovnb' {CMOV}.
    Cmovnb,
    /// Instruction 'cmovnbe' {CMOV}.
    Cmovnbe,
    /// Instruction 'cmovnl' {CMOV}.
    Cmovnl,
    /// Instruction 'cmovnle' {CMOV}.
    Cmovnle,
    /// Instruction 'cmovno' {CMOV}.
    Cmovno,
    /// Instruction 'cmovnp' {CMOV}.
    Cmovnp,
    /// Instruction 'cmovns' {CMOV}.
    Cmovns,
    /// Instruction 'cmovnz' {CMOV}.
    Cmovnz,
    /// Instruction 'cmovo' {CMOV}.
    Cmovo,
    /// Instruction 'cmovp' {CMOV}.
    Cmovp,
    /// Instruction 'cmovs' {CMOV}.
    Cmovs,
    /// Instruction 'cmovz' {CMOV}.
    Cmovz,
    /// Instruction 'cmp'.
    Cmp,
    /// Instruction 'cmpbexadd' {CMPCCXADD}.
    Cmpbexadd,
    /// Instruction 'cmpbxadd' {CMPCCXADD}.
    Cmpbxadd,
    /// Instruction 'cmplexadd' {CMPCCXADD}.
    Cmplexadd,
    /// Instruction 'cmplxadd' {CMPCCXADD}.
    Cmplxadd,
    /// Instruction 'cmpnbexadd' {CMPCCXADD}.
    Cmpnbexadd,
    /// Instruction 'cmpnbxadd' {CMPCCXADD}.
    Cmpnbxadd,
    /// Instruction 'cmpnlexadd' {CMPCCXADD}.
    Cmpnlexadd,
    /// Instruction 'cmpnlxadd' {CMPCCXADD}.
    Cmpnlxadd,
    /// Instruction 'cmpnoxadd' {CMPCCXADD}.
    Cmpnoxadd,
    /// Instruction 'cmpnpxadd' {CMPCCXADD}.
    Cmpnpxadd,
    /// Instruction 'cmpnsxadd' {CMPCCXADD}.
    Cmpnsxadd,
    /// Instruction 'cmpnzxadd' {CMPCCXADD}.
    Cmpnzxadd,
    /// Instruction 'cmpoxadd' {CMPCCXADD}.
    Cmpoxadd,
    /// Instruction 'cmppd' {SSE2}.
    Cmppd,
    /// Instruction 'cmpps' {SSE}.
    Cmpps,
    /// Instruction 'cmppxadd' {CMPCCXADD}.
    Cmppxadd,
    /// Instruction 'cmps'.
    Cmps,
    /// Instruction 'cmpsd' {SSE2}.
    Cmpsd,
    /// Instruction 'cmpss' {SSE}.
    Cmpss,
    /// Instruction 'cmpsxadd' {CMPCCXADD}.
    Cmpsxadd,
    /// Instruction 'cmpxchg' {I486}.
    Cmpxchg,
    /// Instruction 'cmpxchg16b' {CMPXCHG16B} (X64).
    Cmpxchg16b,
    /// Instruction 'cmpxchg8b' {CMPXCHG8B}.
    Cmpxchg8b,
    /// Instruction 'cmpzxadd' {CMPCCXADD}.
    Cmpzxadd,
    /// Instruction 'comisd' {SSE2}.
    Comisd,
    /// Instruction 'comiss' {SSE}.
    Comiss,
    /// Instruction 'cpuid' {I486}.
    Cpuid,
    /// Instruction 'cqo' (X64).
    Cqo,
    /// Instruction 'crc32' {SSE4_2}.
    Crc32,
    /// Instruction 'cvtdq2pd' {SSE2}.
    Cvtdq2pd,
    /// Instruction 'cvtdq2ps' {SSE2}.
    Cvtdq2ps,
    /// Instruction 'cvtpd2dq' {SSE2}.
    Cvtpd2dq,
    /// Instruction 'cvtpd2pi' {SSE2}.
    Cvtpd2pi,
    /// Instruction 'cvtpd2ps' {SSE2}.
    Cvtpd2ps,
    /// Instruction 'cvtpi2pd' {SSE2}.
    Cvtpi2pd,
    /// Instruction 'cvtpi2ps' {SSE}.
    Cvtpi2ps,
    /// Instruction 'cvtps2dq' {SSE2}.
    Cvtps2dq,
    /// Instruction 'cvtps2pd' {SSE2}.
    Cvtps2pd,
    /// Instruction 'cvtps2pi' {SSE}.
    Cvtps2pi,
    /// Instruction 'cvtsd2si' {SSE2}.
    Cvtsd2si,
    /// Instruction 'cvtsd2ss' {SSE2}.
    Cvtsd2ss,
    /// Instruction 'cvtsi2sd' {SSE2}.
    Cvtsi2sd,
    /// Instruction 'cvtsi2ss' {SSE}.
    Cvtsi2ss,
    /// Instruction 'cvtss2sd' {SSE2}.
    Cvtss2sd,
    /// Instruction 'cvtss2si' {SSE}.
    Cvtss2si,
    /// Instruction 'cvttpd2dq' {SSE2}.
    Cvttpd2dq,
    /// Instruction 'cvttpd2pi' {SSE2}.
    Cvttpd2pi,
    /// Instruction 'cvttps2dq' {SSE2}.
    Cvttps2dq,
    /// Instruction 'cvttps2pi' {SSE}.
    Cvttps2pi,
    /// Instruction 'cvttsd2si' {SSE2}.
    Cvttsd2si,
    /// Instruction 'cvttss2si' {SSE}.
    Cvttss2si,
    /// Instruction 'cwd'.
    Cwd,
    /// Instruction 'cwde'.
    Cwde,
    /// Instruction 'daa' (X86).
    Daa,
    /// Instruction 'das' (X86).
    Das,
    /// Instruction 'dec'.
    Dec,
    /// Instruction 'div'.
    Div,
    /// Instruction 'divpd' {SSE2}.
    Divpd,
    /// Instruction 'divps' {SSE}.
    Divps,
    /// Instruction 'divsd' {SSE2}.
    Divsd,
    /// Instruction 'divss' {SSE}.
    Divss,
    /// Instruction 'dppd' {SSE4_1}.
    Dppd,
    /// Instruction 'dpps' {SSE4_1}.
    Dpps,
    /// Instruction 'emms' {MMX}.
    Emms,
    /// Instruction 'endbr32' {CET_IBT}.
    Endbr32,
    /// Instruction 'endbr64' {CET_IBT}.
    Endbr64,
    /// Instruction 'enqcmd' {ENQCMD}.
    Enqcmd,
    /// Instruction 'enqcmds' {ENQCMD}.
    Enqcmds,
    /// Instruction 'enter'.
    Enter,
    /// Instruction 'extractps' {SSE4_1}.
    Extractps,
    /// Instruction 'extrq' {SSE4A}.
    Extrq,
    /// Instruction 'f2xm1' {FPU}.
    F2xm1,
    /// Instruction 'fabs' {FPU}.
    Fabs,
    /// Instruction 'fadd' {FPU}.
    Fadd,
    /// Instruction 'faddp' {FPU}.
    Faddp,
    /// Instruction 'fbld' {FPU}.
    Fbld,
    /// Instruction 'fbstp' {FPU}.
    Fbstp,
    /// Instruction 'fchs' {FPU}.
    Fchs,
    /// Instruction 'fclex' {FPU}.
    Fclex,
    /// Instruction 'fcmovb' {CMOV|FPU}.
    Fcmovb,
    /// Instruction 'fcmovbe' {CMOV|FPU}.
    Fcmovbe,
    /// Instruction 'fcmove' {CMOV|FPU}.
    Fcmove,
    /// Instruction 'fcmovnb' {CMOV|FPU}.
    Fcmovnb,
    /// Instruction 'fcmovnbe' {CMOV|FPU}.
    Fcmovnbe,
    /// Instruction 'fcmovne' {CMOV|FPU}.
    Fcmovne,
    /// Instruction 'fcmovnu' {CMOV|FPU}.
    Fcmovnu,
    /// Instruction 'fcmovu' {CMOV|FPU}.
    Fcmovu,
    /// Instruction 'fcom' {FPU}.
    Fcom,
    /// Instruction 'fcomi' {FPU}.
    Fcomi,
    /// Instruction 'fcomip' {FPU}.
    Fcomip,
    /// Instruction 'fcomp' {FPU}.
    Fcomp,
    /// Instruction 'fcompp' {FPU}.
    Fcompp,
    /// Instruction 'fcos' {FPU}.
    Fcos,
    /// Instruction 'fdecstp' {FPU}.
    Fdecstp,
    /// Instruction 'fdiv' {FPU}.
    Fdiv,
    /// Instruction 'fdivp' {FPU}.
    Fdivp,
    /// Instruction 'fdivr' {FPU}.
    Fdivr,
    /// Instruction 'fdivrp' {FPU}.
    Fdivrp,
    /// Instruction 'femms' {3DNOW}.
    Femms,
    /// Instruction 'ffree' {FPU}.
    Ffree,
    /// Instruction 'fiadd' {FPU}.
    Fiadd,
    /// Instruction 'ficom' {FPU}.
    Ficom,
    /// Instruction 'ficomp' {FPU}.
    Ficomp,
    /// Instruction 'fidiv' {FPU}.
    Fidiv,
    /// Instruction 'fidivr' {FPU}.
    Fidivr,
    /// Instruction 'fild' {FPU}.
    Fild,
    /// Instruction 'fimul' {FPU}.
    Fimul,
    /// Instruction 'fincstp' {FPU}.
    Fincstp,
    /// Instruction 'finit' {FPU}.
    Finit,
    /// Instruction 'fist' {FPU}.
    Fist,
    /// Instruction 'fistp' {FPU}.
    Fistp,
    /// Instruction 'fisttp' {SSE3|FPU}.
    Fisttp,
    /// Instruction 'fisub' {FPU}.
    Fisub,
    /// Instruction 'fisubr' {FPU}.
    Fisubr,
    /// Instruction 'fld' {FPU}.
    Fld,
    /// Instruction 'fld1' {FPU}.
    Fld1,
    /// Instruction 'fldcw' {FPU}.
    Fldcw,
    /// Instruction 'fldenv' {FPU}.
    Fldenv,
    /// Instruction 'fldl2e' {FPU}.
    Fldl2e,
    /// Instruction 'fldl2t' {FPU}.
    Fldl2t,
    /// Instruction 'fldlg2' {FPU}.
    Fldlg2,
    /// Instruction 'fldln2' {FPU}.
    Fldln2,
    /// Instruction 'fldpi' {FPU}.
    Fldpi,
    /// Instruction 'fldz' {FPU}.
    Fldz,
    /// Instruction 'fmul' {FPU}.
    Fmul,
    /// Instruction 'fmulp' {FPU}.
    Fmulp,
    /// Instruction 'fnclex' {FPU}.
    Fnclex,
    /// Instruction 'fninit' {FPU}.
    Fninit,
    /// Instruction 'fnop' {FPU}.
    Fnop,
    /// Instruction 'fnsave' {FPU}.
    Fnsave,
    /// Instruction 'fnstcw' {FPU}.
    Fnstcw,
    /// Instruction 'fnstenv' {FPU}.
    Fnstenv,
    /// Instruction 'fnstsw' {FPU}.
    Fnstsw,
    /// Instruction 'fpatan' {FPU}.
    Fpatan,
    /// Instruction 'fprem' {FPU}.
    Fprem,
    /// Instruction 'fprem1' {FPU}.
    Fprem1,
    /// Instruction 'fptan' {FPU}.
    Fptan,
    /// Instruction 'frndint' {FPU}.
    Frndint,
    /// Instruction 'frstor' {FPU}.
    Frstor,
    /// Instruction 'fsave' {FPU}.
    Fsave,
    /// Instruction 'fscale' {FPU}.
    Fscale,
    /// Instruction 'fsin' {FPU}.
    Fsin,
    /// Instruction 'fsincos' {FPU}.
    Fsincos,
    /// Instruction 'fsqrt' {FPU}.
    Fsqrt,
    /// Instruction 'fst' {FPU}.
    Fst,
    /// Instruction 'fstcw' {FPU}.
    Fstcw,
    /// Instruction 'fstenv' {FPU}.
    Fstenv,
    /// Instruction 'fstp' {FPU}.
    Fstp,
    /// Instruction 'fstsw' {FPU}.
    Fstsw,
    /// Instruction 'fsub' {FPU}.
    Fsub,
    /// Instruction 'fsubp' {FPU}.
    Fsubp,
    /// Instruction 'fsubr' {FPU}.
    Fsubr,
    /// Instruction 'fsubrp' {FPU}.
    Fsubrp,
    /// Instruction 'ftst' {FPU}.
    Ftst,
    /// Instruction 'fucom' {FPU}.
    Fucom,
    /// Instruction 'fucomi' {FPU}.
    Fucomi,
    /// Instruction 'fucomip' {FPU}.
    Fucomip,
    /// Instruction 'fucomp' {FPU}.
    Fucomp,
    /// Instruction 'fucompp' {FPU}.
    Fucompp,
    /// Instruction 'fwait' {FPU}.
    Fwait,
    /// Instruction 'fxam' {FPU}.
    Fxam,
    /// Instruction 'fxch' {FPU}.
    Fxch,
    /// Instruction 'fxrstor' {FXSR}.
    Fxrstor,
    /// Instruction 'fxrstor64' {FXSR} (X64).
    Fxrstor64,
    /// Instruction 'fxsave' {FXSR}.
    Fxsave,
    /// Instruction 'fxsave64' {FXSR} (X64).
    Fxsave64,
    /// Instruction 'fxtract' {FPU}.
    Fxtract,
    /// Instruction 'fyl2x' {FPU}.
    Fyl2x,
    /// Instruction 'fyl2xp1' {FPU}.
    Fyl2xp1,
    /// Instruction 'getsec' {SMX}.
    Getsec,
    /// Instruction 'gf2p8affineinvqb' {GFNI}.
    Gf2p8affineinvqb,
    /// Instruction 'gf2p8affineqb' {GFNI}.
    Gf2p8affineqb,
    /// Instruction 'gf2p8mulb' {GFNI}.
    Gf2p8mulb,
    /// Instruction 'haddpd' {SSE3}.
    Haddpd,
    /// Instruction 'haddps' {SSE3}.
    Haddps,
    /// Instruction 'hlt'.
    Hlt,
    /// Instruction 'hreset' {HRESET}.
    Hreset,
    /// Instruction 'hsubpd' {SSE3}.
    Hsubpd,
    /// Instruction 'hsubps' {SSE3}.
    Hsubps,
    /// Instruction 'idiv'.
    Idiv,
    /// Instruction 'imul'.
    Imul,
    /// Instruction 'in'.
    In,
    /// Instruction 'inc'.
    Inc,
    /// Instruction 'incsspd' {CET_SS}.
    Incsspd,
    /// Instruction 'incsspq' {CET_SS} (X64).
    Incsspq,
    /// Instruction 'ins'.
    Ins,
    /// Instruction 'insertps' {SSE4_1}.
    Insertps,
    /// Instruction 'insertq' {SSE4A}.
    Insertq,
    /// Instruction 'int'.
    Int,
    /// Instruction 'int3'.
    Int3,
    /// Instruction 'into' (X86).
    Into,
    /// Instruction 'invd' {I486}.
    Invd,
    /// Instruction 'invept' {VMX}.
    Invept,
    /// Instruction 'invlpg' {I486}.
    Invlpg,
    /// Instruction 'invlpga' {SVM}.
    Invlpga,
    /// Instruction 'invlpgb' {INVLPGB}.
    Invlpgb,
    /// Instruction 'invpcid' {I486}.
    Invpcid,
    /// Instruction 'invvpid' {VMX}.
    Invvpid,
    /// Instruction 'iret'.
    Iret,
    /// Instruction 'iretd'.
    Iretd,
    /// Instruction 'iretq' (X64).
    Iretq,
    /// Instruction 'jb'.
    Jb,
    /// Instruction 'jbe'.
    Jbe,
    /// Instruction 'jecxz'.
    Jecxz,
    /// Instruction 'jl'.
    Jl,
    /// Instruction 'jle'.
    Jle,
    /// Instruction 'jmp'.
    Jmp,
    /// Instruction 'jnb'.
    Jnb,
    /// Instruction 'jnbe'.
    Jnbe,
    /// Instruction 'jnl'.
    Jnl,
    /// Instruction 'jnle'.
    Jnle,
    /// Instruction 'jno'.
    Jno,
    /// Instruction 'jnp'.
    Jnp,
    /// Instruction 'jns'.
    Jns,
    /// Instruction 'jnz'.
    Jnz,
    /// Instruction 'jo'.
    Jo,
    /// Instruction 'jp'.
    Jp,
    /// Instruction 'js'.
    Js,
    /// Instruction 'jz'.
    Jz,
    /// Instruction 'kaddb' {AVX512_DQ}.
    Kaddb,
    /// Instruction 'kaddd' {AVX512_BW}.
    Kaddd,
    /// Instruction 'kaddq' {AVX512_BW}.
    Kaddq,
    /// Instruction 'kaddw' {AVX512_DQ}.
    Kaddw,
    /// Instruction 'kandb' {AVX512_DQ}.
    Kandb,
    /// Instruction 'kandd' {AVX512_BW}.
    Kandd,
    /// Instruction 'kandnb' {AVX512_DQ}.
    Kandnb,
    /// Instruction 'kandnd' {AVX512_BW}.
    Kandnd,
    /// Instruction 'kandnq' {AVX512_BW}.
    Kandnq,
    /// Instruction 'kandnw' {AVX512_F}.
    Kandnw,
    /// Instruction 'kandq' {AVX512_BW}.
    Kandq,
    /// Instruction 'kandw' {AVX512_F}.
    Kandw,
    /// Instruction 'kmovb' {AVX512_DQ}.
    Kmovb,
    /// Instruction 'kmovd' {AVX512_BW}.
    Kmovd,
    /// Instruction 'kmovq' {AVX512_BW}.
    Kmovq,
    /// Instruction 'kmovw' {AVX512_F}.
    Kmovw,
    /// Instruction 'knotb' {AVX512_DQ}.
    Knotb,
    /// Instruction 'knotd' {AVX512_BW}.
    Knotd,
    /// Instruction 'knotq' {AVX512_BW}.
    Knotq,
    /// Instruction 'knotw' {AVX512_F}.
    Knotw,
    /// Instruction 'korb' {AVX512_DQ}.
    Korb,
    /// Instruction 'kord' {AVX512_BW}.
    Kord,
    /// Instruction 'korq' {AVX512_BW}.
    Korq,
    /// Instruction 'kortestb' {AVX512_DQ}.
    Kortestb,
    /// Instruction 'kortestd' {AVX512_BW}.
    Kortestd,
    /// Instruction 'kortestq' {AVX512_BW}.
    Kortestq,
    /// Instruction 'kortestw' {AVX512_F}.
    Kortestw,
    /// Instruction 'korw' {AVX512_F}.
    Korw,
    /// Instruction 'kshiftlb' {AVX512_DQ}.
    Kshiftlb,
    /// Instruction 'kshiftld' {AVX512_BW}.
    Kshiftld,
    /// Instruction 'kshiftlq' {AVX512_BW}.
    Kshiftlq,
    /// Instruction 'kshiftlw' {AVX512_F}.
    Kshiftlw,
    /// Instruction 'kshiftrb' {AVX512_DQ}.
    Kshiftrb,
    /// Instruction 'kshiftrd' {AVX512_BW}.
    Kshiftrd,
    /// Instruction 'kshiftrq' {AVX512_BW}.
    Kshiftrq,
    /// Instruction 'kshiftrw' {AVX512_F}.
    Kshiftrw,
    /// Instruction 'ktestb' {AVX512_DQ}.
    Ktestb,
    /// Instruction 'ktestd' {AVX512_BW}.
    Ktestd,
    /// Instruction 'ktestq' {AVX512_BW}.
    Ktestq,
    /// Instruction 'ktestw' {AVX512_DQ}.
    Ktestw,
    /// Instruction 'kunpckbw' {AVX512_F}.
    Kunpckbw,
    /// Instruction 'kunpckdq' {AVX512_BW}.
    Kunpckdq,
    /// Instruction 'kunpckwd' {AVX512_BW}.
    Kunpckwd,
    /// Instruction 'kxnorb' {AVX512_DQ}.
    Kxnorb,
    /// Instruction 'kxnord' {AVX512_BW}.
    Kxnord,
    /// Instruction 'kxnorq' {AVX512_BW}.
    Kxnorq,
    /// Instruction 'kxnorw' {AVX512_F}.
    Kxnorw,
    /// Instruction 'kxorb' {AVX512_DQ}.
    Kxorb,
    /// Instruction 'kxord' {AVX512_BW}.
    Kxord,
    /// Instruction 'kxorq' {AVX512_BW}.
    Kxorq,
    /// Instruction 'kxorw' {AVX512_F}.
    Kxorw,
    /// Instruction 'lahf' {LAHFSAHF}.
    Lahf,
    /// Instruction 'lar'.
    Lar,
    /// Instruction 'lcall'.
    Lcall,
    /// Instruction 'lddqu' {SSE3}.
    Lddqu,
    /// Instruction 'ldmxcsr' {SSE}.
    Ldmxcsr,
    /// Instruction 'lds' (X86).
    Lds,
    /// Instruction 'ldtilecfg' {AMX_TILE} (X64).
    Ldtilecfg,
    /// Instruction 'lea'.
    Lea,
    /// Instruction 'leave'.
    Leave,
    /// Instruction 'les' (X86).
    Les,
    /// Instruction 'lfence' {SSE2}.
    Lfence,
    /// Instruction 'lfs'.
    Lfs,
    /// Instruction 'lgdt'.
    Lgdt,
    /// Instruction 'lgs'.
    Lgs,
    /// Instruction 'lidt'.
    Lidt,
    /// Instruction 'ljmp'.
    Ljmp,
    /// Instruction 'lldt'.
    Lldt,
    /// Instruction 'llwpcb' {LWP}.
    Llwpcb,
    /// Instruction 'lmsw'.
    Lmsw,
    /// Instruction 'lods'.
    Lods,
    /// Instruction 'loop'.
    Loop,
    /// Instruction 'loope'.
    Loope,
    /// Instruction 'loopne'.
    Loopne,
    /// Instruction 'lsl'.
    Lsl,
    /// Instruction 'lss'.
    Lss,
    /// Instruction 'ltr'.
    Ltr,
    /// Instruction 'lwpins' {LWP}.
    Lwpins,
    /// Instruction 'lwpval' {LWP}.
    Lwpval,
    /// Instruction 'lzcnt' {LZCNT}.
    Lzcnt,
    /// Instruction 'maskmovdqu' {SSE2}.
    Maskmovdqu,
    /// Instruction 'maskmovq' {MMX2}.
    Maskmovq,
    /// Instruction 'maxpd' {SSE2}.
    Maxpd,
    /// Instruction 'maxps' {SSE}.
    Maxps,
    /// Instruction 'maxsd' {SSE2}.
    Maxsd,
    /// Instruction 'maxss' {SSE}.
    Maxss,
    /// Instruction 'mcommit' {MCOMMIT}.
    Mcommit,
    /// Instruction 'mfence' {SSE2}.
    Mfence,
    /// Instruction 'minpd' {SSE2}.
    Minpd,
    /// Instruction 'minps' {SSE}.
    Minps,
    /// Instruction 'minsd' {SSE2}.
    Minsd,
    /// Instruction 'minss' {SSE}.
    Minss,
    /// Instruction 'monitor' {MONITOR}.
    Monitor,
    /// Instruction 'monitorx' {MONITORX}.
    Monitorx,
    /// Instruction 'mov'.
    Mov,
    /// Instruction 'movabs'.
    Movabs,
    /// Instruction 'movapd' {SSE2}.
    Movapd,
    /// Instruction 'movaps' {SSE}.
    Movaps,
    /// Instruction 'movbe' {MOVBE}.
    Movbe,
    /// Instruction 'movd' {MMX|SSE2}.
    Movd,
    /// Instruction 'movddup' {SSE3}.
    Movddup,
    /// Instruction 'movdir64b' {MOVDIR64B}.
    Movdir64b,
    /// Instruction 'movdiri' {MOVDIRI}.
    Movdiri,
    /// Instruction 'movdq2q' {SSE2}.
    Movdq2q,
    /// Instruction 'movdqa' {SSE2}.
    Movdqa,
    /// Instruction 'movdqu' {SSE2}.
    Movdqu,
    /// Instruction 'movhlps' {SSE}.
    Movhlps,
    /// Instruction 'movhpd' {SSE2}.
    Movhpd,
    /// Instruction 'movhps' {SSE}.
    Movhps,
    /// Instruction 'movlhps' {SSE}.
    Movlhps,
    /// Instruction 'movlpd' {SSE2}.
    Movlpd,
    /// Instruction 'movlps' {SSE}.
    Movlps,
    /// Instruction 'movmskpd' {SSE2}.
    Movmskpd,
    /// Instruction 'movmskps' {SSE}.
    Movmskps,
    /// Instruction 'movntdq' {SSE2}.
    Movntdq,
    /// Instruction 'movntdqa' {SSE4_1}.
    Movntdqa,
    /// Instruction 'movnti' {SSE2}.
    Movnti,
    /// Instruction 'movntpd' {SSE2}.
    Movntpd,
    /// Instruction 'movntps' {SSE}.
    Movntps,
    /// Instruction 'movntq' {MMX2}.
    Movntq,
    /// Instruction 'movntsd' {SSE4A}.
    Movntsd,
    /// Instruction 'movntss' {SSE4A}.
    Movntss,
    /// Instruction 'movq' {MMX|SSE2}.
    Movq,
    /// Instruction 'movq2dq' {SSE2}.
    Movq2dq,
    /// Instruction 'movs'.
    Movs,
    /// Instruction 'movsd' {SSE2}.
    Movsd,
    /// Instruction 'movshdup' {SSE3}.
    Movshdup,
    /// Instruction 'movsldup' {SSE3}.
    Movsldup,
    /// Instruction 'movss' {SSE}.
    Movss,
    /// Instruction 'movsx'.
    Movsx,
    /// Instruction 'movsxd' (X64).
    Movsxd,
    /// Instruction 'movupd' {SSE2}.
    Movupd,
    /// Instruction 'movups' {SSE}.
    Movups,
    /// Instruction 'movzx'.
    Movzx,
    /// Instruction 'mpsadbw' {SSE4_1}.
    Mpsadbw,
    /// Instruction 'mul'.
    Mul,
    /// Instruction 'mulpd' {SSE2}.
    Mulpd,
    /// Instruction 'mulps' {SSE}.
    Mulps,
    /// Instruction 'mulsd' {SSE2}.
    Mulsd,
    /// Instruction 'mulss' {SSE}.
    Mulss,
    /// Instruction 'mulx' {BMI2}.
    Mulx,
    /// Instruction 'mwait' {MONITOR}.
    Mwait,
    /// Instruction 'mwaitx' {MONITORX}.
    Mwaitx,
    /// Instruction 'neg'.
    Neg,
    /// Instruction 'nop'.
    Nop,
    /// Instruction 'not'.
    Not,
    /// Instruction 'or'.
    Or,
    /// Instruction 'orpd' {SSE2}.
    Orpd,
    /// Instruction 'orps' {SSE}.
    Orps,
    /// Instruction 'out'.
    Out,
    /// Instruction 'outs'.
    Outs,
    /// Instruction 'pabsb' {SSSE3}.
    Pabsb,
    /// Instruction 'pabsd' {SSSE3}.
    Pabsd,
    /// Instruction 'pabsw' {SSSE3}.
    Pabsw,
    /// Instruction 'packssdw' {MMX|SSE2}.
    Packssdw,
    /// Instruction 'packsswb' {MMX|SSE2}.
    Packsswb,
    /// Instruction 'packusdw' {SSE4_1}.
    Packusdw,
    /// Instruction 'packuswb' {MMX|SSE2}.
    Packuswb,
    /// Instruction 'paddb' {MMX|SSE2}.
    Paddb,
    /// Instruction 'paddd' {MMX|SSE2}.
    Paddd,
    /// Instruction 'paddq' {SSE2}.
    Paddq,
    /// Instruction 'paddsb' {MMX|SSE2}.
    Paddsb,
    /// Instruction 'paddsw' {MMX|SSE2}.
    Paddsw,
    /// Instruction 'paddusb' {MMX|SSE2}.
    Paddusb,
    /// Instruction 'paddusw' {MMX|SSE2}.
    Paddusw,
    /// Instruction 'paddw' {MMX|SSE2}.
    Paddw,
    /// Instruction 'palignr' {SSSE3}.
    Palignr,
    /// Instruction 'pand' {MMX|SSE2}.
    Pand,
    /// Instruction 'pandn' {MMX|SSE2}.
    Pandn,
    /// Instruction 'pause'.
    Pause,
    /// Instruction 'pavgb' {MMX2|SSE2}.
    Pavgb,
    /// Instruction 'pavgusb' {3DNOW}.
    Pavgusb,
    /// Instruction 'pavgw' {MMX2|SSE2}.
    Pavgw,
    /// Instruction 'pblendvb' {SSE4_1}.
    Pblendvb,
    /// Instruction 'pblendw' {SSE4_1}.
    Pblendw,
    /// Instruction 'pclmulqdq' {PCLMULQDQ}.
    Pclmulqdq,
    /// Instruction 'pcmpeqb' {MMX|SSE2}.
    Pcmpeqb,
    /// Instruction 'pcmpeqd' {MMX|SSE2}.
    Pcmpeqd,
    /// Instruction 'pcmpeqq' {SSE4_1}.
    Pcmpeqq,
    /// Instruction 'pcmpeqw' {MMX|SSE2}.
    Pcmpeqw,
    /// Instruction 'pcmpestri' {SSE4_2}.
    Pcmpestri,
    /// Instruction 'pcmpestrm' {SSE4_2}.
    Pcmpestrm,
    /// Instruction 'pcmpgtb' {MMX|SSE2}.
    Pcmpgtb,
    /// Instruction 'pcmpgtd' {MMX|SSE2}.
    Pcmpgtd,
    /// Instruction 'pcmpgtq' {SSE4_2}.
    Pcmpgtq,
    /// Instruction 'pcmpgtw' {MMX|SSE2}.
    Pcmpgtw,
    /// Instruction 'pcmpistri' {SSE4_2}.
    Pcmpistri,
    /// Instruction 'pcmpistrm' {SSE4_2}.
    Pcmpistrm,
    /// Instruction 'pconfig' {PCONFIG}.
    Pconfig,
    /// Instruction 'pdep' {BMI2}.
    Pdep,
    /// Instruction 'pext' {BMI2}.
    Pext,
    /// Instruction 'pextrb' {SSE4_1}.
    Pextrb,
    /// Instruction 'pextrd' {SSE4_1}.
    Pextrd,
    /// Instruction 'pextrq' {SSE4_1} (X64).
    Pextrq,
    /// Instruction 'pextrw' {MMX2|SSE2|SSE4_1}.
    Pextrw,
    /// Instruction 'pf2id' {3DNOW}.
    Pf2id,
    /// Instruction 'pf2iw' {3DNOW2}.
    Pf2iw,
    /// Instruction 'pfacc' {3DNOW}.
    Pfacc,
    /// Instruction 'pfadd' {3DNOW}.
    Pfadd,
    /// Instruction 'pfcmpeq' {3DNOW}.
    Pfcmpeq,
    /// Instruction 'pfcmpge' {3DNOW}.
    Pfcmpge,
    /// Instruction 'pfcmpgt' {3DNOW}.
    Pfcmpgt,
    /// Instruction 'pfmax' {3DNOW}.
    Pfmax,
    /// Instruction 'pfmin' {3DNOW}.
    Pfmin,
    /// Instruction 'pfmul' {3DNOW}.
    Pfmul,
    /// Instruction 'pfnacc' {3DNOW2}.
    Pfnacc,
    /// Instruction 'pfpnacc' {3DNOW2}.
    Pfpnacc,
    /// Instruction 'pfrcp' {3DNOW}.
    Pfrcp,
    /// Instruction 'pfrcpit1' {3DNOW}.
    Pfrcpit1,
    /// Instruction 'pfrcpit2' {3DNOW}.
    Pfrcpit2,
    /// Instruction 'pfrcpv' {GEODE}.
    Pfrcpv,
    /// Instruction 'pfrsqit1' {3DNOW}.
    Pfrsqit1,
    /// Instruction 'pfrsqrt' {3DNOW}.
    Pfrsqrt,
    /// Instruction 'pfrsqrtv' {GEODE}.
    Pfrsqrtv,
    /// Instruction 'pfsub' {3DNOW}.
    Pfsub,
    /// Instruction 'pfsubr' {3DNOW}.
    Pfsubr,
    /// Instruction 'phaddd' {SSSE3}.
    Phaddd,
    /// Instruction 'phaddsw' {SSSE3}.
    Phaddsw,
    /// Instruction 'phaddw' {SSSE3}.
    Phaddw,
    /// Instruction 'phminposuw' {SSE4_1}.
    Phminposuw,
    /// Instruction 'phsubd' {SSSE3}.
    Phsubd,
    /// Instruction 'phsubsw' {SSSE3}.
    Phsubsw,
    /// Instruction 'phsubw' {SSSE3}.
    Phsubw,
    /// Instruction 'pi2fd' {3DNOW}.
    Pi2fd,
    /// Instruction 'pi2fw' {3DNOW2}.
    Pi2fw,
    /// Instruction 'pinsrb' {SSE4_1}.
    Pinsrb,
    /// Instruction 'pinsrd' {SSE4_1}.
    Pinsrd,
    /// Instruction 'pinsrq' {SSE4_1} (X64).
    Pinsrq,
    /// Instruction 'pinsrw' {MMX2|SSE2}.
    Pinsrw,
    /// Instruction 'pmaddubsw' {SSSE3}.
    Pmaddubsw,
    /// Instruction 'pmaddwd' {MMX|SSE2}.
    Pmaddwd,
    /// Instruction 'pmaxsb' {SSE4_1}.
    Pmaxsb,
    /// Instruction 'pmaxsd' {SSE4_1}.
    Pmaxsd,
    /// Instruction 'pmaxsw' {MMX2|SSE2}.
    Pmaxsw,
    /// Instruction 'pmaxub' {MMX2|SSE2}.
    Pmaxub,
    /// Instruction 'pmaxud' {SSE4_1}.
    Pmaxud,
    /// Instruction 'pmaxuw' {SSE4_1}.
    Pmaxuw,
    /// Instruction 'pminsb' {SSE4_1}.
    Pminsb,
    /// Instruction 'pminsd' {SSE4_1}.
    Pminsd,
    /// Instruction 'pminsw' {MMX2|SSE2}.
    Pminsw,
    /// Instruction 'pminub' {MMX2|SSE2}.
    Pminub,
    /// Instruction 'pminud' {SSE4_1}.
    Pminud,
    /// Instruction 'pminuw' {SSE4_1}.
    Pminuw,
    /// Instruction 'pmovmskb' {MMX2|SSE2}.
    Pmovmskb,
    /// Instruction 'pmovsxbd' {SSE4_1}.
    Pmovsxbd,
    /// Instruction 'pmovsxbq' {SSE4_1}.
    Pmovsxbq,
    /// Instruction 'pmovsxbw' {SSE4_1}.
    Pmovsxbw,
    /// Instruction 'pmovsxdq' {SSE4_1}.
    Pmovsxdq,
    /// Instruction 'pmovsxwd' {SSE4_1}.
    Pmovsxwd,
    /// Instruction 'pmovsxwq' {SSE4_1}.
    Pmovsxwq,
    /// Instruction 'pmovzxbd' {SSE4_1}.
    Pmovzxbd,
    /// Instruction 'pmovzxbq' {SSE4_1}.
    Pmovzxbq,
    /// Instruction 'pmovzxbw' {SSE4_1}.
    Pmovzxbw,
    /// Instruction 'pmovzxdq' {SSE4_1}.
    Pmovzxdq,
    /// Instruction 'pmovzxwd' {SSE4_1}.
    Pmovzxwd,
    /// Instruction 'pmovzxwq' {SSE4_1}.
    Pmovzxwq,
    /// Instruction 'pmuldq' {SSE4_1}.
    Pmuldq,
    /// Instruction 'pmulhrsw' {SSSE3}.
    Pmulhrsw,
    /// Instruction 'pmulhrw' {3DNOW}.
    Pmulhrw,
    /// Instruction 'pmulhuw' {MMX2|SSE2}.
    Pmulhuw,
    /// Instruction 'pmulhw' {MMX|SSE2}.
    Pmulhw,
    /// Instruction 'pmulld' {SSE4_1}.
    Pmulld,
    /// Instruction 'pmullw' {MMX|SSE2}.
    Pmullw,
    /// Instruction 'pmuludq' {SSE2}.
    Pmuludq,
    /// Instruction 'pop'.
    Pop,
    /// Instruction 'popa' (X86).
    Popa,
    /// Instruction 'popad' (X86).
    Popad,
    /// Instruction 'popcnt' {POPCNT}.
    Popcnt,
    /// Instruction 'popf'.
    Popf,
    /// Instruction 'popfd' (X86).
    Popfd,
    /// Instruction 'popfq' (X64).
    Popfq,
    /// Instruction 'por' {MMX|SSE2}.
    Por,
    /// Instruction 'prefetch' {3DNOW}.
    Prefetch,
    /// Instruction 'prefetchit0' {PREFETCHI} (X64).
    Prefetchit0,
    /// Instruction 'prefetchit1' {PREFETCHI} (X64).
    Prefetchit1,
    /// Instruction 'prefetchnta' {SSE}.
    Prefetchnta,
    /// Instruction 'prefetcht0' {SSE}.
    Prefetcht0,
    /// Instruction 'prefetcht1' {SSE}.
    Prefetcht1,
    /// Instruction 'prefetcht2' {SSE}.
    Prefetcht2,
    /// Instruction 'prefetchw' {PREFETCHW}.
    Prefetchw,
    /// Instruction 'prefetchwt1' {PREFETCHWT1}.
    Prefetchwt1,
    /// Instruction 'psadbw' {MMX2|SSE2}.
    Psadbw,
    /// Instruction 'pshufb' {SSSE3}.
    Pshufb,
    /// Instruction 'pshufd' {SSE2}.
    Pshufd,
    /// Instruction 'pshufhw' {SSE2}.
    Pshufhw,
    /// Instruction 'pshuflw' {SSE2}.
    Pshuflw,
    /// Instruction 'pshufw' {MMX2}.
    Pshufw,
    /// Instruction 'psignb' {SSSE3}.
    Psignb,
    /// Instruction 'psignd' {SSSE3}.
    Psignd,
    /// Instruction 'psignw' {SSSE3}.
    Psignw,
    /// Instruction 'pslld' {MMX|SSE2}.
    Pslld,
    /// Instruction 'pslldq' {SSE2}.
    Pslldq,
    /// Instruction 'psllq' {MMX|SSE2}.
    Psllq,
    /// Instruction 'psllw' {MMX|SSE2}.
    Psllw,
    /// Instruction 'psmash' {SEV_SNP} (X64).
    Psmash,
    /// Instruction 'psrad' {MMX|SSE2}.
    Psrad,
    /// Instruction 'psraw' {MMX|SSE2}.
    Psraw,
    /// Instruction 'psrld' {MMX|SSE2}.
    Psrld,
    /// Instruction 'psrldq' {SSE2}.
    Psrldq,
    /// Instruction 'psrlq' {MMX|SSE2}.
    Psrlq,
    /// Instruction 'psrlw' {MMX|SSE2}.
    Psrlw,
    /// Instruction 'psubb' {MMX|SSE2}.
    Psubb,
    /// Instruction 'psubd' {MMX|SSE2}.
    Psubd,
    /// Instruction 'psubq' {SSE2}.
    Psubq,
    /// Instruction 'psubsb' {MMX|SSE2}.
    Psubsb,
    /// Instruction 'psubsw' {MMX|SSE2}.
    Psubsw,
    /// Instruction 'psubusb' {MMX|SSE2}.
    Psubusb,
    /// Instruction 'psubusw' {MMX|SSE2}.
    Psubusw,
    /// Instruction 'psubw' {MMX|SSE2}.
    Psubw,
    /// Instruction 'pswapd' {3DNOW2}.
    Pswapd,
    /// Instruction 'ptest' {SSE4_1}.
    Ptest,
    /// Instruction 'ptwrite' {PTWRITE}.
    Ptwrite,
    /// Instruction 'punpckhbw' {MMX|SSE2}.
    Punpckhbw,
    /// Instruction 'punpckhdq' {MMX|SSE2}.
    Punpckhdq,
    /// Instruction 'punpckhqdq' {SSE2}.
    Punpckhqdq,
    /// Instruction 'punpckhwd' {MMX|SSE2}.
    Punpckhwd,
    /// Instruction 'punpcklbw' {MMX|SSE2}.
    Punpcklbw,
    /// Instruction 'punpckldq' {MMX|SSE2}.
    Punpckldq,
    /// Instruction 'punpcklqdq' {SSE2}.
    Punpcklqdq,
    /// Instruction 'punpcklwd' {MMX|SSE2}.
    Punpcklwd,
    /// Instruction 'push'.
    Push,
    /// Instruction 'pusha' (X86).
    Pusha,
    /// Instruction 'pushad' (X86).
    Pushad,
    /// Instruction 'pushf'.
    Pushf,
    /// Instruction 'pushfd' (X86).
    Pushfd,
    /// Instruction 'pushfq' (X64).
    Pushfq,
    /// Instruction 'pushw'.
    Pushw,
    /// Instruction 'pvalidate' {SEV_SNP}.
    Pvalidate,
    /// Instruction 'pxor' {MMX|SSE2}.
    Pxor,
    /// Instruction 'rcl'.
    Rcl,
    /// Instruction 'rcpps' {SSE}.
    Rcpps,
    /// Instruction 'rcpss' {SSE}.
    Rcpss,
    /// Instruction 'rcr'.
    Rcr,
    /// Instruction 'rdfsbase' {FSGSBASE} (X64).
    Rdfsbase,
    /// Instruction 'rdgsbase' {FSGSBASE} (X64).
    Rdgsbase,
    /// Instruction 'rdmsr' {MSR|MSR_IMM}.
    Rdmsr,
    /// Instruction 'rdpid' {RDPID}.
    Rdpid,
    /// Instruction 'rdpkru' {OSPKE}.
    Rdpkru,
    /// Instruction 'rdpmc'.
    Rdpmc,
    /// Instruction 'rdpru' {RDPRU}.
    Rdpru,
    /// Instruction 'rdrand' {RDRAND}.
    Rdrand,
    /// Instruction 'rdseed' {RDSEED}.
    Rdseed,
    /// Instruction 'rdsspd' {CET_SS}.
    Rdsspd,
    /// Instruction 'rdsspq' {CET_SS} (X64).
    Rdsspq,
    /// Instruction 'rdtsc' {RDTSC}.
    Rdtsc,
    /// Instruction 'rdtscp' {RDTSCP}.
    Rdtscp,
    /// Instruction 'ret'.
    Ret,
    /// Instruction 'retf'.
    Retf,
    /// Instruction 'rmpadjust' {SEV_SNP} (X64).
    Rmpadjust,
    /// Instruction 'rmpupdate' {SEV_SNP} (X64).
    Rmpupdate,
    /// Instruction 'rol'.
    Rol,
    /// Instruction 'ror'.
    Ror,
    /// Instruction 'rorx' {BMI2}.
    Rorx,
    /// Instruction 'roundpd' {SSE4_1}.
    Roundpd,
    /// Instruction 'roundps' {SSE4_1}.
    Roundps,
    /// Instruction 'roundsd' {SSE4_1}.
    Roundsd,
    /// Instruction 'roundss' {SSE4_1}.
    Roundss,
    /// Instruction 'rsm' (X86).
    Rsm,
    /// Instruction 'rsqrtps' {SSE}.
    Rsqrtps,
    /// Instruction 'rsqrtss' {SSE}.
    Rsqrtss,
    /// Instruction 'rstorssp' {CET_SS}.
    Rstorssp,
    /// Instruction 'sahf' {LAHFSAHF}.
    Sahf,
    /// Instruction 'sar'.
    Sar,
    /// Instruction 'sarx' {BMI2}.
    Sarx,
    /// Instruction 'saveprevssp' {CET_SS}.
    Saveprevssp,
    /// Instruction 'sbb'.
    Sbb,
    /// Instruction 'scas'.
    Scas,
    /// Instruction 'seamcall' {SEAM}.
    Seamcall,
    /// Instruction 'seamops' {SEAM}.
    Seamops,
    /// Instruction 'seamret' {SEAM}.
    Seamret,
    /// Instruction 'senduipi' {UINTR} (X64).
    Senduipi,
    /// Instruction 'serialize' {SERIALIZE}.
    Serialize,
    /// Instruction 'setb'.
    Setb,
    /// Instruction 'setbe'.
    Setbe,
    /// Instruction 'setl'.
    Setl,
    /// Instruction 'setle'.
    Setle,
    /// Instruction 'setnb'.
    Setnb,
    /// Instruction 'setnbe'.
    Setnbe,
    /// Instruction 'setnl'.
    Setnl,
    /// Instruction 'setnle'.
    Setnle,
    /// Instruction 'setno'.
    Setno,
    /// Instruction 'setnp'.
    Setnp,
    /// Instruction 'setns'.
    Setns,
    /// Instruction 'setnz'.
    Setnz,
    /// Instruction 'seto'.
    Seto,
    /// Instruction 'setp'.
    Setp,
    /// Instruction 'sets'.
    Sets,
    /// Instruction 'setssbsy' {CET_SS}.
    Setssbsy,
    /// Instruction 'setz'.
    Setz,
    /// Instruction 'sfence' {SSE}.
    Sfence,
    /// Instruction 'sgdt'.
    Sgdt,
    /// Instruction 'sha1msg1' {SHA}.
    Sha1msg1,
    /// Instruction 'sha1msg2' {SHA}.
    Sha1msg2,
    /// Instruction 'sha1nexte' {SHA}.
    Sha1nexte,
    /// Instruction 'sha1rnds4' {SHA}.
    Sha1rnds4,
    /// Instruction 'sha256msg1' {SHA}.
    Sha256msg1,
    /// Instruction 'sha256msg2' {SHA}.
    Sha256msg2,
    /// Instruction 'sha256rnds2' {SHA}.
    Sha256rnds2,
    /// Instruction 'shl'.
    Shl,
    /// Instruction 'shld'.
    Shld,
    /// Instruction 'shlx' {BMI2}.
    Shlx,
    /// Instruction 'shr'.
    Shr,
    /// Instruction 'shrd'.
    Shrd,
    /// Instruction 'shrx' {BMI2}.
    Shrx,
    /// Instruction 'shufpd' {SSE2}.
    Shufpd,
    /// Instruction 'shufps' {SSE}.
    Shufps,
    /// Instruction 'sidt'.
    Sidt,
    /// Instruction 'skinit' {SKINIT}.
    Skinit,
    /// Instruction 'sldt'.
    Sldt,
    /// Instruction 'slwpcb' {LWP}.
    Slwpcb,
    /// Instruction 'smsw'.
    Smsw,
    /// Instruction 'sqrtpd' {SSE2}.
    Sqrtpd,
    /// Instruction 'sqrtps' {SSE}.
    Sqrtps,
    /// Instruction 'sqrtsd' {SSE2}.
    Sqrtsd,
    /// Instruction 'sqrtss' {SSE}.
    Sqrtss,
    /// Instruction 'stac' {SMAP}.
    Stac,
    /// Instruction 'stc'.
    Stc,
    /// Instruction 'std'.
    Std,
    /// Instruction 'stgi' {SKINIT}.
    Stgi,
    /// Instruction 'sti'.
    Sti,
    /// Instruction 'stmxcsr' {SSE}.
    Stmxcsr,
    /// Instruction 'stos'.
    Stos,
    /// Instruction 'str'.
    Str,
    /// Instruction 'sttilecfg' {AMX_TILE} (X64).
    Sttilecfg,
    /// Instruction 'stui' {UINTR} (X64).
    Stui,
    /// Instruction 'sub'.
    Sub,
    /// Instruction 'subpd' {SSE2}.
    Subpd,
    /// Instruction 'subps' {SSE}.
    Subps,
    /// Instruction 'subsd' {SSE2}.
    Subsd,
    /// Instruction 'subss' {SSE}.
    Subss,
    /// Instruction 'swapgs' (X64).
    Swapgs,
    /// Instruction 'syscall' (X64).
    Syscall,
    /// Instruction 'sysenter'.
    Sysenter,
    /// Instruction 'sysexit'.
    Sysexit,
    /// Instruction 'sysexitq' (X64).
    Sysexitq,
    /// Instruction 'sysret' (X64).
    Sysret,
    /// Instruction 'sysretq' (X64).
    Sysretq,
    /// Instruction 't1mskc' {TBM}.
    T1mskc,
    /// Instruction 'tcmmimfp16ps' {AMX_COMPLEX} (X64).
    Tcmmimfp16ps,
    /// Instruction 'tcmmrlfp16ps' {AMX_COMPLEX} (X64).
    Tcmmrlfp16ps,
    /// Instruction 'tdcall' {SEAM}.
    Tdcall,
    /// Instruction 'tdpbf16ps' {AMX_BF16} (X64).
    Tdpbf16ps,
    /// Instruction 'tdpbssd' {AMX_INT8} (X64).
    Tdpbssd,
    /// Instruction 'tdpbsud' {AMX_INT8} (X64).
    Tdpbsud,
    /// Instruction 'tdpbusd' {AMX_INT8} (X64).
    Tdpbusd,
    /// Instruction 'tdpbuud' {AMX_INT8} (X64).
    Tdpbuud,
    /// Instruction 'tdpfp16ps' {AMX_FP16} (X64).
    Tdpfp16ps,
    /// Instruction 'test'.
    Test,
    /// Instruction 'testui' {UINTR} (X64).
    Testui,
    /// Instruction 'tileloadd' {AMX_TILE} (X64).
    Tileloadd,
    /// Instruction 'tileloaddt1' {AMX_TILE} (X64).
    Tileloaddt1,
    /// Instruction 'tilerelease' {AMX_TILE} (X64).
    Tilerelease,
    /// Instruction 'tilestored' {AMX_TILE} (X64).
    Tilestored,
    /// Instruction 'tilezero' {AMX_TILE} (X64).
    Tilezero,
    /// Instruction 'tlbsync' {INVLPGB}.
    Tlbsync,
    /// Instruction 'tpause' {WAITPKG}.
    Tpause,
    /// Instruction 'tzcnt' {BMI}.
    Tzcnt,
    /// Instruction 'tzmsk' {TBM}.
    Tzmsk,
    /// Instruction 'ucomisd' {SSE2}.
    Ucomisd,
    /// Instruction 'ucomiss' {SSE}.
    Ucomiss,
    /// Instruction 'ud0'.
    Ud0,
    /// Instruction 'ud1'.
    Ud1,
    /// Instruction 'ud2'.
    Ud2,
    /// Instruction 'uiret' {UINTR} (X64).
    Uiret,
    /// Instruction 'umonitor' {WAITPKG}.
    Umonitor,
    /// Instruction 'umwait' {WAITPKG}.
    Umwait,
    /// Instruction 'unpckhpd' {SSE2}.
    Unpckhpd,
    /// Instruction 'unpckhps' {SSE}.
    Unpckhps,
    /// Instruction 'unpcklpd' {SSE2}.
    Unpcklpd,
    /// Instruction 'unpcklps' {SSE}.
    Unpcklps,
    /// Instruction 'vaddpd' {AVX|AVX512_F+VL}.
    Vaddpd,
    /// Instruction 'vaddph' {AVX512_FP16+VL}.
    Vaddph,
    /// Instruction 'vaddps' {AVX|AVX512_F+VL}.
    Vaddps,
    /// Instruction 'vaddsd' {AVX|AVX512_F+VL}.
    Vaddsd,
    /// Instruction 'vaddsh' {AVX512_FP16+VL}.
    Vaddsh,
    /// Instruction 'vaddss' {AVX|AVX512_F+VL}.
    Vaddss,
    /// Instruction 'vaddsubpd' {AVX}.
    Vaddsubpd,
    /// Instruction 'vaddsubps' {AVX}.
    Vaddsubps,
    /// Instruction 'vaesdec' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesdec,
    /// Instruction 'vaesdeclast' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesdeclast,
    /// Instruction 'vaesenc' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesenc,
    /// Instruction 'vaesenclast' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesenclast,
    /// Instruction 'vaesimc' {AVX & AESNI}.
    Vaesimc,
    /// Instruction 'vaeskeygenassist' {AVX & AESNI}.
    Vaeskeygenassist,
    /// Instruction 'valignd' {AVX512_F+VL}.
    Valignd,
    /// Instruction 'valignq' {AVX512_F+VL}.
    Valignq,
    /// Instruction 'vandnpd' {AVX|AVX512_DQ+VL}.
    Vandnpd,
    /// Instruction 'vandnps' {AVX|AVX512_DQ+VL}.
    Vandnps,
    /// Instruction 'vandpd' {AVX|AVX512_DQ+VL}.
    Vandpd,
    /// Instruction 'vandps' {AVX|AVX512_DQ+VL}.
    Vandps,
    /// Instruction 'vbcstnebf162ps' {AVX_NE_CONVERT}.
    Vbcstnebf162ps,
    /// Instruction 'vbcstnesh2ps' {AVX_NE_CONVERT}.
    Vbcstnesh2ps,
    /// Instruction 'vblendmpd' {AVX512_F+VL}.
    Vblendmpd,
    /// Instruction 'vblendmps' {AVX512_F+VL}.
    Vblendmps,
    /// Instruction 'vblendpd' {AVX}.
    Vblendpd,
    /// Instruction 'vblendps' {AVX}.
    Vblendps,
    /// Instruction 'vblendvpd' {AVX}.
    Vblendvpd,
    /// Instruction 'vblendvps' {AVX}.
    Vblendvps,
    /// Instruction 'vbroadcastf128' {AVX}.
    Vbroadcastf128,
    /// Instruction 'vbroadcastf32x2' {AVX512_DQ+VL}.
    Vbroadcastf32x2,
    /// Instruction 'vbroadcastf32x4' {AVX512_F+VL}.
    Vbroadcastf32x4,
    /// Instruction 'vbroadcastf32x8' {AVX512_DQ+VL}.
    Vbroadcastf32x8,
    /// Instruction 'vbroadcastf64x2' {AVX512_DQ+VL}.
    Vbroadcastf64x2,
    /// Instruction 'vbroadcastf64x4' {AVX512_F+VL}.
    Vbroadcastf64x4,
    /// Instruction 'vbroadcasti128' {AVX2}.
    Vbroadcasti128,
    /// Instruction 'vbroadcasti32x2' {AVX512_DQ+VL}.
    Vbroadcasti32x2,
    /// Instruction 'vbroadcasti32x4' {AVX512_F+VL}.
    Vbroadcasti32x4,
    /// Instruction 'vbroadcasti32x8' {AVX512_DQ+VL}.
    Vbroadcasti32x8,
    /// Instruction 'vbroadcasti64x2' {AVX512_DQ+VL}.
    Vbroadcasti64x2,
    /// Instruction 'vbroadcasti64x4' {AVX512_F+VL}.
    Vbroadcasti64x4,
    /// Instruction 'vbroadcastsd' {AVX|AVX2|AVX512_F+VL}.
    Vbroadcastsd,
    /// Instruction 'vbroadcastss' {AVX|AVX2|AVX512_F+VL}.
    Vbroadcastss,
    /// Instruction 'vcmppd' {AVX|AVX512_F+VL}.
    Vcmppd,
    /// Instruction 'vcmpph' {AVX512_FP16+VL}.
    Vcmpph,
    /// Instruction 'vcmpps' {AVX|AVX512_F+VL}.
    Vcmpps,
    /// Instruction 'vcmpsd' {AVX|AVX512_F+VL}.
    Vcmpsd,
    /// Instruction 'vcmpsh' {AVX512_FP16+VL}.
    Vcmpsh,
    /// Instruction 'vcmpss' {AVX|AVX512_F+VL}.
    Vcmpss,
    /// Instruction 'vcomisd' {AVX|AVX512_F+VL}.
    Vcomisd,
    /// Instruction 'vcomish' {AVX512_FP16+VL}.
    Vcomish,
    /// Instruction 'vcomiss' {AVX|AVX512_F+VL}.
    Vcomiss,
    /// Instruction 'vcompresspd' {AVX512_F+VL}.
    Vcompresspd,
    /// Instruction 'vcompressps' {AVX512_F+VL}.
    Vcompressps,
    /// Instruction 'vcvtdq2pd' {AVX|AVX512_F+VL}.
    Vcvtdq2pd,
    /// Instruction 'vcvtdq2ph' {AVX512_FP16+VL}.
    Vcvtdq2ph,
    /// Instruction 'vcvtdq2ps' {AVX|AVX512_F+VL}.
    Vcvtdq2ps,
    /// Instruction 'vcvtne2ps2bf16' {AVX512_BF16+VL}.
    Vcvtne2ps2bf16,
    /// Instruction 'vcvtneebf162ps' {AVX_NE_CONVERT}.
    Vcvtneebf162ps,
    /// Instruction 'vcvtneeph2ps' {AVX_NE_CONVERT}.
    Vcvtneeph2ps,
    /// Instruction 'vcvtneobf162ps' {AVX_NE_CONVERT}.
    Vcvtneobf162ps,
    /// Instruction 'vcvtneoph2ps' {AVX_NE_CONVERT}.
    Vcvtneoph2ps,
    /// Instruction 'vcvtneps2bf16' {AVX_NE_CONVERT|AVX512_BF16+VL}.
    Vcvtneps2bf16,
    /// Instruction 'vcvtpd2dq' {AVX|AVX512_F+VL}.
    Vcvtpd2dq,
    /// Instruction 'vcvtpd2ph' {AVX512_FP16+VL}.
    Vcvtpd2ph,
    /// Instruction 'vcvtpd2ps' {AVX|AVX512_F+VL}.
    Vcvtpd2ps,
    /// Instruction 'vcvtpd2qq' {AVX512_DQ+VL}.
    Vcvtpd2qq,
    /// Instruction 'vcvtpd2udq' {AVX512_F+VL}.
    Vcvtpd2udq,
    /// Instruction 'vcvtpd2uqq' {AVX512_DQ+VL}.
    Vcvtpd2uqq,
    /// Instruction 'vcvtph2dq' {AVX512_FP16+VL}.
    Vcvtph2dq,
    /// Instruction 'vcvtph2pd' {AVX512_FP16+VL}.
    Vcvtph2pd,
    /// Instruction 'vcvtph2ps' {AVX512_F+VL & F16C}.
    Vcvtph2ps,
    /// Instruction 'vcvtph2psx' {AVX512_FP16+VL}.
    Vcvtph2psx,
    /// Instruction 'vcvtph2qq' {AVX512_FP16+VL}.
    Vcvtph2qq,
    /// Instruction 'vcvtph2udq' {AVX512_FP16+VL}.
    Vcvtph2udq,
    /// Instruction 'vcvtph2uqq' {AVX512_FP16+VL}.
    Vcvtph2uqq,
    /// Instruction 'vcvtph2uw' {AVX512_FP16+VL}.
    Vcvtph2uw,
    /// Instruction 'vcvtph2w' {AVX512_FP16+VL}.
    Vcvtph2w,
    /// Instruction 'vcvtps2dq' {AVX|AVX512_F+VL}.
    Vcvtps2dq,
    /// Instruction 'vcvtps2pd' {AVX|AVX512_F+VL}.
    Vcvtps2pd,
    /// Instruction 'vcvtps2ph' {AVX512_F+VL & F16C}.
    Vcvtps2ph,
    /// Instruction 'vcvtps2phx' {AVX512_FP16+VL}.
    Vcvtps2phx,
    /// Instruction 'vcvtps2qq' {AVX512_DQ+VL}.
    Vcvtps2qq,
    /// Instruction 'vcvtps2udq' {AVX512_F+VL}.
    Vcvtps2udq,
    /// Instruction 'vcvtps2uqq' {AVX512_DQ+VL}.
    Vcvtps2uqq,
    /// Instruction 'vcvtqq2pd' {AVX512_DQ+VL}.
    Vcvtqq2pd,
    /// Instruction 'vcvtqq2ph' {AVX512_FP16+VL}.
    Vcvtqq2ph,
    /// Instruction 'vcvtqq2ps' {AVX512_DQ+VL}.
    Vcvtqq2ps,
    /// Instruction 'vcvtsd2sh' {AVX512_FP16+VL}.
    Vcvtsd2sh,
    /// Instruction 'vcvtsd2si' {AVX|AVX512_F+VL}.
    Vcvtsd2si,
    /// Instruction 'vcvtsd2ss' {AVX|AVX512_F+VL}.
    Vcvtsd2ss,
    /// Instruction 'vcvtsd2usi' {AVX512_F+VL}.
    Vcvtsd2usi,
    /// Instruction 'vcvtsh2sd' {AVX512_FP16+VL}.
    Vcvtsh2sd,
    /// Instruction 'vcvtsh2si' {AVX512_FP16+VL}.
    Vcvtsh2si,
    /// Instruction 'vcvtsh2ss' {AVX512_FP16+VL}.
    Vcvtsh2ss,
    /// Instruction 'vcvtsh2usi' {AVX512_FP16+VL}.
    Vcvtsh2usi,
    /// Instruction 'vcvtsi2sd' {AVX|AVX512_F+VL}.
    Vcvtsi2sd,
    /// Instruction 'vcvtsi2sh' {AVX512_FP16+VL}.
    Vcvtsi2sh,
    /// Instruction 'vcvtsi2ss' {AVX|AVX512_F+VL}.
    Vcvtsi2ss,
    /// Instruction 'vcvtss2sd' {AVX|AVX512_F+VL}.
    Vcvtss2sd,
    /// Instruction 'vcvtss2sh' {AVX512_FP16+VL}.
    Vcvtss2sh,
    /// Instruction 'vcvtss2si' {AVX|AVX512_F+VL}.
    Vcvtss2si,
    /// Instruction 'vcvtss2usi' {AVX512_F+VL}.
    Vcvtss2usi,
    /// Instruction 'vcvttpd2dq' {AVX|AVX512_F+VL}.
    Vcvttpd2dq,
    /// Instruction 'vcvttpd2qq' {AVX512_F+VL}.
    Vcvttpd2qq,
    /// Instruction 'vcvttpd2udq' {AVX512_F+VL}.
    Vcvttpd2udq,
    /// Instruction 'vcvttpd2uqq' {AVX512_DQ+VL}.
    Vcvttpd2uqq,
    /// Instruction 'vcvttph2dq' {AVX512_FP16+VL}.
    Vcvttph2dq,
    /// Instruction 'vcvttph2qq' {AVX512_FP16+VL}.
    Vcvttph2qq,
    /// Instruction 'vcvttph2udq' {AVX512_FP16+VL}.
    Vcvttph2udq,
    /// Instruction 'vcvttph2uqq' {AVX512_FP16+VL}.
    Vcvttph2uqq,
    /// Instruction 'vcvttph2uw' {AVX512_FP16+VL}.
    Vcvttph2uw,
    /// Instruction 'vcvttph2w' {AVX512_FP16+VL}.
    Vcvttph2w,
    /// Instruction 'vcvttps2dq' {AVX|AVX512_F+VL}.
    Vcvttps2dq,
    /// Instruction 'vcvttps2qq' {AVX512_DQ+VL}.
    Vcvttps2qq,
    /// Instruction 'vcvttps2udq' {AVX512_F+VL}.
    Vcvttps2udq,
    /// Instruction 'vcvttps2uqq' {AVX512_DQ+VL}.
    Vcvttps2uqq,
    /// Instruction 'vcvttsd2si' {AVX|AVX512_F+VL}.
    Vcvttsd2si,
    /// Instruction 'vcvttsd2usi' {AVX512_F+VL}.
    Vcvttsd2usi,
    /// Instruction 'vcvttsh2si' {AVX512_FP16+VL}.
    Vcvttsh2si,
    /// Instruction 'vcvttsh2usi' {AVX512_FP16+VL}.
    Vcvttsh2usi,
    /// Instruction 'vcvttss2si' {AVX|AVX512_F+VL}.
    Vcvttss2si,
    /// Instruction 'vcvttss2usi' {AVX512_F+VL}.
    Vcvttss2usi,
    /// Instruction 'vcvtudq2pd' {AVX512_F+VL}.
    Vcvtudq2pd,
    /// Instruction 'vcvtudq2ph' {AVX512_FP16+VL}.
    Vcvtudq2ph,
    /// Instruction 'vcvtudq2ps' {AVX512_F+VL}.
    Vcvtudq2ps,
    /// Instruction 'vcvtuqq2pd' {AVX512_DQ+VL}.
    Vcvtuqq2pd,
    /// Instruction 'vcvtuqq2ph' {AVX512_FP16+VL}.
    Vcvtuqq2ph,
    /// Instruction 'vcvtuqq2ps' {AVX512_DQ+VL}.
    Vcvtuqq2ps,
    /// Instruction 'vcvtusi2sd' {AVX512_F+VL}.
    Vcvtusi2sd,
    /// Instruction 'vcvtusi2sh' {AVX512_FP16+VL}.
    Vcvtusi2sh,
    /// Instruction 'vcvtusi2ss' {AVX512_F+VL}.
    Vcvtusi2ss,
    /// Instruction 'vcvtuw2ph' {AVX512_FP16+VL}.
    Vcvtuw2ph,
    /// Instruction 'vcvtw2ph' {AVX512_FP16+VL}.
    Vcvtw2ph,
    /// Instruction 'vdbpsadbw' {AVX512_BW+VL}.
    Vdbpsadbw,
    /// Instruction 'vdivpd' {AVX|AVX512_F+VL}.
    Vdivpd,
    /// Instruction 'vdivph' {AVX512_FP16+VL}.
    Vdivph,
    /// Instruction 'vdivps' {AVX|AVX512_F+VL}.
    Vdivps,
    /// Instruction 'vdivsd' {AVX|AVX512_F+VL}.
    Vdivsd,
    /// Instruction 'vdivsh' {AVX512_FP16+VL}.
    Vdivsh,
    /// Instruction 'vdivss' {AVX|AVX512_F+VL}.
    Vdivss,
    /// Instruction 'vdpbf16ps' {AVX512_BF16+VL}.
    Vdpbf16ps,
    /// Instruction 'vdppd' {AVX}.
    Vdppd,
    /// Instruction 'vdpps' {AVX}.
    Vdpps,
    /// Instruction 'verr'.
    Verr,
    /// Instruction 'verw'.
    Verw,
    /// Instruction 'vexpandpd' {AVX512_F+VL}.
    Vexpandpd,
    /// Instruction 'vexpandps' {AVX512_F+VL}.
    Vexpandps,
    /// Instruction 'vextractf128' {AVX}.
    Vextractf128,
    /// Instruction 'vextractf32x4' {AVX512_F+VL}.
    Vextractf32x4,
    /// Instruction 'vextractf32x8' {AVX512_DQ+VL}.
    Vextractf32x8,
    /// Instruction 'vextractf64x2' {AVX512_DQ+VL}.
    Vextractf64x2,
    /// Instruction 'vextractf64x4' {AVX512_F+VL}.
    Vextractf64x4,
    /// Instruction 'vextracti128' {AVX2}.
    Vextracti128,
    /// Instruction 'vextracti32x4' {AVX512_F+VL}.
    Vextracti32x4,
    /// Instruction 'vextracti32x8' {AVX512_DQ+VL}.
    Vextracti32x8,
    /// Instruction 'vextracti64x2' {AVX512_DQ+VL}.
    Vextracti64x2,
    /// Instruction 'vextracti64x4' {AVX512_F+VL}.
    Vextracti64x4,
    /// Instruction 'vextractps' {AVX|AVX512_F+VL}.
    Vextractps,
    /// Instruction 'vfcmaddcph' {AVX512_FP16+VL}.
    Vfcmaddcph,
    /// Instruction 'vfcmaddcsh' {AVX512_FP16+VL}.
    Vfcmaddcsh,
    /// Instruction 'vfcmulcph' {AVX512_FP16+VL}.
    Vfcmulcph,
    /// Instruction 'vfcmulcsh' {AVX512_FP16+VL}.
    Vfcmulcsh,
    /// Instruction 'vfixupimmpd' {AVX512_F+VL}.
    Vfixupimmpd,
    /// Instruction 'vfixupimmps' {AVX512_F+VL}.
    Vfixupimmps,
    /// Instruction 'vfixupimmsd' {AVX512_F+VL}.
    Vfixupimmsd,
    /// Instruction 'vfixupimmss' {AVX512_F+VL}.
    Vfixupimmss,
    /// Instruction 'vfmadd132pd' {FMA|AVX512_F+VL}.
    Vfmadd132pd,
    /// Instruction 'vfmadd132ph' {AVX512_FP16+VL}.
    Vfmadd132ph,
    /// Instruction 'vfmadd132ps' {FMA|AVX512_F+VL}.
    Vfmadd132ps,
    /// Instruction 'vfmadd132sd' {FMA|AVX512_F+VL}.
    Vfmadd132sd,
    /// Instruction 'vfmadd132sh' {AVX512_FP16+VL}.
    Vfmadd132sh,
    /// Instruction 'vfmadd132ss' {FMA|AVX512_F+VL}.
    Vfmadd132ss,
    /// Instruction 'vfmadd213pd' {FMA|AVX512_F+VL}.
    Vfmadd213pd,
    /// Instruction 'vfmadd213ph' {AVX512_FP16+VL}.
    Vfmadd213ph,
    /// Instruction 'vfmadd213ps' {FMA|AVX512_F+VL}.
    Vfmadd213ps,
    /// Instruction 'vfmadd213sd' {FMA|AVX512_F+VL}.
    Vfmadd213sd,
    /// Instruction 'vfmadd213sh' {AVX512_FP16+VL}.
    Vfmadd213sh,
    /// Instruction 'vfmadd213ss' {FMA|AVX512_F+VL}.
    Vfmadd213ss,
    /// Instruction 'vfmadd231pd' {FMA|AVX512_F+VL}.
    Vfmadd231pd,
    /// Instruction 'vfmadd231ph' {AVX512_FP16+VL}.
    Vfmadd231ph,
    /// Instruction 'vfmadd231ps' {FMA|AVX512_F+VL}.
    Vfmadd231ps,
    /// Instruction 'vfmadd231sd' {FMA|AVX512_F+VL}.
    Vfmadd231sd,
    /// Instruction 'vfmadd231sh' {AVX512_FP16+VL}.
    Vfmadd231sh,
    /// Instruction 'vfmadd231ss' {FMA|AVX512_F+VL}.
    Vfmadd231ss,
    /// Instruction 'vfmaddcph' {AVX512_FP16+VL}.
    Vfmaddcph,
    /// Instruction 'vfmaddcsh' {AVX512_FP16+VL}.
    Vfmaddcsh,
    /// Instruction 'vfmaddpd' {FMA4}.
    Vfmaddpd,
    /// Instruction 'vfmaddps' {FMA4}.
    Vfmaddps,
    /// Instruction 'vfmaddsd' {FMA4}.
    Vfmaddsd,
    /// Instruction 'vfmaddss' {FMA4}.
    Vfmaddss,
    /// Instruction 'vfmaddsub132pd' {FMA|AVX512_F+VL}.
    Vfmaddsub132pd,
    /// Instruction 'vfmaddsub132ph' {AVX512_FP16+VL}.
    Vfmaddsub132ph,
    /// Instruction 'vfmaddsub132ps' {FMA|AVX512_F+VL}.
    Vfmaddsub132ps,
    /// Instruction 'vfmaddsub213pd' {FMA|AVX512_F+VL}.
    Vfmaddsub213pd,
    /// Instruction 'vfmaddsub213ph' {AVX512_FP16+VL}.
    Vfmaddsub213ph,
    /// Instruction 'vfmaddsub213ps' {FMA|AVX512_F+VL}.
    Vfmaddsub213ps,
    /// Instruction 'vfmaddsub231pd' {FMA|AVX512_F+VL}.
    Vfmaddsub231pd,
    /// Instruction 'vfmaddsub231ph' {AVX512_FP16+VL}.
    Vfmaddsub231ph,
    /// Instruction 'vfmaddsub231ps' {FMA|AVX512_F+VL}.
    Vfmaddsub231ps,
    /// Instruction 'vfmaddsubpd' {FMA4}.
    Vfmaddsubpd,
    /// Instruction 'vfmaddsubps' {FMA4}.
    Vfmaddsubps,
    /// Instruction 'vfmsub132pd' {FMA|AVX512_F+VL}.
    Vfmsub132pd,
    /// Instruction 'vfmsub132ph' {AVX512_FP16+VL}.
    Vfmsub132ph,
    /// Instruction 'vfmsub132ps' {FMA|AVX512_F+VL}.
    Vfmsub132ps,
    /// Instruction 'vfmsub132sd' {FMA|AVX512_F+VL}.
    Vfmsub132sd,
    /// Instruction 'vfmsub132sh' {AVX512_FP16+VL}.
    Vfmsub132sh,
    /// Instruction 'vfmsub132ss' {FMA|AVX512_F+VL}.
    Vfmsub132ss,
    /// Instruction 'vfmsub213pd' {FMA|AVX512_F+VL}.
    Vfmsub213pd,
    /// Instruction 'vfmsub213ph' {AVX512_FP16+VL}.
    Vfmsub213ph,
    /// Instruction 'vfmsub213ps' {FMA|AVX512_F+VL}.
    Vfmsub213ps,
    /// Instruction 'vfmsub213sd' {FMA|AVX512_F+VL}.
    Vfmsub213sd,
    /// Instruction 'vfmsub213sh' {AVX512_FP16+VL}.
    Vfmsub213sh,
    /// Instruction 'vfmsub213ss' {FMA|AVX512_F+VL}.
    Vfmsub213ss,
    /// Instruction 'vfmsub231pd' {FMA|AVX512_F+VL}.
    Vfmsub231pd,
    /// Instruction 'vfmsub231ph' {AVX512_FP16+VL}.
    Vfmsub231ph,
    /// Instruction 'vfmsub231ps' {FMA|AVX512_F+VL}.
    Vfmsub231ps,
    /// Instruction 'vfmsub231sd' {FMA|AVX512_F+VL}.
    Vfmsub231sd,
    /// Instruction 'vfmsub231sh' {AVX512_FP16+VL}.
    Vfmsub231sh,
    /// Instruction 'vfmsub231ss' {FMA|AVX512_F+VL}.
    Vfmsub231ss,
    /// Instruction 'vfmsubadd132pd' {FMA|AVX512_F+VL}.
    Vfmsubadd132pd,
    /// Instruction 'vfmsubadd132ph' {AVX512_FP16+VL}.
    Vfmsubadd132ph,
    /// Instruction 'vfmsubadd132ps' {FMA|AVX512_F+VL}.
    Vfmsubadd132ps,
    /// Instruction 'vfmsubadd213pd' {FMA|AVX512_F+VL}.
    Vfmsubadd213pd,
    /// Instruction 'vfmsubadd213ph' {AVX512_FP16+VL}.
    Vfmsubadd213ph,
    /// Instruction 'vfmsubadd213ps' {FMA|AVX512_F+VL}.
    Vfmsubadd213ps,
    /// Instruction 'vfmsubadd231pd' {FMA|AVX512_F+VL}.
    Vfmsubadd231pd,
    /// Instruction 'vfmsubadd231ph' {AVX512_FP16+VL}.
    Vfmsubadd231ph,
    /// Instruction 'vfmsubadd231ps' {FMA|AVX512_F+VL}.
    Vfmsubadd231ps,
    /// Instruction 'vfmsubaddpd' {FMA4}.
    Vfmsubaddpd,
    /// Instruction 'vfmsubaddps' {FMA4}.
    Vfmsubaddps,
    /// Instruction 'vfmsubpd' {FMA4}.
    Vfmsubpd,
    /// Instruction 'vfmsubps' {FMA4}.
    Vfmsubps,
    /// Instruction 'vfmsubsd' {FMA4}.
    Vfmsubsd,
    /// Instruction 'vfmsubss' {FMA4}.
    Vfmsubss,
    /// Instruction 'vfmulcph' {AVX512_FP16+VL}.
    Vfmulcph,
    /// Instruction 'vfmulcsh' {AVX512_FP16+VL}.
    Vfmulcsh,
    /// Instruction 'vfnmadd132pd' {FMA|AVX512_F+VL}.
    Vfnmadd132pd,
    /// Instruction 'vfnmadd132ph' {AVX512_FP16+VL}.
    Vfnmadd132ph,
    /// Instruction 'vfnmadd132ps' {FMA|AVX512_F+VL}.
    Vfnmadd132ps,
    /// Instruction 'vfnmadd132sd' {FMA|AVX512_F+VL}.
    Vfnmadd132sd,
    /// Instruction 'vfnmadd132sh' {AVX512_FP16+VL}.
    Vfnmadd132sh,
    /// Instruction 'vfnmadd132ss' {FMA|AVX512_F+VL}.
    Vfnmadd132ss,
    /// Instruction 'vfnmadd213pd' {FMA|AVX512_F+VL}.
    Vfnmadd213pd,
    /// Instruction 'vfnmadd213ph' {AVX512_FP16+VL}.
    Vfnmadd213ph,
    /// Instruction 'vfnmadd213ps' {FMA|AVX512_F+VL}.
    Vfnmadd213ps,
    /// Instruction 'vfnmadd213sd' {FMA|AVX512_F+VL}.
    Vfnmadd213sd,
    /// Instruction 'vfnmadd213sh' {AVX512_FP16+VL}.
    Vfnmadd213sh,
    /// Instruction 'vfnmadd213ss' {FMA|AVX512_F+VL}.
    Vfnmadd213ss,
    /// Instruction 'vfnmadd231pd' {FMA|AVX512_F+VL}.
    Vfnmadd231pd,
    /// Instruction 'vfnmadd231ph' {AVX512_FP16+VL}.
    Vfnmadd231ph,
    /// Instruction 'vfnmadd231ps' {FMA|AVX512_F+VL}.
    Vfnmadd231ps,
    /// Instruction 'vfnmadd231sd' {FMA|AVX512_F+VL}.
    Vfnmadd231sd,
    /// Instruction 'vfnmadd231sh' {AVX512_FP16+VL}.
    Vfnmadd231sh,
    /// Instruction 'vfnmadd231ss' {FMA|AVX512_F+VL}.
    Vfnmadd231ss,
    /// Instruction 'vfnmaddpd' {FMA4}.
    Vfnmaddpd,
    /// Instruction 'vfnmaddps' {FMA4}.
    Vfnmaddps,
    /// Instruction 'vfnmaddsd' {FMA4}.
    Vfnmaddsd,
    /// Instruction 'vfnmaddss' {FMA4}.
    Vfnmaddss,
    /// Instruction 'vfnmsub132pd' {FMA|AVX512_F+VL}.
    Vfnmsub132pd,
    /// Instruction 'vfnmsub132ph' {AVX512_FP16+VL}.
    Vfnmsub132ph,
    /// Instruction 'vfnmsub132ps' {FMA|AVX512_F+VL}.
    Vfnmsub132ps,
    /// Instruction 'vfnmsub132sd' {FMA|AVX512_F+VL}.
    Vfnmsub132sd,
    /// Instruction 'vfnmsub132sh' {AVX512_FP16+VL}.
    Vfnmsub132sh,
    /// Instruction 'vfnmsub132ss' {FMA|AVX512_F+VL}.
    Vfnmsub132ss,
    /// Instruction 'vfnmsub213pd' {FMA|AVX512_F+VL}.
    Vfnmsub213pd,
    /// Instruction 'vfnmsub213ph' {AVX512_FP16+VL}.
    Vfnmsub213ph,
    /// Instruction 'vfnmsub213ps' {FMA|AVX512_F+VL}.
    Vfnmsub213ps,
    /// Instruction 'vfnmsub213sd' {FMA|AVX512_F+VL}.
    Vfnmsub213sd,
    /// Instruction 'vfnmsub213sh' {AVX512_FP16+VL}.
    Vfnmsub213sh,
    /// Instruction 'vfnmsub213ss' {FMA|AVX512_F+VL}.
    Vfnmsub213ss,
    /// Instruction 'vfnmsub231pd' {FMA|AVX512_F+VL}.
    Vfnmsub231pd,
    /// Instruction 'vfnmsub231ph' {AVX512_FP16+VL}.
    Vfnmsub231ph,
    /// Instruction 'vfnmsub231ps' {FMA|AVX512_F+VL}.
    Vfnmsub231ps,
    /// Instruction 'vfnmsub231sd' {FMA|AVX512_F+VL}.
    Vfnmsub231sd,
    /// Instruction 'vfnmsub231sh' {AVX512_FP16+VL}.
    Vfnmsub231sh,
    /// Instruction 'vfnmsub231ss' {FMA|AVX512_F+VL}.
    Vfnmsub231ss,
    /// Instruction 'vfnmsubpd' {FMA4}.
    Vfnmsubpd,
    /// Instruction 'vfnmsubps' {FMA4}.
    Vfnmsubps,
    /// Instruction 'vfnmsubsd' {FMA4}.
    Vfnmsubsd,
    /// Instruction 'vfnmsubss' {FMA4}.
    Vfnmsubss,
    /// Instruction 'vfpclasspd' {AVX512_DQ+VL}.
    Vfpclasspd,
    /// Instruction 'vfpclassph' {AVX512_FP16+VL}.
    Vfpclassph,
    /// Instruction 'vfpclassps' {AVX512_DQ+VL}.
    Vfpclassps,
    /// Instruction 'vfpclasssd' {AVX512_DQ+VL}.
    Vfpclasssd,
    /// Instruction 'vfpclasssh' {AVX512_FP16+VL}.
    Vfpclasssh,
    /// Instruction 'vfpclassss' {AVX512_DQ+VL}.
    Vfpclassss,
    /// Instruction 'vfrczpd' {XOP}.
    Vfrczpd,
    /// Instruction 'vfrczps' {XOP}.
    Vfrczps,
    /// Instruction 'vfrczsd' {XOP}.
    Vfrczsd,
    /// Instruction 'vfrczss' {XOP}.
    Vfrczss,
    /// Instruction 'vgatherdpd' {AVX2|AVX512_F+VL}.
    Vgatherdpd,
    /// Instruction 'vgatherdps' {AVX2|AVX512_F+VL}.
    Vgatherdps,
    /// Instruction 'vgatherqpd' {AVX2|AVX512_F+VL}.
    Vgatherqpd,
    /// Instruction 'vgatherqps' {AVX2|AVX512_F+VL}.
    Vgatherqps,
    /// Instruction 'vgetexppd' {AVX512_F+VL}.
    Vgetexppd,
    /// Instruction 'vgetexpph' {AVX512_FP16+VL}.
    Vgetexpph,
    /// Instruction 'vgetexpps' {AVX512_F+VL}.
    Vgetexpps,
    /// Instruction 'vgetexpsd' {AVX512_F+VL}.
    Vgetexpsd,
    /// Instruction 'vgetexpsh' {AVX512_FP16+VL}.
    Vgetexpsh,
    /// Instruction 'vgetexpss' {AVX512_F+VL}.
    Vgetexpss,
    /// Instruction 'vgetmantpd' {AVX512_F+VL}.
    Vgetmantpd,
    /// Instruction 'vgetmantph' {AVX512_FP16+VL}.
    Vgetmantph,
    /// Instruction 'vgetmantps' {AVX512_F+VL}.
    Vgetmantps,
    /// Instruction 'vgetmantsd' {AVX512_F+VL}.
    Vgetmantsd,
    /// Instruction 'vgetmantsh' {AVX512_FP16+VL}.
    Vgetmantsh,
    /// Instruction 'vgetmantss' {AVX512_F+VL}.
    Vgetmantss,
    /// Instruction 'vgf2p8affineinvqb' {AVX|AVX512_F+VL & GFNI}.
    Vgf2p8affineinvqb,
    /// Instruction 'vgf2p8affineqb' {AVX|AVX512_F+VL & GFNI}.
    Vgf2p8affineqb,
    /// Instruction 'vgf2p8mulb' {AVX|AVX512_F+VL & GFNI}.
    Vgf2p8mulb,
    /// Instruction 'vhaddpd' {AVX}.
    Vhaddpd,
    /// Instruction 'vhaddps' {AVX}.
    Vhaddps,
    /// Instruction 'vhsubpd' {AVX}.
    Vhsubpd,
    /// Instruction 'vhsubps' {AVX}.
    Vhsubps,
    /// Instruction 'vinsertf128' {AVX}.
    Vinsertf128,
    /// Instruction 'vinsertf32x4' {AVX512_F+VL}.
    Vinsertf32x4,
    /// Instruction 'vinsertf32x8' {AVX512_DQ+VL}.
    Vinsertf32x8,
    /// Instruction 'vinsertf64x2' {AVX512_DQ+VL}.
    Vinsertf64x2,
    /// Instruction 'vinsertf64x4' {AVX512_F+VL}.
    Vinsertf64x4,
    /// Instruction 'vinserti128' {AVX2}.
    Vinserti128,
    /// Instruction 'vinserti32x4' {AVX512_F+VL}.
    Vinserti32x4,
    /// Instruction 'vinserti32x8' {AVX512_DQ+VL}.
    Vinserti32x8,
    /// Instruction 'vinserti64x2' {AVX512_DQ+VL}.
    Vinserti64x2,
    /// Instruction 'vinserti64x4' {AVX512_F+VL}.
    Vinserti64x4,
    /// Instruction 'vinsertps' {AVX|AVX512_F+VL}.
    Vinsertps,
    /// Instruction 'vlddqu' {AVX}.
    Vlddqu,
    /// Instruction 'vldmxcsr' {AVX}.
    Vldmxcsr,
    /// Instruction 'vmaskmovdqu' {AVX}.
    Vmaskmovdqu,
    /// Instruction 'vmaskmovpd' {AVX}.
    Vmaskmovpd,
    /// Instruction 'vmaskmovps' {AVX}.
    Vmaskmovps,
    /// Instruction 'vmaxpd' {AVX|AVX512_F+VL}.
    Vmaxpd,
    /// Instruction 'vmaxph' {AVX512_FP16+VL}.
    Vmaxph,
    /// Instruction 'vmaxps' {AVX|AVX512_F+VL}.
    Vmaxps,
    /// Instruction 'vmaxsd' {AVX|AVX512_F+VL}.
    Vmaxsd,
    /// Instruction 'vmaxsh' {AVX512_FP16+VL}.
    Vmaxsh,
    /// Instruction 'vmaxss' {AVX|AVX512_F+VL}.
    Vmaxss,
    /// Instruction 'vmcall' {VMX}.
    Vmcall,
    /// Instruction 'vmclear' {VMX}.
    Vmclear,
    /// Instruction 'vmfunc' {VMX}.
    Vmfunc,
    /// Instruction 'vmgexit' {SEV_ES}.
    Vmgexit,
    /// Instruction 'vminpd' {AVX|AVX512_F+VL}.
    Vminpd,
    /// Instruction 'vminph' {AVX512_FP16+VL}.
    Vminph,
    /// Instruction 'vminps' {AVX|AVX512_F+VL}.
    Vminps,
    /// Instruction 'vminsd' {AVX|AVX512_F+VL}.
    Vminsd,
    /// Instruction 'vminsh' {AVX512_FP16+VL}.
    Vminsh,
    /// Instruction 'vminss' {AVX|AVX512_F+VL}.
    Vminss,
    /// Instruction 'vmlaunch' {VMX}.
    Vmlaunch,
    /// Instruction 'vmload' {SVM}.
    Vmload,
    /// Instruction 'vmmcall' {SVM}.
    Vmmcall,
    /// Instruction 'vmovapd' {AVX|AVX512_F+VL}.
    Vmovapd,
    /// Instruction 'vmovaps' {AVX|AVX512_F+VL}.
    Vmovaps,
    /// Instruction 'vmovd' {AVX|AVX512_F+VL}.
    Vmovd,
    /// Instruction 'vmovddup' {AVX|AVX512_F+VL}.
    Vmovddup,
    /// Instruction 'vmovdqa' {AVX}.
    Vmovdqa,
    /// Instruction 'vmovdqa32' {AVX512_F+VL}.
    Vmovdqa32,
    /// Instruction 'vmovdqa64' {AVX512_F+VL}.
    Vmovdqa64,
    /// Instruction 'vmovdqu' {AVX}.
    Vmovdqu,
    /// Instruction 'vmovdqu16' {AVX512_BW+VL}.
    Vmovdqu16,
    /// Instruction 'vmovdqu32' {AVX512_F+VL}.
    Vmovdqu32,
    /// Instruction 'vmovdqu64' {AVX512_F+VL}.
    Vmovdqu64,
    /// Instruction 'vmovdqu8' {AVX512_BW+VL}.
    Vmovdqu8,
    /// Instruction 'vmovhlps' {AVX|AVX512_F+VL}.
    Vmovhlps,
    /// Instruction 'vmovhpd' {AVX|AVX512_F+VL}.
    Vmovhpd,
    /// Instruction 'vmovhps' {AVX|AVX512_F+VL}.
    Vmovhps,
    /// Instruction 'vmovlhps' {AVX|AVX512_F+VL}.
    Vmovlhps,
    /// Instruction 'vmovlpd' {AVX|AVX512_F+VL}.
    Vmovlpd,
    /// Instruction 'vmovlps' {AVX|AVX512_F+VL}.
    Vmovlps,
    /// Instruction 'vmovmskpd' {AVX}.
    Vmovmskpd,
    /// Instruction 'vmovmskps' {AVX}.
    Vmovmskps,
    /// Instruction 'vmovntdq' {AVX|AVX512_F+VL}.
    Vmovntdq,
    /// Instruction 'vmovntdqa' {AVX|AVX2|AVX512_F+VL}.
    Vmovntdqa,
    /// Instruction 'vmovntpd' {AVX|AVX512_F+VL}.
    Vmovntpd,
    /// Instruction 'vmovntps' {AVX|AVX512_F+VL}.
    Vmovntps,
    /// Instruction 'vmovq' {AVX|AVX512_F+VL}.
    Vmovq,
    /// Instruction 'vmovsd' {AVX|AVX512_F+VL}.
    Vmovsd,
    /// Instruction 'vmovsh' {AVX512_FP16+VL}.
    Vmovsh,
    /// Instruction 'vmovshdup' {AVX|AVX512_F+VL}.
    Vmovshdup,
    /// Instruction 'vmovsldup' {AVX|AVX512_F+VL}.
    Vmovsldup,
    /// Instruction 'vmovss' {AVX|AVX512_F+VL}.
    Vmovss,
    /// Instruction 'vmovupd' {AVX|AVX512_F+VL}.
    Vmovupd,
    /// Instruction 'vmovups' {AVX|AVX512_F+VL}.
    Vmovups,
    /// Instruction 'vmovw' {AVX512_FP16+VL}.
    Vmovw,
    /// Instruction 'vmpsadbw' {AVX|AVX2}.
    Vmpsadbw,
    /// Instruction 'vmptrld' {VMX}.
    Vmptrld,
    /// Instruction 'vmptrst' {VMX}.
    Vmptrst,
    /// Instruction 'vmread' {VMX}.
    Vmread,
    /// Instruction 'vmresume' {VMX}.
    Vmresume,
    /// Instruction 'vmrun' {SVM}.
    Vmrun,
    /// Instruction 'vmsave' {SVM}.
    Vmsave,
    /// Instruction 'vmulpd' {AVX|AVX512_F+VL}.
    Vmulpd,
    /// Instruction 'vmulph' {AVX512_FP16+VL}.
    Vmulph,
    /// Instruction 'vmulps' {AVX|AVX512_F+VL}.
    Vmulps,
    /// Instruction 'vmulsd' {AVX|AVX512_F+VL}.
    Vmulsd,
    /// Instruction 'vmulsh' {AVX512_FP16+VL}.
    Vmulsh,
    /// Instruction 'vmulss' {AVX|AVX512_F+VL}.
    Vmulss,
    /// Instruction 'vmwrite' {VMX}.
    Vmwrite,
    /// Instruction 'vmxoff' {VMX}.
    Vmxoff,
    /// Instruction 'vmxon' {VMX}.
    Vmxon,
    /// Instruction 'vorpd' {AVX|AVX512_DQ+VL}.
    Vorpd,
    /// Instruction 'vorps' {AVX|AVX512_DQ+VL}.
    Vorps,
    /// Instruction 'vp2intersectd' {AVX512_VP2INTERSECT+VL}.
    Vp2intersectd,
    /// Instruction 'vp2intersectq' {AVX512_VP2INTERSECT+VL}.
    Vp2intersectq,
    /// Instruction 'vpabsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpabsb,
    /// Instruction 'vpabsd' {AVX|AVX2|AVX512_F+VL}.
    Vpabsd,
    /// Instruction 'vpabsq' {AVX512_F+VL}.
    Vpabsq,
    /// Instruction 'vpabsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpabsw,
    /// Instruction 'vpackssdw' {AVX|AVX2|AVX512_BW+VL}.
    Vpackssdw,
    /// Instruction 'vpacksswb' {AVX|AVX2|AVX512_BW+VL}.
    Vpacksswb,
    /// Instruction 'vpackusdw' {AVX|AVX2|AVX512_BW+VL}.
    Vpackusdw,
    /// Instruction 'vpackuswb' {AVX|AVX2|AVX512_BW+VL}.
    Vpackuswb,
    /// Instruction 'vpaddb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddb,
    /// Instruction 'vpaddd' {AVX|AVX2|AVX512_F+VL}.
    Vpaddd,
    /// Instruction 'vpaddq' {AVX|AVX2|AVX512_F+VL}.
    Vpaddq,
    /// Instruction 'vpaddsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddsb,
    /// Instruction 'vpaddsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddsw,
    /// Instruction 'vpaddusb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddusb,
    /// Instruction 'vpaddusw' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddusw,
    /// Instruction 'vpaddw' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddw,
    /// Instruction 'vpalignr' {AVX|AVX2|AVX512_BW+VL}.
    Vpalignr,
    /// Instruction 'vpand' {AVX|AVX2}.
    Vpand,
    /// Instruction 'vpandd' {AVX512_F+VL}.
    Vpandd,
    /// Instruction 'vpandn' {AVX|AVX2}.
    Vpandn,
    /// Instruction 'vpandnd' {AVX512_F+VL}.
    Vpandnd,
    /// Instruction 'vpandnq' {AVX512_F+VL}.
    Vpandnq,
    /// Instruction 'vpandq' {AVX512_F+VL}.
    Vpandq,
    /// Instruction 'vpavgb' {AVX|AVX2|AVX512_BW+VL}.
    Vpavgb,
    /// Instruction 'vpavgw' {AVX|AVX2|AVX512_BW+VL}.
    Vpavgw,
    /// Instruction 'vpblendd' {AVX2}.
    Vpblendd,
    /// Instruction 'vpblendmb' {AVX512_BW+VL}.
    Vpblendmb,
    /// Instruction 'vpblendmd' {AVX512_F+VL}.
    Vpblendmd,
    /// Instruction 'vpblendmq' {AVX512_F+VL}.
    Vpblendmq,
    /// Instruction 'vpblendmw' {AVX512_BW+VL}.
    Vpblendmw,
    /// Instruction 'vpblendvb' {AVX|AVX2}.
    Vpblendvb,
    /// Instruction 'vpblendw' {AVX|AVX2}.
    Vpblendw,
    /// Instruction 'vpbroadcastb' {AVX2|AVX512_BW+VL}.
    Vpbroadcastb,
    /// Instruction 'vpbroadcastd' {AVX2|AVX512_F+VL}.
    Vpbroadcastd,
    /// Instruction 'vpbroadcastmb2q' {AVX512_CD+VL}.
    Vpbroadcastmb2q,
    /// Instruction 'vpbroadcastmw2d' {AVX512_CD+VL}.
    Vpbroadcastmw2d,
    /// Instruction 'vpbroadcastq' {AVX2|AVX512_F+VL}.
    Vpbroadcastq,
    /// Instruction 'vpbroadcastw' {AVX2|AVX512_BW+VL}.
    Vpbroadcastw,
    /// Instruction 'vpclmulqdq' {AVX|AVX512_F+VL & PCLMULQDQ|VPCLMULQDQ}.
    Vpclmulqdq,
    /// Instruction 'vpcmov' {XOP}.
    Vpcmov,
    /// Instruction 'vpcmpb' {AVX512_BW+VL}.
    Vpcmpb,
    /// Instruction 'vpcmpd' {AVX512_F+VL}.
    Vpcmpd,
    /// Instruction 'vpcmpeqb' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpeqb,
    /// Instruction 'vpcmpeqd' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpeqd,
    /// Instruction 'vpcmpeqq' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpeqq,
    /// Instruction 'vpcmpeqw' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpeqw,
    /// Instruction 'vpcmpestri' {AVX}.
    Vpcmpestri,
    /// Instruction 'vpcmpestrm' {AVX}.
    Vpcmpestrm,
    /// Instruction 'vpcmpgtb' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpgtb,
    /// Instruction 'vpcmpgtd' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpgtd,
    /// Instruction 'vpcmpgtq' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpgtq,
    /// Instruction 'vpcmpgtw' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpgtw,
    /// Instruction 'vpcmpistri' {AVX}.
    Vpcmpistri,
    /// Instruction 'vpcmpistrm' {AVX}.
    Vpcmpistrm,
    /// Instruction 'vpcmpq' {AVX512_F+VL}.
    Vpcmpq,
    /// Instruction 'vpcmpub' {AVX512_BW+VL}.
    Vpcmpub,
    /// Instruction 'vpcmpud' {AVX512_F+VL}.
    Vpcmpud,
    /// Instruction 'vpcmpuq' {AVX512_F+VL}.
    Vpcmpuq,
    /// Instruction 'vpcmpuw' {AVX512_BW+VL}.
    Vpcmpuw,
    /// Instruction 'vpcmpw' {AVX512_BW+VL}.
    Vpcmpw,
    /// Instruction 'vpcomb' {XOP}.
    Vpcomb,
    /// Instruction 'vpcomd' {XOP}.
    Vpcomd,
    /// Instruction 'vpcompressb' {AVX512_VBMI2+VL}.
    Vpcompressb,
    /// Instruction 'vpcompressd' {AVX512_F+VL}.
    Vpcompressd,
    /// Instruction 'vpcompressq' {AVX512_F+VL}.
    Vpcompressq,
    /// Instruction 'vpcompressw' {AVX512_VBMI2+VL}.
    Vpcompressw,
    /// Instruction 'vpcomq' {XOP}.
    Vpcomq,
    /// Instruction 'vpcomub' {XOP}.
    Vpcomub,
    /// Instruction 'vpcomud' {XOP}.
    Vpcomud,
    /// Instruction 'vpcomuq' {XOP}.
    Vpcomuq,
    /// Instruction 'vpcomuw' {XOP}.
    Vpcomuw,
    /// Instruction 'vpcomw' {XOP}.
    Vpcomw,
    /// Instruction 'vpconflictd' {AVX512_CD+VL}.
    Vpconflictd,
    /// Instruction 'vpconflictq' {AVX512_CD+VL}.
    Vpconflictq,
    /// Instruction 'vpdpbssd' {AVX_VNNI_INT8}.
    Vpdpbssd,
    /// Instruction 'vpdpbssds' {AVX_VNNI_INT8}.
    Vpdpbssds,
    /// Instruction 'vpdpbsud' {AVX_VNNI_INT8}.
    Vpdpbsud,
    /// Instruction 'vpdpbsuds' {AVX_VNNI_INT8}.
    Vpdpbsuds,
    /// Instruction 'vpdpbusd' {AVX_VNNI|AVX512_VNNI+VL}.
    Vpdpbusd,
    /// Instruction 'vpdpbusds' {AVX_VNNI|AVX512_VNNI+VL}.
    Vpdpbusds,
    /// Instruction 'vpdpbuud' {AVX_VNNI_INT8}.
    Vpdpbuud,
    /// Instruction 'vpdpbuuds' {AVX_VNNI_INT8}.
    Vpdpbuuds,
    /// Instruction 'vpdpwssd' {AVX_VNNI|AVX512_VNNI+VL}.
    Vpdpwssd,
    /// Instruction 'vpdpwssds' {AVX_VNNI|AVX512_VNNI+VL}.
    Vpdpwssds,
    /// Instruction 'vpdpwsud' {AVX_VNNI_INT16}.
    Vpdpwsud,
    /// Instruction 'vpdpwsuds' {AVX_VNNI_INT16}.
    Vpdpwsuds,
    /// Instruction 'vpdpwusd' {AVX_VNNI_INT16}.
    Vpdpwusd,
    /// Instruction 'vpdpwusds' {AVX_VNNI_INT16}.
    Vpdpwusds,
    /// Instruction 'vpdpwuud' {AVX_VNNI_INT16}.
    Vpdpwuud,
    /// Instruction 'vpdpwuuds' {AVX_VNNI_INT16}.
    Vpdpwuuds,
    /// Instruction 'vperm2f128' {AVX}.
    Vperm2f128,
    /// Instruction 'vperm2i128' {AVX2}.
    Vperm2i128,
    /// Instruction 'vpermb' {AVX512_VBMI+VL}.
    Vpermb,
    /// Instruction 'vpermd' {AVX2|AVX512_F+VL}.
    Vpermd,
    /// Instruction 'vpermi2b' {AVX512_VBMI+VL}.
    Vpermi2b,
    /// Instruction 'vpermi2d' {AVX512_F+VL}.
    Vpermi2d,
    /// Instruction 'vpermi2pd' {AVX512_F+VL}.
    Vpermi2pd,
    /// Instruction 'vpermi2ps' {AVX512_F+VL}.
    Vpermi2ps,
    /// Instruction 'vpermi2q' {AVX512_F+VL}.
    Vpermi2q,
    /// Instruction 'vpermi2w' {AVX512_BW+VL}.
    Vpermi2w,
    /// Instruction 'vpermil2pd' {XOP}.
    Vpermil2pd,
    /// Instruction 'vpermil2ps' {XOP}.
    Vpermil2ps,
    /// Instruction 'vpermilpd' {AVX|AVX512_F+VL}.
    Vpermilpd,
    /// Instruction 'vpermilps' {AVX|AVX512_F+VL}.
    Vpermilps,
    /// Instruction 'vpermpd' {AVX2|AVX512_F+VL}.
    Vpermpd,
    /// Instruction 'vpermps' {AVX2|AVX512_F+VL}.
    Vpermps,
    /// Instruction 'vpermq' {AVX2|AVX512_F+VL}.
    Vpermq,
    /// Instruction 'vpermt2b' {AVX512_VBMI+VL}.
    Vpermt2b,
    /// Instruction 'vpermt2d' {AVX512_F+VL}.
    Vpermt2d,
    /// Instruction 'vpermt2pd' {AVX512_F+VL}.
    Vpermt2pd,
    /// Instruction 'vpermt2ps' {AVX512_F+VL}.
    Vpermt2ps,
    /// Instruction 'vpermt2q' {AVX512_F+VL}.
    Vpermt2q,
    /// Instruction 'vpermt2w' {AVX512_BW+VL}.
    Vpermt2w,
    /// Instruction 'vpermw' {AVX512_BW+VL}.
    Vpermw,
    /// Instruction 'vpexpandb' {AVX512_VBMI2+VL}.
    Vpexpandb,
    /// Instruction 'vpexpandd' {AVX512_F+VL}.
    Vpexpandd,
    /// Instruction 'vpexpandq' {AVX512_F+VL}.
    Vpexpandq,
    /// Instruction 'vpexpandw' {AVX512_VBMI2+VL}.
    Vpexpandw,
    /// Instruction 'vpextrb' {AVX|AVX512_BW+VL}.
    Vpextrb,
    /// Instruction 'vpextrd' {AVX|AVX512_DQ+VL}.
    Vpextrd,
    /// Instruction 'vpextrq' {AVX|AVX512_DQ+VL} (X64).
    Vpextrq,
    /// Instruction 'vpextrw' {AVX|AVX512_BW+VL}.
    Vpextrw,
    /// Instruction 'vpgatherdd' {AVX2|AVX512_F+VL}.
    Vpgatherdd,
    /// Instruction 'vpgatherdq' {AVX2|AVX512_F+VL}.
    Vpgatherdq,
    /// Instruction 'vpgatherqd' {AVX2|AVX512_F+VL}.
    Vpgatherqd,
    /// Instruction 'vpgatherqq' {AVX2|AVX512_F+VL}.
    Vpgatherqq,
    /// Instruction 'vphaddbd' {XOP}.
    Vphaddbd,
    /// Instruction 'vphaddbq' {XOP}.
    Vphaddbq,
    /// Instruction 'vphaddbw' {XOP}.
    Vphaddbw,
    /// Instruction 'vphaddd' {AVX|AVX2}.
    Vphaddd,
    /// Instruction 'vphadddq' {XOP}.
    Vphadddq,
    /// Instruction 'vphaddsw' {AVX|AVX2}.
    Vphaddsw,
    /// Instruction 'vphaddubd' {XOP}.
    Vphaddubd,
    /// Instruction 'vphaddubq' {XOP}.
    Vphaddubq,
    /// Instruction 'vphaddubw' {XOP}.
    Vphaddubw,
    /// Instruction 'vphaddudq' {XOP}.
    Vphaddudq,
    /// Instruction 'vphadduwd' {XOP}.
    Vphadduwd,
    /// Instruction 'vphadduwq' {XOP}.
    Vphadduwq,
    /// Instruction 'vphaddw' {AVX|AVX2}.
    Vphaddw,
    /// Instruction 'vphaddwd' {XOP}.
    Vphaddwd,
    /// Instruction 'vphaddwq' {XOP}.
    Vphaddwq,
    /// Instruction 'vphminposuw' {AVX}.
    Vphminposuw,
    /// Instruction 'vphsubbw' {XOP}.
    Vphsubbw,
    /// Instruction 'vphsubd' {AVX|AVX2}.
    Vphsubd,
    /// Instruction 'vphsubdq' {XOP}.
    Vphsubdq,
    /// Instruction 'vphsubsw' {AVX|AVX2}.
    Vphsubsw,
    /// Instruction 'vphsubw' {AVX|AVX2}.
    Vphsubw,
    /// Instruction 'vphsubwd' {XOP}.
    Vphsubwd,
    /// Instruction 'vpinsrb' {AVX|AVX512_BW+VL}.
    Vpinsrb,
    /// Instruction 'vpinsrd' {AVX|AVX512_DQ+VL}.
    Vpinsrd,
    /// Instruction 'vpinsrq' {AVX|AVX512_DQ+VL} (X64).
    Vpinsrq,
    /// Instruction 'vpinsrw' {AVX|AVX512_BW+VL}.
    Vpinsrw,
    /// Instruction 'vplzcntd' {AVX512_CD+VL}.
    Vplzcntd,
    /// Instruction 'vplzcntq' {AVX512_CD+VL}.
    Vplzcntq,
    /// Instruction 'vpmacsdd' {XOP}.
    Vpmacsdd,
    /// Instruction 'vpmacsdqh' {XOP}.
    Vpmacsdqh,
    /// Instruction 'vpmacsdql' {XOP}.
    Vpmacsdql,
    /// Instruction 'vpmacssdd' {XOP}.
    Vpmacssdd,
    /// Instruction 'vpmacssdqh' {XOP}.
    Vpmacssdqh,
    /// Instruction 'vpmacssdql' {XOP}.
    Vpmacssdql,
    /// Instruction 'vpmacsswd' {XOP}.
    Vpmacsswd,
    /// Instruction 'vpmacssww' {XOP}.
    Vpmacssww,
    /// Instruction 'vpmacswd' {XOP}.
    Vpmacswd,
    /// Instruction 'vpmacsww' {XOP}.
    Vpmacsww,
    /// Instruction 'vpmadcsswd' {XOP}.
    Vpmadcsswd,
    /// Instruction 'vpmadcswd' {XOP}.
    Vpmadcswd,
    /// Instruction 'vpmadd52huq' {AVX_IFMA|AVX512_IFMA+VL}.
    Vpmadd52huq,
    /// Instruction 'vpmadd52luq' {AVX_IFMA|AVX512_IFMA+VL}.
    Vpmadd52luq,
    /// Instruction 'vpmaddubsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaddubsw,
    /// Instruction 'vpmaddwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaddwd,
    /// Instruction 'vpmaskmovd' {AVX2}.
    Vpmaskmovd,
    /// Instruction 'vpmaskmovq' {AVX2}.
    Vpmaskmovq,
    /// Instruction 'vpmaxsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxsb,
    /// Instruction 'vpmaxsd' {AVX|AVX2|AVX512_F+VL}.
    Vpmaxsd,
    /// Instruction 'vpmaxsq' {AVX512_F+VL}.
    Vpmaxsq,
    /// Instruction 'vpmaxsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxsw,
    /// Instruction 'vpmaxub' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxub,
    /// Instruction 'vpmaxud' {AVX|AVX2|AVX512_F+VL}.
    Vpmaxud,
    /// Instruction 'vpmaxuq' {AVX512_F+VL}.
    Vpmaxuq,
    /// Instruction 'vpmaxuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxuw,
    /// Instruction 'vpminsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpminsb,
    /// Instruction 'vpminsd' {AVX|AVX2|AVX512_F+VL}.
    Vpminsd,
    /// Instruction 'vpminsq' {AVX512_F+VL}.
    Vpminsq,
    /// Instruction 'vpminsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpminsw,
    /// Instruction 'vpminub' {AVX|AVX2|AVX512_BW+VL}.
    Vpminub,
    /// Instruction 'vpminud' {AVX|AVX2|AVX512_F+VL}.
    Vpminud,
    /// Instruction 'vpminuq' {AVX512_F+VL}.
    Vpminuq,
    /// Instruction 'vpminuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpminuw,
    /// Instruction 'vpmovb2m' {AVX512_BW+VL}.
    Vpmovb2m,
    /// Instruction 'vpmovd2m' {AVX512_DQ+VL}.
    Vpmovd2m,
    /// Instruction 'vpmovdb' {AVX512_F+VL}.
    Vpmovdb,
    /// Instruction 'vpmovdw' {AVX512_F+VL}.
    Vpmovdw,
    /// Instruction 'vpmovm2b' {AVX512_BW+VL}.
    Vpmovm2b,
    /// Instruction 'vpmovm2d' {AVX512_DQ+VL}.
    Vpmovm2d,
    /// Instruction 'vpmovm2q' {AVX512_DQ+VL}.
    Vpmovm2q,
    /// Instruction 'vpmovm2w' {AVX512_BW+VL}.
    Vpmovm2w,
    /// Instruction 'vpmovmskb' {AVX|AVX2}.
    Vpmovmskb,
    /// Instruction 'vpmovq2m' {AVX512_DQ+VL}.
    Vpmovq2m,
    /// Instruction 'vpmovqb' {AVX512_F+VL}.
    Vpmovqb,
    /// Instruction 'vpmovqd' {AVX512_F+VL}.
    Vpmovqd,
    /// Instruction 'vpmovqw' {AVX512_F+VL}.
    Vpmovqw,
    /// Instruction 'vpmovsdb' {AVX512_F+VL}.
    Vpmovsdb,
    /// Instruction 'vpmovsdw' {AVX512_F+VL}.
    Vpmovsdw,
    /// Instruction 'vpmovsqb' {AVX512_F+VL}.
    Vpmovsqb,
    /// Instruction 'vpmovsqd' {AVX512_F+VL}.
    Vpmovsqd,
    /// Instruction 'vpmovsqw' {AVX512_F+VL}.
    Vpmovsqw,
    /// Instruction 'vpmovswb' {AVX512_BW+VL}.
    Vpmovswb,
    /// Instruction 'vpmovsxbd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxbd,
    /// Instruction 'vpmovsxbq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxbq,
    /// Instruction 'vpmovsxbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmovsxbw,
    /// Instruction 'vpmovsxdq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxdq,
    /// Instruction 'vpmovsxwd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxwd,
    /// Instruction 'vpmovsxwq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxwq,
    /// Instruction 'vpmovusdb' {AVX512_F+VL}.
    Vpmovusdb,
    /// Instruction 'vpmovusdw' {AVX512_F+VL}.
    Vpmovusdw,
    /// Instruction 'vpmovusqb' {AVX512_F+VL}.
    Vpmovusqb,
    /// Instruction 'vpmovusqd' {AVX512_F+VL}.
    Vpmovusqd,
    /// Instruction 'vpmovusqw' {AVX512_F+VL}.
    Vpmovusqw,
    /// Instruction 'vpmovuswb' {AVX512_BW+VL}.
    Vpmovuswb,
    /// Instruction 'vpmovw2m' {AVX512_BW+VL}.
    Vpmovw2m,
    /// Instruction 'vpmovwb' {AVX512_BW+VL}.
    Vpmovwb,
    /// Instruction 'vpmovzxbd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxbd,
    /// Instruction 'vpmovzxbq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxbq,
    /// Instruction 'vpmovzxbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmovzxbw,
    /// Instruction 'vpmovzxdq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxdq,
    /// Instruction 'vpmovzxwd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxwd,
    /// Instruction 'vpmovzxwq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxwq,
    /// Instruction 'vpmuldq' {AVX|AVX2|AVX512_F+VL}.
    Vpmuldq,
    /// Instruction 'vpmulhrsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulhrsw,
    /// Instruction 'vpmulhuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulhuw,
    /// Instruction 'vpmulhw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulhw,
    /// Instruction 'vpmulld' {AVX|AVX2|AVX512_F+VL}.
    Vpmulld,
    /// Instruction 'vpmullq' {AVX512_DQ+VL}.
    Vpmullq,
    /// Instruction 'vpmullw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmullw,
    /// Instruction 'vpmultishiftqb' {AVX512_VBMI+VL}.
    Vpmultishiftqb,
    /// Instruction 'vpmuludq' {AVX|AVX2|AVX512_F+VL}.
    Vpmuludq,
    /// Instruction 'vpopcntb' {AVX512_BITALG+VL}.
    Vpopcntb,
    /// Instruction 'vpopcntd' {AVX512_VPOPCNTDQ+VL}.
    Vpopcntd,
    /// Instruction 'vpopcntq' {AVX512_VPOPCNTDQ+VL}.
    Vpopcntq,
    /// Instruction 'vpopcntw' {AVX512_BITALG+VL}.
    Vpopcntw,
    /// Instruction 'vpor' {AVX|AVX2}.
    Vpor,
    /// Instruction 'vpord' {AVX512_F+VL}.
    Vpord,
    /// Instruction 'vporq' {AVX512_F+VL}.
    Vporq,
    /// Instruction 'vpperm' {XOP}.
    Vpperm,
    /// Instruction 'vprold' {AVX512_F+VL}.
    Vprold,
    /// Instruction 'vprolq' {AVX512_F+VL}.
    Vprolq,
    /// Instruction 'vprolvd' {AVX512_F+VL}.
    Vprolvd,
    /// Instruction 'vprolvq' {AVX512_F+VL}.
    Vprolvq,
    /// Instruction 'vprord' {AVX512_F+VL}.
    Vprord,
    /// Instruction 'vprorq' {AVX512_F+VL}.
    Vprorq,
    /// Instruction 'vprorvd' {AVX512_F+VL}.
    Vprorvd,
    /// Instruction 'vprorvq' {AVX512_F+VL}.
    Vprorvq,
    /// Instruction 'vprotb' {XOP}.
    Vprotb,
    /// Instruction 'vprotd' {XOP}.
    Vprotd,
    /// Instruction 'vprotq' {XOP}.
    Vprotq,
    /// Instruction 'vprotw' {XOP}.
    Vprotw,
    /// Instruction 'vpsadbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsadbw,
    /// Instruction 'vpscatterdd' {AVX512_F+VL}.
    Vpscatterdd,
    /// Instruction 'vpscatterdq' {AVX512_F+VL}.
    Vpscatterdq,
    /// Instruction 'vpscatterqd' {AVX512_F+VL}.
    Vpscatterqd,
    /// Instruction 'vpscatterqq' {AVX512_F+VL}.
    Vpscatterqq,
    /// Instruction 'vpshab' {XOP}.
    Vpshab,
    /// Instruction 'vpshad' {XOP}.
    Vpshad,
    /// Instruction 'vpshaq' {XOP}.
    Vpshaq,
    /// Instruction 'vpshaw' {XOP}.
    Vpshaw,
    /// Instruction 'vpshlb' {XOP}.
    Vpshlb,
    /// Instruction 'vpshld' {XOP}.
    Vpshld,
    /// Instruction 'vpshldd' {AVX512_VBMI2+VL}.
    Vpshldd,
    /// Instruction 'vpshldq' {AVX512_VBMI2+VL}.
    Vpshldq,
    /// Instruction 'vpshldvd' {AVX512_VBMI2+VL}.
    Vpshldvd,
    /// Instruction 'vpshldvq' {AVX512_VBMI2+VL}.
    Vpshldvq,
    /// Instruction 'vpshldvw' {AVX512_VBMI2+VL}.
    Vpshldvw,
    /// Instruction 'vpshldw' {AVX512_VBMI2+VL}.
    Vpshldw,
    /// Instruction 'vpshlq' {XOP}.
    Vpshlq,
    /// Instruction 'vpshlw' {XOP}.
    Vpshlw,
    /// Instruction 'vpshrdd' {AVX512_VBMI2+VL}.
    Vpshrdd,
    /// Instruction 'vpshrdq' {AVX512_VBMI2+VL}.
    Vpshrdq,
    /// Instruction 'vpshrdvd' {AVX512_VBMI2+VL}.
    Vpshrdvd,
    /// Instruction 'vpshrdvq' {AVX512_VBMI2+VL}.
    Vpshrdvq,
    /// Instruction 'vpshrdvw' {AVX512_VBMI2+VL}.
    Vpshrdvw,
    /// Instruction 'vpshrdw' {AVX512_VBMI2+VL}.
    Vpshrdw,
    /// Instruction 'vpshufb' {AVX|AVX2|AVX512_BW+VL}.
    Vpshufb,
    /// Instruction 'vpshufbitqmb' {AVX512_BITALG+VL}.
    Vpshufbitqmb,
    /// Instruction 'vpshufd' {AVX|AVX2|AVX512_F+VL}.
    Vpshufd,
    /// Instruction 'vpshufhw' {AVX|AVX2|AVX512_BW+VL}.
    Vpshufhw,
    /// Instruction 'vpshuflw' {AVX|AVX2|AVX512_BW+VL}.
    Vpshuflw,
    /// Instruction 'vpsignb' {AVX|AVX2}.
    Vpsignb,
    /// Instruction 'vpsignd' {AVX|AVX2}.
    Vpsignd,
    /// Instruction 'vpsignw' {AVX|AVX2}.
    Vpsignw,
    /// Instruction 'vpslld' {AVX|AVX2|AVX512_F+VL}.
    Vpslld,
    /// Instruction 'vpslldq' {AVX|AVX2|AVX512_BW+VL}.
    Vpslldq,
    /// Instruction 'vpsllq' {AVX|AVX2|AVX512_F+VL}.
    Vpsllq,
    /// Instruction 'vpsllvd' {AVX2|AVX512_F+VL}.
    Vpsllvd,
    /// Instruction 'vpsllvq' {AVX2|AVX512_F+VL}.
    Vpsllvq,
    /// Instruction 'vpsllvw' {AVX512_BW+VL}.
    Vpsllvw,
    /// Instruction 'vpsllw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsllw,
    /// Instruction 'vpsrad' {AVX|AVX2|AVX512_F+VL}.
    Vpsrad,
    /// Instruction 'vpsraq' {AVX512_F+VL}.
    Vpsraq,
    /// Instruction 'vpsravd' {AVX2|AVX512_F+VL}.
    Vpsravd,
    /// Instruction 'vpsravq' {AVX512_F+VL}.
    Vpsravq,
    /// Instruction 'vpsravw' {AVX512_BW+VL}.
    Vpsravw,
    /// Instruction 'vpsraw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsraw,
    /// Instruction 'vpsrld' {AVX|AVX2|AVX512_F+VL}.
    Vpsrld,
    /// Instruction 'vpsrldq' {AVX|AVX2|AVX512_BW+VL}.
    Vpsrldq,
    /// Instruction 'vpsrlq' {AVX|AVX2|AVX512_F+VL}.
    Vpsrlq,
    /// Instruction 'vpsrlvd' {AVX2|AVX512_F+VL}.
    Vpsrlvd,
    /// Instruction 'vpsrlvq' {AVX2|AVX512_F+VL}.
    Vpsrlvq,
    /// Instruction 'vpsrlvw' {AVX512_BW+VL}.
    Vpsrlvw,
    /// Instruction 'vpsrlw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsrlw,
    /// Instruction 'vpsubb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubb,
    /// Instruction 'vpsubd' {AVX|AVX2|AVX512_F+VL}.
    Vpsubd,
    /// Instruction 'vpsubq' {AVX|AVX2|AVX512_F+VL}.
    Vpsubq,
    /// Instruction 'vpsubsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubsb,
    /// Instruction 'vpsubsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubsw,
    /// Instruction 'vpsubusb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubusb,
    /// Instruction 'vpsubusw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubusw,
    /// Instruction 'vpsubw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubw,
    /// Instruction 'vpternlogd' {AVX512_F+VL}.
    Vpternlogd,
    /// Instruction 'vpternlogq' {AVX512_F+VL}.
    Vpternlogq,
    /// Instruction 'vptest' {AVX}.
    Vptest,
    /// Instruction 'vptestmb' {AVX512_BW+VL}.
    Vptestmb,
    /// Instruction 'vptestmd' {AVX512_F+VL}.
    Vptestmd,
    /// Instruction 'vptestmq' {AVX512_F+VL}.
    Vptestmq,
    /// Instruction 'vptestmw' {AVX512_BW+VL}.
    Vptestmw,
    /// Instruction 'vptestnmb' {AVX512_BW+VL}.
    Vptestnmb,
    /// Instruction 'vptestnmd' {AVX512_F+VL}.
    Vptestnmd,
    /// Instruction 'vptestnmq' {AVX512_F+VL}.
    Vptestnmq,
    /// Instruction 'vptestnmw' {AVX512_BW+VL}.
    Vptestnmw,
    /// Instruction 'vpunpckhbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpckhbw,
    /// Instruction 'vpunpckhdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpckhdq,
    /// Instruction 'vpunpckhqdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpckhqdq,
    /// Instruction 'vpunpckhwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpckhwd,
    /// Instruction 'vpunpcklbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpcklbw,
    /// Instruction 'vpunpckldq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpckldq,
    /// Instruction 'vpunpcklqdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpcklqdq,
    /// Instruction 'vpunpcklwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpcklwd,
    /// Instruction 'vpxor' {AVX|AVX2}.
    Vpxor,
    /// Instruction 'vpxord' {AVX512_F+VL}.
    Vpxord,
    /// Instruction 'vpxorq' {AVX512_F+VL}.
    Vpxorq,
    /// Instruction 'vrangepd' {AVX512_DQ+VL}.
    Vrangepd,
    /// Instruction 'vrangeps' {AVX512_DQ+VL}.
    Vrangeps,
    /// Instruction 'vrangesd' {AVX512_DQ+VL}.
    Vrangesd,
    /// Instruction 'vrangess' {AVX512_DQ+VL}.
    Vrangess,
    /// Instruction 'vrcp14pd' {AVX512_F+VL}.
    Vrcp14pd,
    /// Instruction 'vrcp14ps' {AVX512_F+VL}.
    Vrcp14ps,
    /// Instruction 'vrcp14sd' {AVX512_F+VL}.
    Vrcp14sd,
    /// Instruction 'vrcp14ss' {AVX512_F+VL}.
    Vrcp14ss,
    /// Instruction 'vrcpph' {AVX512_FP16+VL}.
    Vrcpph,
    /// Instruction 'vrcpps' {AVX}.
    Vrcpps,
    /// Instruction 'vrcpsh' {AVX512_FP16+VL}.
    Vrcpsh,
    /// Instruction 'vrcpss' {AVX}.
    Vrcpss,
    /// Instruction 'vreducepd' {AVX512_DQ+VL}.
    Vreducepd,
    /// Instruction 'vreduceph' {AVX512_FP16+VL}.
    Vreduceph,
    /// Instruction 'vreduceps' {AVX512_DQ+VL}.
    Vreduceps,
    /// Instruction 'vreducesd' {AVX512_DQ+VL}.
    Vreducesd,
    /// Instruction 'vreducesh' {AVX512_FP16+VL}.
    Vreducesh,
    /// Instruction 'vreducess' {AVX512_DQ+VL}.
    Vreducess,
    /// Instruction 'vrndscalepd' {AVX512_F+VL}.
    Vrndscalepd,
    /// Instruction 'vrndscaleph' {AVX512_FP16+VL}.
    Vrndscaleph,
    /// Instruction 'vrndscaleps' {AVX512_F+VL}.
    Vrndscaleps,
    /// Instruction 'vrndscalesd' {AVX512_F+VL}.
    Vrndscalesd,
    /// Instruction 'vrndscalesh' {AVX512_FP16+VL}.
    Vrndscalesh,
    /// Instruction 'vrndscaless' {AVX512_F+VL}.
    Vrndscaless,
    /// Instruction 'vroundpd' {AVX}.
    Vroundpd,
    /// Instruction 'vroundps' {AVX}.
    Vroundps,
    /// Instruction 'vroundsd' {AVX}.
    Vroundsd,
    /// Instruction 'vroundss' {AVX}.
    Vroundss,
    /// Instruction 'vrsqrt14pd' {AVX512_F+VL}.
    Vrsqrt14pd,
    /// Instruction 'vrsqrt14ps' {AVX512_F+VL}.
    Vrsqrt14ps,
    /// Instruction 'vrsqrt14sd' {AVX512_F+VL}.
    Vrsqrt14sd,
    /// Instruction 'vrsqrt14ss' {AVX512_F+VL}.
    Vrsqrt14ss,
    /// Instruction 'vrsqrtph' {AVX512_FP16+VL}.
    Vrsqrtph,
    /// Instruction 'vrsqrtps' {AVX}.
    Vrsqrtps,
    /// Instruction 'vrsqrtsh' {AVX512_FP16+VL}.
    Vrsqrtsh,
    /// Instruction 'vrsqrtss' {AVX}.
    Vrsqrtss,
    /// Instruction 'vscalefpd' {AVX512_F+VL}.
    Vscalefpd,
    /// Instruction 'vscalefph' {AVX512_FP16+VL}.
    Vscalefph,
    /// Instruction 'vscalefps' {AVX512_F+VL}.
    Vscalefps,
    /// Instruction 'vscalefsd' {AVX512_F+VL}.
    Vscalefsd,
    /// Instruction 'vscalefsh' {AVX512_FP16+VL}.
    Vscalefsh,
    /// Instruction 'vscalefss' {AVX512_F+VL}.
    Vscalefss,
    /// Instruction 'vscatterdpd' {AVX512_F+VL}.
    Vscatterdpd,
    /// Instruction 'vscatterdps' {AVX512_F+VL}.
    Vscatterdps,
    /// Instruction 'vscatterqpd' {AVX512_F+VL}.
    Vscatterqpd,
    /// Instruction 'vscatterqps' {AVX512_F+VL}.
    Vscatterqps,
    /// Instruction 'vsha512msg1' {AVX & SHA512}.
    Vsha512msg1,
    /// Instruction 'vsha512msg2' {AVX & SHA512}.
    Vsha512msg2,
    /// Instruction 'vsha512rnds2' {AVX & SHA512}.
    Vsha512rnds2,
    /// Instruction 'vshuff32x4' {AVX512_F+VL}.
    Vshuff32x4,
    /// Instruction 'vshuff64x2' {AVX512_F+VL}.
    Vshuff64x2,
    /// Instruction 'vshufi32x4' {AVX512_F+VL}.
    Vshufi32x4,
    /// Instruction 'vshufi64x2' {AVX512_F+VL}.
    Vshufi64x2,
    /// Instruction 'vshufpd' {AVX|AVX512_F+VL}.
    Vshufpd,
    /// Instruction 'vshufps' {AVX|AVX512_F+VL}.
    Vshufps,
    /// Instruction 'vsm3msg1' {AVX & SM3}.
    Vsm3msg1,
    /// Instruction 'vsm3msg2' {AVX & SM3}.
    Vsm3msg2,
    /// Instruction 'vsm3rnds2' {AVX & SM3}.
    Vsm3rnds2,
    /// Instruction 'vsm4key4' {AVX & SM4}.
    Vsm4key4,
    /// Instruction 'vsm4rnds4' {AVX & SM4}.
    Vsm4rnds4,
    /// Instruction 'vsqrtpd' {AVX|AVX512_F+VL}.
    Vsqrtpd,
    /// Instruction 'vsqrtph' {AVX512_FP16+VL}.
    Vsqrtph,
    /// Instruction 'vsqrtps' {AVX|AVX512_F+VL}.
    Vsqrtps,
    /// Instruction 'vsqrtsd' {AVX|AVX512_F+VL}.
    Vsqrtsd,
    /// Instruction 'vsqrtsh' {AVX512_FP16+VL}.
    Vsqrtsh,
    /// Instruction 'vsqrtss' {AVX|AVX512_F+VL}.
    Vsqrtss,
    /// Instruction 'vstmxcsr' {AVX}.
    Vstmxcsr,
    /// Instruction 'vsubpd' {AVX|AVX512_F+VL}.
    Vsubpd,
    /// Instruction 'vsubph' {AVX512_FP16+VL}.
    Vsubph,
    /// Instruction 'vsubps' {AVX|AVX512_F+VL}.
    Vsubps,
    /// Instruction 'vsubsd' {AVX|AVX512_F+VL}.
    Vsubsd,
    /// Instruction 'vsubsh' {AVX512_FP16+VL}.
    Vsubsh,
    /// Instruction 'vsubss' {AVX|AVX512_F+VL}.
    Vsubss,
    /// Instruction 'vtestpd' {AVX}.
    Vtestpd,
    /// Instruction 'vtestps' {AVX}.
    Vtestps,
    /// Instruction 'vucomisd' {AVX|AVX512_F+VL}.
    Vucomisd,
    /// Instruction 'vucomish' {AVX512_FP16+VL}.
    Vucomish,
    /// Instruction 'vucomiss' {AVX|AVX512_F+VL}.
    Vucomiss,
    /// Instruction 'vunpckhpd' {AVX|AVX512_F+VL}.
    Vunpckhpd,
    /// Instruction 'vunpckhps' {AVX|AVX512_F+VL}.
    Vunpckhps,
    /// Instruction 'vunpcklpd' {AVX|AVX512_F+VL}.
    Vunpcklpd,
    /// Instruction 'vunpcklps' {AVX|AVX512_F+VL}.
    Vunpcklps,
    /// Instruction 'vxorpd' {AVX|AVX512_DQ+VL}.
    Vxorpd,
    /// Instruction 'vxorps' {AVX|AVX512_DQ+VL}.
    Vxorps,
    /// Instruction 'vzeroall' {AVX}.
    Vzeroall,
    /// Instruction 'vzeroupper' {AVX}.
    Vzeroupper,
    /// Instruction 'wbinvd' {I486}.
    Wbinvd,
    /// Instruction 'wbnoinvd' {WBNOINVD}.
    Wbnoinvd,
    /// Instruction 'wrfsbase' {FSGSBASE} (X64).
    Wrfsbase,
    /// Instruction 'wrgsbase' {FSGSBASE} (X64).
    Wrgsbase,
    /// Instruction 'wrmsr' {MSR}.
    Wrmsr,
    /// Instruction 'wrssd' {CET_SS}.
    Wrssd,
    /// Instruction 'wrssq' {CET_SS} (X64).
    Wrssq,
    /// Instruction 'wrussd' {CET_SS}.
    Wrussd,
    /// Instruction 'wrussq' {CET_SS} (X64).
    Wrussq,
    /// Instruction 'xabort' {RTM}.
    Xabort,
    /// Instruction 'xadd' {I486}.
    Xadd,
    /// Instruction 'xbegin' {RTM}.
    Xbegin,
    /// Instruction 'xchg'.
    Xchg,
    /// Instruction 'xend' {RTM}.
    Xend,
    /// Instruction 'xgetbv' {XSAVE}.
    Xgetbv,
    /// Instruction 'xlatb'.
    Xlatb,
    /// Instruction 'xor'.
    Xor,
    /// Instruction 'xorpd' {SSE2}.
    Xorpd,
    /// Instruction 'xorps' {SSE}.
    Xorps,
    /// Instruction 'xresldtrk' {TSXLDTRK}.
    Xresldtrk,
    /// Instruction 'xrstor' {XSAVE}.
    Xrstor,
    /// Instruction 'xrstor64' {XSAVE} (X64).
    Xrstor64,
    /// Instruction 'xrstors' {XSAVES}.
    Xrstors,
    /// Instruction 'xrstors64' {XSAVES} (X64).
    Xrstors64,
    /// Instruction 'xsave' {XSAVE}.
    Xsave,
    /// Instruction 'xsave64' {XSAVE} (X64).
    Xsave64,
    /// Instruction 'xsavec' {XSAVEC}.
    Xsavec,
    /// Instruction 'xsavec64' {XSAVEC} (X64).
    Xsavec64,
    /// Instruction 'xsaveopt' {XSAVEOPT}.
    Xsaveopt,
    /// Instruction 'xsaveopt64' {XSAVEOPT} (X64).
    Xsaveopt64,
    /// Instruction 'xsaves' {XSAVES}.
    Xsaves,
    /// Instruction 'xsaves64' {XSAVES} (X64).
    Xsaves64,
    /// Instruction 'xsetbv' {XSAVE}.
    Xsetbv,
    /// Instruction 'xsusldtrk' {TSXLDTRK}.
    Xsusldtrk,
    /// Instruction 'xtest' {RTM}.
    Xtest,
    _Count
}

/// Instruction aliases (AsmJit `Inst::kIdX = kIdY`).
impl InstId {
    pub const CMOVNAE: InstId = InstId::Cmovb;
    pub const CMOVC: InstId = InstId::Cmovb;
    pub const CMOVNA: InstId = InstId::Cmovbe;
    pub const CMOVNGE: InstId = InstId::Cmovl;
    pub const CMOVNG: InstId = InstId::Cmovle;
    pub const CMOVAE: InstId = InstId::Cmovnb;
    pub const CMOVNC: InstId = InstId::Cmovnb;
    pub const CMOVA: InstId = InstId::Cmovnbe;
    pub const CMOVGE: InstId = InstId::Cmovnl;
    pub const CMOVG: InstId = InstId::Cmovnle;
    pub const CMOVPO: InstId = InstId::Cmovnp;
    pub const CMOVNE: InstId = InstId::Cmovnz;
    pub const CMOVPE: InstId = InstId::Cmovp;
    pub const CMOVE: InstId = InstId::Cmovz;
    pub const WAIT: InstId = InstId::Fwait;
    pub const JNAE: InstId = InstId::Jb;
    pub const JC: InstId = InstId::Jb;
    pub const JNA: InstId = InstId::Jbe;
    pub const JNGE: InstId = InstId::Jl;
    pub const JNG: InstId = InstId::Jle;
    pub const JAE: InstId = InstId::Jnb;
    pub const JNC: InstId = InstId::Jnb;
    pub const JA: InstId = InstId::Jnbe;
    pub const JGE: InstId = InstId::Jnl;
    pub const JG: InstId = InstId::Jnle;
    pub const JPO: InstId = InstId::Jnp;
    pub const JNE: InstId = InstId::Jnz;
    pub const JPE: InstId = InstId::Jp;
    pub const JE: InstId = InstId::Jz;
    pub const SETNAE: InstId = InstId::Setb;
    pub const SETC: InstId = InstId::Setb;
    pub const SETNA: InstId = InstId::Setbe;
    pub const SETNGE: InstId = InstId::Setl;
    pub const SETNG: InstId = InstId::Setle;
    pub const SETAE: InstId = InstId::Setnb;
    pub const SETNC: InstId = InstId::Setnb;
    pub const SETA: InstId = InstId::Setnbe;
    pub const SETGE: InstId = InstId::Setnl;
    pub const SETG: InstId = InstId::Setnle;
    pub const SETPO: InstId = InstId::Setnp;
    pub const SETNE: InstId = InstId::Setnz;
    pub const SETPE: InstId = InstId::Setp;
    pub const SETE: InstId = InstId::Setz;
    pub const SAL: InstId = InstId::Shl;
}

/// Describes which operation mode is supported by an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    /// Invalid mode.
    None = 0x00,
    /// X86 mode supported.
    X86 = 0x01,
    /// X64 mode supported.
    X64 = 0x02,
    /// Both X86 and X64 modes supported.
    Any = 0x03,
}

// Operand signature flags used by [`OpSignature`].
bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct OpFlags: u64 {
        /// No operand flags.
        const NONE = 0;
        /// Operand can be low 8-bit GPB register.
        const REG_GPB_LO = 0x0000000000000001;
        /// Operand can be high 8-bit GPB register.
        const REG_GPB_HI = 0x0000000000000002;
        /// Operand can be 16-bit GPW register.
        const REG_GPW = 0x0000000000000004;
        /// Operand can be 32-bit GPD register.
        const REG_GPD = 0x0000000000000008;
        /// Operand can be 64-bit GPQ register.
        const REG_GPQ = 0x0000000000000010;
        /// Operand can be 128-bit XMM register.
        const REG_XMM = 0x0000000000000020;
        /// Operand can be 256-bit YMM register.
        const REG_YMM = 0x0000000000000040;
        /// Operand can be 512-bit ZMM register.
        const REG_ZMM = 0x0000000000000080;
        /// Operand can be 64-bit MM register.
        const REG_MM = 0x0000000000000100;
        /// Operand can be 64-bit K register.
        const REG_K_REG = 0x0000000000000200;
        /// Operand can be SReg (segment register).
        const REG_S_REG = 0x0000000000000400;
        /// Operand can be CReg (control register).
        const REG_C_REG = 0x0000000000000800;
        /// Operand can be DReg (debug register).
        const REG_D_REG = 0x0000000000001000;
        /// Operand can be 80-bit ST register (X87).
        const REG_ST = 0x0000000000002000;
        /// Operand can be 128-bit BND register.
        const REG_BND = 0x0000000000004000;
        /// Operand can be 0..8192-bit TMM register.
        const REG_TMM = 0x0000000000008000;
        /// Mask of all possible register types.
        const REG_MASK = 0x000000000000FFFF;
        /// Operand can be a scalar memory pointer without size.
        const MEM_UNSPECIFIED = 0x0000000000040000;
        /// Operand can be an 8-bit memory pointer.
        const MEM8 = 0x0000000000080000;
        /// Operand can be a 16-bit memory pointer.
        const MEM16 = 0x0000000000100000;
        /// Operand can be a 32-bit memory pointer.
        const MEM32 = 0x0000000000200000;
        /// Operand can be a 48-bit memory pointer (FAR pointers only).
        const MEM48 = 0x0000000000400000;
        /// Operand can be a 64-bit memory pointer.
        const MEM64 = 0x0000000000800000;
        /// Operand can be an 80-bit memory pointer.
        const MEM80 = 0x0000000001000000;
        /// Operand can be a 128-bit memory pointer.
        const MEM128 = 0x0000000002000000;
        /// Operand can be a 256-bit memory pointer.
        const MEM256 = 0x0000000004000000;
        /// Operand can be a 512-bit memory pointer.
        const MEM512 = 0x0000000008000000;
        /// Operand can be a 1024-bit memory pointer.
        const MEM1024 = 0x0000000010000000;
        /// Mask of all possible scalar memory types.
        const MEM_MASK = 0x000000001FFC0000;
        /// Operand can be a vm32x (vector) pointer.
        const VM32X = 0x0000000040000000;
        /// Operand can be a vm32y (vector) pointer.
        const VM32Y = 0x0000000080000000;
        /// Operand can be a vm32z (vector) pointer.
        const VM32Z = 0x0000000100000000;
        /// Operand can be a vm64x (vector) pointer.
        const VM64X = 0x0000000200000000;
        /// Operand can be a vm64y (vector) pointer.
        const VM64Y = 0x0000000400000000;
        /// Operand can be a vm64z (vector) pointer.
        const VM64Z = 0x0000000800000000;
        /// Mask of all possible vector memory types.
        const VM_MASK = 0x0000000FC0000000;
        /// Operand can be signed 4-bit immediate.
        const IMM_I4 = 0x0000001000000000;
        /// Operand can be unsigned 4-bit immediate.
        const IMM_U4 = 0x0000002000000000;
        /// Operand can be signed 8-bit immediate.
        const IMM_I8 = 0x0000004000000000;
        /// Operand can be unsigned 8-bit immediate.
        const IMM_U8 = 0x0000008000000000;
        /// Operand can be signed 16-bit immediate.
        const IMM_I16 = 0x0000010000000000;
        /// Operand can be unsigned 16-bit immediate.
        const IMM_U16 = 0x0000020000000000;
        /// Operand can be signed 32-bit immediate.
        const IMM_I32 = 0x0000040000000000;
        /// Operand can be unsigned 32-bit immediate.
        const IMM_U32 = 0x0000080000000000;
        /// Operand can be signed 64-bit immediate.
        const IMM_I64 = 0x0000100000000000;
        /// Operand can be unsigned 64-bit immediate.
        const IMM_U64 = 0x0000200000000000;
        /// Mask of all immediate types.
        const IMM_MASK = 0x00003FF000000000;
        /// Operand can be relative 8-bit  displacement.
        const REL8 = 0x0000400000000000;
        /// Operand can be relative 32-bit displacement.
        const REL32 = 0x0000800000000000;
        /// Mask of all relative displacement types.
        const REL_MASK = 0x0000C00000000000;
        /// Flag: Only memory base is allowed (no index, no offset).
        const FLAG_MEM_BASE = 0x0001000000000000;
        /// Flag: Implicit memory operand's DS segment.
        const FLAG_MEM_DS = 0x0002000000000000;
        /// Flag: Implicit memory operand's ES segment.
        const FLAG_MEM_ES = 0x0004000000000000;
        /// Flag: Operand is MIB (base+index) pointer.
        const FLAG_MIB = 0x0008000000000000;
        /// Flag: Operand is TMEM (sib_mem), AMX memory pointer.
        const FLAG_T_MEM = 0x0010000000000000;
        /// Flag: Operand is implicit.
        const FLAG_IMPLICIT = 0x0080000000000000;
        /// Mask of all flags.
        const FLAG_MASK = 0x009F000000000000;
        /// Contains mask of all registers, memory operands, immediate operands, and displacement operands.
        const OP_MASK = Self::REG_MASK.bits() | Self::MEM_MASK.bits() | Self::VM_MASK.bits() | Self::IMM_MASK.bits() | Self::REL_MASK.bits();
    }
}

/// Operand signature: all possible operand combinations, memory size
/// information, and a fixed register id (port of AsmJit's `InstDB::OpSignature`).
#[derive(Debug, Clone, Copy, Default)]
pub struct OpSignature {
    pub flags: u64,
    pub reg_mask: u8,
}

impl OpSignature {
    pub const fn new(flags: u64, reg_mask: u8) -> Self {
        Self { flags, reg_mask }
    }
}

/// Instruction signature: a sequence of operand combinations and other
/// metadata defining a single instruction (port of AsmJit's `InstDB::InstSignature`).
#[derive(Debug, Clone, Copy, Default)]
pub struct InstSignature {
    pub op_count: u8,
    pub mode: u8,
    pub implicit_op_count: u8,
    pub reserved: u8,
    pub op_signature_indexes: [u8; 6],
}

impl InstSignature {
    pub const fn new(
        op_count: u8,
        mode: u8,
        implicit_op_count: u8,
        reserved: u8,
        op_signature_indexes: [u8; 6],
    ) -> Self {
        Self {
            op_count,
            mode,
            implicit_op_count,
            reserved,
            op_signature_indexes,
        }
    }
}

// Instruction flags.
bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct InstFlags: u32 {
        /// No flags.
        const NONE = 0x00000000;
        /// Instruction that accesses FPU registers.
        const FPU = 0x00000100;
        /// Instruction that accesses MMX registers (including 3DNOW and GEODE) and EMMS.
        const MMX = 0x00000200;
        /// Instruction that accesses XMM registers (SSE, AVX, AVX512).
        const VEC = 0x00000400;
        /// FPU instruction can address `word_ptr` (shared with M80).
        const FPU_M16 = 0x00000800;
        /// FPU instruction can address `dword_ptr`.
        const FPU_M32 = 0x00001000;
        /// FPU instruction can address `qword_ptr`.
        const FPU_M64 = 0x00002000;
        /// FPU instruction can address `tword_ptr` (shared with M16).
        const FPU_M80 = 0x00000800;
        /// Instruction can be prefixed with using the REP(REPE) or REPNE prefix.
        const REP = 0x00004000;
        /// Rep prefix is accepted, but it has no effect other than being emitted with the instruction (as an extra byte).
        const REP_IGNORED = 0x00008000;
        /// Instruction can be prefixed with using the LOCK prefix.
        const LOCK = 0x00010000;
        /// Instruction can be prefixed with using the XACQUIRE prefix.
        const X_ACQUIRE = 0x00020000;
        /// Instruction can be prefixed with using the XRELEASE prefix.
        const X_RELEASE = 0x00040000;
        /// Instruction uses MIB (BNDLDX|BNDSTX) to encode two registers.
        const MIB = 0x00080000;
        /// Instruction uses VSIB instead of legacy SIB.
        const VSIB = 0x00100000;
        /// Instruction uses TSIB (or SIB_MEM) encoding (MODRM followed by SIB).
        const TSIB = 0x00200000;
        /// Instruction can be encoded by VEX|XOP (AVX|AVX2|BMI|XOP|...).
        const VEX = 0x00400000;
        /// Instruction can be encoded by EVEX (AVX512).
        const EVEX = 0x00800000;
        /// EVEX encoding is preferred over VEX encoding (AVX515_VNNI vs AVX_VNNI).
        const PREFER_EVEX = 0x01000000;
        /// EVEX and VEX signatures are compatible.
        const EVEX_COMPAT = 0x02000000;
        /// EVEX instruction requires K register in the first operand (compare instructions).
        const EVEX_K_REG = 0x04000000;
        /// EVEX instruction requires two operands and K register as a selector (gather instructions).
        const EVEX_TWO_OP = 0x08000000;
        /// VEX instruction that can be transformed to a compatible EVEX instruction.
        const EVEX_TRANSFORMABLE = 0x10000000;
        /// Instruction uses consecutive registers. Used by VP2INTERSECTD and VP2INTERSECTQ instructions.
        const CONSECUTIVE_REGS = 0x20000000;
    }
}

// AVX-512 flags.
bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct Avx512Flags: u32 {
        /// No AVX-512 flags.
        const NONE = 0;
        /// Supports masking {k1..k7}.
        const K = 0x00000001;
        /// Supports zeroing {z}, must be used together with `kAvx512k`.
        const Z = 0x00000002;
        /// Supports 'embedded-rounding' {er} with implicit {sae},
        const ER = 0x00000004;
        /// Supports 'suppress-all-exceptions' {sae}.
        const SAE = 0x00000008;
        /// Supports 16-bit broadcast 'b16'.
        const B16 = 0x00000010;
        /// Supports 32-bit broadcast 'b32'.
        const B32 = 0x00000020;
        /// Supports 64-bit broadcast 'b64'.
        const B64 = 0x00000040;
        /// Implicit zeroing if {k} masking is used. Using {z} is not valid in this case as it's implicit.
        const IMPLICIT_Z = 0x00000100;
    }
}

/// Instruction encoding (X86|X86_64).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum Encoding {
    /// Never used.
    None = 0,
    /// X86 [OP].
    X86Op,
    /// X86 [OP] (opcode with ModRM byte where MOD must be 11b).
    X86Op_Mod11RM,
    /// X86 [OP] (opcode with ModRM byte + 8-bit immediate).
    X86Op_Mod11RM_I8,
    /// X86 [OP] (implicit address in the first register operand).
    X86Op_xAddr,
    /// X86 [OP] (implicit or explicit '?AX' form).
    X86Op_xAX,
    /// X86 [OP] (implicit or explicit '?DX, ?AX' form).
    X86Op_xDX_xAX,
    /// X86 [OP] (implicit or explicit '[EAX|RAX]' form).
    X86Op_MemZAX,
    /// X86 [I] (implicit or explicit '?AX' form).
    X86I_xAX,
    /// X86 [M] (handles 2|4|8-bytes size).
    X86M,
    /// X86 [M] (handles 2|4|8-bytes size, but doesn't consider memory size).
    X86M_NoMemSize,
    /// X86 [M] (doesn't handle any size).
    X86M_NoSize,
    /// X86 [M] (handles single-byte size).
    X86M_GPB,
    /// X86 [M] (like GPB, handles implicit|explicit MUL|DIV|IDIV).
    X86M_GPB_MulDiv,
    /// X86 [M] (restricted to memory operand of any size).
    X86M_Only,
    /// X86 [M] (memory operand only, followed by implicit `EDX` and `EAX`).
    X86M_Only_EDX_EAX,
    /// X86 [M] (special case of NOP instruction).
    X86M_Nop,
    /// X86 [R] (register must be either 32-bit or 64-bit depending on arch).
    X86R_Native,
    /// X86 [R] - which specifies memory address.
    X86R_FromM,
    /// X86 [R32] followed by implicit `EDX` and `EAX`.
    X86R32_EDX_EAX,
    /// X86 [RM] (doesn't handle single-byte size).
    X86Rm,
    /// X86 [RM] (used by LZCNT, POPCNT, and TZCNT).
    X86Rm_Raw66H,
    /// X86 [RM] (doesn't add REX.W prefix if 64-bit reg is used).
    X86Rm_NoSize,
    /// X86 [MR] (doesn't handle single-byte size).
    X86Mr,
    /// X86 [MR] (doesn't handle any size).
    X86Mr_NoSize,
    /// X86 adc, add, and, cmp, or, sbb, sub, xor.
    X86Arith,
    /// X86 bswap.
    X86Bswap,
    /// X86 bt, btc, btr, bts.
    X86Bt,
    /// X86 call.
    X86Call,
    /// X86 [MR] cmpxchg.
    X86Cmpxchg,
    /// X86 [MR] cmpxchg8b, cmpxchg16b.
    X86Cmpxchg8b_16b,
    /// X86 crc32.
    X86Crc,
    /// X86 enter.
    X86Enter,
    /// X86 imul.
    X86Imul,
    /// X86 in.
    X86In,
    /// X86 ins[b|q|d].
    X86Ins,
    /// X86 inc, dec.
    X86IncDec,
    /// X86 int (interrupt).
    X86Int,
    /// X86 jcc.
    X86Jcc,
    /// X86 jcxz, jecxz, jrcxz, loop, loope, loopne.
    X86JecxzLoop,
    /// X86 jmp.
    X86Jmp,
    /// X86 xbegin.
    X86JmpRel,
    /// X86 lcall/ljmp.
    X86LcallLjmp,
    /// X86 lea.
    X86Lea,
    /// X86 mov (all possible cases).
    X86Mov,
    /// X86 movabs.
    X86Movabs,
    /// X86 movsx, movzx.
    X86MovsxMovzx,
    /// X86 movnti/movdiri.
    X86MovntiMovdiri,
    /// X86 enqcmd/enqcmds/movdir64b.
    X86EnqcmdMovdir64b,
    /// X86 out.
    X86Out,
    /// X86 out[b|w|d].
    X86Outs,
    /// X86 push.
    X86Push,
    /// X86 pushw.
    X86Pushw,
    /// X86 pop.
    X86Pop,
    /// X86 ret.
    X86Ret,
    /// X86 rcl, rcr, rol, ror, sal, sar, shl, shr.
    X86Rot,
    /// X86 setcc.
    X86Set,
    /// X86 shld, shrd.
    X86ShldShrd,
    /// X86 lods.
    X86StrRm,
    /// X86 scas, stos.
    X86StrMr,
    /// X86 cmps, movs.
    X86StrMm,
    /// X86 test.
    X86Test,
    /// X86 xadd.
    X86Xadd,
    /// X86 xchg.
    X86Xchg,
    /// X86 lfence, mfence, sfence.
    X86Fence,
    /// X86 [RM|MR] (used by BNDMOV).
    X86Bndmov,
    /// FPU [OP].
    FpuOp,
    /// FPU fadd, fdiv, fdivr, fmul, fsub, fsubr.
    FpuArith,
    /// FPU fcom, fcomp.
    FpuCom,
    /// FPU fld, fst, fstp.
    FpuFldFst,
    /// FPU fiadd, ficom, ficomp, fidiv, fidivr, fild, fimul, fist, fistp, fisttp, fisub, fisubr.
    FpuM,
    /// FPU fcmov, fcomi, fcomip, ffree, fucom, fucomi, fucomip, fucomp, fxch.
    FpuR,
    /// FPU faddp, fdivp, fdivrp, fmulp, fsubp, fsubrp.
    FpuRDef,
    /// FPU fnstsw, Fstsw.
    FpuStsw,
    /// EXT [RM].
    ExtRm,
    /// EXT [RM<XMM0>].
    ExtRm_XMM0,
    /// EXT [RM<ZDI>].
    ExtRm_ZDI,
    /// EXT [RM] (propagates 66H if the instruction uses XMM register).
    ExtRm_P,
    /// EXT [RM] (propagates REX.W if GPQ is used or the second operand is GPQ/QWORD_PTR).
    ExtRm_Wx,
    /// EXT [RM] (propagates REX.W if the first operand is GPQ register).
    ExtRm_Wx_GpqOnly,
    /// EXT [RM|RI].
    ExtRmRi,
    /// EXT [RM|RI] (propagates 66H if the instruction uses XMM register).
    ExtRmRi_P,
    /// EXT [RMI].
    ExtRmi,
    /// EXT [RMI] (propagates 66H if the instruction uses XMM register).
    ExtRmi_P,
    /// EXT pextrw.
    ExtPextrw,
    /// EXT pextrb, pextrd, pextrq, extractps.
    ExtExtract,
    /// EXT mov?? - #1:[MM|XMM, MM|XMM|Mem] #2:[MM|XMM|Mem, MM|XMM].
    ExtMov,
    /// EXT movbe.
    ExtMovbe,
    /// EXT movd.
    ExtMovd,
    /// EXT movq.
    ExtMovq,
    /// EXT extrq (SSE4A).
    ExtExtrq,
    /// EXT insrq (SSE4A).
    ExtInsertq,
    /// EXT [RMI] (3DNOW specific).
    Ext3dNow,
    /// VEX [OP].
    VexOp,
    /// VEX [OP] with MODR/M.
    VexOpMod,
    /// VEX [RM|MR] (used by kmov[b|w|d|q]).
    VexKmov,
    /// VEX|EVEX [R] (propagatex VEX.W if GPQ used).
    VexR_Wx,
    /// VEX|EVEX [M].
    VexM,
    /// VEX|EVEX [MR] (propagates VEX|EVEX.L if YMM used).
    VexMr_Lx,
    /// VEX|EVEX [MR] (VSIB support).
    VexMr_VM,
    /// VEX|EVEX [MRI].
    VexMri,
    /// VEX|EVEX [MRI] (propagates VEX|EVEX.L if YMM used).
    VexMri_Lx,
    /// VEX|EVEX [MRI] (special case required by VPEXTRW instruction).
    VexMri_Vpextrw,
    /// VEX|EVEX [MVR] (propagates VEX|EVEX.W if GPQ used).
    VexMvr_Wx,
    /// VEX|EVEX [RM].
    VexRm,
    /// VEX|EVEX [RM<ZDI>].
    VexRm_ZDI,
    /// VEX|EVEX [RM] (propagates VEX|EVEX.W if GPQ used).
    VexRm_Wx,
    /// VEX|EVEX [RM] (propagates VEX|EVEX.L if YMM used).
    VexRm_Lx,
    /// VEX|EVEX [RM] (the destination vector size is narrowed).
    VexRm_Lx_Narrow,
    /// VEX|EVEX [RM] (can handle broadcast r32/r64).
    VexRm_Lx_Bcst,
    /// VEX|EVEX [RM] (propagates VEX|EVEX.L, VSIB support).
    VexRm_VM,
    /// VEX|EVEX [RMI].
    VexRmi,
    /// VEX|EVEX [RMI] (propagates VEX|EVEX.W if GPQ used).
    VexRmi_Wx,
    /// VEX|EVEX [RMI] (propagates VEX|EVEX.L if YMM used).
    VexRmi_Lx,
    /// VEX|EVEX [RVM].
    VexRvm,
    /// VEX|EVEX [RVM] (propagates VEX|EVEX.W if GPQ used).
    VexRvm_Wx,
    /// VEX|EVEX [RVM<ZDX>] (propagates VEX|EVEX.W if GPQ used).
    VexRvm_ZDX_Wx,
    /// VEX|EVEX [RVM] (propagates VEX|EVEX.L if YMM used).
    VexRvm_Lx,
    /// VEX|EVEX [RVM] (forces EVEX prefix if K register is used on destination).
    VexRvm_Lx_KEvex,
    /// VEX|EVEX [RVM] (vp2intersectd/vp2intersectq).
    VexRvm_Lx_2xK,
    /// VEX|EVEX [RVMR].
    VexRvmr,
    /// VEX|EVEX [RVMR] (propagates VEX|EVEX.L if YMM used).
    VexRvmr_Lx,
    /// VEX|EVEX [RVMI].
    VexRvmi,
    /// VEX|EVEX [RVMI] (forces EVEX prefix if K register is used on destination).
    VexRvmi_KEvex,
    /// VEX|EVEX [RVMI] (propagates VEX|EVEX.L if YMM used).
    VexRvmi_Lx,
    /// VEX|EVEX [RVMI] (forces EVEX prefix if K register is used on destination).
    VexRvmi_Lx_KEvex,
    /// VEX|EVEX [RMV].
    VexRmv,
    /// VEX|EVEX [RMV] (propagates VEX|EVEX.W if GPQ used).
    VexRmv_Wx,
    /// VEX|EVEX [RMV] (propagates VEX|EVEX.L, VSIB support).
    VexRmv_VM,
    /// VEX|EVEX [RMV|RM] (propagates VEX|EVEX.L, VSIB support).
    VexRmvRm_VM,
    /// VEX|EVEX [RMVI].
    VexRmvi,
    /// VEX|EVEX [RM|MR].
    VexRmMr,
    /// VEX|EVEX [RM|MR] (propagates VEX|EVEX.L if YMM used).
    VexRmMr_Lx,
    /// VEX|EVEX [RVM|RMV].
    VexRvmRmv,
    /// VEX|EVEX [RVM|RMI].
    VexRvmRmi,
    /// VEX|EVEX [RVM|RMI] (propagates VEX|EVEX.L if YMM used).
    VexRvmRmi_Lx,
    /// VEX|EVEX [RVM|RMV|RMI].
    VexRvmRmvRmi,
    /// VEX|EVEX [RVM|MR].
    VexRvmMr,
    /// VEX|EVEX [RVM|MVR].
    VexRvmMvr,
    /// VEX|EVEX [RVM|MVR] (propagates VEX|EVEX.L if YMM used).
    VexRvmMvr_Lx,
    /// VEX|EVEX [RVM|VMI].
    VexRvmVmi,
    /// VEX|EVEX [RVM|VMI] (propagates VEX|EVEX.L if YMM used).
    VexRvmVmi_Lx,
    /// VEX|EVEX [RVM|VMI] (propagates EVEX if the second operand is memory).
    VexRvmVmi_Lx_MEvex,
    /// VEX|EVEX [VM].
    VexVm,
    /// VEX|EVEX [VM] (propagates VEX|EVEX.W if GPQ used).
    VexVm_Wx,
    /// VEX|EVEX [VMI].
    VexVmi,
    /// VEX|EVEX [VMI] (propagates VEX|EVEX.L if YMM used).
    VexVmi_Lx,
    /// VEX|EVEX [VMI] (propagates VEX|EVEX.W if GPQ used, DWORD Immediate).
    VexVmi4_Wx,
    /// VEX|EVEX [VMI] (force EVEX prefix when the second operand is memory)
    VexVmi_Lx_MEvex,
    /// VEX|EVEX [RVRM|RVMR].
    VexRvrmRvmr,
    /// VEX|EVEX [RVRM|RVMR] (propagates VEX|EVEX.L if YMM used).
    VexRvrmRvmr_Lx,
    /// VEX|EVEX [RVRMI|RVMRI] (propagates VEX|EVEX.L if YMM used).
    VexRvrmiRvmri_Lx,
    /// VEX|EVEX vmovd, vmovq.
    VexMovdMovq,
    /// VEX|EVEX vmovss, vmovsd.
    VexMovssMovsd,
    /// FMA4 [R, R, R/M, R/M].
    Fma4,
    /// FMA4 [R, R, R/M, R/M] (propagates AVX.L if YMM used).
    Fma4_Lx,
    /// AMX ldtilecfg/sttilecfg.
    AmxCfg,
    /// AMX [R] - tilezero.
    AmxR,
    /// AMX tileloadd/tileloaddt1.
    AmxRm,
    /// AMX tilestored.
    AmxMr,
    /// AMX instructions that use TMM registers.
    AmxRmv,
    /// Count of instruction encodings.
    Count,
}

/// Aggregated information shared across one or more instructions
/// (port of AsmJit's `InstDB::CommonInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct CommonInfo {
    pub flags: u32,
    pub avx512_flags: u32,
    pub signature_index: u32,
    pub signature_count: u32,
    pub control_flow: InstControlFlow,
    pub same_reg_hint: InstSameRegHint,
}

impl CommonInfo {
    pub const fn new(
        flags: u32,
        avx512_flags: u32,
        signature_index: u32,
        signature_count: u32,
        control_flow: InstControlFlow,
        same_reg_hint: InstSameRegHint,
    ) -> Self {
        Self {
            flags,
            avx512_flags,
            signature_index,
            signature_count,
            control_flow,
            same_reg_hint,
        }
    }

    pub const fn has_flag(&self, flag: InstFlags) -> bool {
        self.flags & flag.bits() != 0
    }

    pub const fn has_avx512_flag(&self, flag: Avx512Flags) -> bool {
        self.avx512_flags & flag.bits() != 0
    }
}

/// Instruction information (port of AsmJit's `InstDB::InstInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct InstInfo {
    pub reserved: u32,
    pub common_info_index: u32,
    pub additional_info_index: u32,
    pub encoding: u8,
    pub main_opcode_value: u8,
    pub main_opcode_index: u8,
    pub alt_opcode_index: u8,
}

impl InstInfo {
    pub const fn new(
        common_info_index: u32,
        additional_info_index: u32,
        encoding: u8,
        main_opcode_value: u8,
        main_opcode_index: u8,
        alt_opcode_index: u8,
    ) -> Self {
        Self {
            reserved: 0,
            common_info_index,
            additional_info_index,
            encoding,
            main_opcode_value,
            main_opcode_index,
            alt_opcode_index,
        }
    }
}

/// Additional information table entry: CPU extensions required to execute an
/// instruction plus RW flags (port of AsmJit's `InstDB::AdditionalInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct AdditionalInfo {
    pub inst_flags_index: u8,
    pub rw_flags_index: u8,
    pub features: [u8; 6],
}

impl AdditionalInfo {
    pub const fn new(inst_flags_index: u8, rw_flags_index: u8, features: [u8; 6]) -> Self {
        Self {
            inst_flags_index,
            rw_flags_index,
            features,
        }
    }
}

/// Read/write information of an instruction (port of AsmJit's `InstDB::RWInfo`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfo {
    pub category: RwInfoCategory,
    pub rm_info: u8,
    pub op_info_index: [u8; 6],
}

impl RwInfo {
    pub const fn new(category: RwInfoCategory, rm_info: u8, op_info_index: [u8; 6]) -> Self {
        Self {
            category,
            rm_info,
            op_info_index,
        }
    }
}

/// Read/write information of a single operand (port of AsmJit's `InstDB::RWInfoOp`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfoOp {
    pub r_byte_mask: u64,
    pub w_byte_mask: u64,
    pub phys_id: u8,
    pub consecutive_lead_count: u8,
    pub flags: OpRwFlags,
}

impl RwInfoOp {
    pub const fn new(
        r_byte_mask: u64,
        w_byte_mask: u64,
        phys_id: u8,
        consecutive_lead_count: u8,
        flags: u32,
    ) -> Self {
        Self {
            r_byte_mask,
            w_byte_mask,
            phys_id,
            consecutive_lead_count,
            flags: OpRwFlags::from_bits_retain(flags),
        }
    }
}

/// R/M information, used to replace a register operand by a memory operand
/// reliably (port of AsmJit's `InstDB::RWInfoRm`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwInfoRm {
    pub category: RwInfoRmCategory,
    pub rm_ops_mask: u8,
    pub fixed_size: u8,
    pub flags: u8,
    pub rm_feature: u8,
}

impl RwInfoRm {
    pub const fn new(
        category: RwInfoRmCategory,
        rm_ops_mask: u8,
        fixed_size: u8,
        flags: u8,
        rm_feature: u8,
    ) -> Self {
        Self {
            category,
            rm_ops_mask,
            fixed_size,
            flags,
            rm_feature,
        }
    }
}

/// CPU/FPU flags read/written information (port of AsmJit's `InstDB::RWFlagsInfoTable`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RwFlagsInfo {
    pub read_flags: u32,
    pub write_flags: u32,
}

impl RwFlagsInfo {
    pub const fn new(read_flags: u32, write_flags: u32) -> Self {
        Self {
            read_flags,
            write_flags,
        }
    }
}

/// Category of [`RwInfo`] (port of AsmJit's `InstDB::RWInfo::Category`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum RwInfoCategory {
    #[default]
    Generic = 0,
    GenericEx,
    Mov,
    Movabs,
    Imul,
    Movh64,
    Punpcklxx,
    Vmaskmov,
    Vmovddup,
    Vmovmskpd,
    Vmovmskps,
    Vmov1_2,
    Vmov1_4,
    Vmov1_8,
    Vmov2_1,
    Vmov4_1,
    Vmov8_1,
}

/// Category of [`RwInfoRm`] (port of AsmJit's `InstDB::RWInfoRm::Category`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum RwInfoRmCategory {
    #[default]
    None = 0,
    Fixed,
    Consistent,
    Half,
    Quarter,
    Eighth,
}

// Flags of [`RwInfoRm`] (port of AsmJit's `InstDB::RWInfoRm::Flags`).
bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
    pub struct RwInfoRmFlags: u8 {
        const AMBIGUOUS = 0x01;
        /// Special semantics for PEXTRW - memory operand can only be used with SSE4.1 instruction and it's forbidden in MMX.
        const PEXTRW = 0x02;
        /// Special semantics for MOVSS and MOVSD - doesn't zero extend the destination if the operation is a reg to reg move.
        const MOVSS_MOVSD = 0x04;
        /// Special semantics for AVX shift instructions that do not provide reg/mem in AVX/AVX2 mode (AVX-512 is required).
        const FEATURE_IF_RMI = 0x08;
        /// No flags.
        const NONE = 0;
    }
}

/// Instruction information table, indexed by [`InstId`].
pub static INST_INFO_TABLE: &[InstInfo] = &[
    InstInfo::new(0, 0, Encoding::None as u8, 0x00, 0, 0), // #0
    InstInfo::new(1, 1, Encoding::X86Op_xAX as u8, 0x37, 0, 0), // #1
    InstInfo::new(2, 1, Encoding::X86I_xAX as u8, 0xD5, 0, 0), // #2
    InstInfo::new(3, 2, Encoding::X86Mr as u8, 0xFC, 1, 0), // #3
    InstInfo::new(2, 1, Encoding::X86I_xAX as u8, 0xD4, 0, 0), // #4
    InstInfo::new(3, 2, Encoding::X86Mr as u8, 0xFC, 2, 0), // #5
    InstInfo::new(1, 1, Encoding::X86Op_xAX as u8, 0x3F, 0, 0), // #6
    InstInfo::new(4, 3, Encoding::X86Arith as u8, 0x10, 3, 0), // #7
    InstInfo::new(5, 4, Encoding::X86Rm as u8, 0xF6, 2, 0), // #8
    InstInfo::new(4, 1, Encoding::X86Arith as u8, 0x00, 0, 0), // #9
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x58, 4, 0), // #10
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x58, 5, 0), // #11
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x58, 6, 0), // #12
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x58, 7, 0), // #13
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0xD0, 4, 0), // #14
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0xD0, 6, 0), // #15
    InstInfo::new(5, 8, Encoding::X86Rm as u8, 0xF6, 8, 0), // #16
    InstInfo::new(6, 9, Encoding::ExtRm as u8, 0xDE, 2, 0), // #17
    InstInfo::new(6, 9, Encoding::ExtRm as u8, 0xDF, 2, 0), // #18
    InstInfo::new(6, 9, Encoding::ExtRm as u8, 0xDC, 2, 0), // #19
    InstInfo::new(6, 9, Encoding::ExtRm as u8, 0xDD, 2, 0), // #20
    InstInfo::new(6, 9, Encoding::ExtRm as u8, 0xDB, 2, 0), // #21
    InstInfo::new(9, 9, Encoding::ExtRmi as u8, 0xDF, 9, 0), // #22
    InstInfo::new(10, 1, Encoding::X86Arith as u8, 0x20, 10, 0), // #23
    InstInfo::new(11, 10, Encoding::VexRvm_Wx as u8, 0xF2, 11, 0), // #24
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x55, 4, 0), // #25
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x55, 5, 0), // #26
    InstInfo::new(12, 5, Encoding::ExtRm as u8, 0x54, 4, 0), // #27
    InstInfo::new(12, 6, Encoding::ExtRm as u8, 0x54, 5, 0), // #28
    InstInfo::new(3, 2, Encoding::X86Mr as u8, 0xFC, 12, 0), // #29
    InstInfo::new(13, 11, Encoding::X86Mr_NoSize as u8, 0x63, 0, 0), // #30
    InstInfo::new(3, 2, Encoding::X86Mr as u8, 0xFC, 8, 0), // #31
    InstInfo::new(14, 10, Encoding::VexRmv_Wx as u8, 0xF7, 11, 0), // #32
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 13, 0), // #33
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x02, 14, 0), // #34
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 15, 0), // #35
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x02, 13, 0), // #36
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 16, 0), // #37
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x0D, 9, 0), // #38
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x0C, 9, 0), // #39
    InstInfo::new(16, 13, Encoding::ExtRm_XMM0 as u8, 0x15, 2, 0), // #40
    InstInfo::new(16, 13, Encoding::ExtRm_XMM0 as u8, 0x14, 2, 0), // #41
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 17, 0), // #42
    InstInfo::new(15, 10, Encoding::VexVm_Wx as u8, 0xF3, 18, 0), // #43
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 14, 0), // #44
    InstInfo::new(15, 10, Encoding::VexVm_Wx as u8, 0xF3, 19, 0), // #45
    InstInfo::new(15, 10, Encoding::VexVm_Wx as u8, 0xF3, 20, 0), // #46
    InstInfo::new(17, 14, Encoding::X86Rm as u8, 0x1A, 7, 0), // #47
    InstInfo::new(17, 14, Encoding::X86Rm as u8, 0x1B, 6, 0), // #48
    InstInfo::new(17, 14, Encoding::X86Rm as u8, 0x1A, 6, 0), // #49
    InstInfo::new(18, 14, Encoding::X86Rm as u8, 0x1A, 5, 0), // #50
    InstInfo::new(19, 14, Encoding::X86Rm as u8, 0x1B, 7, 0), // #51
    InstInfo::new(20, 14, Encoding::X86Bndmov as u8, 0x1A, 4, 1), // #52
    InstInfo::new(21, 14, Encoding::X86Mr as u8, 0x1B, 5, 0), // #53
    InstInfo::new(22, 0, Encoding::X86Rm as u8, 0x62, 0, 0), // #54
    InstInfo::new(23, 1, Encoding::X86Rm as u8, 0xBC, 5, 0), // #55
    InstInfo::new(23, 1, Encoding::X86Rm as u8, 0xBD, 5, 0), // #56
    InstInfo::new(24, 0, Encoding::X86Bswap as u8, 0xC8, 5, 0), // #57
    InstInfo::new(25, 15, Encoding::X86Bt as u8, 0xA3, 5, 2), // #58
    InstInfo::new(26, 15, Encoding::X86Bt as u8, 0xBB, 5, 3), // #59
    InstInfo::new(26, 15, Encoding::X86Bt as u8, 0xB3, 5, 4), // #60
    InstInfo::new(26, 15, Encoding::X86Bt as u8, 0xAB, 5, 5), // #61
    InstInfo::new(14, 16, Encoding::VexRmv_Wx as u8, 0xF5, 11, 0), // #62
    InstInfo::new(27, 1, Encoding::X86Call as u8, 0xFF, 3, 0), // #63
    InstInfo::new(28, 0, Encoding::X86Op_xAX as u8, 0x98, 21, 0), // #64
    InstInfo::new(29, 0, Encoding::X86Op_xDX_xAX as u8, 0x99, 0, 0), // #65
    InstInfo::new(30, 0, Encoding::X86Op_xAX as u8, 0x98, 22, 0), // #66
    InstInfo::new(31, 17, Encoding::X86Op as u8, 0xCA, 23, 0), // #67
    InstInfo::new(31, 18, Encoding::X86Op as u8, 0xF8, 0, 0), // #68
    InstInfo::new(31, 19, Encoding::X86Op as u8, 0xFC, 0, 0), // #69
    InstInfo::new(32, 20, Encoding::X86M_Only as u8, 0x1C, 5, 0), // #70
    InstInfo::new(32, 21, Encoding::X86M_Only as u8, 0xAE, 24, 0), // #71
    InstInfo::new(32, 22, Encoding::X86M_Only as u8, 0xAE, 25, 0), // #72
    InstInfo::new(31, 23, Encoding::X86Op as u8, 0xDD, 23, 0), // #73
    InstInfo::new(31, 24, Encoding::X86Op as u8, 0xFA, 0, 0), // #74
    InstInfo::new(33, 25, Encoding::X86M_Only as u8, 0xAE, 26, 0), // #75
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0x06, 5, 0), // #76
    InstInfo::new(34, 26, Encoding::X86Op as u8, 0xEE, 27, 0), // #77
    InstInfo::new(32, 27, Encoding::X86M_Only as u8, 0xAE, 28, 0), // #78
    InstInfo::new(35, 28, Encoding::X86Op_MemZAX as u8, 0xFC, 23, 0), // #79
    InstInfo::new(31, 29, Encoding::X86Op as u8, 0xF5, 0, 0), // #80
    InstInfo::new(23, 30, Encoding::X86Rm as u8, 0x42, 5, 0), // #81
    InstInfo::new(23, 31, Encoding::X86Rm as u8, 0x46, 5, 0), // #82
    InstInfo::new(23, 32, Encoding::X86Rm as u8, 0x4C, 5, 0), // #83
    InstInfo::new(23, 33, Encoding::X86Rm as u8, 0x4E, 5, 0), // #84
    InstInfo::new(23, 30, Encoding::X86Rm as u8, 0x43, 5, 0), // #85
    InstInfo::new(23, 31, Encoding::X86Rm as u8, 0x47, 5, 0), // #86
    InstInfo::new(23, 32, Encoding::X86Rm as u8, 0x4D, 5, 0), // #87
    InstInfo::new(23, 33, Encoding::X86Rm as u8, 0x4F, 5, 0), // #88
    InstInfo::new(23, 34, Encoding::X86Rm as u8, 0x41, 5, 0), // #89
    InstInfo::new(23, 35, Encoding::X86Rm as u8, 0x4B, 5, 0), // #90
    InstInfo::new(23, 36, Encoding::X86Rm as u8, 0x49, 5, 0), // #91
    InstInfo::new(23, 37, Encoding::X86Rm as u8, 0x45, 5, 0), // #92
    InstInfo::new(23, 34, Encoding::X86Rm as u8, 0x40, 5, 0), // #93
    InstInfo::new(23, 35, Encoding::X86Rm as u8, 0x4A, 5, 0), // #94
    InstInfo::new(23, 36, Encoding::X86Rm as u8, 0x48, 5, 0), // #95
    InstInfo::new(23, 37, Encoding::X86Rm as u8, 0x44, 5, 0), // #96
    InstInfo::new(36, 1, Encoding::X86Arith as u8, 0x38, 29, 0), // #97
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE6, 30, 0), // #98
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE2, 30, 0), // #99
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xEE, 30, 0), // #100
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xEC, 30, 0), // #101
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE7, 30, 0), // #102
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE3, 30, 0), // #103
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xEF, 30, 0), // #104
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xED, 30, 0), // #105
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE1, 30, 0), // #106
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xEB, 30, 0), // #107
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE9, 30, 0), // #108
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE5, 30, 0), // #109
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE0, 30, 0), // #110
    InstInfo::new(9, 5, Encoding::ExtRmi as u8, 0xC2, 4, 0), // #111
    InstInfo::new(9, 6, Encoding::ExtRmi as u8, 0xC2, 5, 0), // #112
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xEA, 30, 0), // #113
    InstInfo::new(38, 39, Encoding::X86StrMm as u8, 0xA6, 0, 0), // #114
    InstInfo::new(39, 5, Encoding::ExtRmi as u8, 0xC2, 6, 0), // #115
    InstInfo::new(40, 6, Encoding::ExtRmi as u8, 0xC2, 7, 0), // #116
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE8, 30, 0), // #117
    InstInfo::new(41, 40, Encoding::X86Cmpxchg as u8, 0xB0, 5, 0), // #118
    InstInfo::new(42, 41, Encoding::X86Cmpxchg8b_16b as u8, 0xC7, 31, 0), // #119
    InstInfo::new(43, 42, Encoding::X86Cmpxchg8b_16b as u8, 0xC7, 32, 0), // #120
    InstInfo::new(37, 38, Encoding::VexMvr_Wx as u8, 0xE4, 30, 0), // #121
    InstInfo::new(7, 43, Encoding::ExtRm as u8, 0x2F, 4, 0), // #122
    InstInfo::new(8, 44, Encoding::ExtRm as u8, 0x2F, 5, 0), // #123
    InstInfo::new(44, 45, Encoding::X86Op as u8, 0xA2, 5, 0), // #124
    InstInfo::new(45, 0, Encoding::X86Op_xDX_xAX as u8, 0x99, 22, 0), // #125
    InstInfo::new(46, 46, Encoding::X86Crc as u8, 0xF0, 12, 0), // #126
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0xE6, 7, 0), // #127
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5B, 5, 0), // #128
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0xE6, 6, 0), // #129
    InstInfo::new(47, 5, Encoding::ExtRm as u8, 0x2D, 4, 0), // #130
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5A, 4, 0), // #131
    InstInfo::new(48, 5, Encoding::ExtRm as u8, 0x2A, 4, 0), // #132
    InstInfo::new(48, 6, Encoding::ExtRm as u8, 0x2A, 5, 0), // #133
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5B, 4, 0), // #134
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5A, 5, 0), // #135
    InstInfo::new(49, 6, Encoding::ExtRm as u8, 0x2D, 5, 0), // #136
    InstInfo::new(50, 5, Encoding::ExtRm_Wx_GpqOnly as u8, 0x2D, 6, 0), // #137
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5A, 6, 0), // #138
    InstInfo::new(51, 5, Encoding::ExtRm_Wx as u8, 0x2A, 6, 0), // #139
    InstInfo::new(51, 6, Encoding::ExtRm_Wx as u8, 0x2A, 7, 0), // #140
    InstInfo::new(8, 5, Encoding::ExtRm as u8, 0x5A, 7, 0), // #141
    InstInfo::new(52, 6, Encoding::ExtRm_Wx_GpqOnly as u8, 0x2D, 7, 0), // #142
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0xE6, 4, 0), // #143
    InstInfo::new(47, 5, Encoding::ExtRm as u8, 0x2C, 4, 0), // #144
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5B, 7, 0), // #145
    InstInfo::new(49, 6, Encoding::ExtRm as u8, 0x2C, 5, 0), // #146
    InstInfo::new(50, 5, Encoding::ExtRm_Wx_GpqOnly as u8, 0x2C, 6, 0), // #147
    InstInfo::new(52, 6, Encoding::ExtRm_Wx_GpqOnly as u8, 0x2C, 7, 0), // #148
    InstInfo::new(53, 0, Encoding::X86Op_xDX_xAX as u8, 0x99, 21, 0), // #149
    InstInfo::new(54, 0, Encoding::X86Op_xAX as u8, 0x98, 0, 0), // #150
    InstInfo::new(1, 1, Encoding::X86Op as u8, 0x27, 0, 0), // #151
    InstInfo::new(1, 1, Encoding::X86Op as u8, 0x2F, 0, 0), // #152
    InstInfo::new(55, 47, Encoding::X86IncDec as u8, 0xFE, 33, 6), // #153
    InstInfo::new(56, 1, Encoding::X86M_GPB_MulDiv as u8, 0xF6, 34, 0), // #154
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5E, 4, 0), // #155
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x5E, 5, 0), // #156
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5E, 6, 0), // #157
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x5E, 7, 0), // #158
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x41, 9, 0), // #159
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x40, 9, 0), // #160
    InstInfo::new(57, 48, Encoding::X86Op as u8, 0x77, 5, 0), // #161
    InstInfo::new(31, 49, Encoding::X86Op_Mod11RM as u8, 0x1E, 35, 0), // #162
    InstInfo::new(31, 49, Encoding::X86Op_Mod11RM as u8, 0x1E, 36, 0), // #163
    InstInfo::new(58, 50, Encoding::X86EnqcmdMovdir64b as u8, 0xF8, 12, 0), // #164
    InstInfo::new(58, 50, Encoding::X86EnqcmdMovdir64b as u8, 0xF8, 8, 0), // #165
    InstInfo::new(59, 0, Encoding::X86Enter as u8, 0xC8, 0, 0), // #166
    InstInfo::new(60, 13, Encoding::ExtExtract as u8, 0x17, 9, 0), // #167
    InstInfo::new(61, 51, Encoding::ExtExtrq as u8, 0x79, 4, 7), // #168
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF0, 37, 0), // #169
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE1, 37, 0), // #170
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xC0, 38, 0), // #171
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xC0, 39, 0), // #172
    InstInfo::new(64, 52, Encoding::X86M_Only as u8, 0xDF, 40, 0), // #173
    InstInfo::new(64, 52, Encoding::X86M_Only as u8, 0xDF, 41, 0), // #174
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE0, 37, 0), // #175
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE2, 42, 0), // #176
    InstInfo::new(65, 53, Encoding::FpuR as u8, 0xC0, 43, 0), // #177
    InstInfo::new(65, 54, Encoding::FpuR as u8, 0xD0, 43, 0), // #178
    InstInfo::new(65, 55, Encoding::FpuR as u8, 0xC8, 43, 0), // #179
    InstInfo::new(65, 53, Encoding::FpuR as u8, 0xC0, 44, 0), // #180
    InstInfo::new(65, 54, Encoding::FpuR as u8, 0xD0, 44, 0), // #181
    InstInfo::new(65, 55, Encoding::FpuR as u8, 0xC8, 44, 0), // #182
    InstInfo::new(65, 56, Encoding::FpuR as u8, 0xD8, 44, 0), // #183
    InstInfo::new(65, 56, Encoding::FpuR as u8, 0xD8, 43, 0), // #184
    InstInfo::new(66, 52, Encoding::FpuCom as u8, 0xD0, 45, 0), // #185
    InstInfo::new(65, 57, Encoding::FpuR as u8, 0xF0, 44, 0), // #186
    InstInfo::new(65, 57, Encoding::FpuR as u8, 0xF0, 46, 0), // #187
    InstInfo::new(66, 52, Encoding::FpuCom as u8, 0xD8, 47, 0), // #188
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xD9, 39, 0), // #189
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFF, 37, 0), // #190
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF6, 37, 0), // #191
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xF8, 48, 0), // #192
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xF8, 39, 0), // #193
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xF0, 49, 0), // #194
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xF0, 39, 0), // #195
    InstInfo::new(31, 58, Encoding::X86Op as u8, 0x0E, 5, 0), // #196
    InstInfo::new(65, 52, Encoding::FpuR as u8, 0xC0, 50, 0), // #197
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 51, 0), // #198
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 52, 0), // #199
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 53, 0), // #200
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 41, 0), // #201
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 54, 0), // #202
    InstInfo::new(68, 52, Encoding::FpuM as u8, 0xDB, 51, 8), // #203
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 55, 0), // #204
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF7, 37, 0), // #205
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE3, 42, 0), // #206
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDB, 52, 0), // #207
    InstInfo::new(68, 52, Encoding::FpuM as u8, 0xDB, 53, 9), // #208
    InstInfo::new(68, 59, Encoding::FpuM as u8, 0xDB, 55, 10), // #209
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 40, 0), // #210
    InstInfo::new(67, 52, Encoding::FpuM as u8, 0xDA, 56, 0), // #211
    InstInfo::new(69, 52, Encoding::FpuFldFst as u8, 0xD9, 51, 11), // #212
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE8, 37, 0), // #213
    InstInfo::new(70, 52, Encoding::X86M_Only as u8, 0xD9, 56, 0), // #214
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xD9, 40, 0), // #215
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xEA, 37, 0), // #216
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE9, 37, 0), // #217
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xEC, 37, 0), // #218
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xED, 37, 0), // #219
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xEB, 37, 0), // #220
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xEE, 37, 0), // #221
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xC8, 57, 0), // #222
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xC8, 39, 0), // #223
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE2, 44, 0), // #224
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE3, 44, 0), // #225
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xD0, 37, 0), // #226
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xDD, 41, 0), // #227
    InstInfo::new(70, 52, Encoding::X86M_Only as u8, 0xD9, 54, 0), // #228
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xD9, 41, 0), // #229
    InstInfo::new(71, 52, Encoding::FpuStsw as u8, 0xDD, 54, 12), // #230
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF3, 37, 0), // #231
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF8, 37, 0), // #232
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF5, 37, 0), // #233
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF2, 37, 0), // #234
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFC, 37, 0), // #235
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xDD, 40, 0), // #236
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xDD, 58, 0), // #237
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFD, 37, 0), // #238
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFE, 37, 0), // #239
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFB, 37, 0), // #240
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xFA, 37, 0), // #241
    InstInfo::new(72, 52, Encoding::FpuFldFst as u8, 0xD9, 52, 0), // #242
    InstInfo::new(70, 52, Encoding::X86M_Only as u8, 0xD9, 59, 0), // #243
    InstInfo::new(32, 52, Encoding::X86M_Only as u8, 0xD9, 58, 0), // #244
    InstInfo::new(69, 52, Encoding::FpuFldFst as u8, 0xD9, 53, 13), // #245
    InstInfo::new(71, 52, Encoding::FpuStsw as u8, 0xDD, 59, 14), // #246
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xE8, 60, 0), // #247
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xE8, 39, 0), // #248
    InstInfo::new(62, 52, Encoding::FpuArith as u8, 0xE0, 61, 0), // #249
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xE0, 39, 0), // #250
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE4, 37, 0), // #251
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xE0, 50, 0), // #252
    InstInfo::new(65, 57, Encoding::FpuR as u8, 0xE8, 44, 0), // #253
    InstInfo::new(65, 57, Encoding::FpuR as u8, 0xE8, 46, 0), // #254
    InstInfo::new(63, 52, Encoding::FpuRDef as u8, 0xE8, 50, 0), // #255
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE9, 43, 0), // #256
    InstInfo::new(31, 52, Encoding::X86Op as u8, 0x9B, 51, 0), // #257
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xE5, 37, 0), // #258
    InstInfo::new(63, 52, Encoding::FpuR as u8, 0xC8, 37, 0), // #259
    InstInfo::new(32, 60, Encoding::X86M_Only as u8, 0xAE, 32, 0), // #260
    InstInfo::new(73, 60, Encoding::X86M_Only as u8, 0xAE, 31, 0), // #261
    InstInfo::new(32, 61, Encoding::X86M_Only as u8, 0xAE, 5, 0), // #262
    InstInfo::new(73, 61, Encoding::X86M_Only as u8, 0xAE, 62, 0), // #263
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF4, 37, 0), // #264
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF1, 37, 0), // #265
    InstInfo::new(31, 52, Encoding::FpuOp as u8, 0xF9, 37, 0), // #266
    InstInfo::new(74, 62, Encoding::X86Op as u8, 0x37, 5, 0), // #267
    InstInfo::new(9, 63, Encoding::ExtRmi as u8, 0xCF, 9, 0), // #268
    InstInfo::new(9, 63, Encoding::ExtRmi as u8, 0xCE, 9, 0), // #269
    InstInfo::new(6, 63, Encoding::ExtRm as u8, 0xCF, 2, 0), // #270
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x7C, 4, 0), // #271
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x7C, 6, 0), // #272
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0xF4, 0, 0), // #273
    InstInfo::new(75, 64, Encoding::X86Op_Mod11RM_I8 as u8, 0xF0, 63, 0), // #274
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x7D, 4, 0), // #275
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x7D, 6, 0), // #276
    InstInfo::new(56, 1, Encoding::X86M_GPB_MulDiv as u8, 0xF6, 29, 0), // #277
    InstInfo::new(76, 1, Encoding::X86Imul as u8, 0xF6, 64, 0), // #278
    InstInfo::new(77, 0, Encoding::X86In as u8, 0xEC, 0, 15), // #279
    InstInfo::new(78, 47, Encoding::X86IncDec as u8, 0xFE, 0, 16), // #280
    InstInfo::new(79, 65, Encoding::X86M as u8, 0xAE, 65, 0), // #281
    InstInfo::new(80, 65, Encoding::X86M as u8, 0xAE, 66, 0), // #282
    InstInfo::new(81, 0, Encoding::X86Ins as u8, 0x6C, 0, 0), // #283
    InstInfo::new(40, 13, Encoding::ExtRmi as u8, 0x21, 9, 0), // #284
    InstInfo::new(82, 51, Encoding::ExtInsertq as u8, 0x79, 6, 17), // #285
    InstInfo::new(83, 0, Encoding::X86Int as u8, 0xCD, 0, 0), // #286
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0xCC, 0, 0), // #287
    InstInfo::new(84, 66, Encoding::X86Op as u8, 0xCE, 0, 0), // #288
    InstInfo::new(31, 45, Encoding::X86Op as u8, 0x08, 5, 0), // #289
    InstInfo::new(85, 67, Encoding::X86Rm_NoSize as u8, 0x80, 2, 0), // #290
    InstInfo::new(32, 45, Encoding::X86M_Only as u8, 0x01, 24, 0), // #291
    InstInfo::new(86, 23, Encoding::X86Op_xAddr as u8, 0xDF, 23, 0), // #292
    InstInfo::new(87, 68, Encoding::X86Op as u8, 0xFE, 23, 0), // #293
    InstInfo::new(85, 45, Encoding::X86Rm_NoSize as u8, 0x82, 2, 0), // #294
    InstInfo::new(85, 67, Encoding::X86Rm_NoSize as u8, 0x81, 2, 0), // #295
    InstInfo::new(88, 1, Encoding::X86Op as u8, 0xCF, 21, 0), // #296
    InstInfo::new(88, 1, Encoding::X86Op as u8, 0xCF, 0, 0), // #297
    InstInfo::new(89, 1, Encoding::X86Op as u8, 0xCF, 22, 0), // #298
    InstInfo::new(90, 69, Encoding::X86Jcc as u8, 0x82, 5, 18), // #299
    InstInfo::new(90, 70, Encoding::X86Jcc as u8, 0x86, 5, 19), // #300
    InstInfo::new(91, 0, Encoding::X86JecxzLoop as u8, 0x00, 0, 20), // #301
    InstInfo::new(90, 71, Encoding::X86Jcc as u8, 0x8C, 5, 21), // #302
    InstInfo::new(90, 72, Encoding::X86Jcc as u8, 0x8E, 5, 22), // #303
    InstInfo::new(92, 0, Encoding::X86Jmp as u8, 0xFF, 10, 23), // #304
    InstInfo::new(90, 69, Encoding::X86Jcc as u8, 0x83, 5, 24), // #305
    InstInfo::new(90, 70, Encoding::X86Jcc as u8, 0x87, 5, 25), // #306
    InstInfo::new(90, 71, Encoding::X86Jcc as u8, 0x8D, 5, 26), // #307
    InstInfo::new(90, 72, Encoding::X86Jcc as u8, 0x8F, 5, 27), // #308
    InstInfo::new(90, 66, Encoding::X86Jcc as u8, 0x81, 5, 28), // #309
    InstInfo::new(90, 73, Encoding::X86Jcc as u8, 0x8B, 5, 29), // #310
    InstInfo::new(90, 74, Encoding::X86Jcc as u8, 0x89, 5, 30), // #311
    InstInfo::new(90, 75, Encoding::X86Jcc as u8, 0x85, 5, 31), // #312
    InstInfo::new(90, 66, Encoding::X86Jcc as u8, 0x80, 5, 32), // #313
    InstInfo::new(90, 73, Encoding::X86Jcc as u8, 0x8A, 5, 33), // #314
    InstInfo::new(90, 74, Encoding::X86Jcc as u8, 0x88, 5, 34), // #315
    InstInfo::new(90, 75, Encoding::X86Jcc as u8, 0x84, 5, 35), // #316
    InstInfo::new(93, 76, Encoding::VexRvm as u8, 0x4A, 67, 0), // #317
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x4A, 68, 0), // #318
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x4A, 69, 0), // #319
    InstInfo::new(93, 76, Encoding::VexRvm as u8, 0x4A, 70, 0), // #320
    InstInfo::new(93, 76, Encoding::VexRvm as u8, 0x41, 67, 0), // #321
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x41, 68, 0), // #322
    InstInfo::new(93, 76, Encoding::VexRvm as u8, 0x42, 67, 0), // #323
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x42, 68, 0), // #324
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x42, 69, 0), // #325
    InstInfo::new(93, 78, Encoding::VexRvm as u8, 0x42, 70, 0), // #326
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x41, 69, 0), // #327
    InstInfo::new(93, 78, Encoding::VexRvm as u8, 0x41, 70, 0), // #328
    InstInfo::new(94, 79, Encoding::VexKmov as u8, 0x90, 71, 36), // #329
    InstInfo::new(95, 80, Encoding::VexKmov as u8, 0x90, 72, 37), // #330
    InstInfo::new(96, 80, Encoding::VexKmov as u8, 0x90, 73, 38), // #331
    InstInfo::new(97, 81, Encoding::VexKmov as u8, 0x90, 74, 39), // #332
    InstInfo::new(98, 76, Encoding::VexRm as u8, 0x44, 71, 0), // #333
    InstInfo::new(98, 77, Encoding::VexRm as u8, 0x44, 72, 0), // #334
    InstInfo::new(98, 77, Encoding::VexRm as u8, 0x44, 73, 0), // #335
    InstInfo::new(98, 78, Encoding::VexRm as u8, 0x44, 74, 0), // #336
    InstInfo::new(93, 76, Encoding::VexRvm as u8, 0x45, 67, 0), // #337
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x45, 68, 0), // #338
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x45, 69, 0), // #339
    InstInfo::new(98, 82, Encoding::VexRm as u8, 0x98, 71, 0), // #340
    InstInfo::new(98, 83, Encoding::VexRm as u8, 0x98, 72, 0), // #341
    InstInfo::new(98, 83, Encoding::VexRm as u8, 0x98, 73, 0), // #342
    InstInfo::new(98, 84, Encoding::VexRm as u8, 0x98, 74, 0), // #343
    InstInfo::new(93, 78, Encoding::VexRvm as u8, 0x45, 70, 0), // #344
    InstInfo::new(99, 76, Encoding::VexRmi as u8, 0x32, 75, 0), // #345
    InstInfo::new(99, 77, Encoding::VexRmi as u8, 0x33, 75, 0), // #346
    InstInfo::new(99, 77, Encoding::VexRmi as u8, 0x33, 76, 0), // #347
    InstInfo::new(99, 78, Encoding::VexRmi as u8, 0x32, 76, 0), // #348
    InstInfo::new(99, 76, Encoding::VexRmi as u8, 0x30, 75, 0), // #349
    InstInfo::new(99, 77, Encoding::VexRmi as u8, 0x31, 75, 0), // #350
    InstInfo::new(99, 77, Encoding::VexRmi as u8, 0x31, 76, 0), // #351
    InstInfo::new(99, 78, Encoding::VexRmi as u8, 0x30, 76, 0), // #352
    InstInfo::new(98, 82, Encoding::VexRm as u8, 0x99, 71, 0), // #353
    InstInfo::new(98, 83, Encoding::VexRm as u8, 0x99, 72, 0), // #354
    InstInfo::new(98, 83, Encoding::VexRm as u8, 0x99, 73, 0), // #355
    InstInfo::new(98, 82, Encoding::VexRm as u8, 0x99, 74, 0), // #356
    InstInfo::new(93, 78, Encoding::VexRvm as u8, 0x4B, 67, 0), // #357
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x4B, 69, 0), // #358
    InstInfo::new(93, 77, Encoding::VexRvm as u8, 0x4B, 70, 0), // #359
    InstInfo::new(100, 76, Encoding::VexRvm as u8, 0x46, 67, 0), // #360
    InstInfo::new(100, 77, Encoding::VexRvm as u8, 0x46, 68, 0), // #361
    InstInfo::new(100, 77, Encoding::VexRvm as u8, 0x46, 69, 0), // #362
    InstInfo::new(100, 78, Encoding::VexRvm as u8, 0x46, 70, 0), // #363
    InstInfo::new(100, 76, Encoding::VexRvm as u8, 0x47, 67, 0), // #364
    InstInfo::new(100, 77, Encoding::VexRvm as u8, 0x47, 68, 0), // #365
    InstInfo::new(100, 77, Encoding::VexRvm as u8, 0x47, 69, 0), // #366
    InstInfo::new(100, 78, Encoding::VexRvm as u8, 0x47, 70, 0), // #367
    InstInfo::new(101, 85, Encoding::X86Op as u8, 0x9F, 0, 0), // #368
    InstInfo::new(102, 11, Encoding::X86Rm as u8, 0x02, 5, 0), // #369
    InstInfo::new(103, 1, Encoding::X86LcallLjmp as u8, 0xFF, 77, 40), // #370
    InstInfo::new(104, 7, Encoding::ExtRm as u8, 0xF0, 6, 0), // #371
    InstInfo::new(105, 6, Encoding::X86M_Only as u8, 0xAE, 78, 0), // #372
    InstInfo::new(106, 0, Encoding::X86Rm as u8, 0xC5, 0, 0), // #373
    InstInfo::new(107, 86, Encoding::AmxCfg as u8, 0x49, 11, 0), // #374
    InstInfo::new(108, 0, Encoding::X86Lea as u8, 0x8D, 0, 0), // #375
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0xC9, 0, 0), // #376
    InstInfo::new(106, 0, Encoding::X86Rm as u8, 0xC4, 0, 0), // #377
    InstInfo::new(31, 5, Encoding::X86Fence as u8, 0xAE, 79, 0), // #378
    InstInfo::new(109, 0, Encoding::X86Rm as u8, 0xB4, 5, 0), // #379
    InstInfo::new(32, 0, Encoding::X86M_Only as u8, 0x01, 78, 0), // #380
    InstInfo::new(109, 0, Encoding::X86Rm as u8, 0xB5, 5, 0), // #381
    InstInfo::new(32, 0, Encoding::X86M_Only as u8, 0x01, 80, 0), // #382
    InstInfo::new(110, 0, Encoding::X86LcallLjmp as u8, 0xFF, 64, 41), // #383
    InstInfo::new(111, 0, Encoding::X86M_NoSize as u8, 0x00, 78, 0), // #384
    InstInfo::new(112, 87, Encoding::VexR_Wx as u8, 0x12, 81, 0), // #385
    InstInfo::new(111, 0, Encoding::X86M_NoSize as u8, 0x01, 82, 0), // #386
    InstInfo::new(113, 88, Encoding::X86StrRm as u8, 0xAC, 0, 0), // #387
    InstInfo::new(114, 0, Encoding::X86JecxzLoop as u8, 0x00, 0, 42), // #388
    InstInfo::new(114, 75, Encoding::X86JecxzLoop as u8, 0x00, 0, 43), // #389
    InstInfo::new(114, 75, Encoding::X86JecxzLoop as u8, 0x00, 0, 44), // #390
    InstInfo::new(115, 11, Encoding::X86Rm as u8, 0x03, 5, 0), // #391
    InstInfo::new(109, 0, Encoding::X86Rm as u8, 0xB2, 5, 0), // #392
    InstInfo::new(111, 0, Encoding::X86M_NoSize as u8, 0x00, 80, 0), // #393
    InstInfo::new(116, 87, Encoding::VexVmi4_Wx as u8, 0x12, 83, 0), // #394
    InstInfo::new(116, 87, Encoding::VexVmi4_Wx as u8, 0x12, 84, 0), // #395
    InstInfo::new(23, 89, Encoding::X86Rm_Raw66H as u8, 0xBD, 7, 0), // #396
    InstInfo::new(117, 5, Encoding::ExtRm_ZDI as u8, 0xF7, 4, 0), // #397
    InstInfo::new(118, 90, Encoding::ExtRm_ZDI as u8, 0xF7, 5, 0), // #398
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5F, 4, 0), // #399
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x5F, 5, 0), // #400
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5F, 6, 0), // #401
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x5F, 7, 0), // #402
    InstInfo::new(31, 91, Encoding::X86Op as u8, 0xFA, 27, 0), // #403
    InstInfo::new(31, 5, Encoding::X86Fence as u8, 0xAE, 82, 0), // #404
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5D, 4, 0), // #405
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x5D, 5, 0), // #406
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5D, 6, 0), // #407
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x5D, 7, 0), // #408
    InstInfo::new(119, 92, Encoding::X86Op as u8, 0xC8, 23, 0), // #409
    InstInfo::new(119, 93, Encoding::X86Op as u8, 0xFA, 23, 0), // #410
    InstInfo::new(120, 94, Encoding::X86Mov as u8, 0x00, 0, 0), // #411
    InstInfo::new(121, 0, Encoding::X86Movabs as u8, 0x00, 0, 0), // #412
    InstInfo::new(122, 95, Encoding::ExtMov as u8, 0x28, 4, 45), // #413
    InstInfo::new(122, 96, Encoding::ExtMov as u8, 0x28, 5, 46), // #414
    InstInfo::new(123, 97, Encoding::ExtMovbe as u8, 0xF0, 1, 47), // #415
    InstInfo::new(124, 98, Encoding::ExtMovd as u8, 0x6E, 5, 48), // #416
    InstInfo::new(7, 7, Encoding::ExtMov as u8, 0x12, 6, 0), // #417
    InstInfo::new(125, 99, Encoding::X86EnqcmdMovdir64b as u8, 0xF8, 2, 0), // #418
    InstInfo::new(3, 100, Encoding::X86MovntiMovdiri as u8, 0xF9, 1, 0), // #419
    InstInfo::new(126, 5, Encoding::ExtMov as u8, 0xD6, 6, 0), // #420
    InstInfo::new(122, 95, Encoding::ExtMov as u8, 0x6F, 4, 49), // #421
    InstInfo::new(122, 95, Encoding::ExtMov as u8, 0x6F, 7, 50), // #422
    InstInfo::new(127, 6, Encoding::ExtMov as u8, 0x12, 5, 0), // #423
    InstInfo::new(128, 5, Encoding::ExtMov as u8, 0x16, 4, 51), // #424
    InstInfo::new(128, 6, Encoding::ExtMov as u8, 0x16, 5, 52), // #425
    InstInfo::new(127, 6, Encoding::ExtMov as u8, 0x16, 5, 0), // #426
    InstInfo::new(128, 5, Encoding::ExtMov as u8, 0x12, 4, 53), // #427
    InstInfo::new(128, 6, Encoding::ExtMov as u8, 0x12, 5, 54), // #428
    InstInfo::new(129, 5, Encoding::ExtMov as u8, 0x50, 4, 0), // #429
    InstInfo::new(129, 6, Encoding::ExtMov as u8, 0x50, 5, 0), // #430
    InstInfo::new(130, 5, Encoding::ExtMov as u8, 0x00, 0, 55), // #431
    InstInfo::new(104, 13, Encoding::ExtMov as u8, 0x2A, 2, 0), // #432
    InstInfo::new(3, 5, Encoding::X86MovntiMovdiri as u8, 0xC3, 5, 0), // #433
    InstInfo::new(130, 5, Encoding::ExtMov as u8, 0x00, 0, 56), // #434
    InstInfo::new(130, 6, Encoding::ExtMov as u8, 0x00, 0, 57), // #435
    InstInfo::new(131, 90, Encoding::ExtMov as u8, 0x00, 0, 58), // #436
    InstInfo::new(132, 51, Encoding::ExtMov as u8, 0x00, 0, 59), // #437
    InstInfo::new(133, 51, Encoding::ExtMov as u8, 0x00, 0, 60), // #438
    InstInfo::new(134, 101, Encoding::ExtMovq as u8, 0x6E, 5, 48), // #439
    InstInfo::new(135, 5, Encoding::ExtRm as u8, 0xD6, 7, 0), // #440
    InstInfo::new(136, 88, Encoding::X86StrMm as u8, 0xA4, 0, 0), // #441
    InstInfo::new(137, 95, Encoding::ExtMov as u8, 0x10, 6, 61), // #442
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x16, 7, 0), // #443
    InstInfo::new(6, 7, Encoding::ExtRm as u8, 0x12, 7, 0), // #444
    InstInfo::new(138, 96, Encoding::ExtMov as u8, 0x10, 7, 62), // #445
    InstInfo::new(139, 0, Encoding::X86MovsxMovzx as u8, 0xBE, 5, 0), // #446
    InstInfo::new(140, 0, Encoding::X86Rm as u8, 0x63, 0, 0), // #447
    InstInfo::new(122, 95, Encoding::ExtMov as u8, 0x10, 4, 63), // #448
    InstInfo::new(122, 96, Encoding::ExtMov as u8, 0x10, 5, 64), // #449
    InstInfo::new(139, 0, Encoding::X86MovsxMovzx as u8, 0xB6, 5, 0), // #450
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x42, 9, 0), // #451
    InstInfo::new(56, 1, Encoding::X86M_GPB_MulDiv as u8, 0xF6, 10, 0), // #452
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x59, 4, 0), // #453
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x59, 5, 0), // #454
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x59, 6, 0), // #455
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x59, 7, 0), // #456
    InstInfo::new(141, 102, Encoding::VexRvm_ZDX_Wx as u8, 0xF6, 85, 0), // #457
    InstInfo::new(142, 92, Encoding::X86Op as u8, 0xC9, 23, 0), // #458
    InstInfo::new(143, 93, Encoding::X86Op as u8, 0xFB, 23, 0), // #459
    InstInfo::new(144, 1, Encoding::X86M_GPB as u8, 0xF6, 77, 0), // #460
    InstInfo::new(145, 0, Encoding::X86M_Nop as u8, 0x90, 0, 0), // #461
    InstInfo::new(144, 0, Encoding::X86M_GPB as u8, 0xF6, 3, 0), // #462
    InstInfo::new(146, 1, Encoding::X86Arith as u8, 0x08, 33, 0), // #463
    InstInfo::new(12, 5, Encoding::ExtRm as u8, 0x56, 4, 0), // #464
    InstInfo::new(12, 6, Encoding::ExtRm as u8, 0x56, 5, 0), // #465
    InstInfo::new(147, 0, Encoding::X86Out as u8, 0xEE, 0, 65), // #466
    InstInfo::new(148, 0, Encoding::X86Outs as u8, 0x6E, 0, 0), // #467
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x1C, 1, 0), // #468
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x1E, 1, 0), // #469
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x1D, 1, 0), // #470
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x6B, 5, 0), // #471
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x63, 5, 0), // #472
    InstInfo::new(6, 13, Encoding::ExtRm as u8, 0x2B, 2, 0), // #473
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x67, 5, 0), // #474
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xFC, 5, 0), // #475
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xFE, 5, 0), // #476
    InstInfo::new(149, 5, Encoding::ExtRm_P as u8, 0xD4, 5, 0), // #477
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xEC, 5, 0), // #478
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xED, 5, 0), // #479
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xDC, 5, 0), // #480
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xDD, 5, 0), // #481
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xFD, 5, 0), // #482
    InstInfo::new(150, 103, Encoding::ExtRmi_P as u8, 0x0F, 86, 0), // #483
    InstInfo::new(151, 98, Encoding::ExtRm_P as u8, 0xDB, 5, 0), // #484
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xDF, 5, 0), // #485
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0x90, 87, 0), // #486
    InstInfo::new(149, 104, Encoding::ExtRm_P as u8, 0xE0, 5, 0), // #487
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xBF, 88, 0), // #488
    InstInfo::new(149, 104, Encoding::ExtRm_P as u8, 0xE3, 5, 0), // #489
    InstInfo::new(16, 13, Encoding::ExtRm_XMM0 as u8, 0x10, 2, 0), // #490
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x0E, 9, 0), // #491
    InstInfo::new(9, 105, Encoding::ExtRmi as u8, 0x44, 9, 0), // #492
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x74, 5, 0), // #493
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x76, 5, 0), // #494
    InstInfo::new(154, 13, Encoding::ExtRm as u8, 0x29, 2, 0), // #495
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x75, 5, 0), // #496
    InstInfo::new(155, 106, Encoding::ExtRmi as u8, 0x61, 9, 0), // #497
    InstInfo::new(156, 106, Encoding::ExtRmi as u8, 0x60, 9, 0), // #498
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x64, 5, 0), // #499
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x66, 5, 0), // #500
    InstInfo::new(154, 46, Encoding::ExtRm as u8, 0x37, 2, 0), // #501
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0x65, 5, 0), // #502
    InstInfo::new(157, 106, Encoding::ExtRmi as u8, 0x63, 9, 0), // #503
    InstInfo::new(158, 106, Encoding::ExtRmi as u8, 0x62, 9, 0), // #504
    InstInfo::new(31, 107, Encoding::X86Op as u8, 0xC5, 23, 0), // #505
    InstInfo::new(11, 102, Encoding::VexRvm_Wx as u8, 0xF5, 85, 0), // #506
    InstInfo::new(11, 102, Encoding::VexRvm_Wx as u8, 0xF5, 89, 0), // #507
    InstInfo::new(159, 13, Encoding::ExtExtract as u8, 0x14, 86, 0), // #508
    InstInfo::new(60, 13, Encoding::ExtExtract as u8, 0x16, 86, 0), // #509
    InstInfo::new(160, 13, Encoding::ExtExtract as u8, 0x16, 90, 0), // #510
    InstInfo::new(161, 108, Encoding::ExtPextrw as u8, 0xC5, 5, 66), // #511
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x1D, 88, 0), // #512
    InstInfo::new(153, 109, Encoding::Ext3dNow as u8, 0x1C, 88, 0), // #513
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xAE, 88, 0), // #514
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x9E, 88, 0), // #515
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xB0, 88, 0), // #516
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x90, 88, 0), // #517
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xA0, 88, 0), // #518
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xA4, 88, 0), // #519
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x94, 88, 0), // #520
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xB4, 88, 0), // #521
    InstInfo::new(153, 109, Encoding::Ext3dNow as u8, 0x8A, 88, 0), // #522
    InstInfo::new(153, 109, Encoding::Ext3dNow as u8, 0x8E, 88, 0), // #523
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x96, 88, 0), // #524
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xA6, 88, 0), // #525
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xB6, 88, 0), // #526
    InstInfo::new(153, 110, Encoding::Ext3dNow as u8, 0x86, 88, 0), // #527
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xA7, 88, 0), // #528
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x97, 88, 0), // #529
    InstInfo::new(153, 110, Encoding::Ext3dNow as u8, 0x87, 88, 0), // #530
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x9A, 88, 0), // #531
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xAA, 88, 0), // #532
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x02, 1, 0), // #533
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x03, 1, 0), // #534
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x01, 1, 0), // #535
    InstInfo::new(6, 13, Encoding::ExtRm as u8, 0x41, 2, 0), // #536
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x06, 1, 0), // #537
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x07, 1, 0), // #538
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x05, 1, 0), // #539
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0x0D, 88, 0), // #540
    InstInfo::new(153, 109, Encoding::Ext3dNow as u8, 0x0C, 88, 0), // #541
    InstInfo::new(162, 13, Encoding::ExtRmi as u8, 0x20, 9, 0), // #542
    InstInfo::new(163, 13, Encoding::ExtRmi as u8, 0x22, 9, 0), // #543
    InstInfo::new(164, 13, Encoding::ExtRmi as u8, 0x22, 91, 0), // #544
    InstInfo::new(165, 104, Encoding::ExtRmi_P as u8, 0xC4, 5, 0), // #545
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x04, 1, 0), // #546
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xF5, 5, 0), // #547
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3C, 2, 0), // #548
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3D, 2, 0), // #549
    InstInfo::new(151, 104, Encoding::ExtRm_P as u8, 0xEE, 5, 0), // #550
    InstInfo::new(151, 104, Encoding::ExtRm_P as u8, 0xDE, 5, 0), // #551
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3F, 2, 0), // #552
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3E, 2, 0), // #553
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x38, 2, 0), // #554
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x39, 2, 0), // #555
    InstInfo::new(151, 104, Encoding::ExtRm_P as u8, 0xEA, 5, 0), // #556
    InstInfo::new(151, 104, Encoding::ExtRm_P as u8, 0xDA, 5, 0), // #557
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3B, 2, 0), // #558
    InstInfo::new(12, 13, Encoding::ExtRm as u8, 0x3A, 2, 0), // #559
    InstInfo::new(166, 104, Encoding::ExtRm_P as u8, 0xD7, 5, 0), // #560
    InstInfo::new(8, 13, Encoding::ExtRm as u8, 0x21, 2, 0), // #561
    InstInfo::new(167, 13, Encoding::ExtRm as u8, 0x22, 2, 0), // #562
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x20, 2, 0), // #563
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x25, 2, 0), // #564
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x23, 2, 0), // #565
    InstInfo::new(8, 13, Encoding::ExtRm as u8, 0x24, 2, 0), // #566
    InstInfo::new(8, 13, Encoding::ExtRm as u8, 0x31, 2, 0), // #567
    InstInfo::new(167, 13, Encoding::ExtRm as u8, 0x32, 2, 0), // #568
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x30, 2, 0), // #569
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x35, 2, 0), // #570
    InstInfo::new(7, 13, Encoding::ExtRm as u8, 0x33, 2, 0), // #571
    InstInfo::new(8, 13, Encoding::ExtRm as u8, 0x34, 2, 0), // #572
    InstInfo::new(6, 13, Encoding::ExtRm as u8, 0x28, 2, 0), // #573
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x0B, 1, 0), // #574
    InstInfo::new(153, 58, Encoding::Ext3dNow as u8, 0xB7, 88, 0), // #575
    InstInfo::new(149, 104, Encoding::ExtRm_P as u8, 0xE4, 5, 0), // #576
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xE5, 5, 0), // #577
    InstInfo::new(6, 13, Encoding::ExtRm as u8, 0x40, 2, 0), // #578
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0xD5, 5, 0), // #579
    InstInfo::new(149, 5, Encoding::ExtRm_P as u8, 0xF4, 5, 0), // #580
    InstInfo::new(168, 0, Encoding::X86Pop as u8, 0x8F, 0, 67), // #581
    InstInfo::new(84, 0, Encoding::X86Op as u8, 0x61, 21, 0), // #582
    InstInfo::new(84, 0, Encoding::X86Op as u8, 0x61, 0, 0), // #583
    InstInfo::new(23, 111, Encoding::X86Rm_Raw66H as u8, 0xB8, 7, 0), // #584
    InstInfo::new(31, 112, Encoding::X86Op as u8, 0x9D, 21, 0), // #585
    InstInfo::new(84, 112, Encoding::X86Op as u8, 0x9D, 0, 0), // #586
    InstInfo::new(34, 112, Encoding::X86Op as u8, 0x9D, 0, 0), // #587
    InstInfo::new(151, 98, Encoding::ExtRm_P as u8, 0xEB, 5, 0), // #588
    InstInfo::new(32, 58, Encoding::X86M_Only as u8, 0x0D, 5, 0), // #589
    InstInfo::new(73, 113, Encoding::X86M_Only as u8, 0x18, 24, 0), // #590
    InstInfo::new(73, 113, Encoding::X86M_Only as u8, 0x18, 82, 0), // #591
    InstInfo::new(32, 6, Encoding::X86M_Only as u8, 0x18, 5, 0), // #592
    InstInfo::new(32, 6, Encoding::X86M_Only as u8, 0x18, 32, 0), // #593
    InstInfo::new(32, 6, Encoding::X86M_Only as u8, 0x18, 78, 0), // #594
    InstInfo::new(32, 6, Encoding::X86M_Only as u8, 0x18, 80, 0), // #595
    InstInfo::new(32, 114, Encoding::X86M_Only as u8, 0x0D, 32, 0), // #596
    InstInfo::new(32, 115, Encoding::X86M_Only as u8, 0x0D, 78, 0), // #597
    InstInfo::new(149, 104, Encoding::ExtRm_P as u8, 0xF6, 5, 0), // #598
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x00, 1, 0), // #599
    InstInfo::new(9, 5, Encoding::ExtRmi as u8, 0x70, 4, 0), // #600
    InstInfo::new(9, 5, Encoding::ExtRmi as u8, 0x70, 7, 0), // #601
    InstInfo::new(9, 5, Encoding::ExtRmi as u8, 0x70, 6, 0), // #602
    InstInfo::new(169, 90, Encoding::ExtRmi_P as u8, 0x70, 5, 0), // #603
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x08, 1, 0), // #604
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x0A, 1, 0), // #605
    InstInfo::new(149, 103, Encoding::ExtRm_P as u8, 0x09, 1, 0), // #606
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xF2, 5, 68), // #607
    InstInfo::new(171, 5, Encoding::ExtRmRi as u8, 0x00, 0, 69), // #608
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xF3, 5, 70), // #609
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xF1, 5, 71), // #610
    InstInfo::new(34, 116, Encoding::X86Op as u8, 0xFF, 27, 0), // #611
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xE2, 5, 72), // #612
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xE1, 5, 73), // #613
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xD2, 5, 74), // #614
    InstInfo::new(171, 5, Encoding::ExtRmRi as u8, 0x00, 0, 75), // #615
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xD3, 5, 76), // #616
    InstInfo::new(170, 98, Encoding::ExtRmRi_P as u8, 0xD1, 5, 77), // #617
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xF8, 5, 0), // #618
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xFA, 5, 0), // #619
    InstInfo::new(152, 5, Encoding::ExtRm_P as u8, 0xFB, 5, 0), // #620
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xE8, 5, 0), // #621
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xE9, 5, 0), // #622
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xD8, 5, 0), // #623
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xD9, 5, 0), // #624
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xF9, 5, 0), // #625
    InstInfo::new(153, 109, Encoding::Ext3dNow as u8, 0xBB, 88, 0), // #626
    InstInfo::new(6, 117, Encoding::ExtRm as u8, 0x17, 2, 0), // #627
    InstInfo::new(172, 118, Encoding::X86M as u8, 0xAE, 92, 0), // #628
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x68, 5, 0), // #629
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x6A, 5, 0), // #630
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x6D, 4, 0), // #631
    InstInfo::new(149, 98, Encoding::ExtRm_P as u8, 0x69, 5, 0), // #632
    InstInfo::new(173, 98, Encoding::ExtRm_P as u8, 0x60, 5, 0), // #633
    InstInfo::new(173, 98, Encoding::ExtRm_P as u8, 0x62, 5, 0), // #634
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x6C, 4, 0), // #635
    InstInfo::new(173, 98, Encoding::ExtRm_P as u8, 0x61, 5, 0), // #636
    InstInfo::new(174, 0, Encoding::X86Push as u8, 0xFF, 34, 78), // #637
    InstInfo::new(84, 0, Encoding::X86Op as u8, 0x60, 21, 0), // #638
    InstInfo::new(84, 0, Encoding::X86Op as u8, 0x60, 0, 0), // #639
    InstInfo::new(31, 119, Encoding::X86Op as u8, 0x9C, 21, 0), // #640
    InstInfo::new(84, 119, Encoding::X86Op as u8, 0x9C, 0, 0), // #641
    InstInfo::new(34, 119, Encoding::X86Op as u8, 0x9C, 0, 0), // #642
    InstInfo::new(175, 0, Encoding::X86Pushw as u8, 0xFF, 34, 78), // #643
    InstInfo::new(31, 120, Encoding::X86Op as u8, 0xFF, 93, 0), // #644
    InstInfo::new(152, 98, Encoding::ExtRm_P as u8, 0xEF, 5, 0), // #645
    InstInfo::new(176, 121, Encoding::X86Rot as u8, 0xD0, 3, 0), // #646
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x53, 5, 0), // #647
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x53, 7, 0), // #648
    InstInfo::new(176, 121, Encoding::X86Rot as u8, 0xD0, 77, 0), // #649
    InstInfo::new(177, 122, Encoding::X86M as u8, 0xAE, 7, 0), // #650
    InstInfo::new(177, 122, Encoding::X86M as u8, 0xAE, 94, 0), // #651
    InstInfo::new(178, 123, Encoding::X86Op as u8, 0x32, 5, 0), // #652
    InstInfo::new(179, 124, Encoding::X86R_Native as u8, 0xC7, 95, 0), // #653
    InstInfo::new(180, 125, Encoding::X86Op as u8, 0xEE, 23, 0), // #654
    InstInfo::new(180, 0, Encoding::X86Op as u8, 0x33, 5, 0), // #655
    InstInfo::new(180, 126, Encoding::X86Op as u8, 0xFD, 23, 0), // #656
    InstInfo::new(24, 127, Encoding::X86M as u8, 0xC7, 82, 0), // #657
    InstInfo::new(24, 128, Encoding::X86M as u8, 0xC7, 24, 0), // #658
    InstInfo::new(79, 65, Encoding::X86M as u8, 0x1E, 94, 0), // #659
    InstInfo::new(80, 65, Encoding::X86M as u8, 0x1E, 94, 0), // #660
    InstInfo::new(29, 129, Encoding::X86Op as u8, 0x31, 5, 0), // #661
    InstInfo::new(180, 130, Encoding::X86Op as u8, 0xF9, 23, 0), // #662
    InstInfo::new(181, 0, Encoding::X86Ret as u8, 0xC2, 0, 0), // #663
    InstInfo::new(182, 0, Encoding::X86Ret as u8, 0xCA, 0, 0), // #664
    InstInfo::new(34, 116, Encoding::X86Op as u8, 0xFE, 27, 0), // #665
    InstInfo::new(34, 116, Encoding::X86Op as u8, 0xFE, 93, 0), // #666
    InstInfo::new(176, 131, Encoding::X86Rot as u8, 0xD0, 0, 0), // #667
    InstInfo::new(176, 131, Encoding::X86Rot as u8, 0xD0, 33, 0), // #668
    InstInfo::new(183, 102, Encoding::VexRmi_Wx as u8, 0xF0, 96, 0), // #669
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x09, 9, 0), // #670
    InstInfo::new(9, 13, Encoding::ExtRmi as u8, 0x08, 9, 0), // #671
    InstInfo::new(39, 13, Encoding::ExtRmi as u8, 0x0B, 9, 0), // #672
    InstInfo::new(40, 13, Encoding::ExtRmi as u8, 0x0A, 9, 0), // #673
    InstInfo::new(84, 1, Encoding::X86Op as u8, 0xAA, 5, 0), // #674
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x52, 5, 0), // #675
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x52, 7, 0), // #676
    InstInfo::new(33, 25, Encoding::X86M_Only as u8, 0x01, 65, 0), // #677
    InstInfo::new(101, 132, Encoding::X86Op as u8, 0x9E, 0, 0), // #678
    InstInfo::new(176, 1, Encoding::X86Rot as u8, 0xD0, 29, 0), // #679
    InstInfo::new(14, 102, Encoding::VexRmv_Wx as u8, 0xF7, 89, 0), // #680
    InstInfo::new(31, 25, Encoding::X86Op as u8, 0xEA, 27, 0), // #681
    InstInfo::new(184, 3, Encoding::X86Arith as u8, 0x18, 77, 0), // #682
    InstInfo::new(185, 39, Encoding::X86StrRm as u8, 0xAE, 0, 0), // #683
    InstInfo::new(31, 133, Encoding::X86Op as u8, 0xCF, 97, 0), // #684
    InstInfo::new(31, 133, Encoding::X86Op as u8, 0xCE, 97, 0), // #685
    InstInfo::new(31, 133, Encoding::X86Op as u8, 0xCD, 97, 0), // #686
    InstInfo::new(80, 26, Encoding::X86M_NoSize as u8, 0xC7, 26, 0), // #687
    InstInfo::new(31, 134, Encoding::X86Op as u8, 0xE8, 23, 0), // #688
    InstInfo::new(186, 69, Encoding::X86Set as u8, 0x92, 5, 0), // #689
    InstInfo::new(186, 70, Encoding::X86Set as u8, 0x96, 5, 0), // #690
    InstInfo::new(186, 71, Encoding::X86Set as u8, 0x9C, 5, 0), // #691
    InstInfo::new(186, 72, Encoding::X86Set as u8, 0x9E, 5, 0), // #692
    InstInfo::new(186, 69, Encoding::X86Set as u8, 0x93, 5, 0), // #693
    InstInfo::new(186, 70, Encoding::X86Set as u8, 0x97, 5, 0), // #694
    InstInfo::new(186, 71, Encoding::X86Set as u8, 0x9D, 5, 0), // #695
    InstInfo::new(186, 72, Encoding::X86Set as u8, 0x9F, 5, 0), // #696
    InstInfo::new(186, 66, Encoding::X86Set as u8, 0x91, 5, 0), // #697
    InstInfo::new(186, 73, Encoding::X86Set as u8, 0x9B, 5, 0), // #698
    InstInfo::new(186, 74, Encoding::X86Set as u8, 0x99, 5, 0), // #699
    InstInfo::new(186, 75, Encoding::X86Set as u8, 0x95, 5, 0), // #700
    InstInfo::new(186, 66, Encoding::X86Set as u8, 0x90, 5, 0), // #701
    InstInfo::new(186, 73, Encoding::X86Set as u8, 0x9A, 5, 0), // #702
    InstInfo::new(186, 74, Encoding::X86Set as u8, 0x98, 5, 0), // #703
    InstInfo::new(31, 65, Encoding::X86Op as u8, 0xE8, 27, 0), // #704
    InstInfo::new(186, 75, Encoding::X86Set as u8, 0x94, 5, 0), // #705
    InstInfo::new(31, 6, Encoding::X86Fence as u8, 0xAE, 24, 0), // #706
    InstInfo::new(32, 0, Encoding::X86M_Only as u8, 0x01, 5, 0), // #707
    InstInfo::new(6, 135, Encoding::ExtRm as u8, 0xC9, 1, 0), // #708
    InstInfo::new(6, 135, Encoding::ExtRm as u8, 0xCA, 1, 0), // #709
    InstInfo::new(6, 135, Encoding::ExtRm as u8, 0xC8, 1, 0), // #710
    InstInfo::new(9, 135, Encoding::ExtRmi as u8, 0xCC, 86, 0), // #711
    InstInfo::new(6, 135, Encoding::ExtRm as u8, 0xCC, 1, 0), // #712
    InstInfo::new(6, 135, Encoding::ExtRm as u8, 0xCD, 1, 0), // #713
    InstInfo::new(16, 135, Encoding::ExtRm_XMM0 as u8, 0xCB, 1, 0), // #714
    InstInfo::new(176, 1, Encoding::X86Rot as u8, 0xD0, 10, 0), // #715
    InstInfo::new(187, 1, Encoding::X86ShldShrd as u8, 0xA4, 5, 0), // #716
    InstInfo::new(14, 102, Encoding::VexRmv_Wx as u8, 0xF7, 30, 0), // #717
    InstInfo::new(176, 1, Encoding::X86Rot as u8, 0xD0, 64, 0), // #718
    InstInfo::new(187, 1, Encoding::X86ShldShrd as u8, 0xAC, 5, 0), // #719
    InstInfo::new(14, 102, Encoding::VexRmv_Wx as u8, 0xF7, 85, 0), // #720
    InstInfo::new(9, 5, Encoding::ExtRmi as u8, 0xC6, 4, 0), // #721
    InstInfo::new(9, 6, Encoding::ExtRmi as u8, 0xC6, 5, 0), // #722
    InstInfo::new(32, 0, Encoding::X86M_Only as u8, 0x01, 32, 0), // #723
    InstInfo::new(54, 136, Encoding::X86Op_xAX as u8, 0xDE, 23, 0), // #724
    InstInfo::new(188, 0, Encoding::X86M_NoMemSize as u8, 0x00, 5, 0), // #725
    InstInfo::new(112, 87, Encoding::VexR_Wx as u8, 0x12, 13, 0), // #726
    InstInfo::new(188, 0, Encoding::X86M_NoMemSize as u8, 0x01, 98, 0), // #727
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x51, 4, 0), // #728
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x51, 5, 0), // #729
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x51, 6, 0), // #730
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x51, 7, 0), // #731
    InstInfo::new(31, 17, Encoding::X86Op as u8, 0xCB, 23, 0), // #732
    InstInfo::new(31, 18, Encoding::X86Op as u8, 0xF9, 0, 0), // #733
    InstInfo::new(31, 19, Encoding::X86Op as u8, 0xFD, 0, 0), // #734
    InstInfo::new(31, 136, Encoding::X86Op as u8, 0xDC, 23, 0), // #735
    InstInfo::new(31, 24, Encoding::X86Op as u8, 0xFB, 0, 0), // #736
    InstInfo::new(105, 6, Encoding::X86M_Only as u8, 0xAE, 80, 0), // #737
    InstInfo::new(189, 88, Encoding::X86StrMr as u8, 0xAA, 0, 0), // #738
    InstInfo::new(188, 0, Encoding::X86M_NoMemSize as u8, 0x00, 32, 0), // #739
    InstInfo::new(107, 86, Encoding::AmxCfg as u8, 0x49, 30, 0), // #740
    InstInfo::new(34, 26, Encoding::X86Op as u8, 0xEF, 27, 0), // #741
    InstInfo::new(184, 1, Encoding::X86Arith as u8, 0x28, 64, 0), // #742
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x5C, 4, 0), // #743
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x5C, 5, 0), // #744
    InstInfo::new(7, 5, Encoding::ExtRm as u8, 0x5C, 6, 0), // #745
    InstInfo::new(8, 6, Encoding::ExtRm as u8, 0x5C, 7, 0), // #746
    InstInfo::new(34, 0, Encoding::X86Op as u8, 0xF8, 23, 0), // #747
    InstInfo::new(34, 0, Encoding::X86Op as u8, 0x05, 5, 0), // #748
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0x34, 5, 0), // #749
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0x35, 5, 0), // #750
    InstInfo::new(34, 0, Encoding::X86Op as u8, 0x35, 62, 0), // #751
    InstInfo::new(34, 0, Encoding::X86Op as u8, 0x07, 5, 0), // #752
    InstInfo::new(34, 0, Encoding::X86Op as u8, 0x07, 62, 0), // #753
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 99, 0), // #754
    InstInfo::new(190, 137, Encoding::AmxRmv as u8, 0x6C, 30, 0), // #755
    InstInfo::new(190, 137, Encoding::AmxRmv as u8, 0x6C, 11, 0), // #756
    InstInfo::new(31, 133, Encoding::X86Op as u8, 0xCC, 97, 0), // #757
    InstInfo::new(190, 138, Encoding::AmxRmv as u8, 0x5C, 89, 0), // #758
    InstInfo::new(190, 139, Encoding::AmxRmv as u8, 0x5E, 85, 0), // #759
    InstInfo::new(190, 139, Encoding::AmxRmv as u8, 0x5E, 89, 0), // #760
    InstInfo::new(190, 139, Encoding::AmxRmv as u8, 0x5E, 30, 0), // #761
    InstInfo::new(190, 139, Encoding::AmxRmv as u8, 0x5E, 11, 0), // #762
    InstInfo::new(190, 140, Encoding::AmxRmv as u8, 0x5C, 85, 0), // #763
    InstInfo::new(191, 1, Encoding::X86Test as u8, 0x84, 0, 79), // #764
    InstInfo::new(34, 141, Encoding::X86Op as u8, 0xED, 27, 0), // #765
    InstInfo::new(192, 86, Encoding::AmxRm as u8, 0x4B, 85, 0), // #766
    InstInfo::new(192, 86, Encoding::AmxRm as u8, 0x4B, 30, 0), // #767
    InstInfo::new(193, 86, Encoding::VexOpMod as u8, 0x49, 11, 0), // #768
    InstInfo::new(194, 86, Encoding::AmxMr as u8, 0x4B, 89, 0), // #769
    InstInfo::new(195, 86, Encoding::AmxR as u8, 0x49, 85, 0), // #770
    InstInfo::new(31, 68, Encoding::X86Op as u8, 0xFF, 23, 0), // #771
    InstInfo::new(196, 142, Encoding::X86R32_EDX_EAX as u8, 0xAE, 28, 0), // #772
    InstInfo::new(23, 10, Encoding::X86Rm_Raw66H as u8, 0xBC, 7, 0), // #773
    InstInfo::new(15, 12, Encoding::VexVm_Wx as u8, 0x01, 100, 0), // #774
    InstInfo::new(7, 43, Encoding::ExtRm as u8, 0x2E, 4, 0), // #775
    InstInfo::new(8, 44, Encoding::ExtRm as u8, 0x2E, 5, 0), // #776
    InstInfo::new(197, 0, Encoding::X86Rm as u8, 0xFF, 5, 0), // #777
    InstInfo::new(197, 0, Encoding::X86Rm as u8, 0xB9, 5, 0), // #778
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0x0B, 5, 0), // #779
    InstInfo::new(34, 26, Encoding::X86Op as u8, 0xEC, 27, 0), // #780
    InstInfo::new(198, 143, Encoding::X86R_FromM as u8, 0xAE, 26, 0), // #781
    InstInfo::new(196, 142, Encoding::X86R32_EDX_EAX as u8, 0xAE, 101, 0), // #782
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x15, 4, 0), // #783
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x15, 5, 0), // #784
    InstInfo::new(6, 5, Encoding::ExtRm as u8, 0x14, 4, 0), // #785
    InstInfo::new(6, 6, Encoding::ExtRm as u8, 0x14, 5, 0), // #786
    InstInfo::new(199, 144, Encoding::VexRvm_Lx as u8, 0x58, 102, 0), // #787
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x58, 103, 0), // #788
    InstInfo::new(201, 144, Encoding::VexRvm_Lx as u8, 0x58, 104, 0), // #789
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x58, 105, 0), // #790
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x58, 106, 0), // #791
    InstInfo::new(204, 144, Encoding::VexRvm as u8, 0x58, 107, 0), // #792
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0xD0, 71, 0), // #793
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0xD0, 108, 0), // #794
    InstInfo::new(206, 147, Encoding::VexRvm_Lx as u8, 0xDE, 109, 0), // #795
    InstInfo::new(206, 147, Encoding::VexRvm_Lx as u8, 0xDF, 109, 0), // #796
    InstInfo::new(206, 147, Encoding::VexRvm_Lx as u8, 0xDC, 109, 0), // #797
    InstInfo::new(206, 147, Encoding::VexRvm_Lx as u8, 0xDD, 109, 0), // #798
    InstInfo::new(207, 148, Encoding::VexRm as u8, 0xDB, 30, 0), // #799
    InstInfo::new(208, 148, Encoding::VexRmi as u8, 0xDF, 75, 0), // #800
    InstInfo::new(209, 149, Encoding::VexRvmi_Lx as u8, 0x03, 110, 0), // #801
    InstInfo::new(210, 149, Encoding::VexRvmi_Lx as u8, 0x03, 111, 0), // #802
    InstInfo::new(211, 150, Encoding::VexRvm_Lx as u8, 0x55, 102, 0), // #803
    InstInfo::new(212, 150, Encoding::VexRvm_Lx as u8, 0x55, 104, 0), // #804
    InstInfo::new(213, 150, Encoding::VexRvm_Lx as u8, 0x54, 102, 0), // #805
    InstInfo::new(214, 150, Encoding::VexRvm_Lx as u8, 0x54, 104, 0), // #806
    InstInfo::new(215, 151, Encoding::VexRm_Lx as u8, 0xB1, 89, 0), // #807
    InstInfo::new(215, 151, Encoding::VexRm_Lx as u8, 0xB1, 30, 0), // #808
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x65, 112, 0), // #809
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x65, 113, 0), // #810
    InstInfo::new(218, 146, Encoding::VexRvmi_Lx as u8, 0x0D, 75, 0), // #811
    InstInfo::new(218, 146, Encoding::VexRvmi_Lx as u8, 0x0C, 75, 0), // #812
    InstInfo::new(219, 146, Encoding::VexRvmr_Lx as u8, 0x4B, 75, 0), // #813
    InstInfo::new(219, 146, Encoding::VexRvmr_Lx as u8, 0x4A, 75, 0), // #814
    InstInfo::new(220, 146, Encoding::VexRm as u8, 0x1A, 114, 0), // #815
    InstInfo::new(221, 152, Encoding::VexRm_Lx as u8, 0x19, 115, 0), // #816
    InstInfo::new(222, 149, Encoding::VexRm_Lx as u8, 0x1A, 116, 0), // #817
    InstInfo::new(223, 152, Encoding::VexRm as u8, 0x1B, 117, 0), // #818
    InstInfo::new(222, 152, Encoding::VexRm_Lx as u8, 0x1A, 118, 0), // #819
    InstInfo::new(223, 149, Encoding::VexRm as u8, 0x1B, 119, 0), // #820
    InstInfo::new(220, 153, Encoding::VexRm as u8, 0x5A, 114, 0), // #821
    InstInfo::new(224, 152, Encoding::VexRm_Lx as u8, 0x59, 115, 0), // #822
    InstInfo::new(222, 149, Encoding::VexRm_Lx as u8, 0x5A, 116, 0), // #823
    InstInfo::new(223, 152, Encoding::VexRm as u8, 0x5B, 117, 0), // #824
    InstInfo::new(222, 152, Encoding::VexRm_Lx as u8, 0x5A, 118, 0), // #825
    InstInfo::new(223, 149, Encoding::VexRm as u8, 0x5B, 119, 0), // #826
    InstInfo::new(225, 154, Encoding::VexRm_Lx as u8, 0x19, 120, 0), // #827
    InstInfo::new(226, 154, Encoding::VexRm_Lx as u8, 0x18, 121, 0), // #828
    InstInfo::new(227, 144, Encoding::VexRvmi_Lx_KEvex as u8, 0xC2, 102, 0), // #829
    InstInfo::new(228, 145, Encoding::VexRvmi_Lx_KEvex as u8, 0xC2, 122, 0), // #830
    InstInfo::new(229, 144, Encoding::VexRvmi_Lx_KEvex as u8, 0xC2, 104, 0), // #831
    InstInfo::new(230, 144, Encoding::VexRvmi_KEvex as u8, 0xC2, 105, 0), // #832
    InstInfo::new(231, 145, Encoding::VexRvmi_KEvex as u8, 0xC2, 123, 0), // #833
    InstInfo::new(232, 144, Encoding::VexRvmi_KEvex as u8, 0xC2, 107, 0), // #834
    InstInfo::new(233, 155, Encoding::VexRm as u8, 0x2F, 124, 0), // #835
    InstInfo::new(234, 156, Encoding::VexRm as u8, 0x2F, 125, 0), // #836
    InstInfo::new(235, 155, Encoding::VexRm as u8, 0x2F, 126, 0), // #837
    InstInfo::new(236, 149, Encoding::VexMr_Lx as u8, 0x8A, 127, 0), // #838
    InstInfo::new(236, 149, Encoding::VexMr_Lx as u8, 0x8A, 128, 0), // #839
    InstInfo::new(237, 144, Encoding::VexRm_Lx as u8, 0xE6, 129, 0), // #840
    InstInfo::new(238, 145, Encoding::VexRm_Lx_Narrow as u8, 0x5B, 103, 0), // #841
    InstInfo::new(239, 144, Encoding::VexRm_Lx as u8, 0x5B, 104, 0), // #842
    InstInfo::new(217, 157, Encoding::VexRvm_Lx as u8, 0x72, 130, 0), // #843
    InstInfo::new(240, 151, Encoding::VexRm_Lx as u8, 0xB0, 89, 0), // #844
    InstInfo::new(240, 151, Encoding::VexRm_Lx as u8, 0xB0, 30, 0), // #845
    InstInfo::new(240, 151, Encoding::VexRm_Lx as u8, 0xB0, 85, 0), // #846
    InstInfo::new(240, 151, Encoding::VexRm_Lx as u8, 0xB0, 11, 0), // #847
    InstInfo::new(241, 158, Encoding::VexRm_Lx_Narrow as u8, 0x72, 131, 0), // #848
    InstInfo::new(242, 144, Encoding::VexRm_Lx_Narrow as u8, 0xE6, 132, 0), // #849
    InstInfo::new(243, 145, Encoding::VexRm_Lx as u8, 0x5A, 133, 0), // #850
    InstInfo::new(242, 144, Encoding::VexRm_Lx_Narrow as u8, 0x5A, 102, 0), // #851
    InstInfo::new(244, 152, Encoding::VexRm_Lx as u8, 0x7B, 134, 0), // #852
    InstInfo::new(245, 149, Encoding::VexRm_Lx_Narrow as u8, 0x79, 135, 0), // #853
    InstInfo::new(244, 152, Encoding::VexRm_Lx as u8, 0x79, 134, 0), // #854
    InstInfo::new(246, 145, Encoding::VexRm_Lx as u8, 0x5B, 136, 0), // #855
    InstInfo::new(247, 145, Encoding::VexRm_Lx as u8, 0x5A, 137, 0), // #856
    InstInfo::new(248, 159, Encoding::VexRm_Lx as u8, 0x13, 138, 0), // #857
    InstInfo::new(249, 145, Encoding::VexRm_Lx as u8, 0x13, 139, 0), // #858
    InstInfo::new(250, 145, Encoding::VexRm_Lx as u8, 0x7B, 140, 0), // #859
    InstInfo::new(246, 145, Encoding::VexRm_Lx as u8, 0x79, 141, 0), // #860
    InstInfo::new(250, 145, Encoding::VexRm_Lx as u8, 0x79, 140, 0), // #861
    InstInfo::new(251, 145, Encoding::VexRm_Lx as u8, 0x7D, 103, 0), // #862
    InstInfo::new(251, 145, Encoding::VexRm_Lx as u8, 0x7D, 142, 0), // #863
    InstInfo::new(239, 144, Encoding::VexRm_Lx as u8, 0x5B, 143, 0), // #864
    InstInfo::new(252, 144, Encoding::VexRm_Lx as u8, 0x5A, 144, 0), // #865
    InstInfo::new(253, 159, Encoding::VexMri_Lx as u8, 0x1D, 145, 0), // #866
    InstInfo::new(238, 145, Encoding::VexRm_Lx_Narrow as u8, 0x1D, 142, 0), // #867
    InstInfo::new(254, 152, Encoding::VexRm_Lx as u8, 0x7B, 146, 0), // #868
    InstInfo::new(255, 149, Encoding::VexRm_Lx as u8, 0x79, 147, 0), // #869
    InstInfo::new(254, 152, Encoding::VexRm_Lx as u8, 0x79, 146, 0), // #870
    InstInfo::new(244, 152, Encoding::VexRm_Lx as u8, 0xE6, 148, 0), // #871
    InstInfo::new(243, 145, Encoding::VexRm_Lx as u8, 0x5B, 149, 0), // #872
    InstInfo::new(245, 152, Encoding::VexRm_Lx_Narrow as u8, 0x5B, 135, 0), // #873
    InstInfo::new(256, 145, Encoding::VexRvm as u8, 0x5A, 150, 0), // #874
    InstInfo::new(257, 144, Encoding::VexRm_Wx as u8, 0x2D, 151, 0), // #875
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x5A, 105, 0), // #876
    InstInfo::new(258, 149, Encoding::VexRm_Wx as u8, 0x79, 152, 0), // #877
    InstInfo::new(259, 145, Encoding::VexRvm as u8, 0x5A, 106, 0), // #878
    InstInfo::new(260, 145, Encoding::VexRm_Wx as u8, 0x2D, 106, 0), // #879
    InstInfo::new(259, 145, Encoding::VexRvm as u8, 0x13, 153, 0), // #880
    InstInfo::new(260, 145, Encoding::VexRm_Wx as u8, 0x79, 106, 0), // #881
    InstInfo::new(261, 144, Encoding::VexRvm_Wx as u8, 0x2A, 154, 0), // #882
    InstInfo::new(262, 145, Encoding::VexRvm_Wx as u8, 0x2A, 155, 0), // #883
    InstInfo::new(261, 144, Encoding::VexRvm_Wx as u8, 0x2A, 156, 0), // #884
    InstInfo::new(263, 144, Encoding::VexRvm as u8, 0x5A, 107, 0), // #885
    InstInfo::new(264, 145, Encoding::VexRvm as u8, 0x1D, 157, 0), // #886
    InstInfo::new(265, 144, Encoding::VexRm_Wx as u8, 0x2D, 107, 0), // #887
    InstInfo::new(266, 149, Encoding::VexRm_Wx as u8, 0x79, 158, 0), // #888
    InstInfo::new(267, 144, Encoding::VexRm_Lx_Narrow as u8, 0xE6, 102, 0), // #889
    InstInfo::new(268, 149, Encoding::VexRm_Lx as u8, 0x7A, 134, 0), // #890
    InstInfo::new(269, 149, Encoding::VexRm_Lx_Narrow as u8, 0x78, 135, 0), // #891
    InstInfo::new(268, 152, Encoding::VexRm_Lx as u8, 0x78, 134, 0), // #892
    InstInfo::new(249, 145, Encoding::VexRm_Lx as u8, 0x5B, 159, 0), // #893
    InstInfo::new(247, 145, Encoding::VexRm_Lx as u8, 0x7A, 140, 0), // #894
    InstInfo::new(249, 145, Encoding::VexRm_Lx as u8, 0x78, 141, 0), // #895
    InstInfo::new(247, 145, Encoding::VexRm_Lx as u8, 0x78, 140, 0), // #896
    InstInfo::new(270, 145, Encoding::VexRm_Lx as u8, 0x7C, 103, 0), // #897
    InstInfo::new(270, 145, Encoding::VexRm_Lx as u8, 0x7C, 142, 0), // #898
    InstInfo::new(271, 144, Encoding::VexRm_Lx as u8, 0x5B, 160, 0), // #899
    InstInfo::new(272, 152, Encoding::VexRm_Lx as u8, 0x7A, 146, 0), // #900
    InstInfo::new(273, 149, Encoding::VexRm_Lx as u8, 0x78, 147, 0), // #901
    InstInfo::new(272, 152, Encoding::VexRm_Lx as u8, 0x78, 146, 0), // #902
    InstInfo::new(274, 144, Encoding::VexRm_Wx as u8, 0x2C, 151, 0), // #903
    InstInfo::new(275, 149, Encoding::VexRm_Wx as u8, 0x78, 152, 0), // #904
    InstInfo::new(276, 145, Encoding::VexRm_Wx as u8, 0x2C, 106, 0), // #905
    InstInfo::new(276, 145, Encoding::VexRm_Wx as u8, 0x78, 106, 0), // #906
    InstInfo::new(277, 144, Encoding::VexRm_Wx as u8, 0x2C, 107, 0), // #907
    InstInfo::new(278, 149, Encoding::VexRm_Wx as u8, 0x78, 158, 0), // #908
    InstInfo::new(254, 149, Encoding::VexRm_Lx as u8, 0x7A, 161, 0), // #909
    InstInfo::new(238, 145, Encoding::VexRm_Lx_Narrow as u8, 0x7A, 162, 0), // #910
    InstInfo::new(255, 149, Encoding::VexRm_Lx as u8, 0x7A, 163, 0), // #911
    InstInfo::new(244, 152, Encoding::VexRm_Lx as u8, 0x7A, 148, 0), // #912
    InstInfo::new(243, 145, Encoding::VexRm_Lx as u8, 0x7A, 164, 0), // #913
    InstInfo::new(245, 152, Encoding::VexRm_Lx_Narrow as u8, 0x7A, 165, 0), // #914
    InstInfo::new(279, 149, Encoding::VexRvm_Wx as u8, 0x7B, 166, 0), // #915
    InstInfo::new(262, 145, Encoding::VexRvm_Wx as u8, 0x7B, 155, 0), // #916
    InstInfo::new(279, 149, Encoding::VexRvm_Wx as u8, 0x7B, 167, 0), // #917
    InstInfo::new(251, 145, Encoding::VexRm_Lx as u8, 0x7D, 162, 0), // #918
    InstInfo::new(251, 145, Encoding::VexRm_Lx as u8, 0x7D, 168, 0), // #919
    InstInfo::new(280, 160, Encoding::VexRvmi_Lx as u8, 0x42, 110, 0), // #920
    InstInfo::new(199, 144, Encoding::VexRvm_Lx as u8, 0x5E, 102, 0), // #921
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x5E, 103, 0), // #922
    InstInfo::new(201, 144, Encoding::VexRvm_Lx as u8, 0x5E, 104, 0), // #923
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x5E, 105, 0), // #924
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x5E, 106, 0), // #925
    InstInfo::new(204, 144, Encoding::VexRvm as u8, 0x5E, 107, 0), // #926
    InstInfo::new(217, 157, Encoding::VexRvm_Lx as u8, 0x52, 169, 0), // #927
    InstInfo::new(281, 146, Encoding::VexRvmi_Lx as u8, 0x41, 75, 0), // #928
    InstInfo::new(218, 146, Encoding::VexRvmi_Lx as u8, 0x40, 75, 0), // #929
    InstInfo::new(111, 11, Encoding::X86M_NoSize as u8, 0x00, 98, 0), // #930
    InstInfo::new(111, 11, Encoding::X86M_NoSize as u8, 0x00, 79, 0), // #931
    InstInfo::new(282, 149, Encoding::VexRm_Lx as u8, 0x88, 127, 0), // #932
    InstInfo::new(282, 149, Encoding::VexRm_Lx as u8, 0x88, 128, 0), // #933
    InstInfo::new(283, 146, Encoding::VexMri as u8, 0x19, 170, 0), // #934
    InstInfo::new(284, 149, Encoding::VexMri_Lx as u8, 0x19, 171, 0), // #935
    InstInfo::new(285, 152, Encoding::VexMri as u8, 0x1B, 172, 0), // #936
    InstInfo::new(284, 152, Encoding::VexMri_Lx as u8, 0x19, 173, 0), // #937
    InstInfo::new(285, 149, Encoding::VexMri as u8, 0x1B, 174, 0), // #938
    InstInfo::new(283, 153, Encoding::VexMri as u8, 0x39, 170, 0), // #939
    InstInfo::new(284, 149, Encoding::VexMri_Lx as u8, 0x39, 171, 0), // #940
    InstInfo::new(285, 152, Encoding::VexMri as u8, 0x3B, 172, 0), // #941
    InstInfo::new(284, 152, Encoding::VexMri_Lx as u8, 0x39, 173, 0), // #942
    InstInfo::new(285, 149, Encoding::VexMri as u8, 0x3B, 174, 0), // #943
    InstInfo::new(286, 144, Encoding::VexMri as u8, 0x17, 175, 0), // #944
    InstInfo::new(287, 145, Encoding::VexRvm_Lx as u8, 0x56, 176, 0), // #945
    InstInfo::new(264, 145, Encoding::VexRvm as u8, 0x57, 177, 0), // #946
    InstInfo::new(287, 145, Encoding::VexRvm_Lx as u8, 0xD6, 176, 0), // #947
    InstInfo::new(264, 145, Encoding::VexRvm as u8, 0xD7, 177, 0), // #948
    InstInfo::new(288, 149, Encoding::VexRvmi_Lx as u8, 0x54, 111, 0), // #949
    InstInfo::new(289, 149, Encoding::VexRvmi_Lx as u8, 0x54, 110, 0), // #950
    InstInfo::new(290, 149, Encoding::VexRvmi as u8, 0x55, 178, 0), // #951
    InstInfo::new(291, 149, Encoding::VexRvmi as u8, 0x55, 179, 0), // #952
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x98, 180, 0), // #953
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x98, 181, 0), // #954
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x98, 109, 0), // #955
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0x99, 182, 0), // #956
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x99, 183, 0), // #957
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0x99, 121, 0), // #958
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xA8, 180, 0), // #959
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xA8, 181, 0), // #960
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xA8, 109, 0), // #961
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xA9, 182, 0), // #962
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xA9, 183, 0), // #963
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xA9, 121, 0), // #964
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xB8, 180, 0), // #965
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xB8, 181, 0), // #966
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xB8, 109, 0), // #967
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xB9, 182, 0), // #968
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xB9, 183, 0), // #969
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xB9, 121, 0), // #970
    InstInfo::new(287, 145, Encoding::VexRvm_Lx as u8, 0x56, 184, 0), // #971
    InstInfo::new(264, 145, Encoding::VexRvm as u8, 0x57, 185, 0), // #972
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x69, 75, 0), // #973
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x68, 75, 0), // #974
    InstInfo::new(293, 162, Encoding::Fma4 as u8, 0x6B, 75, 0), // #975
    InstInfo::new(294, 162, Encoding::Fma4 as u8, 0x6A, 75, 0), // #976
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x96, 180, 0), // #977
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x96, 181, 0), // #978
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x96, 109, 0), // #979
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xA6, 180, 0), // #980
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xA6, 181, 0), // #981
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xA6, 109, 0), // #982
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xB6, 180, 0), // #983
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xB6, 181, 0), // #984
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xB6, 109, 0), // #985
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x5D, 75, 0), // #986
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x5C, 75, 0), // #987
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x9A, 180, 0), // #988
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x9A, 181, 0), // #989
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x9A, 109, 0), // #990
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0x9B, 182, 0), // #991
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x9B, 183, 0), // #992
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0x9B, 121, 0), // #993
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xAA, 180, 0), // #994
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xAA, 181, 0), // #995
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xAA, 109, 0), // #996
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xAB, 182, 0), // #997
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xAB, 183, 0), // #998
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xAB, 121, 0), // #999
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xBA, 180, 0), // #1000
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xBA, 181, 0), // #1001
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xBA, 109, 0), // #1002
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xBB, 182, 0), // #1003
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xBB, 183, 0), // #1004
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xBB, 121, 0), // #1005
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x97, 180, 0), // #1006
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x97, 181, 0), // #1007
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x97, 109, 0), // #1008
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xA7, 180, 0), // #1009
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xA7, 181, 0), // #1010
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xA7, 109, 0), // #1011
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xB7, 180, 0), // #1012
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xB7, 181, 0), // #1013
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xB7, 109, 0), // #1014
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x5F, 75, 0), // #1015
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x5E, 75, 0), // #1016
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x6D, 75, 0), // #1017
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x6C, 75, 0), // #1018
    InstInfo::new(293, 162, Encoding::Fma4 as u8, 0x6F, 75, 0), // #1019
    InstInfo::new(294, 162, Encoding::Fma4 as u8, 0x6E, 75, 0), // #1020
    InstInfo::new(287, 145, Encoding::VexRvm_Lx as u8, 0xD6, 184, 0), // #1021
    InstInfo::new(264, 145, Encoding::VexRvm as u8, 0xD7, 185, 0), // #1022
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x9C, 180, 0), // #1023
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x9C, 181, 0), // #1024
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x9C, 109, 0), // #1025
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0x9D, 182, 0), // #1026
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x9D, 183, 0), // #1027
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0x9D, 121, 0), // #1028
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xAC, 180, 0), // #1029
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xAC, 181, 0), // #1030
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xAC, 109, 0), // #1031
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xAD, 182, 0), // #1032
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xAD, 183, 0), // #1033
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xAD, 121, 0), // #1034
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xBC, 180, 0), // #1035
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xBC, 181, 0), // #1036
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xBC, 109, 0), // #1037
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xBD, 182, 0), // #1038
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xBD, 183, 0), // #1039
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xBD, 121, 0), // #1040
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x79, 75, 0), // #1041
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x78, 75, 0), // #1042
    InstInfo::new(293, 162, Encoding::Fma4 as u8, 0x7B, 75, 0), // #1043
    InstInfo::new(294, 162, Encoding::Fma4 as u8, 0x7A, 75, 0), // #1044
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0x9E, 180, 0), // #1045
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x9E, 181, 0), // #1046
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0x9E, 109, 0), // #1047
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0x9F, 182, 0), // #1048
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x9F, 183, 0), // #1049
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0x9F, 121, 0), // #1050
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xAE, 180, 0), // #1051
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xAE, 181, 0), // #1052
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xAE, 109, 0), // #1053
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xAF, 182, 0), // #1054
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xAF, 183, 0), // #1055
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xAF, 121, 0), // #1056
    InstInfo::new(199, 161, Encoding::VexRvm_Lx as u8, 0xBE, 180, 0), // #1057
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0xBE, 181, 0), // #1058
    InstInfo::new(201, 161, Encoding::VexRvm_Lx as u8, 0xBE, 109, 0), // #1059
    InstInfo::new(202, 161, Encoding::VexRvm as u8, 0xBF, 182, 0), // #1060
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0xBF, 183, 0), // #1061
    InstInfo::new(204, 161, Encoding::VexRvm as u8, 0xBF, 121, 0), // #1062
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x7D, 75, 0), // #1063
    InstInfo::new(292, 162, Encoding::Fma4_Lx as u8, 0x7C, 75, 0), // #1064
    InstInfo::new(293, 162, Encoding::Fma4 as u8, 0x7F, 75, 0), // #1065
    InstInfo::new(294, 162, Encoding::Fma4 as u8, 0x7E, 75, 0), // #1066
    InstInfo::new(295, 152, Encoding::VexRmi_Lx as u8, 0x66, 111, 0), // #1067
    InstInfo::new(296, 145, Encoding::VexRmi_Lx as u8, 0x66, 122, 0), // #1068
    InstInfo::new(297, 152, Encoding::VexRmi_Lx as u8, 0x66, 110, 0), // #1069
    InstInfo::new(298, 152, Encoding::VexRmi as u8, 0x67, 178, 0), // #1070
    InstInfo::new(299, 145, Encoding::VexRmi as u8, 0x67, 186, 0), // #1071
    InstInfo::new(300, 152, Encoding::VexRmi as u8, 0x67, 179, 0), // #1072
    InstInfo::new(301, 163, Encoding::VexRm_Lx as u8, 0x81, 81, 0), // #1073
    InstInfo::new(301, 163, Encoding::VexRm_Lx as u8, 0x80, 81, 0), // #1074
    InstInfo::new(302, 163, Encoding::VexRm as u8, 0x83, 81, 0), // #1075
    InstInfo::new(303, 163, Encoding::VexRm as u8, 0x82, 81, 0), // #1076
    InstInfo::new(304, 164, Encoding::VexRmvRm_VM as u8, 0x92, 187, 80), // #1077
    InstInfo::new(305, 164, Encoding::VexRmvRm_VM as u8, 0x92, 30, 81), // #1078
    InstInfo::new(306, 164, Encoding::VexRmvRm_VM as u8, 0x93, 187, 82), // #1079
    InstInfo::new(307, 164, Encoding::VexRmvRm_VM as u8, 0x93, 30, 83), // #1080
    InstInfo::new(268, 149, Encoding::VexRm_Lx as u8, 0x42, 112, 0), // #1081
    InstInfo::new(270, 145, Encoding::VexRm_Lx as u8, 0x42, 181, 0), // #1082
    InstInfo::new(273, 149, Encoding::VexRm_Lx as u8, 0x42, 113, 0), // #1083
    InstInfo::new(308, 149, Encoding::VexRvm as u8, 0x43, 127, 0), // #1084
    InstInfo::new(259, 145, Encoding::VexRvm as u8, 0x43, 183, 0), // #1085
    InstInfo::new(309, 149, Encoding::VexRvm as u8, 0x43, 128, 0), // #1086
    InstInfo::new(310, 149, Encoding::VexRmi_Lx as u8, 0x26, 111, 0), // #1087
    InstInfo::new(311, 145, Encoding::VexRmi_Lx as u8, 0x26, 122, 0), // #1088
    InstInfo::new(312, 149, Encoding::VexRmi_Lx as u8, 0x26, 110, 0), // #1089
    InstInfo::new(290, 149, Encoding::VexRvmi as u8, 0x27, 178, 0), // #1090
    InstInfo::new(313, 145, Encoding::VexRvmi as u8, 0x27, 186, 0), // #1091
    InstInfo::new(291, 149, Encoding::VexRvmi as u8, 0x27, 179, 0), // #1092
    InstInfo::new(314, 165, Encoding::VexRvmi_Lx as u8, 0xCF, 188, 0), // #1093
    InstInfo::new(314, 165, Encoding::VexRvmi_Lx as u8, 0xCE, 188, 0), // #1094
    InstInfo::new(315, 165, Encoding::VexRvm_Lx as u8, 0xCF, 109, 0), // #1095
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0x7C, 71, 0), // #1096
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0x7C, 108, 0), // #1097
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0x7D, 71, 0), // #1098
    InstInfo::new(205, 146, Encoding::VexRvm_Lx as u8, 0x7D, 108, 0), // #1099
    InstInfo::new(316, 146, Encoding::VexRvmi as u8, 0x18, 170, 0), // #1100
    InstInfo::new(317, 149, Encoding::VexRvmi_Lx as u8, 0x18, 171, 0), // #1101
    InstInfo::new(318, 152, Encoding::VexRvmi as u8, 0x1A, 172, 0), // #1102
    InstInfo::new(317, 152, Encoding::VexRvmi_Lx as u8, 0x18, 173, 0), // #1103
    InstInfo::new(318, 149, Encoding::VexRvmi as u8, 0x1A, 174, 0), // #1104
    InstInfo::new(316, 153, Encoding::VexRvmi as u8, 0x38, 170, 0), // #1105
    InstInfo::new(317, 149, Encoding::VexRvmi_Lx as u8, 0x38, 171, 0), // #1106
    InstInfo::new(318, 152, Encoding::VexRvmi as u8, 0x3A, 172, 0), // #1107
    InstInfo::new(317, 152, Encoding::VexRvmi_Lx as u8, 0x38, 173, 0), // #1108
    InstInfo::new(318, 149, Encoding::VexRvmi as u8, 0x3A, 174, 0), // #1109
    InstInfo::new(319, 144, Encoding::VexRvmi as u8, 0x21, 175, 0), // #1110
    InstInfo::new(240, 146, Encoding::VexRm_Lx as u8, 0xF0, 108, 0), // #1111
    InstInfo::new(320, 146, Encoding::VexM as u8, 0xAE, 189, 0), // #1112
    InstInfo::new(321, 146, Encoding::VexRm_ZDI as u8, 0xF7, 71, 0), // #1113
    InstInfo::new(322, 146, Encoding::VexRvmMvr_Lx as u8, 0x2D, 30, 84), // #1114
    InstInfo::new(322, 146, Encoding::VexRvmMvr_Lx as u8, 0x2C, 30, 85), // #1115
    InstInfo::new(323, 144, Encoding::VexRvm_Lx as u8, 0x5F, 102, 0), // #1116
    InstInfo::new(324, 145, Encoding::VexRvm_Lx as u8, 0x5F, 103, 0), // #1117
    InstInfo::new(325, 144, Encoding::VexRvm_Lx as u8, 0x5F, 104, 0), // #1118
    InstInfo::new(326, 144, Encoding::VexRvm as u8, 0x5F, 105, 0), // #1119
    InstInfo::new(259, 145, Encoding::VexRvm as u8, 0x5F, 106, 0), // #1120
    InstInfo::new(263, 144, Encoding::VexRvm as u8, 0x5F, 107, 0), // #1121
    InstInfo::new(31, 67, Encoding::X86Op as u8, 0xC1, 23, 0), // #1122
    InstInfo::new(33, 67, Encoding::X86M_Only as u8, 0xC7, 28, 0), // #1123
    InstInfo::new(31, 67, Encoding::X86Op as u8, 0xD4, 23, 0), // #1124
    InstInfo::new(31, 166, Encoding::X86Op as u8, 0xD9, 93, 0), // #1125
    InstInfo::new(323, 144, Encoding::VexRvm_Lx as u8, 0x5D, 102, 0), // #1126
    InstInfo::new(324, 145, Encoding::VexRvm_Lx as u8, 0x5D, 103, 0), // #1127
    InstInfo::new(325, 144, Encoding::VexRvm_Lx as u8, 0x5D, 104, 0), // #1128
    InstInfo::new(326, 144, Encoding::VexRvm as u8, 0x5D, 105, 0), // #1129
    InstInfo::new(259, 145, Encoding::VexRvm as u8, 0x5D, 106, 0), // #1130
    InstInfo::new(263, 144, Encoding::VexRvm as u8, 0x5D, 107, 0), // #1131
    InstInfo::new(31, 67, Encoding::X86Op as u8, 0xC2, 23, 0), // #1132
    InstInfo::new(327, 23, Encoding::X86Op_xAX as u8, 0xDA, 23, 0), // #1133
    InstInfo::new(31, 23, Encoding::X86Op as u8, 0xD9, 23, 0), // #1134
    InstInfo::new(328, 167, Encoding::VexRmMr_Lx as u8, 0x28, 102, 86), // #1135
    InstInfo::new(328, 167, Encoding::VexRmMr_Lx as u8, 0x28, 104, 87), // #1136
    InstInfo::new(329, 144, Encoding::VexMovdMovq as u8, 0x6E, 190, 88), // #1137
    InstInfo::new(330, 144, Encoding::VexRm_Lx as u8, 0x12, 191, 0), // #1138
    InstInfo::new(331, 168, Encoding::VexRmMr_Lx as u8, 0x6F, 71, 89), // #1139
    InstInfo::new(332, 169, Encoding::VexRmMr_Lx as u8, 0x6F, 192, 90), // #1140
    InstInfo::new(332, 169, Encoding::VexRmMr_Lx as u8, 0x6F, 134, 91), // #1141
    InstInfo::new(331, 168, Encoding::VexRmMr_Lx as u8, 0x6F, 193, 92), // #1142
    InstInfo::new(332, 170, Encoding::VexRmMr_Lx as u8, 0x6F, 165, 93), // #1143
    InstInfo::new(332, 169, Encoding::VexRmMr_Lx as u8, 0x6F, 194, 94), // #1144
    InstInfo::new(332, 169, Encoding::VexRmMr_Lx as u8, 0x6F, 148, 95), // #1145
    InstInfo::new(332, 170, Encoding::VexRmMr_Lx as u8, 0x6F, 163, 96), // #1146
    InstInfo::new(333, 144, Encoding::VexRvm as u8, 0x12, 74, 0), // #1147
    InstInfo::new(334, 144, Encoding::VexRvmMr as u8, 0x16, 124, 97), // #1148
    InstInfo::new(334, 144, Encoding::VexRvmMr as u8, 0x16, 195, 98), // #1149
    InstInfo::new(333, 144, Encoding::VexRvm as u8, 0x16, 74, 0), // #1150
    InstInfo::new(334, 144, Encoding::VexRvmMr as u8, 0x12, 124, 99), // #1151
    InstInfo::new(334, 144, Encoding::VexRvmMr as u8, 0x12, 195, 100), // #1152
    InstInfo::new(335, 146, Encoding::VexRm_Lx as u8, 0x50, 71, 0), // #1153
    InstInfo::new(335, 146, Encoding::VexRm_Lx as u8, 0x50, 74, 0), // #1154
    InstInfo::new(336, 144, Encoding::VexMr_Lx as u8, 0xE7, 143, 0), // #1155
    InstInfo::new(337, 154, Encoding::VexRm_Lx as u8, 0x2A, 109, 0), // #1156
    InstInfo::new(336, 144, Encoding::VexMr_Lx as u8, 0x2B, 102, 0), // #1157
    InstInfo::new(336, 144, Encoding::VexMr_Lx as u8, 0x2B, 104, 0), // #1158
    InstInfo::new(338, 167, Encoding::VexMovdMovq as u8, 0x6E, 124, 101), // #1159
    InstInfo::new(339, 167, Encoding::VexMovssMovsd as u8, 0x10, 105, 102), // #1160
    InstInfo::new(340, 145, Encoding::VexMovssMovsd as u8, 0x10, 106, 103), // #1161
    InstInfo::new(341, 144, Encoding::VexRm_Lx as u8, 0x16, 160, 0), // #1162
    InstInfo::new(341, 144, Encoding::VexRm_Lx as u8, 0x12, 160, 0), // #1163
    InstInfo::new(342, 167, Encoding::VexMovssMovsd as u8, 0x10, 107, 104), // #1164
    InstInfo::new(328, 167, Encoding::VexRmMr_Lx as u8, 0x10, 102, 105), // #1165
    InstInfo::new(328, 167, Encoding::VexRmMr_Lx as u8, 0x10, 104, 106), // #1166
    InstInfo::new(343, 145, Encoding::VexMovdMovq as u8, 0x6E, 196, 107), // #1167
    InstInfo::new(218, 171, Encoding::VexRvmi_Lx as u8, 0x42, 75, 0), // #1168
    InstInfo::new(33, 67, Encoding::X86M_Only as u8, 0xC7, 82, 0), // #1169
    InstInfo::new(33, 67, Encoding::X86M_Only as u8, 0xC7, 24, 0), // #1170
    InstInfo::new(344, 67, Encoding::X86Mr_NoSize as u8, 0x78, 5, 0), // #1171
    InstInfo::new(31, 67, Encoding::X86Op as u8, 0xC3, 23, 0), // #1172
    InstInfo::new(327, 23, Encoding::X86Op_xAX as u8, 0xD8, 23, 0), // #1173
    InstInfo::new(327, 23, Encoding::X86Op_xAX as u8, 0xDB, 23, 0), // #1174
    InstInfo::new(199, 144, Encoding::VexRvm_Lx as u8, 0x59, 102, 0), // #1175
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x59, 103, 0), // #1176
    InstInfo::new(201, 144, Encoding::VexRvm_Lx as u8, 0x59, 104, 0), // #1177
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x59, 105, 0), // #1178
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x59, 106, 0), // #1179
    InstInfo::new(204, 144, Encoding::VexRvm as u8, 0x59, 107, 0), // #1180
    InstInfo::new(345, 67, Encoding::X86Rm_NoSize as u8, 0x79, 5, 0), // #1181
    InstInfo::new(31, 67, Encoding::X86Op as u8, 0xC4, 23, 0), // #1182
    InstInfo::new(33, 67, Encoding::X86M_Only as u8, 0xC7, 26, 0), // #1183
    InstInfo::new(213, 150, Encoding::VexRvm_Lx as u8, 0x56, 102, 0), // #1184
    InstInfo::new(214, 150, Encoding::VexRvm_Lx as u8, 0x56, 104, 0), // #1185
    InstInfo::new(346, 172, Encoding::VexRvm_Lx_2xK as u8, 0x68, 130, 0), // #1186
    InstInfo::new(347, 172, Encoding::VexRvm_Lx_2xK as u8, 0x68, 197, 0), // #1187
    InstInfo::new(341, 173, Encoding::VexRm_Lx as u8, 0x1C, 109, 0), // #1188
    InstInfo::new(348, 154, Encoding::VexRm_Lx as u8, 0x1E, 109, 0), // #1189
    InstInfo::new(349, 149, Encoding::VexRm_Lx as u8, 0x1F, 112, 0), // #1190
    InstInfo::new(341, 173, Encoding::VexRm_Lx as u8, 0x1D, 109, 0), // #1191
    InstInfo::new(212, 173, Encoding::VexRvm_Lx as u8, 0x6B, 143, 0), // #1192
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x63, 143, 0), // #1193
    InstInfo::new(212, 173, Encoding::VexRvm_Lx as u8, 0x2B, 109, 0), // #1194
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x67, 143, 0), // #1195
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xFC, 143, 0), // #1196
    InstInfo::new(212, 154, Encoding::VexRvm_Lx as u8, 0xFE, 143, 0), // #1197
    InstInfo::new(211, 154, Encoding::VexRvm_Lx as u8, 0xD4, 102, 0), // #1198
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xEC, 143, 0), // #1199
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xED, 143, 0), // #1200
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xDC, 143, 0), // #1201
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xDD, 143, 0), // #1202
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xFD, 143, 0), // #1203
    InstInfo::new(314, 173, Encoding::VexRvmi_Lx as u8, 0x0F, 198, 0), // #1204
    InstInfo::new(350, 171, Encoding::VexRvm_Lx as u8, 0xDB, 71, 0), // #1205
    InstInfo::new(351, 149, Encoding::VexRvm_Lx as u8, 0xDB, 192, 0), // #1206
    InstInfo::new(352, 171, Encoding::VexRvm_Lx as u8, 0xDF, 71, 0), // #1207
    InstInfo::new(353, 149, Encoding::VexRvm_Lx as u8, 0xDF, 192, 0), // #1208
    InstInfo::new(354, 149, Encoding::VexRvm_Lx as u8, 0xDF, 134, 0), // #1209
    InstInfo::new(355, 149, Encoding::VexRvm_Lx as u8, 0xDB, 134, 0), // #1210
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xE0, 143, 0), // #1211
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xE3, 143, 0), // #1212
    InstInfo::new(218, 153, Encoding::VexRvmi_Lx as u8, 0x02, 75, 0), // #1213
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x66, 113, 0), // #1214
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x64, 113, 0), // #1215
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x64, 112, 0), // #1216
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x66, 112, 0), // #1217
    InstInfo::new(219, 171, Encoding::VexRvmr_Lx as u8, 0x4C, 75, 0), // #1218
    InstInfo::new(218, 171, Encoding::VexRvmi_Lx as u8, 0x0E, 75, 0), // #1219
    InstInfo::new(357, 174, Encoding::VexRm_Lx_Bcst as u8, 0x78, 30, 108), // #1220
    InstInfo::new(358, 164, Encoding::VexRm_Lx_Bcst as u8, 0x58, 121, 109), // #1221
    InstInfo::new(359, 175, Encoding::VexRm_Lx as u8, 0x2A, 199, 0), // #1222
    InstInfo::new(359, 175, Encoding::VexRm_Lx as u8, 0x3A, 200, 0), // #1223
    InstInfo::new(360, 164, Encoding::VexRm_Lx_Bcst as u8, 0x59, 120, 110), // #1224
    InstInfo::new(361, 174, Encoding::VexRm_Lx_Bcst as u8, 0x79, 201, 111), // #1225
    InstInfo::new(362, 176, Encoding::VexRvmi_Lx as u8, 0x44, 198, 0), // #1226
    InstInfo::new(363, 163, Encoding::VexRvrmRvmr_Lx as u8, 0xA2, 202, 0), // #1227
    InstInfo::new(364, 160, Encoding::VexRvmi_Lx as u8, 0x3F, 110, 0), // #1228
    InstInfo::new(365, 149, Encoding::VexRvmi_Lx as u8, 0x1F, 110, 0), // #1229
    InstInfo::new(366, 173, Encoding::VexRvm_Lx_KEvex as u8, 0x74, 143, 0), // #1230
    InstInfo::new(367, 154, Encoding::VexRvm_Lx_KEvex as u8, 0x76, 143, 0), // #1231
    InstInfo::new(368, 154, Encoding::VexRvm_Lx_KEvex as u8, 0x29, 203, 0), // #1232
    InstInfo::new(366, 173, Encoding::VexRvm_Lx_KEvex as u8, 0x75, 143, 0), // #1233
    InstInfo::new(369, 177, Encoding::VexRmi as u8, 0x61, 75, 0), // #1234
    InstInfo::new(370, 177, Encoding::VexRmi as u8, 0x60, 75, 0), // #1235
    InstInfo::new(366, 173, Encoding::VexRvm_Lx_KEvex as u8, 0x64, 143, 0), // #1236
    InstInfo::new(367, 154, Encoding::VexRvm_Lx_KEvex as u8, 0x66, 143, 0), // #1237
    InstInfo::new(368, 154, Encoding::VexRvm_Lx_KEvex as u8, 0x37, 203, 0), // #1238
    InstInfo::new(366, 173, Encoding::VexRvm_Lx_KEvex as u8, 0x65, 143, 0), // #1239
    InstInfo::new(371, 177, Encoding::VexRmi as u8, 0x63, 75, 0), // #1240
    InstInfo::new(372, 177, Encoding::VexRmi as u8, 0x62, 75, 0), // #1241
    InstInfo::new(373, 149, Encoding::VexRvmi_Lx as u8, 0x1F, 111, 0), // #1242
    InstInfo::new(364, 160, Encoding::VexRvmi_Lx as u8, 0x3E, 110, 0), // #1243
    InstInfo::new(365, 149, Encoding::VexRvmi_Lx as u8, 0x1E, 110, 0), // #1244
    InstInfo::new(373, 149, Encoding::VexRvmi_Lx as u8, 0x1E, 111, 0), // #1245
    InstInfo::new(364, 160, Encoding::VexRvmi_Lx as u8, 0x3E, 111, 0), // #1246
    InstInfo::new(364, 160, Encoding::VexRvmi_Lx as u8, 0x3F, 111, 0), // #1247
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xCC, 202, 0), // #1248
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xCE, 202, 0), // #1249
    InstInfo::new(236, 178, Encoding::VexMr_Lx as u8, 0x63, 204, 0), // #1250
    InstInfo::new(236, 149, Encoding::VexMr_Lx as u8, 0x8B, 128, 0), // #1251
    InstInfo::new(236, 149, Encoding::VexMr_Lx as u8, 0x8B, 127, 0), // #1252
    InstInfo::new(236, 178, Encoding::VexMr_Lx as u8, 0x63, 205, 0), // #1253
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xCF, 202, 0), // #1254
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xEC, 202, 0), // #1255
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xEE, 202, 0), // #1256
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xEF, 202, 0), // #1257
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xED, 202, 0), // #1258
    InstInfo::new(281, 163, Encoding::VexRvmi as u8, 0xCD, 202, 0), // #1259
    InstInfo::new(374, 175, Encoding::VexRm_Lx as u8, 0xC4, 113, 0), // #1260
    InstInfo::new(374, 175, Encoding::VexRm_Lx as u8, 0xC4, 112, 0), // #1261
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x50, 85, 0), // #1262
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x51, 85, 0), // #1263
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x50, 89, 0), // #1264
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x51, 89, 0), // #1265
    InstInfo::new(375, 180, Encoding::VexRvm_Lx as u8, 0x50, 109, 0), // #1266
    InstInfo::new(375, 180, Encoding::VexRvm_Lx as u8, 0x51, 109, 0), // #1267
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x50, 11, 0), // #1268
    InstInfo::new(205, 179, Encoding::VexRvm_Lx as u8, 0x51, 11, 0), // #1269
    InstInfo::new(375, 180, Encoding::VexRvm_Lx as u8, 0x52, 109, 0), // #1270
    InstInfo::new(375, 180, Encoding::VexRvm_Lx as u8, 0x53, 109, 0), // #1271
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD2, 89, 0), // #1272
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD3, 89, 0), // #1273
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD2, 30, 0), // #1274
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD3, 30, 0), // #1275
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD2, 11, 0), // #1276
    InstInfo::new(205, 181, Encoding::VexRvm_Lx as u8, 0xD3, 11, 0), // #1277
    InstInfo::new(376, 146, Encoding::VexRvmi as u8, 0x06, 170, 0), // #1278
    InstInfo::new(376, 153, Encoding::VexRvmi as u8, 0x46, 170, 0), // #1279
    InstInfo::new(356, 182, Encoding::VexRvm_Lx as u8, 0x8D, 113, 0), // #1280
    InstInfo::new(377, 164, Encoding::VexRvm_Lx as u8, 0x36, 109, 0), // #1281
    InstInfo::new(356, 182, Encoding::VexRvm_Lx as u8, 0x75, 113, 0), // #1282
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x76, 113, 0), // #1283
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x77, 112, 0), // #1284
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x77, 113, 0), // #1285
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x76, 112, 0), // #1286
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x75, 112, 0), // #1287
    InstInfo::new(378, 163, Encoding::VexRvrmiRvmri_Lx as u8, 0x49, 75, 0), // #1288
    InstInfo::new(378, 163, Encoding::VexRvrmiRvmri_Lx as u8, 0x48, 75, 0), // #1289
    InstInfo::new(379, 144, Encoding::VexRvmRmi_Lx as u8, 0x0D, 203, 112), // #1290
    InstInfo::new(380, 144, Encoding::VexRvmRmi_Lx as u8, 0x0C, 109, 113), // #1291
    InstInfo::new(381, 164, Encoding::VexRvmRmi_Lx as u8, 0x16, 206, 114), // #1292
    InstInfo::new(377, 164, Encoding::VexRvm_Lx as u8, 0x16, 109, 0), // #1293
    InstInfo::new(381, 164, Encoding::VexRvmRmi_Lx as u8, 0x36, 112, 115), // #1294
    InstInfo::new(356, 182, Encoding::VexRvm_Lx as u8, 0x7D, 113, 0), // #1295
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x7E, 113, 0), // #1296
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x7F, 112, 0), // #1297
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x7F, 113, 0), // #1298
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x7E, 112, 0), // #1299
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x7D, 112, 0), // #1300
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x8D, 112, 0), // #1301
    InstInfo::new(282, 178, Encoding::VexRm_Lx as u8, 0x62, 204, 0), // #1302
    InstInfo::new(282, 149, Encoding::VexRm_Lx as u8, 0x89, 128, 0), // #1303
    InstInfo::new(282, 149, Encoding::VexRm_Lx as u8, 0x89, 127, 0), // #1304
    InstInfo::new(282, 178, Encoding::VexRm_Lx as u8, 0x62, 205, 0), // #1305
    InstInfo::new(382, 183, Encoding::VexMri as u8, 0x14, 75, 0), // #1306
    InstInfo::new(286, 150, Encoding::VexMri as u8, 0x16, 175, 0), // #1307
    InstInfo::new(383, 150, Encoding::VexMri as u8, 0x16, 207, 0), // #1308
    InstInfo::new(384, 183, Encoding::VexMri_Vpextrw as u8, 0x15, 208, 0), // #1309
    InstInfo::new(305, 164, Encoding::VexRmvRm_VM as u8, 0x90, 30, 116), // #1310
    InstInfo::new(304, 164, Encoding::VexRmvRm_VM as u8, 0x90, 187, 117), // #1311
    InstInfo::new(307, 164, Encoding::VexRmvRm_VM as u8, 0x91, 30, 118), // #1312
    InstInfo::new(306, 164, Encoding::VexRmvRm_VM as u8, 0x91, 187, 119), // #1313
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xC2, 81, 0), // #1314
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xC3, 81, 0), // #1315
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xC1, 81, 0), // #1316
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x02, 30, 0), // #1317
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xCB, 81, 0), // #1318
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x03, 30, 0), // #1319
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xD2, 81, 0), // #1320
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xD3, 81, 0), // #1321
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xD1, 81, 0), // #1322
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xDB, 81, 0), // #1323
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xD6, 81, 0), // #1324
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xD7, 81, 0), // #1325
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x01, 30, 0), // #1326
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xC6, 81, 0), // #1327
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xC7, 81, 0), // #1328
    InstInfo::new(207, 146, Encoding::VexRm as u8, 0x41, 30, 0), // #1329
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xE1, 81, 0), // #1330
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x06, 30, 0), // #1331
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xE3, 81, 0), // #1332
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x07, 30, 0), // #1333
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x05, 30, 0), // #1334
    InstInfo::new(207, 163, Encoding::VexRm as u8, 0xE2, 81, 0), // #1335
    InstInfo::new(385, 183, Encoding::VexRvmi as u8, 0x20, 75, 0), // #1336
    InstInfo::new(386, 150, Encoding::VexRvmi as u8, 0x22, 175, 0), // #1337
    InstInfo::new(387, 150, Encoding::VexRvmi as u8, 0x22, 207, 0), // #1338
    InstInfo::new(388, 183, Encoding::VexRvmi as u8, 0xC4, 209, 0), // #1339
    InstInfo::new(374, 175, Encoding::VexRm_Lx as u8, 0x44, 113, 0), // #1340
    InstInfo::new(349, 175, Encoding::VexRm_Lx as u8, 0x44, 112, 0), // #1341
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x9E, 202, 0), // #1342
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x9F, 202, 0), // #1343
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x97, 202, 0), // #1344
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x8E, 202, 0), // #1345
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x8F, 202, 0), // #1346
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x87, 202, 0), // #1347
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x86, 202, 0), // #1348
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x85, 202, 0), // #1349
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x96, 202, 0), // #1350
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0x95, 202, 0), // #1351
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0xA6, 202, 0), // #1352
    InstInfo::new(389, 163, Encoding::VexRvmr as u8, 0xB6, 202, 0), // #1353
    InstInfo::new(390, 184, Encoding::VexRvm_Lx as u8, 0xB5, 180, 0), // #1354
    InstInfo::new(390, 184, Encoding::VexRvm_Lx as u8, 0xB4, 180, 0), // #1355
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x04, 109, 0), // #1356
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xF5, 143, 0), // #1357
    InstInfo::new(322, 153, Encoding::VexRvmMvr_Lx as u8, 0x8C, 30, 120), // #1358
    InstInfo::new(322, 153, Encoding::VexRvmMvr_Lx as u8, 0x8C, 187, 121), // #1359
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0x3C, 109, 0), // #1360
    InstInfo::new(214, 154, Encoding::VexRvm_Lx as u8, 0x3D, 109, 0), // #1361
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x3D, 112, 0), // #1362
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0xEE, 143, 0), // #1363
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0xDE, 143, 0), // #1364
    InstInfo::new(214, 154, Encoding::VexRvm_Lx as u8, 0x3F, 109, 0), // #1365
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x3F, 112, 0), // #1366
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0x3E, 109, 0), // #1367
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0x38, 109, 0), // #1368
    InstInfo::new(214, 154, Encoding::VexRvm_Lx as u8, 0x39, 109, 0), // #1369
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x39, 112, 0), // #1370
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0xEA, 143, 0), // #1371
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0xDA, 143, 0), // #1372
    InstInfo::new(214, 154, Encoding::VexRvm_Lx as u8, 0x3B, 109, 0), // #1373
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x3B, 112, 0), // #1374
    InstInfo::new(391, 173, Encoding::VexRvm_Lx as u8, 0x3A, 109, 0), // #1375
    InstInfo::new(392, 160, Encoding::VexRm_Lx as u8, 0x29, 200, 0), // #1376
    InstInfo::new(392, 152, Encoding::VexRm_Lx as u8, 0x39, 200, 0), // #1377
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x31, 210, 0), // #1378
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x33, 211, 0), // #1379
    InstInfo::new(359, 160, Encoding::VexRm_Lx as u8, 0x28, 200, 0), // #1380
    InstInfo::new(359, 152, Encoding::VexRm_Lx as u8, 0x38, 200, 0), // #1381
    InstInfo::new(359, 152, Encoding::VexRm_Lx as u8, 0x38, 199, 0), // #1382
    InstInfo::new(359, 160, Encoding::VexRm_Lx as u8, 0x28, 199, 0), // #1383
    InstInfo::new(335, 171, Encoding::VexRm_Lx as u8, 0xD7, 71, 0), // #1384
    InstInfo::new(392, 152, Encoding::VexRm_Lx as u8, 0x39, 199, 0), // #1385
    InstInfo::new(395, 149, Encoding::VexMr_Lx as u8, 0x32, 212, 0), // #1386
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x35, 211, 0), // #1387
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x34, 210, 0), // #1388
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x21, 210, 0), // #1389
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x23, 211, 0), // #1390
    InstInfo::new(395, 149, Encoding::VexMr_Lx as u8, 0x22, 212, 0), // #1391
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x25, 211, 0), // #1392
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x24, 210, 0), // #1393
    InstInfo::new(394, 160, Encoding::VexMr_Lx as u8, 0x20, 211, 0), // #1394
    InstInfo::new(396, 154, Encoding::VexRm_Lx as u8, 0x21, 213, 0), // #1395
    InstInfo::new(397, 154, Encoding::VexRm_Lx as u8, 0x22, 214, 0), // #1396
    InstInfo::new(398, 173, Encoding::VexRm_Lx as u8, 0x20, 138, 0), // #1397
    InstInfo::new(398, 154, Encoding::VexRm_Lx as u8, 0x25, 138, 0), // #1398
    InstInfo::new(398, 154, Encoding::VexRm_Lx as u8, 0x23, 138, 0), // #1399
    InstInfo::new(396, 154, Encoding::VexRm_Lx as u8, 0x24, 213, 0), // #1400
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x11, 210, 0), // #1401
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x13, 211, 0), // #1402
    InstInfo::new(395, 149, Encoding::VexMr_Lx as u8, 0x12, 212, 0), // #1403
    InstInfo::new(394, 149, Encoding::VexMr_Lx as u8, 0x15, 211, 0), // #1404
    InstInfo::new(393, 149, Encoding::VexMr_Lx as u8, 0x14, 210, 0), // #1405
    InstInfo::new(394, 160, Encoding::VexMr_Lx as u8, 0x10, 211, 0), // #1406
    InstInfo::new(392, 160, Encoding::VexRm_Lx as u8, 0x29, 199, 0), // #1407
    InstInfo::new(394, 160, Encoding::VexMr_Lx as u8, 0x30, 211, 0), // #1408
    InstInfo::new(396, 154, Encoding::VexRm_Lx as u8, 0x31, 213, 0), // #1409
    InstInfo::new(397, 154, Encoding::VexRm_Lx as u8, 0x32, 214, 0), // #1410
    InstInfo::new(398, 173, Encoding::VexRm_Lx as u8, 0x30, 138, 0), // #1411
    InstInfo::new(398, 154, Encoding::VexRm_Lx as u8, 0x35, 138, 0), // #1412
    InstInfo::new(398, 154, Encoding::VexRm_Lx as u8, 0x33, 138, 0), // #1413
    InstInfo::new(396, 154, Encoding::VexRm_Lx as u8, 0x34, 213, 0), // #1414
    InstInfo::new(211, 154, Encoding::VexRvm_Lx as u8, 0x28, 203, 0), // #1415
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x0B, 109, 0), // #1416
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xE4, 143, 0), // #1417
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xE5, 143, 0), // #1418
    InstInfo::new(212, 154, Encoding::VexRvm_Lx as u8, 0x40, 109, 0), // #1419
    InstInfo::new(216, 152, Encoding::VexRvm_Lx as u8, 0x40, 112, 0), // #1420
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0xD5, 143, 0), // #1421
    InstInfo::new(216, 182, Encoding::VexRvm_Lx as u8, 0x83, 112, 0), // #1422
    InstInfo::new(211, 154, Encoding::VexRvm_Lx as u8, 0xF4, 102, 0), // #1423
    InstInfo::new(282, 185, Encoding::VexRm_Lx as u8, 0x54, 113, 0), // #1424
    InstInfo::new(374, 186, Encoding::VexRm_Lx as u8, 0x55, 113, 0), // #1425
    InstInfo::new(349, 186, Encoding::VexRm_Lx as u8, 0x55, 112, 0), // #1426
    InstInfo::new(282, 185, Encoding::VexRm_Lx as u8, 0x54, 112, 0), // #1427
    InstInfo::new(350, 171, Encoding::VexRvm_Lx as u8, 0xEB, 71, 0), // #1428
    InstInfo::new(351, 149, Encoding::VexRvm_Lx as u8, 0xEB, 192, 0), // #1429
    InstInfo::new(355, 149, Encoding::VexRvm_Lx as u8, 0xEB, 134, 0), // #1430
    InstInfo::new(399, 163, Encoding::VexRvrmRvmr as u8, 0xA3, 202, 0), // #1431
    InstInfo::new(400, 149, Encoding::VexVmi_Lx as u8, 0x72, 215, 0), // #1432
    InstInfo::new(401, 149, Encoding::VexVmi_Lx as u8, 0x72, 216, 0), // #1433
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x15, 113, 0), // #1434
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x15, 112, 0), // #1435
    InstInfo::new(400, 149, Encoding::VexVmi_Lx as u8, 0x72, 192, 0), // #1436
    InstInfo::new(401, 149, Encoding::VexVmi_Lx as u8, 0x72, 134, 0), // #1437
    InstInfo::new(217, 149, Encoding::VexRvm_Lx as u8, 0x14, 113, 0), // #1438
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x14, 112, 0), // #1439
    InstInfo::new(402, 163, Encoding::VexRvmRmvRmi as u8, 0x90, 81, 122), // #1440
    InstInfo::new(402, 163, Encoding::VexRvmRmvRmi as u8, 0x92, 81, 123), // #1441
    InstInfo::new(402, 163, Encoding::VexRvmRmvRmi as u8, 0x93, 81, 124), // #1442
    InstInfo::new(402, 163, Encoding::VexRvmRmvRmi as u8, 0x91, 81, 125), // #1443
    InstInfo::new(206, 173, Encoding::VexRvm_Lx as u8, 0xF6, 143, 0), // #1444
    InstInfo::new(403, 149, Encoding::VexMr_VM as u8, 0xA0, 128, 0), // #1445
    InstInfo::new(404, 149, Encoding::VexMr_VM as u8, 0xA0, 127, 0), // #1446
    InstInfo::new(405, 149, Encoding::VexMr_VM as u8, 0xA1, 128, 0), // #1447
    InstInfo::new(406, 149, Encoding::VexMr_VM as u8, 0xA1, 127, 0), // #1448
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x98, 81, 0), // #1449
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x9A, 81, 0), // #1450
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x9B, 81, 0), // #1451
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x99, 81, 0), // #1452
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x94, 81, 0), // #1453
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x96, 81, 0), // #1454
    InstInfo::new(209, 178, Encoding::VexRvmi_Lx as u8, 0x71, 110, 0), // #1455
    InstInfo::new(210, 178, Encoding::VexRvmi_Lx as u8, 0x71, 111, 0), // #1456
    InstInfo::new(217, 178, Encoding::VexRvm_Lx as u8, 0x71, 113, 0), // #1457
    InstInfo::new(216, 178, Encoding::VexRvm_Lx as u8, 0x71, 112, 0), // #1458
    InstInfo::new(356, 178, Encoding::VexRvm_Lx as u8, 0x70, 112, 0), // #1459
    InstInfo::new(280, 178, Encoding::VexRvmi_Lx as u8, 0x70, 111, 0), // #1460
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x97, 81, 0), // #1461
    InstInfo::new(407, 163, Encoding::VexRvmRmv as u8, 0x95, 81, 0), // #1462
    InstInfo::new(209, 178, Encoding::VexRvmi_Lx as u8, 0x73, 110, 0), // #1463
    InstInfo::new(210, 178, Encoding::VexRvmi_Lx as u8, 0x73, 111, 0), // #1464
    InstInfo::new(217, 178, Encoding::VexRvm_Lx as u8, 0x73, 113, 0), // #1465
    InstInfo::new(216, 178, Encoding::VexRvm_Lx as u8, 0x73, 112, 0), // #1466
    InstInfo::new(356, 178, Encoding::VexRvm_Lx as u8, 0x72, 112, 0), // #1467
    InstInfo::new(280, 178, Encoding::VexRvmi_Lx as u8, 0x72, 111, 0), // #1468
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x00, 109, 0), // #1469
    InstInfo::new(408, 185, Encoding::VexRvm_Lx as u8, 0x8F, 113, 0), // #1470
    InstInfo::new(409, 154, Encoding::VexRmi_Lx as u8, 0x70, 143, 0), // #1471
    InstInfo::new(410, 173, Encoding::VexRmi_Lx as u8, 0x70, 160, 0), // #1472
    InstInfo::new(410, 173, Encoding::VexRmi_Lx as u8, 0x70, 217, 0), // #1473
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x08, 30, 0), // #1474
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x0A, 30, 0), // #1475
    InstInfo::new(205, 171, Encoding::VexRvm_Lx as u8, 0x09, 30, 0), // #1476
    InstInfo::new(411, 154, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xF2, 218, 126), // #1477
    InstInfo::new(412, 173, Encoding::VexVmi_Lx_MEvex as u8, 0x73, 219, 0), // #1478
    InstInfo::new(413, 154, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xF3, 220, 127), // #1479
    InstInfo::new(212, 164, Encoding::VexRvm_Lx as u8, 0x47, 109, 0), // #1480
    InstInfo::new(211, 164, Encoding::VexRvm_Lx as u8, 0x47, 180, 0), // #1481
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x12, 112, 0), // #1482
    InstInfo::new(414, 173, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xF1, 218, 128), // #1483
    InstInfo::new(411, 154, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xE2, 218, 129), // #1484
    InstInfo::new(415, 149, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xE2, 221, 130), // #1485
    InstInfo::new(212, 164, Encoding::VexRvm_Lx as u8, 0x46, 109, 0), // #1486
    InstInfo::new(216, 149, Encoding::VexRvm_Lx as u8, 0x46, 112, 0), // #1487
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x11, 112, 0), // #1488
    InstInfo::new(414, 173, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xE1, 218, 131), // #1489
    InstInfo::new(411, 154, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xD2, 218, 132), // #1490
    InstInfo::new(412, 173, Encoding::VexVmi_Lx_MEvex as u8, 0x73, 222, 0), // #1491
    InstInfo::new(413, 154, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xD3, 220, 133), // #1492
    InstInfo::new(212, 164, Encoding::VexRvm_Lx as u8, 0x45, 109, 0), // #1493
    InstInfo::new(211, 164, Encoding::VexRvm_Lx as u8, 0x45, 180, 0), // #1494
    InstInfo::new(356, 160, Encoding::VexRvm_Lx as u8, 0x10, 112, 0), // #1495
    InstInfo::new(414, 173, Encoding::VexRvmVmi_Lx_MEvex as u8, 0xD1, 218, 134), // #1496
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xF8, 143, 0), // #1497
    InstInfo::new(417, 154, Encoding::VexRvm_Lx as u8, 0xFA, 143, 0), // #1498
    InstInfo::new(418, 154, Encoding::VexRvm_Lx as u8, 0xFB, 102, 0), // #1499
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xE8, 143, 0), // #1500
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xE9, 143, 0), // #1501
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xD8, 143, 0), // #1502
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xD9, 143, 0), // #1503
    InstInfo::new(416, 173, Encoding::VexRvm_Lx as u8, 0xF9, 143, 0), // #1504
    InstInfo::new(209, 149, Encoding::VexRvmi_Lx as u8, 0x25, 110, 0), // #1505
    InstInfo::new(210, 149, Encoding::VexRvmi_Lx as u8, 0x25, 111, 0), // #1506
    InstInfo::new(301, 177, Encoding::VexRm_Lx as u8, 0x17, 30, 0), // #1507
    InstInfo::new(408, 160, Encoding::VexRvm_Lx as u8, 0x26, 113, 0), // #1508
    InstInfo::new(419, 149, Encoding::VexRvm_Lx as u8, 0x27, 113, 0), // #1509
    InstInfo::new(420, 149, Encoding::VexRvm_Lx as u8, 0x27, 112, 0), // #1510
    InstInfo::new(408, 160, Encoding::VexRvm_Lx as u8, 0x26, 112, 0), // #1511
    InstInfo::new(408, 160, Encoding::VexRvm_Lx as u8, 0x26, 169, 0), // #1512
    InstInfo::new(419, 149, Encoding::VexRvm_Lx as u8, 0x27, 169, 0), // #1513
    InstInfo::new(420, 149, Encoding::VexRvm_Lx as u8, 0x27, 223, 0), // #1514
    InstInfo::new(408, 160, Encoding::VexRvm_Lx as u8, 0x26, 223, 0), // #1515
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x68, 143, 0), // #1516
    InstInfo::new(212, 154, Encoding::VexRvm_Lx as u8, 0x6A, 143, 0), // #1517
    InstInfo::new(211, 154, Encoding::VexRvm_Lx as u8, 0x6D, 102, 0), // #1518
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x69, 143, 0), // #1519
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x60, 143, 0), // #1520
    InstInfo::new(212, 154, Encoding::VexRvm_Lx as u8, 0x62, 143, 0), // #1521
    InstInfo::new(211, 154, Encoding::VexRvm_Lx as u8, 0x6C, 102, 0), // #1522
    InstInfo::new(315, 173, Encoding::VexRvm_Lx as u8, 0x61, 143, 0), // #1523
    InstInfo::new(352, 171, Encoding::VexRvm_Lx as u8, 0xEF, 71, 0), // #1524
    InstInfo::new(353, 149, Encoding::VexRvm_Lx as u8, 0xEF, 192, 0), // #1525
    InstInfo::new(354, 149, Encoding::VexRvm_Lx as u8, 0xEF, 134, 0), // #1526
    InstInfo::new(288, 152, Encoding::VexRvmi_Lx as u8, 0x50, 111, 0), // #1527
    InstInfo::new(289, 152, Encoding::VexRvmi_Lx as u8, 0x50, 110, 0), // #1528
    InstInfo::new(290, 152, Encoding::VexRvmi as u8, 0x51, 178, 0), // #1529
    InstInfo::new(291, 152, Encoding::VexRvmi as u8, 0x51, 179, 0), // #1530
    InstInfo::new(349, 149, Encoding::VexRm_Lx as u8, 0x4C, 112, 0), // #1531
    InstInfo::new(374, 149, Encoding::VexRm_Lx as u8, 0x4C, 113, 0), // #1532
    InstInfo::new(421, 149, Encoding::VexRvm as u8, 0x4D, 127, 0), // #1533
    InstInfo::new(422, 149, Encoding::VexRvm as u8, 0x4D, 128, 0), // #1534
    InstInfo::new(423, 145, Encoding::VexRm_Lx as u8, 0x4C, 181, 0), // #1535
    InstInfo::new(301, 146, Encoding::VexRm_Lx as u8, 0x53, 74, 0), // #1536
    InstInfo::new(424, 145, Encoding::VexRvm as u8, 0x4D, 183, 0), // #1537
    InstInfo::new(425, 146, Encoding::VexRvm as u8, 0x53, 193, 0), // #1538
    InstInfo::new(401, 152, Encoding::VexRmi_Lx as u8, 0x56, 111, 0), // #1539
    InstInfo::new(311, 145, Encoding::VexRmi_Lx as u8, 0x56, 122, 0), // #1540
    InstInfo::new(400, 152, Encoding::VexRmi_Lx as u8, 0x56, 110, 0), // #1541
    InstInfo::new(426, 152, Encoding::VexRvmi as u8, 0x57, 178, 0), // #1542
    InstInfo::new(313, 145, Encoding::VexRvmi as u8, 0x57, 186, 0), // #1543
    InstInfo::new(427, 152, Encoding::VexRvmi as u8, 0x57, 179, 0), // #1544
    InstInfo::new(310, 149, Encoding::VexRmi_Lx as u8, 0x09, 111, 0), // #1545
    InstInfo::new(311, 145, Encoding::VexRmi_Lx as u8, 0x08, 122, 0), // #1546
    InstInfo::new(312, 149, Encoding::VexRmi_Lx as u8, 0x08, 110, 0), // #1547
    InstInfo::new(290, 149, Encoding::VexRvmi as u8, 0x0B, 178, 0), // #1548
    InstInfo::new(313, 145, Encoding::VexRvmi as u8, 0x0A, 186, 0), // #1549
    InstInfo::new(291, 149, Encoding::VexRvmi as u8, 0x0A, 179, 0), // #1550
    InstInfo::new(428, 146, Encoding::VexRmi_Lx as u8, 0x09, 75, 0), // #1551
    InstInfo::new(428, 146, Encoding::VexRmi_Lx as u8, 0x08, 75, 0), // #1552
    InstInfo::new(429, 146, Encoding::VexRvmi as u8, 0x0B, 75, 0), // #1553
    InstInfo::new(430, 146, Encoding::VexRvmi as u8, 0x0A, 75, 0), // #1554
    InstInfo::new(349, 149, Encoding::VexRm_Lx as u8, 0x4E, 112, 0), // #1555
    InstInfo::new(374, 149, Encoding::VexRm_Lx as u8, 0x4E, 113, 0), // #1556
    InstInfo::new(421, 149, Encoding::VexRvm as u8, 0x4F, 127, 0), // #1557
    InstInfo::new(422, 149, Encoding::VexRvm as u8, 0x4F, 128, 0), // #1558
    InstInfo::new(423, 145, Encoding::VexRm_Lx as u8, 0x4E, 181, 0), // #1559
    InstInfo::new(301, 146, Encoding::VexRm_Lx as u8, 0x52, 74, 0), // #1560
    InstInfo::new(424, 145, Encoding::VexRvm as u8, 0x4F, 183, 0), // #1561
    InstInfo::new(425, 146, Encoding::VexRvm as u8, 0x52, 193, 0), // #1562
    InstInfo::new(431, 149, Encoding::VexRvm_Lx as u8, 0x2C, 112, 0), // #1563
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x2C, 181, 0), // #1564
    InstInfo::new(287, 149, Encoding::VexRvm_Lx as u8, 0x2C, 113, 0), // #1565
    InstInfo::new(256, 149, Encoding::VexRvm as u8, 0x2D, 127, 0), // #1566
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x2D, 183, 0), // #1567
    InstInfo::new(264, 149, Encoding::VexRvm as u8, 0x2D, 128, 0), // #1568
    InstInfo::new(404, 149, Encoding::VexMr_VM as u8, 0xA2, 127, 0), // #1569
    InstInfo::new(403, 149, Encoding::VexMr_VM as u8, 0xA2, 128, 0), // #1570
    InstInfo::new(406, 149, Encoding::VexMr_VM as u8, 0xA3, 127, 0), // #1571
    InstInfo::new(405, 149, Encoding::VexMr_VM as u8, 0xA3, 128, 0), // #1572
    InstInfo::new(432, 187, Encoding::VexRm as u8, 0xCC, 224, 0), // #1573
    InstInfo::new(433, 187, Encoding::VexRm as u8, 0xCD, 224, 0), // #1574
    InstInfo::new(434, 187, Encoding::VexRvm as u8, 0xCB, 224, 0), // #1575
    InstInfo::new(435, 149, Encoding::VexRvmi_Lx as u8, 0x23, 110, 0), // #1576
    InstInfo::new(436, 149, Encoding::VexRvmi_Lx as u8, 0x23, 111, 0), // #1577
    InstInfo::new(435, 149, Encoding::VexRvmi_Lx as u8, 0x43, 110, 0), // #1578
    InstInfo::new(436, 149, Encoding::VexRvmi_Lx as u8, 0x43, 111, 0), // #1579
    InstInfo::new(437, 144, Encoding::VexRvmi_Lx as u8, 0xC6, 102, 0), // #1580
    InstInfo::new(438, 144, Encoding::VexRvmi_Lx as u8, 0xC6, 104, 0), // #1581
    InstInfo::new(439, 188, Encoding::VexRvm as u8, 0xDA, 11, 0), // #1582
    InstInfo::new(439, 188, Encoding::VexRvm as u8, 0xDA, 30, 0), // #1583
    InstInfo::new(281, 188, Encoding::VexRvmi as u8, 0xDE, 75, 0), // #1584
    InstInfo::new(205, 189, Encoding::VexRvm_Lx as u8, 0xDA, 89, 0), // #1585
    InstInfo::new(205, 189, Encoding::VexRvm_Lx as u8, 0xDA, 85, 0), // #1586
    InstInfo::new(440, 144, Encoding::VexRm_Lx as u8, 0x51, 102, 0), // #1587
    InstInfo::new(251, 145, Encoding::VexRm_Lx as u8, 0x51, 103, 0), // #1588
    InstInfo::new(239, 144, Encoding::VexRm_Lx as u8, 0x51, 104, 0), // #1589
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x51, 105, 0), // #1590
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x51, 106, 0), // #1591
    InstInfo::new(204, 144, Encoding::VexRvm as u8, 0x51, 107, 0), // #1592
    InstInfo::new(320, 146, Encoding::VexM as u8, 0xAE, 225, 0), // #1593
    InstInfo::new(199, 144, Encoding::VexRvm_Lx as u8, 0x5C, 102, 0), // #1594
    InstInfo::new(200, 145, Encoding::VexRvm_Lx as u8, 0x5C, 103, 0), // #1595
    InstInfo::new(201, 144, Encoding::VexRvm_Lx as u8, 0x5C, 104, 0), // #1596
    InstInfo::new(202, 144, Encoding::VexRvm as u8, 0x5C, 105, 0), // #1597
    InstInfo::new(203, 145, Encoding::VexRvm as u8, 0x5C, 106, 0), // #1598
    InstInfo::new(204, 144, Encoding::VexRvm as u8, 0x5C, 107, 0), // #1599
    InstInfo::new(301, 177, Encoding::VexRm_Lx as u8, 0x0F, 30, 0), // #1600
    InstInfo::new(301, 177, Encoding::VexRm_Lx as u8, 0x0E, 30, 0), // #1601
    InstInfo::new(233, 155, Encoding::VexRm as u8, 0x2E, 124, 0), // #1602
    InstInfo::new(234, 156, Encoding::VexRm as u8, 0x2E, 125, 0), // #1603
    InstInfo::new(235, 155, Encoding::VexRm as u8, 0x2E, 126, 0), // #1604
    InstInfo::new(211, 144, Encoding::VexRvm_Lx as u8, 0x15, 102, 0), // #1605
    InstInfo::new(212, 144, Encoding::VexRvm_Lx as u8, 0x15, 104, 0), // #1606
    InstInfo::new(211, 144, Encoding::VexRvm_Lx as u8, 0x14, 102, 0), // #1607
    InstInfo::new(212, 144, Encoding::VexRvm_Lx as u8, 0x14, 104, 0), // #1608
    InstInfo::new(418, 150, Encoding::VexRvm_Lx as u8, 0x57, 102, 0), // #1609
    InstInfo::new(417, 150, Encoding::VexRvm_Lx as u8, 0x57, 104, 0), // #1610
    InstInfo::new(441, 146, Encoding::VexOp as u8, 0x77, 70, 0), // #1611
    InstInfo::new(441, 146, Encoding::VexOp as u8, 0x77, 74, 0), // #1612
    InstInfo::new(31, 45, Encoding::X86Op as u8, 0x09, 5, 0), // #1613
    InstInfo::new(31, 190, Encoding::X86Op as u8, 0x09, 7, 0), // #1614
    InstInfo::new(177, 122, Encoding::X86M as u8, 0xAE, 226, 0), // #1615
    InstInfo::new(177, 122, Encoding::X86M as u8, 0xAE, 227, 0), // #1616
    InstInfo::new(180, 191, Encoding::X86Op as u8, 0x30, 5, 0), // #1617
    InstInfo::new(442, 65, Encoding::X86Mr as u8, 0xF6, 1, 0), // #1618
    InstInfo::new(443, 65, Encoding::X86Mr as u8, 0xF6, 228, 0), // #1619
    InstInfo::new(442, 65, Encoding::X86Mr as u8, 0xF5, 2, 0), // #1620
    InstInfo::new(443, 65, Encoding::X86Mr as u8, 0xF5, 229, 0), // #1621
    InstInfo::new(83, 192, Encoding::X86Op_Mod11RM_I8 as u8, 0xC6, 29, 0), // #1622
    InstInfo::new(444, 40, Encoding::X86Xadd as u8, 0xC0, 5, 0), // #1623
    InstInfo::new(445, 192, Encoding::X86JmpRel as u8, 0xC7, 29, 0), // #1624
    InstInfo::new(446, 0, Encoding::X86Xchg as u8, 0x86, 0, 0), // #1625
    InstInfo::new(31, 192, Encoding::X86Op as u8, 0xD5, 23, 0), // #1626
    InstInfo::new(180, 193, Encoding::X86Op as u8, 0xD0, 23, 0), // #1627
    InstInfo::new(31, 0, Encoding::X86Op as u8, 0xD7, 0, 0), // #1628
    InstInfo::new(184, 1, Encoding::X86Arith as u8, 0x30, 34, 0), // #1629
    InstInfo::new(154, 5, Encoding::ExtRm as u8, 0x57, 4, 0), // #1630
    InstInfo::new(154, 6, Encoding::ExtRm as u8, 0x57, 5, 0), // #1631
    InstInfo::new(31, 194, Encoding::X86Op as u8, 0xE9, 93, 0), // #1632
    InstInfo::new(447, 193, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 79, 0), // #1633
    InstInfo::new(448, 193, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 230, 0), // #1634
    InstInfo::new(447, 195, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 80, 0), // #1635
    InstInfo::new(448, 195, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 231, 0), // #1636
    InstInfo::new(447, 193, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 98, 0), // #1637
    InstInfo::new(448, 193, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 232, 0), // #1638
    InstInfo::new(447, 196, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 98, 0), // #1639
    InstInfo::new(448, 196, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 232, 0), // #1640
    InstInfo::new(447, 197, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 82, 0), // #1641
    InstInfo::new(448, 197, Encoding::X86M_Only_EDX_EAX as u8, 0xAE, 233, 0), // #1642
    InstInfo::new(447, 195, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 79, 0), // #1643
    InstInfo::new(448, 195, Encoding::X86M_Only_EDX_EAX as u8, 0xC7, 230, 0), // #1644
    InstInfo::new(180, 193, Encoding::X86Op as u8, 0xD1, 23, 0), // #1645
    InstInfo::new(31, 194, Encoding::X86Op as u8, 0xE8, 93, 0), // #1646
    InstInfo::new(31, 198, Encoding::X86Op as u8, 0xD6, 23, 0), // #1647
];

pub static MAIN_OPCODE_TABLE: &[u32] = &[
    0x00000000, // #0 [ref=56x]
    0x00000200, // #1 [ref=25x]
    0x00200200, // #2 [ref=44x]
    0x00080000, // #3 [ref=4x]
    0x00200100, // #4 [ref=38x]
    0x00000100, // #5 [ref=189x]
    0x00600100, // #6 [ref=24x]
    0x00400100, // #7 [ref=29x]
    0x00400200, // #8 [ref=3x]
    0x00200300, // #9 [ref=22x]
    0x00100000, // #10 [ref=4x]
    0x00000200, // #11 [ref=13x]
    0x00600200, // #12 [ref=3x]
    0x00040900, // #13 [ref=3x]
    0x00180900, // #14 [ref=2x]
    0x00140900, // #15 [ref=1x]
    0x000C0900, // #16 [ref=1x]
    0x00080900, // #17 [ref=1x]
    0x000C0200, // #18 [ref=1x]
    0x00080200, // #19 [ref=1x]
    0x00040200, // #20 [ref=1x]
    0x00200000, // #21 [ref=7x]
    0x08000000, // #22 [ref=3x]
    0x00000400, // #23 [ref=32x]
    0x001C0100, // #24 [ref=6x]
    0x003C0100, // #25 [ref=1x]
    0x00580100, // #26 [ref=4x]
    0x00400400, // #27 [ref=9x]
    0x00380100, // #28 [ref=3x]
    0x001C0000, // #29 [ref=5x]
    0x00200200, // #30 [ref=48x]
    0x08040100, // #31 [ref=2x]
    0x00040100, // #32 [ref=6x]
    0x00040000, // #33 [ref=3x]
    0x00180000, // #34 [ref=4x]
    0x005C6100, // #35 [ref=1x]
    0x005C4100, // #36 [ref=1x]
    0x00036400, // #37 [ref=29x]
    0x00030000, // #38 [ref=1x]
    0x00037800, // #39 [ref=7x]
    0x00100000, // #40 [ref=4x]
    0x00180000, // #41 [ref=4x]
    0x00E36C00, // #42 [ref=2x]
    0x00036800, // #43 [ref=5x]
    0x00036C00, // #44 [ref=8x]
    0x000B4000, // #45 [ref=1x]
    0x00037C00, // #46 [ref=2x]
    0x000F6000, // #47 [ref=1x]
    0x001BC000, // #48 [ref=1x]
    0x001FE000, // #49 [ref=1x]
    0x00037400, // #50 [ref=3x]
    0x00000000, // #51 [ref=4x]
    0x00080000, // #52 [ref=3x]
    0x000C0000, // #53 [ref=3x]
    0x001C0000, // #54 [ref=3x]
    0x00040000, // #55 [ref=2x]
    0x00140000, // #56 [ref=2x]
    0x00072000, // #57 [ref=1x]
    0x00F80000, // #58 [ref=2x]
    0x00FC0000, // #59 [ref=2x]
    0x00138000, // #60 [ref=1x]
    0x0017A000, // #61 [ref=1x]
    0x08000100, // #62 [ref=3x]
    0x00400300, // #63 [ref=1x]
    0x00140000, // #64 [ref=4x]
    0x00540100, // #65 [ref=2x]
    0x08540100, // #66 [ref=1x]
    0x20200100, // #67 [ref=7x]
    0x28200100, // #68 [ref=6x]
    0x28000100, // #69 [ref=7x]
    0x20000100, // #70 [ref=8x]
    0x00200100, // #71 [ref=15x]
    0x08200100, // #72 [ref=4x]
    0x08000100, // #73 [ref=4x]
    0x00000100, // #74 [ref=10x]
    0x00200300, // #75 [ref=48x]
    0x08200300, // #76 [ref=4x]
    0x000C0000, // #77 [ref=4x]
    0x00080100, // #78 [ref=5x]
    0x00140100, // #79 [ref=4x]
    0x000C0100, // #80 [ref=5x]
    0x00000900, // #81 [ref=32x]
    0x00180100, // #82 [ref=6x]
    0x00000A00, // #83 [ref=1x]
    0x00040A00, // #84 [ref=1x]
    0x00600200, // #85 [ref=11x]
    0x00000300, // #86 [ref=4x]
    0x00400000, // #87 [ref=1x]
    0x00000100, // #88 [ref=26x]
    0x00400200, // #89 [ref=12x]
    0x08000300, // #90 [ref=1x]
    0x08200300, // #91 [ref=1x]
    0x00500100, // #92 [ref=1x]
    0x00600400, // #93 [ref=5x]
    0x00440100, // #94 [ref=3x]
    0x005C0100, // #95 [ref=1x]
    0x00600300, // #96 [ref=1x]
    0x00200400, // #97 [ref=4x]
    0x00100100, // #98 [ref=4x]
    0x001C0900, // #99 [ref=1x]
    0x00100900, // #100 [ref=1x]
    0x00780100, // #101 [ref=1x]
    0x10218100, // #102 [ref=25x]
    0x00019500, // #103 [ref=10x]
    0x00018100, // #104 [ref=19x]
    0x10606100, // #105 [ref=10x]
    0x00403500, // #106 [ref=13x]
    0x00404100, // #107 [ref=12x]
    0x00600100, // #108 [ref=4x]
    0x00218200, // #109 [ref=50x]
    0x00219300, // #110 [ref=17x]
    0x10219300, // #111 [ref=18x]
    0x10219200, // #112 [ref=38x]
    0x00219200, // #113 [ref=25x]
    0x20200200, // #114 [ref=2x]
    0x00207200, // #115 [ref=2x]
    0x00209200, // #116 [ref=2x]
    0x4020B200, // #117 [ref=2x]
    0x10209200, // #118 [ref=2x]
    0x5020B200, // #119 [ref=2x]
    0x10206200, // #120 [ref=2x]
    0x00204200, // #121 [ref=14x]
    0x00019300, // #122 [ref=5x]
    0x00403300, // #123 [ref=1x]
    0x10206100, // #124 [ref=5x]
    0x00003500, // #125 [ref=2x]
    0x00004100, // #126 [ref=2x]
    0x10207200, // #127 [ref=12x]
    0x00205200, // #128 [ref=12x]
    0x00416100, // #129 [ref=1x]
    0x00619200, // #130 [ref=2x]
    0x00418200, // #131 [ref=1x]
    0x10618100, // #132 [ref=1x]
    0x10219500, // #133 [ref=1x]
    0x10219100, // #134 [ref=10x]
    0x10019100, // #135 [ref=3x]
    0x00217500, // #136 [ref=1x]
    0x00015500, // #137 [ref=1x]
    0x00216200, // #138 [ref=7x]
    0x00217600, // #139 [ref=1x]
    0x00215500, // #140 [ref=4x]
    0x00017500, // #141 [ref=2x]
    0x00219500, // #142 [ref=3x]
    0x00218100, // #143 [ref=43x]
    0x00016100, // #144 [ref=1x]
    0x00216300, // #145 [ref=1x]
    0x00217100, // #146 [ref=4x]
    0x00019100, // #147 [ref=2x]
    0x10419100, // #148 [ref=3x]
    0x10019500, // #149 [ref=1x]
    0x10607500, // #150 [ref=1x]
    0x00606100, // #151 [ref=2x]
    0x00607100, // #152 [ref=2x]
    0x00003600, // #153 [ref=1x]
    0x00624100, // #154 [ref=1x]
    0x00425500, // #155 [ref=2x]
    0x00424100, // #156 [ref=1x]
    0x00005500, // #157 [ref=1x]
    0x00405100, // #158 [ref=2x]
    0x00417500, // #159 [ref=1x]
    0x00418100, // #160 [ref=4x]
    0x00417100, // #161 [ref=1x]
    0x00619500, // #162 [ref=2x]
    0x00619100, // #163 [ref=2x]
    0x10619500, // #164 [ref=1x]
    0x10619100, // #165 [ref=2x]
    0x00625100, // #166 [ref=1x]
    0x00425100, // #167 [ref=1x]
    0x00419500, // #168 [ref=1x]
    0x00419200, // #169 [ref=3x]
    0x20200300, // #170 [ref=6x]
    0x00209300, // #171 [ref=4x]
    0x4020B300, // #172 [ref=4x]
    0x10209300, // #173 [ref=4x]
    0x5020B300, // #174 [ref=4x]
    0x00204300, // #175 [ref=4x]
    0x00619600, // #176 [ref=2x]
    0x00605600, // #177 [ref=2x]
    0x10207300, // #178 [ref=6x]
    0x00205300, // #179 [ref=6x]
    0x18218200, // #180 [ref=22x]
    0x00219600, // #181 [ref=22x]
    0x18206200, // #182 [ref=12x]
    0x00203600, // #183 [ref=16x]
    0x00419600, // #184 [ref=2x]
    0x00405600, // #185 [ref=2x]
    0x00003300, // #186 [ref=4x]
    0x08200200, // #187 [ref=5x]
    0x18218300, // #188 [ref=2x]
    0x00080100, // #189 [ref=1x]
    0x00204100, // #190 [ref=1x]
    0x10636100, // #191 [ref=1x]
    0x00219100, // #192 [ref=6x]
    0x00400100, // #193 [ref=3x]
    0x00419100, // #194 [ref=1x]
    0x00006100, // #195 [ref=2x]
    0x00203500, // #196 [ref=1x]
    0x10619200, // #197 [ref=1x]
    0x00218300, // #198 [ref=2x]
    0x10401200, // #199 [ref=5x]
    0x00401200, // #200 [ref=5x]
    0x00202200, // #201 [ref=1x]
    0x00000800, // #202 [ref=22x]
    0x10218200, // #203 [ref=4x]
    0x00201200, // #204 [ref=2x]
    0x10203200, // #205 [ref=2x]
    0x18219200, // #206 [ref=1x]
    0x18206300, // #207 [ref=2x]
    0x00202300, // #208 [ref=1x]
    0x00202100, // #209 [ref=1x]
    0x00415200, // #210 [ref=6x]
    0x00417200, // #211 [ref=9x]
    0x00413200, // #212 [ref=3x]
    0x00214200, // #213 [ref=4x]
    0x00212200, // #214 [ref=2x]
    0x00259100, // #215 [ref=1x]
    0x10259100, // #216 [ref=1x]
    0x00618100, // #217 [ref=1x]
    0x00208100, // #218 [ref=6x]
    0x003D8100, // #219 [ref=1x]
    0x10208100, // #220 [ref=2x]
    0x10209100, // #221 [ref=1x]
    0x002D8100, // #222 [ref=1x]
    0x10419200, // #223 [ref=2x]
    0x20600200, // #224 [ref=3x]
    0x000C0100, // #225 [ref=1x]
    0x00480100, // #226 [ref=1x]
    0x004C0100, // #227 [ref=1x]
    0x08000200, // #228 [ref=1x]
    0x08200200, // #229 [ref=1x]
    0x08140100, // #230 [ref=2x]
    0x080C0100, // #231 [ref=1x]
    0x08100100, // #232 [ref=2x]
    0x08180100, // #233 [ref=1x]
];

pub static ALT_OPCODE_TABLE: &[u32] = &[
    0x00000000, // #0 [ref=1512x]
    0x0020011B, // #1 [ref=1x]
    0x001001BA, // #2 [ref=1x]
    0x001C01BA, // #3 [ref=1x]
    0x001801BA, // #4 [ref=1x]
    0x001401BA, // #5 [ref=1x]
    0x00000048, // #6 [ref=1x]
    0x00200178, // #7 [ref=1x]
    0x001400DF, // #8 [ref=1x]
    0x001C00DF, // #9 [ref=1x]
    0x000400DD, // #10 [ref=1x]
    0x001400DB, // #11 [ref=1x]
    0x00037CE0, // #12 [ref=1x]
    0x001C00DB, // #13 [ref=1x]
    0x00E37CE0, // #14 [ref=1x]
    0x000000E4, // #15 [ref=1x]
    0x00000040, // #16 [ref=1x]
    0x00600178, // #17 [ref=1x]
    0x00000072, // #18 [ref=1x]
    0x00000076, // #19 [ref=1x]
    0x000000E3, // #20 [ref=1x]
    0x0000007C, // #21 [ref=1x]
    0x0000007E, // #22 [ref=1x]
    0x000000EB, // #23 [ref=1x]
    0x00000073, // #24 [ref=1x]
    0x00000077, // #25 [ref=1x]
    0x0000007D, // #26 [ref=1x]
    0x0000007F, // #27 [ref=1x]
    0x00000071, // #28 [ref=1x]
    0x0000007B, // #29 [ref=1x]
    0x00000079, // #30 [ref=1x]
    0x00000075, // #31 [ref=1x]
    0x00000070, // #32 [ref=1x]
    0x0000007A, // #33 [ref=1x]
    0x00000078, // #34 [ref=1x]
    0x00000074, // #35 [ref=1x]
    0x00200192, // #36 [ref=1x]
    0x00600192, // #37 [ref=1x]
    0x08600192, // #38 [ref=1x]
    0x00000192, // #39 [ref=1x]
    0x0000009A, // #40 [ref=1x]
    0x000000EA, // #41 [ref=1x]
    0x000000E2, // #42 [ref=1x]
    0x000000E1, // #43 [ref=1x]
    0x000000E0, // #44 [ref=1x]
    0x00200129, // #45 [ref=1x]
    0x00000129, // #46 [ref=1x]
    0x000002F1, // #47 [ref=1x]
    0x0000017E, // #48 [ref=2x]
    0x0020017F, // #49 [ref=1x]
    0x0040017F, // #50 [ref=1x]
    0x00200117, // #51 [ref=1x]
    0x00000117, // #52 [ref=1x]
    0x00200113, // #53 [ref=1x]
    0x00000113, // #54 [ref=1x]
    0x002001E7, // #55 [ref=1x]
    0x0020012B, // #56 [ref=1x]
    0x0000012B, // #57 [ref=1x]
    0x000001E7, // #58 [ref=1x]
    0x0060012B, // #59 [ref=1x]
    0x0040012B, // #60 [ref=1x]
    0x00600111, // #61 [ref=1x]
    0x00400111, // #62 [ref=1x]
    0x00200111, // #63 [ref=1x]
    0x00000111, // #64 [ref=1x]
    0x000000E6, // #65 [ref=1x]
    0x00000315, // #66 [ref=1x]
    0x00000058, // #67 [ref=1x]
    0x00180172, // #68 [ref=1x]
    0x003C0173, // #69 [ref=1x]
    0x00180173, // #70 [ref=1x]
    0x00180171, // #71 [ref=1x]
    0x00100172, // #72 [ref=1x]
    0x00100171, // #73 [ref=1x]
    0x00080172, // #74 [ref=1x]
    0x002C0173, // #75 [ref=1x]
    0x00080173, // #76 [ref=1x]
    0x00080171, // #77 [ref=1x]
    0x00000050, // #78 [ref=2x]
    0x000000F6, // #79 [ref=1x]
    0x10207292, // #80 [ref=1x]
    0x00205292, // #81 [ref=1x]
    0x10207293, // #82 [ref=1x]
    0x00205293, // #83 [ref=1x]
    0x0020022F, // #84 [ref=1x]
    0x0020022E, // #85 [ref=1x]
    0x10218129, // #86 [ref=1x]
    0x00018129, // #87 [ref=1x]
    0x0020417E, // #88 [ref=1x]
    0x0020017F, // #89 [ref=1x]
    0x0021917F, // #90 [ref=1x]
    0x1021917F, // #91 [ref=1x]
    0x0040017F, // #92 [ref=1x]
    0x1061917F, // #93 [ref=1x]
    0x0041917F, // #94 [ref=1x]
    0x1041917F, // #95 [ref=1x]
    0x0061917F, // #96 [ref=1x]
    0x10206117, // #97 [ref=1x]
    0x00006117, // #98 [ref=1x]
    0x10206113, // #99 [ref=1x]
    0x00006113, // #100 [ref=1x]
    0x1020617E, // #101 [ref=1x]
    0x10606111, // #102 [ref=1x]
    0x00403511, // #103 [ref=1x]
    0x00404111, // #104 [ref=1x]
    0x10218111, // #105 [ref=1x]
    0x00018111, // #106 [ref=1x]
    0x0020357E, // #107 [ref=1x]
    0x0020127A, // #108 [ref=1x]
    0x0020127C, // #109 [ref=1x]
    0x1020127C, // #110 [ref=1x]
    0x0020127B, // #111 [ref=1x]
    0x10218305, // #112 [ref=1x]
    0x00218304, // #113 [ref=1x]
    0x18218301, // #114 [ref=1x]
    0x18218300, // #115 [ref=1x]
    0x00205290, // #116 [ref=1x]
    0x10207290, // #117 [ref=1x]
    0x00205291, // #118 [ref=1x]
    0x10207291, // #119 [ref=1x]
    0x0020028E, // #120 [ref=1x]
    0x0820028E, // #121 [ref=1x]
    0x000008C0, // #122 [ref=1x]
    0x000008C2, // #123 [ref=1x]
    0x000008C3, // #124 [ref=1x]
    0x000008C1, // #125 [ref=1x]
    0x00398172, // #126 [ref=1x]
    0x10398173, // #127 [ref=1x]
    0x00398171, // #128 [ref=1x]
    0x00318172, // #129 [ref=1x]
    0x10319172, // #130 [ref=1x]
    0x00318171, // #131 [ref=1x]
    0x00298172, // #132 [ref=1x]
    0x10298173, // #133 [ref=1x]
    0x00298171, // #134 [ref=1x]
];

/// Aggregated instruction information, indexed by [`InstInfo::common_info_index`].
pub static INST_COMMON_INFO_TABLE: &[CommonInfo] = &[
    CommonInfo::new(0, 0, 0, 0, InstControlFlow::Regular, InstSameRegHint::None), // #0 [ref=1x]
    CommonInfo::new(0, 0, 487, 1, InstControlFlow::Regular, InstSameRegHint::None), // #1 [ref=4x]
    CommonInfo::new(0, 0, 488, 1, InstControlFlow::Regular, InstSameRegHint::None), // #2 [ref=2x]
    CommonInfo::new(0, 0, 143, 2, InstControlFlow::Regular, InstSameRegHint::None), // #3 [ref=6x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 20, 13, InstControlFlow::Regular, InstSameRegHint::None), // #4 [ref=2x]
    CommonInfo::new(0, 0, 77, 2, InstControlFlow::Regular, InstSameRegHint::None), // #5 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 99, 1, InstControlFlow::Regular, InstSameRegHint::None), // #6 [ref=54x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 172, 1, InstControlFlow::Regular, InstSameRegHint::None), // #7 [ref=19x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 313, 1, InstControlFlow::Regular, InstSameRegHint::None), // #8 [ref=16x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 322, 1, InstControlFlow::Regular, InstSameRegHint::None), // #9 [ref=20x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 33, 13, InstControlFlow::Regular, InstSameRegHint::RO), // #10 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 355, 2, InstControlFlow::Regular, InstSameRegHint::None), // #11 [ref=3x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 99, 1, InstControlFlow::Regular, InstSameRegHint::RO), // #12 [ref=12x]
    CommonInfo::new(0, 0, 489, 1, InstControlFlow::Regular, InstSameRegHint::None), // #13 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 357, 2, InstControlFlow::Regular, InstSameRegHint::None), // #14 [ref=5x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 77, 2, InstControlFlow::Regular, InstSameRegHint::None), // #15 [ref=12x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 490, 1, InstControlFlow::Regular, InstSameRegHint::None), // #16 [ref=4x]
    CommonInfo::new(0, 0, 359, 2, InstControlFlow::Regular, InstSameRegHint::None), // #17 [ref=3x]
    CommonInfo::new(InstFlags::MIB.bits(), 0, 491, 1, InstControlFlow::Regular, InstSameRegHint::None), // #18 [ref=1x]
    CommonInfo::new(0, 0, 492, 1, InstControlFlow::Regular, InstSameRegHint::None), // #19 [ref=1x]
    CommonInfo::new(0, 0, 361, 2, InstControlFlow::Regular, InstSameRegHint::None), // #20 [ref=1x]
    CommonInfo::new(InstFlags::MIB.bits(), 0, 493, 1, InstControlFlow::Regular, InstSameRegHint::None), // #21 [ref=1x]
    CommonInfo::new(0, 0, 363, 2, InstControlFlow::Regular, InstSameRegHint::None), // #22 [ref=1x]
    CommonInfo::new(0, 0, 76, 3, InstControlFlow::Regular, InstSameRegHint::None), // #23 [ref=21x]
    CommonInfo::new(0, 0, 365, 2, InstControlFlow::Regular, InstSameRegHint::None), // #24 [ref=3x]
    CommonInfo::new(0, 0, 163, 5, InstControlFlow::Regular, InstSameRegHint::None), // #25 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 163, 5, InstControlFlow::Regular, InstSameRegHint::None), // #26 [ref=3x]
    CommonInfo::new(InstFlags::REP.bits() | InstFlags::REP_IGNORED.bits(), 0, 268, 3, InstControlFlow::Call, InstSameRegHint::None), // #27 [ref=1x]
    CommonInfo::new(0, 0, 494, 1, InstControlFlow::Regular, InstSameRegHint::None), // #28 [ref=1x]
    CommonInfo::new(0, 0, 495, 1, InstControlFlow::Regular, InstSameRegHint::None), // #29 [ref=2x]
    CommonInfo::new(0, 0, 468, 1, InstControlFlow::Regular, InstSameRegHint::None), // #30 [ref=1x]
    CommonInfo::new(0, 0, 145, 1, InstControlFlow::Regular, InstSameRegHint::None), // #31 [ref=88x]
    CommonInfo::new(0, 0, 496, 1, InstControlFlow::Regular, InstSameRegHint::None), // #32 [ref=24x]
    CommonInfo::new(0, 0, 497, 1, InstControlFlow::Regular, InstSameRegHint::None), // #33 [ref=6x]
    CommonInfo::new(0, 0, 498, 1, InstControlFlow::Regular, InstSameRegHint::None), // #34 [ref=14x]
    CommonInfo::new(0, 0, 499, 1, InstControlFlow::Regular, InstSameRegHint::None), // #35 [ref=1x]
    CommonInfo::new(0, 0, 46, 13, InstControlFlow::Regular, InstSameRegHint::None), // #36 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 367, 2, InstControlFlow::Regular, InstSameRegHint::None), // #37 [ref=16x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 208, 4, InstControlFlow::Regular, InstSameRegHint::None), // #38 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 500, 1, InstControlFlow::Regular, InstSameRegHint::None), // #39 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 501, 1, InstControlFlow::Regular, InstSameRegHint::None), // #40 [ref=3x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 212, 4, InstControlFlow::Regular, InstSameRegHint::None), // #41 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 502, 1, InstControlFlow::Regular, InstSameRegHint::None), // #42 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 503, 1, InstControlFlow::Regular, InstSameRegHint::None), // #43 [ref=1x]
    CommonInfo::new(0, 0, 504, 1, InstControlFlow::Regular, InstSameRegHint::None), // #44 [ref=1x]
    CommonInfo::new(0, 0, 505, 1, InstControlFlow::Regular, InstSameRegHint::None), // #45 [ref=1x]
    CommonInfo::new(0, 0, 369, 2, InstControlFlow::Regular, InstSameRegHint::None), // #46 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 506, 1, InstControlFlow::Regular, InstSameRegHint::None), // #47 [ref=2x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 507, 1, InstControlFlow::Regular, InstSameRegHint::None), // #48 [ref=2x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 508, 1, InstControlFlow::Regular, InstSameRegHint::None), // #49 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 371, 2, InstControlFlow::Regular, InstSameRegHint::None), // #50 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 373, 2, InstControlFlow::Regular, InstSameRegHint::None), // #51 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 375, 2, InstControlFlow::Regular, InstSameRegHint::None), // #52 [ref=2x]
    CommonInfo::new(0, 0, 509, 1, InstControlFlow::Regular, InstSameRegHint::None), // #53 [ref=1x]
    CommonInfo::new(0, 0, 510, 1, InstControlFlow::Regular, InstSameRegHint::None), // #54 [ref=2x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 271, 3, InstControlFlow::Regular, InstSameRegHint::None), // #55 [ref=1x]
    CommonInfo::new(0, 0, 72, 4, InstControlFlow::Regular, InstSameRegHint::None), // #56 [ref=3x]
    CommonInfo::new(InstFlags::MMX.bits(), 0, 145, 1, InstControlFlow::Regular, InstSameRegHint::None), // #57 [ref=1x]
    CommonInfo::new(0, 0, 377, 2, InstControlFlow::Regular, InstSameRegHint::None), // #58 [ref=2x]
    CommonInfo::new(0, 0, 511, 1, InstControlFlow::Regular, InstSameRegHint::None), // #59 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 512, 1, InstControlFlow::Regular, InstSameRegHint::None), // #60 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 379, 2, InstControlFlow::Regular, InstSameRegHint::None), // #61 [ref=1x]
    CommonInfo::new(InstFlags::FPU_M32.bits() | InstFlags::FPU_M64.bits(), 0, 274, 3, InstControlFlow::Regular, InstSameRegHint::None), // #62 [ref=6x]
    CommonInfo::new(0, 0, 381, 2, InstControlFlow::Regular, InstSameRegHint::None), // #63 [ref=9x]
    CommonInfo::new(InstFlags::FPU_M80.bits(), 0, 513, 1, InstControlFlow::Regular, InstSameRegHint::None), // #64 [ref=2x]
    CommonInfo::new(0, 0, 382, 1, InstControlFlow::Regular, InstSameRegHint::None), // #65 [ref=13x]
    CommonInfo::new(InstFlags::FPU_M32.bits() | InstFlags::FPU_M64.bits(), 0, 383, 2, InstControlFlow::Regular, InstSameRegHint::None), // #66 [ref=2x]
    CommonInfo::new(InstFlags::FPU_M16.bits() | InstFlags::FPU_M32.bits(), 0, 514, 1, InstControlFlow::Regular, InstSameRegHint::None), // #67 [ref=9x]
    CommonInfo::new(InstFlags::FPU_M16.bits() | InstFlags::FPU_M32.bits() | InstFlags::FPU_M64.bits(), 0, 515, 1, InstControlFlow::Regular, InstSameRegHint::None), // #68 [ref=3x]
    CommonInfo::new(InstFlags::FPU_M32.bits() | InstFlags::FPU_M64.bits() | InstFlags::FPU_M80.bits(), 0, 516, 1, InstControlFlow::Regular, InstSameRegHint::None), // #69 [ref=2x]
    CommonInfo::new(InstFlags::FPU_M16.bits(), 0, 517, 1, InstControlFlow::Regular, InstSameRegHint::None), // #70 [ref=3x]
    CommonInfo::new(InstFlags::FPU_M16.bits(), 0, 518, 1, InstControlFlow::Regular, InstSameRegHint::None), // #71 [ref=2x]
    CommonInfo::new(InstFlags::FPU_M32.bits() | InstFlags::FPU_M64.bits(), 0, 384, 1, InstControlFlow::Regular, InstSameRegHint::None), // #72 [ref=1x]
    CommonInfo::new(0, 0, 519, 1, InstControlFlow::Regular, InstSameRegHint::None), // #73 [ref=4x]
    CommonInfo::new(0, 0, 520, 1, InstControlFlow::Regular, InstSameRegHint::None), // #74 [ref=1x]
    CommonInfo::new(0, 0, 521, 1, InstControlFlow::Regular, InstSameRegHint::None), // #75 [ref=1x]
    CommonInfo::new(0, 0, 72, 10, InstControlFlow::Regular, InstSameRegHint::None), // #76 [ref=1x]
    CommonInfo::new(0, 0, 522, 1, InstControlFlow::Regular, InstSameRegHint::None), // #77 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits(), 0, 271, 3, InstControlFlow::Regular, InstSameRegHint::None), // #78 [ref=1x]
    CommonInfo::new(0, 0, 407, 1, InstControlFlow::Regular, InstSameRegHint::None), // #79 [ref=2x]
    CommonInfo::new(0, 0, 366, 1, InstControlFlow::Regular, InstSameRegHint::None), // #80 [ref=3x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 523, 1, InstControlFlow::Regular, InstSameRegHint::None), // #81 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 385, 2, InstControlFlow::Regular, InstSameRegHint::None), // #82 [ref=1x]
    CommonInfo::new(0, 0, 524, 1, InstControlFlow::Regular, InstSameRegHint::None), // #83 [ref=2x]
    CommonInfo::new(0, 0, 525, 1, InstControlFlow::Regular, InstSameRegHint::None), // #84 [ref=8x]
    CommonInfo::new(0, 0, 387, 2, InstControlFlow::Regular, InstSameRegHint::None), // #85 [ref=3x]
    CommonInfo::new(0, 0, 389, 2, InstControlFlow::Regular, InstSameRegHint::None), // #86 [ref=1x]
    CommonInfo::new(0, 0, 391, 2, InstControlFlow::Regular, InstSameRegHint::None), // #87 [ref=1x]
    CommonInfo::new(0, 0, 145, 1, InstControlFlow::Return, InstSameRegHint::None), // #88 [ref=2x]
    CommonInfo::new(0, 0, 498, 1, InstControlFlow::Return, InstSameRegHint::None), // #89 [ref=1x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 393, 2, InstControlFlow::Branch, InstSameRegHint::None), // #90 [ref=16x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 395, 2, InstControlFlow::Branch, InstSameRegHint::None), // #91 [ref=1x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 277, 3, InstControlFlow::Jump, InstSameRegHint::None), // #92 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 526, 1, InstControlFlow::Regular, InstSameRegHint::None), // #93 [ref=19x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 397, 2, InstControlFlow::Regular, InstSameRegHint::None), // #94 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 399, 2, InstControlFlow::Regular, InstSameRegHint::None), // #95 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 216, 4, InstControlFlow::Regular, InstSameRegHint::None), // #96 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 401, 2, InstControlFlow::Regular, InstSameRegHint::None), // #97 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 527, 1, InstControlFlow::Regular, InstSameRegHint::None), // #98 [ref=12x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 528, 1, InstControlFlow::Regular, InstSameRegHint::None), // #99 [ref=8x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 526, 1, InstControlFlow::Regular, InstSameRegHint::WO), // #100 [ref=8x]
    CommonInfo::new(0, 0, 529, 1, InstControlFlow::Regular, InstSameRegHint::None), // #101 [ref=2x]
    CommonInfo::new(0, 0, 286, 2, InstControlFlow::Regular, InstSameRegHint::None), // #102 [ref=1x]
    CommonInfo::new(0, 0, 280, 3, InstControlFlow::Call, InstSameRegHint::None), // #103 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 198, 1, InstControlFlow::Regular, InstSameRegHint::None), // #104 [ref=2x]
    CommonInfo::new(0, 0, 530, 1, InstControlFlow::Regular, InstSameRegHint::None), // #105 [ref=2x]
    CommonInfo::new(0, 0, 403, 2, InstControlFlow::Regular, InstSameRegHint::None), // #106 [ref=2x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 531, 1, InstControlFlow::Regular, InstSameRegHint::None), // #107 [ref=2x]
    CommonInfo::new(0, 0, 405, 2, InstControlFlow::Regular, InstSameRegHint::None), // #108 [ref=1x]
    CommonInfo::new(0, 0, 283, 3, InstControlFlow::Regular, InstSameRegHint::None), // #109 [ref=3x]
    CommonInfo::new(0, 0, 280, 3, InstControlFlow::Jump, InstSameRegHint::None), // #110 [ref=1x]
    CommonInfo::new(0, 0, 532, 1, InstControlFlow::Regular, InstSameRegHint::None), // #111 [ref=5x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 407, 2, InstControlFlow::Regular, InstSameRegHint::None), // #112 [ref=2x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 220, 4, InstControlFlow::Regular, InstSameRegHint::None), // #113 [ref=1x]
    CommonInfo::new(0, 0, 395, 2, InstControlFlow::Branch, InstSameRegHint::None), // #114 [ref=3x]
    CommonInfo::new(0, 0, 286, 3, InstControlFlow::Regular, InstSameRegHint::None), // #115 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 409, 2, InstControlFlow::Regular, InstSameRegHint::None), // #116 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 533, 1, InstControlFlow::Regular, InstSameRegHint::None), // #117 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits(), 0, 534, 1, InstControlFlow::Regular, InstSameRegHint::None), // #118 [ref=1x]
    CommonInfo::new(0, 0, 535, 1, InstControlFlow::Regular, InstSameRegHint::None), // #119 [ref=2x]
    CommonInfo::new(InstFlags::X_RELEASE.bits(), 0, 0, 20, InstControlFlow::Regular, InstSameRegHint::None), // #120 [ref=1x]
    CommonInfo::new(0, 0, 82, 9, InstControlFlow::Regular, InstSameRegHint::None), // #121 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 411, 2, InstControlFlow::Regular, InstSameRegHint::None), // #122 [ref=6x]
    CommonInfo::new(0, 0, 139, 6, InstControlFlow::Regular, InstSameRegHint::None), // #123 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 413, 2, InstControlFlow::Regular, InstSameRegHint::None), // #124 [ref=1x]
    CommonInfo::new(0, 0, 415, 2, InstControlFlow::Regular, InstSameRegHint::None), // #125 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 536, 1, InstControlFlow::Regular, InstSameRegHint::None), // #126 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 380, 1, InstControlFlow::Regular, InstSameRegHint::None), // #127 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 107, 2, InstControlFlow::Regular, InstSameRegHint::None), // #128 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 537, 1, InstControlFlow::Regular, InstSameRegHint::None), // #129 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 101, 1, InstControlFlow::Regular, InstSameRegHint::None), // #130 [ref=3x]
    CommonInfo::new(InstFlags::MMX.bits(), 0, 538, 1, InstControlFlow::Regular, InstSameRegHint::None), // #131 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 107, 1, InstControlFlow::Regular, InstSameRegHint::None), // #132 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 115, 1, InstControlFlow::Regular, InstSameRegHint::None), // #133 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 168, 5, InstControlFlow::Regular, InstSameRegHint::None), // #134 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 539, 1, InstControlFlow::Regular, InstSameRegHint::None), // #135 [ref=1x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 224, 4, InstControlFlow::Regular, InstSameRegHint::None), // #136 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 417, 2, InstControlFlow::Regular, InstSameRegHint::None), // #137 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 419, 2, InstControlFlow::Regular, InstSameRegHint::None), // #138 [ref=1x]
    CommonInfo::new(0, 0, 289, 3, InstControlFlow::Regular, InstSameRegHint::None), // #139 [ref=2x]
    CommonInfo::new(0, 0, 421, 2, InstControlFlow::Regular, InstSameRegHint::None), // #140 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 423, 2, InstControlFlow::Regular, InstSameRegHint::None), // #141 [ref=1x]
    CommonInfo::new(0, 0, 540, 1, InstControlFlow::Regular, InstSameRegHint::None), // #142 [ref=1x]
    CommonInfo::new(0, 0, 541, 1, InstControlFlow::Regular, InstSameRegHint::None), // #143 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 272, 2, InstControlFlow::Regular, InstSameRegHint::None), // #144 [ref=2x]
    CommonInfo::new(0, 0, 145, 6, InstControlFlow::Regular, InstSameRegHint::None), // #145 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 59, 13, InstControlFlow::Regular, InstSameRegHint::RO), // #146 [ref=1x]
    CommonInfo::new(0, 0, 542, 1, InstControlFlow::Regular, InstSameRegHint::None), // #147 [ref=1x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 543, 1, InstControlFlow::Regular, InstSameRegHint::None), // #148 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 425, 2, InstControlFlow::Regular, InstSameRegHint::None), // #149 [ref=37x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 427, 2, InstControlFlow::Regular, InstSameRegHint::None), // #150 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 425, 2, InstControlFlow::Regular, InstSameRegHint::RO), // #151 [ref=6x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 425, 2, InstControlFlow::Regular, InstSameRegHint::WO), // #152 [ref=16x]
    CommonInfo::new(InstFlags::MMX.bits(), 0, 168, 1, InstControlFlow::Regular, InstSameRegHint::None), // #153 [ref=26x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 99, 1, InstControlFlow::Regular, InstSameRegHint::WO), // #154 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 544, 1, InstControlFlow::Regular, InstSameRegHint::None), // #155 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 545, 1, InstControlFlow::Regular, InstSameRegHint::None), // #156 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 546, 1, InstControlFlow::Regular, InstSameRegHint::None), // #157 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 547, 1, InstControlFlow::Regular, InstSameRegHint::None), // #158 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 548, 1, InstControlFlow::Regular, InstSameRegHint::None), // #159 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 549, 1, InstControlFlow::Regular, InstSameRegHint::None), // #160 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 429, 2, InstControlFlow::Regular, InstSameRegHint::None), // #161 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 550, 1, InstControlFlow::Regular, InstSameRegHint::None), // #162 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 551, 1, InstControlFlow::Regular, InstSameRegHint::None), // #163 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 552, 1, InstControlFlow::Regular, InstSameRegHint::None), // #164 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 553, 1, InstControlFlow::Regular, InstSameRegHint::None), // #165 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 554, 1, InstControlFlow::Regular, InstSameRegHint::None), // #166 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 343, 1, InstControlFlow::Regular, InstSameRegHint::None), // #167 [ref=2x]
    CommonInfo::new(0, 0, 173, 5, InstControlFlow::Regular, InstSameRegHint::None), // #168 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits(), 0, 427, 1, InstControlFlow::Regular, InstSameRegHint::None), // #169 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 431, 2, InstControlFlow::Regular, InstSameRegHint::None), // #170 [ref=8x]
    CommonInfo::new(InstFlags::VEC.bits(), 0, 555, 1, InstControlFlow::Regular, InstSameRegHint::None), // #171 [ref=2x]
    CommonInfo::new(0, 0, 433, 2, InstControlFlow::Regular, InstSameRegHint::None), // #172 [ref=1x]
    CommonInfo::new(InstFlags::MMX.bits() | InstFlags::VEC.bits(), 0, 435, 2, InstControlFlow::Regular, InstSameRegHint::None), // #173 [ref=3x]
    CommonInfo::new(0, 0, 178, 5, InstControlFlow::Regular, InstSameRegHint::None), // #174 [ref=1x]
    CommonInfo::new(0, 0, 556, 1, InstControlFlow::Regular, InstSameRegHint::None), // #175 [ref=1x]
    CommonInfo::new(0, 0, 437, 2, InstControlFlow::Regular, InstSameRegHint::None), // #176 [ref=7x]
    CommonInfo::new(0, 0, 557, 1, InstControlFlow::Regular, InstSameRegHint::None), // #177 [ref=4x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 439, 2, InstControlFlow::Regular, InstSameRegHint::None), // #178 [ref=1x]
    CommonInfo::new(0, 0, 441, 2, InstControlFlow::Regular, InstSameRegHint::None), // #179 [ref=1x]
    CommonInfo::new(0, 0, 439, 1, InstControlFlow::Regular, InstSameRegHint::None), // #180 [ref=7x]
    CommonInfo::new(InstFlags::REP.bits() | InstFlags::REP_IGNORED.bits(), 0, 443, 2, InstControlFlow::Return, InstSameRegHint::None), // #181 [ref=1x]
    CommonInfo::new(0, 0, 443, 2, InstControlFlow::Return, InstSameRegHint::None), // #182 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 445, 2, InstControlFlow::Regular, InstSameRegHint::None), // #183 [ref=1x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 20, 13, InstControlFlow::Regular, InstSameRegHint::WO), // #184 [ref=3x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 228, 4, InstControlFlow::Regular, InstSameRegHint::None), // #185 [ref=1x]
    CommonInfo::new(0, 0, 558, 1, InstControlFlow::Regular, InstSameRegHint::None), // #186 [ref=16x]
    CommonInfo::new(0, 0, 292, 3, InstControlFlow::Regular, InstSameRegHint::None), // #187 [ref=2x]
    CommonInfo::new(0, 0, 447, 2, InstControlFlow::Regular, InstSameRegHint::None), // #188 [ref=3x]
    CommonInfo::new(InstFlags::REP.bits(), 0, 232, 4, InstControlFlow::Regular, InstSameRegHint::None), // #189 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 559, 1, InstControlFlow::Regular, InstSameRegHint::None), // #190 [ref=8x]
    CommonInfo::new(0, 0, 91, 8, InstControlFlow::Regular, InstSameRegHint::None), // #191 [ref=1x]
    CommonInfo::new(InstFlags::TSIB.bits() | InstFlags::VEX.bits(), 0, 560, 1, InstControlFlow::Regular, InstSameRegHint::None), // #192 [ref=2x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 498, 1, InstControlFlow::Regular, InstSameRegHint::None), // #193 [ref=1x]
    CommonInfo::new(InstFlags::TSIB.bits() | InstFlags::VEX.bits(), 0, 561, 1, InstControlFlow::Regular, InstSameRegHint::None), // #194 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 562, 1, InstControlFlow::Regular, InstSameRegHint::None), // #195 [ref=1x]
    CommonInfo::new(0, 0, 563, 1, InstControlFlow::Regular, InstSameRegHint::None), // #196 [ref=2x]
    CommonInfo::new(0, 0, 77, 1, InstControlFlow::Regular, InstSameRegHint::None), // #197 [ref=2x]
    CommonInfo::new(0, 0, 449, 2, InstControlFlow::Regular, InstSameRegHint::None), // #198 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #199 [ref=22x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #200 [ref=23x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #201 [ref=22x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 564, 1, InstControlFlow::Regular, InstSameRegHint::None), // #202 [ref=18x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 565, 1, InstControlFlow::Regular, InstSameRegHint::None), // #203 [ref=18x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #204 [ref=17x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 295, 2, InstControlFlow::Regular, InstSameRegHint::None), // #205 [ref=29x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #206 [ref=5x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 99, 1, InstControlFlow::Regular, InstSameRegHint::None), // #207 [ref=17x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 322, 1, InstControlFlow::Regular, InstSameRegHint::None), // #208 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #209 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #210 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #211 [ref=10x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #212 [ref=12x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::RO), // #213 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::RO), // #214 [ref=6x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 567, 1, InstControlFlow::Regular, InstSameRegHint::None), // #215 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #216 [ref=17x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #217 [ref=12x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 298, 2, InstControlFlow::Regular, InstSameRegHint::None), // #218 [ref=6x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 451, 2, InstControlFlow::Regular, InstSameRegHint::None), // #219 [ref=3x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 568, 1, InstControlFlow::Regular, InstSameRegHint::None), // #220 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 569, 1, InstControlFlow::Regular, InstSameRegHint::None), // #221 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 570, 1, InstControlFlow::Regular, InstSameRegHint::None), // #222 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 571, 1, InstControlFlow::Regular, InstSameRegHint::None), // #223 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 477, 1, InstControlFlow::Regular, InstSameRegHint::None), // #224 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 569, 1, InstControlFlow::Regular, InstSameRegHint::None), // #225 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 572, 1, InstControlFlow::Regular, InstSameRegHint::None), // #226 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 301, 3, InstControlFlow::Regular, InstSameRegHint::None), // #227 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 304, 3, InstControlFlow::Regular, InstSameRegHint::None), // #228 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 301, 3, InstControlFlow::Regular, InstSameRegHint::None), // #229 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 573, 1, InstControlFlow::Regular, InstSameRegHint::None), // #230 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 574, 1, InstControlFlow::Regular, InstSameRegHint::None), // #231 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits(), 575, 1, InstControlFlow::Regular, InstSameRegHint::None), // #232 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::SAE.bits(), 172, 1, InstControlFlow::Regular, InstSameRegHint::None), // #233 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::SAE.bits(), 343, 1, InstControlFlow::Regular, InstSameRegHint::None), // #234 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::SAE.bits(), 313, 1, InstControlFlow::Regular, InstSameRegHint::None), // #235 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 307, 3, InstControlFlow::Regular, InstSameRegHint::None), // #236 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #237 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #238 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #239 [ref=3x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 198, 2, InstControlFlow::Regular, InstSameRegHint::None), // #240 [ref=5x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::PREFER_EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #241 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #242 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 576, 1, InstControlFlow::Regular, InstSameRegHint::None), // #243 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #244 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #245 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #246 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 313, 3, InstControlFlow::Regular, InstSameRegHint::None), // #247 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #248 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #249 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 313, 3, InstControlFlow::Regular, InstSameRegHint::None), // #250 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #251 [ref=5x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #252 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 316, 3, InstControlFlow::Regular, InstSameRegHint::None), // #253 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #254 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #255 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 564, 1, InstControlFlow::Regular, InstSameRegHint::None), // #256 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 371, 2, InstControlFlow::Regular, InstSameRegHint::None), // #257 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 371, 2, InstControlFlow::Regular, InstSameRegHint::None), // #258 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 565, 1, InstControlFlow::Regular, InstSameRegHint::None), // #259 [ref=5x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 455, 2, InstControlFlow::Regular, InstSameRegHint::None), // #260 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 457, 2, InstControlFlow::Regular, InstSameRegHint::None), // #261 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 459, 2, InstControlFlow::Regular, InstSameRegHint::None), // #262 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #263 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #264 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 375, 2, InstControlFlow::Regular, InstSameRegHint::None), // #265 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 375, 2, InstControlFlow::Regular, InstSameRegHint::None), // #266 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #267 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #268 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 453, 2, InstControlFlow::Regular, InstSameRegHint::None), // #269 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #270 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #271 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #272 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #273 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::SAE.bits(), 371, 2, InstControlFlow::Regular, InstSameRegHint::None), // #274 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::SAE.bits(), 371, 2, InstControlFlow::Regular, InstSameRegHint::None), // #275 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::SAE.bits(), 455, 2, InstControlFlow::Regular, InstSameRegHint::None), // #276 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::SAE.bits(), 375, 2, InstControlFlow::Regular, InstSameRegHint::None), // #277 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::SAE.bits(), 375, 2, InstControlFlow::Regular, InstSameRegHint::None), // #278 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::ER.bits() | Avx512Flags::SAE.bits(), 457, 2, InstControlFlow::Regular, InstSameRegHint::None), // #279 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #280 [ref=3x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 298, 1, InstControlFlow::Regular, InstSameRegHint::None), // #281 [ref=10x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #282 [ref=8x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 317, 1, InstControlFlow::Regular, InstSameRegHint::None), // #283 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 577, 1, InstControlFlow::Regular, InstSameRegHint::None), // #284 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 318, 1, InstControlFlow::Regular, InstSameRegHint::None), // #285 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 512, 1, InstControlFlow::Regular, InstSameRegHint::None), // #286 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #287 [ref=5x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #288 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #289 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 578, 1, InstControlFlow::Regular, InstSameRegHint::None), // #290 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 579, 1, InstControlFlow::Regular, InstSameRegHint::None), // #291 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 236, 4, InstControlFlow::Regular, InstSameRegHint::None), // #292 [ref=12x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 461, 2, InstControlFlow::Regular, InstSameRegHint::None), // #293 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 463, 2, InstControlFlow::Regular, InstSameRegHint::None), // #294 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 580, 1, InstControlFlow::Regular, InstSameRegHint::None), // #295 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 580, 1, InstControlFlow::Regular, InstSameRegHint::None), // #296 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 580, 1, InstControlFlow::Regular, InstSameRegHint::None), // #297 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 581, 1, InstControlFlow::Regular, InstSameRegHint::None), // #298 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 582, 1, InstControlFlow::Regular, InstSameRegHint::None), // #299 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 583, 1, InstControlFlow::Regular, InstSameRegHint::None), // #300 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 99, 2, InstControlFlow::Regular, InstSameRegHint::None), // #301 [ref=7x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 172, 1, InstControlFlow::Regular, InstSameRegHint::None), // #302 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 313, 1, InstControlFlow::Regular, InstSameRegHint::None), // #303 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_TWO_OP.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 240, 4, InstControlFlow::Regular, InstSameRegHint::None), // #304 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_TWO_OP.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 183, 5, InstControlFlow::Regular, InstSameRegHint::None), // #305 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_TWO_OP.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 188, 5, InstControlFlow::Regular, InstSameRegHint::None), // #306 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_TWO_OP.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 319, 3, InstControlFlow::Regular, InstSameRegHint::None), // #307 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 564, 1, InstControlFlow::Regular, InstSameRegHint::None), // #308 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #309 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #310 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #311 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #312 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 584, 1, InstControlFlow::Regular, InstSameRegHint::None), // #313 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #314 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #315 [ref=22x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 465, 1, InstControlFlow::Regular, InstSameRegHint::None), // #316 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 465, 2, InstControlFlow::Regular, InstSameRegHint::None), // #317 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 585, 1, InstControlFlow::Regular, InstSameRegHint::None), // #318 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 579, 1, InstControlFlow::Regular, InstSameRegHint::None), // #319 [ref=1x]
    CommonInfo::new(InstFlags::VEX.bits(), 0, 530, 1, InstControlFlow::Regular, InstSameRegHint::None), // #320 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 533, 1, InstControlFlow::Regular, InstSameRegHint::None), // #321 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 244, 4, InstControlFlow::Regular, InstSameRegHint::None), // #322 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #323 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #324 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #325 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 564, 1, InstControlFlow::Regular, InstSameRegHint::None), // #326 [ref=2x]
    CommonInfo::new(0, 0, 467, 2, InstControlFlow::Regular, InstSameRegHint::None), // #327 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 99, 8, InstControlFlow::Regular, InstSameRegHint::None), // #328 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 469, 2, InstControlFlow::Regular, InstSameRegHint::None), // #329 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 325, 3, InstControlFlow::Regular, InstSameRegHint::None), // #330 [ref=1x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 99, 4, InstControlFlow::Regular, InstSameRegHint::None), // #331 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 6, InstControlFlow::Regular, InstSameRegHint::None), // #332 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 109, 2, InstControlFlow::Regular, InstSameRegHint::None), // #333 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 248, 4, InstControlFlow::Regular, InstSameRegHint::None), // #334 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 586, 1, InstControlFlow::Regular, InstSameRegHint::None), // #335 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 193, 5, InstControlFlow::Regular, InstSameRegHint::None), // #336 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 198, 5, InstControlFlow::Regular, InstSameRegHint::None), // #337 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 203, 5, InstControlFlow::Regular, InstSameRegHint::None), // #338 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 107, 8, InstControlFlow::Regular, InstSameRegHint::None), // #339 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 252, 4, InstControlFlow::Regular, InstSameRegHint::None), // #340 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #341 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 115, 8, InstControlFlow::Regular, InstSameRegHint::None), // #342 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 471, 2, InstControlFlow::Regular, InstSameRegHint::None), // #343 [ref=1x]
    CommonInfo::new(0, 0, 473, 2, InstControlFlow::Regular, InstSameRegHint::None), // #344 [ref=1x]
    CommonInfo::new(0, 0, 475, 2, InstControlFlow::Regular, InstSameRegHint::None), // #345 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits(), 328, 3, InstControlFlow::Regular, InstSameRegHint::None), // #346 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits(), 328, 3, InstControlFlow::Regular, InstSameRegHint::None), // #347 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #348 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #349 [ref=5x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 295, 2, InstControlFlow::Regular, InstSameRegHint::RO), // #350 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::RO), // #351 [ref=2x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 295, 2, InstControlFlow::Regular, InstSameRegHint::WO), // #352 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #353 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #354 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::RO), // #355 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #356 [ref=13x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 587, 1, InstControlFlow::Regular, InstSameRegHint::None), // #357 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 588, 1, InstControlFlow::Regular, InstSameRegHint::None), // #358 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), 0, 589, 1, InstControlFlow::Regular, InstSameRegHint::None), // #359 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 477, 2, InstControlFlow::Regular, InstSameRegHint::None), // #360 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 590, 1, InstControlFlow::Regular, InstSameRegHint::None), // #361 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #362 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 256, 4, InstControlFlow::Regular, InstSameRegHint::None), // #363 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 304, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #364 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 304, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #365 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 331, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #366 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 331, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #367 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_K_REG.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 331, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #368 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 544, 1, InstControlFlow::Regular, InstSameRegHint::None), // #369 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 545, 1, InstControlFlow::Regular, InstSameRegHint::None), // #370 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 546, 1, InstControlFlow::Regular, InstSameRegHint::None), // #371 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 547, 1, InstControlFlow::Regular, InstSameRegHint::None), // #372 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 304, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #373 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #374 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::PREFER_EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #375 [ref=4x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 299, 1, InstControlFlow::Regular, InstSameRegHint::None), // #376 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 296, 2, InstControlFlow::Regular, InstSameRegHint::None), // #377 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 260, 4, InstControlFlow::Regular, InstSameRegHint::None), // #378 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 123, 8, InstControlFlow::Regular, InstSameRegHint::None), // #379 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 123, 8, InstControlFlow::Regular, InstSameRegHint::None), // #380 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 264, 4, InstControlFlow::Regular, InstSameRegHint::None), // #381 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 548, 1, InstControlFlow::Regular, InstSameRegHint::None), // #382 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 549, 1, InstControlFlow::Regular, InstSameRegHint::None), // #383 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 591, 1, InstControlFlow::Regular, InstSameRegHint::None), // #384 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 592, 1, InstControlFlow::Regular, InstSameRegHint::None), // #385 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 593, 1, InstControlFlow::Regular, InstSameRegHint::None), // #386 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 594, 1, InstControlFlow::Regular, InstSameRegHint::None), // #387 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 595, 1, InstControlFlow::Regular, InstSameRegHint::None), // #388 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 451, 1, InstControlFlow::Regular, InstSameRegHint::None), // #389 [ref=12x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::PREFER_EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #390 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::RO), // #391 [ref=8x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), 0, 596, 1, InstControlFlow::Regular, InstSameRegHint::None), // #392 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 334, 3, InstControlFlow::Regular, InstSameRegHint::None), // #393 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 337, 3, InstControlFlow::Regular, InstSameRegHint::None), // #394 [ref=9x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 340, 3, InstControlFlow::Regular, InstSameRegHint::None), // #395 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 313, 3, InstControlFlow::Regular, InstSameRegHint::None), // #396 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 343, 3, InstControlFlow::Regular, InstSameRegHint::None), // #397 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 310, 3, InstControlFlow::Regular, InstSameRegHint::None), // #398 [ref=6x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 256, 2, InstControlFlow::Regular, InstSameRegHint::None), // #399 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #400 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #401 [ref=3x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 479, 2, InstControlFlow::Regular, InstSameRegHint::None), // #402 [ref=4x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 346, 3, InstControlFlow::Regular, InstSameRegHint::None), // #403 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 481, 2, InstControlFlow::Regular, InstSameRegHint::None), // #404 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 483, 2, InstControlFlow::Regular, InstSameRegHint::None), // #405 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits() | InstFlags::VSIB.bits(), Avx512Flags::K.bits(), 349, 3, InstControlFlow::Regular, InstSameRegHint::None), // #406 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 485, 2, InstControlFlow::Regular, InstSameRegHint::None), // #407 [ref=8x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 352, 3, InstControlFlow::Regular, InstSameRegHint::None), // #408 [ref=5x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #409 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #410 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 157, 6, InstControlFlow::Regular, InstSameRegHint::None), // #411 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 322, 3, InstControlFlow::Regular, InstSameRegHint::None), // #412 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 157, 6, InstControlFlow::Regular, InstSameRegHint::None), // #413 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 157, 6, InstControlFlow::Regular, InstSameRegHint::None), // #414 [ref=3x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 157, 6, InstControlFlow::Regular, InstSameRegHint::None), // #415 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #416 [ref=6x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #417 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::WO), // #418 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 352, 3, InstControlFlow::Regular, InstSameRegHint::None), // #419 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::IMPLICIT_Z.bits() | Avx512Flags::K.bits(), 352, 3, InstControlFlow::Regular, InstSameRegHint::None), // #420 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 564, 1, InstControlFlow::Regular, InstSameRegHint::None), // #421 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #422 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B16.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #423 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 565, 1, InstControlFlow::Regular, InstSameRegHint::None), // #424 [ref=2x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 566, 1, InstControlFlow::Regular, InstSameRegHint::None), // #425 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 578, 1, InstControlFlow::Regular, InstSameRegHint::None), // #426 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 579, 1, InstControlFlow::Regular, InstSameRegHint::None), // #427 [ref=1x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 322, 2, InstControlFlow::Regular, InstSameRegHint::None), // #428 [ref=2x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 578, 1, InstControlFlow::Regular, InstSameRegHint::None), // #429 [ref=1x]
    CommonInfo::new(InstFlags::EVEX_TRANSFORMABLE.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 579, 1, InstControlFlow::Regular, InstSameRegHint::None), // #430 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 295, 3, InstControlFlow::Regular, InstSameRegHint::None), // #431 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 597, 1, InstControlFlow::Regular, InstSameRegHint::None), // #432 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 598, 1, InstControlFlow::Regular, InstSameRegHint::None), // #433 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 599, 1, InstControlFlow::Regular, InstSameRegHint::None), // #434 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 299, 2, InstControlFlow::Regular, InstSameRegHint::None), // #435 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::VEC.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 299, 2, InstControlFlow::Regular, InstSameRegHint::None), // #436 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #437 [ref=1x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B32.bits() | Avx512Flags::K.bits() | Avx512Flags::Z.bits(), 298, 3, InstControlFlow::Regular, InstSameRegHint::None), // #438 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 295, 1, InstControlFlow::Regular, InstSameRegHint::None), // #439 [ref=2x]
    CommonInfo::new(InstFlags::EVEX.bits() | InstFlags::EVEX_COMPAT.bits() | InstFlags::VEC.bits() | InstFlags::VEX.bits(), Avx512Flags::B64.bits() | Avx512Flags::ER.bits() | Avx512Flags::K.bits() | Avx512Flags::SAE.bits() | Avx512Flags::Z.bits(), 151, 3, InstControlFlow::Regular, InstSameRegHint::None), // #440 [ref=1x]
    CommonInfo::new(InstFlags::VEC.bits() | InstFlags::VEX.bits(), 0, 145, 1, InstControlFlow::Regular, InstSameRegHint::None), // #441 [ref=2x]
    CommonInfo::new(0, 0, 143, 1, InstControlFlow::Regular, InstSameRegHint::None), // #442 [ref=2x]
    CommonInfo::new(0, 0, 42, 1, InstControlFlow::Regular, InstSameRegHint::None), // #443 [ref=2x]
    CommonInfo::new(InstFlags::LOCK.bits() | InstFlags::X_ACQUIRE.bits() | InstFlags::X_RELEASE.bits(), 0, 20, 4, InstControlFlow::Regular, InstSameRegHint::None), // #444 [ref=1x]
    CommonInfo::new(0, 0, 269, 1, InstControlFlow::Regular, InstSameRegHint::None), // #445 [ref=1x]
    CommonInfo::new(InstFlags::X_ACQUIRE.bits(), 0, 131, 8, InstControlFlow::Regular, InstSameRegHint::RO), // #446 [ref=1x]
    CommonInfo::new(0, 0, 600, 1, InstControlFlow::Regular, InstSameRegHint::None), // #447 [ref=6x]
    CommonInfo::new(0, 0, 601, 1, InstControlFlow::Regular, InstSameRegHint::None), // #448 [ref=6x]
];

/// CPU feature requirements and RW flag indexes, indexed by [`InstInfo::additional_info_index`].
pub static ADDITIONAL_INFO_TABLE: &[AdditionalInfo] = &[
    AdditionalInfo::new(0, 0, [0, 0, 0, 0, 0, 0]), // #0 [ref=68x]
    AdditionalInfo::new(0, 1, [0, 0, 0, 0, 0, 0]), // #1 [ref=31x]
    AdditionalInfo::new(0, 0, [CpuFeature::RAO_INT as u8, 0, 0, 0, 0, 0]), // #2 [ref=4x]
    AdditionalInfo::new(0, 2, [0, 0, 0, 0, 0, 0]), // #3 [ref=2x]
    AdditionalInfo::new(0, 3, [CpuFeature::ADX as u8, 0, 0, 0, 0, 0]), // #4 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE2 as u8, 0, 0, 0, 0, 0]), // #5 [ref=60x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE as u8, 0, 0, 0, 0, 0]), // #6 [ref=46x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE3 as u8, 0, 0, 0, 0, 0]), // #7 [ref=10x]
    AdditionalInfo::new(0, 4, [CpuFeature::ADX as u8, 0, 0, 0, 0, 0]), // #8 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::AESNI as u8, 0, 0, 0, 0, 0]), // #9 [ref=6x]
    AdditionalInfo::new(0, 1, [CpuFeature::BMI as u8, 0, 0, 0, 0, 0]), // #10 [ref=6x]
    AdditionalInfo::new(0, 5, [0, 0, 0, 0, 0, 0]), // #11 [ref=5x]
    AdditionalInfo::new(0, 0, [CpuFeature::TBM as u8, 0, 0, 0, 0, 0]), // #12 [ref=9x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE4_1 as u8, 0, 0, 0, 0, 0]), // #13 [ref=47x]
    AdditionalInfo::new(0, 0, [CpuFeature::MPX as u8, 0, 0, 0, 0, 0]), // #14 [ref=7x]
    AdditionalInfo::new(0, 6, [0, 0, 0, 0, 0, 0]), // #15 [ref=4x]
    AdditionalInfo::new(0, 1, [CpuFeature::BMI2 as u8, 0, 0, 0, 0, 0]), // #16 [ref=1x]
    AdditionalInfo::new(0, 7, [CpuFeature::SMAP as u8, 0, 0, 0, 0, 0]), // #17 [ref=2x]
    AdditionalInfo::new(0, 8, [0, 0, 0, 0, 0, 0]), // #18 [ref=2x]
    AdditionalInfo::new(0, 9, [0, 0, 0, 0, 0, 0]), // #19 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::CLDEMOTE as u8, 0, 0, 0, 0, 0]), // #20 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::CLFLUSH as u8, 0, 0, 0, 0, 0]), // #21 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::CLFLUSHOPT as u8, 0, 0, 0, 0, 0]), // #22 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::SVM as u8, 0, 0, 0, 0, 0]), // #23 [ref=6x]
    AdditionalInfo::new(0, 10, [0, 0, 0, 0, 0, 0]), // #24 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::CET_SS as u8, 0, 0, 0, 0, 0]), // #25 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::UINTR as u8, 0, 0, 0, 0, 0]), // #26 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::CLWB as u8, 0, 0, 0, 0, 0]), // #27 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::CLZERO as u8, 0, 0, 0, 0, 0]), // #28 [ref=1x]
    AdditionalInfo::new(0, 3, [0, 0, 0, 0, 0, 0]), // #29 [ref=1x]
    AdditionalInfo::new(0, 11, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #30 [ref=2x]
    AdditionalInfo::new(0, 12, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #31 [ref=2x]
    AdditionalInfo::new(0, 13, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #32 [ref=2x]
    AdditionalInfo::new(0, 14, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #33 [ref=2x]
    AdditionalInfo::new(0, 15, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #34 [ref=2x]
    AdditionalInfo::new(0, 16, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #35 [ref=2x]
    AdditionalInfo::new(0, 17, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #36 [ref=2x]
    AdditionalInfo::new(0, 18, [CpuFeature::CMOV as u8, 0, 0, 0, 0, 0]), // #37 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::CMPCCXADD as u8, 0, 0, 0, 0, 0]), // #38 [ref=16x]
    AdditionalInfo::new(0, 19, [0, 0, 0, 0, 0, 0]), // #39 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::I486 as u8, 0, 0, 0, 0, 0]), // #40 [ref=2x]
    AdditionalInfo::new(0, 5, [CpuFeature::CMPXCHG16B as u8, 0, 0, 0, 0, 0]), // #41 [ref=1x]
    AdditionalInfo::new(0, 5, [CpuFeature::CMPXCHG8B as u8, 0, 0, 0, 0, 0]), // #42 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::SSE2 as u8, 0, 0, 0, 0, 0]), // #43 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::SSE as u8, 0, 0, 0, 0, 0]), // #44 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::I486 as u8, 0, 0, 0, 0, 0]), // #45 [ref=5x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE4_2 as u8, 0, 0, 0, 0, 0]), // #46 [ref=2x]
    AdditionalInfo::new(0, 20, [0, 0, 0, 0, 0, 0]), // #47 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::MMX as u8, 0, 0, 0, 0, 0]), // #48 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::CET_IBT as u8, 0, 0, 0, 0, 0]), // #49 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::ENQCMD as u8, 0, 0, 0, 0, 0]), // #50 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSE4A as u8, 0, 0, 0, 0, 0]), // #51 [ref=4x]
    AdditionalInfo::new(0, 21, [CpuFeature::FPU as u8, 0, 0, 0, 0, 0]), // #52 [ref=80x]
    AdditionalInfo::new(0, 22, [CpuFeature::CMOV as u8, CpuFeature::FPU as u8, 0, 0, 0, 0]), // #53 [ref=2x]
    AdditionalInfo::new(0, 23, [CpuFeature::CMOV as u8, CpuFeature::FPU as u8, 0, 0, 0, 0]), // #54 [ref=2x]
    AdditionalInfo::new(0, 24, [CpuFeature::CMOV as u8, CpuFeature::FPU as u8, 0, 0, 0, 0]), // #55 [ref=2x]
    AdditionalInfo::new(0, 25, [CpuFeature::CMOV as u8, CpuFeature::FPU as u8, 0, 0, 0, 0]), // #56 [ref=2x]
    AdditionalInfo::new(0, 26, [CpuFeature::FPU as u8, 0, 0, 0, 0, 0]), // #57 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::_3DNOW as u8, 0, 0, 0, 0, 0]), // #58 [ref=21x]
    AdditionalInfo::new(0, 21, [CpuFeature::SSE3 as u8, CpuFeature::FPU as u8, 0, 0, 0, 0]), // #59 [ref=1x]
    AdditionalInfo::new(0, 21, [CpuFeature::FXSR as u8, 0, 0, 0, 0, 0]), // #60 [ref=2x]
    AdditionalInfo::new(0, 27, [CpuFeature::FXSR as u8, 0, 0, 0, 0, 0]), // #61 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::SMX as u8, 0, 0, 0, 0, 0]), // #62 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::GFNI as u8, 0, 0, 0, 0, 0]), // #63 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::HRESET as u8, 0, 0, 0, 0, 0]), // #64 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::CET_SS as u8, 0, 0, 0, 0, 0]), // #65 [ref=9x]
    AdditionalInfo::new(0, 15, [0, 0, 0, 0, 0, 0]), // #66 [ref=5x]
    AdditionalInfo::new(0, 0, [CpuFeature::VMX as u8, 0, 0, 0, 0, 0]), // #67 [ref=13x]
    AdditionalInfo::new(0, 0, [CpuFeature::INVLPGB as u8, 0, 0, 0, 0, 0]), // #68 [ref=2x]
    AdditionalInfo::new(0, 11, [0, 0, 0, 0, 0, 0]), // #69 [ref=4x]
    AdditionalInfo::new(0, 12, [0, 0, 0, 0, 0, 0]), // #70 [ref=4x]
    AdditionalInfo::new(0, 13, [0, 0, 0, 0, 0, 0]), // #71 [ref=4x]
    AdditionalInfo::new(0, 14, [0, 0, 0, 0, 0, 0]), // #72 [ref=4x]
    AdditionalInfo::new(0, 16, [0, 0, 0, 0, 0, 0]), // #73 [ref=4x]
    AdditionalInfo::new(0, 17, [0, 0, 0, 0, 0, 0]), // #74 [ref=4x]
    AdditionalInfo::new(0, 18, [0, 0, 0, 0, 0, 0]), // #75 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_DQ as u8, 0, 0, 0, 0, 0]), // #76 [ref=10x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_BW as u8, 0, 0, 0, 0, 0]), // #77 [ref=20x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_F as u8, 0, 0, 0, 0, 0]), // #78 [ref=9x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX512_DQ as u8, 0, 0, 0, 0, 0]), // #79 [ref=1x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX512_BW as u8, 0, 0, 0, 0, 0]), // #80 [ref=2x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX512_F as u8, 0, 0, 0, 0, 0]), // #81 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX512_DQ as u8, 0, 0, 0, 0, 0]), // #82 [ref=3x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX512_BW as u8, 0, 0, 0, 0, 0]), // #83 [ref=4x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX512_F as u8, 0, 0, 0, 0, 0]), // #84 [ref=1x]
    AdditionalInfo::new(0, 28, [CpuFeature::LAHFSAHF as u8, 0, 0, 0, 0, 0]), // #85 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::AMX_TILE as u8, 0, 0, 0, 0, 0]), // #86 [ref=7x]
    AdditionalInfo::new(0, 0, [CpuFeature::LWP as u8, 0, 0, 0, 0, 0]), // #87 [ref=4x]
    AdditionalInfo::new(0, 29, [0, 0, 0, 0, 0, 0]), // #88 [ref=3x]
    AdditionalInfo::new(0, 1, [CpuFeature::LZCNT as u8, 0, 0, 0, 0, 0]), // #89 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MMX2 as u8, 0, 0, 0, 0, 0]), // #90 [ref=3x]
    AdditionalInfo::new(0, 1, [CpuFeature::MCOMMIT as u8, 0, 0, 0, 0, 0]), // #91 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MONITOR as u8, 0, 0, 0, 0, 0]), // #92 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::MONITORX as u8, 0, 0, 0, 0, 0]), // #93 [ref=2x]
    AdditionalInfo::new(1, 0, [0, 0, 0, 0, 0, 0]), // #94 [ref=1x]
    AdditionalInfo::new(1, 0, [CpuFeature::SSE2 as u8, 0, 0, 0, 0, 0]), // #95 [ref=5x]
    AdditionalInfo::new(1, 0, [CpuFeature::SSE as u8, 0, 0, 0, 0, 0]), // #96 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::MOVBE as u8, 0, 0, 0, 0, 0]), // #97 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MMX as u8, CpuFeature::SSE2 as u8, 0, 0, 0, 0]), // #98 [ref=45x]
    AdditionalInfo::new(0, 0, [CpuFeature::MOVDIR64B as u8, 0, 0, 0, 0, 0]), // #99 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MOVDIRI as u8, 0, 0, 0, 0, 0]), // #100 [ref=1x]
    AdditionalInfo::new(1, 0, [CpuFeature::MMX as u8, CpuFeature::SSE2 as u8, 0, 0, 0, 0]), // #101 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::BMI2 as u8, 0, 0, 0, 0, 0]), // #102 [ref=7x]
    AdditionalInfo::new(0, 0, [CpuFeature::SSSE3 as u8, 0, 0, 0, 0, 0]), // #103 [ref=16x]
    AdditionalInfo::new(0, 0, [CpuFeature::MMX2 as u8, CpuFeature::SSE2 as u8, 0, 0, 0, 0]), // #104 [ref=10x]
    AdditionalInfo::new(0, 0, [CpuFeature::PCLMULQDQ as u8, 0, 0, 0, 0, 0]), // #105 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::SSE4_2 as u8, 0, 0, 0, 0, 0]), // #106 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::PCONFIG as u8, 0, 0, 0, 0, 0]), // #107 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MMX2 as u8, CpuFeature::SSE2 as u8, CpuFeature::SSE4_1 as u8, 0, 0, 0]), // #108 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::_3DNOW2 as u8, 0, 0, 0, 0, 0]), // #109 [ref=5x]
    AdditionalInfo::new(0, 0, [CpuFeature::GEODE as u8, 0, 0, 0, 0, 0]), // #110 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::POPCNT as u8, 0, 0, 0, 0, 0]), // #111 [ref=1x]
    AdditionalInfo::new(0, 30, [0, 0, 0, 0, 0, 0]), // #112 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::PREFETCHI as u8, 0, 0, 0, 0, 0]), // #113 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::PREFETCHW as u8, 0, 0, 0, 0, 0]), // #114 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::PREFETCHWT1 as u8, 0, 0, 0, 0, 0]), // #115 [ref=1x]
    AdditionalInfo::new(0, 20, [CpuFeature::SEV_SNP as u8, 0, 0, 0, 0, 0]), // #116 [ref=3x]
    AdditionalInfo::new(0, 1, [CpuFeature::SSE4_1 as u8, 0, 0, 0, 0, 0]), // #117 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::PTWRITE as u8, 0, 0, 0, 0, 0]), // #118 [ref=1x]
    AdditionalInfo::new(0, 31, [0, 0, 0, 0, 0, 0]), // #119 [ref=3x]
    AdditionalInfo::new(0, 1, [CpuFeature::SEV_SNP as u8, 0, 0, 0, 0, 0]), // #120 [ref=1x]
    AdditionalInfo::new(0, 32, [0, 0, 0, 0, 0, 0]), // #121 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::FSGSBASE as u8, 0, 0, 0, 0, 0]), // #122 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::MSR as u8, CpuFeature::MSR_IMM as u8, 0, 0, 0, 0]), // #123 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::RDPID as u8, 0, 0, 0, 0, 0]), // #124 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::OSPKE as u8, 0, 0, 0, 0, 0]), // #125 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::RDPRU as u8, 0, 0, 0, 0, 0]), // #126 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::RDRAND as u8, 0, 0, 0, 0, 0]), // #127 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::RDSEED as u8, 0, 0, 0, 0, 0]), // #128 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::RDTSC as u8, 0, 0, 0, 0, 0]), // #129 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::RDTSCP as u8, 0, 0, 0, 0, 0]), // #130 [ref=1x]
    AdditionalInfo::new(0, 33, [0, 0, 0, 0, 0, 0]), // #131 [ref=2x]
    AdditionalInfo::new(0, 34, [CpuFeature::LAHFSAHF as u8, 0, 0, 0, 0, 0]), // #132 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::SEAM as u8, 0, 0, 0, 0, 0]), // #133 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::SERIALIZE as u8, 0, 0, 0, 0, 0]), // #134 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::SHA as u8, 0, 0, 0, 0, 0]), // #135 [ref=7x]
    AdditionalInfo::new(0, 0, [CpuFeature::SKINIT as u8, 0, 0, 0, 0, 0]), // #136 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AMX_COMPLEX as u8, 0, 0, 0, 0, 0]), // #137 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AMX_BF16 as u8, 0, 0, 0, 0, 0]), // #138 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::AMX_INT8 as u8, 0, 0, 0, 0, 0]), // #139 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::AMX_FP16 as u8, 0, 0, 0, 0, 0]), // #140 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::UINTR as u8, 0, 0, 0, 0, 0]), // #141 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::WAITPKG as u8, 0, 0, 0, 0, 0]), // #142 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::WAITPKG as u8, 0, 0, 0, 0, 0]), // #143 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #144 [ref=71x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_FP16 as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #145 [ref=104x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, 0, 0, 0, 0, 0]), // #146 [ref=35x]
    AdditionalInfo::new(0, 0, [CpuFeature::AESNI as u8, CpuFeature::VAES as u8, CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0]), // #147 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::AESNI as u8, CpuFeature::AVX as u8, 0, 0, 0, 0]), // #148 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #149 [ref=135x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX512_DQ as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #150 [ref=12x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_NE_CONVERT as u8, 0, 0, 0, 0, 0]), // #151 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_DQ as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #152 [ref=42x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX2 as u8, 0, 0, 0, 0, 0]), // #153 [ref=7x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX2 as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0]), // #154 [ref=39x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #155 [ref=4x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX512_FP16 as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #156 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_BF16 as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #157 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_NE_CONVERT as u8, CpuFeature::AVX512_BF16 as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #158 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::F16C as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #159 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_BW as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #160 [ref=24x]
    AdditionalInfo::new(0, 0, [CpuFeature::FMA as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #161 [ref=60x]
    AdditionalInfo::new(0, 0, [CpuFeature::FMA4 as u8, 0, 0, 0, 0, 0]), // #162 [ref=20x]
    AdditionalInfo::new(0, 0, [CpuFeature::XOP as u8, 0, 0, 0, 0, 0]), // #163 [ref=55x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX2 as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #164 [ref=19x]
    AdditionalInfo::new(0, 0, [CpuFeature::GFNI as u8, CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0]), // #165 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::SEV_ES as u8, 0, 0, 0, 0, 0]), // #166 [ref=1x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #167 [ref=7x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX as u8, 0, 0, 0, 0, 0]), // #168 [ref=2x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #169 [ref=4x]
    AdditionalInfo::new(1, 0, [CpuFeature::AVX512_BW as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #170 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX2 as u8, 0, 0, 0, 0]), // #171 [ref=17x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_VL as u8, CpuFeature::AVX512_VP2INTERSECT as u8, 0, 0, 0, 0]), // #172 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX2 as u8, CpuFeature::AVX512_BW as u8, CpuFeature::AVX512_VL as u8, 0, 0]), // #173 [ref=54x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX2 as u8, CpuFeature::AVX512_BW as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #174 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_CD as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #175 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::PCLMULQDQ as u8, CpuFeature::VPCLMULQDQ as u8, CpuFeature::AVX as u8, CpuFeature::AVX512_F as u8, CpuFeature::AVX512_VL as u8, 0]), // #176 [ref=1x]
    AdditionalInfo::new(0, 1, [CpuFeature::AVX as u8, 0, 0, 0, 0, 0]), // #177 [ref=7x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_VBMI2 as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #178 [ref=16x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_VNNI_INT8 as u8, 0, 0, 0, 0, 0]), // #179 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_VNNI as u8, CpuFeature::AVX512_VL as u8, CpuFeature::AVX512_VNNI as u8, 0, 0, 0]), // #180 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_VNNI_INT16 as u8, 0, 0, 0, 0, 0]), // #181 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_VBMI as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #182 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX as u8, CpuFeature::AVX512_BW as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #183 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX_IFMA as u8, CpuFeature::AVX512_IFMA as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0]), // #184 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_BITALG as u8, CpuFeature::AVX512_VL as u8, 0, 0, 0, 0]), // #185 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::AVX512_VL as u8, CpuFeature::AVX512_VPOPCNTDQ as u8, 0, 0, 0, 0]), // #186 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::SHA512 as u8, CpuFeature::AVX as u8, 0, 0, 0, 0]), // #187 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::SM3 as u8, CpuFeature::AVX as u8, 0, 0, 0, 0]), // #188 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::SM4 as u8, CpuFeature::AVX as u8, 0, 0, 0, 0]), // #189 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::WBNOINVD as u8, 0, 0, 0, 0, 0]), // #190 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::MSR as u8, 0, 0, 0, 0, 0]), // #191 [ref=1x]
    AdditionalInfo::new(0, 0, [CpuFeature::RTM as u8, 0, 0, 0, 0, 0]), // #192 [ref=3x]
    AdditionalInfo::new(0, 0, [CpuFeature::XSAVE as u8, 0, 0, 0, 0, 0]), // #193 [ref=6x]
    AdditionalInfo::new(0, 0, [CpuFeature::TSXLDTRK as u8, 0, 0, 0, 0, 0]), // #194 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::XSAVES as u8, 0, 0, 0, 0, 0]), // #195 [ref=4x]
    AdditionalInfo::new(0, 0, [CpuFeature::XSAVEC as u8, 0, 0, 0, 0, 0]), // #196 [ref=2x]
    AdditionalInfo::new(0, 0, [CpuFeature::XSAVEOPT as u8, 0, 0, 0, 0, 0]), // #197 [ref=2x]
    AdditionalInfo::new(0, 1, [CpuFeature::RTM as u8, 0, 0, 0, 0, 0]), // #198 [ref=1x]
];

/// CPU flags read/written, indexed by [`AdditionalInfo::rw_flags_index`].
pub static RW_FLAGS_INFO_TABLE: &[RwFlagsInfo] = &[
    RwFlagsInfo::new(0, 0), // #0 [ref=1352x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #1 [ref=103x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits(), CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #2 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits(), CpuRwFlags::X86_CF.bits()), // #3 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_OF.bits(), CpuRwFlags::X86_OF.bits()), // #4 [ref=1x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_ZF.bits()), // #5 [ref=7x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits()), // #6 [ref=4x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AC.bits()), // #7 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_CF.bits()), // #8 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_DF.bits()), // #9 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_IF.bits()), // #10 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits(), 0), // #11 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_ZF.bits(), 0), // #12 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_SF.bits(), 0), // #13 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits(), 0), // #14 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_OF.bits(), 0), // #15 [ref=7x]
    RwFlagsInfo::new(CpuRwFlags::X86_PF.bits(), 0), // #16 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_SF.bits(), 0), // #17 [ref=6x]
    RwFlagsInfo::new(CpuRwFlags::X86_ZF.bits(), 0), // #18 [ref=8x]
    RwFlagsInfo::new(CpuRwFlags::X86_DF.bits(), CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #19 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #20 [ref=5x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits()), // #21 [ref=83x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits(), CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits()), // #22 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_ZF.bits(), CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits()), // #23 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_ZF.bits(), CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits()), // #24 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_PF.bits(), CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits()), // #25 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_ZF.bits()), // #26 [ref=4x]
    RwFlagsInfo::new(CpuRwFlags::X86_C0.bits() | CpuRwFlags::X86_C1.bits() | CpuRwFlags::X86_C2.bits() | CpuRwFlags::X86_C3.bits(), 0), // #27 [ref=2x]
    RwFlagsInfo::new(CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits(), 0), // #28 [ref=1x]
    RwFlagsInfo::new(CpuRwFlags::X86_DF.bits(), 0), // #29 [ref=3x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_DF.bits() | CpuRwFlags::X86_IF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #30 [ref=3x]
    RwFlagsInfo::new(CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_DF.bits() | CpuRwFlags::X86_IF.bits() | CpuRwFlags::X86_OF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits(), 0), // #31 [ref=3x]
    RwFlagsInfo::new(CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits(), CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits()), // #32 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_OF.bits()), // #33 [ref=2x]
    RwFlagsInfo::new(0, CpuRwFlags::X86_AF.bits() | CpuRwFlags::X86_CF.bits() | CpuRwFlags::X86_PF.bits() | CpuRwFlags::X86_SF.bits() | CpuRwFlags::X86_ZF.bits()), // #34 [ref=1x]
];

/// Instruction RW flags, indexed by [`AdditionalInfo::inst_flags_index`].
pub static INST_FLAGS_TABLE: &[InstRwFlags] = &[
    InstRwFlags::NONE, // #0 [ref=1619x]
    InstRwFlags::MOV_OP, // #1 [ref=29x]
];

/// Maps the first letter of an instruction name to a span of [`InstId`] values.
pub static INST_NAME_INDEX: &[(u16, u16)] = &[
    (InstId::Aaa as u16, InstId::Axor as u16 + 1),
    (InstId::Bextr as u16, InstId::Bzhi as u16 + 1),
    (InstId::Call as u16, InstId::Cwde as u16 + 1),
    (InstId::Daa as u16, InstId::Dpps as u16 + 1),
    (InstId::Emms as u16, InstId::Extrq as u16 + 1),
    (InstId::F2xm1 as u16, InstId::Fyl2xp1 as u16 + 1),
    (InstId::Getsec as u16, InstId::Gf2p8mulb as u16 + 1),
    (InstId::Haddpd as u16, InstId::Hsubps as u16 + 1),
    (InstId::Idiv as u16, InstId::Iretq as u16 + 1),
    (InstId::Jb as u16, InstId::Jz as u16 + 1),
    (InstId::Kaddb as u16, InstId::Kxorw as u16 + 1),
    (InstId::Lahf as u16, InstId::Lzcnt as u16 + 1),
    (InstId::Maskmovdqu as u16, InstId::Mwaitx as u16 + 1),
    (InstId::Neg as u16, InstId::Not as u16 + 1),
    (InstId::Or as u16, InstId::Outs as u16 + 1),
    (InstId::Pabsb as u16, InstId::Pxor as u16 + 1),
    (InstId::None as u16, InstId::None as u16 + 1),
    (InstId::Rcl as u16, InstId::Rstorssp as u16 + 1),
    (InstId::Sahf as u16, InstId::Sysretq as u16 + 1),
    (InstId::T1mskc as u16, InstId::Tzmsk as u16 + 1),
    (InstId::Ucomisd as u16, InstId::Unpcklps as u16 + 1),
    (InstId::Vaddpd as u16, InstId::Vzeroupper as u16 + 1),
    (InstId::Wbinvd as u16, InstId::Wrussq as u16 + 1),
    (InstId::Xabort as u16, InstId::Xtest as u16 + 1),
    (InstId::None as u16, InstId::None as u16 + 1),
    (InstId::None as u16, InstId::None as u16 + 1),
];

pub const MAX_NAME_LENGTH: u16 = 17;

pub static INST_NAME_STRING_TABLE: &[u8] = b"cmovb\x0Ccmov.b|nae|ccmovbe\x0Acmov.be|nacmovl\x0Acmov.l|ngecmovle\x0Acmov.le|ngcmovnb\x0Dcmov.nb|ae|nccmovnbe\x0Acmov.nbe|acmovnl\x0Acmov.nl|gecmovnle\x0Acmov.nle|gcmovnp\x0Acmov.np|pocmovnz\x0Acmov.nz|necmovp\x09cmov.p|pecmovz\x08cmov.z|ejb\x0Ajb|jnae|jcjbe\x07jbe|jnajl\x07jl|jngejle\x07jle|jngjnb\x0Bjnb|jae|jncjnbe\x07jnbe|jajnl\x07jnl|jgejnle\x07jnle|jgjnp\x07jnp|jpojnz\x07jnz|jnejp\x06jp|jpejz\x05jz|jesetb\x0Bset.b|nae|csetbe\x09set.be|nasetl\x09set.l|ngesetle\x09set.le|ngsetnb\x0Cset.nb|ae|ncsetnbe\x09set.nbe|asetnl\x09set.nl|gesetnle\x09set.nle|gsetnp\x09set.np|posetnz\x09set.nz|nesetp\x08set.p|pesetz\x07set.z|evgf2p8affineinvqbvaeskeygenassisvbroadcastf32x464x264x4i32x2i32x4i32x8i64x2i64x4vpbroadcastmb2w2dvbcstnebf162p128i128vcvtne2ps2vcvtneebf16vcvtneobf16vfmaddsub132ph213pd213ph213ps231pd231ph231psvfmsubadd132vpmultishiftvcvtneps2vextracvextractfvp2intersecttcmmimfp16tcmmrlfp16sh2pssdph2psvfnmadd132213sd213sh213ss231sd231sh231ssvfnmsub132vinservinsertfvpshufbitqvsha512rndprefetchit0ntawt1saveprevsssha256rndtileloaddtilerelevaesdeclvaesenclvcompressvcvttpd2udqqvcvttph2uqqvcvttpsvcvttsd2uvcvttsh2uvcvttss2uvfixupimmvfmadd132vfmsub132vmaskmovdqvpcompressvpconflictvphminposuvpmadd52hluqvpscatterqdvpunpckhqlqdqvrndscalevscatterdqpdqpsmsg1msg2clflushopcmpnbexcmpnlexcmpxchg16t2tilestorevcvtpdvcvtph2psvcvtps2phvcvtsd2uvcvtsh2uvcvtss2u2dq2qqvcvtudq2vcvtuqq2vcvtusi2vfcmaddcvfpclassvgatherdvgetmanmulbvpclmuvpcmpestrvpcmpistrvperm2fvpermil2vpgathervpmacssdqvpmadcsswubswvpmaskmovpternlogbwwdlbwldqlwdvrsqrt14vshufvshuffvzeroupxsaveoptcmpbexcmplexcmpnbxcmpnlxcmpnoxcmpnpxcmpnsxcmpnzx8b2pifxrstorldtilecfmovdir64pvalidarmpadjurmpupdaserialisha1nexsha1rndssttilecftdpbf16tdpfp16vaddsubvblendmvpdvcvtdq2uwvcvtqq2vcvtsi2vcvtuwvdbpsadvdpbf16vexpandvfcmulccphcshvgetexpvmovdqau16u32u64vmovmskvmovntvmovshdvmovsldvpackssdwbvpackuswbvpblendmdvpdpbssudsvpdpbusvpdpwssvpdpwus2pdvpermtvpexpanvphaddubwqdqhvpmovmskvpmovsxbvpmovusqwvpmovzxbvpmulhrvptestnmqvreducevscalefvsm3rndvsm4rndsvunpckhlpdlpsxresldtrs64xsusldtrcldemoclrssbscmpbxcmplxcmpoxcmppxcmpsxcmpzxcvtpifxsavekortestwkshiftrbkunpckmonitorpfrcpipfrsqirtvrdfsbrdgsbsspseamcalsenduisetssbssysesysexumvcvtwvfmulvldmxcsvmlaundupu8vmovhvmovlhvmpsadvmresumvpadduvpaligngtbgtdgtqgtw2bbdbqvphsubvplzcnb2md2mq2mw2mvpopcnvpshldvqvpshrdvwhwvpsubuvrangevrcp14vroundvsm4keyvstmxcsvucomiallwbnoinwrfsbwrgsbc64blcfiblsfiendbrenqcmfcmovnufdecsfincsfnstefrndfsincfucomfyl2xincsspqinvlinvlpinvpcinvvpmcommmovqpavgupfcmpepfpnaptwriseamoseamrsyscsysretdpbutlbsyvaesivaligvandnvcomivfrczvhaddvhsubvmclevmgexvmmcvmovavmovuvmptvmwrivpandvpextrwvpinsvpmaxvpminvprolvprorvpsadvpsigvpslvpsllvpsravpsrlvsqrvtes";

#[rustfmt::skip]
pub static INST_NAME_INDEX_TABLE: &[u32] = &[
    0x80000000, // Small ''.
    0x80000421, // Small 'aaa'.
    0x80001021, // Small 'aad'.
    0x80021021, // Small 'aadd'.
    0x80003421, // Small 'aam'.
    0x80023821, // Small 'aand'.
    0x80004C21, // Small 'aas'.
    0x80000C81, // Small 'adc'.
    0x800C0C81, // Small 'adcx'.
    0x80001081, // Small 'add'.
    0x80481081, // Small 'addpd'.
    0x81381081, // Small 'addps'.
    0x80499081, // Small 'addsd'.
    0x81399081, // Small 'addss'.
    0x22AC629E, // Large 'addsub|pd'.
    0x2282629E, // Large 'addsub|ps'.
    0x800C3C81, // Small 'adox'.
    0x86524CA1, // Small 'aesdec'.
    0x322D73AE, // Large 'aesdecl|ast'.
    0x86E2CCA1, // Small 'aesenc'.
    0x322D73B6, // Large 'aesencl|ast'.
    0x86D4CCA1, // Small 'aesimc'.
    0x1154E218, // Large 'aeskeygenassis|t'.
    0x800011C1, // Small 'and'.
    0x800711C1, // Small 'andn'.
    0x890711C1, // Small 'andnpd'.
    0xA70711C1, // Small 'andnps'.
    0x804811C1, // Small 'andpd'.
    0x813811C1, // Small 'andps'.
    0x800049E1, // Small 'aor'.
    0x80064241, // Small 'arpl'.
    0x80093F01, // Small 'axor'.
    0x812A60A2, // Small 'bextr'.
    0x28BA58CF, // Large 'blcfi|ll'.
    0x80048D82, // Small 'blci'.
    0x80348D82, // Small 'blcic'.
    0x97368D82, // Small 'blcmsk'.
    0x80098D82, // Small 'blcs'.
    0x22AC563C, // Large 'blend|pd'.
    0x2282563C, // Large 'blend|ps'.
    0x3642563C, // Large 'blend|vpd'.
    0x3364563C, // Large 'blend|vps'.
    0x28BA58D4, // Large 'blsfi|ll'.
    0x8004CD82, // Small 'blsi'.
    0x8034CD82, // Small 'blsic'.
    0x9736CD82, // Small 'blsmsk'.
    0x80094D82, // Small 'blsr'.
    0x80C191C2, // Small 'bndcl'.
    0x80E191C2, // Small 'bndcn'.
    0x815191C2, // Small 'bndcu'.
    0xB04611C2, // Small 'bndldx'.
    0x80B691C2, // Small 'bndmk'.
    0xACF691C2, // Small 'bndmov'.
    0xB14991C2, // Small 'bndstx'.
    0x804755E2, // Small 'bound'.
    0x80001A62, // Small 'bsf'.
    0x80004A62, // Small 'bsr'.
    0x8100DE62, // Small 'bswap'.
    0x80000282, // Small 'bt'.
    0x80000E82, // Small 'btc'.
    0x80004A82, // Small 'btr'.
    0x80004E82, // Small 'bts'.
    0x8004A342, // Small 'bzhi'.
    0x80063023, // Small 'call'.
    0x80005C43, // Small 'cbw'.
    0x80004483, // Small 'cdq'.
    0x8002C483, // Small 'cdqe'.
    0x80018583, // Small 'clac'.
    0x80000D83, // Small 'clc'.
    0x80001183, // Small 'cld'.
    0x22FD677C, // Large 'cldemo|te'.
    0x00007486, // Large 'clflush'.
    0x11549486, // Large 'clflushop|t'.
    0x80049D83, // Small 'clgi'.
    0x80002583, // Small 'cli'.
    0x121D7782, // Large 'clrssbs|y'.
    0x8009D183, // Small 'clts'.
    0x8004D583, // Small 'clui'.
    0x80015D83, // Small 'clwb'.
    0x9F22E983, // Small 'clzero'.
    0x80000DA3, // Small 'cmc'.
    0x0FFF5000, // Large 'cmovb' + 'cmov.b|nae|c'
    0x0FFF6012, // Large 'cmovbe' + 'cmov.be|na'
    0x0FFF5023, // Large 'cmovl' + 'cmov.l|nge'
    0x0FFF6033, // Large 'cmovle' + 'cmov.le|ng'
    0x0FFF6044, // Large 'cmovnb' + 'cmov.nb|ae|nc'
    0x0FFF7058, // Large 'cmovnbe' + 'cmov.nbe|a'
    0x0FFF606A, // Large 'cmovnl' + 'cmov.nl|ge'
    0x0FFF707B, // Large 'cmovnle' + 'cmov.nle|g'
    0x9EEB3DA3, // Small 'cmovno'.
    0x0FFF608D, // Large 'cmovnp' + 'cmov.np|po'
    0xA6EB3DA3, // Small 'cmovns'.
    0x0FFF609E, // Large 'cmovnz' + 'cmov.nz|ne'
    0x80FB3DA3, // Small 'cmovo'.
    0x0FFF50AF, // Large 'cmovp' + 'cmov.p|pe'
    0x813B3DA3, // Small 'cmovs'.
    0x0FFF50BE, // Large 'cmovz' + 'cmov.z|e'
    0x800041A3, // Small 'cmp'.
    0x329E65A7, // Large 'cmpbex|add'.
    0x329E5789, // Large 'cmpbx|add'.
    0x329E65AD, // Large 'cmplex|add'.
    0x329E578E, // Large 'cmplx|add'.
    0x329E748F, // Large 'cmpnbex|add'.
    0x329E65B3, // Large 'cmpnbx|add'.
    0x329E7496, // Large 'cmpnlex|add'.
    0x329E65B9, // Large 'cmpnlx|add'.
    0x329E65BF, // Large 'cmpnox|add'.
    0x329E65C5, // Large 'cmpnpx|add'.
    0x329E65CB, // Large 'cmpnsx|add'.
    0x329E65D1, // Large 'cmpnzx|add'.
    0x329E5793, // Large 'cmpox|add'.
    0x804841A3, // Small 'cmppd'.
    0x813841A3, // Small 'cmpps'.
    0x329E5798, // Large 'cmppx|add'.
    0x8009C1A3, // Small 'cmps'.
    0x8049C1A3, // Small 'cmpsd'.
    0x8139C1A3, // Small 'cmpss'.
    0x329E579D, // Large 'cmpsx|add'.
    0x0000749D, // Large 'cmpxchg'.
    0x1004949D, // Large 'cmpxchg16|b'.
    0x25D7749D, // Large 'cmpxchg|8b'.
    0x329E57A2, // Large 'cmpzx|add'.
    0x8934B5E3, // Small 'comisd'.
    0xA734B5E3, // Small 'comiss'.
    0x8044D603, // Small 'cpuid'.
    0x80003E23, // Small 'cqo'.
    0x81DF0E43, // Small 'crc32'.
    0x22AC6646, // Large 'cvtdq2|pd'.
    0x22826646, // Large 'cvtdq2|ps'.
    0x34E154B2, // Large 'cvtpd|2dq'.
    0x35D954B2, // Large 'cvtpd|2pi'.
    0x328154B2, // Large 'cvtpd|2ps'.
    0x36F157A7, // Large 'cvtpi|2pd'.
    0x328157A7, // Large 'cvtpi|2ps'.
    0x23CF64C1, // Large 'cvtps2|dq'.
    0x122B74C1, // Large 'cvtps2p|d'.
    0x120F74C1, // Large 'cvtps2p|i'.
    0x222364CA, // Large 'cvtsd2|si'.
    0x222264CA, // Large 'cvtsd2|ss'.
    0x231D6656, // Large 'cvtsi2|sd'.
    0x22226656, // Large 'cvtsi2|ss'.
    0x231D64DA, // Large 'cvtss2|sd'.
    0x222364DA, // Large 'cvtss2|si'.
    0x23CF73C7, // Large 'cvttpd2|dq'.
    0x240473C7, // Large 'cvttpd2|pi'.
    0x34E163DE, // Large 'cvttps|2dq'.
    0x35D963DE, // Large 'cvttps|2pi'.
    0x222373E5, // Large 'cvttsd2|si'.
    0x222373F7, // Large 'cvttss2|si'.
    0x800012E3, // Small 'cwd'.
    0x800292E3, // Small 'cwde'.
    0x80000424, // Small 'daa'.
    0x80004C24, // Small 'das'.
    0x80000CA4, // Small 'dec'.
    0x80005924, // Small 'div'.
    0x80485924, // Small 'divpd'.
    0x81385924, // Small 'divps'.
    0x8049D924, // Small 'divsd'.
    0x8139D924, // Small 'divss'.
    0x80024204, // Small 'dppd'.
    0x8009C204, // Small 'dpps'.
    0x8009B5A5, // Small 'emms'.
    0x223158D9, // Large 'endbr|32'.
    0x223558D9, // Large 'endbr|64'.
    0x88D1C5C5, // Small 'enqcmd'.
    0x22A058DE, // Large 'enqcm|ds'.
    0x8122D1C5, // Small 'enter'.
    0x228272F0, // Large 'extract|ps'.
    0x81195305, // Small 'extrq'.
    0x81C6E3A6, // Small 'f2xm1'.
    0x80098826, // Small 'fabs'.
    0x80021026, // Small 'fadd'.
    0x81021026, // Small 'faddp'.
    0x80023046, // Small 'fbld'.
    0x810A4C46, // Small 'fbstp'.
    0x8009A066, // Small 'fchs'.
    0x8182B066, // Small 'fclex'.
    0x8567B466, // Small 'fcmovb'.
    0x40143500, // Large 'fcm|ovbe'.
    0x8B67B466, // Small 'fcmove'.
    0x40463500, // Large 'fcm|ovnb'.
    0x505A3500, // Large 'fcm|ovnbe'.
    0x20AD58E3, // Large 'fcmov|ne'.
    0x28E858E3, // Large 'fcmov|nu'.
    0xAB67B466, // Small 'fcmovu'.
    0x8006BC66, // Small 'fcom'.
    0x8096BC66, // Small 'fcomi'.
    0xA096BC66, // Small 'fcomip'.
    0x8106BC66, // Small 'fcomp'.
    0xA106BC66, // Small 'fcompp'.
    0x8009BC66, // Small 'fcos'.
    0x21EF58EA, // Large 'fdecs|tp'.
    0x800B2486, // Small 'fdiv'.
    0x810B2486, // Small 'fdivp'.
    0x812B2486, // Small 'fdivr'.
    0xA12B2486, // Small 'fdivrp'.
    0x8136B4A6, // Small 'femms'.
    0x8052C8C6, // Small 'ffree'.
    0x80420526, // Small 'fiadd'.
    0x80D78D26, // Small 'ficom'.
    0xA0D78D26, // Small 'ficomp'.
    0x81649126, // Small 'fidiv'.
    0xA5649126, // Small 'fidivr'.
    0x80023126, // Small 'fild'.
    0x80CAB526, // Small 'fimul'.
    0x21EF58EF, // Large 'fincs|tp'.
    0x8144B926, // Small 'finit'.
    0x800A4D26, // Small 'fist'.
    0x810A4D26, // Small 'fistp'.
    0xA14A4D26, // Small 'fisttp'.
    0x802ACD26, // Small 'fisub'.
    0xA42ACD26, // Small 'fisubr'.
    0x80001186, // Small 'fld'.
    0x800E1186, // Small 'fld1'.
    0x81719186, // Small 'fldcw'.
    0xACE29186, // Small 'fldenv'.
    0x8BD61186, // Small 'fldl2e'.
    0xA9D61186, // Small 'fldl2t'.
    0xBA761186, // Small 'fldlg2'.
    0xBAE61186, // Small 'fldln2'.
    0x80981186, // Small 'fldpi'.
    0x800D1186, // Small 'fldz'.
    0x800655A6, // Small 'fmul'.
    0x810655A6, // Small 'fmulp'.
    0xB0560DC6, // Small 'fnclex'.
    0xA89725C6, // Small 'fninit'.
    0x80083DC6, // Small 'fnop'.
    0x8B60CDC6, // Small 'fnsave'.
    0xAE3A4DC6, // Small 'fnstcw'.
    0x221358F4, // Large 'fnste|nv'.
    0xAF3A4DC6, // Small 'fnstsw'.
    0x9C1A0606, // Small 'fpatan'.
    0x80D2CA06, // Small 'fprem'.
    0xB8D2CA06, // Small 'fprem1'.
    0x80E0D206, // Small 'fptan'.
    0x32FB48F9, // Large 'frnd|int'.
    0xA4FA4E46, // Small 'frstor'.
    0x805B0666, // Small 'fsave'.
    0x8AC08E66, // Small 'fscale'.
    0x80072666, // Small 'fsin'.
    0x21DD58FD, // Large 'fsinc|os'.
    0x81494666, // Small 'fsqrt'.
    0x80005266, // Small 'fst'.
    0x8171D266, // Small 'fstcw'.
    0xACE2D266, // Small 'fstenv'.
    0x80085266, // Small 'fstp'.
    0x8179D266, // Small 'fstsw'.
    0x80015666, // Small 'fsub'.
    0x81015666, // Small 'fsubp'.
    0x81215666, // Small 'fsubr'.
    0xA1215666, // Small 'fsubrp'.
    0x800A4E86, // Small 'ftst'.
    0x80D78EA6, // Small 'fucom'.
    0x92D78EA6, // Small 'fucomi'.
    0x27D45902, // Large 'fucom|ip'.
    0xA0D78EA6, // Small 'fucomp'.
    0x279A5902, // Large 'fucom|pp'.
    0x814486E6, // Small 'fwait'.
    0x80068706, // Small 'fxam'.
    0x80040F06, // Small 'fxch'.
    0x000075DC, // Large 'fxrstor'.
    0x223575DC, // Large 'fxrstor|64'.
    0x8B60CF06, // Small 'fxsave'.
    0x223567AC, // Large 'fxsave|64'.
    0x52F225DC, // Large 'fx|tract'.
    0x818EB326, // Small 'fyl2x'.
    0x22735907, // Large 'fyl2x|p1'.
    0x8659D0A7, // Small 'getsec'.
    0x1004F207, // Large 'gf2p8affineinvq|b'.
    0x2215B207, // Large 'gf2p8affine|qb'.
    0x451E5207, // Large 'gf2p8|mulb'.
    0x89021028, // Small 'haddpd'.
    0xA7021028, // Small 'haddps'.
    0x80005188, // Small 'hlt'.
    0xA8599648, // Small 'hreset'.
    0x89015668, // Small 'hsubpd'.
    0xA7015668, // Small 'hsubps'.
    0x800B2489, // Small 'idiv'.
    0x800655A9, // Small 'imul'.
    0x800001C9, // Small 'in'.
    0x80000DC9, // Small 'inc'.
    0x22AC590C, // Large 'incss|pd'.
    0x2911590C, // Large 'incss|pq'.
    0x80004DC9, // Small 'ins'.
    0x2282635D, // Large 'insert|ps'.
    0x1215635D, // Large 'insert|q'.
    0x800051C9, // Small 'int'.
    0x800F51C9, // Small 'int3'.
    0x8007D1C9, // Small 'into'.
    0x800259C9, // Small 'invd'.
    0xA902D9C9, // Small 'invept'.
    0x8F0659C9, // Small 'invlpg'.
    0x354A4913, // Large 'invl|pga'.
    0x25775917, // Large 'invlp|gb'.
    0x25F7591C, // Large 'invpc|id'.
    0x25F75921, // Large 'invvp|id'.
    0x800A1649, // Small 'iret'.
    0x804A1649, // Small 'iretd'.
    0x811A1649, // Small 'iretq'.
    0x0FFF20CC, // Large 'jb' + 'jb|jnae|jc'
    0x0FFF30D9, // Large 'jbe' + 'jbe|jna'
    0x81AC0CAA, // Small 'jecxz'.
    0x0FFF20E4, // Large 'jl' + 'jl|jnge'
    0x0FFF30EE, // Large 'jle' + 'jle|jng'
    0x800041AA, // Small 'jmp'.
    0x0FFF30F9, // Large 'jnb' + 'jnb|jae|jnc'
    0x0FFF4108, // Large 'jnbe' + 'jnbe|ja'
    0x0FFF3114, // Large 'jnl' + 'jnl|jge'
    0x0FFF411F, // Large 'jnle' + 'jnle|jg'
    0x80003DCA, // Small 'jno'.
    0x0FFF312B, // Large 'jnp' + 'jnp|jpo'
    0x80004DCA, // Small 'jns'.
    0x0FFF3136, // Large 'jnz' + 'jnz|jne'
    0x800001EA, // Small 'jo'.
    0x0FFF2141, // Large 'jp' + 'jp|jpe'
    0x8000026A, // Small 'js'.
    0x0FFF214A, // Large 'jz' + 'jz|je'
    0x8022102B, // Small 'kaddb'.
    0x8042102B, // Small 'kaddd'.
    0x8112102B, // Small 'kaddq'.
    0x8172102B, // Small 'kaddw'.
    0x8022382B, // Small 'kandb'.
    0x8042382B, // Small 'kandd'.
    0x84E2382B, // Small 'kandnb'.
    0x88E2382B, // Small 'kandnd'.
    0xA2E2382B, // Small 'kandnq'.
    0xAEE2382B, // Small 'kandnw'.
    0x8112382B, // Small 'kandq'.
    0x8172382B, // Small 'kandw'.
    0x802B3DAB, // Small 'kmovb'.
    0x804B3DAB, // Small 'kmovd'.
    0x811B3DAB, // Small 'kmovq'.
    0x817B3DAB, // Small 'kmovw'.
    0x802A3DCB, // Small 'knotb'.
    0x804A3DCB, // Small 'knotd'.
    0x811A3DCB, // Small 'knotq'.
    0x817A3DCB, // Small 'knotw'.
    0x800149EB, // Small 'korb'.
    0x800249EB, // Small 'kord'.
    0x8008C9EB, // Small 'korq'.
    0x215467B2, // Large 'kortes|tb'.
    0x262667B2, // Large 'kortes|td'.
    0x236C67B2, // Large 'kortes|tq'.
    0x27B867B2, // Large 'kortes|tw'.
    0x800BC9EB, // Small 'korw'.
    0x252067BA, // Large 'kshift|lb'.
    0x257F67BA, // Large 'kshift|ld'.
    0x246267BA, // Large 'kshift|lq'.
    0x258267BA, // Large 'kshift|lw'.
    0x27C067BA, // Large 'kshift|rb'.
    0x122B77BA, // Large 'kshiftr|d'.
    0x121577BA, // Large 'kshiftr|q'.
    0x126477BA, // Large 'kshiftr|w'.
    0x8549968B, // Small 'ktestb'.
    0x8949968B, // Small 'ktestd'.
    0xA349968B, // Small 'ktestq'.
    0xAF49968B, // Small 'ktestw'.
    0x257867C2, // Large 'kunpck|bw'.
    0x23CF67C2, // Large 'kunpck|dq'.
    0x257A67C2, // Large 'kunpck|wd'.
    0x8527BB0B, // Small 'kxnorb'.
    0x8927BB0B, // Small 'kxnord'.
    0xA327BB0B, // Small 'kxnorq'.
    0xAF27BB0B, // Small 'kxnorw'.
    0x80293F0B, // Small 'kxorb'.
    0x80493F0B, // Small 'kxord'.
    0x81193F0B, // Small 'kxorq'.
    0x81793F0B, // Small 'kxorw'.
    0x8003202C, // Small 'lahf'.
    0x8000482C, // Small 'lar'.
    0x80C6046C, // Small 'lcall'.
    0x8158908C, // Small 'lddqu'.
    0x12286815, // Large 'ldmxcs|r'.
    0x80004C8C, // Small 'lds'.
    0x103185E3, // Large 'ldtilecf|g'.
    0x800004AC, // Small 'lea'.
    0x805B04AC, // Small 'leave'.
    0x80004CAC, // Small 'les'.
    0x8A3714CC, // Small 'lfence'.
    0x80004CCC, // Small 'lfs'.
    0x800A10EC, // Small 'lgdt'.
    0x80004CEC, // Small 'lgs'.
    0x800A112C, // Small 'lidt'.
    0x8008354C, // Small 'ljmp'.
    0x800A118C, // Small 'lldt'.
    0x84385D8C, // Small 'llwpcb'.
    0x800BCDAC, // Small 'lmsw'.
    0x800991EC, // Small 'lods'.
    0x80083DEC, // Small 'loop'.
    0x80583DEC, // Small 'loope'.
    0x8AE83DEC, // Small 'loopne'.
    0x8000326C, // Small 'lsl'.
    0x80004E6C, // Small 'lss'.
    0x80004A8C, // Small 'ltr'.
    0xA6E4C2EC, // Small 'lwpins'.
    0x981B42EC, // Small 'lwpval'.
    0x81470F4C, // Small 'lzcnt'.
    0x12A2941B, // Large 'maskmovdq|u'.
    0x1215741B, // Large 'maskmov|q'.
    0x8048602D, // Small 'maxpd'.
    0x8138602D, // Small 'maxps'.
    0x8049E02D, // Small 'maxsd'.
    0x8139E02D, // Small 'maxss'.
    0x236B5926, // Large 'mcomm|it'.
    0x8A3714CD, // Small 'mfence'.
    0x8048392D, // Small 'minpd'.
    0x8138392D, // Small 'minps'.
    0x8049B92D, // Small 'minsd'.
    0x8139B92D, // Small 'minss'.
    0x000077C8, // Large 'monitor'.
    0x123377C8, // Large 'monitor|x'.
    0x800059ED, // Small 'mov'.
    0xA620D9ED, // Small 'movabs'.
    0x8900D9ED, // Small 'movapd'.
    0xA700D9ED, // Small 'movaps'.
    0x805159ED, // Small 'movbe'.
    0x800259ED, // Small 'movd'.
    0x3821441F, // Large 'movd|dup'.
    0x100485EB, // Large 'movdir64|b'.
    0x120F65EB, // Large 'movdir|i'.
    0x24E4541F, // Large 'movdq|2q'.
    0x831259ED, // Small 'movdqa'.
    0xAB1259ED, // Small 'movdqu'.
    0x37664827, // Large 'movh|lps'.
    0x890459ED, // Small 'movhpd'.
    0xA70459ED, // Small 'movhps'.
    0x2282582C, // Large 'movlh|ps'.
    0x890659ED, // Small 'movlpd'.
    0xA70659ED, // Small 'movlps'.
    0x22AC669C, // Large 'movmsk|pd'.
    0x2282669C, // Large 'movmsk|ps'.
    0x23CF56A3, // Large 'movnt|dq'.
    0x368F56A3, // Large 'movnt|dqa'.
    0x934759ED, // Small 'movnti'.
    0x22AC56A3, // Large 'movnt|pd'.
    0x228256A3, // Large 'movnt|ps'.
    0xA34759ED, // Small 'movntq'.
    0x231D56A3, // Large 'movnt|sd'.
    0x222256A3, // Large 'movnt|ss'.
    0x8008D9ED, // Small 'movq'.
    0x34E1492B, // Large 'movq|2dq'.
    0x8009D9ED, // Small 'movs'.
    0x8049D9ED, // Small 'movsd'.
    0x240366A9, // Large 'movshd|up'.
    0x240366B0, // Large 'movsld|up'.
    0x8139D9ED, // Small 'movss'.
    0x8189D9ED, // Small 'movsx'.
    0x8989D9ED, // Small 'movsxd'.
    0x890AD9ED, // Small 'movupd'.
    0xA70AD9ED, // Small 'movups'.
    0x818D59ED, // Small 'movzx'.
    0x25785832, // Large 'mpsad|bw'.
    0x800032AD, // Small 'mul'.
    0x804832AD, // Small 'mulpd'.
    0x813832AD, // Small 'mulps'.
    0x8049B2AD, // Small 'mulsd'.
    0x8139B2AD, // Small 'mulss'.
    0x800C32AD, // Small 'mulx'.
    0x814486ED, // Small 'mwait'.
    0xB14486ED, // Small 'mwaitx'.
    0x80001CAE, // Small 'neg'.
    0x800041EE, // Small 'nop'.
    0x800051EE, // Small 'not'.
    0x8000024F, // Small 'or'.
    0x8002424F, // Small 'orpd'.
    0x8009C24F, // Small 'orps'.
    0x800052AF, // Small 'out'.
    0x8009D2AF, // Small 'outs'.
    0x80298830, // Small 'pabsb'.
    0x80498830, // Small 'pabsd'.
    0x81798830, // Small 'pabsw'.
    0x000086B7, // Large 'packssdw'.
    0x26BE66B7, // Large 'packss|wb'.
    0x26BD66C1, // Large 'packus|dw'.
    0x000086C1, // Large 'packuswb'.
    0x80221030, // Small 'paddb'.
    0x80421030, // Small 'paddd'.
    0x81121030, // Small 'paddq'.
    0x85321030, // Small 'paddsb'.
    0xAF321030, // Small 'paddsw'.
    0x2786583F, // Large 'paddu|sb'.
    0x2561583F, // Large 'paddu|sw'.
    0x81721030, // Small 'paddw'.
    0x12286845, // Large 'palign|r'.
    0x80023830, // Small 'pand'.
    0x80E23830, // Small 'pandn'.
    0x8059D430, // Small 'pause'.
    0x8023D830, // Small 'pavgb'.
    0x2786592F, // Large 'pavgu|sb'.
    0x8173D830, // Small 'pavgw'.
    0x200366CA, // Large 'pblend|vb'.
    0x126466CA, // Large 'pblend|w'.
    0x44625523, // Large 'pclmu|lqdq'.
    0x22155529, // Large 'pcmpe|qb'.
    0x24575529, // Large 'pcmpe|qd'.
    0x23D05529, // Large 'pcmpe|qq'.
    0x27255529, // Large 'pcmpe|qw'.
    0x120F8529, // Large 'pcmpestr|i'.
    0x10018529, // Large 'pcmpestr|m'.
    0x384B448E, // Large 'pcmp|gtb'.
    0x384E448E, // Large 'pcmp|gtd'.
    0x3851448E, // Large 'pcmp|gtq'.
    0x3854448E, // Large 'pcmp|gtw'.
    0x120F8532, // Large 'pcmpistr|i'.
    0x10018532, // Large 'pcmpistr|m'.
    0x2848542F, // Large 'pconf|ig'.
    0x80081490, // Small 'pdep'.
    0x800A60B0, // Small 'pext'.
    0x852A60B0, // Small 'pextrb'.
    0x892A60B0, // Small 'pextrd'.
    0xA32A60B0, // Small 'pextrq'.
    0xAF2A60B0, // Small 'pextrw'.
    0x8044F4D0, // Small 'pf2id'.
    0x8174F4D0, // Small 'pf2iw'.
    0x803184D0, // Small 'pfacc'.
    0x804204D0, // Small 'pfadd'.
    0x12156934, // Large 'pfcmpe|q'.
    0x20315934, // Large 'pfcmp|ge'.
    0x284B5934, // Large 'pfcmp|gt'.
    0x8180B4D0, // Small 'pfmax'.
    0x80E4B4D0, // Small 'pfmin'.
    0x80CAB4D0, // Small 'pfmul'.
    0x8630B8D0, // Small 'pfnacc'.
    0x2011593A, // Large 'pfpna|cc'.
    0x8101C8D0, // Small 'pfrcp'.
    0x238767CF, // Large 'pfrcpi|t1'.
    0x24A667CF, // Large 'pfrcpi|t2'.
    0xAD01C8D0, // Small 'pfrcpv'.
    0x238767D5, // Large 'pfrsqi|t1'.
    0x236157D5, // Large 'pfrsq|rt'.
    0x37DB57D5, // Large 'pfrsq|rtv'.
    0x802ACCD0, // Small 'pfsub'.
    0xA42ACCD0, // Small 'pfsubr'.
    0x88420510, // Small 'phaddd'.
    0x25615702, // Large 'phadd|sw'.
    0xAE420510, // Small 'phaddw'.
    0x12649439, // Large 'phminposu|w'.
    0x882ACD10, // Small 'phsubd'.
    0x2561585E, // Large 'phsub|sw'.
    0xAE2ACD10, // Small 'phsubw'.
    0x80437530, // Small 'pi2fd'.
    0x81737530, // Small 'pi2fw'.
    0x8529B930, // Small 'pinsrb'.
    0x8929B930, // Small 'pinsrd'.
    0xA329B930, // Small 'pinsrq'.
    0xAF29B930, // Small 'pinsrw'.
    0x45635443, // Large 'pmadd|ubsw'.
    0x257A5443, // Large 'pmadd|wd'.
    0x853C05B0, // Small 'pmaxsb'.
    0x893C05B0, // Small 'pmaxsd'.
    0xAF3C05B0, // Small 'pmaxsw'.
    0x855C05B0, // Small 'pmaxub'.
    0x895C05B0, // Small 'pmaxud'.
    0xAF5C05B0, // Small 'pmaxuw'.
    0x853725B0, // Small 'pminsb'.
    0x893725B0, // Small 'pminsd'.
    0xAF3725B0, // Small 'pminsw'.
    0x855725B0, // Small 'pminub'.
    0x895725B0, // Small 'pminud'.
    0xAF5725B0, // Small 'pminuw'.
    0x1004770F, // Large 'pmovmsk|b'.
    0x122B7717, // Large 'pmovsxb|d'.
    0x12157717, // Large 'pmovsxb|q'.
    0x12647717, // Large 'pmovsxb|w'.
    0x23CF6717, // Large 'pmovsx|dq'.
    0x257A6717, // Large 'pmovsx|wd'.
    0x27096717, // Large 'pmovsx|wq'.
    0x122B7728, // Large 'pmovzxb|d'.
    0x12157728, // Large 'pmovzxb|q'.
    0x12647728, // Large 'pmovzxb|w'.
    0x23CF6728, // Large 'pmovzx|dq'.
    0x257A6728, // Large 'pmovzx|wd'.
    0x27096728, // Large 'pmovzx|wq'.
    0xA24655B0, // Small 'pmuldq'.
    0x25616730, // Large 'pmulhr|sw'.
    0x12646730, // Large 'pmulhr|w'.
    0x264C5730, // Large 'pmulh|uw'.
    0xAE8655B0, // Small 'pmulhw'.
    0x88C655B0, // Small 'pmulld'.
    0xAEC655B0, // Small 'pmullw'.
    0x33CE42D4, // Large 'pmul|udq'.
    0x800041F0, // Small 'pop'.
    0x8000C1F0, // Small 'popa'.
    0x8040C1F0, // Small 'popad'.
    0xA8E1C1F0, // Small 'popcnt'.
    0x800341F0, // Small 'popf'.
    0x804341F0, // Small 'popfd'.
    0x811341F0, // Small 'popfq'.
    0x800049F0, // Small 'por'.
    0x00008378, // Large 'prefetch'.
    0x0000B378, // Large 'prefetchit0'.
    0x1270A378, // Large 'prefetchit|1'.
    0x33838378, // Large 'prefetch|nta'.
    0x23818378, // Large 'prefetch|t0'.
    0x23878378, // Large 'prefetch|t1'.
    0x24A68378, // Large 'prefetch|t2'.
    0x12648378, // Large 'prefetch|w'.
    0x33868378, // Large 'prefetch|wt1'.
    0xAE220670, // Small 'psadbw'.
    0x846AA270, // Small 'pshufb'.
    0x886AA270, // Small 'pshufd'.
    0x288B5365, // Large 'pshuf|hw'.
    0x25825365, // Large 'pshuf|lw'.
    0xAE6AA270, // Small 'pshufw'.
    0x84E3A670, // Small 'psignb'.
    0x88E3A670, // Small 'psignd'.
    0xAEE3A670, // Small 'psignw'.
    0x80463270, // Small 'pslld'.
    0xA2463270, // Small 'pslldq'.
    0x81163270, // Small 'psllq'.
    0x81763270, // Small 'psllw'.
    0x9130B670, // Small 'psmash'.
    0x8040CA70, // Small 'psrad'.
    0x8170CA70, // Small 'psraw'.
    0x80464A70, // Small 'psrld'.
    0xA2464A70, // Small 'psrldq'.
    0x81164A70, // Small 'psrlq'.
    0x81764A70, // Small 'psrlw'.
    0x80215670, // Small 'psubb'.
    0x80415670, // Small 'psubd'.
    0x81115670, // Small 'psubq'.
    0x85315670, // Small 'psubsb'.
    0xAF315670, // Small 'psubsw'.
    0x2786588E, // Large 'psubu|sb'.
    0x2561588E, // Large 'psubu|sw'.
    0x81715670, // Small 'psubw'.
    0x8900DE70, // Small 'pswapd'.
    0x81499690, // Small 'ptest'.
    0x22FD593F, // Large 'ptwri|te'.
    0x2578745A, // Large 'punpckh|bw'.
    0x23CF745A, // Large 'punpckh|dq'.
    0x23CF845A, // Large 'punpckhq|dq'.
    0x257A745A, // Large 'punpckh|wd'.
    0x357C645A, // Large 'punpck|lbw'.
    0x357F645A, // Large 'punpck|ldq'.
    0x4462645A, // Large 'punpck|lqdq'.
    0x3582645A, // Large 'punpck|lwd'.
    0x80044EB0, // Small 'push'.
    0x80144EB0, // Small 'pusha'.
    0x88144EB0, // Small 'pushad'.
    0x80644EB0, // Small 'pushf'.
    0x88644EB0, // Small 'pushfd'.
    0xA2644EB0, // Small 'pushfq'.
    0x81744EB0, // Small 'pushw'.
    0x22FD75F3, // Large 'pvalida|te'.
    0x80093F10, // Small 'pxor'.
    0x80003072, // Small 'rcl'.
    0x81384072, // Small 'rcpps'.
    0x8139C072, // Small 'rcpss'.
    0x80004872, // Small 'rcr'.
    0x317057DE, // Large 'rdfsb|ase'.
    0x317057E3, // Large 'rdgsb|ase'.
    0x8129B492, // Small 'rdmsr'.
    0x8044C092, // Small 'rdpid'.
    0xAB25C092, // Small 'rdpkru'.
    0x8036C092, // Small 'rdpmc'.
    0x81594092, // Small 'rdpru'.
    0x88E0C892, // Small 'rdrand'.
    0x8852CC92, // Small 'rdseed'.
    0x8909CC92, // Small 'rdsspd'.
    0xA309CC92, // Small 'rdsspq'.
    0x8039D092, // Small 'rdtsc'.
    0xA039D092, // Small 'rdtscp'.
    0x800050B2, // Small 'ret'.
    0x800350B2, // Small 'retf'.
    0x222E75FA, // Large 'rmpadju|st'.
    0x22FD7601, // Large 'rmpupda|te'.
    0x800031F2, // Small 'rol'.
    0x800049F2, // Small 'ror'.
    0x800C49F2, // Small 'rorx'.
    0x22AC58A0, // Large 'round|pd'.
    0x228258A0, // Large 'round|ps'.
    0x231D58A0, // Large 'round|sd'.
    0x222258A0, // Large 'round|ss'.
    0x80003672, // Small 'rsm'.
    0x22825586, // Large 'rsqrt|ps'.
    0x22225586, // Large 'rsqrt|ss'.
    0x37E855DE, // Large 'rstor|ssp'.
    0x80032033, // Small 'sahf'.
    0x80004833, // Small 'sar'.
    0x800C4833, // Small 'sarx'.
    0x1092A389, // Large 'saveprevss|p'.
    0x80000853, // Small 'sbb'.
    0x80098473, // Small 'scas'.
    0x102777EB, // Large 'seamcal|l'.
    0x22825944, // Large 'seamo|ps'.
    0x21535949, // Large 'seamr|et'.
    0x240467F2, // Large 'sendui|pi'.
    0x25997608, // Large 'seriali|ze'.
    0x0FFF4152, // Large 'setb' + 'set.b|nae|c'
    0x0FFF5162, // Large 'setbe' + 'set.be|na'
    0x0FFF4171, // Large 'setl' + 'set.l|nge'
    0x0FFF517F, // Large 'setle' + 'set.le|ng'
    0x0FFF518E, // Large 'setnb' + 'set.nb|ae|nc'
    0x0FFF61A0, // Large 'setnbe' + 'set.nbe|a'
    0x0FFF51B0, // Large 'setnl' + 'set.nl|ge'
    0x0FFF61BF, // Large 'setnle' + 'set.nle|g'
    0x80F750B3, // Small 'setno'.
    0x0FFF51CF, // Large 'setnp' + 'set.np|po'
    0x813750B3, // Small 'setns'.
    0x0FFF51DE, // Large 'setnz' + 'set.nz|ne'
    0x8007D0B3, // Small 'seto'.
    0x0FFF41ED, // Large 'setp' + 'set.p|pe'
    0x8009D0B3, // Small 'sets'.
    0x121D77F8, // Large 'setssbs|y'.
    0x0FFF41FA, // Large 'setz' + 'set.z|e'
    0x8A3714D3, // Small 'sfence'.
    0x800A10F3, // Small 'sgdt'.
    0x447E460F, // Large 'sha1|msg1'.
    0x4482460F, // Large 'sha1|msg2'.
    0x22FD760F, // Large 'sha1nex|te'.
    0x12348616, // Large 'sha1rnds|4'.
    0x447E6393, // Large 'sha256|msg1'.
    0x44826393, // Large 'sha256|msg2'.
    0x22839393, // Large 'sha256rnd|s2'.
    0x80003113, // Small 'shl'.
    0x80023113, // Small 'shld'.
    0x800C3113, // Small 'shlx'.
    0x80004913, // Small 'shr'.
    0x80024913, // Small 'shrd'.
    0x800C4913, // Small 'shrx'.
    0x89035513, // Small 'shufpd'.
    0xA7035513, // Small 'shufps'.
    0x800A1133, // Small 'sidt'.
    0xA8972573, // Small 'skinit'.
    0x800A1193, // Small 'sldt'.
    0x84385D93, // Small 'slwpcb'.
    0x800BCDB3, // Small 'smsw'.
    0x890A4A33, // Small 'sqrtpd'.
    0xA70A4A33, // Small 'sqrtps'.
    0x893A4A33, // Small 'sqrtsd'.
    0xA73A4A33, // Small 'sqrtss'.
    0x80018693, // Small 'stac'.
    0x80000E93, // Small 'stc'.
    0x80001293, // Small 'std'.
    0x80049E93, // Small 'stgi'.
    0x80002693, // Small 'sti'.
    0x122868AD, // Large 'stmxcs|r'.
    0x8009BE93, // Small 'stos'.
    0x80004A93, // Small 'str'.
    0x1031861E, // Large 'sttilecf|g'.
    0x8004D693, // Small 'stui'.
    0x80000AB3, // Small 'sub'.
    0x80480AB3, // Small 'subpd'.
    0x81380AB3, // Small 'subps'.
    0x80498AB3, // Small 'subsd'.
    0x81398AB3, // Small 'subss'.
    0xA67806F3, // Small 'swapgs'.
    0x38B9494E, // Large 'sysc|all'.
    0x42FC47FF, // Large 'syse|nter'.
    0x236B5803, // Large 'sysex|it'.
    0x336B5803, // Large 'sysex|itq'.
    0xA8594F33, // Small 'sysret'.
    0x236C5952, // Large 'sysre|tq'.
    0x86B9B794, // Small 't1mskc'.
    0x2282A304, // Large 'tcmmimfp16|ps'.
    0x2282A30E, // Large 'tcmmrlfp16|ps'.
    0x98C08C94, // Small 'tdcall'.
    0x22827626, // Large 'tdpbf16|ps'.
    0x331C4626, // Large 'tdpb|ssd'.
    0x36D84626, // Large 'tdpb|sud'.
    0x231D5957, // Large 'tdpbu|sd'.
    0x23CE5957, // Large 'tdpbu|ud'.
    0x2282762D, // Large 'tdpfp16|ps'.
    0x800A4CB4, // Small 'test'.
    0x935A4CB4, // Small 'testui'.
    0x0000939C, // Large 'tileloadd'.
    0x2387939C, // Large 'tileloadd|t1'.
    0x317083A5, // Large 'tilerele|ase'.
    0x122B94A8, // Large 'tilestore|d'.
    0x4599439C, // Large 'tile|zero'.
    0x2056595C, // Large 'tlbsy|nc'.
    0x8B3A8614, // Small 'tpause'.
    0x81470F54, // Small 'tzcnt'.
    0x80B9B754, // Small 'tzmsk'.
    0x231D58B4, // Large 'ucomi|sd'.
    0x222258B4, // Large 'ucomi|ss'.
    0x80006C95, // Small 'ud0'.
    0x80007095, // Small 'ud1'.
    0x80007495, // Small 'ud2'.
    0x8142C935, // Small 'uiret'.
    0x67C92808, // Large 'um|onitor'.
    0xA890DDB5, // Small 'umwait'.
    0x22AC645B, // Large 'unpckh|pd'.
    0x2282645B, // Large 'unpckh|ps'.
    0x3763545B, // Large 'unpck|lpd'.
    0x3766545B, // Large 'unpck|lps'.
    0x89021036, // Small 'vaddpd'.
    0x91021036, // Small 'vaddph'.
    0xA7021036, // Small 'vaddps'.
    0x89321036, // Small 'vaddsd'.
    0x91321036, // Small 'vaddsh'.
    0xA7321036, // Small 'vaddss'.
    0x22AC7634, // Large 'vaddsub|pd'.
    0x22827634, // Large 'vaddsub|ps'.
    0x000073AD, // Large 'vaesdec'.
    0x322D83AD, // Large 'vaesdecl|ast'.
    0x000073B5, // Large 'vaesenc'.
    0x322D83B5, // Large 'vaesencl|ast'.
    0x27EE5961, // Large 'vaesi|mc'.
    0x1154F217, // Large 'vaeskeygenassis|t'.
    0x23765966, // Large 'valig|nd'.
    0x28DF5966, // Large 'valig|nq'.
    0x22AC596B, // Large 'vandn|pd'.
    0x2282596B, // Large 'vandn|ps'.
    0x89023836, // Small 'vandpd'.
    0xA7023836, // Small 'vandps'.
    0x1152D267, // Large 'vbcstnebf162p|s'.
    0x53187267, // Large 'vbcstne|sh2ps'.
    0x22AC763B, // Large 'vblendm|pd'.
    0x2282763B, // Large 'vblendm|ps'.
    0x22AC663B, // Large 'vblend|pd'.
    0x2282663B, // Large 'vblend|ps'.
    0x3642663B, // Large 'vblend|vpd'.
    0x3364663B, // Large 'vblend|vps'.
    0x3274B226, // Large 'vbroadcastf|128'.
    0x1209E226, // Large 'vbroadcastf32x|2'.
    0x1234E226, // Large 'vbroadcastf32x|4'.
    0x120BE226, // Large 'vbroadcastf32x|8'.
    0x4235B226, // Large 'vbroadcastf|64x2'.
    0x4239B226, // Large 'vbroadcastf|64x4'.
    0x4277A226, // Large 'vbroadcast|i128'.
    0x523DA226, // Large 'vbroadcast|i32x2'.
    0x5242A226, // Large 'vbroadcast|i32x4'.
    0x5247A226, // Large 'vbroadcast|i32x8'.
    0x524CA226, // Large 'vbroadcast|i64x2'.
    0x5251A226, // Large 'vbroadcast|i64x4'.
    0x231DA226, // Large 'vbroadcast|sd'.
    0x2222A226, // Large 'vbroadcast|ss'.
    0x89083476, // Small 'vcmppd'.
    0x91083476, // Small 'vcmpph'.
    0xA7083476, // Small 'vcmpps'.
    0x89383476, // Small 'vcmpsd'.
    0x91383476, // Small 'vcmpsh'.
    0xA7383476, // Small 'vcmpss'.
    0x231D5970, // Large 'vcomi|sd'.
    0x22DA5970, // Large 'vcomi|sh'.
    0x22225970, // Large 'vcomi|ss'.
    0x22AC93BD, // Large 'vcompress|pd'.
    0x228293BD, // Large 'vcompress|ps'.
    0x22AC7645, // Large 'vcvtdq2|pd'.
    0x22A77645, // Large 'vcvtdq2|ph'.
    0x22827645, // Large 'vcvtdq2|ps'.
    0x426EA27B, // Large 'vcvtne2ps2|bf16'.
    0x3281B285, // Large 'vcvtneebf16|2ps'.
    0x531F7285, // Large 'vcvtnee|ph2ps'.
    0x3281B290, // Large 'vcvtneobf16|2ps'.
    0x531F7290, // Large 'vcvtneo|ph2ps'.
    0x426E92DF, // Large 'vcvtneps2|bf16'.
    0x34E164B1, // Large 'vcvtpd|2dq'.
    0x32A664B1, // Large 'vcvtpd|2ph'.
    0x328164B1, // Large 'vcvtpd|2ps'.
    0x34E464B1, // Large 'vcvtpd|2qq'.
    0x63CB427B, // Large 'vcvt|pd2udq'.
    0x43D964B1, // Large 'vcvtpd|2uqq'.
    0x23CF74B7, // Large 'vcvtph2|dq'.
    0x122B84B7, // Large 'vcvtph2p|d'.
    0x000094B7, // Large 'vcvtph2ps'.
    0x123394B7, // Large 'vcvtph2ps|x'.
    0x23D074B7, // Large 'vcvtph2|qq'.
    0x33CE74B7, // Large 'vcvtph2|udq'.
    0x33DA74B7, // Large 'vcvtph2|uqq'.
    0x264C74B7, // Large 'vcvtph2|uw'.
    0x126474B7, // Large 'vcvtph2|w'.
    0x23CF74C0, // Large 'vcvtps2|dq'.
    0x122B84C0, // Large 'vcvtps2p|d'.
    0x000094C0, // Large 'vcvtps2ph'.
    0x123394C0, // Large 'vcvtps2ph|x'.
    0x23D074C0, // Large 'vcvtps2|qq'.
    0x33CE74C0, // Large 'vcvtps2|udq'.
    0x33DA74C0, // Large 'vcvtps2|uqq'.
    0x22AC764E, // Large 'vcvtqq2|pd'.
    0x22A7764E, // Large 'vcvtqq2|ph'.
    0x2282764E, // Large 'vcvtqq2|ps'.
    0x22DA74C9, // Large 'vcvtsd2|sh'.
    0x222374C9, // Large 'vcvtsd2|si'.
    0x222274C9, // Large 'vcvtsd2|ss'.
    0x222384C9, // Large 'vcvtsd2u|si'.
    0x231D74D1, // Large 'vcvtsh2|sd'.
    0x222374D1, // Large 'vcvtsh2|si'.
    0x222274D1, // Large 'vcvtsh2|ss'.
    0x222384D1, // Large 'vcvtsh2u|si'.
    0x231D7655, // Large 'vcvtsi2|sd'.
    0x22DA7655, // Large 'vcvtsi2|sh'.
    0x22227655, // Large 'vcvtsi2|ss'.
    0x231D74D9, // Large 'vcvtss2|sd'.
    0x22DA74D9, // Large 'vcvtss2|sh'.
    0x222374D9, // Large 'vcvtss2|si'.
    0x222384D9, // Large 'vcvtss2u|si'.
    0x23CF83C6, // Large 'vcvttpd2|dq'.
    0x23D083C6, // Large 'vcvttpd2|qq'.
    0x1215A3C6, // Large 'vcvttpd2ud|q'.
    0x23D093C6, // Large 'vcvttpd2u|qq'.
    0x23CF83D2, // Large 'vcvttph2|dq'.
    0x23D083D2, // Large 'vcvttph2|qq'.
    0x43CD73D2, // Large 'vcvttph|2udq'.
    0x43D973D2, // Large 'vcvttph|2uqq'.
    0x126493D2, // Large 'vcvttph2u|w'.
    0x126483D2, // Large 'vcvttph2|w'.
    0x34E173DD, // Large 'vcvttps|2dq'.
    0x34E473DD, // Large 'vcvttps|2qq'.
    0x43CD73DD, // Large 'vcvttps|2udq'.
    0x43D973DD, // Large 'vcvttps|2uqq'.
    0x222383E4, // Large 'vcvttsd2|si'.
    0x222393E4, // Large 'vcvttsd2u|si'.
    0x222383ED, // Large 'vcvttsh2|si'.
    0x222393ED, // Large 'vcvttsh2u|si'.
    0x222383F6, // Large 'vcvttss2|si'.
    0x222393F6, // Large 'vcvttss2u|si'.
    0x22AC84E7, // Large 'vcvtudq2|pd'.
    0x22A784E7, // Large 'vcvtudq2|ph'.
    0x228284E7, // Large 'vcvtudq2|ps'.
    0x22AC84EF, // Large 'vcvtuqq2|pd'.
    0x22A784EF, // Large 'vcvtuqq2|ph'.
    0x228284EF, // Large 'vcvtuqq2|ps'.
    0x231D84F7, // Large 'vcvtusi2|sd'.
    0x22DA84F7, // Large 'vcvtusi2|sh'.
    0x222284F7, // Large 'vcvtusi2|ss'.
    0x32A6665C, // Large 'vcvtuw|2ph'.
    0x32A6580A, // Large 'vcvtw|2ph'.
    0x25787662, // Large 'vdbpsad|bw'.
    0x890B2496, // Small 'vdivpd'.
    0x910B2496, // Small 'vdivph'.
    0xA70B2496, // Small 'vdivps'.
    0x893B2496, // Small 'vdivsd'.
    0x913B2496, // Small 'vdivsh'.
    0xA73B2496, // Small 'vdivss'.
    0x22827669, // Large 'vdpbf16|ps'.
    0x80484096, // Small 'vdppd'.
    0x81384096, // Small 'vdpps'.
    0x800948B6, // Small 'verr'.
    0x800BC8B6, // Small 'verw'.
    0x22AC7670, // Large 'vexpand|pd'.
    0x22827670, // Large 'vexpand|ps'.
    0x327492EF, // Large 'vextractf|128'.
    0x622F72E8, // Large 'vextrac|tf32x4'.
    0x424892EF, // Large 'vextractf|32x8'.
    0x423592EF, // Large 'vextractf|64x2'.
    0x423992EF, // Large 'vextractf|64x4'.
    0x427782EF, // Large 'vextract|i128'.
    0x524282EF, // Large 'vextract|i32x4'.
    0x524782EF, // Large 'vextract|i32x8'.
    0x524C82EF, // Large 'vextract|i64x2'.
    0x525182EF, // Large 'vextract|i64x4'.
    0x228282EF, // Large 'vextract|ps'.
    0x22A784FF, // Large 'vfcmaddc|ph'.
    0x22DA84FF, // Large 'vfcmaddc|sh'.
    0x22A77677, // Large 'vfcmulc|ph'.
    0x22DA7677, // Large 'vfcmulc|sh'.
    0x22AC93FF, // Large 'vfixupimm|pd'.
    0x228293FF, // Large 'vfixupimm|ps'.
    0x231D93FF, // Large 'vfixupimm|sd'.
    0x222293FF, // Large 'vfixupimm|ss'.
    0x22AC9408, // Large 'vfmadd132|pd'.
    0x22A79408, // Large 'vfmadd132|ph'.
    0x22829408, // Large 'vfmadd132|ps'.
    0x231D9408, // Large 'vfmadd132|sd'.
    0x22DA9408, // Large 'vfmadd132|sh'.
    0x22229408, // Large 'vfmadd132|ss'.
    0x52A9629B, // Large 'vfmadd|213pd'.
    0x52AE629B, // Large 'vfmadd|213ph'.
    0x52B3629B, // Large 'vfmadd|213ps'.
    0x532E629B, // Large 'vfmadd|213sd'.
    0x5333629B, // Large 'vfmadd|213sh'.
    0x5338629B, // Large 'vfmadd|213ss'.
    0x52B8629B, // Large 'vfmadd|231pd'.
    0x52BD629B, // Large 'vfmadd|231ph'.
    0x52C2629B, // Large 'vfmadd|231ps'.
    0x533D629B, // Large 'vfmadd|231sd'.
    0x5342629B, // Large 'vfmadd|231sh'.
    0x5347629B, // Large 'vfmadd|231ss'.
    0x367E629B, // Large 'vfmadd|cph'.
    0x3681629B, // Large 'vfmadd|csh'.
    0x22AC629B, // Large 'vfmadd|pd'.
    0x2282629B, // Large 'vfmadd|ps'.
    0x122B729B, // Large 'vfmadds|d'.
    0x1152729B, // Large 'vfmadds|s'.
    0x122BD29B, // Large 'vfmaddsub132p|d'.
    0x12A8D29B, // Large 'vfmaddsub132p|h'.
    0x1152D29B, // Large 'vfmaddsub132p|s'.
    0x52A9929B, // Large 'vfmaddsub|213pd'.
    0x52AE929B, // Large 'vfmaddsub|213ph'.
    0x52B3929B, // Large 'vfmaddsub|213ps'.
    0x52B8929B, // Large 'vfmaddsub|231pd'.
    0x52BD929B, // Large 'vfmaddsub|231ph'.
    0x52C2929B, // Large 'vfmaddsub|231ps'.
    0x22AC929B, // Large 'vfmaddsub|pd'.
    0x2282929B, // Large 'vfmaddsub|ps'.
    0x22AC9411, // Large 'vfmsub132|pd'.
    0x22A79411, // Large 'vfmsub132|ph'.
    0x22829411, // Large 'vfmsub132|ps'.
    0x231D9411, // Large 'vfmsub132|sd'.
    0x22DA9411, // Large 'vfmsub132|sh'.
    0x22229411, // Large 'vfmsub132|ss'.
    0x52A962C7, // Large 'vfmsub|213pd'.
    0x52AE62C7, // Large 'vfmsub|213ph'.
    0x52B362C7, // Large 'vfmsub|213ps'.
    0x532E62C7, // Large 'vfmsub|213sd'.
    0x533362C7, // Large 'vfmsub|213sh'.
    0x533862C7, // Large 'vfmsub|213ss'.
    0x52B862C7, // Large 'vfmsub|231pd'.
    0x52BD62C7, // Large 'vfmsub|231ph'.
    0x52C262C7, // Large 'vfmsub|231ps'.
    0x533D62C7, // Large 'vfmsub|231sd'.
    0x534262C7, // Large 'vfmsub|231sh'.
    0x534762C7, // Large 'vfmsub|231ss'.
    0x22ACC2C7, // Large 'vfmsubadd132|pd'.
    0x22A7C2C7, // Large 'vfmsubadd132|ph'.
    0x2282C2C7, // Large 'vfmsubadd132|ps'.
    0x52A992C7, // Large 'vfmsubadd|213pd'.
    0x52AE92C7, // Large 'vfmsubadd|213ph'.
    0x52B392C7, // Large 'vfmsubadd|213ps'.
    0x52B892C7, // Large 'vfmsubadd|231pd'.
    0x52BD92C7, // Large 'vfmsubadd|231ph'.
    0x52C292C7, // Large 'vfmsubadd|231ps'.
    0x22AC92C7, // Large 'vfmsubadd|pd'.
    0x228292C7, // Large 'vfmsubadd|ps'.
    0x22AC62C7, // Large 'vfmsub|pd'.
    0x228262C7, // Large 'vfmsub|ps'.
    0x231D62C7, // Large 'vfmsub|sd'.
    0x222262C7, // Large 'vfmsub|ss'.
    0x367E580F, // Large 'vfmul|cph'.
    0x3681580F, // Large 'vfmul|csh'.
    0x22ACA324, // Large 'vfnmadd132|pd'.
    0x22A7A324, // Large 'vfnmadd132|ph'.
    0x2282A324, // Large 'vfnmadd132|ps'.
    0x231DA324, // Large 'vfnmadd132|sd'.
    0x22DAA324, // Large 'vfnmadd132|sh'.
    0x2222A324, // Large 'vfnmadd132|ss'.
    0x52A97324, // Large 'vfnmadd|213pd'.
    0x52AE7324, // Large 'vfnmadd|213ph'.
    0x52B37324, // Large 'vfnmadd|213ps'.
    0x532E7324, // Large 'vfnmadd|213sd'.
    0x53337324, // Large 'vfnmadd|213sh'.
    0x53387324, // Large 'vfnmadd|213ss'.
    0x52B87324, // Large 'vfnmadd|231pd'.
    0x52BD7324, // Large 'vfnmadd|231ph'.
    0x52C27324, // Large 'vfnmadd|231ps'.
    0x533D7324, // Large 'vfnmadd|231sd'.
    0x53427324, // Large 'vfnmadd|231sh'.
    0x53477324, // Large 'vfnmadd|231ss'.
    0x22AC7324, // Large 'vfnmadd|pd'.
    0x22827324, // Large 'vfnmadd|ps'.
    0x231D7324, // Large 'vfnmadd|sd'.
    0x22227324, // Large 'vfnmadd|ss'.
    0x22ACA34C, // Large 'vfnmsub132|pd'.
    0x22A7A34C, // Large 'vfnmsub132|ph'.
    0x2282A34C, // Large 'vfnmsub132|ps'.
    0x231DA34C, // Large 'vfnmsub132|sd'.
    0x22DAA34C, // Large 'vfnmsub132|sh'.
    0x2222A34C, // Large 'vfnmsub132|ss'.
    0x52A9734C, // Large 'vfnmsub|213pd'.
    0x52AE734C, // Large 'vfnmsub|213ph'.
    0x52B3734C, // Large 'vfnmsub|213ps'.
    0x532E734C, // Large 'vfnmsub|213sd'.
    0x5333734C, // Large 'vfnmsub|213sh'.
    0x5338734C, // Large 'vfnmsub|213ss'.
    0x52B8734C, // Large 'vfnmsub|231pd'.
    0x52BD734C, // Large 'vfnmsub|231ph'.
    0x52C2734C, // Large 'vfnmsub|231ps'.
    0x533D734C, // Large 'vfnmsub|231sd'.
    0x5342734C, // Large 'vfnmsub|231sh'.
    0x5347734C, // Large 'vfnmsub|231ss'.
    0x22AC734C, // Large 'vfnmsub|pd'.
    0x2282734C, // Large 'vfnmsub|ps'.
    0x231D734C, // Large 'vfnmsub|sd'.
    0x2222734C, // Large 'vfnmsub|ss'.
    0x22AC8507, // Large 'vfpclass|pd'.
    0x22A78507, // Large 'vfpclass|ph'.
    0x22828507, // Large 'vfpclass|ps'.
    0x231D8507, // Large 'vfpclass|sd'.
    0x22DA8507, // Large 'vfpclass|sh'.
    0x22228507, // Large 'vfpclass|ss'.
    0x22AC5975, // Large 'vfrcz|pd'.
    0x22825975, // Large 'vfrcz|ps'.
    0x231D5975, // Large 'vfrcz|sd'.
    0x22225975, // Large 'vfrcz|ss'.
    0x22AC850F, // Large 'vgatherd|pd'.
    0x2282850F, // Large 'vgatherd|ps'.
    0x3478750F, // Large 'vgather|qpd'.
    0x347B750F, // Large 'vgather|qps'.
    0x22AC7684, // Large 'vgetexp|pd'.
    0x22A77684, // Large 'vgetexp|ph'.
    0x22827684, // Large 'vgetexp|ps'.
    0x231D7684, // Large 'vgetexp|sd'.
    0x22DA7684, // Large 'vgetexp|sh'.
    0x22227684, // Large 'vgetexp|ss'.
    0x33CA7517, // Large 'vgetman|tpd'.
    0x33D67517, // Large 'vgetman|tph'.
    0x33E17517, // Large 'vgetman|tps'.
    0x33E87517, // Large 'vgetman|tsd'.
    0x33F17517, // Large 'vgetman|tsh'.
    0x33FA7517, // Large 'vgetman|tss'.
    0x2215F206, // Large 'vgf2p8affineinv|qb'.
    0x2215C206, // Large 'vgf2p8affine|qb'.
    0x451E6206, // Large 'vgf2p8|mulb'.
    0x22AC597A, // Large 'vhadd|pd'.
    0x2282597A, // Large 'vhadd|ps'.
    0x22AC597F, // Large 'vhsub|pd'.
    0x2282597F, // Large 'vhsub|ps'.
    0x3274835C, // Large 'vinsertf|128'.
    0x622F6356, // Large 'vinser|tf32x4'.
    0x4248835C, // Large 'vinsertf|32x8'.
    0x4235835C, // Large 'vinsertf|64x2'.
    0x4239835C, // Large 'vinsertf|64x4'.
    0x4277735C, // Large 'vinsert|i128'.
    0x5242735C, // Large 'vinsert|i32x4'.
    0x5247735C, // Large 'vinsert|i32x8'.
    0x524C735C, // Large 'vinsert|i64x2'.
    0x5251735C, // Large 'vinsert|i64x4'.
    0x2282735C, // Large 'vinsert|ps'.
    0xAB121196, // Small 'vlddqu'.
    0x12287814, // Large 'vldmxcs|r'.
    0x12A2A41A, // Large 'vmaskmovdq|u'.
    0x22AC841A, // Large 'vmaskmov|pd'.
    0x2282841A, // Large 'vmaskmov|ps'.
    0x890C05B6, // Small 'vmaxpd'.
    0x910C05B6, // Small 'vmaxph'.
    0xA70C05B6, // Small 'vmaxps'.
    0x893C05B6, // Small 'vmaxsd'.
    0x913C05B6, // Small 'vmaxsh'.
    0xA73C05B6, // Small 'vmaxss'.
    0x98C08DB6, // Small 'vmcall'.
    0x25F95984, // Large 'vmcle|ar'.
    0x86EA99B6, // Small 'vmfunc'.
    0x236B5989, // Large 'vmgex|it'.
    0x890725B6, // Small 'vminpd'.
    0x910725B6, // Small 'vminph'.
    0xA70725B6, // Small 'vminps'.
    0x893725B6, // Small 'vminsd'.
    0x913725B6, // Small 'vminsh'.
    0xA73725B6, // Small 'vminss'.
    0x237E681B, // Large 'vmlaun|ch'.
    0x8817B1B6, // Small 'vmload'.
    0x38B9498E, // Large 'vmmc|all'.
    0x22AC5992, // Large 'vmova|pd'.
    0x22825992, // Large 'vmova|ps'.
    0x804B3DB6, // Small 'vmovd'.
    0x3821568B, // Large 'vmovd|dup'.
    0x0000768B, // Large 'vmovdqa'.
    0x2231768B, // Large 'vmovdqa|32'.
    0x2235768B, // Large 'vmovdqa|64'.
    0x12A2668B, // Large 'vmovdq|u'.
    0x3692668B, // Large 'vmovdq|u16'.
    0x3695668B, // Large 'vmovdq|u32'.
    0x3698668B, // Large 'vmovdq|u64'.
    0x2824668B, // Large 'vmovdq|u8'.
    0x37665826, // Large 'vmovh|lps'.
    0x22AC5826, // Large 'vmovh|pd'.
    0x22825826, // Large 'vmovh|ps'.
    0x2282682B, // Large 'vmovlh|ps'.
    0x22AC582B, // Large 'vmovl|pd'.
    0x2282582B, // Large 'vmovl|ps'.
    0x22AC769B, // Large 'vmovmsk|pd'.
    0x2282769B, // Large 'vmovmsk|ps'.
    0x23CF66A2, // Large 'vmovnt|dq'.
    0x368F66A2, // Large 'vmovnt|dqa'.
    0x22AC66A2, // Large 'vmovnt|pd'.
    0x228266A2, // Large 'vmovnt|ps'.
    0x811B3DB6, // Small 'vmovq'.
    0x893B3DB6, // Small 'vmovsd'.
    0x913B3DB6, // Small 'vmovsh'.
    0x240376A8, // Large 'vmovshd|up'.
    0x240376AF, // Large 'vmovsld|up'.
    0xA73B3DB6, // Small 'vmovss'.
    0x3604468B, // Large 'vmov|upd'.
    0x22825997, // Large 'vmovu|ps'.
    0x817B3DB6, // Small 'vmovw'.
    0x25786831, // Large 'vmpsad|bw'.
    0x35E2499C, // Large 'vmpt|rld'.
    0x35DE499C, // Large 'vmpt|rst'.
    0x8812C9B6, // Small 'vmread'.
    0x100F7837, // Large 'vmresum|e'.
    0x80EAC9B6, // Small 'vmrun'.
    0x8B60CDB6, // Small 'vmsave'.
    0x890655B6, // Small 'vmulpd'.
    0x910655B6, // Small 'vmulph'.
    0xA70655B6, // Small 'vmulps'.
    0x893655B6, // Small 'vmulsd'.
    0x913655B6, // Small 'vmulsh'.
    0xA73655B6, // Small 'vmulss'.
    0x22FD59A0, // Large 'vmwri|te'.
    0x8C67E1B6, // Small 'vmxoff'.
    0x80E7E1B6, // Small 'vmxon'.
    0x804849F6, // Small 'vorpd'.
    0x813849F6, // Small 'vorps'.
    0x122BC2F8, // Large 'vp2intersect|d'.
    0x1215C2F8, // Large 'vp2intersect|q'.
    0x85310616, // Small 'vpabsb'.
    0x89310616, // Small 'vpabsd'.
    0xA3310616, // Small 'vpabsq'.
    0xAF310616, // Small 'vpabsw'.
    0x126486B6, // Large 'vpackssd|w'.
    0x26BE76B6, // Large 'vpackss|wb'.
    0x36BC66C0, // Large 'vpacku|sdw'.
    0x36C666C0, // Large 'vpacku|swb'.
    0x84420616, // Small 'vpaddb'.
    0x88420616, // Small 'vpaddd'.
    0xA2420616, // Small 'vpaddq'.
    0x2786583E, // Large 'vpadd|sb'.
    0x2561583E, // Large 'vpadd|sw'.
    0x2786683E, // Large 'vpaddu|sb'.
    0x2561683E, // Large 'vpaddu|sw'.
    0xAE420616, // Small 'vpaddw'.
    0x12287844, // Large 'vpalign|r'.
    0x80470616, // Small 'vpand'.
    0x88470616, // Small 'vpandd'.
    0x9C470616, // Small 'vpandn'.
    0x237659A5, // Large 'vpand|nd'.
    0x28DF59A5, // Large 'vpand|nq'.
    0xA2470616, // Small 'vpandq'.
    0x847B0616, // Small 'vpavgb'.
    0xAE7B0616, // Small 'vpavgw'.
    0x122B76C9, // Large 'vpblend|d'.
    0x226176C9, // Large 'vpblend|mb'.
    0x26D076C9, // Large 'vpblend|md'.
    0x121586C9, // Large 'vpblendm|q'.
    0x126486C9, // Large 'vpblendm|w'.
    0x200376C9, // Large 'vpblend|vb'.
    0x126476C9, // Large 'vpblend|w'.
    0x1004B256, // Large 'vpbroadcast|b'.
    0x122BB256, // Large 'vpbroadcast|d'.
    0x1215E256, // Large 'vpbroadcastmb2|q'.
    0x3264C256, // Large 'vpbroadcastm|w2d'.
    0x1215B256, // Large 'vpbroadcast|q'.
    0x1264B256, // Large 'vpbroadcast|w'.
    0x44626522, // Large 'vpclmu|lqdq'.
    0xACF68E16, // Small 'vpcmov'.
    0x85068E16, // Small 'vpcmpb'.
    0x89068E16, // Small 'vpcmpd'.
    0x22156528, // Large 'vpcmpe|qb'.
    0x24576528, // Large 'vpcmpe|qd'.
    0x23D06528, // Large 'vpcmpe|qq'.
    0x27256528, // Large 'vpcmpe|qw'.
    0x120F9528, // Large 'vpcmpestr|i'.
    0x10019528, // Large 'vpcmpestr|m'.
    0x384B5528, // Large 'vpcmp|gtb'.
    0x384E5528, // Large 'vpcmp|gtd'.
    0x38515528, // Large 'vpcmp|gtq'.
    0x38545528, // Large 'vpcmp|gtw'.
    0x120F9531, // Large 'vpcmpistr|i'.
    0x10019531, // Large 'vpcmpistr|m'.
    0xA3068E16, // Small 'vpcmpq'.
    0x22A25528, // Large 'vpcmp|ub'.
    0x23CE5528, // Large 'vpcmp|ud'.
    0x23DA5528, // Large 'vpcmp|uq'.
    0x264C5528, // Large 'vpcmp|uw'.
    0xAF068E16, // Small 'vpcmpw'.
    0x84D78E16, // Small 'vpcomb'.
    0x88D78E16, // Small 'vpcomd'.
    0x1004A424, // Large 'vpcompress|b'.
    0x122BA424, // Large 'vpcompress|d'.
    0x1215A424, // Large 'vpcompress|q'.
    0x1264A424, // Large 'vpcompress|w'.
    0xA2D78E16, // Small 'vpcomq'.
    0x22A25424, // Large 'vpcom|ub'.
    0x23CE5424, // Large 'vpcom|ud'.
    0x23DA5424, // Large 'vpcom|uq'.
    0x264C5424, // Large 'vpcom|uw'.
    0xAED78E16, // Small 'vpcomw'.
    0x122BA42E, // Large 'vpconflict|d'.
    0x1215A42E, // Large 'vpconflict|q'.
    0x122B76D2, // Large 'vpdpbss|d'.
    0x22A076D2, // Large 'vpdpbss|ds'.
    0x23CE66D2, // Large 'vpdpbs|ud'.
    0x36D966D2, // Large 'vpdpbs|uds'.
    0x122B76DC, // Large 'vpdpbus|d'.
    0x22A076DC, // Large 'vpdpbus|ds'.
    0x23CE66DC, // Large 'vpdpbu|ud'.
    0x36D966DC, // Large 'vpdpbu|uds'.
    0x122B76E3, // Large 'vpdpwss|d'.
    0x22A076E3, // Large 'vpdpwss|ds'.
    0x23CE66E3, // Large 'vpdpws|ud'.
    0x36D966E3, // Large 'vpdpws|uds'.
    0x122B76EA, // Large 'vpdpwus|d'.
    0x22A076EA, // Large 'vpdpwus|ds'.
    0x23CE66EA, // Large 'vpdpwu|ud'.
    0x36D966EA, // Large 'vpdpwu|uds'.
    0x3274753A, // Large 'vperm2f|128'.
    0x4277653A, // Large 'vperm2|i128'.
    0x84D91616, // Small 'vpermb'.
    0x88D91616, // Small 'vpermd'.
    0x28576541, // Large 'vpermi|2b'.
    0x22656541, // Large 'vpermi|2d'.
    0x36F16541, // Large 'vpermi|2pd'.
    0x32816541, // Large 'vpermi|2ps'.
    0x24E46541, // Large 'vpermi|2q'.
    0x22636541, // Large 'vpermi|2w'.
    0x22AC8541, // Large 'vpermil2|pd'.
    0x22828541, // Large 'vpermil2|ps'.
    0x22AC7541, // Large 'vpermil|pd'.
    0x22827541, // Large 'vpermil|ps'.
    0x22AC553A, // Large 'vperm|pd'.
    0x2282553A, // Large 'vperm|ps'.
    0xA2D91616, // Small 'vpermq'.
    0x285766F4, // Large 'vpermt|2b'.
    0x226566F4, // Large 'vpermt|2d'.
    0x36F166F4, // Large 'vpermt|2pd'.
    0x328166F4, // Large 'vpermt|2ps'.
    0x24E466F4, // Large 'vpermt|2q'.
    0x226366F4, // Large 'vpermt|2w'.
    0xAED91616, // Small 'vpermw'.
    0x266376FA, // Large 'vpexpan|db'.
    0x229F76FA, // Large 'vpexpan|dd'.
    0x23CF76FA, // Large 'vpexpan|dq'.
    0x26BD76FA, // Large 'vpexpan|dw'.
    0x37BF46FA, // Large 'vpex|trb'.
    0x247659AA, // Large 'vpext|rd'.
    0x245659AA, // Large 'vpext|rq'.
    0x29AF59AA, // Large 'vpext|rw'.
    0x229F8549, // Large 'vpgather|dd'.
    0x23CF8549, // Large 'vpgather|dq'.
    0x24578549, // Large 'vpgather|qd'.
    0x23D08549, // Large 'vpgather|qq'.
    0x28596701, // Large 'vphadd|bd'.
    0x285B6701, // Large 'vphadd|bq'.
    0x25786701, // Large 'vphadd|bw'.
    0x122B6701, // Large 'vphadd|d'.
    0x23CF6701, // Large 'vphadd|dq'.
    0x25616701, // Large 'vphadd|sw'.
    0x122B8701, // Large 'vphaddub|d'.
    0x12158701, // Large 'vphaddub|q'.
    0x12648701, // Large 'vphaddub|w'.
    0x23CF7701, // Large 'vphaddu|dq'.
    0x257A7701, // Large 'vphaddu|wd'.
    0x27097701, // Large 'vphaddu|wq'.
    0x12646701, // Large 'vphadd|w'.
    0x257A6701, // Large 'vphadd|wd'.
    0x27096701, // Large 'vphadd|wq'.
    0x1264A438, // Large 'vphminposu|w'.
    0x2578685D, // Large 'vphsub|bw'.
    0x122B685D, // Large 'vphsub|d'.
    0x23CF685D, // Large 'vphsub|dq'.
    0x2561685D, // Large 'vphsub|sw'.
    0x1264685D, // Large 'vphsub|w'.
    0x257A685D, // Large 'vphsub|wd'.
    0x27C059B1, // Large 'vpins|rb'.
    0x247659B1, // Large 'vpins|rd'.
    0x245659B1, // Large 'vpins|rq'.
    0x29AF59B1, // Large 'vpins|rw'.
    0x26266863, // Large 'vplzcn|td'.
    0x236C6863, // Large 'vplzcn|tq'.
    0x229F6551, // Large 'vpmacs|dd'.
    0x370B6551, // Large 'vpmacs|dqh'.
    0x35806551, // Large 'vpmacs|dql'.
    0x122B8551, // Large 'vpmacssd|d'.
    0x12A89551, // Large 'vpmacssdq|h'.
    0x10279551, // Large 'vpmacssdq|l'.
    0x257A7551, // Large 'vpmacss|wd'.
    0x25797551, // Large 'vpmacss|ww'.
    0x257A6551, // Large 'vpmacs|wd'.
    0x25796551, // Large 'vpmacs|ww'.
    0x122B955A, // Large 'vpmadcssw|d'.
    0x257A755A, // Large 'vpmadcs|wd'.
    0x23DA9442, // Large 'vpmadd52h|uq'.
    0x344B8442, // Large 'vpmadd52|luq'.
    0x45636442, // Large 'vpmadd|ubsw'.
    0x257A6442, // Large 'vpmadd|wd'.
    0x641D4442, // Large 'vpma|skmovd'.
    0x22148567, // Large 'vpmaskmo|vq'.
    0x278659B6, // Large 'vpmax|sb'.
    0x231D59B6, // Large 'vpmax|sd'.
    0x258759B6, // Large 'vpmax|sq'.
    0x256159B6, // Large 'vpmax|sw'.
    0x22A259B6, // Large 'vpmax|ub'.
    0x23CE59B6, // Large 'vpmax|ud'.
    0x23DA59B6, // Large 'vpmax|uq'.
    0x264C59B6, // Large 'vpmax|uw'.
    0x278659BB, // Large 'vpmin|sb'.
    0x231D59BB, // Large 'vpmin|sd'.
    0x258759BB, // Large 'vpmin|sq'.
    0x256159BB, // Large 'vpmin|sw'.
    0x22A259BB, // Large 'vpmin|ub'.
    0x23CE59BB, // Large 'vpmin|ud'.
    0x23DA59BB, // Large 'vpmin|uq'.
    0x264C59BB, // Large 'vpmin|uw'.
    0x3869570E, // Large 'vpmov|b2m'.
    0x386C570E, // Large 'vpmov|d2m'.
    0x2663570E, // Large 'vpmov|db'.
    0x26BD570E, // Large 'vpmov|dw'.
    0x2857670E, // Large 'vpmovm|2b'.
    0x2265670E, // Large 'vpmovm|2d'.
    0x24E4670E, // Large 'vpmovm|2q'.
    0x2263670E, // Large 'vpmovm|2w'.
    0x1004870E, // Large 'vpmovmsk|b'.
    0x386F570E, // Large 'vpmov|q2m'.
    0x2215570E, // Large 'vpmov|qb'.
    0x2457570E, // Large 'vpmov|qd'.
    0x2725570E, // Large 'vpmov|qw'.
    0x26636716, // Large 'vpmovs|db'.
    0x26BD6716, // Large 'vpmovs|dw'.
    0x22156716, // Large 'vpmovs|qb'.
    0x24576716, // Large 'vpmovs|qd'.
    0x27256716, // Large 'vpmovs|qw'.
    0x26BE6716, // Large 'vpmovs|wb'.
    0x122B8716, // Large 'vpmovsxb|d'.
    0x12158716, // Large 'vpmovsxb|q'.
    0x12648716, // Large 'vpmovsxb|w'.
    0x23CF7716, // Large 'vpmovsx|dq'.
    0x257A7716, // Large 'vpmovsx|wd'.
    0x27097716, // Large 'vpmovsx|wq'.
    0x2663771E, // Large 'vpmovus|db'.
    0x26BD771E, // Large 'vpmovus|dw'.
    0x2215771E, // Large 'vpmovus|qb'.
    0x2457771E, // Large 'vpmovus|qd'.
    0x2725771E, // Large 'vpmovus|qw'.
    0x26BE771E, // Large 'vpmovus|wb'.
    0x3872570E, // Large 'vpmov|w2m'.
    0x26BE570E, // Large 'vpmov|wb'.
    0x122B8727, // Large 'vpmovzxb|d'.
    0x12158727, // Large 'vpmovzxb|q'.
    0x12648727, // Large 'vpmovzxb|w'.
    0x23CF7727, // Large 'vpmovzx|dq'.
    0x257A7727, // Large 'vpmovzx|wd'.
    0x27097727, // Large 'vpmovzx|wq'.
    0x23CF52D3, // Large 'vpmul|dq'.
    0x2561772F, // Large 'vpmulhr|sw'.
    0x264C672F, // Large 'vpmulh|uw'.
    0x1264672F, // Large 'vpmulh|w'.
    0x257F52D3, // Large 'vpmul|ld'.
    0x246252D3, // Large 'vpmul|lq'.
    0x258252D3, // Large 'vpmul|lw'.
    0x2215C2D3, // Large 'vpmultishift|qb'.
    0x33CE52D3, // Large 'vpmul|udq'.
    0x21546875, // Large 'vpopcn|tb'.
    0x26266875, // Large 'vpopcn|td'.
    0x236C6875, // Large 'vpopcn|tq'.
    0x27B86875, // Large 'vpopcn|tw'.
    0x80093E16, // Small 'vpor'.
    0x80493E16, // Small 'vpord'.
    0x81193E16, // Small 'vporq'.
    0x9B22C216, // Small 'vpperm'.
    0x88C7CA16, // Small 'vprold'.
    0xA2C7CA16, // Small 'vprolq'.
    0x242159C0, // Large 'vprol|vd'.
    0x221459C0, // Large 'vprol|vq'.
    0x8927CA16, // Small 'vprord'.
    0xA327CA16, // Small 'vprorq'.
    0x242159C5, // Large 'vpror|vd'.
    0x221459C5, // Large 'vpror|vq'.
    0x8547CA16, // Small 'vprotb'.
    0x8947CA16, // Small 'vprotd'.
    0xA347CA16, // Small 'vprotq'.
    0xAF47CA16, // Small 'vprotw'.
    0x257859CA, // Large 'vpsad|bw'.
    0x229F944E, // Large 'vpscatter|dd'.
    0x23CF944E, // Large 'vpscatter|dq'.
    0x2457944E, // Large 'vpscatter|qd'.
    0x1215A44E, // Large 'vpscatterq|q'.
    0x84144E16, // Small 'vpshab'.
    0x88144E16, // Small 'vpshad'.
    0xA2144E16, // Small 'vpshaq'.
    0xAE144E16, // Small 'vpshaw'.
    0x84C44E16, // Small 'vpshlb'.
    0x88C44E16, // Small 'vpshld'.
    0x122B687B, // Large 'vpshld|d'.
    0x1215687B, // Large 'vpshld|q'.
    0x3668587B, // Large 'vpshl|dvd'.
    0x3880587B, // Large 'vpshl|dvq'.
    0x1264787B, // Large 'vpshldv|w'.
    0x1264687B, // Large 'vpshld|w'.
    0xA2C44E16, // Small 'vpshlq'.
    0xAEC44E16, // Small 'vpshlw'.
    0x122B6883, // Large 'vpshrd|d'.
    0x12156883, // Large 'vpshrd|q'.
    0x36685883, // Large 'vpshr|dvd'.
    0x38805883, // Large 'vpshr|dvq'.
    0x38885883, // Large 'vpshr|dvw'.
    0x12646883, // Large 'vpshrd|w'.
    0x00007364, // Large 'vpshufb'.
    0x2261A364, // Large 'vpshufbitq|mb'.
    0x122B6364, // Large 'vpshuf|d'.
    0x288B6364, // Large 'vpshuf|hw'.
    0x25826364, // Large 'vpshuf|lw'.
    0x204859CF, // Large 'vpsig|nb'.
    0x237659CF, // Large 'vpsig|nd'.
    0x28C159CF, // Large 'vpsig|nw'.
    0x88C64E16, // Small 'vpslld'.
    0x357F49D4, // Large 'vpsl|ldq'.
    0xA2C64E16, // Small 'vpsllq'.
    0x242159D8, // Large 'vpsll|vd'.
    0x221459D8, // Large 'vpsll|vq'.
    0x288959D8, // Large 'vpsll|vw'.
    0xAEC64E16, // Small 'vpsllw'.
    0x88194E16, // Small 'vpsrad'.
    0xA2194E16, // Small 'vpsraq'.
    0x242159DD, // Large 'vpsra|vd'.
    0x221459DD, // Large 'vpsra|vq'.
    0x288959DD, // Large 'vpsra|vw'.
    0xAE194E16, // Small 'vpsraw'.
    0x88C94E16, // Small 'vpsrld'.
    0x357F49DD, // Large 'vpsr|ldq'.
    0xA2C94E16, // Small 'vpsrlq'.
    0x242159E2, // Large 'vpsrl|vd'.
    0x221459E2, // Large 'vpsrl|vq'.
    0x288959E2, // Large 'vpsrl|vw'.
    0xAEC94E16, // Small 'vpsrlw'.
    0x842ACE16, // Small 'vpsubb'.
    0x882ACE16, // Small 'vpsubd'.
    0xA22ACE16, // Small 'vpsubq'.
    0x2786588D, // Large 'vpsub|sb'.
    0x2561588D, // Large 'vpsub|sw'.
    0x2786688D, // Large 'vpsubu|sb'.
    0x2561688D, // Large 'vpsubu|sw'.
    0xAE2ACE16, // Small 'vpsubw'.
    0x122B956F, // Large 'vpternlog|d'.
    0x1215956F, // Large 'vpternlog|q'.
    0xA932D216, // Small 'vptest'.
    0x22616736, // Large 'vptest|mb'.
    0x26D06736, // Large 'vptest|md'.
    0x273D6736, // Large 'vptest|mq'.
    0x28716736, // Large 'vptest|mw'.
    0x22617736, // Large 'vptestn|mb'.
    0x26D07736, // Large 'vptestn|md'.
    0x273D7736, // Large 'vptestn|mq'.
    0x12648736, // Large 'vptestnm|w'.
    0x25788459, // Large 'vpunpckh|bw'.
    0x23CF8459, // Large 'vpunpckh|dq'.
    0x23CF9459, // Large 'vpunpckhq|dq'.
    0x257A8459, // Large 'vpunpckh|wd'.
    0x357C7459, // Large 'vpunpck|lbw'.
    0x357F7459, // Large 'vpunpck|ldq'.
    0x44627459, // Large 'vpunpck|lqdq'.
    0x35827459, // Large 'vpunpck|lwd'.
    0x8127E216, // Small 'vpxor'.
    0x8927E216, // Small 'vpxord'.
    0xA327E216, // Small 'vpxorq'.
    0x22AC6893, // Large 'vrange|pd'.
    0x22826893, // Large 'vrange|ps'.
    0x231D6893, // Large 'vrange|sd'.
    0x22226893, // Large 'vrange|ss'.
    0x22AC6899, // Large 'vrcp14|pd'.
    0x22826899, // Large 'vrcp14|ps'.
    0x231D6899, // Large 'vrcp14|sd'.
    0x22226899, // Large 'vrcp14|ss'.
    0x91080E56, // Small 'vrcpph'.
    0xA7080E56, // Small 'vrcpps'.
    0x91380E56, // Small 'vrcpsh'.
    0xA7380E56, // Small 'vrcpss'.
    0x22AC773F, // Large 'vreduce|pd'.
    0x22A7773F, // Large 'vreduce|ph'.
    0x2282773F, // Large 'vreduce|ps'.
    0x231D773F, // Large 'vreduce|sd'.
    0x22DA773F, // Large 'vreduce|sh'.
    0x2222773F, // Large 'vreduce|ss'.
    0x22AC9466, // Large 'vrndscale|pd'.
    0x22A79466, // Large 'vrndscale|ph'.
    0x22829466, // Large 'vrndscale|ps'.
    0x231D9466, // Large 'vrndscale|sd'.
    0x22DA9466, // Large 'vrndscale|sh'.
    0x22229466, // Large 'vrndscale|ss'.
    0x22AC689F, // Large 'vround|pd'.
    0x2282689F, // Large 'vround|ps'.
    0x231D689F, // Large 'vround|sd'.
    0x2222689F, // Large 'vround|ss'.
    0x22AC8585, // Large 'vrsqrt14|pd'.
    0x22828585, // Large 'vrsqrt14|ps'.
    0x231D8585, // Large 'vrsqrt14|sd'.
    0x22228585, // Large 'vrsqrt14|ss'.
    0x22A76585, // Large 'vrsqrt|ph'.
    0x22826585, // Large 'vrsqrt|ps'.
    0x22DA6585, // Large 'vrsqrt|sh'.
    0x22226585, // Large 'vrsqrt|ss'.
    0x22AC7746, // Large 'vscalef|pd'.
    0x22A77746, // Large 'vscalef|ph'.
    0x22827746, // Large 'vscalef|ps'.
    0x231D7746, // Large 'vscalef|sd'.
    0x22DA7746, // Large 'vscalef|sh'.
    0x22227746, // Large 'vscalef|ss'.
    0x22AC946F, // Large 'vscatterd|pd'.
    0x2282946F, // Large 'vscatterd|ps'.
    0x3478846F, // Large 'vscatter|qpd'.
    0x347B846F, // Large 'vscatter|qps'.
    0x447E736E, // Large 'vsha512|msg1'.
    0x4482736E, // Large 'vsha512|msg2'.
    0x2283A36E, // Large 'vsha512rnd|s2'.
    0x5230558D, // Large 'vshuf|f32x4'.
    0x42356592, // Large 'vshuff|64x2'.
    0x5242558D, // Large 'vshuf|i32x4'.
    0x524C558D, // Large 'vshuf|i64x2'.
    0x22AC558D, // Large 'vshuf|pd'.
    0x2282558D, // Large 'vshuf|ps'.
    0x447E474D, // Large 'vsm3|msg1'.
    0x4482474D, // Large 'vsm3|msg2'.
    0x2283774D, // Large 'vsm3rnd|s2'.
    0x123478A5, // Large 'vsm4key|4'.
    0x12348754, // Large 'vsm4rnds|4'.
    0x33CA49E7, // Large 'vsqr|tpd'.
    0x33D649E7, // Large 'vsqr|tph'.
    0x33E149E7, // Large 'vsqr|tps'.
    0x33E849E7, // Large 'vsqr|tsd'.
    0x33F149E7, // Large 'vsqr|tsh'.
    0x33FA49E7, // Large 'vsqr|tss'.
    0x122878AC, // Large 'vstmxcs|r'.
    0x89015676, // Small 'vsubpd'.
    0x91015676, // Small 'vsubph'.
    0xA7015676, // Small 'vsubps'.
    0x89315676, // Small 'vsubsd'.
    0x91315676, // Small 'vsubsh'.
    0xA7315676, // Small 'vsubss'.
    0x33CA49EB, // Large 'vtes|tpd'.
    0x33E149EB, // Large 'vtes|tps'.
    0x231D68B3, // Large 'vucomi|sd'.
    0x22DA68B3, // Large 'vucomi|sh'.
    0x222268B3, // Large 'vucomi|ss'.
    0x22AC775C, // Large 'vunpckh|pd'.
    0x2282775C, // Large 'vunpckh|ps'.
    0x3763675C, // Large 'vunpck|lpd'.
    0x3766675C, // Large 'vunpck|lps'.
    0x89093F16, // Small 'vxorpd'.
    0xA7093F16, // Small 'vxorps'.
    0x38B95598, // Large 'vzero|all'.
    0x353B7598, // Large 'vzeroup|per'.
    0x89672457, // Small 'wbinvd'.
    0x242168BC, // Large 'wbnoin|vd'.
    0x317058C2, // Large 'wrfsb|ase'.
    0x317058C7, // Large 'wrgsb|ase'.
    0x8129B657, // Small 'wrmsr'.
    0x8049CE57, // Small 'wrssd'.
    0x8119CE57, // Small 'wrssq'.
    0x8939D657, // Small 'wrussd'.
    0xA339D657, // Small 'wrussq'.
    0xA9278838, // Small 'xabort'.
    0x80021038, // Small 'xadd'.
    0x9C939458, // Small 'xbegin'.
    0x8003A078, // Small 'xchg'.
    0x800238B8, // Small 'xend'.
    0xAC2A14F8, // Small 'xgetbv'.
    0x802A0598, // Small 'xlatb'.
    0x800049F8, // Small 'xor'.
    0x804849F8, // Small 'xorpd'.
    0x813849F8, // Small 'xorps'.
    0x121B8769, // Large 'xresldtr|k'.
    0xA4FA4E58, // Small 'xrstor'.
    0x223565DD, // Large 'xrstor|64'.
    0x115265DD, // Large 'xrstor|s'.
    0x377165DD, // Large 'xrstor|s64'.
    0x805B0678, // Small 'xsave'.
    0x2235559F, // Large 'xsave|64'.
    0x865B0678, // Small 'xsavec'.
    0x38CC559F, // Large 'xsave|c64'.
    0x0000859F, // Large 'xsaveopt'.
    0x2235859F, // Large 'xsaveopt|64'.
    0xA65B0678, // Small 'xsaves'.
    0x3771559F, // Large 'xsave|s64'.
    0xAC2A1678, // Small 'xsetbv'.
    0x121B8774, // Large 'xsusldtr|k'.
    0x81499698 // Small 'xtest'.
];

pub static ALIAS_NAME_STRING_TABLE: &[u8] = b"cmovnaege";

pub static ALIAS_NAME_INDEX_TABLE: &[u32] = &[
    0x801B3DA3, // Small 'cmova'.
    0x8A1B3DA3, // Small 'cmovae'.
    0x803B3DA3, // Small 'cmovc'.
    0x805B3DA3, // Small 'cmove'.
    0x807B3DA3, // Small 'cmovg'.
    0x8A7B3DA3, // Small 'cmovge'.
    0x82EB3DA3, // Small 'cmovna'.
    0x00007000, // Large 'cmovnae'.
    0x86EB3DA3, // Small 'cmovnc'.
    0x8AEB3DA3, // Small 'cmovne'.
    0x8EEB3DA3, // Small 'cmovng'.
    0x20075000, // Large 'cmovn|ge'.
    0x8B0B3DA3, // Small 'cmovpe'.
    0x9F0B3DA3, // Small 'cmovpo'.
    0x8000002A, // Small 'ja'.
    0x8000142A, // Small 'jae'.
    0x8000006A, // Small 'jc'.
    0x800000AA, // Small 'je'.
    0x800000EA, // Small 'jg'.
    0x800014EA, // Small 'jge'.
    0x800005CA, // Small 'jna'.
    0x800285CA, // Small 'jnae'.
    0x80000DCA, // Small 'jnc'.
    0x800015CA, // Small 'jne'.
    0x80001DCA, // Small 'jng'.
    0x80029DCA, // Small 'jnge'.
    0x8000160A, // Small 'jpe'.
    0x80003E0A, // Small 'jpo'.
    0x80003033, // Small 'sal'.
    0x8000D0B3, // Small 'seta'.
    0x8050D0B3, // Small 'setae'.
    0x8001D0B3, // Small 'setc'.
    0x8002D0B3, // Small 'sete'.
    0x8003D0B3, // Small 'setg'.
    0x8053D0B3, // Small 'setge'.
    0x801750B3, // Small 'setna'.
    0x8A1750B3, // Small 'setnae'.
    0x803750B3, // Small 'setnc'.
    0x805750B3, // Small 'setne'.
    0x807750B3, // Small 'setng'.
    0x8A7750B3, // Small 'setnge'.
    0x805850B3, // Small 'setpe'.
    0x80F850B3, // Small 'setpo'.
    0x800A2437 // Small 'wait'.
];

pub static ALIAS_INDEX_TO_INST_ID_TABLE: &[u32] = &[
    InstId::Cmovnbe as u32, // #0
    InstId::Cmovnb as u32, // #1
    InstId::Cmovb as u32, // #2
    InstId::Cmovz as u32, // #3
    InstId::Cmovnle as u32, // #4
    InstId::Cmovnl as u32, // #5
    InstId::Cmovbe as u32, // #6
    InstId::Cmovb as u32, // #7
    InstId::Cmovnb as u32, // #8
    InstId::Cmovnz as u32, // #9
    InstId::Cmovle as u32, // #10
    InstId::Cmovl as u32, // #11
    InstId::Cmovp as u32, // #12
    InstId::Cmovnp as u32, // #13
    InstId::Jnbe as u32, // #14
    InstId::Jnb as u32, // #15
    InstId::Jb as u32, // #16
    InstId::Jz as u32, // #17
    InstId::Jnle as u32, // #18
    InstId::Jnl as u32, // #19
    InstId::Jbe as u32, // #20
    InstId::Jb as u32, // #21
    InstId::Jnb as u32, // #22
    InstId::Jnz as u32, // #23
    InstId::Jle as u32, // #24
    InstId::Jl as u32, // #25
    InstId::Jp as u32, // #26
    InstId::Jnp as u32, // #27
    InstId::Shl as u32, // #28
    InstId::Setnbe as u32, // #29
    InstId::Setnb as u32, // #30
    InstId::Setb as u32, // #31
    InstId::Setz as u32, // #32
    InstId::Setnle as u32, // #33
    InstId::Setnl as u32, // #34
    InstId::Setbe as u32, // #35
    InstId::Setb as u32, // #36
    InstId::Setnb as u32, // #37
    InstId::Setnz as u32, // #38
    InstId::Setle as u32, // #39
    InstId::Setl as u32, // #40
    InstId::Setp as u32, // #41
    InstId::Setnp as u32, // #42
    InstId::Fwait as u32 // #43
];

pub const ALIAS_TABLE_SIZE: u32 = 44;

/// Instruction signatures, see [`CommonInfo::signature_index`].
pub static INST_SIGNATURE_TABLE: &[InstSignature] = &[
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #0   {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [3, 4, 0, 0, 0, 0]), // {r16|m16|mem|sreg, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [5, 6, 0, 0, 0, 0]), // {r32|m32|mem|sreg, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [7, 8, 0, 0, 0, 0]), // {r64|m64|mem|sreg|creg|dreg, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [9, 10, 0, 0, 0, 0]), // {r8lo|r8hi|m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [11, 12, 0, 0, 0, 0]), // {r16|m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [13, 14, 0, 0, 0, 0]), // {r32|m32, i32|u32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 16, 0, 0, 0, 0]), // {r64|m64, i32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 17, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 18, 0, 0, 0, 0]), // {r16, m16|mem|sreg}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 19, 0, 0, 0, 0]), // {r32, m32|mem|sreg}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 20, 0, 0, 0, 0]), // {r64, m64|mem|i64|u64|sreg|creg|dreg}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [21, 22, 0, 0, 0, 0]), // {m16|mem, sreg}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [21, 22, 0, 0, 0, 0]), // {m16|mem, sreg}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [21, 22, 0, 0, 0, 0]), // {m16|mem, sreg}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [22, 21, 0, 0, 0, 0]), // {sreg, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [22, 21, 0, 0, 0, 0]), // {sreg, m16|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [22, 21, 0, 0, 0, 0]), // {sreg, m16|mem}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [6, 23, 0, 0, 0, 0]), // {r32, creg|dreg}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [23, 6, 0, 0, 0, 0]), // {creg|dreg, r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #20  {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [9, 10, 0, 0, 0, 0]), // {r8lo|r8hi|m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [27, 28, 0, 0, 0, 0]), // {r16|m16|r32|m32, i8}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 29, 0, 0, 0, 0]), // {r64|m64, i8|i32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [11, 12, 0, 0, 0, 0]), // {r16|m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [13, 14, 0, 0, 0, 0]), // {r32|m32, i32|u32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 17, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 21, 0, 0, 0, 0]), // {r16, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 30, 0, 0, 0, 0]), // {r32, m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 31, 0, 0, 0, 0]), // {r64, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [9, 10, 0, 0, 0, 0]), // #33  {r8lo|r8hi|m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [11, 12, 0, 0, 0, 0]), // {r16|m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [13, 14, 0, 0, 0, 0]), // {r32|m32, i32|u32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 32, 0, 0, 0, 0]), // {r64, u32|i32|i8|r64|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [33, 29, 0, 0, 0, 0]), // {m64, i32|i8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [27, 28, 0, 0, 0, 0]), // {r16|m16|r32|m32, i8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [31, 8, 0, 0, 0, 0]), // #42  {m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 17, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 21, 0, 0, 0, 0]), // {r16, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 30, 0, 0, 0, 0]), // {r32, m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #46  {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 34, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem|i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 35, 0, 0, 0, 0]), // {r16, m16|mem|i8|i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 36, 0, 0, 0, 0]), // {r32, m32|mem|i8|i32|u32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 37, 0, 0, 0, 0]), // {r64, m64|mem|i8|i32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [38, 10, 0, 0, 0, 0]), // {m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [39, 28, 0, 0, 0, 0]), // {m16|m32, i8}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [33, 29, 0, 0, 0, 0]), // {m64, i8|i32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [40, 12, 0, 0, 0, 0]), // {m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [41, 14, 0, 0, 0, 0]), // {m32, i32|u32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #59  {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [9, 10, 0, 0, 0, 0]), // {r8lo|r8hi|m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [11, 12, 0, 0, 0, 0]), // {r16|m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [13, 14, 0, 0, 0, 0]), // {r32|m32, i32|u32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 29, 0, 0, 0, 0]), // {r64|m64, i32|i8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [27, 28, 0, 0, 0, 0]), // {r16|m16|r32|m32, i8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 17, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 21, 0, 0, 0, 0]), // {r16, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 30, 0, 0, 0, 0]), // {r32, m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 31, 0, 0, 0, 0]), // {r64, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 1, 0, [42, 1, 0, 0, 0, 0]), // #72  {<ax>, r8lo|r8hi|m8|mem}
    InstSignature::new(3, Mode::Any as u8, 2, 0, [43, 42, 24, 0, 0, 0]), // {<dx>, <ax>, r16|m16|mem}
    InstSignature::new(3, Mode::Any as u8, 2, 0, [44, 45, 25, 0, 0, 0]), // {<edx>, <eax>, r32|m32|mem}
    InstSignature::new(3, Mode::X64 as u8, 2, 0, [46, 47, 26, 0, 0, 0]), // {<rdx>, <rax>, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 24, 0, 0, 0, 0]), // #76  {r16, r16|m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 25, 0, 0, 0, 0]), // #77  {r32, r32|m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 26, 0, 0, 0, 0]), // {r64, r64|m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [4, 24, 48, 0, 0, 0]), // {r16, r16|m16|mem, i8|i16|u16}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 25, 49, 0, 0, 0]), // {r32, r32|m32|mem, i8|i32|u32}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [8, 26, 29, 0, 0, 0]), // {r64, r64|m64|mem, i8|i32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 50, 0, 0, 0, 0]), // #82  {r64, i64|u64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [51, 17, 0, 0, 0, 0]), // {al, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [52, 21, 0, 0, 0, 0]), // {ax, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [53, 30, 0, 0, 0, 0]), // {eax, m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [54, 31, 0, 0, 0, 0]), // {rax, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [17, 51, 0, 0, 0, 0]), // {m8|mem, al}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [21, 52, 0, 0, 0, 0]), // {m16|mem, ax}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [30, 53, 0, 0, 0, 0]), // {m32|mem, eax}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [31, 54, 0, 0, 0, 0]), // {m64|mem, rax}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #91  {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [9, 10, 0, 0, 0, 0]), // {r8lo|r8hi|m8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [11, 12, 0, 0, 0, 0]), // {r16|m16, i16|u16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [13, 14, 0, 0, 0, 0]), // {r32|m32, i32|u32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 16, 0, 0, 0, 0]), // {r64|m64, i32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 56, 0, 0, 0, 0]), // #99  {xmm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 58, 0, 0, 0, 0]), // {ymm, ymm|m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // #101 {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [60, 57, 0, 0, 0, 0]), // {m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 62, 0, 0, 0, 0]), // {zmm, zmm|m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [60, 57, 0, 0, 0, 0]), // {m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [63, 61, 0, 0, 0, 0]), // {m512|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // #107 {m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 31, 0, 0, 0, 0]), // {xmm, m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // #109 {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // {m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 31, 0, 0, 0, 0]), // {xmm, m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [30, 55, 0, 0, 0, 0]), // #115 {m32|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 30, 0, 0, 0, 0]), // {xmm, m32|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [30, 55, 0, 0, 0, 0]), // {m32|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 30, 0, 0, 0, 0]), // {xmm, m32|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 64, 0, 0, 0]), // #123 {xmm, xmm, xmm|m128|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 59, 10, 0, 0, 0]), // {xmm, m128|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 65, 0, 0, 0]), // {ymm, ymm, ymm|m256|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 60, 10, 0, 0, 0]), // {ymm, m256|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 61, 66, 0, 0, 0]), // {zmm, zmm, zmm|m512|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 59, 10, 0, 0, 0]), // {xmm, m128|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 60, 10, 0, 0, 0]), // {ymm, m256|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 63, 10, 0, 0, 0]), // {zmm, m512|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [1, 2, 0, 0, 0, 0]), // #131 {r8lo|r8hi|m8|mem, r8lo|r8hi}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [2, 17, 0, 0, 0, 0]), // {r8lo|r8hi, m8|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 21, 0, 0, 0, 0]), // {r16, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 30, 0, 0, 0, 0]), // {r32, m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 31, 0, 0, 0, 0]), // {r64, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 21, 0, 0, 0, 0]), // #139 {r16, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 30, 0, 0, 0, 0]), // {r32, m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 31, 0, 0, 0, 0]), // {r64, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [21, 4, 0, 0, 0, 0]), // {m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [30, 6, 0, 0, 0, 0]), // #143 {m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [31, 8, 0, 0, 0, 0]), // {m64|mem, r64}
    InstSignature::new(0, Mode::Any as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #145 {}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [27, 0, 0, 0, 0, 0]), // {r16|m16|r32|m32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [15, 0, 0, 0, 0, 0]), // {r64|m64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 56, 0, 0, 0, 0]), // #151 {xmm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 58, 0, 0, 0, 0]), // {ymm, ymm|m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 62, 0, 0, 0, 0]), // {zmm, zmm|m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [60, 57, 0, 0, 0, 0]), // {m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [63, 61, 0, 0, 0, 0]), // {m512|mem, zmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 64, 0, 0, 0]), // #157 {xmm, xmm, xmm|m128|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 64, 0, 0, 0]), // {ymm, ymm, xmm|m128|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 61, 64, 0, 0, 0]), // {zmm, zmm, xmm|m128|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 59, 10, 0, 0, 0]), // {xmm, m128|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 60, 10, 0, 0, 0]), // {ymm, m256|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 63, 10, 0, 0, 0]), // {zmm, m512|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // #163 {r16|m16|mem, r16}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [27, 10, 0, 0, 0, 0]), // {r16|m16|r32|m32, i8|u8}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 10, 0, 0, 0, 0]), // {r64|m64, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 68, 0, 0, 0, 0]), // #168 {mm, mm|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [69, 26, 0, 0, 0, 0]), // {mm|xmm, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 69, 0, 0, 0, 0]), // {m64|mem, mm|xmm}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 69, 0, 0, 0, 0]), // {r64|m64|mem, mm|xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 70, 0, 0, 0, 0]), // #172 {xmm, xmm|m64|mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [11, 0, 0, 0, 0, 0]), // #173 {r16|m16}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [13, 0, 0, 0, 0, 0]), // {r32|m32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [15, 0, 0, 0, 0, 0]), // {r64|m64}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [71, 0, 0, 0, 0, 0]), // {ds|es|ss}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [72, 0, 0, 0, 0, 0]), // {fs|gs}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [73, 0, 0, 0, 0, 0]), // #178 {r16|m16|i8|u8|i16|u16}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [74, 0, 0, 0, 0, 0]), // {r32|m32|i32|u32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [75, 0, 0, 0, 0, 0]), // {r64|m64|i32}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [76, 0, 0, 0, 0, 0]), // {cs|ss|ds|es}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [72, 0, 0, 0, 0, 0]), // {fs|gs}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 77, 55, 0, 0, 0]), // #183 {xmm, vm32x, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 78, 57, 0, 0, 0]), // {ymm, vm32y, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 77, 0, 0, 0, 0]), // {xmm, vm32x}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 78, 0, 0, 0, 0]), // {ymm, vm32y}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 79, 0, 0, 0, 0]), // {zmm, vm32z}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 80, 55, 0, 0, 0]), // #188 {xmm, vm64x, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 81, 57, 0, 0, 0]), // {ymm, vm64y, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 80, 0, 0, 0, 0]), // {xmm, vm64x}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 81, 0, 0, 0, 0]), // {ymm, vm64y}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 82, 0, 0, 0, 0]), // {zmm, vm64z}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // #193 {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [60, 57, 0, 0, 0, 0]), // {m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [60, 57, 0, 0, 0, 0]), // {m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [63, 61, 0, 0, 0, 0]), // {m512|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 59, 0, 0, 0, 0]), // #198 {xmm, m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 60, 0, 0, 0, 0]), // {ymm, m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 59, 0, 0, 0, 0]), // {xmm, m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 60, 0, 0, 0, 0]), // {ymm, m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 63, 0, 0, 0, 0]), // {zmm, m512|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 55, 0, 0, 0, 0]), // #203 {r64|m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 70, 0, 0, 0, 0]), // {xmm, xmm|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [55, 26, 0, 0, 0, 0]), // {xmm, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // {m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // {m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [83, 84, 0, 0, 0, 0]), // #208 {ds:[memBase|zsi|m8], es:[memBase|zdi|m8]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [85, 86, 0, 0, 0, 0]), // {ds:[memBase|zsi|m16], es:[memBase|zdi|m16]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [87, 88, 0, 0, 0, 0]), // {ds:[memBase|zsi|m32], es:[memBase|zdi|m32]}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [89, 90, 0, 0, 0, 0]), // {ds:[memBase|zsi|m64], es:[memBase|zdi|m64]}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [1, 2, 91, 0, 0, 0]), // #212 {r8lo|r8hi|m8|mem, r8lo|r8hi, <al>}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [24, 4, 42, 0, 0, 0]), // {r16|m16|mem, r16, <ax>}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [25, 6, 45, 0, 0, 0]), // {r32|m32|mem, r32, <eax>}
    InstSignature::new(3, Mode::X64 as u8, 1, 0, [26, 8, 47, 0, 0, 0]), // {r64|m64|mem, r64, <rax>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 93, 0, 0, 0, 0]), // #216 {k, k|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [92, 8, 0, 0, 0, 0]), // {k, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 92, 0, 0, 0, 0]), // {m64|mem, k}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 92, 0, 0, 0, 0]), // {r64, k}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [51, 94, 0, 0, 0, 0]), // #220 {al, ds:[memBase|zsi|m8|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [52, 95, 0, 0, 0, 0]), // {ax, ds:[memBase|zsi|m16|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [53, 96, 0, 0, 0, 0]), // {eax, ds:[memBase|zsi|m32|mem]}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [54, 97, 0, 0, 0, 0]), // {rax, ds:[memBase|zsi|m64|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [84, 83, 0, 0, 0, 0]), // #224 {es:[memBase|zdi|m8], ds:[memBase|zsi|m8]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [86, 85, 0, 0, 0, 0]), // {es:[memBase|zdi|m16], ds:[memBase|zsi|m16]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [88, 87, 0, 0, 0, 0]), // {es:[memBase|zdi|m32], ds:[memBase|zsi|m32]}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [90, 89, 0, 0, 0, 0]), // {es:[memBase|zdi|m64], ds:[memBase|zsi|m64]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [51, 98, 0, 0, 0, 0]), // #228 {al, es:[memBase|zdi|m8|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [52, 99, 0, 0, 0, 0]), // {ax, es:[memBase|zdi|m16|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [53, 100, 0, 0, 0, 0]), // {eax, es:[memBase|zdi|m32|mem]}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [54, 101, 0, 0, 0, 0]), // {rax, es:[memBase|zdi|m64|mem]}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [98, 51, 0, 0, 0, 0]), // #232 {es:[memBase|zdi|m8|mem], al}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [99, 52, 0, 0, 0, 0]), // {es:[memBase|zdi|m16|mem], ax}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [100, 53, 0, 0, 0, 0]), // {es:[memBase|zdi|m32|mem], eax}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [101, 54, 0, 0, 0, 0]), // {es:[memBase|zdi|m64|mem], rax}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 55, 56, 0, 0]), // #236 {xmm, xmm, xmm, xmm|m128|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 57, 58, 0, 0]), // {ymm, ymm, ymm, ymm|m256|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 59, 55, 0, 0]), // {xmm, xmm, m128|mem, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 60, 57, 0, 0]), // {ymm, ymm, m256|mem, ymm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 77, 55, 0, 0, 0]), // #240 {xmm, vm32x, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 77, 57, 0, 0, 0]), // {ymm, vm32x, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [102, 77, 0, 0, 0, 0]), // {xmm|ymm, vm32x}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 78, 0, 0, 0, 0]), // {zmm, vm32y}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [59, 55, 55, 0, 0, 0]), // #244 {m128|mem, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [60, 57, 57, 0, 0, 0]), // {m256|mem, ymm, ymm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 59, 0, 0, 0]), // {xmm, xmm, m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 60, 0, 0, 0]), // {ymm, ymm, m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // #248 {m64|mem, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 31, 0, 0, 0]), // {xmm, xmm, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // {m64|mem, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 31, 0, 0, 0]), // {xmm, xmm, m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [21, 55, 0, 0, 0, 0]), // #252 {m16|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 21, 0, 0, 0, 0]), // {xmm, m16|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 55, 0, 0, 0]), // {xmm, xmm, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 55, 56, 0, 0]), // #256 {xmm, xmm, xmm, xmm|m128|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 59, 55, 0, 0]), // {xmm, xmm, m128|mem, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 57, 58, 0, 0]), // {ymm, ymm, ymm, ymm|m256|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 60, 57, 0, 0]), // {ymm, ymm, m256|mem, ymm}
    InstSignature::new(5, Mode::Any as u8, 0, 0, [55, 55, 56, 55, 103, 0]), // #260 {xmm, xmm, xmm|m128|mem, xmm, i4|u4}
    InstSignature::new(5, Mode::Any as u8, 0, 0, [55, 55, 55, 59, 103, 0]), // {xmm, xmm, xmm, m128|mem, i4|u4}
    InstSignature::new(5, Mode::Any as u8, 0, 0, [57, 57, 58, 57, 103, 0]), // {ymm, ymm, ymm|m256|mem, ymm, i4|u4}
    InstSignature::new(5, Mode::Any as u8, 0, 0, [57, 57, 57, 60, 103, 0]), // {ymm, ymm, ymm, m256|mem, i4|u4}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 58, 10, 0, 0, 0]), // #264 {ymm, ymm|m256|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 58, 0, 0, 0]), // {ymm, ymm, ymm|m256|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 61, 66, 0, 0, 0]), // {zmm, zmm, zmm|m512|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 63, 10, 0, 0, 0]), // {zmm, m512|mem, i8|u8}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [104, 0, 0, 0, 0, 0]), // #268 {rel16|r16|m16|mem|r32|m32}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [105, 0, 0, 0, 0, 0]), // #269 {rel32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [26, 0, 0, 0, 0, 0]), // {r64|m64|mem}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [106, 0, 0, 0, 0, 0]), // #271 {r16|r32}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [107, 0, 0, 0, 0, 0]), // #272 {r8lo|r8hi|m8|r16|m16|r32|m32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [15, 0, 0, 0, 0, 0]), // {r64|m64}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [108, 0, 0, 0, 0, 0]), // #274 {m32|m64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [109, 110, 0, 0, 0, 0]), // {st0, st}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [110, 109, 0, 0, 0, 0]), // {st, st0}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [111, 0, 0, 0, 0, 0]), // #277 {rel8|rel32}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [112, 0, 0, 0, 0, 0]), // {rel16|r32|m32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [15, 0, 0, 0, 0, 0]), // {r64|m64}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [12, 113, 0, 0, 0, 0]), // #280 {i16|u16, i16|u16|i32|u32}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [114, 0, 0, 0, 0, 0]), // {m32|mem|m48}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [115, 0, 0, 0, 0, 0]), // {m80|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 30, 0, 0, 0, 0]), // #283 {r16, m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 116, 0, 0, 0, 0]), // {r32, m48|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 115, 0, 0, 0, 0]), // {r64, m80|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 24, 0, 0, 0, 0]), // #286 {r16, r16|m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 117, 0, 0, 0, 0]), // {r32, r32|m16|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 117, 0, 0, 0, 0]), // {r64, r32|m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [4, 9, 0, 0, 0, 0]), // #289 {r16, r8lo|r8hi|m8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 118, 0, 0, 0, 0]), // {r32, r8lo|r8hi|m8|r16|m16}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 118, 0, 0, 0, 0]), // {r64, r8lo|r8hi|m8|r16|m16}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [24, 4, 119, 0, 0, 0]), // #292 {r16|m16|mem, r16, cl|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [25, 6, 119, 0, 0, 0]), // {r32|m32|mem, r32, cl|i8|u8}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [26, 8, 119, 0, 0, 0]), // {r64|m64|mem, r64, cl|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 56, 0, 0, 0]), // #295 {xmm, xmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 58, 0, 0, 0]), // #296 {ymm, ymm, ymm|m256|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 61, 62, 0, 0, 0]), // {zmm, zmm, zmm|m512|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 56, 10, 0, 0]), // #298 {xmm, xmm, xmm|m128|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 58, 10, 0, 0]), // #299 {ymm, ymm, ymm|m256|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [61, 61, 62, 10, 0, 0]), // {zmm, zmm, zmm|m512|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [120, 55, 56, 10, 0, 0]), // #301 {xmm|k, xmm, xmm|m128|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [121, 57, 58, 10, 0, 0]), // {ymm|k, ymm, ymm|m256|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 61, 62, 10, 0, 0]), // {k, zmm, zmm|m512|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 55, 56, 10, 0, 0]), // #304 {k, xmm, xmm|m128|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 57, 58, 10, 0, 0]), // {k, ymm, ymm|m256|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 61, 62, 10, 0, 0]), // {k, zmm, zmm|m512|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [56, 55, 0, 0, 0, 0]), // #307 {xmm|m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [58, 57, 0, 0, 0, 0]), // {ymm|m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [62, 61, 0, 0, 0, 0]), // {zmm|m512|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 70, 0, 0, 0, 0]), // #310 {xmm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 56, 0, 0, 0, 0]), // {ymm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 58, 0, 0, 0, 0]), // {zmm, ymm|m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 122, 0, 0, 0, 0]), // #313 {xmm, xmm|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 70, 0, 0, 0, 0]), // {ymm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 56, 0, 0, 0, 0]), // {zmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [70, 55, 10, 0, 0, 0]), // #316 {xmm|m64|mem, xmm, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [56, 57, 10, 0, 0, 0]), // #317 {xmm|m128|mem, ymm, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [58, 61, 10, 0, 0, 0]), // #318 {ymm|m256|mem, zmm, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 123, 55, 0, 0, 0]), // #319 {xmm, vm64x|vm64y, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 123, 0, 0, 0, 0]), // {xmm, vm64x|vm64y}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 82, 0, 0, 0, 0]), // {ymm, vm64z}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 56, 10, 0, 0, 0]), // #322 {xmm, xmm|m128|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 58, 10, 0, 0, 0]), // {ymm, ymm|m256|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [61, 62, 10, 0, 0, 0]), // {zmm, zmm|m512|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 70, 0, 0, 0, 0]), // #325 {xmm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 58, 0, 0, 0, 0]), // {ymm, ymm|m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 62, 0, 0, 0, 0]), // {zmm, zmm|m512|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 92, 55, 56, 0, 0]), // #328 {k, k, xmm, xmm|m128|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 92, 57, 58, 0, 0]), // {k, k, ymm, ymm|m256|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 92, 61, 62, 0, 0]), // {k, k, zmm, zmm|m512|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [120, 55, 56, 0, 0, 0]), // #331 {xmm|k, xmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [121, 57, 58, 0, 0, 0]), // {ymm|k, ymm, ymm|m256|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 61, 62, 0, 0, 0]), // {k, zmm, zmm|m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [122, 55, 0, 0, 0, 0]), // #334 {xmm|m32|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [70, 57, 0, 0, 0, 0]), // {xmm|m64|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [56, 61, 0, 0, 0, 0]), // {xmm|m128|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [70, 55, 0, 0, 0, 0]), // #337 {xmm|m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [56, 57, 0, 0, 0, 0]), // {xmm|m128|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [58, 61, 0, 0, 0, 0]), // {ymm|m256|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [124, 55, 0, 0, 0, 0]), // #340 {xmm|m16|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [122, 57, 0, 0, 0, 0]), // {xmm|m32|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [70, 61, 0, 0, 0, 0]), // {xmm|m64|mem, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 124, 0, 0, 0, 0]), // #343 {xmm, xmm|m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 122, 0, 0, 0, 0]), // {ymm, xmm|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 70, 0, 0, 0, 0]), // {zmm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [77, 55, 0, 0, 0, 0]), // #346 {vm32x, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [78, 57, 0, 0, 0, 0]), // {vm32y, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [79, 61, 0, 0, 0, 0]), // {vm32z, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [80, 55, 0, 0, 0, 0]), // #349 {vm64x, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [81, 57, 0, 0, 0, 0]), // {vm64y, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [82, 61, 0, 0, 0, 0]), // {vm64z, zmm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 55, 56, 0, 0, 0]), // #352 {k, xmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 57, 58, 0, 0, 0]), // {k, ymm, ymm|m256|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 61, 62, 0, 0, 0]), // {k, zmm, zmm|m512|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 6, 25, 0, 0, 0]), // #355 {r32, r32, r32|m32|mem}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [8, 8, 26, 0, 0, 0]), // {r64, r64, r64|m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 25, 6, 0, 0, 0]), // #357 {r32, r32|m32|mem, r32}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [8, 26, 8, 0, 0, 0]), // {r64, r64|m64|mem, r64}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [125, 25, 0, 0, 0, 0]), // #359 {bnd, r32|m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [125, 26, 0, 0, 0, 0]), // {bnd, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [125, 126, 0, 0, 0, 0]), // #361 {bnd, bnd|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [127, 125, 0, 0, 0, 0]), // {mem, bnd}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [4, 30, 0, 0, 0, 0]), // #363 {r16, m32|mem}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [6, 31, 0, 0, 0, 0]), // {r32, m64|mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [106, 0, 0, 0, 0, 0]), // #365 {r16|r32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [8, 0, 0, 0, 0, 0]), // #366 {r64}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [30, 6, 6, 0, 0, 0]), // #367 {m32|mem, r32, r32}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [31, 8, 8, 0, 0, 0]), // {m64|mem, r64, r64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 107, 0, 0, 0, 0]), // #369 {r32, r8lo|r8hi|m8|r16|m16|r32|m32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 128, 0, 0, 0, 0]), // {r64, r8lo|r8hi|m8|r64|m64}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 70, 0, 0, 0, 0]), // #371 {r32, xmm|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 70, 0, 0, 0, 0]), // {r64, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 25, 0, 0, 0, 0]), // #373 {xmm, r32|m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [55, 26, 0, 0, 0, 0]), // {xmm, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 122, 0, 0, 0, 0]), // #375 {r32, xmm|m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 122, 0, 0, 0, 0]), // {r64, xmm|m32|mem}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [129, 63, 0, 0, 0, 0]), // #377 {es:[mem|m512|memBase], m512|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [129, 63, 0, 0, 0, 0]), // {es:[mem|m512|memBase], m512|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 10, 10, 0, 0, 0]), // #379 {xmm, i8|u8, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 55, 0, 0, 0, 0]), // #380 {xmm, xmm}
    InstSignature::new(0, Mode::Any as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #381 {}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [110, 0, 0, 0, 0, 0]), // #382 {st}
    InstSignature::new(0, Mode::Any as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #383 {}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [130, 0, 0, 0, 0, 0]), // #384 {m32|m64|st}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 55, 0, 0, 0, 0]), // #385 {xmm, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 10, 10, 0, 0]), // {xmm, xmm, i8|u8, i8|u8}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [6, 59, 0, 0, 0, 0]), // #387 {r32, m128|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 59, 0, 0, 0, 0]), // {r64, m128|mem}
    InstSignature::new(2, Mode::X86 as u8, 2, 0, [45, 131, 0, 0, 0, 0]), // #389 {<eax>, <ecx>}
    InstSignature::new(2, Mode::X64 as u8, 2, 0, [132, 131, 0, 0, 0, 0]), // {<eax|rax>, <ecx>}
    InstSignature::new(3, Mode::X86 as u8, 3, 0, [45, 44, 131, 0, 0, 0]), // #391 {<eax>, <edx>, <ecx>}
    InstSignature::new(3, Mode::X64 as u8, 3, 0, [132, 44, 131, 0, 0, 0]), // {<eax|rax>, <edx>, <ecx>}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [111, 0, 0, 0, 0, 0]), // #393 {rel8|rel32}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [105, 0, 0, 0, 0, 0]), // {rel16}
    InstSignature::new(2, Mode::X86 as u8, 1, 0, [133, 134, 0, 0, 0, 0]), // #395 {<cx|ecx>, rel8}
    InstSignature::new(2, Mode::X64 as u8, 1, 0, [135, 134, 0, 0, 0, 0]), // {<ecx|rcx>, rel8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 136, 0, 0, 0, 0]), // #397 {k, k|m8|mem|r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [137, 92, 0, 0, 0, 0]), // {m8|mem|r32, k}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 138, 0, 0, 0, 0]), // #399 {k, k|m32|mem|r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 92, 0, 0, 0, 0]), // {m32|mem|r32, k}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 139, 0, 0, 0, 0]), // #401 {k, k|m16|mem|r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [117, 92, 0, 0, 0, 0]), // {m16|mem|r32, k}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [4, 30, 0, 0, 0, 0]), // #403 {r16, m32|mem}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [6, 116, 0, 0, 0, 0]), // {r32, m48|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [106, 140, 0, 0, 0, 0]), // #405 {r16|r32, mem|m8|m16|m32|m48|m64|m80|m128|m256|m512|m1024}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 140, 0, 0, 0, 0]), // {r64, mem|m8|m16|m32|m48|m64|m80|m128|m256|m512|m1024}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [6, 0, 0, 0, 0, 0]), // #407 {r32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [8, 0, 0, 0, 0, 0]), // {r64}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 25, 14, 0, 0, 0]), // #409 {r32, r32|m32|mem, i32|u32}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [8, 25, 14, 0, 0, 0]), // {r64, r32|m32|mem, i32|u32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 56, 0, 0, 0, 0]), // #411 {xmm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [59, 55, 0, 0, 0, 0]), // {m128|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [69, 25, 0, 0, 0, 0]), // #413 {mm|xmm, r32|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 69, 0, 0, 0, 0]), // {r32|m32|mem, mm|xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [129, 63, 0, 0, 0, 0]), // #415 {es:[mem|m512|memBase], m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [129, 63, 0, 0, 0, 0]), // {es:[mem|m512|memBase], m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 70, 0, 0, 0, 0]), // #417 {xmm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 55, 0, 0, 0, 0]), // {m64|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 122, 0, 0, 0, 0]), // #419 {xmm, xmm|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [30, 55, 0, 0, 0, 0]), // {m32|mem, xmm}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [4, 24, 0, 0, 0, 0]), // #421 {r16, r16|m16|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [141, 25, 0, 0, 0, 0]), // {r32|r64, r32|m32|mem}
    InstSignature::new(4, Mode::Any as u8, 1, 0, [6, 6, 25, 44, 0, 0]), // #423 {r32, r32, r32|m32|mem, <edx>}
    InstSignature::new(4, Mode::X64 as u8, 1, 0, [8, 8, 26, 46, 0, 0]), // {r64, r64, r64|m64|mem, <rdx>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 68, 0, 0, 0, 0]), // #425 {mm, mm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 56, 0, 0, 0, 0]), // {xmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [67, 68, 10, 0, 0, 0]), // #427 {mm, mm|m64|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 56, 10, 0, 0, 0]), // {xmm, xmm|m128|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 69, 10, 0, 0, 0]), // #429 {r32, mm|xmm, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [21, 55, 10, 0, 0, 0]), // {m16|mem, xmm, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 142, 0, 0, 0, 0]), // #431 {mm, mm|m64|mem|i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 64, 0, 0, 0, 0]), // {xmm, xmm|m128|mem|i8|u8}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [25, 0, 0, 0, 0, 0]), // #433 {r32|m32|mem}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [26, 0, 0, 0, 0, 0]), // {r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 143, 0, 0, 0, 0]), // #435 {mm, mm|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 56, 0, 0, 0, 0]), // {xmm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [107, 119, 0, 0, 0, 0]), // #437 {r8lo|r8hi|m8|r16|m16|r32|m32, cl|i8|u8}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [15, 119, 0, 0, 0, 0]), // {r64|m64, cl|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 3, 0, [44, 45, 131, 0, 0, 0]), // #439 {<edx>, <eax>, <ecx>}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 14, 0, 0, 0, 0]), // {r64, i32|u32}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [6, 0, 0, 0, 0, 0]), // #441 {r32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [8, 0, 0, 0, 0, 0]), // {r64}
    InstSignature::new(0, Mode::Any as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #443 {}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [144, 0, 0, 0, 0, 0]), // {u16}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [6, 25, 10, 0, 0, 0]), // #445 {r32, r32|m32|mem, i8|u8}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [8, 26, 10, 0, 0, 0]), // {r64, r64|m64|mem, i8|u8}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [145, 0, 0, 0, 0, 0]), // #447 {r16|m16|mem|r32}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [146, 0, 0, 0, 0, 0]), // {r64|m16|mem}
    InstSignature::new(1, Mode::X86 as u8, 0, 0, [147, 0, 0, 0, 0, 0]), // #449 {ds:[mem|memBase]}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [147, 0, 0, 0, 0, 0]), // {ds:[mem|memBase]}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 56, 55, 0, 0]), // #451 {xmm, xmm, xmm|m128|mem, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 58, 57, 0, 0]), // {ymm, ymm, ymm|m256|mem, ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 148, 0, 0, 0, 0]), // #453 {xmm, xmm|m128|ymm|m256}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 62, 0, 0, 0, 0]), // {ymm, zmm|m512|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 124, 0, 0, 0, 0]), // #455 {r32, xmm|m16|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 124, 0, 0, 0, 0]), // {r64, xmm|m16|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 25, 0, 0, 0]), // #457 {xmm, xmm, r32|m32|mem}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [55, 55, 26, 0, 0, 0]), // {xmm, xmm, r64|m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 13, 0, 0, 0]), // #459 {xmm, xmm, r32|m32}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [55, 55, 15, 0, 0, 0]), // {xmm, xmm, r64|m64}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 55, 70, 0, 0]), // #461 {xmm, xmm, xmm, xmm|m64|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 31, 55, 0, 0]), // {xmm, xmm, m64|mem, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 55, 122, 0, 0]), // #463 {xmm, xmm, xmm, xmm|m32|mem}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 30, 55, 0, 0]), // {xmm, xmm, m32|mem, xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [57, 57, 56, 10, 0, 0]), // #465 {ymm, ymm, xmm|m128|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [61, 61, 56, 10, 0, 0]), // {zmm, zmm, xmm|m128|mem, i8|u8}
    InstSignature::new(1, Mode::X86 as u8, 1, 0, [45, 0, 0, 0, 0, 0]), // #467 {<eax>}
    InstSignature::new(1, Mode::X64 as u8, 1, 0, [47, 0, 0, 0, 0, 0]), // #468 {<rax>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [25, 55, 0, 0, 0, 0]), // #469 {r32|m32|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 25, 0, 0, 0, 0]), // {xmm, r32|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [117, 55, 0, 0, 0, 0]), // #471 {r32|m16|mem, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 117, 0, 0, 0, 0]), // {xmm, r32|m16|mem}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [25, 6, 0, 0, 0, 0]), // #473 {r32|m32|mem, r32}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [26, 8, 0, 0, 0, 0]), // {r64|m64|mem, r64}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [6, 25, 0, 0, 0, 0]), // #475 {r32, r32|m32|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [8, 26, 0, 0, 0, 0]), // {r64, r64|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 70, 0, 0, 0, 0]), // #477 {xmm|ymm|zmm, xmm|m64|mem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [149, 8, 0, 0, 0, 0]), // {xmm|ymm|zmm, r64}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 64, 0, 0, 0]), // #479 {xmm, xmm, xmm|m128|mem|i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 59, 150, 0, 0, 0]), // {xmm, m128|mem, i8|u8|xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [77, 102, 0, 0, 0, 0]), // #481 {vm32x, xmm|ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [78, 61, 0, 0, 0, 0]), // {vm32y, zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [123, 55, 0, 0, 0, 0]), // #483 {vm64x|vm64y, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [82, 57, 0, 0, 0, 0]), // {vm64z, ymm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 56, 0, 0, 0]), // #485 {xmm, xmm, xmm|m128|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 59, 55, 0, 0, 0]), // {xmm, m128|mem, xmm}
    InstSignature::new(1, Mode::X86 as u8, 1, 0, [42, 0, 0, 0, 0, 0]), // #487 {<ax>}
    InstSignature::new(2, Mode::X86 as u8, 1, 0, [42, 10, 0, 0, 0, 0]), // #488 {<ax>, i8|u8}
    InstSignature::new(2, Mode::X86 as u8, 0, 0, [24, 4, 0, 0, 0, 0]), // #489 {r16|m16|mem, r16}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [55, 56, 151, 0, 0, 0]), // #490 {xmm, xmm|m128|mem, <xmm0>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [125, 152, 0, 0, 0, 0]), // #491 {bnd, mib}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [125, 127, 0, 0, 0, 0]), // #492 {bnd, mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [152, 125, 0, 0, 0, 0]), // #493 {mib, bnd}
    InstSignature::new(1, Mode::Any as u8, 1, 0, [42, 0, 0, 0, 0, 0]), // #494 {<ax>}
    InstSignature::new(2, Mode::Any as u8, 2, 0, [44, 45, 0, 0, 0, 0]), // #495 {<edx>, <eax>}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [127, 0, 0, 0, 0, 0]), // #496 {mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [31, 0, 0, 0, 0, 0]), // #497 {m64|mem}
    InstSignature::new(0, Mode::X64 as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #498 {}
    InstSignature::new(1, Mode::Any as u8, 1, 0, [153, 0, 0, 0, 0, 0]), // #499 {<ds:[mem|m512|memBase|zax]>}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 70, 10, 0, 0, 0]), // #500 {xmm, xmm|m64|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 122, 10, 0, 0, 0]), // #501 {xmm, xmm|m32|mem, i8|u8}
    InstSignature::new(5, Mode::X64 as u8, 4, 0, [59, 46, 47, 154, 155, 0]), // #502 {m128|mem, <rdx>, <rax>, <rcx>, <rbx>}
    InstSignature::new(5, Mode::Any as u8, 4, 0, [31, 44, 45, 131, 156, 0]), // #503 {m64|mem, <edx>, <eax>, <ecx>, <ebx>}
    InstSignature::new(4, Mode::Any as u8, 4, 0, [45, 156, 131, 44, 0, 0]), // #504 {<eax>, <ebx>, <ecx>, <edx>}
    InstSignature::new(2, Mode::X64 as u8, 2, 0, [46, 47, 0, 0, 0, 0]), // #505 {<rdx>, <rax>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 56, 0, 0, 0, 0]), // #506 {mm, xmm|m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 68, 0, 0, 0, 0]), // #507 {xmm, mm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 70, 0, 0, 0, 0]), // #508 {mm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 2, 0, [43, 42, 0, 0, 0, 0]), // #509 {<dx>, <ax>}
    InstSignature::new(1, Mode::Any as u8, 1, 0, [45, 0, 0, 0, 0, 0]), // #510 {<eax>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [12, 10, 0, 0, 0, 0]), // #511 {i16|u16, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [25, 55, 10, 0, 0, 0]), // #512 {r32|m32|mem, xmm, i8|u8}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [115, 0, 0, 0, 0, 0]), // #513 {m80|mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [39, 0, 0, 0, 0, 0]), // #514 {m16|m32}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [157, 0, 0, 0, 0, 0]), // #515 {m16|m32|m64}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [158, 0, 0, 0, 0, 0]), // #516 {m32|m64|m80|st}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [21, 0, 0, 0, 0, 0]), // #517 {m16|mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [159, 0, 0, 0, 0, 0]), // #518 {ax|m16|mem}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [127, 0, 0, 0, 0, 0]), // #519 {mem}
    InstSignature::new(2, Mode::Any as u8, 2, 0, [45, 156, 0, 0, 0, 0]), // #520 {<eax>, <ebx>}
    InstSignature::new(2, Mode::Any as u8, 1, 0, [10, 45, 0, 0, 0, 0]), // #521 {i8|u8, <eax>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [160, 161, 0, 0, 0, 0]), // #522 {al|ax|eax, i8|u8|dx}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [162, 163, 0, 0, 0, 0]), // #523 {es:[memBase|zdi|m8|m16|m32], dx}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [10, 0, 0, 0, 0, 0]), // #524 {i8|u8}
    InstSignature::new(0, Mode::X86 as u8, 0, 0, [0, 0, 0, 0, 0, 0]), // #525 {}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 92, 92, 0, 0, 0]), // #526 {k, k, k}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 92, 0, 0, 0, 0]), // #527 {k, k}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 92, 10, 0, 0, 0]), // #528 {k, k, i8|u8}
    InstSignature::new(1, Mode::Any as u8, 1, 0, [164, 0, 0, 0, 0, 0]), // #529 {<ah>}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [30, 0, 0, 0, 0, 0]), // #530 {m32|mem}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [63, 0, 0, 0, 0, 0]), // #531 {m512|mem}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [24, 0, 0, 0, 0, 0]), // #532 {r16|m16|mem}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [55, 55, 165, 0, 0, 0]), // #533 {xmm, xmm, <ds:[mem|m128|memBase|zdi]>}
    InstSignature::new(3, Mode::Any as u8, 1, 0, [67, 67, 166, 0, 0, 0]), // #534 {mm, mm, <ds:[mem|m64|memBase|zdi]>}
    InstSignature::new(3, Mode::Any as u8, 3, 0, [167, 131, 44, 0, 0, 0]), // #535 {<ds:[mem|memBase|zax]>, <ecx>, <edx>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [67, 55, 0, 0, 0, 0]), // #536 {mm, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 55, 0, 0, 0, 0]), // #537 {r32, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [31, 67, 0, 0, 0, 0]), // #538 {m64|mem, mm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 67, 0, 0, 0, 0]), // #539 {xmm, mm}
    InstSignature::new(2, Mode::Any as u8, 2, 0, [45, 131, 0, 0, 0, 0]), // #540 {<eax>, <ecx>}
    InstSignature::new(3, Mode::Any as u8, 3, 0, [45, 131, 156, 0, 0, 0]), // #541 {<eax>, <ecx>, <ebx>}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [161, 160, 0, 0, 0, 0]), // #542 {i8|u8|dx, al|ax|eax}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [163, 168, 0, 0, 0, 0]), // #543 {dx, ds:[memBase|zsi|m8|m16|m32]}
    InstSignature::new(6, Mode::Any as u8, 3, 0, [55, 56, 10, 131, 45, 44]), // #544 {xmm, xmm|m128|mem, i8|u8, <ecx>, <eax>, <edx>}
    InstSignature::new(6, Mode::Any as u8, 3, 0, [55, 56, 10, 151, 45, 44]), // #545 {xmm, xmm|m128|mem, i8|u8, <xmm0>, <eax>, <edx>}
    InstSignature::new(4, Mode::Any as u8, 1, 0, [55, 56, 10, 131, 0, 0]), // #546 {xmm, xmm|m128|mem, i8|u8, <ecx>}
    InstSignature::new(4, Mode::Any as u8, 1, 0, [55, 56, 10, 151, 0, 0]), // #547 {xmm, xmm|m128|mem, i8|u8, <xmm0>}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [137, 55, 10, 0, 0, 0]), // #548 {r32|m8|mem, xmm, i8|u8}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [26, 55, 10, 0, 0, 0]), // #549 {r64|m64|mem, xmm, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 137, 10, 0, 0, 0]), // #550 {xmm, r32|m8|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 25, 10, 0, 0, 0]), // #551 {xmm, r32|m32|mem, i8|u8}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [55, 26, 10, 0, 0, 0]), // #552 {xmm, r64|m64|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [69, 117, 10, 0, 0, 0]), // #553 {mm|xmm, r32|m16|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 69, 0, 0, 0, 0]), // #554 {r32, mm|xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 10, 0, 0, 0, 0]), // #555 {xmm, i8|u8}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [12, 0, 0, 0, 0, 0]), // #556 {i16|u16}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [141, 0, 0, 0, 0, 0]), // #557 {r32|r64}
    InstSignature::new(1, Mode::Any as u8, 0, 0, [1, 0, 0, 0, 0, 0]), // #558 {r8lo|r8hi|m8|mem}
    InstSignature::new(3, Mode::X64 as u8, 0, 0, [169, 169, 169, 0, 0, 0]), // #559 {tmm, tmm, tmm}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [169, 170, 0, 0, 0, 0]), // #560 {tmm, tmem}
    InstSignature::new(2, Mode::X64 as u8, 0, 0, [170, 169, 0, 0, 0, 0]), // #561 {tmem, tmm}
    InstSignature::new(1, Mode::X64 as u8, 0, 0, [169, 0, 0, 0, 0, 0]), // #562 {tmm}
    InstSignature::new(3, Mode::Any as u8, 2, 0, [6, 44, 45, 0, 0, 0]), // #563 {r32, <edx>, <eax>}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 70, 0, 0, 0]), // #564 {xmm, xmm, xmm|m64|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 124, 0, 0, 0]), // #565 {xmm, xmm, xmm|m16|mem}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [55, 55, 122, 0, 0, 0]), // #566 {xmm, xmm, xmm|m32|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [102, 21, 0, 0, 0, 0]), // #567 {xmm|ymm, m16|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 59, 0, 0, 0, 0]), // #568 {ymm, m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [171, 70, 0, 0, 0, 0]), // #569 {ymm|zmm, xmm|m64|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [171, 59, 0, 0, 0, 0]), // #570 {ymm|zmm, m128|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [61, 60, 0, 0, 0, 0]), // #571 {zmm, m256|mem}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 122, 0, 0, 0, 0]), // #572 {xmm|ymm|zmm, m32|mem|xmm}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [120, 55, 70, 10, 0, 0]), // #573 {xmm|k, xmm, xmm|m64|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [92, 55, 124, 10, 0, 0]), // #574 {k, xmm, xmm|m16|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [120, 55, 122, 10, 0, 0]), // #575 {xmm|k, xmm, xmm|m32|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [55, 172, 0, 0, 0, 0]), // #576 {xmm, xmm|m128|ymm|m256|zmm|m512}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [56, 171, 10, 0, 0, 0]), // #577 {xmm|m128|mem, ymm|zmm, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 70, 10, 0, 0]), // #578 {xmm, xmm, xmm|m64|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 122, 10, 0, 0]), // #579 {xmm, xmm, xmm|m32|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 172, 10, 0, 0, 0]), // #580 {k, xmm|m128|ymm|m256|zmm|m512, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 70, 10, 0, 0, 0]), // #581 {k, xmm|m64|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 124, 10, 0, 0, 0]), // #582 {k, xmm|m16|mem, i8|u8}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [92, 122, 10, 0, 0, 0]), // #583 {k, xmm|m32|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 124, 10, 0, 0]), // #584 {xmm, xmm, xmm|m16|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [61, 61, 58, 10, 0, 0]), // #585 {zmm, zmm, ymm|m256|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [6, 102, 0, 0, 0, 0]), // #586 {r32, xmm|ymm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 173, 0, 0, 0, 0]), // #587 {xmm|ymm|zmm, xmm|m8|mem|r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 174, 0, 0, 0, 0]), // #588 {xmm|ymm|zmm, xmm|m32|mem|r32}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 92, 0, 0, 0, 0]), // #589 {xmm|ymm|zmm, k}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [149, 175, 0, 0, 0, 0]), // #590 {xmm|ymm|zmm, xmm|m16|mem|r32}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [117, 55, 10, 0, 0, 0]), // #591 {r32|m16|mem, xmm, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 137, 10, 0, 0]), // #592 {xmm, xmm, r32|m8|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 25, 10, 0, 0]), // #593 {xmm, xmm, r32|m32|mem, i8|u8}
    InstSignature::new(4, Mode::X64 as u8, 0, 0, [55, 55, 26, 10, 0, 0]), // #594 {xmm, xmm, r64|m64|mem, i8|u8}
    InstSignature::new(4, Mode::Any as u8, 0, 0, [55, 55, 117, 10, 0, 0]), // #595 {xmm, xmm, r32|m16|mem, i8|u8}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [92, 149, 0, 0, 0, 0]), // #596 {k, xmm|ymm|zmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 55, 0, 0, 0, 0]), // #597 {ymm, xmm}
    InstSignature::new(2, Mode::Any as u8, 0, 0, [57, 57, 0, 0, 0, 0]), // #598 {ymm, ymm}
    InstSignature::new(3, Mode::Any as u8, 0, 0, [57, 57, 55, 0, 0, 0]), // #599 {ymm, ymm, xmm}
    InstSignature::new(3, Mode::Any as u8, 2, 0, [127, 44, 45, 0, 0, 0]), // #600 {mem, <edx>, <eax>}
    InstSignature::new(3, Mode::X64 as u8, 2, 0, [127, 44, 45, 0, 0, 0]), // #601 {mem, <edx>, <eax>}
];

/// Operand signatures, indexed by [`InstSignature::op_signature_indexes`].
pub static OP_SIGNATURE_TABLE: &[OpSignature] = &[
    OpSignature::new(0, 0xFF),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_S_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_S_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::REG_S_REG.bits() | OpFlags::REG_C_REG.bits() | OpFlags::REG_D_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I16.bits() | OpFlags::IMM_U16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits() | OpFlags::REG_C_REG.bits() | OpFlags::REG_D_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::IMM_I64.bits() | OpFlags::IMM_U64.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits(), 0x00),
    OpSignature::new(OpFlags::REG_C_REG.bits() | OpFlags::REG_D_REG.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I8.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I8.bits() | OpFlags::IMM_I32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_I16.bits() | OpFlags::IMM_U16.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_I32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::MEM16.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x04),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x04),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x04),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::IMM_I8.bits() | OpFlags::IMM_I16.bits() | OpFlags::IMM_U16.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I8.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I64.bits() | OpFlags::IMM_U64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPW.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPD.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPQ.bits(), 0x01),
    OpSignature::new(OpFlags::REG_XMM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM128.bits(), 0x00),
    OpSignature::new(OpFlags::REG_YMM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_YMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM256.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM128.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM256.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ZMM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ZMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM512.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM512.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM128.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_YMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM256.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ZMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM512.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_MM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_MM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_MM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits(), 0x1A),
    OpSignature::new(OpFlags::REG_S_REG.bits(), 0x60),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::MEM16.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits() | OpFlags::IMM_I16.bits() | OpFlags::IMM_U16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM32.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::MEM64.bits() | OpFlags::IMM_I32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_S_REG.bits(), 0x1E),
    OpSignature::new(OpFlags::VM32X.bits(), 0x00),
    OpSignature::new(OpFlags::VM32Y.bits(), 0x00),
    OpSignature::new(OpFlags::VM32Z.bits(), 0x00),
    OpSignature::new(OpFlags::VM64X.bits(), 0x00),
    OpSignature::new(OpFlags::VM64Y.bits(), 0x00),
    OpSignature::new(OpFlags::VM64Z.bits(), 0x00),
    OpSignature::new(OpFlags::MEM8.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM8.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM16.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM16.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM64.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM64.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::REG_K_REG.bits(), 0x00),
    OpSignature::new(OpFlags::REG_K_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_YMM.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I4.bits() | OpFlags::IMM_U4.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_I64.bits() | OpFlags::REL32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I32.bits() | OpFlags::IMM_I64.bits() | OpFlags::REL32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits() | OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits() | OpFlags::MEM8.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM32.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ST.bits(), 0x01),
    OpSignature::new(OpFlags::REG_ST.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I32.bits() | OpFlags::IMM_I64.bits() | OpFlags::REL8.bits() | OpFlags::REL32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM32.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_I64.bits() | OpFlags::REL32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_I16.bits() | OpFlags::IMM_U16.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_U32.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits() | OpFlags::MEM48.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM80.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM48.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits() | OpFlags::REG_GPW.bits() | OpFlags::MEM8.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x02),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_K_REG.bits(), 0x00),
    OpSignature::new(OpFlags::REG_YMM.bits() | OpFlags::REG_K_REG.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::VM64X.bits() | OpFlags::VM64Y.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_BND.bits(), 0x00),
    OpSignature::new(OpFlags::REG_BND.bits() | OpFlags::MEM_UNSPECIFIED.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPB_HI.bits() | OpFlags::REG_GPQ.bits() | OpFlags::MEM8.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM512.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ST.bits() | OpFlags::MEM32.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x02),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x02),
    OpSignature::new(OpFlags::IMM_I32.bits() | OpFlags::IMM_I64.bits() | OpFlags::REL8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x02),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_K_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_K_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_K_REG.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits() | OpFlags::MEM48.bits() | OpFlags::MEM64.bits() | OpFlags::MEM80.bits() | OpFlags::MEM128.bits() | OpFlags::MEM256.bits() | OpFlags::MEM512.bits() | OpFlags::MEM1024.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_GPQ.bits(), 0x00),
    OpSignature::new(OpFlags::REG_MM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_MM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::IMM_U16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_YMM.bits() | OpFlags::MEM128.bits() | OpFlags::MEM256.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_YMM.bits() | OpFlags::REG_ZMM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::FLAG_MIB.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM512.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x02),
    OpSignature::new(OpFlags::REG_GPQ.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x08),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x08),
    OpSignature::new(OpFlags::MEM16.bits() | OpFlags::MEM32.bits() | OpFlags::MEM64.bits(), 0x00),
    OpSignature::new(OpFlags::REG_ST.bits() | OpFlags::MEM32.bits() | OpFlags::MEM64.bits() | OpFlags::MEM80.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPB_LO.bits() | OpFlags::REG_GPW.bits() | OpFlags::REG_GPD.bits(), 0x01),
    OpSignature::new(OpFlags::REG_GPW.bits() | OpFlags::IMM_I8.bits() | OpFlags::IMM_U8.bits(), 0x04),
    OpSignature::new(OpFlags::MEM8.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_ES.bits(), 0x80),
    OpSignature::new(OpFlags::REG_GPW.bits(), 0x04),
    OpSignature::new(OpFlags::REG_GPB_HI.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM128.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x80),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM64.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x80),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits() | OpFlags::FLAG_IMPLICIT.bits(), 0x01),
    OpSignature::new(OpFlags::MEM8.bits() | OpFlags::MEM16.bits() | OpFlags::MEM32.bits() | OpFlags::FLAG_MEM_BASE.bits() | OpFlags::FLAG_MEM_DS.bits(), 0x40),
    OpSignature::new(OpFlags::REG_TMM.bits(), 0x00),
    OpSignature::new(OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::FLAG_T_MEM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_YMM.bits() | OpFlags::REG_ZMM.bits(), 0x00),
    OpSignature::new(OpFlags::REG_XMM.bits() | OpFlags::REG_YMM.bits() | OpFlags::REG_ZMM.bits() | OpFlags::MEM128.bits() | OpFlags::MEM256.bits() | OpFlags::MEM512.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM8.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM32.bits(), 0x00),
    OpSignature::new(OpFlags::REG_GPD.bits() | OpFlags::REG_XMM.bits() | OpFlags::MEM_UNSPECIFIED.bits() | OpFlags::MEM16.bits(), 0x00),
];

pub static RW_INFO_INDEX_A_TABLE: &[u8] = &[
    0, 0, 1, 2, 1, 2, 0, 3, 4, 3, 5, 5, 6, 7, 5, 5, 4, 5, 5, 5, 5, 8, 0, 3,
    0, 5, 5, 5, 5, 2, 9, 2, 0, 10, 10, 10, 10, 10, 0, 0, 0, 0, 10, 10, 10, 10, 10, 11,
    11, 11, 12, 12, 13, 14, 15, 10, 10, 0, 16, 17, 17, 17, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0,
    0, 0, 21, 22, 0, 23, 24, 25, 8, 26, 26, 26, 25, 27, 8, 25, 28, 29, 30, 31, 32, 33, 34, 26,
    26, 8, 28, 29, 34, 35, 0, 0, 0, 0, 36, 5, 5, 6, 7, 0, 0, 0, 0, 0, 37, 37, 0, 0,
    38, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    39, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 39, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 40, 0, 0, 5, 5, 5, 0, 41, 5, 5, 36, 42, 43, 0, 0, 0, 44, 0, 38, 0, 0,
    0, 0, 45, 0, 46, 0, 45, 45, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 49, 50, 51, 52, 53, 54,
    55, 0, 0, 0, 56, 57, 58, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 57, 58, 59, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 61, 0, 2, 0, 62, 0, 2, 0, 2, 0, 2, 0, 0,
    0, 0, 0, 63, 64, 64, 64, 60, 2, 0, 0, 0, 10, 0, 0, 5, 5, 6, 7, 0, 0, 5, 5, 6,
    7, 0, 0, 65, 66, 67, 67, 68, 49, 25, 37, 68, 54, 67, 67, 69, 70, 70, 71, 72, 72, 73, 73, 61,
    61, 68, 61, 61, 72, 72, 74, 50, 54, 75, 76, 8, 8, 77, 78, 10, 67, 67, 78, 0, 36, 5, 5, 6,
    7, 0, 79, 0, 0, 80, 0, 3, 5, 5, 81, 82, 10, 10, 10, 4, 4, 5, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 0, 4, 4, 0, 4, 83, 4, 0, 0, 0, 4, 4, 5, 4, 0, 0, 4, 4, 5, 4, 0,
    0, 0, 0, 0, 0, 0, 0, 84, 28, 28, 83, 83, 83, 83, 83, 83, 83, 83, 83, 83, 28, 83, 83, 83,
    28, 28, 83, 83, 83, 4, 4, 4, 85, 4, 4, 4, 28, 28, 0, 0, 0, 0, 4, 4, 5, 5, 4, 4,
    5, 5, 5, 5, 4, 4, 5, 5, 86, 87, 88, 25, 25, 25, 87, 87, 88, 25, 25, 25, 87, 5, 4, 83,
    4, 4, 5, 4, 4, 0, 0, 0, 10, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4,
    0, 0, 0, 0, 4, 4, 4, 4, 89, 4, 4, 0, 4, 4, 4, 89, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 28, 90, 0, 4, 4, 5, 4, 91, 91, 5, 91, 0, 0, 0, 0, 0, 0, 0, 0, 4, 92, 8,
    93, 92, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 92, 92, 0, 0, 0,
    0, 0, 0, 8, 93, 0, 0, 92, 0, 0, 3, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0, 5, 5, 0, 92, 0, 0, 92, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 27, 93, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 3, 5,
    5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 98, 98,
    0, 99, 0, 0, 0, 10, 10, 21, 22, 100, 100, 0, 0, 0, 0, 5, 5, 5, 5, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 101, 101, 0, 0, 0, 0, 0, 0, 102,
    29, 103, 104, 103, 104, 102, 29, 103, 104, 103, 104, 105, 106, 0, 0, 0, 0, 0, 0, 21, 107, 22, 108, 108,
    109, 110, 10, 0, 68, 68, 68, 68, 110, 110, 111, 110, 10, 110, 10, 109, 112, 109, 109, 112, 109, 112, 10, 10,
    10, 109, 0, 110, 109, 10, 109, 10, 113, 110, 0, 29, 0, 29, 0, 114, 0, 114, 0, 0, 0, 0, 0, 34,
    34, 110, 10, 110, 10, 109, 112, 109, 112, 10, 10, 10, 109, 10, 109, 29, 29, 114, 114, 34, 34, 109, 110, 10,
    10, 111, 110, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 28, 115, 2, 2, 2,
    116, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 117, 117, 49, 118, 117, 117, 117, 117, 117, 117, 117, 117, 0, 119, 119, 0, 72,
    72, 120, 121, 68, 68, 68, 68, 122, 72, 123, 10, 10, 74, 117, 117, 51, 0, 0, 0, 108, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 34, 126, 126,
    29, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 108, 108, 108, 108, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, 0, 2, 2, 116, 2, 8, 8, 8, 0, 8, 0,
    8, 8, 8, 8, 8, 8, 0, 8, 8, 85, 8, 0, 8, 0, 0, 8, 0, 0, 0, 0, 10, 10, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 127, 127, 128, 129, 126, 126, 126, 126, 86, 127, 130, 129, 128, 128, 129, 130,
    129, 128, 129, 112, 131, 109, 109, 109, 112, 128, 129, 130, 129, 128, 129, 127, 129, 112, 131, 109, 109, 109, 112, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 68, 68, 132, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 10,
    10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 10,
    10, 0, 0, 0, 0, 0, 0, 0, 0, 68, 68, 68, 132, 133, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 124, 21, 107, 22, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 72, 74, 72, 0, 135, 0, 136, 0, 0, 0, 3, 5, 5,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub static RW_INFO_INDEX_B_TABLE: &[u8] = &[
    0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 5, 6, 6, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 4, 8, 1, 0, 9, 0, 0, 0, 10, 10,
    10, 0, 0, 11, 0, 0, 10, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 5, 5, 13, 0, 14, 15, 13, 16, 17,
    18, 13, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 20, 1, 1, 21, 22, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 23,
    24, 0, 0, 25, 26, 27, 28, 0, 0, 26, 26, 26, 26, 26, 26, 26, 26, 29, 30, 30, 29, 0, 0, 0,
    25, 26, 25, 26, 0, 26, 25, 25, 25, 25, 25, 25, 25, 0, 0, 31, 31, 31, 25, 25, 29, 0, 32, 10,
    0, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 33, 34, 33, 35, 0, 0, 0, 0, 0, 10, 33, 0, 0,
    0, 0, 36, 34, 33, 36, 35, 25, 26, 25, 26, 0, 30, 30, 30, 30, 0, 0, 0, 26, 10, 10, 33, 33,
    0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 22, 37, 0, 21, 38, 38, 0, 39, 40, 0, 0,
    0, 0, 0, 10, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 42, 43, 44, 45, 42, 43, 42, 43, 44, 45, 44, 45, 0, 0, 0, 0, 0, 0, 0,
    0, 42, 43, 44, 0, 0, 0, 0, 45, 46, 47, 48, 49, 46, 47, 48, 49, 0, 0, 0, 0, 50, 51, 52,
    42, 43, 44, 45, 42, 43, 44, 45, 53, 0, 25, 0, 54, 0, 55, 0, 0, 0, 0, 0, 10, 0, 10, 25,
    56, 57, 56, 0, 0, 0, 0, 0, 0, 56, 58, 58, 0, 59, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 61, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 62, 0, 0, 0, 0, 0, 5, 63, 0, 0, 0,
    0, 64, 0, 65, 21, 66, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 6, 5, 5, 0, 0, 0, 0, 68, 69, 0, 0, 0, 0, 70,
    71, 0, 3, 3, 72, 23, 73, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 39, 76, 77, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0,
    2, 2, 2, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 80, 80, 81, 80, 81, 81, 81, 80, 80, 82, 83, 0, 84, 0, 0, 0, 0, 0, 0, 85, 2, 2,
    86, 87, 0, 0, 0, 11, 88, 0, 4, 0, 0, 0, 0, 0, 0, 89, 0, 90, 90, 90, 90, 90, 90, 90,
    90, 90, 90, 90, 90, 90, 90, 90, 0, 90, 0, 33, 0, 0, 0, 5, 0, 0, 6, 0, 91, 4, 0, 91,
    4, 5, 5, 33, 20, 92, 80, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 92, 94, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 95, 0, 95, 95, 95, 95, 95, 95, 0, 0, 0, 0,
    0, 0, 96, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 10, 97, 0, 0, 0, 0, 3, 3, 3, 98, 99,
    100, 3, 3, 3, 3, 3, 3, 0, 2, 3, 3, 3, 3, 3, 3, 0, 0, 3, 3, 3, 3, 101, 101, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 102, 3, 103, 104, 105, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 98, 0, 107, 0, 99, 0, 108, 0, 109, 110, 111, 112, 113, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 109, 110, 111, 0, 0, 3, 3, 3, 3, 98, 99, 100, 3, 114, 3, 56, 56, 0, 0, 115, 116,
    117, 116, 117, 115, 116, 117, 116, 117, 23, 118, 119, 118, 119, 120, 120, 121, 122, 120, 120, 120, 123, 124, 125, 120,
    120, 120, 123, 124, 125, 120, 120, 120, 123, 124, 125, 118, 119, 126, 126, 127, 128, 120, 120, 120, 120, 120, 120, 120,
    120, 120, 126, 126, 120, 120, 120, 123, 129, 125, 120, 120, 120, 123, 129, 125, 120, 120, 120, 123, 129, 125, 120, 120,
    120, 120, 120, 120, 120, 120, 120, 126, 126, 126, 126, 127, 128, 118, 130, 120, 120, 120, 123, 124, 125, 120, 120, 120,
    123, 124, 125, 120, 120, 120, 123, 124, 125, 126, 126, 127, 128, 120, 120, 120, 123, 129, 125, 120, 120, 120, 123, 129,
    125, 120, 120, 120, 131, 129, 132, 126, 126, 127, 128, 133, 133, 133, 79, 134, 135, 0, 0, 0, 0, 136, 137, 137,
    138, 0, 0, 0, 139, 140, 141, 85, 85, 85, 139, 140, 141, 3, 3, 3, 3, 3, 3, 3, 142, 143, 144, 143,
    144, 142, 143, 144, 143, 144, 100, 0, 54, 59, 145, 145, 3, 3, 3, 98, 99, 100, 0, 11, 0, 0, 3, 3,
    3, 98, 99, 100, 0, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 148, 148, 149, 150,
    150, 0, 0, 0, 0, 0, 0, 0, 151, 152, 0, 0, 153, 0, 0, 0, 3, 11, 154, 0, 0, 155, 146, 3,
    3, 3, 98, 99, 100, 0, 0, 11, 3, 3, 156, 156, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 101, 3, 0, 0, 0, 0,
    0, 0, 3, 126, 102, 102, 3, 3, 3, 3, 68, 69, 3, 3, 3, 3, 70, 71, 102, 102, 102, 102, 102, 102,
    114, 114, 0, 0, 0, 0, 114, 114, 114, 114, 114, 114, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
    120, 120, 120, 120, 120, 120, 157, 157, 3, 3, 120, 120, 3, 3, 120, 120, 126, 126, 158, 158, 158, 3, 158, 120,
    120, 120, 120, 120, 120, 3, 0, 0, 0, 0, 72, 23, 73, 159, 137, 136, 138, 137, 0, 0, 0, 3, 0, 3,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 3, 3, 0, 160, 100, 98, 99, 0, 0, 161, 161,
    161, 161, 161, 161, 161, 161, 161, 161, 161, 161, 120, 120, 3, 3, 145, 145, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 162, 85, 85, 3, 3, 85, 85, 3, 3,
    163, 163, 163, 163, 3, 0, 0, 0, 0, 163, 163, 163, 163, 163, 163, 3, 3, 120, 120, 120, 3, 163, 163, 3,
    3, 120, 120, 120, 3, 3, 102, 85, 85, 85, 3, 3, 3, 164, 165, 164, 3, 3, 3, 166, 164, 167, 3, 3,
    3, 166, 164, 165, 164, 3, 3, 3, 166, 3, 3, 3, 3, 3, 3, 3, 3, 168, 168, 0, 102, 102, 102, 102,
    102, 102, 102, 102, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 139, 141, 0, 0, 139, 141, 0,
    0, 140, 141, 85, 85, 85, 139, 140, 141, 85, 85, 85, 139, 140, 141, 85, 85, 139, 141, 0, 0, 139, 141, 0,
    0, 140, 141, 3, 3, 3, 98, 99, 100, 0, 0, 0, 0, 0, 0, 169, 3, 3, 3, 3, 3, 3, 170, 170,
    170, 3, 3, 0, 0, 0, 139, 140, 141, 93, 3, 3, 3, 98, 99, 100, 0, 0, 0, 0, 0, 3, 3, 3,
    3, 3, 3, 0, 0, 0, 0, 57, 57, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0,
    0, 172, 172, 172, 172, 173, 173, 173, 173, 173, 173, 173, 173, 171, 0, 0,
];

pub static RW_INFO_A_TABLE: &[RwInfo] = &[
    RwInfo::new(RwInfoCategory::Generic, 0, [0, 0, 0, 0, 0, 0]), // #0 [ref=999x]
    RwInfo::new(RwInfoCategory::Generic, 0, [1, 0, 0, 0, 0, 0]), // #1 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [2, 3, 0, 0, 0, 0]), // #2 [ref=15x]
    RwInfo::new(RwInfoCategory::Generic, 1, [2, 3, 0, 0, 0, 0]), // #3 [ref=7x]
    RwInfo::new(RwInfoCategory::Generic, 2, [2, 3, 0, 0, 0, 0]), // #4 [ref=82x]
    RwInfo::new(RwInfoCategory::Generic, 3, [4, 5, 0, 0, 0, 0]), // #5 [ref=55x]
    RwInfo::new(RwInfoCategory::Generic, 4, [6, 7, 0, 0, 0, 0]), // #6 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 5, [8, 9, 0, 0, 0, 0]), // #7 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 3, [10, 5, 0, 0, 0, 0]), // #8 [ref=26x]
    RwInfo::new(RwInfoCategory::Generic, 7, [12, 13, 0, 0, 0, 0]), // #9 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 2, [11, 3, 0, 0, 0, 0]), // #10 [ref=75x]
    RwInfo::new(RwInfoCategory::Generic, 2, [5, 3, 0, 0, 0, 0]), // #11 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 8, [10, 3, 0, 0, 0, 0]), // #12 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 9, [10, 5, 0, 0, 0, 0]), // #13 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [15, 5, 0, 0, 0, 0]), // #14 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [3, 3, 0, 0, 0, 0]), // #15 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 10, [3, 3, 0, 0, 0, 0]), // #16 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 10, [2, 3, 0, 0, 0, 0]), // #17 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [16, 17, 0, 0, 0, 0]), // #18 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 1, [3, 3, 0, 0, 0, 0]), // #19 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 14, [20, 21, 0, 0, 0, 0]), // #20 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 4, [7, 7, 0, 0, 0, 0]), // #21 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 5, [9, 9, 0, 0, 0, 0]), // #22 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 0, [33, 34, 0, 0, 0, 0]), // #23 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 16, [2, 3, 0, 0, 0, 0]), // #24 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 4, [10, 7, 0, 0, 0, 0]), // #25 [ref=10x]
    RwInfo::new(RwInfoCategory::Generic, 3, [35, 5, 0, 0, 0, 0]), // #26 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 4, [36, 7, 0, 0, 0, 0]), // #27 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 4, [35, 7, 0, 0, 0, 0]), // #28 [ref=11x]
    RwInfo::new(RwInfoCategory::Generic, 4, [11, 7, 0, 0, 0, 0]), // #29 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 4, [37, 7, 0, 0, 0, 0]), // #30 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 16, [36, 3, 0, 0, 0, 0]), // #31 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 16, [37, 3, 0, 0, 0, 0]), // #32 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [36, 9, 0, 0, 0, 0]), // #33 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [11, 9, 0, 0, 0, 0]), // #34 [ref=7x]
    RwInfo::new(RwInfoCategory::Generic, 0, [38, 39, 0, 0, 0, 0]), // #35 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 17, [1, 40, 0, 0, 0, 0]), // #36 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 13, [43, 44, 0, 0, 0, 0]), // #37 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [4, 5, 0, 0, 0, 0]), // #38 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [46, 47, 0, 0, 0, 0]), // #39 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [51, 30, 0, 0, 0, 0]), // #40 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [0, 51, 0, 0, 0, 0]), // #41 [ref=1x]
    RwInfo::new(RwInfoCategory::Imul, 2, [0, 0, 0, 0, 0, 0]), // #42 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [52, 53, 0, 0, 0, 0]), // #43 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 14, [54, 53, 0, 0, 0, 0]), // #44 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 15, [3, 5, 0, 0, 0, 0]), // #45 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [22, 29, 0, 0, 0, 0]), // #46 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [56, 0, 0, 0, 0, 0]), // #47 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 23, [57, 40, 0, 0, 0, 0]), // #48 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 24, [45, 9, 0, 0, 0, 0]), // #49 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 25, [35, 7, 0, 0, 0, 0]), // #50 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 26, [49, 13, 0, 0, 0, 0]), // #51 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [57, 40, 0, 0, 0, 0]), // #52 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [45, 9, 0, 0, 0, 0]), // #53 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 7, 0, 0, 0, 0]), // #54 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [49, 13, 0, 0, 0, 0]), // #55 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [40, 40, 0, 0, 0, 0]), // #56 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [9, 9, 0, 0, 0, 0]), // #57 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [7, 7, 0, 0, 0, 0]), // #58 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [13, 13, 0, 0, 0, 0]), // #59 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 27, [11, 3, 0, 0, 0, 0]), // #60 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 15, [10, 5, 0, 0, 0, 0]), // #61 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 8, [11, 3, 0, 0, 0, 0]), // #62 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [52, 20, 0, 0, 0, 0]), // #63 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [59, 0, 0, 0, 0, 0]), // #64 [ref=3x]
    RwInfo::new(RwInfoCategory::Mov, 29, [0, 0, 0, 0, 0, 0]), // #65 [ref=1x]
    RwInfo::new(RwInfoCategory::Movabs, 0, [0, 0, 0, 0, 0, 0]), // #66 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 30, [10, 5, 0, 0, 0, 0]), // #67 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [11, 3, 0, 0, 0, 0]), // #68 [ref=18x]
    RwInfo::new(RwInfoCategory::Generic, 0, [36, 63, 0, 0, 0, 0]), // #69 [ref=1x]
    RwInfo::new(RwInfoCategory::Movh64, 12, [0, 0, 0, 0, 0, 0]), // #70 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [64, 7, 0, 0, 0, 0]), // #71 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [35, 7, 0, 0, 0, 0]), // #72 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 0, [57, 5, 0, 0, 0, 0]), // #73 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 28, [45, 9, 0, 0, 0, 0]), // #74 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 14, [65, 20, 0, 0, 0, 0]), // #75 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 31, [35, 7, 0, 0, 0, 0]), // #76 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 33, [45, 9, 0, 0, 0, 0]), // #77 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 16, [11, 3, 0, 0, 0, 0]), // #78 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [17, 29, 0, 0, 0, 0]), // #79 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 11, [3, 3, 0, 0, 0, 0]), // #80 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [53, 22, 0, 0, 0, 0]), // #81 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 14, [53, 68, 0, 0, 0, 0]), // #82 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 4, [26, 7, 0, 0, 0, 0]), // #83 [ref=18x]
    RwInfo::new(RwInfoCategory::Generic, 36, [0, 0, 0, 0, 0, 0]), // #84 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 3, [71, 5, 0, 0, 0, 0]), // #85 [ref=2x]
    RwInfo::new(RwInfoCategory::Vmov1_8, 0, [0, 0, 0, 0, 0, 0]), // #86 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 5, [10, 9, 0, 0, 0, 0]), // #87 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 27, [10, 13, 0, 0, 0, 0]), // #88 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [4, 0, 0, 0, 0, 0]), // #89 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 3, [5, 5, 0, 0, 0, 0]), // #90 [ref=1x]
    RwInfo::new(RwInfoCategory::Punpcklxx, 38, [0, 0, 0, 0, 0, 0]), // #91 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 10, [2, 72, 0, 0, 0, 0]), // #92 [ref=7x]
    RwInfo::new(RwInfoCategory::Generic, 5, [37, 9, 0, 0, 0, 0]), // #93 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 0, 0, 0, 0, 0]), // #94 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [16, 51, 0, 0, 0, 0]), // #95 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [22, 21, 0, 0, 0, 0]), // #96 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [65, 22, 0, 0, 0, 0]), // #97 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [43, 3, 0, 0, 0, 0]), // #98 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 8, [11, 44, 0, 0, 0, 0]), // #99 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [76, 9, 0, 0, 0, 0]), // #100 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 21, [11, 13, 0, 0, 0, 0]), // #101 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 15, [77, 5, 0, 0, 0, 0]), // #102 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 15, [11, 5, 0, 0, 0, 0]), // #103 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 43, [43, 78, 0, 0, 0, 0]), // #104 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 44, [11, 7, 0, 0, 0, 0]), // #105 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 45, [11, 9, 0, 0, 0, 0]), // #106 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 27, [13, 13, 0, 0, 0, 0]), // #107 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 11, [11, 3, 0, 0, 0, 0]), // #108 [ref=7x]
    RwInfo::new(RwInfoCategory::Vmov2_1, 46, [0, 0, 0, 0, 0, 0]), // #109 [ref=19x]
    RwInfo::new(RwInfoCategory::Vmov1_2, 16, [0, 0, 0, 0, 0, 0]), // #110 [ref=11x]
    RwInfo::new(RwInfoCategory::Vmov1_4, 16, [0, 0, 0, 0, 0, 0]), // #111 [ref=2x]
    RwInfo::new(RwInfoCategory::Vmov4_1, 47, [0, 0, 0, 0, 0, 0]), // #112 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 16, [10, 3, 0, 0, 0, 0]), // #113 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 27, [11, 13, 0, 0, 0, 0]), // #114 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 5, [45, 9, 0, 0, 0, 0]), // #115 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 14, [2, 3, 0, 0, 0, 0]), // #116 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 57, [11, 3, 0, 0, 0, 0]), // #117 [ref=12x]
    RwInfo::new(RwInfoCategory::Vmovddup, 38, [0, 0, 0, 0, 0, 0]), // #118 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [35, 63, 0, 0, 0, 0]), // #119 [ref=2x]
    RwInfo::new(RwInfoCategory::Vmovmskpd, 0, [0, 0, 0, 0, 0, 0]), // #120 [ref=1x]
    RwInfo::new(RwInfoCategory::Vmovmskps, 0, [0, 0, 0, 0, 0, 0]), // #121 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 58, [35, 7, 0, 0, 0, 0]), // #122 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 21, [49, 13, 0, 0, 0, 0]), // #123 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 2, [3, 3, 0, 0, 0, 0]), // #124 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 17, [11, 40, 0, 0, 0, 0]), // #125 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [11, 7, 0, 0, 0, 0]), // #126 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 3, 0, 0, 0, 0]), // #127 [ref=4x]
    RwInfo::new(RwInfoCategory::Vmov1_4, 61, [0, 0, 0, 0, 0, 0]), // #128 [ref=6x]
    RwInfo::new(RwInfoCategory::Vmov1_2, 48, [0, 0, 0, 0, 0, 0]), // #129 [ref=9x]
    RwInfo::new(RwInfoCategory::Vmov1_8, 62, [0, 0, 0, 0, 0, 0]), // #130 [ref=3x]
    RwInfo::new(RwInfoCategory::Vmov8_1, 63, [0, 0, 0, 0, 0, 0]), // #131 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 14, [11, 3, 0, 0, 0, 0]), // #132 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [87, 5, 0, 0, 0, 0]), // #133 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [87, 78, 0, 0, 0, 0]), // #134 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 11, [2, 2, 0, 0, 0, 0]), // #135 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 57, [2, 2, 0, 0, 0, 0]), // #136 [ref=1x]
];

pub static RW_INFO_B_TABLE: &[RwInfo] = &[
    RwInfo::new(RwInfoCategory::Generic, 0, [0, 0, 0, 0, 0, 0]), // #0 [ref=758x]
    RwInfo::new(RwInfoCategory::Generic, 0, [1, 0, 0, 0, 0, 0]), // #1 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 3, [10, 5, 0, 0, 0, 0]), // #2 [ref=7x]
    RwInfo::new(RwInfoCategory::Generic, 6, [11, 3, 3, 0, 0, 0]), // #3 [ref=193x]
    RwInfo::new(RwInfoCategory::Generic, 2, [11, 3, 3, 0, 0, 0]), // #4 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 3, [4, 5, 0, 0, 0, 0]), // #5 [ref=14x]
    RwInfo::new(RwInfoCategory::Generic, 3, [4, 5, 14, 0, 0, 0]), // #6 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 0, [2, 0, 0, 0, 0, 0]), // #7 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 11, [3, 0, 0, 0, 0, 0]), // #8 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [18, 0, 0, 0, 0, 0]), // #9 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [3, 0, 0, 0, 0, 0]), // #10 [ref=21x]
    RwInfo::new(RwInfoCategory::Generic, 12, [7, 0, 0, 0, 0, 0]), // #11 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 13, [19, 0, 0, 0, 0, 0]), // #12 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [2, 2, 3, 0, 0, 0]), // #13 [ref=16x]
    RwInfo::new(RwInfoCategory::Generic, 4, [6, 7, 0, 0, 0, 0]), // #14 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [8, 9, 0, 0, 0, 0]), // #15 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 11, [2, 3, 22, 0, 0, 0]), // #16 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 15, [4, 23, 18, 24, 25, 0]), // #17 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [26, 27, 28, 29, 30, 0]), // #18 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [28, 31, 32, 16, 0, 0]), // #19 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [28, 0, 0, 0, 0, 0]), // #20 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 10, [2, 0, 0, 0, 0, 0]), // #21 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 6, [41, 42, 3, 0, 0, 0]), // #22 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 18, [45, 5, 0, 0, 0, 0]), // #23 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 0, [4, 0, 0, 0, 0, 0]), // #24 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 14, [3, 0, 0, 0, 0, 0]), // #25 [ref=17x]
    RwInfo::new(RwInfoCategory::Generic, 0, [46, 0, 0, 0, 0, 0]), // #26 [ref=16x]
    RwInfo::new(RwInfoCategory::Generic, 19, [47, 0, 0, 0, 0, 0]), // #27 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 19, [48, 0, 0, 0, 0, 0]), // #28 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 20, [3, 0, 0, 0, 0, 0]), // #29 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [47, 0, 0, 0, 0, 0]), // #30 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 14, [11, 0, 0, 0, 0, 0]), // #31 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 21, [13, 0, 0, 0, 0, 0]), // #32 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [11, 0, 0, 0, 0, 0]), // #33 [ref=8x]
    RwInfo::new(RwInfoCategory::Generic, 21, [49, 0, 0, 0, 0, 0]), // #34 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 7, [50, 0, 0, 0, 0, 0]), // #35 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 20, [11, 0, 0, 0, 0, 0]), // #36 [ref=2x]
    RwInfo::new(RwInfoCategory::Imul, 22, [0, 0, 0, 0, 0, 0]), // #37 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [40, 0, 0, 0, 0, 0]), // #38 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 5, [4, 9, 0, 0, 0, 0]), // #39 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [4, 5, 0, 0, 0, 0]), // #40 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [22, 55, 56, 0, 0, 0]), // #41 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [57, 40, 40, 0, 0, 0]), // #42 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [45, 9, 9, 0, 0, 0]), // #43 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 7, 7, 0, 0, 0]), // #44 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [49, 13, 13, 0, 0, 0]), // #45 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [57, 40, 0, 0, 0, 0]), // #46 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [45, 9, 0, 0, 0, 0]), // #47 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 7, 0, 0, 0, 0]), // #48 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [49, 13, 0, 0, 0, 0]), // #49 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [49, 40, 40, 0, 0, 0]), // #50 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 9, 9, 0, 0, 0]), // #51 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [45, 13, 13, 0, 0, 0]), // #52 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [58, 0, 0, 0, 0, 0]), // #53 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 28, [9, 0, 0, 0, 0, 0]), // #54 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 13, [44, 0, 0, 0, 0, 0]), // #55 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 7, [13, 0, 0, 0, 0, 0]), // #56 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 0, [3, 0, 0, 0, 0, 0]), // #57 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 5, [3, 9, 0, 0, 0, 0]), // #58 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 15, [5, 5, 60, 0, 0, 0]), // #59 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 12, [7, 7, 61, 0, 0, 0]), // #60 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [62, 29, 55, 0, 0, 0]), // #61 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 32, [0, 0, 0, 0, 0, 0]), // #62 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 6, [66, 42, 3, 0, 0, 0]), // #63 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 6, [11, 11, 3, 67, 0, 0]), // #64 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [17, 29, 30, 0, 0, 0]), // #65 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 10, [3, 0, 0, 0, 0, 0]), // #66 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 2, [2, 3, 0, 0, 0, 0]), // #67 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 3, [5, 5, 0, 69, 17, 55]), // #68 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 3, [5, 5, 0, 70, 17, 55]), // #69 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 3, [5, 5, 0, 69, 0, 0]), // #70 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 3, [5, 5, 0, 70, 0, 0]), // #71 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 34, [57, 5, 0, 0, 0, 0]), // #72 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 35, [35, 5, 0, 0, 0, 0]), // #73 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 37, [49, 3, 0, 0, 0, 0]), // #74 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 17, [4, 40, 0, 0, 0, 0]), // #75 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 4, [4, 7, 0, 0, 0, 0]), // #76 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 27, [2, 13, 0, 0, 0, 0]), // #77 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 10, [11, 0, 0, 0, 0, 0]), // #78 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 4, [35, 7, 0, 0, 0, 0]), // #79 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [11, 0, 0, 0, 0, 0]), // #80 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 0, [16, 51, 29, 0, 0, 0]), // #81 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 0, [45, 0, 0, 0, 0, 0]), // #82 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [35, 0, 0, 0, 0, 0]), // #83 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [16, 51, 69, 0, 0, 0]), // #84 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 2, [11, 3, 0, 0, 0, 0]), // #85 [ref=19x]
    RwInfo::new(RwInfoCategory::Generic, 4, [36, 7, 0, 0, 0, 0]), // #86 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [37, 9, 0, 0, 0, 0]), // #87 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [73, 0, 0, 0, 0, 0]), // #88 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [7, 0, 0, 0, 0, 0]), // #89 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 34, [74, 0, 0, 0, 0, 0]), // #90 [ref=16x]
    RwInfo::new(RwInfoCategory::Generic, 11, [2, 3, 72, 0, 0, 0]), // #91 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 39, [11, 0, 0, 0, 0, 0]), // #92 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 28, [45, 0, 0, 0, 0, 0]), // #93 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 13, [43, 0, 0, 0, 0, 0]), // #94 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [75, 44, 44, 0, 0, 0]), // #95 [ref=8x]
    RwInfo::new(RwInfoCategory::Generic, 0, [43, 0, 0, 0, 0, 0]), // #96 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [9, 55, 17, 0, 0, 0]), // #97 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 40, [10, 5, 7, 0, 0, 0]), // #98 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 41, [10, 5, 13, 0, 0, 0]), // #99 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 42, [10, 5, 9, 0, 0, 0]), // #100 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 6, [11, 3, 3, 3, 0, 0]), // #101 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 6, [35, 3, 3, 0, 0, 0]), // #102 [ref=18x]
    RwInfo::new(RwInfoCategory::Generic, 40, [11, 5, 7, 0, 0, 0]), // #103 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 41, [35, 13, 13, 0, 0, 0]), // #104 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 42, [11, 5, 9, 0, 0, 0]), // #105 [ref=1x]
    RwInfo::new(RwInfoCategory::Vmov1_2, 48, [0, 0, 0, 0, 0, 0]), // #106 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 40, [10, 79, 7, 0, 0, 0]), // #107 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 41, [10, 5, 5, 0, 0, 0]), // #108 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 49, [10, 63, 3, 0, 0, 0]), // #109 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 49, [10, 3, 3, 0, 0, 0]), // #110 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 49, [10, 79, 3, 0, 0, 0]), // #111 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 42, [10, 63, 9, 0, 0, 0]), // #112 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 42, [10, 5, 5, 0, 0, 0]), // #113 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 50, [10, 5, 5, 0, 0, 0]), // #114 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 51, [10, 78, 0, 0, 0, 0]), // #115 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 51, [10, 3, 0, 0, 0, 0]), // #116 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 52, [77, 44, 0, 0, 0, 0]), // #117 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 6, [80, 3, 3, 0, 0, 0]), // #118 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 42, [81, 5, 5, 0, 0, 0]), // #119 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 6, [2, 3, 3, 0, 0, 0]), // #120 [ref=90x]
    RwInfo::new(RwInfoCategory::Generic, 40, [4, 63, 7, 0, 0, 0]), // #121 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 42, [4, 79, 9, 0, 0, 0]), // #122 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 40, [6, 7, 7, 0, 0, 0]), // #123 [ref=11x]
    RwInfo::new(RwInfoCategory::Generic, 41, [82, 5, 5, 0, 0, 0]), // #124 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 42, [8, 9, 9, 0, 0, 0]), // #125 [ref=11x]
    RwInfo::new(RwInfoCategory::Generic, 53, [11, 3, 3, 3, 0, 0]), // #126 [ref=15x]
    RwInfo::new(RwInfoCategory::Generic, 54, [35, 7, 7, 7, 0, 0]), // #127 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 55, [45, 9, 9, 9, 0, 0]), // #128 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 41, [82, 5, 13, 0, 0, 0]), // #129 [ref=6x]
    RwInfo::new(RwInfoCategory::Generic, 42, [83, 5, 5, 0, 0, 0]), // #130 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 40, [26, 7, 7, 0, 0, 0]), // #131 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 42, [76, 9, 9, 0, 0, 0]), // #132 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 16, [35, 3, 0, 0, 0, 0]), // #133 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 27, [35, 13, 0, 0, 0, 0]), // #134 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 5, [35, 9, 0, 0, 0, 0]), // #135 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 8, [2, 3, 2, 0, 0, 0]), // #136 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [2, 3, 2, 0, 0, 0]), // #137 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 14, [4, 3, 4, 0, 0, 0]), // #138 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 40, [10, 63, 7, 0, 0, 0]), // #139 [ref=9x]
    RwInfo::new(RwInfoCategory::Generic, 41, [10, 84, 13, 0, 0, 0]), // #140 [ref=7x]
    RwInfo::new(RwInfoCategory::Generic, 42, [10, 79, 9, 0, 0, 0]), // #141 [ref=11x]
    RwInfo::new(RwInfoCategory::Generic, 50, [77, 78, 5, 0, 0, 0]), // #142 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 50, [11, 3, 5, 0, 0, 0]), // #143 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 56, [43, 44, 78, 0, 0, 0]), // #144 [ref=4x]
    RwInfo::new(RwInfoCategory::Vmaskmov, 0, [0, 0, 0, 0, 0, 0]), // #145 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 0, [22, 0, 0, 0, 0, 0]), // #146 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [10, 63, 63, 0, 0, 0]), // #147 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [10, 7, 7, 0, 0, 0]), // #148 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [10, 7, 7, 0, 0, 0]), // #149 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [10, 63, 7, 0, 0, 0]), // #150 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [10, 63, 7, 0, 0, 0]), // #151 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [10, 84, 13, 0, 0, 0]), // #152 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [10, 79, 9, 0, 0, 0]), // #153 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 12, [35, 0, 0, 0, 0, 0]), // #154 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 0, [85, 0, 0, 0, 0, 0]), // #155 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 59, [35, 86, 3, 3, 0, 0]), // #156 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 56, [77, 78, 78, 0, 0, 0]), // #157 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 22, [11, 3, 3, 0, 0, 0]), // #158 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 7, [49, 5, 0, 0, 0, 0]), // #159 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 60, [10, 5, 40, 0, 0, 0]), // #160 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 50, [10, 5, 5, 5, 0, 0]), // #161 [ref=12x]
    RwInfo::new(RwInfoCategory::Generic, 64, [10, 5, 5, 5, 0, 0]), // #162 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 65, [10, 5, 5, 0, 0, 0]), // #163 [ref=12x]
    RwInfo::new(RwInfoCategory::Generic, 66, [11, 3, 5, 0, 0, 0]), // #164 [ref=5x]
    RwInfo::new(RwInfoCategory::Generic, 67, [11, 3, 0, 0, 0, 0]), // #165 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 68, [11, 3, 5, 0, 0, 0]), // #166 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 22, [11, 3, 5, 0, 0, 0]), // #167 [ref=1x]
    RwInfo::new(RwInfoCategory::GenericEx, 6, [2, 3, 3, 0, 0, 0]), // #168 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 0, [87, 78, 5, 0, 0, 0]), // #169 [ref=1x]
    RwInfo::new(RwInfoCategory::Generic, 50, [4, 5, 5, 0, 0, 0]), // #170 [ref=3x]
    RwInfo::new(RwInfoCategory::Generic, 0, [55, 17, 29, 0, 0, 0]), // #171 [ref=2x]
    RwInfo::new(RwInfoCategory::Generic, 8, [3, 55, 17, 0, 0, 0]), // #172 [ref=4x]
    RwInfo::new(RwInfoCategory::Generic, 8, [11, 55, 17, 0, 0, 0]), // #173 [ref=8x]
];

pub static RW_INFO_OP_TABLE: &[RwInfoOp] = &[
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::NONE.bits()), // #0 [ref=16348x]
    RwInfoOp::new(0x0000000000000003, 0x0000000000000003, 0x00, 0, OpRwFlags::RW.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #1 [ref=10x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #2 [ref=267x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #3 [ref=1091x]
    RwInfoOp::new(0x000000000000FFFF, 0x000000000000FFFF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #4 [ref=93x]
    RwInfoOp::new(0x000000000000FFFF, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #5 [ref=338x]
    RwInfoOp::new(0x00000000000000FF, 0x00000000000000FF, 0xFF, 0, OpRwFlags::RW.bits()), // #6 [ref=18x]
    RwInfoOp::new(0x00000000000000FF, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #7 [ref=186x]
    RwInfoOp::new(0x000000000000000F, 0x000000000000000F, 0xFF, 0, OpRwFlags::RW.bits()), // #8 [ref=18x]
    RwInfoOp::new(0x000000000000000F, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #9 [ref=133x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000FFFF, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #10 [ref=178x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #11 [ref=445x]
    RwInfoOp::new(0x0000000000000003, 0x0000000000000003, 0xFF, 0, OpRwFlags::RW.bits()), // #12 [ref=1x]
    RwInfoOp::new(0x0000000000000003, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #13 [ref=71x]
    RwInfoOp::new(0x000000000000FFFF, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #14 [ref=4x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::MEM_BASE_WRITE.bits() | OpRwFlags::MEM_INDEX_WRITE.bits()), // #15 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0x02, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #16 [ref=9x]
    RwInfoOp::new(0x000000000000000F, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #17 [ref=23x]
    RwInfoOp::new(0x00000000000000FF, 0x00000000000000FF, 0x00, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #18 [ref=2x]
    RwInfoOp::new(0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #19 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x06, 0, OpRwFlags::READ.bits() | OpRwFlags::MEM_BASE_RW.bits() | OpRwFlags::MEM_BASE_POST_MODIFY.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #20 [ref=3x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x07, 0, OpRwFlags::READ.bits() | OpRwFlags::MEM_BASE_RW.bits() | OpRwFlags::MEM_BASE_POST_MODIFY.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #21 [ref=2x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #22 [ref=8x]
    RwInfoOp::new(0x00000000000000FF, 0x00000000000000FF, 0x02, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #23 [ref=1x]
    RwInfoOp::new(0x00000000000000FF, 0x0000000000000000, 0x01, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #24 [ref=1x]
    RwInfoOp::new(0x00000000000000FF, 0x0000000000000000, 0x03, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #25 [ref=1x]
    RwInfoOp::new(0x00000000000000FF, 0x00000000000000FF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #26 [ref=20x]
    RwInfoOp::new(0x000000000000000F, 0x000000000000000F, 0x02, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #27 [ref=1x]
    RwInfoOp::new(0x000000000000000F, 0x000000000000000F, 0x00, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #28 [ref=4x]
    RwInfoOp::new(0x000000000000000F, 0x0000000000000000, 0x01, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #29 [ref=13x]
    RwInfoOp::new(0x000000000000000F, 0x0000000000000000, 0x03, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #30 [ref=3x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0x03, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #31 [ref=1x]
    RwInfoOp::new(0x000000000000000F, 0x000000000000000F, 0x01, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #32 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x00000000000000FF, 0x02, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #33 [ref=1x]
    RwInfoOp::new(0x00000000000000FF, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #34 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x00000000000000FF, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #35 [ref=84x]
    RwInfoOp::new(0x0000000000000000, 0x00000000000000FF, 0xFF, 0, OpRwFlags::WRITE.bits()), // #36 [ref=6x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0xFF, 0, OpRwFlags::WRITE.bits()), // #37 [ref=6x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000003, 0x02, 0, OpRwFlags::WRITE.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #38 [ref=1x]
    RwInfoOp::new(0x0000000000000003, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #39 [ref=1x]
    RwInfoOp::new(0x0000000000000001, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #40 [ref=30x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x02, 0, OpRwFlags::RW.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #41 [ref=2x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x00, 0, OpRwFlags::RW.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #42 [ref=3x]
    RwInfoOp::new(0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #43 [ref=15x]
    RwInfoOp::new(0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #44 [ref=29x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #45 [ref=30x]
    RwInfoOp::new(0x00000000000003FF, 0x00000000000003FF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #46 [ref=22x]
    RwInfoOp::new(0x00000000000003FF, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #47 [ref=13x]
    RwInfoOp::new(0x0000000000000000, 0x00000000000003FF, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #48 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000003, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #49 [ref=17x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000003, 0x00, 0, OpRwFlags::WRITE.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #50 [ref=2x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0x00, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #51 [ref=9x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x00, 0, OpRwFlags::WRITE.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #52 [ref=2x]
    RwInfoOp::new(0x0000000000000003, 0x0000000000000000, 0x02, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #53 [ref=4x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x07, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #54 [ref=1x]
    RwInfoOp::new(0x000000000000000F, 0x0000000000000000, 0x02, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #55 [ref=23x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x01, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #56 [ref=2x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000001, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #57 [ref=14x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000001, 0x00, 0, OpRwFlags::WRITE.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #58 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x01, 0, OpRwFlags::RW.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #59 [ref=3x]
    RwInfoOp::new(0x000000000000FFFF, 0x000000000000FFFF, 0x07, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #60 [ref=2x]
    RwInfoOp::new(0x00000000000000FF, 0x00000000000000FF, 0x07, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #61 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #62 [ref=2x]
    RwInfoOp::new(0x000000000000FF00, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #63 [ref=21x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000FF00, 0xFF, 0, OpRwFlags::WRITE.bits()), // #64 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x07, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::MEM_BASE_RW.bits() | OpRwFlags::MEM_BASE_POST_MODIFY.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #65 [ref=2x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x02, 0, OpRwFlags::WRITE.bits() | OpRwFlags::REG_PHYS_ID.bits() | OpRwFlags::ZEXT.bits()), // #66 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x02, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #67 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x06, 0, OpRwFlags::READ.bits() | OpRwFlags::MEM_PHYS_ID.bits()), // #68 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000000F, 0x01, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #69 [ref=5x]
    RwInfoOp::new(0x0000000000000000, 0x000000000000FFFF, 0x00, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #70 [ref=4x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000007, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #71 [ref=2x]
    RwInfoOp::new(0x0000000000000001, 0x0000000000000000, 0x01, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #72 [ref=9x]
    RwInfoOp::new(0x0000000000000001, 0x0000000000000000, 0x00, 0, OpRwFlags::READ.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #73 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000001, 0xFF, 0, OpRwFlags::WRITE.bits()), // #74 [ref=16x]
    RwInfoOp::new(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #75 [ref=8x]
    RwInfoOp::new(0x000000000000000F, 0x000000000000000F, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #76 [ref=3x]
    RwInfoOp::new(0x0000000000000000, 0x00000000FFFFFFFF, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits()), // #77 [ref=10x]
    RwInfoOp::new(0x00000000FFFFFFFF, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #78 [ref=18x]
    RwInfoOp::new(0x000000000000FFF0, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #79 [ref=16x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::UNIQUE.bits() | OpRwFlags::ZEXT.bits()), // #80 [ref=4x]
    RwInfoOp::new(0x000000000000FFFF, 0x000000000000FFFF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::UNIQUE.bits()), // #81 [ref=3x]
    RwInfoOp::new(0x000000000000FFFF, 0x000000000000FFFF, 0xFF, 0, OpRwFlags::RW.bits()), // #82 [ref=12x]
    RwInfoOp::new(0x000000000000FFFF, 0x000000000000FFFF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::UNIQUE.bits() | OpRwFlags::ZEXT.bits()), // #83 [ref=1x]
    RwInfoOp::new(0x000000000000FFFC, 0x0000000000000000, 0xFF, 0, OpRwFlags::READ.bits()), // #84 [ref=8x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0x00, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::REG_PHYS_ID.bits()), // #85 [ref=1x]
    RwInfoOp::new(0x0000000000000000, 0x0000000000000000, 0xFF, 0, OpRwFlags::WRITE.bits() | OpRwFlags::ZEXT.bits() | OpRwFlags::CONSECUTIVE.bits()), // #86 [ref=2x]
    RwInfoOp::new(0x00000000FFFFFFFF, 0x00000000FFFFFFFF, 0xFF, 0, OpRwFlags::RW.bits() | OpRwFlags::ZEXT.bits()), // #87 [ref=3x]
];

pub static RW_INFO_RM_TABLE: &[RwInfoRm] = &[
    RwInfoRm::new(RwInfoRmCategory::None, 0x00, 0, 0, 0), // #0 [ref=1996x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x03, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #1 [ref=8x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x02, 0, 0, 0), // #2 [ref=190x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 16, 0, 0), // #3 [ref=122x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 8, 0, 0), // #4 [ref=66x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 4, 0, 0), // #5 [ref=35x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x04, 0, 0, 0), // #6 [ref=314x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 2, 0, 0), // #7 [ref=9x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 0, 0, 0), // #8 [ref=52x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 0, 0, 0), // #9 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x01, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #10 [ref=20x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x01, 0, 0, 0), // #11 [ref=14x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 8, 0, 0), // #12 [ref=25x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 64, 0, 0), // #13 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x00, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #14 [ref=30x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 16, 0, 0), // #15 [ref=17x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x02, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #16 [ref=22x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 1, 0, 0), // #17 [ref=5x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 4, 0, 0), // #18 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 10, 0, 0), // #19 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x01, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #20 [ref=5x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 2, 0, 0), // #21 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x06, 0, 0, 0), // #22 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 1, 0, 0), // #23 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 4, 0, 0), // #24 [ref=3x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 8, 0, 0), // #25 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 2, 0, 0), // #26 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 2, 0, 0), // #27 [ref=13x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 4, 0, 0), // #28 [ref=8x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x03, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #29 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 16, 0, 0), // #30 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 8, RwInfoRmFlags::MOVSS_MOVSD.bits(), 0), // #31 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x00, 0, RwInfoRmFlags::MOVSS_MOVSD.bits(), 0), // #32 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 4, RwInfoRmFlags::MOVSS_MOVSD.bits(), 0), // #33 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 1, 0, 0), // #34 [ref=18x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 8, 0, 0), // #35 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x00, 0, RwInfoRmFlags::PEXTRW.bits(), 0), // #36 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 2, RwInfoRmFlags::PEXTRW.bits(), CpuFeature::SSE4_1 as u8), // #37 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::None, 0x02, 0, 0, 0), // #38 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 2, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #39 [ref=3x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 8, 0, 0), // #40 [ref=33x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 2, 0, 0), // #41 [ref=30x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 4, 0, 0), // #42 [ref=40x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x00, 32, 0, 0), // #43 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 8, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #44 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x02, 4, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #45 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Half, 0x02, 0, 0, 0), // #46 [ref=19x]
    RwInfoRm::new(RwInfoRmCategory::Quarter, 0x02, 0, 0, 0), // #47 [ref=9x]
    RwInfoRm::new(RwInfoRmCategory::Half, 0x01, 0, 0, 0), // #48 [ref=10x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x04, 0, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #49 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 16, 0, 0), // #50 [ref=30x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 16, 0, 0), // #51 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x01, 32, 0, 0), // #52 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x0C, 0, 0, 0), // #53 [ref=15x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x0C, 8, 0, 0), // #54 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x0C, 4, 0, 0), // #55 [ref=4x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 32, 0, 0), // #56 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x03, 0, 0, 0), // #57 [ref=13x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x03, 8, RwInfoRmFlags::AMBIGUOUS.bits(), 0), // #58 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x08, 0, 0, 0), // #59 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x04, 1, 0, 0), // #60 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Quarter, 0x01, 0, 0, 0), // #61 [ref=6x]
    RwInfoRm::new(RwInfoRmCategory::Eighth, 0x01, 0, 0, 0), // #62 [ref=3x]
    RwInfoRm::new(RwInfoRmCategory::Eighth, 0x02, 0, 0, 0), // #63 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x0C, 16, 0, 0), // #64 [ref=1x]
    RwInfoRm::new(RwInfoRmCategory::Fixed, 0x06, 16, 0, 0), // #65 [ref=12x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x06, 0, RwInfoRmFlags::FEATURE_IF_RMI.bits(), CpuFeature::AVX512_F as u8), // #66 [ref=5x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x02, 0, RwInfoRmFlags::FEATURE_IF_RMI.bits(), CpuFeature::AVX512_BW as u8), // #67 [ref=2x]
    RwInfoRm::new(RwInfoRmCategory::Consistent, 0x06, 0, RwInfoRmFlags::FEATURE_IF_RMI.bits(), CpuFeature::AVX512_BW as u8), // #68 [ref=3x]
];

