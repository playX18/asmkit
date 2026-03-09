//! MacroAssembler infrastructure
//!
//! This module provides "portable" assembler across all supported architectures.
//!
//! It is a best effort to provide a common API across all architectures, but some features may be architecture specific.

use smallvec::SmallVec;

use crate::{core::operand::Label, x86::CondCode};

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

    pub const fn x86_condition(self) -> CondCode {
        match self {
            Self::Equal => CondCode::E,
            Self::NotEqual => CondCode::NE,
            Self::Above => CondCode::A,
            Self::AboveOrEqual => CondCode::AE,
            Self::Below => CondCode::B,
            Self::BelowOrEqual => CondCode::BE,
            Self::GreaterThan => CondCode::G,
            Self::GreaterThanOrEqual => CondCode::GE,
            Self::LessThan => CondCode::L,
            Self::LessThanOrEqual => CondCode::LE,
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

    pub const fn x86_condition(self) -> CondCode {
        match self {
            Self::Carry => CondCode::C,
            Self::Overflow => CondCode::O,
            Self::Signed => CondCode::S,
            Self::PositiveOrZero => CondCode::NS,
            Self::Zero => CondCode::E,
            Self::NonZero => CondCode::NE,
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
    pub const fn x86_condition(self) -> (bool, bool, CondCode) {
        match self {
            Self::EqualAndOrdered => (true, false, CondCode::E),
            Self::NotEqualAndOrdered => (false, false, CondCode::NE),
            Self::GreaterThanAndOrdered => (false, false, CondCode::A),
            Self::GreaterThanOrEqualAndOrdered => (false, false, CondCode::AE),
            Self::LessThanAndOrdered => (false, true, CondCode::A),
            Self::LessThanOrEqualAndOrdered => (false, true, CondCode::AE),
            Self::EqualOrUnordered => (false, false, CondCode::E),
            Self::NotEqualOrUnordered => (true, false, CondCode::NP),
            Self::GreaterThanOrUnordered => (false, true, CondCode::B),
            Self::GreaterThanOrEqualOrUnordered => (false, true, CondCode::BE),
            Self::LessThanOrUnordered => (false, false, CondCode::B),
            Self::LessThanOrEqualOrUnordered => (false, false, CondCode::BE),
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
