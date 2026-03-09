#![allow(unused_imports, dead_code, non_snake_case, clippy::all)]
extern crate alloc;
use super::dyn_emit::Instruction;
use crate::AsmError;
use crate::core::buffer::{ExternalName, RelocDistance};
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::Assembler;
use crate::x86::operands::*;
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Look up a register by its lowercase name, returning the `Operand`.
fn lookup_register(name: &str) -> Option<Operand> {
    match name {
        "al" => Some(*AL.as_operand()),
        "cl" => Some(*CL.as_operand()),
        "dl" => Some(*DL.as_operand()),
        "bl" => Some(*BL.as_operand()),
        "spl" => Some(*SPL.as_operand()),
        "bpl" => Some(*BPL.as_operand()),
        "sil" => Some(*SIL.as_operand()),
        "dil" => Some(*DIL.as_operand()),
        "r8b" => Some(*R8B.as_operand()),
        "r9b" => Some(*R9B.as_operand()),
        "r10b" => Some(*R10B.as_operand()),
        "r11b" => Some(*R11B.as_operand()),
        "r12b" => Some(*R12B.as_operand()),
        "r13b" => Some(*R13B.as_operand()),
        "r14b" => Some(*R14B.as_operand()),
        "r15b" => Some(*R15B.as_operand()),
        "ah" => Some(*AH.as_operand()),
        "ch" => Some(*CH.as_operand()),
        "dh" => Some(*DH.as_operand()),
        "bh" => Some(*BH.as_operand()),
        "ax" => Some(*AX.as_operand()),
        "cx" => Some(*CX.as_operand()),
        "dx" => Some(*DX.as_operand()),
        "bx" => Some(*BX.as_operand()),
        "sp" => Some(*SP.as_operand()),
        "bp" => Some(*BP.as_operand()),
        "si" => Some(*SI.as_operand()),
        "di" => Some(*DI.as_operand()),
        "r8w" => Some(*R8W.as_operand()),
        "r9w" => Some(*R9W.as_operand()),
        "r10w" => Some(*R10W.as_operand()),
        "r11w" => Some(*R11W.as_operand()),
        "r12w" => Some(*R12W.as_operand()),
        "r13w" => Some(*R13W.as_operand()),
        "r14w" => Some(*R14W.as_operand()),
        "r15w" => Some(*R15W.as_operand()),
        "eax" => Some(*EAX.as_operand()),
        "ecx" => Some(*ECX.as_operand()),
        "edx" => Some(*EDX.as_operand()),
        "ebx" => Some(*EBX.as_operand()),
        "esp" => Some(*ESP.as_operand()),
        "ebp" => Some(*EBP.as_operand()),
        "esi" => Some(*ESI.as_operand()),
        "edi" => Some(*EDI.as_operand()),
        "r8d" => Some(*R8D.as_operand()),
        "r9d" => Some(*R9D.as_operand()),
        "r10d" => Some(*R10D.as_operand()),
        "r11d" => Some(*R11D.as_operand()),
        "r12d" => Some(*R12D.as_operand()),
        "r13d" => Some(*R13D.as_operand()),
        "r14d" => Some(*R14D.as_operand()),
        "r15d" => Some(*R15D.as_operand()),
        "rax" => Some(*RAX.as_operand()),
        "rcx" => Some(*RCX.as_operand()),
        "rdx" => Some(*RDX.as_operand()),
        "rbx" => Some(*RBX.as_operand()),
        "rsp" => Some(*RSP.as_operand()),
        "rbp" => Some(*RBP.as_operand()),
        "rsi" => Some(*RSI.as_operand()),
        "rdi" => Some(*RDI.as_operand()),
        "r8" => Some(*R8.as_operand()),
        "r9" => Some(*R9.as_operand()),
        "r10" => Some(*R10.as_operand()),
        "r11" => Some(*R11.as_operand()),
        "r12" => Some(*R12.as_operand()),
        "r13" => Some(*R13.as_operand()),
        "r14" => Some(*R14.as_operand()),
        "r15" => Some(*R15.as_operand()),
        "xmm0" => Some(*XMM0.as_operand()),
        "xmm1" => Some(*XMM1.as_operand()),
        "xmm2" => Some(*XMM2.as_operand()),
        "xmm3" => Some(*XMM3.as_operand()),
        "xmm4" => Some(*XMM4.as_operand()),
        "xmm5" => Some(*XMM5.as_operand()),
        "xmm6" => Some(*XMM6.as_operand()),
        "xmm7" => Some(*XMM7.as_operand()),
        "xmm8" => Some(*XMM8.as_operand()),
        "xmm9" => Some(*XMM9.as_operand()),
        "xmm10" => Some(*XMM10.as_operand()),
        "xmm11" => Some(*XMM11.as_operand()),
        "xmm12" => Some(*XMM12.as_operand()),
        "xmm13" => Some(*XMM13.as_operand()),
        "xmm14" => Some(*XMM14.as_operand()),
        "xmm15" => Some(*XMM15.as_operand()),
        "xmm16" => Some(*XMM16.as_operand()),
        "xmm17" => Some(*XMM17.as_operand()),
        "xmm18" => Some(*XMM18.as_operand()),
        "xmm19" => Some(*XMM19.as_operand()),
        "xmm20" => Some(*XMM20.as_operand()),
        "xmm21" => Some(*XMM21.as_operand()),
        "xmm22" => Some(*XMM22.as_operand()),
        "xmm23" => Some(*XMM23.as_operand()),
        "xmm24" => Some(*XMM24.as_operand()),
        "xmm25" => Some(*XMM25.as_operand()),
        "xmm26" => Some(*XMM26.as_operand()),
        "xmm27" => Some(*XMM27.as_operand()),
        "xmm28" => Some(*XMM28.as_operand()),
        "xmm29" => Some(*XMM29.as_operand()),
        "xmm30" => Some(*XMM30.as_operand()),
        "xmm31" => Some(*XMM31.as_operand()),
        "ymm0" => Some(*YMM0.as_operand()),
        "ymm1" => Some(*YMM1.as_operand()),
        "ymm2" => Some(*YMM2.as_operand()),
        "ymm3" => Some(*YMM3.as_operand()),
        "ymm4" => Some(*YMM4.as_operand()),
        "ymm5" => Some(*YMM5.as_operand()),
        "ymm6" => Some(*YMM6.as_operand()),
        "ymm7" => Some(*YMM7.as_operand()),
        "ymm8" => Some(*YMM8.as_operand()),
        "ymm9" => Some(*YMM9.as_operand()),
        "ymm10" => Some(*YMM10.as_operand()),
        "ymm11" => Some(*YMM11.as_operand()),
        "ymm12" => Some(*YMM12.as_operand()),
        "ymm13" => Some(*YMM13.as_operand()),
        "ymm14" => Some(*YMM14.as_operand()),
        "ymm15" => Some(*YMM15.as_operand()),
        "ymm16" => Some(*YMM16.as_operand()),
        "ymm17" => Some(*YMM17.as_operand()),
        "ymm18" => Some(*YMM18.as_operand()),
        "ymm19" => Some(*YMM19.as_operand()),
        "ymm20" => Some(*YMM20.as_operand()),
        "ymm21" => Some(*YMM21.as_operand()),
        "ymm22" => Some(*YMM22.as_operand()),
        "ymm23" => Some(*YMM23.as_operand()),
        "ymm24" => Some(*YMM24.as_operand()),
        "ymm25" => Some(*YMM25.as_operand()),
        "ymm26" => Some(*YMM26.as_operand()),
        "ymm27" => Some(*YMM27.as_operand()),
        "ymm28" => Some(*YMM28.as_operand()),
        "ymm29" => Some(*YMM29.as_operand()),
        "ymm30" => Some(*YMM30.as_operand()),
        "ymm31" => Some(*YMM31.as_operand()),
        "zmm0" => Some(*ZMM0.as_operand()),
        "zmm1" => Some(*ZMM1.as_operand()),
        "zmm2" => Some(*ZMM2.as_operand()),
        "zmm3" => Some(*ZMM3.as_operand()),
        "zmm4" => Some(*ZMM4.as_operand()),
        "zmm5" => Some(*ZMM5.as_operand()),
        "zmm6" => Some(*ZMM6.as_operand()),
        "zmm7" => Some(*ZMM7.as_operand()),
        "zmm8" => Some(*ZMM8.as_operand()),
        "zmm9" => Some(*ZMM9.as_operand()),
        "zmm10" => Some(*ZMM10.as_operand()),
        "zmm11" => Some(*ZMM11.as_operand()),
        "zmm12" => Some(*ZMM12.as_operand()),
        "zmm13" => Some(*ZMM13.as_operand()),
        "zmm14" => Some(*ZMM14.as_operand()),
        "zmm15" => Some(*ZMM15.as_operand()),
        "zmm16" => Some(*ZMM16.as_operand()),
        "zmm17" => Some(*ZMM17.as_operand()),
        "zmm18" => Some(*ZMM18.as_operand()),
        "zmm19" => Some(*ZMM19.as_operand()),
        "zmm20" => Some(*ZMM20.as_operand()),
        "zmm21" => Some(*ZMM21.as_operand()),
        "zmm22" => Some(*ZMM22.as_operand()),
        "zmm23" => Some(*ZMM23.as_operand()),
        "zmm24" => Some(*ZMM24.as_operand()),
        "zmm25" => Some(*ZMM25.as_operand()),
        "zmm26" => Some(*ZMM26.as_operand()),
        "zmm27" => Some(*ZMM27.as_operand()),
        "zmm28" => Some(*ZMM28.as_operand()),
        "zmm29" => Some(*ZMM29.as_operand()),
        "zmm30" => Some(*ZMM30.as_operand()),
        "zmm31" => Some(*ZMM31.as_operand()),
        "mm0" => Some(*MM0.as_operand()),
        "mm1" => Some(*MM1.as_operand()),
        "mm2" => Some(*MM2.as_operand()),
        "mm3" => Some(*MM3.as_operand()),
        "mm4" => Some(*MM4.as_operand()),
        "mm5" => Some(*MM5.as_operand()),
        "mm6" => Some(*MM6.as_operand()),
        "mm7" => Some(*MM7.as_operand()),
        "k0" => Some(*K0.as_operand()),
        "k1" => Some(*K1.as_operand()),
        "k2" => Some(*K2.as_operand()),
        "k3" => Some(*K3.as_operand()),
        "k4" => Some(*K4.as_operand()),
        "k5" => Some(*K5.as_operand()),
        "k6" => Some(*K6.as_operand()),
        "k7" => Some(*K7.as_operand()),
        "st0" => Some(*ST0.as_operand()),
        "st1" => Some(*ST1.as_operand()),
        "st2" => Some(*ST2.as_operand()),
        "st3" => Some(*ST3.as_operand()),
        "st4" => Some(*ST4.as_operand()),
        "st5" => Some(*ST5.as_operand()),
        "st6" => Some(*ST6.as_operand()),
        "st7" => Some(*ST7.as_operand()),
        "st" => Some(*ST0.as_operand()),
        "bnd0" => Some(*BND0.as_operand()),
        "bnd1" => Some(*BND1.as_operand()),
        "bnd2" => Some(*BND2.as_operand()),
        "bnd3" => Some(*BND3.as_operand()),
        "tmm0" => Some(*TMM0.as_operand()),
        "tmm1" => Some(*TMM1.as_operand()),
        "tmm2" => Some(*TMM2.as_operand()),
        "tmm3" => Some(*TMM3.as_operand()),
        "tmm4" => Some(*TMM4.as_operand()),
        "tmm5" => Some(*TMM5.as_operand()),
        "tmm6" => Some(*TMM6.as_operand()),
        "tmm7" => Some(*TMM7.as_operand()),
        "es" => Some(*ES.as_operand()),
        "cs" => Some(*CS.as_operand()),
        "ss" => Some(*SS.as_operand()),
        "ds" => Some(*DS.as_operand()),
        "fs" => Some(*FS.as_operand()),
        "gs" => Some(*GS.as_operand()),
        "rip" => Some(*RIP.as_operand()),
        "cr0" => Some(*CR0.as_operand()),
        "cr2" => Some(*CR2.as_operand()),
        "cr3" => Some(*CR3.as_operand()),
        "cr4" => Some(*CR4.as_operand()),
        "cr8" => Some(*CR8.as_operand()),
        "dr0" => Some(*DR0.as_operand()),
        "dr1" => Some(*DR1.as_operand()),
        "dr2" => Some(*DR2.as_operand()),
        "dr3" => Some(*DR3.as_operand()),
        "dr6" => Some(*DR6.as_operand()),
        "dr7" => Some(*DR7.as_operand()),
        _ => None,
    }
}

