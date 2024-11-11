use super::decode::{Decoder, Instruction, OperandType, RegType};
use super::decode_tab::{Opcode, MNEMONIC_LENS, MNEMONIC_OFFS, MNEMONIC_STR};

use core::fmt::{self, Write};

fn strpcatnum<W: Write>(w: &mut W, mut val: u64) -> fmt::Result {
    let lz = (val | 1).leading_zeros() as usize;
    let numbytes = 16 - (lz / 4);
    let mut dst = [0u8; 32];
    let mut idx = numbytes + 2;
    loop {
        idx -= 1;
        dst[idx] = b"0123456789abcdef"[(val % 16) as usize];
        val /= 16;
        if val == 0 {
            break;
        }
    }

    dst[0] = b'0';
    dst[1] = b'x';

    write!(w, "{}", unsafe {
        core::str::from_utf8_unchecked(&dst[..numbytes + 2])
    })
}

pub fn strpcatreg<W: Write>(w: &mut W, rt: RegType, ri: usize, size: usize) -> fmt::Result {
    let nametab = [
        2, 97, 108, 4, 98, 110, 100, 48, 2, 99, 108, 4, 98, 110, 100, 49, 2, 100, 108, 4, 98, 110,
        100, 50, 2, 98, 108, 4, 98, 110, 100, 51, 3, 115, 112, 108, 0, 32, 32, 32, 3, 98, 112, 108,
        0, 32, 32, 32, 3, 115, 105, 108, 0, 32, 32, 32, 3, 100, 105, 108, 0, 32, 32, 32, 3, 114,
        56, 98, 0, 32, 32, 32, 3, 114, 57, 98, 0, 32, 32, 32, 4, 114, 49, 48, 98, 0, 32, 32, 4,
        114, 49, 49, 98, 0, 32, 32, 4, 114, 49, 50, 98, 2, 97, 104, 4, 114, 49, 51, 98, 2, 99, 104,
        4, 114, 49, 52, 98, 2, 100, 104, 4, 114, 49, 53, 98, 2, 98, 104, 0, 0, 32, 32, 32, 32, 32,
        32, 2, 97, 120, 4, 116, 109, 109, 48, 2, 99, 120, 4, 116, 109, 109, 49, 2, 100, 120, 4,
        116, 109, 109, 50, 2, 98, 120, 4, 116, 109, 109, 51, 2, 115, 112, 4, 116, 109, 109, 52, 2,
        98, 112, 4, 116, 109, 109, 53, 2, 115, 105, 4, 116, 109, 109, 54, 2, 100, 105, 4, 116, 109,
        109, 55, 3, 114, 56, 119, 32, 2, 101, 115, 3, 114, 57, 119, 32, 2, 99, 115, 4, 114, 49, 48,
        119, 2, 115, 115, 4, 114, 49, 49, 119, 2, 100, 115, 4, 114, 49, 50, 119, 2, 102, 115, 4,
        114, 49, 51, 119, 2, 103, 115, 4, 114, 49, 52, 119, 0, 32, 32, 4, 114, 49, 53, 119, 0, 32,
        32, 2, 105, 112, 0, 32, 32, 32, 32, 3, 101, 97, 120, 3, 109, 109, 48, 3, 101, 99, 120, 3,
        109, 109, 49, 3, 101, 100, 120, 3, 109, 109, 50, 3, 101, 98, 120, 3, 109, 109, 51, 3, 101,
        115, 112, 3, 109, 109, 52, 3, 101, 98, 112, 3, 109, 109, 53, 3, 101, 115, 105, 3, 109, 109,
        54, 3, 101, 100, 105, 3, 109, 109, 55, 3, 114, 56, 100, 32, 2, 107, 48, 3, 114, 57, 100,
        32, 2, 107, 49, 4, 114, 49, 48, 100, 2, 107, 50, 4, 114, 49, 49, 100, 2, 107, 51, 4, 114,
        49, 50, 100, 2, 107, 52, 4, 114, 49, 51, 100, 2, 107, 53, 4, 114, 49, 52, 100, 2, 107, 54,
        4, 114, 49, 53, 100, 2, 107, 55, 3, 101, 105, 112, 0, 32, 32, 32, 3, 114, 97, 120, 3, 99,
        114, 48, 3, 114, 99, 120, 0, 32, 32, 32, 3, 114, 100, 120, 3, 99, 114, 50, 3, 114, 98, 120,
        3, 99, 114, 51, 3, 114, 115, 112, 3, 99, 114, 52, 3, 114, 98, 112, 0, 32, 32, 32, 3, 114,
        115, 105, 0, 32, 32, 32, 3, 114, 100, 105, 0, 32, 32, 32, 2, 114, 56, 32, 3, 99, 114, 56,
        2, 114, 57, 32, 3, 100, 114, 48, 3, 114, 49, 48, 3, 100, 114, 49, 3, 114, 49, 49, 3, 100,
        114, 50, 3, 114, 49, 50, 3, 100, 114, 51, 3, 114, 49, 51, 3, 100, 114, 52, 3, 114, 49, 52,
        3, 100, 114, 53, 3, 114, 49, 53, 3, 100, 114, 54, 3, 114, 105, 112, 3, 100, 114, 55, 5,
        115, 116, 40, 48, 41, 0, 32, 5, 115, 116, 40, 49, 41, 0, 32, 5, 115, 116, 40, 50, 41, 0,
        32, 5, 115, 116, 40, 51, 41, 0, 32, 5, 115, 116, 40, 52, 41, 0, 32, 5, 115, 116, 40, 53,
        41, 0, 32, 5, 115, 116, 40, 54, 41, 0, 32, 5, 115, 116, 40, 55, 41, 0, 32, 4, 120, 109,
        109, 48, 0, 32, 32, 4, 120, 109, 109, 49, 0, 32, 32, 4, 120, 109, 109, 50, 0, 32, 32, 4,
        120, 109, 109, 51, 0, 32, 32, 4, 120, 109, 109, 52, 0, 32, 32, 4, 120, 109, 109, 53, 0, 32,
        32, 4, 120, 109, 109, 54, 0, 32, 32, 4, 120, 109, 109, 55, 0, 32, 32, 4, 120, 109, 109, 56,
        0, 32, 32, 4, 120, 109, 109, 57, 0, 32, 32, 5, 120, 109, 109, 49, 48, 0, 32, 5, 120, 109,
        109, 49, 49, 0, 32, 5, 120, 109, 109, 49, 50, 0, 32, 5, 120, 109, 109, 49, 51, 0, 32, 5,
        120, 109, 109, 49, 52, 0, 32, 5, 120, 109, 109, 49, 53, 0, 32, 5, 120, 109, 109, 49, 54, 0,
        32, 5, 120, 109, 109, 49, 55, 0, 32, 5, 120, 109, 109, 49, 56, 0, 32, 5, 120, 109, 109, 49,
        57, 0, 32, 5, 120, 109, 109, 50, 48, 0, 32, 5, 120, 109, 109, 50, 49, 0, 32, 5, 120, 109,
        109, 50, 50, 0, 32, 5, 120, 109, 109, 50, 51, 0, 32, 5, 120, 109, 109, 50, 52, 0, 32, 5,
        120, 109, 109, 50, 53, 0, 32, 5, 120, 109, 109, 50, 54, 0, 32, 5, 120, 109, 109, 50, 55, 0,
        32, 5, 120, 109, 109, 50, 56, 0, 32, 5, 120, 109, 109, 50, 57, 0, 32, 5, 120, 109, 109, 51,
        48, 0, 32, 5, 120, 109, 109, 51, 49, 0, 32, 0,
    ];

    let nametab_idx = [608u16, 0, 69, 205, 544, 276, 139, 341, 3, 412, 484];
    let idx = if rt == RegType::Gpl {
        size * 17 * 8
    } else {
        nametab_idx[rt as usize] as usize
    };

    let mut dst = [0; 16];
    let name = &nametab[idx + 8 * ri..];

    for i in 0..8 {
        dst[i] = name[i + 1];
    }

    if rt == RegType::Vec && size > 4 {
        dst[0] += size as u8 - 4;
    }

    write!(
        w,
        "{}",
        core::str::from_utf8(&dst[..name[0] as usize]).unwrap()
    )
}

