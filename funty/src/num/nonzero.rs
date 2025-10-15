use core::num::NonZero as CoreNonZero;

use super::{
	Signed,
	Unsigned,
};
use crate::Fundamental;

mod error;
mod traits;
pub use self::error::ZeroValueError;

/// Helper that corresponds to the still-unstable [`ZeroablePrimitive`][0]
/// trait. Only used as a generic bound for [`NonZero`].
///
/// [0]: `core::num::ZeroablePrimitive`
pub trait Zeroable: Fundamental {
	/// The standard library `core::num::NonZero<Self>` type being wrapped.
	type NonZero: Copy;

	/// The minimum non-zero value of this type. This trait is implemented on
	/// `T`, not `NonZero<T>`, so it needs to be named differently to not
	/// conflict.
	const MIN_NZ: Self::NonZero;

	/// The maximum non-zero value of this type. This trait is implemented on
	/// `T`, not `NonZero<T>`, so it needs to be named differently to not
	/// conflict.
	const MAX_NZ: Self::NonZero;

	/// Fallible `(T -> NonZero<T>)` constructor.
	fn new(self) -> Option<Self::NonZero>;

	/// Infallible `(T -> NonZero<T>)` constructor.
	///
	/// # Safety
	///
	/// Behavior is undefined when `self` is `T::ZERO`.
	unsafe fn new_unchecked(self) -> Self::NonZero;

	/// `(NonZero<T> -> T)` deconstructor. Field projection on range-limited
	/// types is illegal.
	fn get(this: Self::NonZero) -> Self;
}

macro_rules! nonzero {
	( $($t:ty => $nz:ident $(, @lo $min:expr)?);+ $(;)? ) => { $(
		impl Zeroable for $t {
			type NonZero = CoreNonZero<$nz>;

			const MIN_NZ: Self::NonZero = unsafe {
				<Self::NonZero>::new_unchecked([$($min,)? <$nz>::MIN][0])
			};
			const MAX_NZ: Self::NonZero = unsafe {
				<Self::NonZero>::new_unchecked(<$nz>::MAX)
			};

			#[inline(always)]
			fn new(self) -> Option<Self::NonZero> {
				<Self::NonZero>::new(self as $nz)
			}

			#[inline(always)]
			unsafe fn new_unchecked(self) -> Self::NonZero {
				unsafe { <Self::NonZero>::new_unchecked(self as $nz) }
			}

			#[inline(always)]
			fn get(this: Self::NonZero) -> Self {
				this.get()
			}
		}
	)+ };
}

nonzero! {
	i8    => i8;
	i16   => i16;
	i32   => i32;
	i64   => i64;
	i128  => i128;
	isize => isize;
	u8    => u8,    @lo 1;
	u16   => u16,   @lo 1;
	u32   => u32,   @lo 1;
	u64   => u64,   @lo 1;
	u128  => u128,  @lo 1;
	usize => usize, @lo 1;
	char  => char,  @lo '\x01';
}

