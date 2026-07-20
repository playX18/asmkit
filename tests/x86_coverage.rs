#![cfg(all(feature = "x86", feature = "validation"))]
//! Signature-driven emit-coverage regression test.
//!
//! For every x86 `InstId`, walks every declared signature in the instdb,
//! builds canonical operands for it (registers from `reg_mask` or sequential
//! ids, sized scalar memory, MIB/T_MEM base+index memory, VSIB vector-index
//! memory, a small immediate, or a bound label for REL operands), and drives
//! the real `Assembler::emit_n` path — in both 64-bit and 32-bit mode. Every
//! instruction must emit at least one of its signatures successfully, except
//! the mode-invalid instructions in the allowlists below. Emitting runs under
//! `catch_unwind` with a panic hook that records panic locations; any encoder
//! panic fails the test.

use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::sync::Mutex;

use asmkit::Arch;
use asmkit::CodeBuffer;
use asmkit::Environment;
use asmkit::x86::Assembler;
use asmkit::x86::InstId;
use asmkit::x86::coverage::*;
use asmkit::x86::operands::{ptr, ptr_index, regs};
use asmkit::{Label, Operand, OperandCast, RegType, imm};

/// Legacy instructions with no 64-bit encoding.
const X64_INVALID: &[InstId] = &[
    InstId::Aaa,
    InstId::Aad,
    InstId::Aam,
    InstId::Aas,
    InstId::Arpl,
    InstId::Bound,
    InstId::Daa,
    InstId::Das,
    InstId::Into,
    InstId::Lds,
    InstId::Les,
    InstId::Popa,
    InstId::Popad,
    InstId::Popfd,
    InstId::Pusha,
    InstId::Pushad,
    InstId::Pushfd,
    InstId::Rsm,
];

/// X64-only instructions with no 32-bit encoding.
const X86_INVALID: &[InstId] = &[
    InstId::Cdqe,
    InstId::Clui,
    InstId::Cmpxchg16b,
    InstId::Cqo,
    InstId::Fxrstor64,
    InstId::Fxsave64,
    InstId::Incsspq,
    InstId::Iretq,
    InstId::Ldtilecfg,
    InstId::Movabs,
    InstId::Movsxd,
    InstId::Pextrq,
    InstId::Pinsrq,
    InstId::Popfq,
    InstId::Prefetchit0,
    InstId::Prefetchit1,
    InstId::Psmash,
    InstId::Pushfq,
    InstId::Rdfsbase,
    InstId::Rdgsbase,
    InstId::Rdsspq,
    InstId::Rmpadjust,
    InstId::Rmpupdate,
    InstId::Senduipi,
    InstId::Sttilecfg,
    InstId::Stui,
    InstId::Swapgs,
    InstId::Syscall,
    InstId::Sysexitq,
    InstId::Sysret,
    InstId::Sysretq,
    InstId::Tcmmimfp16ps,
    InstId::Tcmmrlfp16ps,
    InstId::Tdpbf16ps,
    InstId::Tdpbssd,
    InstId::Tdpbsud,
    InstId::Tdpbusd,
    InstId::Tdpbuud,
    InstId::Tdpfp16ps,
    InstId::Testui,
    InstId::Tileloadd,
    InstId::Tileloaddt1,
    InstId::Tilerelease,
    InstId::Tilestored,
    InstId::Tilezero,
    InstId::Uiret,
    InstId::Vpextrq,
    InstId::Vpinsrq,
    InstId::Wrfsbase,
    InstId::Wrgsbase,
    InstId::Wrssq,
    InstId::Wrussq,
    InstId::Xrstor64,
    InstId::Xrstors64,
    InstId::Xsave64,
    InstId::Xsavec64,
    InstId::Xsaveopt64,
    InstId::Xsaves64,
];

static PANIC_LOCS: Mutex<Option<HashMap<String, usize>>> = Mutex::new(None);

/// Decodes an instruction name from the instdb name tables (small-string or
/// prefix+suffix form), mirroring asmjit's `InstNameIndex` packing.
fn inst_name(id: u32) -> String {
    let v = INST_NAME_INDEX_TABLE[id as usize];
    if v & 0x8000_0000 != 0 {
        let mut s = String::new();
        let mut x = v;
        for _ in 0..6 {
            let c = x & 0x1F;
            if c == 0 {
                break;
            }
            s.push(if c <= 26 {
                (b'a' + c as u8 - 1) as char
            } else {
                (b'0' + c as u8 - 27) as char
            });
            x >>= 5;
        }
        s
    } else {
        let pb = (v & 0xFFF) as usize;
        let ps = ((v >> 12) & 0xF) as usize;
        let sb = ((v >> 16) & 0xFFF) as usize;
        let ss = ((v >> 28) & 0x7) as usize;
        let t = INST_NAME_STRING_TABLE;
        let mut s = String::from_utf8_lossy(&t[pb..pb + ps]).into_owned();
        if ss != 0 {
            s.push_str(&String::from_utf8_lossy(&t[sb..sb + ss]));
        }
        s
    }
}

