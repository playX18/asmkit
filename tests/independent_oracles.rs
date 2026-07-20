//! Independent encoding and decode checks against LLVM MC.
//!
//! CI installs LLVM for this required release gate. RISC-V sweeps every
//! generated signature; AArch64 ratchets broad instruction-ID coverage because
//! its validation metadata does not expose generated signatures.
#![cfg(all(feature = "aarch64", feature = "riscv", feature = "validation"))]

use std::io::Write;
use std::process::{Command, Stdio};

use asmkit::Arch;
use asmkit::AsmError;
use asmkit::CodeBuffer;
use asmkit::Environment;
use asmkit::aarch64::Assembler as Aarch64Assembler;
use asmkit::aarch64::CPU_FEATURE_NAMES as AARCH64_CPU_FEATURE_NAMES;
use asmkit::aarch64::InstId;
use asmkit::aarch64::operands::{ptr as aarch64_ptr, regs as aarch64_regs};
use asmkit::riscv::Assembler as RiscvAssembler;
use asmkit::riscv::CPU_FEATURE_NAMES as RISCV_CPU_FEATURE_NAMES;
use asmkit::riscv::Opcode;
use asmkit::riscv::coverage::{
    ALL_OPCODES, ANY, FP, GP, IMM, INST_INFO_TABLE, OPCODE_STR, OPCODE_XLEN, SIGNATURE_TABLE, VEC,
};
use asmkit::riscv::operands::regs as riscv_regs;
use asmkit::{Operand, OperandCast, imm};

// Keep this deliberately tiny. A change here must either make the LLVM bytes
// match (then remove the entry) or be reviewed as a new oracle discrepancy.
const RISCV_ORACLE_ALLOWLIST: &[(&str, &[u8])] = &[("vadd.vv operand order", &[0xd7, 0, 0x31, 2])];

const RISCV32_LLVM_DECODE_EXCLUSIONS: &[(&str, &str)] = &[
    ("fcvt.h.q", "LLVM 22 has no half/quad conversion mnemonic"),
    ("fcvt.q.h", "LLVM 22 has no half/quad conversion mnemonic"),
    (
        "vloxei64.v",
        "LLVM 22 requires RV64I for a 64-bit vector index EEW",
    ),
    (
        "vluxei64.v",
        "LLVM 22 requires RV64I for a 64-bit vector index EEW",
    ),
    (
        "vsoxei64.v",
        "LLVM 22 requires RV64I for a 64-bit vector index EEW",
    ),
    (
        "vsuxei64.v",
        "LLVM 22 requires RV64I for a 64-bit vector index EEW",
    ),
];

const RISCV64_LLVM_DECODE_EXCLUSIONS: &[(&str, &str)] = &[
    ("fcvt.h.q", "LLVM 22 has no half/quad conversion mnemonic"),
    ("fcvt.q.h", "LLVM 22 has no half/quad conversion mnemonic"),
];

fn llvm_mc(triple: &str, source: &str, mattr: Option<&str>) -> Option<Vec<u8>> {
    let mut command = Command::new("llvm-mc");
    command
        .arg(format!("--triple={triple}"))
        .arg("--show-encoding")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(mattr) = mattr {
        command.arg(format!("--mattr={mattr}"));
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            panic!("llvm-mc is required for the independent oracle gate")
        }
        Err(error) => panic!("failed to run llvm-mc: {error}"),
    };
    child
        .stdin
        .take()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "llvm-mc rejected {source:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let bytes: Vec<u8> = stdout
        .split("encoding: [")
        .skip(1)
        .flat_map(|line| {
            line.split_once(']')
                .unwrap_or_else(|| {
                    panic!("llvm-mc reported a malformed encoding for {source:?}: {stdout}")
                })
                .0
                .split(',')
                .map(|byte| {
                    u8::from_str_radix(byte.trim().strip_prefix("0x").unwrap(), 16).unwrap()
                })
        })
        .collect();
    assert!(
        !bytes.is_empty(),
        "llvm-mc did not report an encoding for {source:?}: {stdout}"
    );
    Some(bytes)
}

fn aarch64_bytes(id: InstId, operands: &[Operand]) -> Vec<u8> {
    let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
    let mut assembler = Aarch64Assembler::new(&mut buffer);
    let operands: Vec<_> = operands.iter().collect();
    assembler.try_emit_n(id as u32, &operands).unwrap();
    buffer.data().to_vec()
}

