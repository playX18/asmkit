use core::mem::size_of;

pub fn align_down(x: usize, alignment: usize) -> usize {
    x & !(alignment - 1)
}

pub fn align_up(x: usize, alignment: usize) -> usize {
    x.wrapping_add(alignment.wrapping_sub(1)) & !(alignment - 1)
}

pub fn bit_vector_get_bit(buf: &[u32], index: usize) -> bool {
    let vec_index = index / 32;
    let bit_index = index % 32;

    ((buf[vec_index] >> bit_index) & 1) != 0
}

pub fn bit_vector_set_bit(buf: &mut [u32], index: usize, value: bool) {
    let vec_index = index / 32;
    let bit_index = index % 32;

    if value {
        buf[vec_index] |= 1 << bit_index;
    } else {
        buf[vec_index] &= !(1 << bit_index);
    }
}

pub fn bit_vector_flip_bit(buf: &mut [u32], index: usize) {
    let vec_index = index / 32;
    let bit_index = index % 32;

    buf[vec_index] ^= 1 << bit_index;
}

macro_rules! bitvector_op {
    ($name: ident, $op: expr, $opf: expr) => {
        pub fn $name(mut buf: &mut [u32], index: usize, mut count: usize) {
            if count == 0 {
                return;
            }

            let vec_index = index / 32;
            let bit_index = index % 32;

            buf = &mut buf[vec_index..];

            const FILL_MASK: u32 = u32::MAX;

            let first_n_bits = (32usize.wrapping_sub(bit_index)).min(count);

            buf[0] = $op(
                buf[0],
                (FILL_MASK >> (32 - first_n_bits as u32)) << bit_index as u32,
            );

            buf = &mut buf[1..];

            count -= first_n_bits;

            while count >= 32 {
                buf[0] = $opf(buf[0], FILL_MASK);
                buf = &mut buf[1..];
                count -= 32;
            }

            if count != 0 {
                buf[0] = $op(buf[0], FILL_MASK >> (32 - count as u32));
            }
        }
    };
}

bitvector_op!(bit_vector_fill, |x, y| x | y, |_, x| x);
bitvector_op!(bit_vector_clear, |x: u32, y: u32| x & !y, |_, y: u32| !y);

pub fn bit_vector_index_of(buf: &[u32], start: usize, value: bool) -> usize {
    let vec_index = start / 32;
    let bit_index = start % 32;

    let mut p = &buf[vec_index..];
    const FILL_MASK: u32 = u32::MAX;

    let flip_mask = if value { 0 } else { FILL_MASK };

    let mut bits = (p[0] ^ flip_mask) & (FILL_MASK << bit_index as u32);

    loop {
        if bits != 0 {
            return ((p.as_ptr() as usize - buf.as_ptr() as usize) / size_of::<u32>()) * 32
                + bits.trailing_zeros() as usize;
        }

        p = &p[1..];

        if p.is_empty() {
            return buf.len() * 32;
        }

        bits = p[0] ^ flip_mask;
    }
}

pub mod os;
pub mod virtual_memory;