/** A value that is known not to equal zero.

This enables some memory layout optimization. In particular,
`Option<NonZero<T>>` is the same size as `T`:

```rust
use core::mem;
use funty::num::NonZero;

assert_eq!(mem::size_of::<Option<NonZero<u32>>>(), mem::size_of::<u32>());
```

# Original

[`core::num::NonZero`]

# API Differences

Like the `core` type, this allows `NonZero<char>`, even though there is no
`NonZeroChar` direct type.

# Implementation

This type is a `repr(transparent)` wrapper over `core::num::NonZero<T>` and
therefore inherits all of its properties and guarantees.

# Layout

`NonZero<T>` is guaranteed to have the same layout and bit validity as `T`, with
the exception that the all-zero bit pattern is invalid. `Option<NonZero<T>>` is
guaranteed to be compatible with `T`, incnluding in FFI.

Thanks to the [null pointer optimization][0], `NonZero<T>` and
`Option<NonZero<T>>` are guaranteed to have the same size and alignment:

```rust
use core::mem;
use funty::num::{NonZero, Zeroable};

const fn same_layout<T: Zeroable>() -> bool {
  mem::size_of::<NonZero<T>>() == mem::size_of::<Option<NonZero<T>>>() &&
  mem::align_of::<NonZero<T>>() == mem::align_of::<Option<NonZero<T>>>()
}

assert!(same_layout::<u64>());
assert!(same_layout::<char>());
```

# Note on Generic Usage

`NonZero<T>` only works on the numeric (and `char`) language primitives. The
type parameter `T` must implement Funty’s internal trait [`Zeroable`], which is
sealed against foreign implementation. Therefore, you cannot fill `NonZero<T>`
with your own types. You can, however, implement traits for all
`NonZero<T: Zeroable>`, since unlike the standard library, Funty cannot
partially hide its public symbols.

[0]: https://doc.rust-lang.org/core/option/index.html#representation
*/
#[repr(transparent)]
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonZero<T>
where T: Zeroable
{
	inner: T::NonZero,
}

impl<T> NonZero<T>
where T: Zeroable
{
	/// The size of this non-zero integer type in bits.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::{Fundamental, num::NonZero};
	/// assert_eq!(NonZero::<i32>::BITS, i32::BITS);
	/// ```
	pub const BITS: u32 = <T as Fundamental>::BITS;
	/// The maximum value this type can hold.
	pub const MAX: Self = Self::from_nonzero(T::MAX_NZ);
	/// The minimum value this type can hold. For signed types, it is not `-1`,
	/// but rather `<int>::MIN`. The value range is discontiguous at zero.
	pub const MIN: Self = Self::from_nonzero(T::MIN_NZ);

	/// Creates a non-zero if the given value is not zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::new`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	#[inline(always)]
	pub fn new(value: T) -> Option<Self> {
		Zeroable::new(value).map(Self::from_nonzero)
	}

	/// Creates a non-zero without checking whether the value is non-zero. This
	/// results in undefined behavior if the value is zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::new_unchecked`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Safety
	///
	/// The value must not be zero.
	#[inline(always)]
	pub unsafe fn new_unchecked(value: T) -> Self {
		Self::from_nonzero(unsafe { Zeroable::new_unchecked(value) })
	}

	/// Returns the contained value as a primitive type.
	///
	/// # Original
	///
	/// [`core::num::NonZero::get`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	#[inline(always)]
	pub fn get(self) -> T {
		Zeroable::get(self.inner)
	}

	/// Converts `core::num::NonZero` into `funty::num::NonZero`.
	#[inline(always)]
	const fn from_nonzero(value: T::NonZero) -> Self {
		Self { inner: value }
	}
}