/// Parse a decimal or hexadecimal integer.
fn parse_int(s: &str) -> Option<i64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()
    } else if let Some(neg) = s.strip_prefix('-') {
        parse_int(neg).map(|v| -v)
    } else {
        s.parse::<i64>().ok()
    }
}

/// Strip an optional size-override keyword prefix from a memory operand string.
/// Returns the remainder after stripping e.g. `"byte ptr"`, `"qword ptr"`, etc.
fn strip_size_prefix(s: &str) -> &str {
    let prefixes = [
        "byte ptr",
        "word ptr",
        "dword ptr",
        "qword ptr",
        "tbyte ptr",
        "tword ptr",
        "oword ptr",
        "xmmword ptr",
        "ymmword ptr",
        "zmmword ptr",
        "fword ptr",
        "byte",
        "word",
        "dword",
        "qword",
    ];
    let sl = s.trim();
    for p in &prefixes {
        if sl.to_ascii_lowercase().starts_with(p) {
            let rest = &sl[p.len()..].trim_start();
            if rest.starts_with('[') {
                return rest;
            }
        }
    }
    s
}

/// Parse a memory expression of the form `[base]`, `[base+disp]`,
/// `[base+idx*scale]`, `[base+idx*scale+disp]`, or `[abs]`.
fn parse_mem(s: &str) -> Option<Operand> {
    let s = s.trim();
    let inner = s.strip_prefix('[')?.strip_suffix(']')?.trim();

    // Split on '+' and '-' while preserving sign
    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0usize;
    let bytes = inner.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i] == b'+' || bytes[i] == b'-' {
            parts.push(inner[start..i].trim());
            start = i;
        }
    }
    parts.push(inner[start..].trim());

    let mut base: Option<Operand> = None;
    let mut index: Option<Operand> = None;
    let mut scale: u32 = 1;
    let mut disp: i32 = 0;

    for part in &parts {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        // Check for idx*scale
        if let Some(star_pos) = part.find('*') {
            let idx_s = part[..star_pos].trim();
            let sc_s = part[star_pos + 1..].trim();
            if let Some(reg) = lookup_register(&idx_s.to_ascii_lowercase()) {
                if let Some(sc) = parse_int(sc_s) {
                    index = Some(reg);
                    scale = sc as u32;
                    continue;
                }
            }
        }
        // Displacement (with leading sign already included)
        if let Some(v) = parse_int(part) {
            disp = disp.wrapping_add(v as i32);
            continue;
        }
        // Register (base)
        let name_l = part.to_ascii_lowercase();
        let name_l = name_l.trim_start_matches('+').trim_start_matches('-');
        if let Some(reg) = lookup_register(name_l) {
            if base.is_none() {
                base = Some(reg);
            } else if index.is_none() {
                index = Some(reg);
            }
            continue;
        }
        return None; // unrecognised token
    }

    // Build a Mem operand
    use crate::core::operand::{OperandSignature, OperandType};
    use crate::x86::operands::{AddrType, Mem};
    let sig_base = OperandSignature::from_op_type(OperandType::Mem);
    let mem = match (base, index) {
        (Some(b), Some(idx)) => {
            let b_reg = Reg::from_operand(&b);
            let ix_reg = Reg::from_operand(&idx);
            let shift_val = match scale {
                2 => 1,
                4 => 2,
                8 => 3,
                _ => 0,
            };
            Mem::from_base_and_index_shift_disp(&*b_reg, &*ix_reg, shift_val, disp, 0, sig_base)
        }
        (Some(b), None) => {
            let b_reg = Reg::from_operand(&b);
            Mem::from_base_and_disp(&*b_reg, disp, 0, sig_base)
        }
        (None, None) => {
            // Treat as absolute 64-bit address
            let abs = disp as u64;
            Mem::from_u64(
                abs,
                0,
                OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(
                    AddrType::Abs as _,
                ),
            )
        }
        _ => return None,
    };
    Some(*mem.as_operand())
}