fn mem_size(f: OpFlags) -> u32 {
    for (flag, size) in [
        (OpFlags::MEM8, 1),
        (OpFlags::MEM16, 2),
        (OpFlags::MEM32, 4),
        (OpFlags::MEM48, 6),
        (OpFlags::MEM64, 8),
        (OpFlags::MEM80, 10),
        (OpFlags::MEM128, 16),
        (OpFlags::MEM256, 32),
        (OpFlags::MEM512, 64),
        (OpFlags::MEM1024, 128),
    ] {
        if f.contains(flag) {
            return size;
        }
    }
    0
}

/// Builds one canonical operand for an op-signature entry. Implicit entries
/// are included — `validate_signature` also matches them when passed
/// explicitly, and their `reg_mask` pins the right register. REL entries are
/// reported via `needs_label`; the caller substitutes a bound label operand.
/// `k_seq` numbers K registers 0,1,2,...: valid for ordinary K instructions
/// and yields the even/odd pair (k0, k1) required by CONSECUTIVE_REGS
/// instructions such as vp2intersect.
fn canonical_op(
    osig: &OpSignature,
    is_32bit: bool,
    k_seq: &mut u32,
    needs_label: &mut bool,
) -> Operand {
    let f = OpFlags::from_bits_retain(osig.flags);
    let id = if osig.reg_mask != 0 {
        osig.reg_mask.trailing_zeros()
    } else {
        1
    };

    // Registers, first set flag wins.
    for (flag, kind) in [
        (OpFlags::REG_GPB_LO, 0u8),
        (OpFlags::REG_GPB_HI, 1),
        (OpFlags::REG_GPW, 2),
        (OpFlags::REG_GPD, 3),
        (OpFlags::REG_GPQ, 4),
        (OpFlags::REG_XMM, 5),
        (OpFlags::REG_YMM, 6),
        (OpFlags::REG_ZMM, 7),
        (OpFlags::REG_MM, 8),
        (OpFlags::REG_K_REG, 9),
        (OpFlags::REG_S_REG, 10),
        (OpFlags::REG_C_REG, 11),
        (OpFlags::REG_D_REG, 12),
        (OpFlags::REG_ST, 13),
        (OpFlags::REG_BND, 14),
        (OpFlags::REG_TMM, 15),
    ] {
        if f.contains(flag) {
            if kind == 9 && osig.reg_mask == 0 {
                let id = *k_seq;
                *k_seq += 1;
                return *regs::k(id).as_operand();
            }
            return match kind {
                0 => *regs::gpb_lo(id).as_operand(),
                1 => *regs::gpb_hi(id & 3).as_operand(),
                2 => *regs::gpw(id).as_operand(),
                3 => *regs::gpd(id).as_operand(),
                4 => *regs::gpq(id).as_operand(),
                5 => *regs::xmm(id).as_operand(),
                6 => *regs::ymm(id).as_operand(),
                7 => *regs::zmm(id).as_operand(),
                8 => *regs::mm(id).as_operand(),
                9 => *regs::k(id).as_operand(),
                10 => *regs::sreg(id).as_operand(),
                11 => *regs::cr(id).as_operand(),
                12 => *regs::dr(id).as_operand(),
                13 => *regs::st(id).as_operand(),
                14 => *regs::bnd(id).as_operand(),
                _ => *regs::tmm(id).as_operand(),
            };
        }
    }

    // VSIB vector memory.
    for (flag, rt, esize) in [
        (OpFlags::VM32X, RegType::Vec128, 4u32),
        (OpFlags::VM32Y, RegType::Vec256, 4),
        (OpFlags::VM32Z, RegType::Vec512, 4),
        (OpFlags::VM64X, RegType::Vec128, 8),
        (OpFlags::VM64Y, RegType::Vec256, 8),
        (OpFlags::VM64Z, RegType::Vec512, 8),
    ] {
        if f.contains(flag) {
            let mut m = if is_32bit {
                ptr(regs::gpd(1), 0, esize)
            } else {
                ptr(regs::gpq(1), 0, esize)
            };
            m.set_index_type(rt);
            m.set_index_id(2);
            return *m.as_operand();
        }
    }

    // Scalar memory; a nonzero reg_mask pins the implicit base register
    // (es:[zdi], ds:[zsi], ...).
    if f.intersects(OpFlags::MEM_MASK) {
        let size = mem_size(f);
        let base_id = if osig.reg_mask != 0 {
            osig.reg_mask.trailing_zeros()
        } else {
            1
        };
        let disp = i32::from(!f.contains(OpFlags::FLAG_MEM_BASE)) * 0x10;
        if f.intersects(OpFlags::FLAG_MIB | OpFlags::FLAG_T_MEM) {
            let m = if is_32bit {
                ptr_index(regs::gpd(base_id), regs::gpd(2), 0, disp, size)
            } else {
                ptr_index(regs::gpq(base_id), regs::gpq(2), 0, disp, size)
            };
            return *m.as_operand();
        }
        let m = if is_32bit {
            ptr(regs::gpd(base_id), disp, size)
        } else {
            ptr(regs::gpq(base_id), disp, size)
        };
        return *m.as_operand();
    }

    // Immediates.
    if f.intersects(OpFlags::IMM_MASK) {
        return *imm(1i64).as_operand();
    }

    // Relative.
    if f.intersects(OpFlags::REL_MASK) {
        *needs_label = true;
        // Placeholder; the caller substitutes a bound label operand.
        return *imm(0i64).as_operand();
    }

    Operand::new()
}

