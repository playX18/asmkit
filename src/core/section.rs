use alloc::borrow::Cow;

use crate::AsmError;

use super::buffer::{CodeBuffer, CodeBufferFinalized};
use super::target::Environment;

/// A named section: its own code buffer plus an alignment requirement.
///
/// Sections are emitted independently (each gets its own [`CodeBuffer`], so
/// labels and fixups stay section-local) and are laid out — concatenated with
/// alignment — at link time by [`Linker`](crate::core::linker::Linker). A
/// section name is diagnostic only: the in-memory linker creates one flat
/// image and does not model per-section read/write/execute permissions.
pub struct Section {
    name: Cow<'static, str>,
    align: u32,
    buffer: CodeBuffer,
}

impl Section {
    /// Creates a section with the given name (conventionally `.text`, `.data`,
    /// `.rodata`, ...) and alignment for the host target.
    ///
    /// Cross-target users should use [`Self::with_env`].
    pub fn new(name: impl Into<Cow<'static, str>>, align: u32) -> Result<Self, AsmError> {
        Self::with_env(name, align, Environment::host())
    }

    /// Creates a section for an explicit target environment.
    pub fn with_env(
        name: impl Into<Cow<'static, str>>,
        align: u32,
        environment: Environment,
    ) -> Result<Self, AsmError> {
        if !align.is_power_of_two() {
            return Err(AsmError::InvalidArgument);
        }
        Ok(Self {
            name: name.into(),
            align,
            buffer: CodeBuffer::new(environment),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn align(&self) -> u32 {
        self.align
    }

    pub fn buffer(&self) -> &CodeBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut CodeBuffer {
        &mut self.buffer
    }

    /// Finalizes the section's buffer, making it ready for linking.
    pub fn finish(mut self) -> Result<FinalizedSection, AsmError> {
        Ok(FinalizedSection {
            name: self.name,
            align: self.align,
            code: self.buffer.finish()?,
        })
    }
}

/// A section whose buffer has been finalized, ready for linking.
pub struct FinalizedSection {
    pub(crate) name: Cow<'static, str>,
    pub(crate) align: u32,
    pub(crate) code: CodeBufferFinalized,
}

impl FinalizedSection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn align(&self) -> u32 {
        self.align
    }

    pub fn code(&self) -> &CodeBufferFinalized {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::arch_traits::Arch;

    #[test]
    fn section_holds_named_buffer_with_alignment() {
        let mut section = Section::new(".rodata", 16).unwrap();
        assert_eq!(section.name(), ".rodata");
        assert_eq!(section.align(), 16);

        section.buffer_mut().write_u64(0x1122_3344_5566_7788);

        let finalized = section.finish().unwrap();
        assert_eq!(finalized.name(), ".rodata");
        assert_eq!(finalized.align(), 16);
        assert_eq!(
            finalized.code().data(),
            &0x1122_3344_5566_7788u64.to_ne_bytes()
        );
    }

    #[test]
    fn section_alignment_must_be_power_of_two() {
        assert_eq!(
            Section::new(".text", 3).err(),
            Some(AsmError::InvalidArgument)
        );
    }

    #[test]
    fn section_preserves_explicit_target() {
        let section = Section::with_env(".text", 4, Environment::new(Arch::AArch64)).unwrap();

        assert_eq!(section.buffer().env().arch(), Arch::AArch64);
    }
}
