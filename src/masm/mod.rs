//! MacroAssembler infrastructure
//!
//! This module provides "portable" assembler across all supported architectures.
//!
//! It is a best effort to provide a common API across all architectures, but some features may be architecture specific.

use smallvec::SmallVec;

use crate::core::operand::Label;

#[cfg(feature = "x86")]
use crate::x86::CondCode as X86CondCode;

//pub mod x86;

pub enum ExtensionSize {
    I8ToI32,
    I8ToI64,
    I16ToI32,
    I16ToI64,
    I32ToI64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelationalCondition {
    Equal,
    NotEqual,
    Above,
    AboveOrEqual,
    Below,
    BelowOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultCondition {
    Carry,
    Overflow,
    Signed,
    PositiveOrZero,
    Zero,
    NonZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DoubleCondition {
    EqualAndOrdered,
    NotEqualAndOrdered,
    GreaterThanAndOrdered,
    GreaterThanOrEqualAndOrdered,
    LessThanAndOrdered,
    LessThanOrEqualAndOrdered,
    EqualOrUnordered,
    NotEqualOrUnordered,
    GreaterThanOrUnordered,
    GreaterThanOrEqualOrUnordered,
    LessThanOrUnordered,
    LessThanOrEqualOrUnordered,
}

impl RelationalCondition {
    pub const fn invert(self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::Above => Self::BelowOrEqual,
            Self::AboveOrEqual => Self::Below,
            Self::Below => Self::AboveOrEqual,
            Self::BelowOrEqual => Self::Above,
            Self::GreaterThan => Self::LessThanOrEqual,
            Self::GreaterThanOrEqual => Self::LessThan,
            Self::LessThan => Self::GreaterThanOrEqual,
            Self::LessThanOrEqual => Self::GreaterThan,
        }
    }

    #[cfg(feature = "x86")]
    pub const fn x86_condition(self) -> X86CondCode {
        match self {
            Self::Equal => X86CondCode::E,
            Self::NotEqual => X86CondCode::NE,
            Self::Above => X86CondCode::A,
            Self::AboveOrEqual => X86CondCode::AE,
            Self::Below => X86CondCode::B,
            Self::BelowOrEqual => X86CondCode::BE,
            Self::GreaterThan => X86CondCode::G,
            Self::GreaterThanOrEqual => X86CondCode::GE,
            Self::LessThan => X86CondCode::L,
            Self::LessThanOrEqual => X86CondCode::LE,
        }
    }
}

impl ResultCondition {
    pub const fn invert(self) -> Self {
        match self {
            Self::Carry => Self::PositiveOrZero,
            Self::Overflow => Self::NonZero,
            Self::Signed => Self::PositiveOrZero,
            Self::PositiveOrZero => Self::Carry,
            Self::Zero => Self::NonZero,
            Self::NonZero => Self::Zero,
        }
    }

    #[cfg(feature = "x86")]
    pub const fn x86_condition(self) -> X86CondCode {
        match self {
            Self::Carry => X86CondCode::C,
            Self::Overflow => X86CondCode::O,
            Self::Signed => X86CondCode::S,
            Self::PositiveOrZero => X86CondCode::NS,
            Self::Zero => X86CondCode::E,
            Self::NonZero => X86CondCode::NE,
        }
    }
}

impl DoubleCondition {
    pub const fn invert(self) -> Self {
        match self {
            Self::EqualAndOrdered => Self::NotEqualOrUnordered,
            Self::NotEqualAndOrdered => Self::EqualOrUnordered,
            Self::GreaterThanAndOrdered => Self::LessThanOrEqualOrUnordered,
            Self::GreaterThanOrEqualAndOrdered => Self::LessThanOrUnordered,
            Self::LessThanAndOrdered => Self::GreaterThanOrEqualOrUnordered,
            Self::LessThanOrEqualAndOrdered => Self::GreaterThanOrUnordered,
            Self::EqualOrUnordered => Self::NotEqualAndOrdered,
            Self::NotEqualOrUnordered => Self::EqualAndOrdered,
            Self::GreaterThanOrUnordered => Self::LessThanOrEqualAndOrdered,
            Self::GreaterThanOrEqualOrUnordered => Self::LessThanAndOrdered,
            Self::LessThanOrUnordered => Self::GreaterThanOrEqualAndOrdered,
            Self::LessThanOrEqualOrUnordered => Self::GreaterThanAndOrdered,
        }
    }

    /// Convert to x86 condition code, returning a tuple of (special, invert, condition_code)
    #[cfg(feature = "x86")]
    pub const fn x86_condition(self) -> (bool, bool, X86CondCode) {
        match self {
            Self::EqualAndOrdered => (true, false, X86CondCode::E),
            Self::NotEqualAndOrdered => (false, false, X86CondCode::NE),
            Self::GreaterThanAndOrdered => (false, false, X86CondCode::A),
            Self::GreaterThanOrEqualAndOrdered => (false, false, X86CondCode::AE),
            Self::LessThanAndOrdered => (false, true, X86CondCode::A),
            Self::LessThanOrEqualAndOrdered => (false, true, X86CondCode::AE),
            Self::EqualOrUnordered => (false, false, X86CondCode::E),
            Self::NotEqualOrUnordered => (true, false, X86CondCode::NP),
            Self::GreaterThanOrUnordered => (false, true, X86CondCode::B),
            Self::GreaterThanOrEqualOrUnordered => (false, true, X86CondCode::BE),
            Self::LessThanOrUnordered => (false, false, X86CondCode::B),
            Self::LessThanOrEqualOrUnordered => (false, false, X86CondCode::BE),
        }
    }
}

pub const fn imm_fits<const BITS: usize>(imm: i64) -> bool {
    if BITS >= 64 {
        return true;
    }
    let min = -(1i64 << (BITS - 1));
    let max = (1i64 << (BITS - 1)) - 1;
    imm >= min && imm <= max
}

pub struct JumpList {
    pub labels: SmallVec<[Label; 4]>,
}

impl Default for JumpList {
    fn default() -> Self {
        Self::new()
    }
}

impl JumpList {
    pub fn new() -> Self {
        Self {
            labels: SmallVec::new(),
        }
    }

    pub fn push(&mut self, label: Label) {
        self.labels.push(label);
    }
}
