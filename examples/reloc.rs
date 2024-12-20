use asmkit::core::buffer::{perform_relocations, CodeBuffer, ExternalName, RelocDistance};
use asmkit::core::jit_allocator::JitAllocator;

extern "C" {
    fn puts(_: *const i8);
}

fn main() {
    {
        use asmkit::x86::*;
        use formatter::pretty_disassembler;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let str_constant = asm.add_constant("Hello, World!\0");
        let puts_sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol("puts".into()), RelocDistance::Far);

        asm.lea64rm(RDI, ptr64_label(str_constant, 0));
        asm.callm(ptr64_sym(puts_sym, 0));
        asm.ret();

        let result = buf.finish();

        for reloc in result.relocs() {
            println!("{:?}", reloc);
        }

        let mut jit = JitAllocator::new(Default::default());

        // allocate memory for GOT table and for code itself
        let mut span = jit
            .alloc(result.data().len() + result.relocs().len() * 8)
            .unwrap();

        let mut got_addr_rx = std::ptr::null();

        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
                got_addr_rx = span.rx().add(result.data().len());
                span.rw()
                    .add(result.data().len())
                    .cast::<usize>()
                    .write(puts as *const u8 as usize);
                // we only link to one symbol in GOT table, don't bother with anything else...
                perform_relocations(
                    span.rw(),
                    span.rx(),
                    &result.relocs(),
                    |_| unreachable!(),
                    |_| got_addr_rx,
                    |_| unreachable!(),
                );
            })
            .unwrap();

            let mut out = String::new();
            pretty_disassembler(
                &mut out,
                64,
                std::slice::from_raw_parts(span.rx(), result.data().len()),
                span.rx() as _,
            )
            .unwrap();

            println!("{}", out);
            #[cfg(target_arch = "x86_64")]
            {
                let f: extern "C" fn() = std::mem::transmute(span.rx());

                f();
            }
        }
    }

    {
        use asmkit::riscv::*;
        use formatter::pretty_disassembler;

        let mut buf = CodeBuffer::new();
        buf.env_mut().set_pic(false);
        let mut asm = Assembler::new(&mut buf);

        let str_constant = asm.add_constant("Hello, World!\0");
        let puts_sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol("puts".into()), RelocDistance::Far);
        asm.addi(SP, SP, imm(-16));
        asm.sd(SP, RA, imm(8));
        asm.sd(SP, S0, imm(0));
        asm.la(A0, str_constant);
        asm.la(A1, puts_sym);
        asm.call(A1);
        asm.ld(RA, SP, imm(8));
        asm.ld(S0, SP, imm(0));
        asm.addi(SP, SP, imm(16));
        asm.ret();

        let result = buf.finish();

        for reloc in result.relocs() {
            println!("{:?}", reloc);
        }

        let mut jit = JitAllocator::new(Default::default());

        // allocate memory for GOT table and for code itself
        let mut span = jit
            .alloc(result.data().len() + result.relocs().len() * 8)
            .unwrap();

        let mut got_addr_rx = std::ptr::null();

        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
                got_addr_rx = span.rx().add(result.data().len());
                span.rw()
                    .add(result.data().len())
                    .cast::<usize>()
                    .write(puts as *const u8 as usize);
                // we only link to one symbol in GOT table, don't bother with anything else...
                perform_relocations(
                    span.rw(),
                    span.rx(),
                    &result.relocs(),
                    |_| puts as *const u8,
                    |_| got_addr_rx,
                    |_| unreachable!(),
                );
            })
            .unwrap();

            let mut out = String::new();
            pretty_disassembler(
                &mut out,
                64,
                std::slice::from_raw_parts(span.rx(), result.data().len()),
                span.rx() as _,
            )
            .unwrap();

            println!("{}", out);


            #[cfg(target_arch = "riscv64")]
            {
                let f: extern "C" fn()= std::mem::transmute(span.rx());

                f();
            }
        }
    }
}
