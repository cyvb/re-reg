//! ## Raw-Reg
//!
//! This lib contains register utilities and raw pointer utilitis. The raw pointer part
//! is merely a demonstration, but please feel free to use it if you need.
//!
//! Use `use raw_reg::prelude::*` to quick import these register utilities.

#![cfg_attr(not(feature = "std"), no_std)]

mod int;
mod register;

pub mod prelude {
    pub use core::marker::PhantomData;

    pub use crate::registers_layout;
    pub use crate::reg_bitfields;

    pub use crate::int::UIntLike;

    pub use crate::register::bitfield::BitsLike;
    pub use crate::register::bitfield::Bits;
    pub use crate::register::RegName;
    pub use crate::register::{ROInnerRegister, WOInnerRegister, RWInnerRegister};
    pub use crate::register::bitfield::{ReadableIO, WritableIO, ReadWritableIO};
}
