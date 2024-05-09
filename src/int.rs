//! ## crate::int
//!
//! This mod contains `UIntLike` trait to group available integer types for a register.
//! It also contains two useful method `zero()`, which returns 0, and `full()`, which
//! returns the maximun number of given type.
//!
//! This crate currently supports 8-bit, 16-bit, 32-bit and 64-bit registers.

use core::ops::{
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Not,
    Shl,
    Shr
};

pub trait UIntLike :
    BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Eq
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + Copy
    + Clone
{
    fn zero() -> Self;
    fn all() -> Self;
}

macro_rules! impl_uintlike_zero {
    ($typ:ty) => {
        impl UIntLike for $typ {
            #[inline]
            fn zero() -> Self {
                0 as $typ
            }
            #[inline]
            fn all() -> Self {
                (0. as $typ).wrapping_sub(1)
            }
        }
    };
}

impl_uintlike_zero!(u8);
impl_uintlike_zero!(u16);
impl_uintlike_zero!(u32);
impl_uintlike_zero!(u64);
