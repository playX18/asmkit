use uasm::encdec::riscv::*;

fn main() {
    let x = C_UIMM8SP_S.iter().fold(0, |op, enc| op | enc.encode(48)) | 0b1100000000100010;
    println!("{:b}", x);
    for i in 0..16 {
        let bit: u32 = (x >> i) & 1;
        println!("Bit {}: {}", i, bit);
    }
}
