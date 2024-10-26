pub mod assembler;
pub use assembler::*;

use crate::encdec::riscv::InstructionValue;

pub const fn is_imm12(val: i64) -> bool {
    val >= -2048 && val <= 2047
}

pub(crate) fn generate_imm(value: u64) -> (u32, u32) {
    if is_imm12(value as _) {
        return (
            0,
            InstructionValue::new(0)
                .set_imm12(value as i64 as i32)
                .value,
        );
    }

    let value = value as i64;

    let mod_num = 4096i64;
    let (imm20, imm12) = if value > 0 {
        let mut imm20 = value / mod_num;
        let mut imm12 = value % mod_num;

        if imm12 >= 2048 {
            imm12 -= mod_num;
            imm20 += 1;
        }

        (imm20, imm12)
    } else {
        let value_abs = value.abs();
        let imm20 = value_abs / mod_num;
        let imm12 = value_abs % mod_num;
        let mut imm20 = -imm20;
        let mut imm12 = -imm12;
        if imm12 < -2048 {
            imm12 += mod_num;
            imm20 -= 1;
        }
        (imm20, imm12)
    };
    (
        InstructionValue::new(0).set_imm20(imm20 as _).value,
        InstructionValue::new(0).set_imm12(imm12 as _).value,
    )
}
