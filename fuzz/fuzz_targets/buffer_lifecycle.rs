#![no_main]

use asmkit::Arch;
use asmkit::{CodeBuffer, LabelUse};
use asmkit::Linker;
use asmkit::Environment;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let word = data
        .get(..4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
        .unwrap_or_default();

    let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
    let label = buffer.get_label();
    buffer.write_u32(word);
    let offset = data.get(4).copied().unwrap_or_default() as u32;
    let _ = buffer.try_record_patch_site(offset, LabelUse::X86JmpRel32, 0);
    if data.get(5).copied().unwrap_or_default() & 1 == 0 {
        buffer.bind_label(label);
    } else {
        buffer.use_label_at_offset(0, label, LabelUse::X86JmpRel32);
    }
    let _ = buffer.finish_patched();

    let mut first = CodeBuffer::new(Environment::new(Arch::X64));
    first.write_u32(word);
    let mut second = CodeBuffer::new(Environment::new(Arch::X64));
    second.write_u32(!word);
    let mut linker = Linker::new();
    linker.add_buffer(first.finish().unwrap());
    linker.add_buffer(second.finish().unwrap());
    let _ = linker.link();
});