impl<T> NonZero<T>
where T: Zeroable + super::Integral
{
	/// Returns the number of leading zeros in the binary representation of
	/// `self`.
	///
	/// On many architectures, this function can perform better than
	/// `leading_zeros()` on the underlying integer type, as special handling of
	/// zero can be avoided.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::leading_zeros`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let n = NonZero::new(-1i32)?;
	/// assert_eq!(n.leading_zeros(), 0);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn leading_zeros(self) -> u32 {
		self.get().leading_zeros()
	}

	/// Returns the number of trailing zeros in the binary representation of
	/// `self`.
	///
	/// On many architectures, this function can perform better than
	/// `trailing_zeros()` on the underlying integer type, as special handling
	/// of zero can be avoided.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::trailing_zeros`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let n = NonZero::new(-8i32)?;
	/// assert_eq!(n.trailing_zeros(), 3);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn trailing_zeros(self) -> u32 {
		self.get().trailing_zeros()
	}

	/// Returns the number of ones in the binary representation of `self`.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::count_ones`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Feature Gate
	///
	/// ```toml
	/// [dependencies.funty]
	/// features = ["rust_186"]
	/// ```
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let a = NonZero::new(0b100_0000i32)?;
	/// let b = NonZero::new(0b100_0011i32)?;
	///
	/// assert_eq!(a.count_ones(), NonZero::new(1)?);
	/// assert_eq!(b.count_ones(), NonZero::new(3)?);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	#[cfg(feature = "rust_186")]
	pub fn count_ones(self) -> NonZero<u32> {
		unsafe { NonZero::new_unchecked(self.get().count_ones()) }
	}

	/// Multiplies two non-zero integers together. Returns `None` on overflow.
	/// As a consequence, the result cannot wrap to zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::checked_mul`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let two = NonZero::new(2i32)?;
	/// let four = NonZero::new(4i32)?;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(Some(four), two.checked_mul(two));
	/// assert!(max.checked_mul(two).is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_mul(self, other: Self) -> Option<Self> {
		self.get().checked_mul(other.get()).and_then(Self::new)
	}

	/// Multiplies two non-zero integers together, returning [`NonZero::MAX`] on
	/// overflow.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::saturating_mul`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let two = NonZero::new(2i32)?;
	/// let four = NonZero::new(4i32)?;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(four, two.saturating_mul(two));
	/// assert_eq!(max, four.saturating_mul(max));
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn saturating_mul(self, other: Self) -> Self {
		unsafe { Self::new_unchecked(self.get().saturating_mul(other.get())) }
	}

	/// Raises a non-zero value to an integer power, returning `None` on
	/// overflow. As a consequence, the result cannot wrap to zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::checked_pow`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let three = NonZero::new(3i32)?;
	/// let twenty_seven = NonZero::new(27i32)?;
	/// let half_max = NonZero::new(i32::MAX / 2)?;
	///
	/// assert_eq!(Some(twenty_seven), three.checked_pow(3));
	/// assert!(half_max.checked_pow(3).is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_pow(self, other: u32) -> Option<Self> {
		self.get().checked_pow(other).and_then(Self::new)
	}

	/// Raises a non-zero value to an integer power, returning [`NonZero::MIN`]
	/// or [`NonZero::MAX`] on overflow.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::saturating_pow`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let three = NonZero::new(3i32)?;
	/// let twenty_seven = NonZero::new(27i32)?;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(twenty_seven, three.saturating_pow(3));
	/// assert_eq!(max, max.saturating_pow(3));
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn saturating_pow(self, other: u32) -> Self {
		unsafe { Self::new_unchecked(self.get().saturating_pow(other)) }
	}
}