fn riscv_bytes(arch: Arch, opcode: Opcode, operands: &[Operand]) -> Vec<u8> {
    let mut buffer = CodeBuffer::new(Environment::new(arch));
    let mut assembler = RiscvAssembler::new(&mut buffer);
    let operands: Vec<_> = operands.iter().collect();
    assembler.try_emit_n(opcode as i64, &operands).unwrap();
    buffer.data().to_vec()
}

fn riscv_signature_operands(opcode: Opcode, immediate: i64) -> Vec<Operand> {
    SIGNATURE_TABLE[INST_INFO_TABLE[opcode as usize].signature_index as usize]
        .iter()
        .take_while(|class| **class != ANY)
        .map(|class| match *class {
            GP => *riscv_regs::x(8).as_operand(),
            FP => *riscv_regs::f(8).as_operand(),
            VEC => *riscv_regs::v(8).as_operand(),
            IMM => *imm(immediate).as_operand(),
            _ => unreachable!("unknown generated RISC-V operand class"),
        })
        .collect()
}

fn llvm_riscv_features(triple: &str) -> String {
    let output = Command::new("llvm-mc")
        .arg(format!("--triple={triple}"))
        .arg("--mattr=help")
        .output()
        .unwrap_or_else(|error| panic!("failed to run llvm-mc: {error}"));
    assert!(output.status.success(), "llvm-mc --mattr=help failed");
    let available = String::from_utf8(output.stderr).unwrap();

    let mut features = RISCV_CPU_FEATURE_NAMES
        .iter()
        .flat_map(|name| {
            name.strip_prefix("rv32_")
                .or_else(|| name.strip_prefix("rv64_"))
                .or_else(|| name.strip_prefix("rv_"))
                .unwrap_or(name)
                .split('_')
        })
        .filter_map(|feature| {
            let experimental = format!("experimental-{feature}");
            available
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{feature} ")))
                .then_some(feature.to_owned())
                .or_else(|| {
                    available
                        .lines()
                        .any(|line| line.trim_start().starts_with(&format!("{experimental} ")))
                        .then_some(experimental)
                })
        })
        .collect::<Vec<_>>();
    features.sort_unstable();
    features.dedup();
    for feature in ["zicbom", "zicbop", "zicboz"] {
        if available
            .lines()
            .any(|line| line.trim_start().starts_with(&format!("{feature} ")))
        {
            features.push(feature.to_owned());
        }
    }
    features
        .into_iter()
        .map(|feature| format!("+{feature}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn llvm_decodable(triple: &str, mattr: &str, cases: &[(&str, Vec<u8>)]) -> Vec<bool> {
    let mut child = Command::new("llvm-mc")
        .arg(format!("--triple={triple}"))
        .arg("--disassemble")
        .arg("--show-encoding")
        .arg("-M=no-aliases")
        .arg(format!("--mattr={mattr}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("failed to run llvm-mc: {error}"));
    {
        let mut stdin = child.stdin.take().unwrap();
        for (name, bytes) in cases {
            for byte in bytes {
                write!(stdin, "0x{byte:02x} ").unwrap();
            }
            writeln!(stdin, "# {name}").unwrap();
        }
    }
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        output.status.success(),
        "LLVM {triple} decoder failed: {stderr}"
    );

    let mut decoded = vec![true; cases.len()];
    for line in stderr.lines() {
        if let Some(line_number) = line
            .strip_prefix("<stdin>:")
            .and_then(|rest| rest.split(':').next())
            .and_then(|number| number.parse::<usize>().ok())
        {
            decoded[line_number - 1] = false;
        }
    }
    decoded
}

fn llvm_aarch64_features() -> String {
    let output = Command::new("llvm-mc")
        .args(["--triple=aarch64", "--mattr=help"])
        .output()
        .unwrap_or_else(|error| panic!("failed to run llvm-mc: {error}"));
    assert!(output.status.success(), "llvm-mc --mattr=help failed");
    let available = String::from_utf8(output.stderr).unwrap();
    let mut features = AARCH64_CPU_FEATURE_NAMES
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .filter(|feature| {
            available
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{feature} ")))
        })
        .collect::<Vec<_>>();
    if available
        .lines()
        .any(|line| line.trim_start().starts_with("neon "))
    {
        features.push("neon".to_owned());
    }
    features.sort_unstable();
    features.dedup();
    features
        .into_iter()
        .map(|feature| format!("+{feature}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn riscv_label_pair_bytes(arch: Arch) -> Vec<u8> {
    let mut buffer = CodeBuffer::new(Environment::new(arch));
    let target = buffer.get_label();
    {
        let mut assembler = RiscvAssembler::new(&mut buffer);
        assembler
            .try_emit_n(
                Opcode::AUIPC as i64,
                &[riscv_regs::x(10).as_operand(), target.as_operand()],
            )
            .unwrap();
        assembler
            .try_emit_n(
                Opcode::JAL as i64,
                &[riscv_regs::x(11).as_operand(), target.as_operand()],
            )
            .unwrap();
        assembler.try_bind_label(target).unwrap();
    }
    buffer.finish().unwrap().data().to_vec()
}

#[test]
fn aarch64_integer_and_addressing_match_llvm() {
    if llvm_mc("aarch64", "nop\n", None).is_none() {
        return;
    }

    for immediate in [0, 1, 4095] {
        let operands = [
            *aarch64_regs::x(1).as_operand(),
            *aarch64_regs::x(2).as_operand(),
            *imm(immediate).as_operand(),
        ];
        assert_eq!(
            aarch64_bytes(InstId::Add, &operands),
            llvm_mc("aarch64", &format!("add x1, x2, #{immediate}\n"), None).unwrap()
        );
        assert_eq!(
            aarch64_bytes(InstId::Sub, &operands),
            llvm_mc("aarch64", &format!("sub x1, x2, #{immediate}\n"), None).unwrap()
        );
    }

    for offset in [0, 8, 32_760] {
        let operands = [
            *aarch64_regs::x(3).as_operand(),
            *aarch64_ptr(aarch64_regs::x(4), offset).as_operand(),
        ];
        assert_eq!(
            aarch64_bytes(InstId::Ldr, &operands),
            llvm_mc("aarch64", &format!("ldr x3, [x4, #{offset}]\n"), None).unwrap()
        );
    }

    for offset in [0, 4, 16_380] {
        let operands = [
            *aarch64_regs::w(5).as_operand(),
            *aarch64_ptr(aarch64_regs::x(6), offset).as_operand(),
        ];
        assert_eq!(
            aarch64_bytes(InstId::Str, &operands),
            llvm_mc("aarch64", &format!("str w5, [x6, #{offset}]\n"), None).unwrap()
        );
    }
}

#[test]
fn aarch64_alias_system_and_feature_forms_match_llvm() {
    if llvm_mc("aarch64", "nop\n", None).is_none() {
        return;
    }

    for immediate in [0, 1, 0xff, 0xffff] {
        let operands = [
            *aarch64_regs::x(1).as_operand(),
            *imm(immediate).as_operand(),
        ];
        assert_eq!(
            aarch64_bytes(InstId::Mov, &operands),
            llvm_mc("aarch64", &format!("mov x1, #{immediate}\n"), None).unwrap()
        );
    }

    let mrs = [*aarch64_regs::x(1).as_operand(), *imm(0xDA10).as_operand()];
    assert_eq!(
        aarch64_bytes(InstId::Mrs, &mrs),
        llvm_mc("aarch64", "mrs x1, nzcv\n", None).unwrap()
    );

    let aese = [
        *aarch64_regs::v(1).b16().as_operand(),
        *aarch64_regs::v(2).b16().as_operand(),
    ];
    assert_eq!(
        aarch64_bytes(InstId::Aese_v, &aese),
        llvm_mc("aarch64", "aese v1.16b, v2.16b\n", Some("+crypto")).unwrap()
    );
}

#[test]
fn aarch64_instruction_ids_have_ratcheted_llvm_decode_coverage() {
    if llvm_mc("aarch64", "nop\n", None).is_none() {
        return;
    }

    let w0 = *aarch64_regs::w(8).as_operand();
    let w1 = *aarch64_regs::w(9).as_operand();
    let w2 = *aarch64_regs::w(10).as_operand();
    let x0 = *aarch64_regs::x(8).as_operand();
    let x1 = *aarch64_regs::x(9).as_operand();
    let x2 = *aarch64_regs::x(10).as_operand();
    let b0 = *aarch64_regs::v(8).b().as_operand();
    let h0 = *aarch64_regs::v(8).h().as_operand();
    let s0 = *aarch64_regs::v(8).s().as_operand();
    let d0 = *aarch64_regs::v(8).d().as_operand();
    let q0 = *aarch64_regs::v(8).q().as_operand();
    let va0 = *aarch64_regs::v(8).b16().as_operand();
    let va1 = *aarch64_regs::v(9).b16().as_operand();
    let va2 = *aarch64_regs::v(10).b16().as_operand();
    let element = *aarch64_regs::v(9).b_at(1).as_operand();
    let immediate = *imm(1).as_operand();
    let memory = *aarch64_ptr(aarch64_regs::x(12), 16).as_operand();
    let shapes = vec![
        vec![],
        vec![w0],
        vec![x0],
        vec![b0],
        vec![h0],
        vec![s0],
        vec![d0],
        vec![q0],
        vec![va0],
        vec![immediate],
        vec![w0, w1],
        vec![x0, x1],
        vec![va0, va1],
        vec![s0, s0],
        vec![d0, d0],
        vec![q0, q0],
        vec![element, va0],
        vec![va0, element],
        vec![w0, immediate],
        vec![x0, immediate],
        vec![va0, immediate],
        vec![w0, memory],
        vec![x0, memory],
        vec![va0, memory],
        vec![w0, w1, w2],
        vec![x0, x1, x2],
        vec![va0, va1, va2],
        vec![w0, w1, immediate],
        vec![x0, x1, immediate],
        vec![va0, va1, immediate],
        vec![va0, va1, element],
        vec![w0, w1, memory],
        vec![x0, x1, memory],
        vec![va0, va1, memory],
        vec![w0, w1, w2, immediate],
        vec![x0, x1, x2, immediate],
        vec![va0, va1, va2, immediate],
        vec![va0, va1, va2, element],
        vec![va0, va1, va2, va0, memory],
        vec![va0, va1, va2, va0, va1, memory],
    ];
    let mut decoded = vec![false; InstId::_Count as usize];
    let mattr = llvm_aarch64_features();

    for shape in shapes {
        let mut indexes = Vec::new();
        let mut cases = Vec::new();
        for (index, is_decoded) in decoded.iter().copied().enumerate().skip(1) {
            if is_decoded {
                continue;
            }
            let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
            let operands: Vec<_> = shape.iter().collect();
            if Aarch64Assembler::new(&mut buffer)
                .try_emit_n(index as u32, &operands)
                .is_ok()
                && !buffer.data().is_empty()
            {
                indexes.push(index);
                cases.push(("AArch64 generated instruction", buffer.data().to_vec()));
            }
        }
        for (index, is_decoded) in indexes
            .into_iter()
            .zip(llvm_decodable("aarch64", &mattr, &cases))
        {
            decoded[index] = is_decoded;
        }
    }

    let count = decoded.iter().filter(|decoded| **decoded).count();
    eprintln!(
        "AArch64: {count}/{} instruction IDs independently decoded by LLVM",
        InstId::_Count as usize - 1
    );
    assert!(
        count >= 500,
        "AArch64 LLVM decode coverage regressed to {count}"
    );
}

#[test]
fn riscv_raw_integer_emission_matches_llvm() {
    if llvm_mc("riscv64", "addi x1, x2, 0\n", None).is_none() {
        return;
    }

    for arch in [Arch::RISCV32, Arch::RISCV64] {
        let triple = if arch == Arch::RISCV32 {
            "riscv32"
        } else {
            "riscv64"
        };
        for immediate in [-2048, -1, 0, 1, 2047] {
            let operands = [
                *riscv_regs::x(1).as_operand(),
                *riscv_regs::x(2).as_operand(),
                *imm(immediate).as_operand(),
            ];
            assert_eq!(
                riscv_bytes(arch, Opcode::ADDI, &operands),
                llvm_mc(triple, &format!("addi x1, x2, {immediate}\n"), None).unwrap()
            );
        }

        let add = [
            *riscv_regs::x(3).as_operand(),
            *riscv_regs::x(4).as_operand(),
            *riscv_regs::x(5).as_operand(),
        ];
        assert_eq!(
            riscv_bytes(arch, Opcode::ADD, &add),
            llvm_mc(triple, "add x3, x4, x5\n", None).unwrap()
        );

        let load = [
            *riscv_regs::x(6).as_operand(),
            *riscv_regs::x(7).as_operand(),
            *imm(16).as_operand(),
        ];
        assert_eq!(
            riscv_bytes(arch, Opcode::LW, &load),
            llvm_mc(triple, "lw x6, 16(x7)\n", None).unwrap()
        );

        let store = [
            *riscv_regs::x(7).as_operand(),
            *riscv_regs::x(6).as_operand(),
            *imm(-16).as_operand(),
        ];
        assert_eq!(
            riscv_bytes(arch, Opcode::SW, &store),
            llvm_mc(triple, "sw x6, -16(x7)\n", None).unwrap()
        );
    }

    let compressed = [*riscv_regs::x(1).as_operand(), *imm(1).as_operand()];
    assert_eq!(
        riscv_bytes(Arch::RISCV64, Opcode::CADDI, &compressed),
        llvm_mc("riscv64", "c.addi x1, 1\n", Some("+c")).unwrap()
    );

    for arch in [Arch::RISCV32, Arch::RISCV64] {
        let triple = if arch == Arch::RISCV32 {
            "riscv32"
        } else {
            "riscv64"
        };
        assert_eq!(
            riscv_label_pair_bytes(arch),
            llvm_mc(triple, "auipc x10, 0\njal x11, 4\n", None).unwrap()
        );
    }
}

#[test]
fn riscv_extension_forms_match_llvm() {
    if llvm_mc("riscv64", "addi x1, x2, 0\n", None).is_none() {
        return;
    }

    let atomic = [
        *riscv_regs::x(5).as_operand(),
        *riscv_regs::x(6).as_operand(),
        *riscv_regs::x(7).as_operand(),
        *imm(1).as_operand(),
        *imm(1).as_operand(),
    ];
    assert_eq!(
        riscv_bytes(Arch::RISCV64, Opcode::AMOADDW, &atomic),
        llvm_mc("riscv64", "amoadd.w.aqrl x5, x7, (x6)\n", Some("+a")).unwrap()
    );

    let csr = [
        *riscv_regs::x(1).as_operand(),
        *riscv_regs::x(2).as_operand(),
        *imm(0x300).as_operand(),
    ];
    assert_eq!(
        riscv_bytes(Arch::RISCV64, Opcode::CSRRW, &csr),
        llvm_mc("riscv64", "csrrw x1, mstatus, x2\n", Some("+zicsr")).unwrap()
    );

    let cbo = [*riscv_regs::x(8).as_operand()];
    assert_eq!(
        riscv_bytes(Arch::RISCV64, Opcode::CBOCLEAN, &cbo),
        llvm_mc("riscv64", "cbo.clean (x8)\n", Some("+zicbom")).unwrap()
    );

    let vector = [
        *riscv_regs::v(1).as_operand(),
        *riscv_regs::v(2).as_operand(),
        *riscv_regs::v(3).as_operand(),
        *imm(1).as_operand(),
    ];
    let ours = riscv_bytes(Arch::RISCV64, Opcode::VADDVV, &vector);
    let llvm = llvm_mc("riscv64", "vadd.vv v1, v2, v3\n", Some("+v")).unwrap();
    if ours != llvm {
        assert_eq!(
            RISCV_ORACLE_ALLOWLIST,
            &[("vadd.vv operand order", ours.as_slice())],
            "unreviewed RISC-V LLVM-MC discrepancy"
        );
    }
}

#[test]
fn riscv_generated_signatures_have_ratcheted_llvm_decode_coverage() {
    if llvm_mc("riscv64", "addi x1, x2, 0\n", None).is_none() {
        return;
    }

    for (arch, triple, xlen) in [(Arch::RISCV32, "riscv32", 1), (Arch::RISCV64, "riscv64", 2)] {
        let mattr = llvm_riscv_features(triple);
        let mut independently_decoded = vec![false; ALL_OPCODES.len()];
        let mut typed_rejected = vec![false; ALL_OPCODES.len()];
        let mut accepted = 0;
        let mut rejected = 0;
        for (index, opcode) in ALL_OPCODES.iter().copied().enumerate() {
            let operands = riscv_signature_operands(opcode, 1);
            let operand_refs: Vec<_> = operands.iter().collect();
            let mut buffer = CodeBuffer::new(Environment::new(arch));
            let result = RiscvAssembler::new(&mut buffer)
                .try_emit_n(opcode as i64, &operand_refs)
                .map(|()| buffer.data().to_vec());

            if OPCODE_XLEN[index] & xlen == 0 {
                assert_eq!(result, Err(AsmError::InvalidInstruction));
            } else {
                match result {
                    Ok(_) => accepted += 1,
                    Err(AsmError::UnsupportedInstruction { .. }) => {
                        rejected += 1;
                        typed_rejected[index] = true;
                    }
                    Err(error) => panic!("{triple} {}: {error}", OPCODE_STR[index]),
                }
            }
        }

        assert_eq!(rejected, if arch == Arch::RISCV32 { 8 } else { 9 });

        // Different forms constrain their immediate fields differently. Trying
        // a short list of ordinary values avoids copying those constraints from
        // the encoder into this independent decoder check.
        for immediate in [1, 16, 0, 2, 4, 8, 31, -1, 255, 2047] {
            let mut indexes = Vec::new();
            let mut cases = Vec::new();
            for (index, opcode) in ALL_OPCODES.iter().copied().enumerate() {
                if independently_decoded[index] || OPCODE_XLEN[index] & xlen == 0 {
                    continue;
                }
                let operands = riscv_signature_operands(opcode, immediate);
                let operand_refs: Vec<_> = operands.iter().collect();
                let mut buffer = CodeBuffer::new(Environment::new(arch));
                if RiscvAssembler::new(&mut buffer)
                    .try_emit_n(opcode as i64, &operand_refs)
                    .is_ok()
                {
                    indexes.push(index);
                    cases.push((OPCODE_STR[index], buffer.data().to_vec()));
                }
            }
            for (index, decoded) in indexes
                .into_iter()
                .zip(llvm_decodable(triple, &mattr, &cases))
            {
                independently_decoded[index] = decoded;
            }
        }

        // Retry the small remainder individually. LLVM can attach a diagnostic
        // for one byte sequence to the preceding line in a mixed-width batch.
        for (index, opcode) in ALL_OPCODES.iter().copied().enumerate() {
            if independently_decoded[index]
                || typed_rejected[index]
                || OPCODE_XLEN[index] & xlen == 0
            {
                continue;
            }
            for immediate in [1, 16, 0, 2, 4, 8, 31, -1, 255, 2047] {
                let operands = riscv_signature_operands(opcode, immediate);
                let operand_refs: Vec<_> = operands.iter().collect();
                let mut buffer = CodeBuffer::new(Environment::new(arch));
                if RiscvAssembler::new(&mut buffer)
                    .try_emit_n(opcode as i64, &operand_refs)
                    .is_ok()
                    && llvm_decodable(
                        triple,
                        &mattr,
                        &[(OPCODE_STR[index], buffer.data().to_vec())],
                    )[0]
                {
                    independently_decoded[index] = true;
                    break;
                }
            }
        }

        let decoded = independently_decoded
            .iter()
            .filter(|decoded| **decoded)
            .count();
        let unresolved: Vec<_> = independently_decoded
            .iter()
            .enumerate()
            .filter_map(|(index, decoded)| {
                (!decoded && !typed_rejected[index] && OPCODE_XLEN[index] & xlen != 0)
                    .then_some(OPCODE_STR[index])
            })
            .collect();
        let exclusions = if arch == Arch::RISCV32 {
            RISCV32_LLVM_DECODE_EXCLUSIONS
        } else {
            RISCV64_LLVM_DECODE_EXCLUSIONS
        };
        assert_eq!(
            unresolved,
            exclusions
                .iter()
                .map(|(name, _reason)| *name)
                .collect::<Vec<_>>(),
            "{triple} LLVM decode exclusions changed; classify every difference"
        );
        eprintln!(
            "{triple}: {accepted} generated signatures accepted, {rejected} typed-rejected, {decoded} independently decoded by LLVM"
        );
        assert_eq!(accepted, decoded + exclusions.len());
    }
}