fn mnemonic<W: Write>(w: &mut W, inst: &Instruction) -> fmt::Result {
    let mut mnem = &MNEMONIC_STR[MNEMONIC_OFFS[inst.typ as usize] as usize..];
    let mut mnem_len = MNEMONIC_LENS[inst.code() as usize];

    let mut prefix_xacq_xrel = false;
    let mut prefix_segment = false;

    let mut sizesuffix = [0u8; 4];
    let mut sizesuffixlen = 0;

    if inst.op_type(0) == OperandType::Off && inst.op_size_log(0) == 1 {
        sizesuffix[0] = b'w';
        sizesuffixlen = 0;
    }

    match inst.code() {
        Opcode::C_SEP => {
            mnem = &mnem[inst.opsize() & 0xc..];
            mnem_len = 3;
        }

        Opcode::C_EX => {
            mnem = &mnem[inst.opsize() & 0xc..];
            mnem_len = if inst.opsize() < 4 { 3 } else { 4 };
        }

        Opcode::CMPXCHGD => match inst.opsize_log() {
            2 => {
                sizesuffix[0] = b'8';
                sizesuffix[1] = b'b';
                sizesuffixlen = 2;
            }
            3 => {
                sizesuffix[0] = b'1';
                sizesuffix[1] = b'6';
                sizesuffix[2] = b'b';
                sizesuffixlen = 3;
            }

            _ => (),
        },

        Opcode::JCXZ => {
            mnem_len = if inst.addrsize_log() == 1 { 4 } else { 5 };
            mnem = &mnem[5 * (inst.addrsize_log() - 1)..];
        }

        Opcode::PUSH => {
            if inst.op_size_log(0) == 1 && inst.op_type(0) == OperandType::Imm {
                sizesuffix[0] = b'w';
                sizesuffixlen = 1;
            }
            if inst.op_size_log(0) == 1
                && inst.op_type(0) == OperandType::Reg
                && inst.op_reg_type(0) == Some(RegType::Seg)
            {
                sizesuffix[0] = b'w';
                sizesuffixlen = 1;
            }
        }

        Opcode::POP => {
            if inst.op_size_log(0) == 1
                && inst.op_type(0) == OperandType::Reg
                && inst.op_reg_type(0) == Some(RegType::Seg)
            {
                sizesuffix[0] = b'w';
                sizesuffixlen = 1;
            }
        }

        Opcode::MOV => {
            if inst.has_rep()
                && inst.op_type(0) == OperandType::Mem
                && inst.op_type(1) == OperandType::Imm
            {
                prefix_xacq_xrel = true;
            }
        }

        Opcode::FXSAVE
        | Opcode::FXRSTOR
        | Opcode::XSAVE
        | Opcode::XSAVEC
        | Opcode::XSAVEOPT
        | Opcode::XSAVES
        | Opcode::XRSTOR
        | Opcode::XRSTORS => {
            if inst.opsize_log() == 3 {
                sizesuffix[0] = b'6';
                sizesuffix[1] = b'4';
                sizesuffixlen = 2;
            }
        }

        Opcode::EVX_MOV_G2X | Opcode::EVX_MOV_X2G => {
            sizesuffix[0] = b"bwdq"[inst.op_size_log(0)];
            sizesuffixlen = 1;
        }

        Opcode::EVX_PBROADCAST => {
            sizesuffix[0] = b"bwdq"[inst.op_size_log(1)];
            sizesuffixlen = 1;
        }

        Opcode::EVX_PINSR => {
            sizesuffix[0] = b"bwdq"[inst.op_size_log(2)];
            sizesuffixlen = 1;
        }

        Opcode::RET | Opcode::ENTER | Opcode::LEAVE => {
            if inst.opsize_log() == 1 {
                sizesuffix[0] = b'w';
                sizesuffixlen = 1;
            }
        }

        Opcode::LODS | Opcode::MOVS | Opcode::CMPS | Opcode::OUTS => {
            prefix_segment = true;
            if inst.has_rep() {
                write!(w, "rep ")?;
            }

            if inst.has_repnz() {
                write!(w, "repnz ")?;
            }

            if inst.is_64() && inst.addrsize_log() == 2 {
                write!(w, "addr32 ")?;
            }

            if !inst.is_64() && inst.addrsize_log() == 1 {
                write!(w, "addr16 ")?;
            }
        }

        Opcode::STOS | Opcode::SCAS | Opcode::INS => {
            if inst.has_rep() {
                write!(w, "rep ")?;
            }

            if inst.has_repnz() {
                write!(w, "repnz ")?;
            }

            if inst.is_64() && inst.addrsize_log() == 2 {
                write!(w, "addr32 ")?;
            }

            if !inst.is_64() && inst.addrsize_log() == 1 {
                write!(w, "addr16 ")?;
            }
        }

        Opcode::PUSHA
        | Opcode::POPA
        | Opcode::PUSHF
        | Opcode::POPF
        | Opcode::RETF
        | Opcode::IRET
        | Opcode::IN
        | Opcode::OUT => {
            sizesuffix[0] = b"bwdq"[inst.opsize_log()];
            sizesuffixlen = 1;
        }

        _ => (),
    }

    if prefix_xacq_xrel || inst.has_lock() {
        if inst.has_rep() {
            write!(w, "xrelease ")?;
        }

        if inst.has_repnz() {
            write!(w, "xacquire ")?;
        }
    }

    if inst.has_lock() {
        write!(w, "lock ")?;
    }

    if prefix_segment && inst.segment().is_some() {
        write!(w, "{}s ", b"ecsdfg"[inst.segment as usize & 7])?;
    }

    for c in mnem[..mnem_len as usize].chars() {
        write!(w, "{}", c.to_lowercase())?;
    }

    write!(
        w,
        "{}",
        core::str::from_utf8(&sizesuffix[..sizesuffixlen]).unwrap()
    )?;
    Ok(())
}