impl<T> NonZero<T>
where T: Zeroable + Signed
{
	/// Computes the absolute value of `self`. See [`Signed::abs`] for
	/// documentation on overflow behavior.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(1i32)?;
	/// let neg = NonZero::new(-1i32)?;
	///
	/// assert_eq!(Some(pos), neg.checked_abs());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn abs(self) -> Self {
		unsafe { Self::new_unchecked(self.get().abs()) }
	}

	/// Checked absolute value. Checks for overflow and returns `None` if `self
	/// == Signed::MIN`. The result cannot be zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::checked_abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(1i32)?;
	/// let neg = NonZero::new(-1i32)?;
	/// let min = NonZero::<i32>::MIN;
	///
	/// assert_eq!(Some(pos), neg.checked_abs());
	/// assert!(min.checked_abs().is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_abs(self) -> Option<Self> {
		self.get().checked_abs().and_then(Self::new)
	}

	/// Computes the absolute value of `self`, with overflow information.
	///
	/// See [`Signed::overflowing_abs`].
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::overflowing_abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(1i32)?;
	/// let neg = NonZero::new(-1i32)?;
	/// let min = NonZero::<i32>::MIN;
	///
	/// assert_eq!((pos, false), pos.overflowing_abs());
	/// assert_eq!((pos, false), neg.overflowing_abs());
	/// assert_eq!((min, true), min.overflowing_abs());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn overflowing_abs(self) -> (Self, bool) {
		let (abs, ovf) = self.get().overflowing_abs();
		(unsafe { Self::new_unchecked(abs) }, ovf)
	}

	/// Saturating absolute value. See [`Signed::saturating_abs`].
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::saturating_abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(1i32)?;
	/// let neg = NonZero::new(-1i32)?;
	/// let min = NonZero::<i32>::MIN;
	/// let min_plus = NonZero::new(i32::MIN + 1)?;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(pos, pos.saturating_abs());
	/// assert_eq!(pos, neg.saturating_abs());
	/// assert_eq!(max, min.saturating_abs());
	/// assert_eq!(max, min_plus.saturating_abs());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn saturating_abs(self) -> Self {
		unsafe { Self::new_unchecked(self.get().saturating_abs()) }
	}

	/// Wrapping absolute value. See [`Signed::wrapping_abs`].
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::wrapping_abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(1i32)?;
	/// let neg = NonZero::new(-1i32)?;
	/// let min = NonZero::<i32>::MIN;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(pos, pos.wrapping_abs());
	/// assert_eq!(pos, neg.wrapping_abs());
	/// assert_eq!(min, min.wrapping_abs());
	/// assert_eq!(max, (-max).wrapping_abs());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn wrapping_abs(self) -> Self {
		unsafe { Self::new_unchecked(self.get().wrapping_abs()) }
	}

	/// Computes the absolute value of `self` without any wrapping or panicking.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::unsigned_abs`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let u_pos = NonZero::new(1u32)?;
	/// let i_pos = NonZero::new(1i32)?;
	/// let i_neg = NonZero::new(-1i32)?;
	/// let i_min = NonZero::<i32>::MIN;
	/// let u_max = NonZero::new(u32::MAX / 2 + 1)?;
	///
	/// assert_eq!(u_pos, i_pos.unsigned_abs());
	/// assert_eq!(u_pos, i_neg.unsigned_abs());
	/// assert_eq!(u_max, i_min.unsigned_abs());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn unsigned_abs(self) -> NonZero<T::Unsigned> {
		unsafe { NonZero::new_unchecked(self.get().unsigned_abs()) }
	}

	/// Tests if `self` is positive or not.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::is_positive`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// assert!(NonZero::new(5i32)?.is_positive());
	/// assert!(!NonZero::new(-5i32)?.is_positive());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn is_positive(self) -> bool {
		self.get().is_positive()
	}

	/// Tests if `self` is negative or not.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::is_negative`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// assert!(!NonZero::new(5i32)?.is_negative());
	/// assert!(NonZero::new(-5i32)?.is_negative());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn is_negative(self) -> bool {
		self.get().is_negative()
	}

	/// Checked negation. Computes `-self`, returning `None` if `self ==
	/// NonZero::MIN`.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::checked_neg`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// assert_eq!(NonZero::new(5i32)?.checked_neg(), NonZero::new(-5i32));
	/// assert!(NonZero::<i32>::MIN.checked_neg().is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_neg(self) -> Option<Self> {
		self.get().checked_neg().and_then(Self::new)
	}

	/// Negates `self`, overflowing if this is equal to the minimum value.
	///
	/// See [`Integral::overflowing_neg`] for documentation on overflow
	/// behavior.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::overflowing_neg`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(5i32)?;
	/// let neg = NonZero::new(-5i32)?;
	/// let min = NonZero::<i32>::MIN;
	///
	/// assert_eq!(pos.overflowing_neg(), (neg, false));
	/// assert_eq!(min.overflowing_neg(), (min, true));
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline]
	pub fn overflowing_neg(self) -> (Self, bool) {
		let (neg, ovf) = self.get().overflowing_neg();
		(unsafe { Self::new_unchecked(neg) }, ovf)
	}

	/// Saturating negation. Computes `-self`, returning [`NonZero::MAX`] if
	/// `self == NonZero::MIN` instead of overflowing.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::saturating_neg`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(5i32)?;
	/// let neg = NonZero::new(-5i32)?;
	/// let min = NonZero::<i32>::MIN;
	/// let min_plus = NonZero::new(i32::MIN + 1)?;
	/// let max = NonZero::<i32>::MAX;
	///
	/// assert_eq!(pos.saturating_neg(), neg);
	/// assert_eq!(min.saturating_neg(), max);
	/// assert_eq!(max.saturating_neg(), min_plus);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn saturating_neg(self) -> Self {
		unsafe { Self::new_unchecked(self.get().saturating_neg()) }
	}

	/// Wrapping (modular) negation. Computes `-self`, wrapping around at the
	/// boundary of the type.
	///
	/// See [`Integral::wrapping_neg`] for documentation on overflow behavior.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::wrapping_neg`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let pos = NonZero::new(5i32)?;
	/// let neg = NonZero::new(-5i32)?;
	/// let min = NonZero::<i32>::MIN;
	///
	/// assert_eq!(pos.wrapping_neg(), neg);
	/// assert_eq!(min.wrapping_neg(), min);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn wrapping_neg(self) -> Self {
		unsafe { Self::new_unchecked(self.get().wrapping_neg()) }
	}

	/// Returns the bit-pattern of `self`, reïnterpreted as an unsigned integer
	/// of the same width.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<i32>::cast_unsigned`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Feature Gate
	///
	/// ```toml
	/// [dependencies.funty]
	/// features = ["rust_186"]
	/// ```
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let n = NonZero::new(-1i32)?;
	/// assert_eq!(n.cast_unsigned(), NonZero::<u32>::MAX);
	/// # Some(()) })().ok_or_else(ZeroValueError::<i32>::new).unwrap()
	/// ```
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub fn cast_unsigned(self) -> NonZero<T::Unsigned> {
		unsafe { NonZero::new_unchecked(self.get().cast_unsigned()) }
	}
}

