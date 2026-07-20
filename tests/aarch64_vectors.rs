//! AArch64 encoder regression vectors.
//!
//! Expected bytes are taken from AsmJit's own assembler test suite
//! (`asmjit-testing/tests/asmjit_test_assembler_a64.cpp`), which the encoder
//! in `src/aarch64/` is a port of. Each case emits through the real
//! `Assembler::emit_n` path and compares the produced word. The negative
//! cases pin error behavior for inputs that previously panicked or were
//! silently mis-encoded.
#![cfg(feature = "aarch64")]

use asmkit::Arch;
use asmkit::CodeBuffer;
use asmkit::Environment;
use asmkit::aarch64::Assembler;
use asmkit::aarch64::InstId;
use asmkit::aarch64::operands::{ptr, regs};
use asmkit::{Operand, OperandCast, imm};

/// Emits `id` with `ops` and expects the little-endian word `expected`
/// (given as the hex value from AsmJit's test suite).
fn check(expected: u32, id: InstId, ops: &[Operand]) {
    let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
    let mut a = Assembler::new(&mut buf);
    let refs: Vec<&Operand> = ops.iter().collect();
    a.emit_n(id as u32, &refs);
    assert!(
        buf.error().is_none(),
        "{id:?} failed to emit: {:?}",
        buf.error()
    );
    assert_eq!(
        u32::from_le_bytes(buf.data().try_into().unwrap()),
        expected,
        "{id:?} emitted wrong bytes"
    );
}

/// Emits `id` with `ops` and expects rejection (and no panic).
fn check_err(id: InstId, ops: &[Operand]) {
    let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
    let mut a = Assembler::new(&mut buf);
    let refs: Vec<&Operand> = ops.iter().collect();
    a.emit_n(id as u32, &refs);
    assert!(buf.error().is_some(), "{id:?} unexpectedly accepted");
}

fn w(i: u32) -> Operand {
    *regs::w(i).as_operand()
}
fn x(i: u32) -> Operand {
    *regs::x(i).as_operand()
}
fn im(v: i64) -> Operand {
    *imm(v).as_operand()
}

#[test]
fn base_integer() {
    // addg/subg: BaseRRII (imm range is a bitmask, low discard bits must be zero).
    check(0x91822C41, InstId::Addg, &[x(1), x(2), im(32), im(11)]);
    check(0xD1822C41, InstId::Subg, &[x(1), x(2), im(32), im(11)]);
    check_err(InstId::Addg, &[x(1), x(2), im(17), im(0)]);
    check_err(InstId::Addg, &[x(1), x(2), im(-16), im(0)]);

    // Bitfield aliases.
    check(0x33183C41, InstId::Bfi, &[w(1), w(2), im(8), im(16)]);
    check(0xB3783C41, InstId::Bfi, &[x(1), x(2), im(8), im(16)]);
    check(0x131B2441, InstId::Sbfiz, &[w(1), w(2), im(5), im(10)]);
    check(0x937B2441, InstId::Sbfiz, &[x(1), x(2), im(5), im(10)]);
    check(0x53053841, InstId::Ubfx, &[w(1), w(2), im(5), im(10)]);
    check(0xD3453841, InstId::Ubfx, &[x(1), x(2), im(5), im(10)]);

    // Mixed-width CRC and long multiplies.
    check(0x9AC34C41, InstId::Crc32x, &[w(1), w(2), x(3)]);
    check(0x9B237C41, InstId::Smull, &[x(1), w(2), w(3)]);
    check(0x9BA31041, InstId::Umaddl, &[x(1), w(2), w(3), x(4)]);

    // chkfeat only accepts x16.
    check(0xD503251F, InstId::Chkfeat, &[x(16)]);
    check_err(InstId::Chkfeat, &[x(15)]);
}

#[test]
fn system_instructions() {
    // at S1E0R = AT::encode(0, 7, 8, 2).
    check(0xD508784F, InstId::At, &[im(0x3C2), x(15)]);
    // mrs with NZCV = SysReg::encode(3, 3, 4, 2, 0).
    check(0xD53B4201, InstId::Mrs, &[x(1), im(0xDA10)]);
    check(
        0xD5092393,
        InstId::Sys,
        &[im(1), im(2), im(3), im(4), x(19)],
    );
    check(
        0x48227C68,
        InstId::Casp,
        &[x(2), x(3), x(8), x(9), *ptr(regs::x(3), 0).as_operand()],
    );
    check(
        0x08227C68,
        InstId::Casp,
        &[w(2), w(3), w(8), w(9), *ptr(regs::x(3), 0).as_operand()],
    );
}