/// Tries one signature of `id` through the real emit path; returns true if
/// it emitted without error (panics are recorded by the hook and count as
/// failure).
fn try_signature(id: u32, sig: &InstSignature, is_32bit: bool) -> bool {
    let mut k_seq = 0u32;
    let mut needs_label = false;
    let mut ops: Vec<Operand> = Vec::new();
    let mut rel_slots: Vec<usize> = Vec::new();
    for r in 0..sig.op_count as usize {
        let osig = &OP_SIGNATURE_TABLE[sig.op_signature_indexes[r] as usize];
        let idx = ops.len();
        let op = canonical_op(osig, is_32bit, &mut k_seq, &mut needs_label);
        if needs_label && op.is_imm() {
            rel_slots.push(idx);
            needs_label = false;
        }
        ops.push(op);
    }

    std::panic::catch_unwind(AssertUnwindSafe(|| {
        let environment = Environment::new(if is_32bit { Arch::X86 } else { Arch::X64 });
        let mut buf = CodeBuffer::new(environment);
        let label: Option<Label> = if !rel_slots.is_empty() {
            let l = buf.get_label();
            buf.bind_label(l);
            Some(l)
        } else {
            None
        };
        if let Some(l) = label {
            for &i in &rel_slots {
                ops[i] = *l.as_operand();
            }
        }
        let refs: Vec<&Operand> = ops.iter().collect();
        let mut a = Assembler::new(&mut buf);
        a.emit_n(id, &refs);
        buf.error().is_none()
    }))
    .unwrap_or(false)
}

/// Returns the ids of instructions for which no mode-compatible signature
/// emitted successfully.
fn sweep(is_32bit: bool) -> Vec<u32> {
    let mode_bit = if is_32bit {
        Mode::X86 as u8
    } else {
        Mode::X64 as u8
    };
    let mut never = Vec::new();

    for id in 1..INST_INFO_TABLE.len() as u32 {
        let info = &INST_INFO_TABLE[id as usize];
        let common = &INST_COMMON_INFO_TABLE[info.common_info_index as usize];
        let sigs = &INST_SIGNATURE_TABLE[common.signature_index as usize
            ..common.signature_index as usize + common.signature_count as usize];

        let ok = sigs
            .iter()
            .filter(|sig| sig.mode & mode_bit != 0)
            .any(|sig| try_signature(id, sig, is_32bit));
        if !ok {
            never.push(id);
        }
    }
    never
}

/// Checks that `never` is exactly `allowlist`; on mismatch prints the
/// offending instruction names and fails.
fn check_never(mode: &str, never: &[u32], allowlist: &[InstId]) {
    let allowed: Vec<u32> = allowlist.iter().map(|&i| i as u32).collect();
    let unexpected: Vec<u32> = never
        .iter()
        .copied()
        .filter(|id| !allowed.contains(id))
        .collect();
    let fixed: Vec<u32> = allowed
        .iter()
        .copied()
        .filter(|id| !never.contains(id))
        .collect();

    for id in &unexpected {
        eprintln!(
            "{mode}: {id} {} failed to emit any signature",
            inst_name(*id)
        );
    }
    for id in &fixed {
        eprintln!(
            "{mode}: {id} {} now emits — remove it from the allowlist",
            inst_name(*id)
        );
    }
    assert!(
        unexpected.is_empty() && fixed.is_empty(),
        "{mode}: {} instruction(s) failed to emit any signature, {} allowlist entr(ies) now emit",
        unexpected.len(),
        fixed.len()
    );
}

#[test]
fn x86_signature_coverage() {
    *PANIC_LOCS.lock().unwrap() = Some(HashMap::new());
    std::panic::set_hook(Box::new(|info| {
        if let Some(loc) = info.location() {
            let mut guard = PANIC_LOCS.lock().unwrap();
            if let Some(map) = guard.as_mut() {
                *map.entry(format!("{}:{}", loc.file(), loc.line()))
                    .or_default() += 1;
            }
        }
    }));

    let never64 = sweep(false);
    let never32 = sweep(true);

    let _ = std::panic::take_hook();
    let panic_locs = PANIC_LOCS.lock().unwrap().take().unwrap();

    check_never("64-bit", &never64, X64_INVALID);
    check_never("32-bit", &never32, X86_INVALID);

    for (loc, count) in &panic_locs {
        eprintln!("encoder panic x{count} at {loc}");
    }
    assert!(
        panic_locs.is_empty(),
        "encoder panicked {} time(s) during the coverage sweep",
        panic_locs.values().sum::<usize>()
    );
}