pub struct Formatter {}

impl Formatter {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn format<W: Write>(&self, out: &mut W, inst: &Instruction) -> fmt::Result {
        mnemonic(out, inst)?;

        for i in 0..4 {
            let op_type = inst.op_type(i);
            if op_type == OperandType::None {
                break;
            }

            if i > 0 {
                write!(out, ",")?;
            }

            write!(out, " ")?;

            let mut size = inst.op_size_log(i);
            if size == 0 {
                size = inst.addrsize_log();
            }

            if op_type == OperandType::Reg {
                let typ = inst.op_reg_type(i).unwrap();
                let idx = inst.operands[i].reg as usize;
                strpcatreg(out, typ, idx as _, size)?;
            } else if op_type == OperandType::Mem || op_type == OperandType::MemBCST {
                let mut idx_rt = RegType::Gpl;
                let mut idx_sz = inst.addrsize_log();
                use Opcode::*;
                match inst.code() {
                    CMPXCHGD => {
                        size = inst.opsize_log() + 1;
                    }
                    BOUND => {
                        size += 1;
                    }

                    JMPF | CALLF | LDS | LES | LFS | LGS | LSS => size += 6,
                    FLD | FSTP | FBLD | FBSTP => {
                        size = if size != 0 { size } else { 9 };
                    }

                    VPGATHERQD | VGATHERQPS | EVX_PGATHERQD | EVX_GATHERQPS => {
                        idx_rt = RegType::Vec;
                        idx_sz = inst.op_size_log(0) + 1;
                    }

                    EVX_PSCATTERQD | EVX_SCATTERQPS => {
                        idx_rt = RegType::Vec;
                        idx_sz = inst.op_size_log(1) + 1;
                    }

                    VPGATHERDD | VPGATHERQQ | VGATHERDPS | VGATHERQPD | EVX_PGATHERDD
                    | EVX_PGATHERQQ | EVX_GATHERDPS | EVX_GATHERQPD => {
                        idx_rt = RegType::Vec;
                        idx_sz = inst.op_size_log(0);
                    }

                    EVX_PSCATTERDD | EVX_PSCATTERQQ | EVX_SCATTERDPS | EVX_SCATTERQPD => {
                        idx_rt = RegType::Vec;
                        idx_sz = inst.op_size_log(1);
                    }
                    _ => (),
                }

                if op_type == OperandType::MemBCST {
                    size = inst.op_bcstsz_log(i);
                }

                static PTR_SIZES: [u8; 177] = [
                    0, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 9, 98, 121, 116,
                    101, 32, 112, 116, 114, 32, 32, 32, 32, 32, 32, 32, 9, 119, 111, 114, 100, 32,
                    112, 116, 114, 32, 32, 32, 32, 32, 32, 32, 10, 100, 119, 111, 114, 100, 32,
                    112, 116, 114, 32, 32, 32, 32, 32, 32, 10, 113, 119, 111, 114, 100, 32, 112,
                    116, 114, 32, 32, 32, 32, 32, 32, 12, 120, 109, 109, 119, 111, 114, 100, 32,
                    112, 116, 114, 32, 32, 32, 32, 12, 121, 109, 109, 119, 111, 114, 100, 32, 112,
                    116, 114, 32, 32, 32, 32, 12, 122, 109, 109, 119, 111, 114, 100, 32, 112, 116,
                    114, 32, 32, 32, 32, 10, 100, 119, 111, 114, 100, 32, 112, 116, 114, 32, 32,
                    32, 32, 32, 32, 10, 102, 119, 111, 114, 100, 32, 112, 116, 114, 32, 32, 32, 32,
                    32, 32, 10, 116, 98, 121, 116, 101, 32, 112, 116, 114, 32, 32, 32, 32, 32, 32,
                    0,
                ];

                let ptrsize = &PTR_SIZES[16 * (size + 1)..];
                let len = ptrsize[0];
                write!(
                    out,
                    "{}",
                    core::str::from_utf8(&ptrsize[1..len as usize]).unwrap()
                )?;

                if let Some(seg) = inst.segment() {
                    write!(out, "{}s:", b"ecsdfg\0"[seg as usize & 7] as char)?
                }

                write!(out, "[")?;

                let has_base = inst.op_base(i).is_some();
                let has_idx = inst.op_index(i).is_some();

                if has_base {
                    strpcatreg(
                        out,
                        RegType::Gpl,
                        inst.op_base(i).unwrap() as _,
                        inst.addrsize_log(),
                    )?;
                }

                if has_idx {
                    if has_base {
                        write!(out, "+")?
                    }

                    write!(out, "0{}", char::from_u32(1 << inst.op_scale(i)).unwrap())?;
                    write!(out, "*")?;
                    println!("{:?}", inst.op_index(i).unwrap());
                    strpcatreg(out, idx_rt, inst.op_index(i).unwrap() as _, idx_sz)?;
                }

                let mut disp = inst.op_disp(i);

                if disp != 0 && (has_base || has_idx) {
                    write!(out, "{}", if disp < 0 { "-" } else { "+" })?;

                    if disp < 0 {
                        disp = disp.wrapping_neg();
                    }

                    if inst.addrsize_log() == 1 {
                        disp &= 0xffff;
                    } else if inst.addrsize_log() == 2 {
                        disp &= 0xffffffff;
                    }

                    if disp != 0 || (!has_base && !has_idx) {
                        strpcatnum(out, disp as _)?;
                    }

                    
                }
                write!(out, "]")?;
            } else if op_type == OperandType::Imm || op_type == OperandType::Off {
                let mut immediate = inst.op_imm(i) as u64;
                use Opcode::*;
                match inst.code() {
                    SSE_EXTRQ | SSE_INSERTQ => {
                        write!(out, "0x{:x}", immediate & 0xff)?;
                        write!(out, ", ")?;
                        immediate = (immediate >> 8) & 0xff;
                    }

                    ENTER => {
                        write!(out, "0x{:x}, ", immediate & 0xffff)?;
                        immediate = (immediate >> 16) & 0xff;
                    }

                    JMPF | CALLF => {
                        write!(out, "0x{:x}:", (immediate >> (8 << size)) & 0xffff)?;
                    }

                    _ => (),
                }

                if op_type == OperandType::Off {
                    immediate += inst.address + inst.size() as u64;
                }

                if size == 0 {
                    immediate &= 0xff;
                } else if size == 1 {
                    immediate &= 0xffff;
                } else if size == 2 {
                    immediate &= 0xffffffff;
                }

                write!(out, "{:x}", immediate)?;
            }

            if i == 0 && inst.maskreg().is_some() {
                write!(out, "{{")?;
                strpcatreg(out, RegType::Mask, inst.maskreg().unwrap() as usize, 0)?;
                write!(out, "}}")?;
                if inst.maskzero() {
                    write!(out, "{{z}}")?;
                }
            }
        }

        Ok(())
    }
}

pub fn pretty_disassembler<W: Write>(
    out: &mut W,
    bitness: usize,
    data: &[u8],
    address: u64,
) -> fmt::Result {
    let mut decoder = Decoder::new(bitness as _, data, address);
    let fmt = Formatter::new();

    let mut inst = Instruction::default();
    let start = address;
    while decoder.can_decode() {
        decoder.decode_out(&mut inst);
        let ix = (inst.address - start) as usize;

        let instr_bytes = data[ix..ix + inst.size()]
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<String>>()
            .join(" ");

        let mut outs = String::new();
        fmt.format(&mut outs, &inst)?;
        write!(out, "{:<15.016x} {:<20} {}\n", inst.address, instr_bytes, outs)?;
    }
    Ok(())
}
