use alloc::borrow::Cow;

use crate::AsmError;

use super::buffer::{CodeBuffer, CodeBufferFinalized};
use super::target::Environment;

/// A named section: its own code buffer plus an alignment requirement.
///
/// Sections are emitted independently (each gets its own [`CodeBuffer`], so
/// labels and fixups stay section-local) and are laid out: concatenated with
/// alignment: at link time by [`Linker`](crate::core::linker::Linker). A
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