#[test]
fn fp_scalar_and_vector() {
    // pick_fp_opcode: scalar and vector paths.
    check(
        0x1FC31041,
        InstId::Fmadd_v,
        &[q_h(1), q_h(2), q_h(3), q_h(4)],
    );
    check(
        0x1F031041,
        InstId::Fmadd_v,
        &[q_s(1), q_s(2), q_s(3), q_s(4)],
    );
    check(
        0x1F431041,
        InstId::Fmadd_v,
        &[q_d(1), q_d(2), q_d(3), q_d(4)],
    );
    check(0x4E285841, InstId::Aesd_v, &[b16(1), b16(2)]);
    check(0x4E284841, InstId::Aese_v, &[b16(1), b16(2)]);
    check(0x4E286841, InstId::Aesmc_v, &[b16(1), b16(2)]);

    // fccmp: sz derived from Vec16 base, cond from the 4th operand (EQ = 2).
    check(0x1EE20424, InstId::Fccmp_v, &[q_h(1), q_h(2), im(4), im(2)]);
    check(0x1E220424, InstId::Fccmp_v, &[q_s(1), q_s(2), im(4), im(2)]);
    check(0x1E620424, InstId::Fccmp_v, &[q_d(1), q_d(2), im(4), im(2)]);

    check(0x1E7E0041, InstId::Fjcvtzs_v, &[w(1), q_d(2)]);
}

fn q_h(i: u32) -> Operand {
    *regs::h(i).as_operand()
}
fn q_s(i: u32) -> Operand {
    *regs::s(i).as_operand()
}
fn q_d(i: u32) -> Operand {
    *regs::d(i).as_operand()
}
fn b16(i: u32) -> Operand {
    *regs::v(i).b16().as_operand()
}

#[test]
fn simd_long_narrow() {
    // match_signature: Long/Narrow forms must not require equal signatures.
    check(0x0E234041, InstId::Addhn_v, &[b8(1), h8(2), h8(3)]);
    check(0x0EA34041, InstId::Addhn_v, &[s2(1), d2(2), d2(3)]);
    check(0x0E230041, InstId::Saddl_v, &[h8(1), b8(2), b8(3)]);
    check(0x0EA30041, InstId::Saddl_v, &[d2(1), s2(2), s2(3)]);
    check(0x0E212841, InstId::Xtn_v, &[b8(1), h8(2)]);
    check(0x0EA12841, InstId::Xtn_v, &[s2(1), d2(2)]);
    check(0x0F08A441, InstId::Sxtl_v, &[h8(1), b8(2)]);
    check(0x0F20A441, InstId::Sxtl_v, &[d2(1), s2(2)]);
    check(0x2F08A441, InstId::Uxtl_v, &[h8(1), b8(2)]);
    check(0x6E231041, InstId::Uaddw2_v, &[h8(1), h8(2), b16(3)]);
    check(0x6EA31041, InstId::Uaddw2_v, &[d2(1), d2(2), s4(3)]);
    check(0x0E217841, InstId::Fcvtl_v, &[s4(1), h4(2)]);
    check(0x4E217841, InstId::Fcvtl2_v, &[s4(1), h8(2)]);
    check(0x0E216841, InstId::Fcvtn_v, &[h4(1), s4(2)]);
    check(0x0EA16841, InstId::Bfcvtn_v, &[h4(1), s4(2)]);
}

fn b8(i: u32) -> Operand {
    *regs::v(i).b8().as_operand()
}
fn h4(i: u32) -> Operand {
    *regs::v(i).h4().as_operand()
}
fn h8(i: u32) -> Operand {
    *regs::v(i).h8().as_operand()
}
fn s2(i: u32) -> Operand {
    *regs::v(i).s2().as_operand()
}
fn s4(i: u32) -> Operand {
    *regs::v(i).s4().as_operand()
}
fn d2(i: u32) -> Operand {
    *regs::v(i).d2().as_operand()
}

