/// Type identifier provides a minimalist type system used across AsmJit library.
///
/// This is an additional information that can be used to describe a value-type of physical or virtual register. It's
/// used mostly by BaseCompiler to describe register representation (the group of data stored in the register and the
/// width used) and it's also used by APIs that allow to describe and work with function signatures.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TypeId {
    /// Void type.
    Void = 0,

    /// Abstract signed integer type that has a native size.
    IntPtr = 32,
    /// Abstract unsigned integer type that has a native size.
    UIntPtr = 33,

    /// 8-bit signed integer type.
    Int8 = 34,
    /// 8-bit unsigned integer type.
    UInt8 = 35,
    /// 16-bit signed integer type.
    Int16 = 36,
    /// 16-bit unsigned integer type.
    UInt16 = 37,
    /// 32-bit signed integer type.
    Int32 = 38,
    /// 32-bit unsigned integer type.
    UInt32 = 39,
    /// 64-bit signed integer type.
    Int64 = 40,
    /// 64-bit unsigned integer type.
    UInt64 = 41,

    /// 32-bit floating point type.
    Float32 = 42,
    /// 64-bit floating point type.
    Float64 = 43,
    /// 80-bit floating point type.
    Float80 = 44,

    /// 8-bit opmask register (K).
    Mask8 = 45,
    /// 16-bit opmask register (K).
    Mask16 = 46,
    /// 32-bit opmask register (K).
    Mask32 = 47,
    /// 64-bit opmask register (K).
    Mask64 = 48,

    /// 64-bit MMX register only used for 32 bits.
    Mmx32 = 49,
    /// 64-bit MMX register.
    Mmx64 = 50,

    Int8x4 = 51,
    UInt8x4 = 52,
    Int16x2 = 53,
    UInt16x2 = 54,
    Int32x1 = 55,
    UInt32x1 = 56,
    Float32x1 = 59,

    Int8x8 = 61,
    UInt8x8 = 62,
    Int16x4 = 63,
    UInt16x4 = 64,
    Int32x2 = 65,
    UInt32x2 = 66,
    Int64x1 = 67,
    UInt64x1 = 68,
    Float32x2 = 69,
    Float64x1 = 70,

    Int8x16 = 71,
    UInt8x16 = 72,
    Int16x8 = 73,
    UInt16x8 = 74,
    Int32x4 = 75,
    UInt32x4 = 76,
    Int64x2 = 77,
    UInt64x2 = 78,
    Float32x4 = 79,
    Float64x2 = 80,

    Int8x32 = 81,
    UInt8x32 = 82,
    Int16x16 = 83,
    UInt16x16 = 84,
    Int32x8 = 85,
    UInt32x8 = 86,
    Int64x4 = 87,
    UInt64x4 = 88,
    Float32x8 = 89,
    Float64x4 = 90,

    Int8x64 = 91,
    UInt8x64 = 92,
    Int16x32 = 93,
    UInt16x32 = 94,
    Int32x16 = 95,
    UInt32x16 = 96,
    Int64x8 = 97,
    UInt64x8 = 98,
    Float32x16 = 99,
    Float64x8 = 100,

    MaxValue = 255,
}

impl TypeId {
    pub const VEC128_END: u32 = 80;
    pub const VEC128_START: u32 = 71;

    pub const VEC256_END: u32 = 90;
    pub const VEC256_START: u32 = 81;

    pub const VEC512_END: u32 = 100;
    pub const VEC512_START: u32 = 91;

    pub const VEC32_START: u32 = 51;
    pub const VEC32_END: u32 = 60;
    pub const VEC64_START: u32 = 61;
    pub const VEC64_END: u32 = 70;

    pub const MMX_START: u32 = 49;
    pub const MMX_END: u32 = 50;
    pub const MASK_START: u32 = 45;
    pub const MASK_END: u32 = 48;

    pub const FLOAT_START: u32 = 42;
    pub const FLOAT_END: u32 = 44;

    pub const INT_START: u32 = 32;
    pub const INT_END: u32 = 41;

    pub fn size(&self) -> usize {
        match self {
            Self::UIntPtr | Self::IntPtr => size_of::<usize>(),
            Self::Int8 | Self::UInt8 => 1,
            Self::Int16 | Self::UInt16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
            Self::Int64 | Self::UInt64 | Self::Float64 => 8,
            Self::Float80 => 10,
            Self::Mask8 => 1,
            Self::Mask16 => 2,
            Self::Mask32 => 4,
            Self::Mask64 => 8,
            Self::Mmx32 | Self::Mmx64 => 8,
            Self::Int8x4
            | Self::UInt8x4
            | Self::Int16x2
            | Self::UInt16x2
            | Self::Int32x1
            | Self::UInt32x1
            | Self::Float32x1 => 4,
            Self::Int8x8
            | Self::UInt8x8
            | Self::Int16x4
            | Self::UInt16x4
            | Self::Int32x2
            | Self::UInt32x2
            | Self::Int64x1
            | Self::UInt64x1
            | Self::Float32x2
            | Self::Float64x1 => 8,
            Self::Int8x16
            | Self::UInt8x16
            | Self::Int16x8
            | Self::UInt16x8
            | Self::Int32x4
            | Self::UInt32x4
            | Self::Int64x2
            | Self::UInt64x2
            | Self::Float32x4
            | Self::Float64x2 => 16,
            Self::Int8x32
            | Self::UInt8x32
            | Self::Int16x16
            | Self::UInt16x16
            | Self::Int32x8
            | Self::UInt32x8
            | Self::Int64x4
            | Self::UInt64x4
            | Self::Float32x8
            | Self::Float64x4 => 32,
            Self::Int8x64
            | Self::UInt8x64
            | Self::Int16x32
            | Self::UInt16x32
            | Self::Int32x16
            | Self::UInt32x16
            | Self::Int64x8
            | Self::UInt64x8
            | Self::Float32x16
            | Self::Float64x8 => 64,
            _ => 0,
        }
    }

    pub fn deabstract(&self, register_size: usize) -> Self {
        match self {
            Self::IntPtr => {
                if register_size == 8 {
                    Self::Int64
                } else {
                    Self::Int32
                }
            }
            Self::UIntPtr => {
                if register_size == 8 {
                    Self::UInt64
                } else {
                    Self::UInt32
                }
            }
            _ => *self,
        }
    }
}
