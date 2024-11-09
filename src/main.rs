use capstone::prelude::*;
use uasm::core::jit_allocator::{JitAllocator, JitAllocatorOptions};
use uasm::aarch64::opcodes::*;
fn main() {
    println!("{:?}", INST_INFO[Opcode::ADDG as usize].encoding);
}
