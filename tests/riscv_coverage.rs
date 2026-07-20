#![cfg(all(feature = "riscv", feature = "validation"))]
//! Signature-driven RISC-V typed-emitter coverage.

use std::panic::{AssertUnwindSafe, catch_unwind};

use asmkit::Arch;
use asmkit::AsmError;
use asmkit::CodeBuffer;
use asmkit::Environment;
use asmkit::riscv::coverage::*;
use asmkit::riscv::operands::regs;
use asmkit::riscv::{Assembler, Opcode};
use asmkit::{Operand, OperandCast, imm};

const OPCODE_COUNT: usize = 1_039;
const TYPED_OPCODE_COUNT: usize = 1_030;
const RAW_ONLY_OPCODE_COUNT: usize = 9;

const RAW_ONLY: &[(Opcode, &str)] = &[
    (
        Opcode::CMPOP,
        "RISC-V compressed register-list instructions are not implemented",
    ),
    (
        Opcode::CMPOPRET,
        "RISC-V compressed register-list instructions are not implemented",
    ),
    (
        Opcode::CMPOPRETZ,
        "RISC-V compressed register-list instructions are not implemented",
    ),
    (
        Opcode::CMPUSH,
        "RISC-V compressed register-list instructions are not implemented",
    ),
    (
        Opcode::CMMVA01S,
        "RISC-V compressed saved-register moves are not implemented",
    ),
    (
        Opcode::CMMVSA01,
        "RISC-V compressed saved-register moves are not implemented",
    ),
    (
        Opcode::MOPRN,
        "RISC-V MOP.RN instructions are not implemented",
    ),
    (
        Opcode::MOPRRN,
        "RISC-V MOP.RR.N instructions are not implemented",
    ),
    (
        Opcode::CSEXTW,
        "RISC-V RdRs1N0 instructions are not implemented",
    ),
];

fn raw_only(encoding: Encoding) -> bool {
    matches!(
        encoding,
        Encoding::CRlistCSpimm
            | Encoding::CSreg1CSreg2
            | Encoding::MopRT30MopRT2726MopRT2120RdRs1
            | Encoding::MopRrT30MopRrT2726RdRs1Rs2
            | Encoding::RdRs1N0
    )
}

fn raw_only_reason(opcode: Opcode) -> Option<&'static str> {
    RAW_ONLY
        .iter()
        .find_map(|(candidate, reason)| (*candidate == opcode).then_some(*reason))
}

fn operands(opcode: Opcode) -> Vec<Operand> {
    SIGNATURE_TABLE[INST_INFO_TABLE[opcode as usize].signature_index as usize]
        .iter()
        .take_while(|class| **class != ANY)
        .map(|class| match *class {
            GP => *regs::x(8).as_operand(),
            FP => *regs::f(8).as_operand(),
            VEC => *regs::v(8).as_operand(),
            IMM => *imm(1).as_operand(),
            _ => unreachable!("unknown generated RISC-V operand class"),
        })
        .collect()
}

#[derive(Default)]
struct Counts {
    accepted: usize,
    mode_rejected: usize,
    raw_only_rejected: usize,
    unexpected: Vec<String>,
}

fn sweep(arch: Arch) -> Counts {
    let xlen = if arch == Arch::RISCV32 { 1 } else { 2 };
    let mut counts = Counts::default();

    for (index, opcode) in ALL_OPCODES.iter().copied().enumerate() {
        let name = OPCODE_STR[index];
        let result = catch_unwind(AssertUnwindSafe(|| {
            let operands = operands(opcode);
            let operand_refs: Vec<_> = operands.iter().collect();
            let mut buffer = CodeBuffer::new(Environment::new(arch));
            Assembler::new(&mut buffer).try_emit_n(opcode as i64, &operand_refs)
        }));
        let result = result.unwrap_or_else(|_| panic!("{arch:?} {name} panicked"));

        if OPCODE_XLEN[index] & xlen == 0 {
            assert_eq!(result, Err(AsmError::InvalidInstruction), "{arch:?} {name}");
            counts.mode_rejected += 1;
        } else if let Some(expected_reason) = raw_only_reason(opcode) {
            assert!(
                matches!(
                    result,
                    Err(AsmError::UnsupportedInstruction { reason }) if reason == expected_reason
                ),
                "{arch:?} {name}: {result:?}"
            );
            counts.raw_only_rejected += 1;
        } else {
            if result == Ok(()) {
                counts.accepted += 1;
            } else {
                counts.unexpected.push(format!("{name}: {result:?}"));
            }
        }
    }

    counts
}

#[test]
fn every_generated_signature_encodes_or_is_explicitly_rejected() {
    assert_eq!(ALL_OPCODES.len(), OPCODE_COUNT);
    assert_eq!(OPCODE_STR.len(), OPCODE_COUNT + 1);
    assert_eq!(OPCODE_STR[OPCODE_COUNT], "<invalid>");
    assert_eq!(OPCODE_XLEN.len(), OPCODE_COUNT);
    assert!(OPCODE_XLEN.iter().all(|xlen| *xlen != 0));
    assert_eq!(OPCODE_XLEN[Opcode::SSAMOSWAPD as usize], 2);
    assert_eq!(INST_INFO_TABLE.len(), OPCODE_COUNT);
    assert_eq!(SIGNATURE_TABLE.len(), 45);
    assert_eq!(
        ALL_OPCODES
            .iter()
            .filter(|opcode| raw_only(opcode.encoding()))
            .count(),
        RAW_ONLY_OPCODE_COUNT
    );
    assert_eq!(RAW_ONLY.len(), RAW_ONLY_OPCODE_COUNT);
    assert!(
        RAW_ONLY
            .iter()
            .all(|(opcode, _)| raw_only(opcode.encoding()))
    );
    assert_eq!(OPCODE_COUNT - RAW_ONLY_OPCODE_COUNT, TYPED_OPCODE_COUNT);

    let rv32 = sweep(Arch::RISCV32);
    let rv64 = sweep(Arch::RISCV64);

    eprintln!(
        "RV32: {} accepted, {} mode-rejected, {} raw-only rejected; RV64: {} accepted, {} mode-rejected, {} raw-only rejected",
        rv32.accepted,
        rv32.mode_rejected,
        rv32.raw_only_rejected,
        rv64.accepted,
        rv64.mode_rejected,
        rv64.raw_only_rejected,
    );
    assert!(
        rv32.unexpected.is_empty() && rv64.unexpected.is_empty(),
        "RV32 unexpected results:\n{}\nRV64 unexpected results:\n{}",
        rv32.unexpected.join("\n"),
        rv64.unexpected.join("\n")
    );
    assert_eq!(
        (rv32.accepted, rv32.mode_rejected, rv32.raw_only_rejected),
        (941, 90, 8)
    );
    assert_eq!(
        (rv64.accepted, rv64.mode_rejected, rv64.raw_only_rejected),
        (995, 35, 9)
    );
}

#[test]
fn fixed_opcode_bits_survive_inst_encoding() {
    let mut buffer = CodeBuffer::new(Environment::new(Arch::RISCV32));
    let mut assembler = Assembler::new(&mut buffer);

    assembler.c_mop_1();
    assembler.c_not(regs::x(8));

    assert_eq!(assembler.data(), [0x81, 0x60, 0x75, 0x9c]);
    assert_eq!(assembler.error(), None);
}