/// Check if `s` looks like an assembler identifier
/// (starts with a letter or `_`, followed by alphanumerics, `_`, or `.`).
fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
}

/// Try to strip a leading `name:` label definition from `text`.
/// Returns `(label_name, tail_after_colon)`, or `None` if no valid label prefix.
///
/// Colons inside brackets (memory expressions) are ignored.
fn try_strip_label<'a>(text: &'a str) -> Option<(&'a str, &'a str)> {
    let mut depth: usize = 0;
    for (i, b) in text.bytes().enumerate() {
        match b {
            b'[' | b'(' => depth += 1,
            b']' | b')' => depth = depth.saturating_sub(1),
            // Whitespace before the colon means this is not a label prefix.
            b' ' | b'\t' if depth == 0 => return None,
            b':' if depth == 0 => {
                let label_name = text[..i].trim();
                if is_identifier(label_name) {
                    return Some((label_name, &text[i + 1..]));
                }
                return None;
            }
            _ => {}
        }
    }
    None
}

/// Split a comma-delimited operand list, respecting bracket nesting so that
/// memory expressions like `[rax + rbx*4 + 8]` are not split internally.
fn split_operands(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth: usize = 0;
    let mut start = 0usize;
    for (i, b) in s.bytes().enumerate() {
        match b {
            b'[' | b'(' | b'{' => depth += 1,
            b']' | b')' | b'}' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                result.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    result.push(&s[start..]);
    result
}

/// Like [`parse_mem`] but falls back to the name-resolution tables in `ctx`
/// for base addresses that are labels or external symbols.
fn parse_mem_with_ctx(s: &str, ctx: &mut AsmContext, asm: &mut Assembler<'_>) -> Option<Operand> {
    let s = s.trim();
    let inner = s.strip_prefix('[')?.strip_suffix(']')?.trim();

    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0usize;
    let bytes = inner.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i] == b'+' || bytes[i] == b'-' {
            parts.push(inner[start..i].trim());
            start = i;
        }
    }
    parts.push(inner[start..].trim());

    let mut base_reg: Option<Operand> = None;
    let mut base_label: Option<Label> = None;
    let mut base_sym: Option<Sym> = None;
    let mut index: Option<Operand> = None;
    let mut scale: u32 = 1;
    let mut disp: i32 = 0;

    for part in &parts {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        // idx*scale
        if let Some(star_pos) = part.find('*') {
            let idx_s = part[..star_pos].trim();
            let sc_s = part[star_pos + 1..].trim();
            if let Some(reg) = lookup_register(&idx_s.to_ascii_lowercase()) {
                if let Some(sc) = parse_int(sc_s) {
                    index = Some(reg);
                    scale = sc as u32;
                    continue;
                }
            }
        }
        // Signed displacement
        if let Some(v) = parse_int(part) {
            disp = disp.wrapping_add(v as i32);
            continue;
        }
        // Strip leading sign for name lookup
        let name_raw = part.trim_start_matches('+').trim_start_matches('-');
        let name_lo = name_raw.to_ascii_lowercase();
        if let Some(reg) = lookup_register(&name_lo) {
            if base_reg.is_none() && base_label.is_none() && base_sym.is_none() {
                base_reg = Some(reg);
            } else if index.is_none() {
                index = Some(reg);
            }
            continue;
        }
        // Identifier → external symbol (if declared) or label (forward-declared if needed)
        if is_identifier(name_raw) {
            let no_base = base_reg.is_none() && base_label.is_none() && base_sym.is_none();
            if no_base {
                if let Some(sym) = ctx.get_sym(name_raw) {
                    base_sym = Some(sym);
                } else {
                    base_label = Some(ctx.get_or_create_label(asm, name_raw));
                }
                continue;
            }
        }
        return None;
    }

    use crate::core::operand::{OperandSignature, OperandType};
    use crate::x86::operands::{AddrType, Mem};
    let sig_base = OperandSignature::from_op_type(OperandType::Mem);
    let shift_for = |sc: u32| match sc {
        2 => 1,
        4 => 2,
        8 => 3,
        _ => 0,
    };
    let mem = match (base_reg, base_label, base_sym, index) {
        (Some(b), None, None, Some(idx)) => {
            let b_reg = Reg::from_operand(&b);
            let ix_reg = Reg::from_operand(&idx);
            Mem::from_base_and_index_shift_disp(
                &*b_reg,
                &*ix_reg,
                shift_for(scale),
                disp,
                0,
                sig_base,
            )
        }
        (Some(b), None, None, None) => {
            let b_reg = Reg::from_operand(&b);
            Mem::from_base_and_disp(&*b_reg, disp, 0, sig_base)
        }
        (None, Some(lbl), None, Some(idx)) => {
            let ix_reg = Reg::from_operand(&idx);
            Mem::from_label_and_index_shift_disp(
                &lbl,
                &*ix_reg,
                shift_for(scale),
                disp,
                0,
                sig_base,
            )
        }
        (None, Some(lbl), None, None) => Mem::from_label_and_disp(&lbl, disp, 0, sig_base),
        (None, None, Some(sym), Some(idx)) => {
            let ix_reg = Reg::from_operand(&idx);
            Mem::from_sym_and_index_shift_disp(&sym, &*ix_reg, shift_for(scale), disp, 0, sig_base)
        }
        (None, None, Some(sym), None) => Mem::from_sym_and_disp(&sym, disp, 0, sig_base),
        (None, None, None, None) => {
            let abs = disp as u64;
            Mem::from_u64(
                abs,
                0,
                OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(
                    AddrType::Abs as _,
                ),
            )
        }
        _ => return None,
    };
    Some(*mem.as_operand())
}

