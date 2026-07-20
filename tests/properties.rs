//! Small deterministic properties for stateful emission APIs.
//!
//! This is intentionally plain `#[test]` code: the fixed seed makes failures
//! reproducible without adding a property-testing dependency.
#![cfg(all(feature = "x86", feature = "aarch64", feature = "riscv"))]

use asmkit::Arch;
use asmkit::Environment;
use asmkit::Linker;
use asmkit::aarch64::{Assembler as Aarch64Assembler, InstId as Aarch64InstId};
use asmkit::riscv::{Assembler as RiscvAssembler, Opcode};
use asmkit::x86::{Assembler as X86Assembler, InstId as X86InstId};
use asmkit::{CodeBuffer, LabelUse};

#[test]
fn fixed_seed_failures_are_transactional() {
    let mut state = 0xA5A5_5A5Au32;
    for _ in 0..128 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);

        let mut x86 = CodeBuffer::new(Environment::new(Arch::X64));
        let x86_bytes = x86.data().to_vec();
        let x86_relocs = x86.relocs().len();
        {
            let mut assembler = X86Assembler::new(&mut x86);
            assert!(
                assembler
                    .try_emit_n(X86InstId::_Count as u32 + state, &[])
                    .is_err()
            );
        }
        assert_eq!(x86.data(), x86_bytes);
        assert_eq!(x86.relocs().len(), x86_relocs);

        let mut aarch64 = CodeBuffer::new(Environment::new(Arch::AArch64));
        let aarch64_bytes = aarch64.data().to_vec();
        let aarch64_relocs = aarch64.relocs().len();
        {
            let mut assembler = Aarch64Assembler::new(&mut aarch64);
            assert!(
                assembler
                    .try_emit_n(Aarch64InstId::_Count as u32 + state, &[])
                    .is_err()
            );
        }
        assert_eq!(aarch64.data(), aarch64_bytes);
        assert_eq!(aarch64.relocs().len(), aarch64_relocs);

        let mut riscv = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let riscv_bytes = riscv.data().to_vec();
        let riscv_relocs = riscv.relocs().len();
        {
            let mut assembler = RiscvAssembler::new(&mut riscv);
            assert!(assembler.try_emit_n(-(state as i64) - 1, &[]).is_err());
        }
        assert_eq!(riscv.data(), riscv_bytes);
        assert_eq!(riscv.relocs().len(), riscv_relocs);
    }
}

#[test]
fn fixed_seed_fixups_patches_and_linking_preserve_metadata() {
    let mut state = 0xC001_D00Du32;
    for _ in 0..64 {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);

        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        buffer.write_u32(state);
        let bytes = buffer.data().to_vec();
        assert!(
            buffer
                .try_record_patch_site(4 + (state & 15), LabelUse::X86JmpRel32, 0)
                .is_err()
        );
        assert_eq!(buffer.data(), bytes);
        assert!(buffer.relocs().is_empty());
        let patched = buffer.finish_patched().unwrap();
        assert!(patched.patch_catalog().sites().is_empty());

        let mut first = CodeBuffer::new(Environment::new(Arch::X64));
        first.write_u32(state);
        let mut second = CodeBuffer::new(Environment::new(Arch::X64));
        second.write_u32(!state);
        let mut linker = Linker::new();
        linker.add_buffer(first.finish().unwrap());
        linker.add_buffer(second.finish().unwrap());
        let image = linker.link().unwrap();
        assert_eq!(&image.data()[..4], &state.to_le_bytes());
        assert!(image.data().ends_with(&(!state).to_le_bytes()));
    }
}

#[test]
fn raw_label_fixups_finish_or_reject_without_panicking() {
    for arch in [Arch::X64, Arch::AArch64, Arch::RISCV64] {
        let mut buffer = CodeBuffer::new(Environment::new(arch));
        let label = buffer.get_label();
        let width = 4;
        buffer.write_u32(0);
        let kind = match arch {
            Arch::X64 => LabelUse::X86JmpRel32,
            Arch::AArch64 => LabelUse::A64Branch26,
            Arch::RISCV64 => LabelUse::RVJal20,
            _ => unreachable!(),
        };
        buffer.use_label_at_offset(0, label, kind);
        assert_eq!(buffer.data().len(), width);
        assert!(buffer.finish().is_err());
    }

    let _ = Opcode::ADDI; // Keep the generated RISC-V opcode surface in this target.
}
