//! ## crate::register
//!
//! This mod contains built-in register utilities. There are Read-Only, Write-Only and
//! Read-Write `InnerRegister` structs.
//!
//! Users should not use these structs directly. Please use `registers_layout!{}`
//! to generate the layout.

pub mod bitfield;
pub mod macros;

use core::ptr;
use core::cell::UnsafeCell;
use core::marker::PhantomData;

use crate::int::UIntLike;

use bitfield::{BitsLike, ReadableIO, WritableIO};

pub trait RegName {}

impl RegName for () {}

/// ## Read-Only register
#[repr(transparent)]
pub struct ROInnerRegister<T, R = ()>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    raw: T,
    _reg: PhantomData<R>
}

impl<T, R> ReadableIO<T, R> for ROInnerRegister<T, R>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    #[inline]
    fn read(&self) -> T {
        unsafe {
            ptr::read_volatile(&self.raw)
        }
    }
}

/// ## Write-Only register
#[repr(transparent)]
pub struct WOInnerRegister<T: UIntLike, R = ()>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    raw: UnsafeCell<T>,
    _reg: PhantomData<R>
}

impl<T, R> WritableIO<T, R> for WOInnerRegister<T, R>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    #[inline]
    fn write(&self, val: T) {
        unsafe {
            self.raw.get().write_volatile(val);
        }
    }
}

/// ## Read-Write register
///
/// Although `ReadWritableIO` implements `WritableIO`, we don't use `WritableIO`'s
/// detault `put()` method. If an IO is both readable and writable, we always retain
/// other bits' value when we manipulate some bits. Therefore we re-write this method
/// here to keep things this way.
#[repr(transparent)]
pub struct RWInnerRegister<T, R = ()>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    raw: UnsafeCell<T>,
    _reg: PhantomData<R>
}

impl<T, R> ReadableIO<T, R> for RWInnerRegister<T, R>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    #[inline]
    fn read(&self) -> T {
        unsafe {
            self.raw.get().read()
        }
    }
}

impl<T, R> WritableIO<T, R> for RWInnerRegister<T, R>
where
    T: UIntLike,
    R: RegName + BitsLike<T>
{
    #[inline]
    fn write(&self, val: T) {
        unsafe {
            self.raw.get().write_volatile(val);
        }
    }
}