impl<T> NonZero<T>
where T: Zeroable + Unsigned
{
	/// Adds an unsigned integer to a non-zero value, returning `None` on
	/// overflow. As a consequence, the result cannot wrap to zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::checked_add`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let one = NonZero::new(1u32)?;
	/// let two = NonZero::new(2u32)?;
	/// let max = NonZero::<u32>::MAX;
	///
	/// assert_eq!(Some(two), one.checked_add(1));
	/// assert!(max.checked_add(1).is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_add(self, other: T) -> Option<Self> {
		self.get().checked_add(other).and_then(Self::new)
	}

	/// Adds an unsigned integer to a non-zero value, returning [`NonZero::MAX`]
	/// on overflow.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::saturating_add`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let one = NonZero::new(1u32)?;
	/// let two = NonZero::new(2u32)?;
	/// let max = NonZero::<u32>::MAX;
	///
	/// assert_eq!(two, one.saturating_add(1));
	/// assert_eq!(max, max.saturating_add(1));
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn saturating_add(self, other: T) -> Self {
		unsafe { Self::new_unchecked(self.get().saturating_add(other)) }
	}

	/// Finds the smallest power of two greater than or equal to `self`,
	/// returning `None` if the next power of two is greater than the type’s
	/// maximum value. As a consequence, the result cannot wrap to zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::checked_next_power_of_two`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let two = NonZero::new(2u32)?;
	/// let three = NonZero::new(3u32)?;
	/// let four = NonZero::new(4u32)?;
	/// let max = NonZero::new(u32::MAX)?;
	///
	/// assert_eq!(Some(two), two.checked_next_power_of_two());
	/// assert_eq!(Some(four), three.checked_next_power_of_two());
	/// assert!(max.checked_next_power_of_two().is_none());
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline]
	pub fn checked_next_power_of_two(self) -> Option<Self> {
		self.get().checked_next_power_of_two().and_then(Self::new)
	}

	/// Returns the base-2 logarithm of the number, rounded down
	///
	/// This is the same operation as [`Integral::ilog2`], except that it has no
	/// failure cases to worry about since this value can never be zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::ilog2`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// assert_eq!(NonZero::new(7u32)?.ilog2(), 2);
	/// assert_eq!(NonZero::new(8u32)?.ilog2(), 3);
	/// assert_eq!(NonZero::new(9u32)?.ilog2(), 3);
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn ilog2(self) -> u32 {
		self.get().ilog2()
	}

	/// Returns the base-10 logarithm of the number, rounded down
	///
	/// This is the same operation as [`Integral::ilog10`], except that it has
	/// no failure cases to worry about since this value can never be zero.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::ilog10`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// assert_eq!(NonZero::new(99u32)?.ilog10(), 1);
	/// assert_eq!(NonZero::new(100u32)?.ilog10(), 2);
	/// assert_eq!(NonZero::new(101u32)?.ilog10(), 2);
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn ilog10(self) -> u32 {
		self.get().ilog10()
	}

	/// Calculates the midpoint (average) between `self` and `rhs`.
	///
	/// `midpoint(a, b)` is `(a + b) >> 1` as if it were performed in a
	/// sufficiently-large signed integral type. This implies that the result is
	/// always rounded towards negative infinity and that no overflow will ever
	/// occur.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::midpoint`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let one = NonZero::new(1u32)?;
	/// let two = NonZero::new(2u32)?;
	/// let four = NonZero::new(4u32)?;
	///
	/// assert_eq!(one.midpoint(four), two);
	/// assert_eq!(four.midpoint(one), two);
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub fn midpoint(self, rhs: Self) -> Self {
		unsafe { Self::new_unchecked(self.get().midpoint(rhs.get())) }
	}

	/// Tests if `self` has exactly one bit set (is equal to `1 << k` for some k
	/// in `0..BITS`).
	///
	/// On many architectures, this function can perform better than
	/// `is_power_of_two()` on the underlying integer type, as special handling
	/// of zero can be avoided.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::is_power_of_two`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let eight = NonZero::new(8u32)?;
	/// assert!(eight.is_power_of_two());
	/// let ten = NonZero::new(10u32)?;
	/// assert!(!ten.is_power_of_two());
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn is_power_of_two(self) -> bool {
		self.get().is_power_of_two()
	}

	/// Returns the square root of the number, rounded down.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::isqrt`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let ten = NonZero::new(10u32)?;
	/// let three = NonZero::new(3u32)?;
	///
	/// assert_eq!(ten.isqrt(), three);
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	pub fn isqrt(self) -> Self {
		unsafe { Self::new_unchecked(self.get().isqrt()) }
	}

	/// Returns the bit-pattern of `self`, reïnterpreted as a signed integer of
	/// the same width.
	///
	/// # Original
	///
	/// [`core::num::NonZero::<u32>::cast_signed`]
	///
	/// # API Differences
	///
	/// Because this routes through a helper trait, it is not `const fn`.
	///
	/// # Feature Gate
	///
	/// ```toml
	/// [dependencies.funty]
	/// features = ["rust_187"]
	/// ```
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::num::*; (|| -> Option<()> {
	/// let n = NonZero::<u32>::MAX;
	/// assert_eq!(n.cast_signed(), NonZero::new(-1i32)?);
	/// # Some(()) })().ok_or_else(ZeroValueError::<u32>::new).unwrap()
	/// ```
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub fn cast_signed(self) -> NonZero<T::Signed> {
		unsafe { NonZero::new_unchecked(self.get().cast_signed()) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_layout<T: Zeroable>() {
		assert!(crate::layout_equal::<T, NonZero<T>>());
		assert!(crate::layout_equal::<NonZero<T>, Option<NonZero<T>>>());
	}

	#[test]
	fn layout() {
		assert_layout::<i8>();
		assert_layout::<i16>();
		assert_layout::<i32>();
		assert_layout::<i64>();
		assert_layout::<i128>();
		assert_layout::<isize>();
		assert_layout::<u8>();
		assert_layout::<u16>();
		assert_layout::<u32>();
		assert_layout::<u64>();
		assert_layout::<u128>();
		assert_layout::<usize>();
	}
}