/// Match an ASCII keyword at the start of `text` (case-insensitive).
/// Returns the trimmed remainder, or `None` if not present or immediately
/// followed by a non-whitespace character (prevents matching "external" for "extern").
fn strip_keyword<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    if text.len() < keyword.len() {
        return None;
    }
    if text[..keyword.len()].eq_ignore_ascii_case(keyword) {
        let rest = &text[keyword.len()..];
        if rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace()) {
            return Some(rest.trim_start());
        }
    }
    None
}

/// Parser context that tracks named labels and external symbols across multiple
/// `assemble_line` calls, enabling forward label references and `.extern`
/// symbol declarations.
///
/// # Example
/// ```rust,ignore
/// let mut ctx  = AsmContext::new();
/// let mut buf  = CodeBuffer::default();
/// let mut asm  = Assembler::new(&mut buf);
///
/// ctx.assemble(&mut asm, "
///     .extern near printf
///     call printf
/// printf_done:
///     nop
/// ")?;
/// ```
pub struct AsmContext {
    labels: BTreeMap<String, Label>,
    symbols: BTreeMap<String, Sym>,
}

impl Default for AsmContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AsmContext {
    /// Create a new, empty context.
    pub fn new() -> Self {
        Self {
            labels: BTreeMap::new(),
            symbols: BTreeMap::new(),
        }
    }