#[test]
fn simd_shift_by_immediate() {
    check(0x0F0F5441, InstId::Shl_v, &[b8(1), b8(2), im(7)]);
    check(0x5F7F5441, InstId::Shl_v, &[q_d(1), q_d(2), im(63)]);
    check(0x4F7F5441, InstId::Shl_v, &[d2(1), d2(2), im(63)]);
    check(0x0F090441, InstId::Sshr_v, &[b8(1), b8(2), im(7)]);
    check(0x5F410441, InstId::Sshr_v, &[q_d(1), q_d(2), im(63)]);
    check(0x2F094441, InstId::Sri_v, &[b8(1), b8(2), im(7)]);
    check(0x7F414441, InstId::Sri_v, &[q_d(1), q_d(2), im(63)]);
    check(0x0F092441, InstId::Srshr_v, &[b8(1), b8(2), im(7)]);
    check(0x5F412441, InstId::Srshr_v, &[q_d(1), q_d(2), im(63)]);
    check(0x6F088C41, InstId::Sqrshrun2_v, &[b16(1), h8(2), im(8)]);
    check(0x2F0FA441, InstId::Ushll_v, &[h8(1), b8(2), im(7)]);
    // Out-of-range and zero inverted shifts are rejected, not panicked.
    check_err(InstId::Sshr_v, &[b8(1), b8(2), im(64)]);
    check_err(InstId::Shl_v, &[x(1), x(2), im(3)]);
}

#[test]
fn simd_crypto_and_dot() {
    check(
        0x5E030041,
        InstId::Sha1c_v,
        &[*regs::q(1).as_operand(), q_s(2), s4(3)],
    );
    check(
        0xCE448041,
        InstId::Sm3tt1a_v,
        &[s4(1), s4(2), *regs::v(4).s_at(0).as_operand()],
    );
    check(
        0xCE44B041,
        InstId::Sm3tt1a_v,
        &[s4(1), s4(2), *regs::v(4).s_at(3).as_operand()],
    );
    check(0x6E43FC41, InstId::Bfdot_v, &[s4(1), h8(2), h8(3)]);
    check(0x6E43EC41, InstId::Bfmmla_v, &[s4(1), h8(2), h8(3)]);
    check(0x6EC3FC41, InstId::Bfmlalt_v, &[s4(1), h8(2), h8(3)]);
    check(
        0x0F03F041,
        InstId::Sudot_v,
        &[s2(1), b8(2), *regs::v(3).b4_at(0).as_operand()],
    );
    check(
        0x4F23F841,
        InstId::Sudot_v,
        &[s4(1), b16(2), *regs::v(3).b4_at(3).as_operand()],
    );
}

#[test]
fn simd_multiply_long() {
    check(0x0E638041, InstId::Smlal_v, &[s4(1), h4(2), h4(3)]);
    check(0x0E63C041, InstId::Smull_v, &[s4(1), h4(2), h4(3)]);
    check(0x0E23C041, InstId::Smull_v, &[h8(1), b8(2), b8(3)]);
    check(0x6E23CC41, InstId::Fmlal2_v, &[s4(1), h4(2), h4(3)]);
    check(0x6EA3CC41, InstId::Fmlsl2_v, &[s4(1), h4(2), h4(3)]);
    // Element forms.
    check(
        0x0F73A041,
        InstId::Smull_v,
        &[s4(1), h4(2), *regs::v(3).h_at(3).as_operand()],
    );
    // NOTE: AsmJit's test file at the pinned revision expects 0x6F83C041 here,
    // but LLVM (`llvm-mc -mattr=+fp16fml`) and the pinned AsmJit itself agree
    // on 0x6F838041 — the test-data entry is wrong upstream.
    check(
        0x6F838041,
        InstId::Fmlal2_v,
        &[s4(1), h4(2), *regs::v(3).h_at(0).as_operand()],
    );
}

#[test]
fn simd_load_store_multiple() {
    let m4 = *ptr(regs::x(4), 0).as_operand();
    let m5 = *ptr(regs::x(5), 0).as_operand();
    check(
        0x0D60C061,
        InstId::Ld2r_v,
        &[b8(1), b8(2), *ptr(regs::x(3), 0).as_operand()],
    );
    check(0x0C404081, InstId::Ld3_v, &[b8(1), b8(2), b8(3), m4]);
    check(0x4C404481, InstId::Ld3_v, &[h8(1), h8(2), h8(3), m4]);
    check(
        0x0D2020A1,
        InstId::St4_v,
        &[
            *regs::v(1).b_at(0).as_operand(),
            *regs::v(2).b_at(0).as_operand(),
            *regs::v(3).b_at(0).as_operand(),
            *regs::v(4).b_at(0).as_operand(),
            m5,
        ],
    );
}

#[test]
fn invalid_inputs_do_not_panic() {
    // Register-class mismatches and out-of-range immediates must be clean
    // errors; these previously panicked in debug builds.
    check_err(InstId::Mrs, &[x(1), im(-1)]);
    check_err(InstId::Hint, &[im(-1)]);
    check_err(InstId::Fmadd_v, &[x(1), x(2), x(3), x(4)]);
    check_err(InstId::Sxtl_v, &[h8(1), x(2)]);
}
