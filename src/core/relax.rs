//! x86-64 branch relaxation.

use alloc::vec::Vec;

use super::buffer::{CodeOffset, LabelUse};
use super::operand::Label;

/// A `jmp` or `jcc` whose size relaxation may change.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RelaxableJump {
    /// Offset of the opcode.
    pub start: CodeOffset,
    /// Bytes emitted: 2 for rel8, else the rel32 length.
    pub len: u8,
    pub label: Label,
    /// The `jcc` condition (the low nibble of the opcode), or `None` for `jmp`.
    pub cc: Option<u8>,
}

impl RelaxableJump {
    pub const SHORT_LEN: u32 = 2;

    /// Length of the rel32 form: `E9 rel32` or `0F 8x rel32`.
    pub const fn long_len(&self) -> u32 {
        if self.cc.is_some() { 6 } else { 5 }
    }

    /// Opcode bytes of the rel8 form.
    pub fn short_opcode(&self) -> [u8; 1] {
        match self.cc {
            Some(cc) => [0x70 | cc],
            None => [0xEB],
        }
    }

    /// Opcode bytes of the rel32 form.
    pub fn long_opcode(&self) -> &'static [u8] {
        const JCC: [[u8; 2]; 16] = {
            let mut t = [[0x0F, 0x80]; 16];
            let mut cc = 0;
            while cc < 16 {
                t[cc][1] = 0x80 | cc as u8;
                cc += 1;
            }
            t
        };
        match self.cc {
            Some(cc) => &JCC[cc as usize],
            None => &[0xE9],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum LayoutEvent {
    Jump(RelaxableJump),
    /// Code-alignment padding emitted at `start`.
    Align {
        start: CodeOffset,
        len: CodeOffset,
        align: CodeOffset,
    },
}

impl LayoutEvent {
    pub fn start(&self) -> CodeOffset {
        match self {
            Self::Jump(j) => j.start,
            Self::Align { start, .. } => *start,
        }
    }

    pub fn len(&self) -> CodeOffset {
        match self {
            Self::Jump(j) => CodeOffset::from(j.len),
            Self::Align { len, .. } => *len,
        }
    }
}

/// A label-relative field outside a [`RelaxableJump`]: re-encoded after
/// relaxation moves the code.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LabelRef {
    pub offset: CodeOffset,
    pub label: Label,
    pub kind: LabelUse,
    /// Encoded value is `label + addend - offset - field size`.
    pub addend: i32,
}

/// Where one [`LayoutEvent`] was emitted and where it ended up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Segment {
    pub old_start: CodeOffset,
    pub old_end: CodeOffset,
    pub new_start: CodeOffset,
    pub new_end: CodeOffset,
}

/// Maps offsets from emission time to the relaxed layout. Empty when
/// nothing moved.
#[derive(Clone, Default, Debug)]
pub(crate) struct OffsetMap {
    segments: Vec<Segment>,
}

impl OffsetMap {
    /// Lays out `events` (in emission order) with jump `events[i]` rel32
    /// when `long[i]` holds and rel8 otherwise. `long` has one entry per
    /// event; entries for padding are ignored.
    pub fn layout(events: &[LayoutEvent], long: &[bool]) -> Self {
        let mut segments = Vec::with_capacity(events.len());
        let mut shift: i64 = 0;
        for (event, &long) in events.iter().zip(long) {
            let old_start = event.start();
            let old_end = old_start + event.len();
            let new_start = (i64::from(old_start) + shift) as CodeOffset;
            let new_len = match event {
                LayoutEvent::Jump(j) if long => j.long_len(),
                LayoutEvent::Jump(_) => RelaxableJump::SHORT_LEN,
                LayoutEvent::Align { align, .. } => new_start.wrapping_neg() & (align - 1),
            };
            shift += i64::from(new_len) - i64::from(event.len());
            segments.push(Segment {
                old_start,
                old_end,
                new_start,
                new_end: new_start + new_len,
            });
        }
        Self { segments }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Moves an emission-time offset into the relaxed layout.
    pub fn map(&self, offset: CodeOffset) -> CodeOffset {
        if offset == u32::MAX {
            return offset;
        }
        let i = self.segments.partition_point(|s| s.old_start <= offset);
        let Some(s) = i.checked_sub(1).map(|i| &self.segments[i]) else {
            return offset;
        };
        if offset >= s.old_end {
            offset - s.old_end + s.new_end
        } else {
            offset - s.old_start + s.new_start
        }
    }
}