    /// Return (or lazily allocate) the [`Label`] for `name`.
    ///
    /// Forward references are supported: calling this before the `name:` line
    /// is assembled is safe — the label will be bound when that line is reached.
    pub fn get_or_create_label(&mut self, asm: &mut Assembler<'_>, name: &str) -> Label {
        if let Some(&lbl) = self.labels.get(name) {
            return lbl;
        }
        let lbl = asm.get_label();
        self.labels.insert(name.to_string(), lbl);
        lbl
    }

    /// Look up a previously declared external symbol by name.
    pub fn get_sym(&self, name: &str) -> Option<Sym> {
        self.symbols.get(name).copied()
    }

    /// Declare (or retrieve) an external symbol.
    ///
    /// A second call with the same `name` is a no-op; the original [`Sym`]
    /// is returned and its distance is left unchanged.
    pub fn add_extern(
        &mut self,
        asm: &mut Assembler<'_>,
        name: &str,
        distance: RelocDistance,
    ) -> Sym {
        if let Some(&sym) = self.symbols.get(name) {
            return sym;
        }
        let sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol(Cow::Owned(name.to_string())), distance);
        self.symbols.insert(name.to_string(), sym);
        sym
    }

    /// Parse a single operand string, resolving register names, integer
    /// immediates, memory expressions, label references, and external-symbol
    /// references.
    ///
    /// Unknown identifiers are treated as forward-declared labels.
    pub fn parse_operand(&mut self, asm: &mut Assembler<'_>, s: &str) -> Result<Operand, AsmError> {
        let s = s.trim();
        let mem_s = strip_size_prefix(s);
        if mem_s.starts_with('[') {
            return parse_mem_with_ctx(mem_s, self, asm).ok_or(AsmError::InvalidOperand);
        }
        if let Some(reg) = lookup_register(&s.to_ascii_lowercase()) {
            return Ok(reg);
        }
        if let Some(v) = parse_int(s) {
            return Ok(*crate::core::operand::imm(v).as_operand());
        }
        if is_identifier(s) {
            // Declared external symbol takes priority over a label.
            if let Some(sym) = self.get_sym(s) {
                return Ok(*sym.as_operand());
            }
            // Forward-declared (or already existing) label.
            let lbl = self.get_or_create_label(asm, s);
            return Ok(*lbl.as_operand());
        }
        Err(AsmError::InvalidOperand)
    }

    /// Assemble a single line of Intel-syntax x86 assembly, with full label
    /// and external-symbol support.
    ///
    /// Recognised pseudo-directives:
    ///
    /// | Syntax | Effect |
    /// |--------|---------|
    /// | `.extern name[, name…]` | Declare near external symbol(s) |
    /// | `.extern near name[, name…]` | Declare near external symbol(s) |
    /// | `.extern far name[, name…]` | Declare far (GOT-indirect) symbol(s) |
    /// | `name:` | Define a label at the current position |
    /// | `name: <insn>` | Define a label, then assemble the instruction |
    ///
    /// Blank lines and lines whose first non-whitespace character is `;` or `#`
    /// are silently ignored.
    pub fn assemble_line(&mut self, asm: &mut Assembler<'_>, text: &str) -> Result<(), AsmError> {
        let text = text.trim();
        if text.is_empty() || text.starts_with(';') || text.starts_with('#') {
            return Ok(());
        }
        // Strip inline comments.
        let text = text.split(';').next().unwrap_or("").trim();
        let text = text.split('#').next().unwrap_or("").trim();

        // Handle `.extern` / `extern` directive.
        let after_extern = strip_keyword(text, ".extern").or_else(|| strip_keyword(text, "extern"));
        if let Some(rest) = after_extern {
            let (distance, names) = if let Some(r) = strip_keyword(rest, "near") {
                (RelocDistance::Near, r)
            } else if let Some(r) = strip_keyword(rest, "far") {
                (RelocDistance::Far, r)
            } else {
                (RelocDistance::Near, rest)
            };
            for name in names.split(',') {
                let name = name.trim();
                if !name.is_empty() && is_identifier(name) {
                    self.add_extern(asm, name, distance);
                }
            }
            return Ok(());
        }

        // Handle a `name:` label definition prefix.
        let remaining = if let Some((label_name, tail)) = try_strip_label(text) {
            let lbl = self.get_or_create_label(asm, label_name);
            asm.bind_label(lbl);
            let tail = tail.trim();
            if tail.is_empty() {
                return Ok(());
            }
            tail
        } else {
            text
        };

        // Emit the instruction.
        self.emit_instruction(asm, remaining)
    }

    fn emit_instruction(&mut self, asm: &mut Assembler<'_>, text: &str) -> Result<(), AsmError> {
        let (mnem, rest) = if let Some(pos) = text.find(|c: char| c.is_ascii_whitespace()) {
            (&text[..pos], text[pos..].trim())
        } else {
            (text, "")
        };
        let ops: Vec<Operand> = if rest.is_empty() {
            Vec::new()
        } else {
            split_operands(rest)
                .into_iter()
                .map(|s| self.parse_operand(asm, s.trim()))
                .collect::<Result<Vec<_>, _>>()?
        };
        asm.emit_dyn(mnem, &ops)
    }

    /// Assemble multiple lines of Intel-syntax x86 assembly, sharing a
    /// single label/symbol namespace across all lines (forward refs work).
    pub fn assemble(&mut self, asm: &mut Assembler<'_>, text: &str) -> Result<(), AsmError> {
        for line in text.lines() {
            self.assemble_line(asm, line)?;
        }
        Ok(())
    }
}

