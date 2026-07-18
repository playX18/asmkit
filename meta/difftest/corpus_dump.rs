//! Corpus inspector for the x86 differential test.
//!
//! Reads a corpus file (see meta/difftest/FORMAT.md), prints records in
//! human-readable form, and disassembles each record's expected bytes with
//! iced-x86. Used to spot-check that recorded operands match recorded bytes.
//!
//! Usage: corpus_dump <corpus.bin> [--head N] [--skip N] [--stats]

use iced_x86::Formatter;
use std::io::Read;

const RT: &[&str] = &[
    "none", "label", "sym", "rip", "gp8lo", "gp8hi", "gp16", "gp32", "gp64", "vec8", "vec16",
    "vec32", "vec64", "xmm", "ymm", "zmm", "vecn", "k", "mm", "sreg", "creg", "dreg", "st", "bnd",
    "tmm",
];

fn reg_name(rt: u8, id: u8) -> String {
    format!("{}{}", RT.get(rt as usize).unwrap_or(&"?"), id)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .expect("usage: corpus_dump <corpus.bin> [--head N] [--skip N] [--stats]");
    let mut head = 10usize;
    let mut skip = 0usize;
    let mut stats_only = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--head" => head = args.next().unwrap().parse().unwrap(),
            "--skip" => skip = args.next().unwrap().parse().unwrap(),
            "--stats" => stats_only = true,
            other => panic!("unknown arg {other}"),
        }
    }
    let mut data = Vec::new();
    std::fs::File::open(&path)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();
    assert_eq!(&data[..8], b"AKDFCOR1", "bad magic");
    let mut pos = 16usize; // magic + flags + reserved

    let mut n_records = 0u64;
    let mut total_bytes = 0u64;
    let mut mnems = std::collections::HashSet::new();
    let mut shown = 0usize;
    let mut fmt = iced_x86::IntelFormatter::new();

    while pos < data.len() {
        let plen = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        let p = &data[pos + 4..pos + 4 + plen];
        pos += 4 + plen;
        let mut c = 0usize;
        let mlen = u16::from_le_bytes(p[c..c + 2].try_into().unwrap()) as usize;
        c += 2;
        let mnem = std::str::from_utf8(&p[c..c + mlen]).unwrap().to_string();
        c += mlen;
        let opcode = i64::from_le_bytes(p[c..c + 8].try_into().unwrap());
        c += 8;
        let nops = p[c] as usize;
        let opts0 = p[c + 1];
        let opts1 = p[c + 2];
        let k_id = p[c + 3];
        let label_layout = p[c + 4];
        c += 5;
        let label_bind = i32::from_le_bytes(p[c..c + 4].try_into().unwrap());
        c += 4;
        let instr_off = u32::from_le_bytes(p[c..c + 4].try_into().unwrap());
        c += 4;
        let instr_len = u32::from_le_bytes(p[c..c + 4].try_into().unwrap());
        c += 4;

        let mut ops_desc = Vec::new();
        for _ in 0..nops {
            let kind = p[c];
            c += 1;
            match kind {
                1 => {
                    ops_desc.push(reg_name(p[c], p[c + 1]));
                    c += 2;
                }
                2 => {
                    let (bt, bi, it, ii, sc, sz, at, bc) = (
                        p[c],
                        p[c + 1],
                        p[c + 2],
                        p[c + 3],
                        p[c + 4],
                        p[c + 5],
                        p[c + 6],
                        p[c + 7],
                    );
                    let disp = i64::from_le_bytes(p[c + 8..c + 16].try_into().unwrap());
                    c += 16;
                    let mut d = String::from("[");
                    if bt != 0 {
                        d += &reg_name(bt, bi);
                    }
                    if it != 0 {
                        d += &format!("+{}*{}", reg_name(it, ii), 1u32 << sc);
                    }
                    d += &format!(";disp={disp};sz={sz};at={at};bc={bc}]");
                    ops_desc.push(d);
                }
                3 => {
                    let v = i64::from_le_bytes(p[c..c + 8].try_into().unwrap());
                    c += 8;
                    ops_desc.push(format!("imm:{v}"));
                }
                4 => ops_desc.push("label".to_string()),
                other => panic!("bad op kind {other}"),
            }
        }
        let blen = u32::from_le_bytes(p[c..c + 4].try_into().unwrap()) as usize;
        c += 4;
        let block = &p[c..c + blen];

        n_records += 1;
        total_bytes += blen as u64;
        mnems.insert(mnem.clone());

        if stats_only {
            continue;
        }
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if shown >= head {
            continue;
        }
        shown += 1;

        let seg = opts1 & 7;
        let er = (opts1 >> 3) & 7;
        let mut flags = String::new();
        if opts0 & 1 != 0 {
            flags += " lock";
        }
        if opts0 & 2 != 0 {
            flags += " rep";
        }
        if opts0 & 4 != 0 {
            flags += " repnz";
        }
        if seg != 0 {
            flags += &format!(" seg={seg}");
        }
        if er != 0 {
            flags += &format!(" er={er}");
        }
        if opts1 & 0x40 != 0 {
            flags += &format!(" k={k_id}");
        }
        if label_layout != 0xFF {
            flags += &format!(" label(lay={label_layout},bind={label_bind})");
        }

        let start = instr_off as usize;
        let bytes = &block[start..start + instr_len as usize];
        let hex: Vec<String> = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let mut dec =
            iced_x86::Decoder::with_ip(64, bytes, instr_off as u64, iced_x86::DecoderOptions::NONE);
        let instr = dec.decode();
        let mut text = String::new();
        fmt.format(&instr, &mut text);

        println!(
            "#{:<6} {:24} {:40}{} \n         opc={opcode:#018x} bytes={}\n         iced: {}",
            n_records - 1,
            mnem,
            ops_desc.join(", "),
            flags,
            hex.join(" "),
            text,
        );
    }

    println!("---");
    println!("records: {n_records}");
    println!("distinct mnemonics: {}", mnems.len());
    println!("total instruction bytes: {total_bytes}");
    println!("file size: {}", data.len());
    if stats_only {
        let mut list: Vec<_> = mnems.into_iter().collect();
        list.sort();
        let out = format!("{path}.mnems.txt");
        std::fs::write(&out, list.join("\n") + "\n").unwrap();
        println!("mnemonic list written to {out}");
    }
}
