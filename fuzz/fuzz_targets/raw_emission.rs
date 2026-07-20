#![no_main]

use asmkit::aarch64::{Assembler as Aarch64Assembler, InstId as Aarch64InstId};
use asmkit::Arch;
use asmkit::CodeBuffer;
use asmkit::{Operand, OperandCast, OperandSignature, imm};
use asmkit::Environment;
use asmkit::riscv::Assembler as RiscvAssembler;
use asmkit::x86::{Assembler as X86Assembler, InstId as X86InstId};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let opcode = data
        .get(..4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
        .unwrap_or_default();
    let operand = imm(data.get(4).copied().unwrap_or_default() as i64);
    let empty = Operand::new();
    let mut arbitrary = Operand::new();
    let mut signature_bytes = [0u8; 4];
    if let Some(bytes) = data.get(4..8) {
        signature_bytes.copy_from_slice(bytes);
    }
    arbitrary.set_signature(OperandSignature::from(u32::from_le_bytes(signature_bytes)));
    let empty_operands = [&empty];
    let immediate_operands = [operand.as_operand()];
    let arbitrary_operands = [&arbitrary];
    let operands: &[&Operand] = match data.get(8).copied().unwrap_or_default() % 3 {
        0 => &empty_operands,
        1 => &immediate_operands,
        _ => &arbitrary_operands,
    };

    let mut x86 = CodeBuffer::new(Environment::new(Arch::X64));
    let _ = X86Assembler::new(&mut x86).try_emit_n(opcode, &operands);

    let mut aarch64 = CodeBuffer::new(Environment::new(Arch::AArch64));
    let _ = Aarch64Assembler::new(&mut aarch64)
        .try_emit_n((opcode % Aarch64InstId::_Count as u32) + 1, &operands);

    let mut riscv = CodeBuffer::new(Environment::new(Arch::RISCV64));
    let _ = RiscvAssembler::new(&mut riscv).try_emit_n(opcode as i64, &operands);

    let _ = X86InstId::_Count;
});