/// Parse a single operand token into an [`Operand`].
///
/// Supports register names (case-insensitive), integer immediates
/// (decimal and `0x`-prefixed hex), and memory expressions `[...]`
/// with optional `byte/word/dword/qword ptr` size override prefixes.
pub fn parse_operand(s: &str) -> Result<Operand, AsmError> {
    let s = s.trim();
    let mem_s = strip_size_prefix(s);
    if mem_s.starts_with('[') {
        return parse_mem(mem_s).ok_or(AsmError::InvalidOperand);
    }
    if let Some(reg) = lookup_register(&s.to_ascii_lowercase()) {
        return Ok(reg);
    }
    if let Some(v) = parse_int(s) {
        return Ok(*crate::core::operand::imm(v).as_operand());
    }
    Err(AsmError::InvalidOperand)
}

/// Parse and assemble a single line of x86 Intel-syntax assembly text.
///
/// # Syntax
/// ```text
/// mnemonic [op0 [, op1 [, op2 [, op3]]]]
/// ```
/// Blank lines and lines whose first non-whitespace character is `;` or `#`
/// are silently ignored.
///
/// Label definitions (`name:`) and `.extern` directives are recognised, but
/// because a fresh [`AsmContext`] is created per call, forward label references
/// across separate calls will not resolve.  Use [`AsmContext::assemble_line`]
/// for multi-line assembly with forward references.
///
/// # Errors
/// - [`AsmError::InvalidInstruction`] — unknown mnemonic.
/// - [`AsmError::InvalidOperand`] — unrecognised operand or wrong types.
pub fn assemble_line(asm: &mut Assembler<'_>, text: &str) -> Result<(), AsmError> {
    AsmContext::new().assemble_line(asm, text)
}

/// Parse and assemble multiple lines of x86 Intel-syntax assembly text.
///
/// A single [`AsmContext`] is shared across all lines, so forward label
/// references and `.extern` declarations work correctly within `text`.
/// For multi-call scenarios, create an [`AsmContext`] and call
/// [`AsmContext::assemble`] directly.
pub fn assemble(asm: &mut Assembler<'_>, text: &str) -> Result<(), AsmError> {
    AsmContext::new().assemble(asm, text)
}
