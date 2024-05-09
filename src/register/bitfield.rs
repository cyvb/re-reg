//! ## crate::register::bitfield
//!
//! This mod contains bit field utilities.

use core::marker::PhantomData;
use core::ops::Add;

use crate::int::UIntLike;

use super::RegName;


/// Together with trait `RegName`, this trait offers compile time checks
/// to ensure a bit field can be used only on the corresponding register.
pub trait BitsLike<T: UIntLike> {}

impl BitsLike<u8>  for () {}
impl BitsLike<u16> for () {}
impl BitsLike<u32> for () {}
impl BitsLike<u64> for () {}

/// ## Masked Value
///
/// This struct indicates the result of putting values into target bits.
///
/// You can combine different `MaskedValue` and `Bits` to represent the final state
/// in the target bits.
pub struct MaskedVal<T: UIntLike, R: RegName> {
    val: T,
    mask: T,
    _reg: PhantomData<R>
}

impl<T: UIntLike, R: RegName> Add<MaskedVal<T, R>> for MaskedVal<T, R> {
    type Output = MaskedVal<T, R>;

    #[inline]
    fn add(mut self, rhs: MaskedVal<T, R>) -> Self::Output {
        self.val  |= rhs.val;
        self.mask |= rhs.mask;
        self
    }
}

impl<T: UIntLike, R: RegName> Add<Bits<T, R>> for MaskedVal<T, R> {
    type Output = MaskedVal<T, R>;

    #[inline]
    fn add(mut self, rhs: Bits<T, R>) -> Self::Output {
        self.val  |= rhs.mask;
        self.mask |= rhs.mask;
        self
    }
}

impl<T: UIntLike, R: RegName> Add<MaskedVal<T, R>> for Bits<T, R> {
    type Output = MaskedVal<T, R>;

    #[inline]
    fn add(self, mut rhs: MaskedVal<T, R>) -> Self::Output {
        rhs.val  |= self.mask;
        rhs.mask |= self.mask;
        rhs
    }
}

/// ## Bits
///
/// The behind-the-scene struct used to perform value calculations for bit-field
/// operations.
///
/// The `R: RegName` generic prevents users to wronly use a bit field for register
/// `A` on another register `B`.
///
/// Bit field operations are implemented inside corresponding `IO` traits. `Bits` itself
/// only provides a combination method (by implementing `Add`) and a `val()` method to
/// represent a value in the target field.
#[derive(Clone, Copy)]
pub struct Bits<T: UIntLike, R: RegName> {
    offset: u8,
    mask: T,
    _reg: PhantomData<R>
}

impl<T: UIntLike, R: RegName> Add<Bits<T, R>> for Bits<T, R> {
    type Output = Bits<T, R>;

    #[inline]
    fn add(self, rhs: Bits<T, R>) -> Self::Output {
        Bits {
            offset: core::cmp::min(self.offset, rhs.offset),
            mask: self.mask | rhs.mask,
            _reg: PhantomData
        }
    }
}

impl<T: UIntLike, R: RegName> Bits<T, R> {
    pub const fn new(offset: u8, mask: T) -> Self {
        Self {
            offset,
            mask,
            _reg: PhantomData
        }
    }
}

impl<T: UIntLike, R: RegName> Bits<T, R> {
    /// The result of putting a value into target bits.
    #[inline]
    pub fn val(&self, val: T) -> MaskedVal<T, R> {
        MaskedVal {
            val: (val << self.offset as usize) & self.mask,
            mask: self.mask,
            _reg: PhantomData
        }
    }
}


/// ## Readable IO trait
///
/// This trait contains reading-related operations.
pub trait ReadableIO<T: UIntLike, R: RegName = ()> {
    fn read(&self) -> T;

    /// Get the value in the target field. Don't use combined `Bits` here.
    #[inline]
    fn get(&self, bits: Bits<T, R>) -> T {
        (self.read() & bits.mask) >> (bits.offset as usize)
    }

    /// Check if target bits are set.
    #[inline]
    fn is_set(&self, bits: Bits<T, R>) -> bool {
        (self.read() & bits.mask) == bits.mask
    }
}

/// Writable IO trait
///
/// This trait contains writing related operations. Methods here will overwrite
/// non-target bits' values to 0.
pub trait WritableIO<T: UIntLike, R: RegName = ()>
{
    /// Write a value to the IO.
    fn write(&self, val: T);

    /// Put a value into target bits, other bits become 0.
    #[inline]
    fn put(&self, val: MaskedVal<T, R>) {
        self.write(val.val);
    }

    /// Set target bits to 1, other bits become 0.
    #[inline]
    fn set(&self, bits: Bits<T, R>) {
        self.write(bits.mask);
    }

    /// Set all bits to 1.
    #[inline]
    fn set_all(&self) {
        self.write(T::all());
    }

    /// Set all bits to 0.
    #[inline]
    fn clear_all(&self) {
        self.write(T::zero());
    }
}

/// Read-Writable IO trait
///
/// This trait contains methods that only exist if an IO is both readable and writable.
/// The addition `put_back()`, `set_back()` and `clear()` methods are useful if you want
/// to keep other bits' value while putting new values into target bits.
pub trait ReadWritableIO<T: UIntLike, R: RegName = ()> {
    fn put_back(&self, val: MaskedVal<T, R>);
    fn set_back(&self, bits: Bits<T, R>);
    fn clear(&self, bits: Bits<T, R>);
}

impl<U, T: UIntLike, R: RegName> ReadWritableIO<T, R> for U
where
    U: ReadableIO<T, R> + WritableIO<T, R>
{
    /// Put the value into target bits, while keeping others untouched.
    #[inline]
    fn put_back(&self, val: MaskedVal<T, R>) {
        self.write(self.read() & (!val.mask) | val.val );
    }

    /// Set target bits, while keeping others untouched.
    #[inline]
    fn set_back(&self, bits: Bits<T, R>) {
        self.write(self.read() | bits.mask);
    }

    /// Set target bits to 0, while keeping others untouched.
    #[inline]
    fn clear(&self, bits: Bits<T, R>) {
        self.write(self.read() & (!bits.mask));
    }
}
