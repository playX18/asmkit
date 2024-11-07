use super::*;
pub struct Decoder<'a> {
    buf: &'a [u8],
    cursor: usize,
    address: u64,
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Decode32,
    Decode64,
}

impl Mode {
    pub fn is_64(self) -> bool {
        self == Self::Decode64
    }
}

impl TryFrom<usize> for Mode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            32 => Ok(Self::Decode32),
            64 => Ok(Self::Decode64),
            _ => Err(()),
        }
    }
}
pub struct Instruction {
    pub code: Opcode,
    pub value: InstructionValue,
    pub len: usize,
    pub address: u64,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            len: 0,
            code: Opcode::Invalid,
            value: InstructionValue::new(0),
            address: 0,
        }
    }
}

/// Return the length (in bytes) of an instruction given the low 16 bits of it.
///
/// The current spec reserves a bit pattern for instructions of length >= 192 bits, but for
/// simplicity this function just returns 24 in that case. The largest instructions currently
/// defined are 4 bytes so it will likely be a long time until this diffence matters.
fn instruction_length(i: u16) -> usize {
    if i & 0b11 != 0b11 {
        2
    } else if i & 0b11100 != 0b11100 {
        4
    } else if i & 0b111111 == 0b011111 {
        6
    } else if i & 0b1111111 == 0b011111 {
        8
    } else {
        10 + 2 * ((i >> 12) & 0b111) as usize
    }
}

impl<'a> Decoder<'a> {
    pub fn new(bitness: usize, buf: &'a [u8], address: u64) -> Self {
        Self {
            buf,
            cursor: 0,
            address,
            mode: Mode::try_from(bitness).expect("only 32 and 64 bit bitness supported"),
        }
    }

    pub fn can_decode(&self) -> bool {
        self.cursor < self.buf.len()
    }

    fn read_u8(&mut self) -> u8 {
        let val = self.buf[self.cursor];
        self.cursor += 1;
        val
    }

    pub fn decode_out(&mut self, inst: &mut Instruction) {
        if self.cursor >= self.buf.len() {
            return;
        }

        let start = self.cursor;

        let b0 = self.read_u8();
        let b1 = self.read_u8();

        let short = u16::from_le_bytes([b0, b1]);
        let len = instruction_length(short);
        let mut val = InstructionValue::new(short as _);
        let masks = if self.mode.is_64() {
            &OPCODE64_MASK
        } else {
            &OPCODE32_MASK
        };

        let matches = if self.mode.is_64() {
            &OPCODE64_MATCH
        } else {
            &OPCODE32_MATCH
        };
        let mut opc = None;

        if len == 2 {
            let mut zero_mask = None;

            for &op in SHORT_OPCODES.iter() {
                let mask = masks[op as usize];
                let match_ = matches[op as usize];
                if short as u32 & mask == match_ {
                    if mask == 0 {
                        zero_mask = Some(op);
                        continue;
                    }

                    opc = Some(op);
                    break;
                }
            }

            if opc.is_none() {
                opc = zero_mask;
            }
        } else if len == 4 {
            let b2 = self.read_u8();
            let b3 = self.read_u8();
            let long = u32::from_le_bytes([b0, b1, b2, b3]);
            for &op in ALL_OPCODES.iter() {
                let mask = masks[op as usize];
                let match_ = matches[op as usize];
                if long & mask == match_ {
                    opc = Some(op);
                    val = InstructionValue::new(long);
                    break;
                }
            }
        } else {
            self.cursor = start + len;
            return;
        }

        let opc = opc.unwrap_or(Opcode::Invalid);
        inst.code = opc;
        inst.value = val;
        inst.len = len;
        inst.address = self.address + start as u64;
    }
}
