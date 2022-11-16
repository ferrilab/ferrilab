//! Traits for the numeric types.

use crate::Fundamental;
use core::{
	fmt::{
		Binary,
		LowerExp,
		LowerHex,
		Octal,
		UpperExp,
		UpperHex,
	},
	hash::Hash,
	iter::{
		Product,
		Sum,
	},
	num::{
		FpCategory,
		ParseIntError,
	},
	ops::{
		Add,
		AddAssign,
		BitAnd,
		BitAndAssign,
		BitOr,
		BitOrAssign,
		BitXor,
		BitXorAssign,
		Div,
		DivAssign,
		Mul,
		MulAssign,
		Neg,
		Not,
		Rem,
		RemAssign,
		Shl,
		ShlAssign,
		Shr,
		ShrAssign,
		Sub,
		SubAssign,
	},
};

new_trait! {
	/// Declares that a type is an abstract number.
	///
	/// This unifies all of the signed-integer, unsigned-integer, and
	/// floating-point types.
	Numeric:
		Fundamental
		//  iter
		, Product<Self>
		, @for<'a> Product<&'a Self>
		, Sum<Self>
		, @for<'a> Sum<&'a Self>
		//  numeric ops
		, Add<Self, Output = Self>
		, @for<'a> Add<&'a Self, Output = Self>
		, AddAssign<Self>
		, @for<'a> AddAssign<&'a Self>
		, Sub<Self, Output = Self>
		, @for<'a> Sub<&'a Self, Output = Self>
		, SubAssign<Self>
		, @for<'a> SubAssign<&'a Self>
		, Mul<Self, Output = Self>
		, @for<'a> Mul<&'a Self, Output = Self>
		, MulAssign<Self>
		, @for<'a> MulAssign<&'a Self>
		, Div<Self, Output = Self>
		, @for<'a> Div<&'a Self, Output = Self>
		, DivAssign<Self>
		, @for<'a> DivAssign<&'a Self>
		, Rem<Self, Output = Self>
		, @for<'a> Rem<&'a Self, Output = Self>
		, RemAssign<Self>
		, @for<'a> RemAssign<&'a Self>
	{
		/// The `[u8; N]` byte array that stores values of `Self`.
		type Bytes;

		new_trait! { i32 @
			fn to_be_bytes(self) -> Self::Bytes;
			fn to_le_bytes(self) -> Self::Bytes;
			fn to_ne_bytes(self) -> Self::Bytes;

			fn from_be_bytes(bytes: Self::Bytes) -> Self;
			fn from_le_bytes(bytes: Self::Bytes) -> Self;
			fn from_ne_bytes(bytes: Self::Bytes) -> Self;
		}
	}
}

new_trait! {
	/// Declares that a type is a fixed-point integer.
	///
	/// This unifies all of the signed and unsigned integral types.
	Integral:
		Numeric
		, Hash
		, Eq
		, Ord
		, Binary
		, LowerHex
		, UpperHex
		, Octal
		, BitAnd<Self, Output = Self>
		, @for<'a> BitAnd<&'a Self, Output = Self>
		, BitAndAssign<Self>
		, @for<'a> BitAndAssign<&'a Self>
		, BitOr<Self, Output = Self>
		, @for<'a> BitOr<&'a Self, Output = Self>
		, BitOrAssign<Self>
		, @for<'a> BitOrAssign<&'a Self>
		, BitXor<Self, Output = Self>
		, @for<'a> BitXor<&'a Self, Output = Self>
		, BitXorAssign<Self>
		, @for<'a> BitXorAssign<&'a Self>
		, Not<Output = Self>
		, TryFrom<i8>
		, TryFrom<u8>
		, TryFrom<i16>
		, TryFrom<u16>
		, TryFrom<i32>
		, TryFrom<u32>
		, TryFrom<i64>
		, TryFrom<u64>
		, TryFrom<i128>
		, TryFrom<u128>
		, TryFrom<isize>
		, TryFrom<usize>
		, TryInto<i8>
		, TryInto<u8>
		, TryInto<i16>
		, TryInto<u16>
		, TryInto<i32>
		, TryInto<u32>
		, TryInto<i64>
		, TryInto<u64>
		, TryInto<i128>
		, TryInto<u128>
		, TryInto<isize>
		, TryInto<usize>
		, Shl<Self, Output = Self>
		, @for<'a> Shl<&'a Self, Output = Self>
		, ShlAssign<Self>
		, @for<'a> ShlAssign<&'a Self>
		, Shr<Self, Output = Self>
		, @for<'a> Shr<&'a Self, Output = Self>
		, ShrAssign<Self>
		, @for<'a> ShrAssign<&'a Self>
		, Shl<i8, Output = Self>
		, @for<'a> Shl<&'a i8, Output = Self>
		, ShlAssign<i8>
		, @for<'a> ShlAssign<&'a i8>
		, Shr<i8, Output = Self>
		, @for<'a> Shr<&'a i8, Output = Self>
		, ShrAssign<i8>
		, @for<'a> ShrAssign<&'a i8>
		, Shl<u8, Output = Self>
		, @for<'a> Shl<&'a u8, Output = Self>
		, ShlAssign<u8>
		, @for<'a> ShlAssign<&'a u8>
		, Shr<u8, Output = Self>
		, @for<'a> Shr<&'a u8, Output = Self>
		, ShrAssign<u8>
		, @for<'a> ShrAssign<&'a u8>
		, Shl<i16, Output = Self>
		, @for<'a> Shl<&'a i16, Output = Self>
		, ShlAssign<i16>
		, @for<'a> ShlAssign<&'a i16>
		, Shr<i16, Output = Self>
		, @for<'a> Shr<&'a i16, Output = Self>
		, ShrAssign<i16>
		, @for<'a> ShrAssign<&'a i16>
		, Shl<u16, Output = Self>
		, @for<'a> Shl<&'a u16, Output = Self>
		, ShlAssign<u16>
		, @for<'a> ShlAssign<&'a u16>
		, Shr<u16, Output = Self>
		, @for<'a> Shr<&'a u16, Output = Self>
		, ShrAssign<u16>
		, @for<'a> ShrAssign<&'a u16>
		, Shl<i32, Output = Self>
		, @for<'a> Shl<&'a i32, Output = Self>
		, ShlAssign<i32>
		, @for<'a> ShlAssign<&'a i32>
		, Shr<i32, Output = Self>
		, @for<'a> Shr<&'a i32, Output = Self>
		, ShrAssign<i32>
		, @for<'a> ShrAssign<&'a i32>
		, Shl<u32, Output = Self>
		, @for<'a> Shl<&'a u32, Output = Self>
		, ShlAssign<u32>
		, @for<'a> ShlAssign<&'a u32>
		, Shr<u32, Output = Self>
		, @for<'a> Shr<&'a u32, Output = Self>
		, ShrAssign<u32>
		, @for<'a> ShrAssign<&'a u32>
		, Shl<i64, Output = Self>
		, @for<'a> Shl<&'a i64, Output = Self>
		, ShlAssign<i64>
		, @for<'a> ShlAssign<&'a i64>
		, Shr<i64, Output = Self>
		, @for<'a> Shr<&'a i64, Output = Self>
		, ShrAssign<i64>
		, @for<'a> ShrAssign<&'a i64>
		, Shl<u64, Output = Self>
		, @for<'a> Shl<&'a u64, Output = Self>
		, ShlAssign<u64>
		, @for<'a> ShlAssign<&'a u64>
		, Shr<u64, Output = Self>
		, @for<'a> Shr<&'a u64, Output = Self>
		, ShrAssign<u64>
		, @for<'a> ShrAssign<&'a u64>
		, Shl<i128, Output = Self>
		, @for<'a> Shl<&'a i128, Output = Self>
		, ShlAssign<i128>
		, @for<'a> ShlAssign<&'a i128>
		, Shr<i128, Output = Self>
		, @for<'a> Shr<&'a i128, Output = Self>
		, ShrAssign<i128>
		, @for<'a> ShrAssign<&'a i128>
		, Shl<u128, Output = Self>
		, @for<'a> Shl<&'a u128, Output = Self>
		, ShlAssign<u128>
		, @for<'a> ShlAssign<&'a u128>
		, Shr<u128, Output = Self>
		, @for<'a> Shr<&'a u128, Output = Self>
		, ShrAssign<u128>
		, @for<'a> ShrAssign<&'a u128>
		, Shl<isize, Output = Self>
		, @for<'a> Shl<&'a isize, Output = Self>
		, ShlAssign<isize>
		, @for<'a> ShlAssign<&'a isize>
		, Shr<isize, Output = Self>
		, @for<'a> Shr<&'a isize, Output = Self>
		, ShrAssign<isize>
		, @for<'a> ShrAssign<&'a isize>
		, Shl<usize, Output = Self>
		, @for<'a> Shl<&'a usize, Output = Self>
		, ShlAssign<usize>
		, @for<'a> ShlAssign<&'a usize>
		, Shr<usize, Output = Self>
		, @for<'a> Shr<&'a usize, Output = Self>
		, ShrAssign<usize>
		, @for<'a> ShrAssign<&'a usize>
	{
		/// The signed integer of this bit width.
		type Signed: Integral;

		/// The unsigned integer of this bit width.
		type Unsigned: Integral;

		/// The type’s zero value.
		const ZERO: Self;

		/// The type’s step value.
		const ONE: Self;

		new_trait! { i32 @
			const MIN: Self;
			const MAX: Self;
			const BITS: u32;
		}

		new_trait! { i32 @
			#[deprecated = "Deprecating in a future Rust version: replaced by the `MIN` associated constant on this type"]
			fn min_value() -> Self;

			#[deprecated = "Deprecating in a future Rust version: replaced by the `MAX` associated constant on this type"]
			fn max_value() -> Self;

			fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
			fn count_ones(self) -> u32;
			fn count_zeros(self) -> u32;
			fn leading_zeros(self) -> u32;
			fn trailing_zeros(self) -> u32;
			fn leading_ones(self) -> u32;
			fn trailing_ones(self) -> u32;
			fn rotate_left(self, n: u32) -> Self;
			fn rotate_right(self, n: u32) -> Self;
			fn swap_bytes(self) -> Self;
			fn reverse_bits(self) -> Self;

			#[allow(clippy::wrong_self_convention)]
			fn from_be(self) -> Self;

			#[allow(clippy::wrong_self_convention)]
			fn from_le(self) -> Self;

			fn to_be(self) -> Self;
			fn to_le(self) -> Self;

			fn checked_add(self, rhs: Self) -> Option<Self>;
			fn checked_sub(self, rhs: Self) -> Option<Self>;
			fn checked_mul(self, rhs: Self) -> Option<Self>;
			fn checked_div(self, rhs: Self) -> Option<Self>;
			fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
			fn checked_rem(self, rhs: Self) -> Option<Self>;
			fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
			fn checked_neg(self) -> Option<Self>;
			fn checked_shl(self, rhs: u32) -> Option<Self>;
			fn checked_shr(self, rhs: u32) -> Option<Self>;
			fn checked_pow(self, rhs: u32) -> Option<Self>;

			fn saturating_add(self, rhs: Self) -> Self;
			fn saturating_sub(self, rhs: Self) -> Self;
			fn saturating_mul(self, rhs: Self) -> Self;
			fn saturating_div(self, rhs: Self) -> Self;
			fn saturating_pow(self, rhs: u32) -> Self;

			fn wrapping_add(self, rhs: Self) -> Self;
			fn wrapping_sub(self, rhs: Self) -> Self;
			fn wrapping_mul(self, rhs: Self) -> Self;
			fn wrapping_div(self, rhs: Self) -> Self;
			fn wrapping_div_euclid(self, rhs: Self) -> Self;
			fn wrapping_rem(self, rhs: Self) -> Self;
			fn wrapping_rem_euclid(self, rhs: Self) -> Self;
			fn wrapping_neg(self) -> Self;
			fn wrapping_shl(self, rhs: u32) -> Self;
			fn wrapping_shr(self, rhs: u32) -> Self;
			fn wrapping_pow(self, rhs: u32) -> Self;

			fn overflowing_add(self, rhs: Self) -> (Self, bool);
			fn overflowing_sub(self, rhs: Self) -> (Self, bool);
			fn overflowing_mul(self, rhs: Self) -> (Self, bool);
			fn overflowing_div(self, rhs: Self) -> (Self, bool);
			fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
			fn overflowing_rem(self, rhs: Self) -> (Self, bool);
			fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
			fn overflowing_neg(self) -> (Self, bool);
			fn overflowing_shl(self, rhs: u32) -> (Self, bool);
			fn overflowing_shr(self, rhs: u32) -> (Self, bool);
			fn overflowing_pow(self, rhs: u32) -> (Self, bool);

			fn abs_diff(self, rhs: Self) -> Self::Unsigned;
			fn pow(self, rhs: u32) -> Self;
			fn div_euclid(self, rhs: Self) -> Self;
			fn rem_euclid(self, rhs: Self) -> Self;
		}
	}
}

new_trait! {
	/// Declares that a type is a signed integer.
	Signed: Integral, Neg {
		new_trait! { i32 @
			fn checked_abs(self) -> Option<Self>;
			fn wrapping_abs(self) -> Self;
			fn overflowing_abs(self) -> (Self, bool);
			fn abs(self) -> Self;
			fn signum(self) -> Self;
			fn is_positive(self) -> bool;
			fn is_negative(self) -> bool;
		}
	}
}

new_trait! {
	/// Declares that a type is an unsigned integer.
	Unsigned: Integral {
		new_trait! { u32 @
			fn is_power_of_two(self) -> bool;
			fn next_power_of_two(self) -> Self;
			fn checked_next_power_of_two(self) -> Option<Self>;
		}
	}
}

new_trait! {
	/// Declares that a type is a floating-point number.
	Floating:
		Numeric
		, LowerExp
		, UpperExp
		, Neg
		, From<f32>
		, From<i8>
		, From<i16>
		, From<u8>
		, From<u16>
	{
		/// The unsigned integer type of the same bit-width as `Self`.
		type Raw: Unsigned;

		new_trait! { f32 @
			const RADIX: u32;
			const MANTISSA_DIGITS: u32;
			const DIGITS: u32;
			const EPSILON: Self;
			const MIN: Self;
			const MIN_POSITIVE: Self;
			const MAX: Self;
			const MIN_EXP: i32;
			const MAX_EXP: i32;
			const MIN_10_EXP: i32;
			const MAX_10_EXP: i32;
			const NAN: Self;
			const INFINITY: Self;
			const NEG_INFINITY: Self;
		}

		new_trait! { f32 @
			mod const PI: Self;
			mod const FRAC_PI_2: Self;
			mod const FRAC_PI_3: Self;
			mod const FRAC_PI_4: Self;
			mod const FRAC_PI_6: Self;
			mod const FRAC_PI_8: Self;
			mod const FRAC_1_PI: Self;
			mod const FRAC_2_PI: Self;
			mod const FRAC_2_SQRT_PI: Self;
			mod const SQRT_2: Self;
			mod const FRAC_1_SQRT_2: Self;
			mod const E: Self;
			mod const LOG2_E: Self;
			mod const LOG10_E: Self;
			mod const LN_2: Self;
			mod const LN_10: Self;
		}

		// These functions are only available in `std`, because they rely on the
		// system math library `libm` which `core` does not provide.

		new_trait! { f32 @
			#[cfg(feature = "std")] fn floor(self) -> Self;
			#[cfg(feature = "std")] fn ceil(self) -> Self;
			#[cfg(feature = "std")] fn round(self) -> Self;
			#[cfg(feature = "std")] fn trunc(self) -> Self;
			#[cfg(feature = "std")] fn fract(self) -> Self;
			#[cfg(feature = "std")] fn abs(self) -> Self;
			#[cfg(feature = "std")] fn signum(self) -> Self;
			#[cfg(feature = "std")] fn copysign(self, sign: Self) -> Self;
			#[cfg(feature = "std")] fn mul_add(self, a: Self, b: Self) -> Self;
			#[cfg(feature = "std")] fn div_euclid(self, rhs: Self) -> Self;
			#[cfg(feature = "std")] fn rem_euclid(self, rhs: Self) -> Self;
			#[cfg(feature = "std")] fn powi(self, n: i32) -> Self;
			#[cfg(feature = "std")] fn powf(self, n: Self) -> Self;
			#[cfg(feature = "std")] fn sqrt(self) -> Self;
			#[cfg(feature = "std")] fn exp(self) -> Self;
			#[cfg(feature = "std")] fn exp2(self) -> Self;
			#[cfg(feature = "std")] fn ln(self) -> Self;
			#[cfg(feature = "std")] fn log(self, base: Self) -> Self;
			#[cfg(feature = "std")] fn log2(self) -> Self;
			#[cfg(feature = "std")] fn log10(self) -> Self;
			#[cfg(feature = "std")] fn cbrt(self) -> Self;
			#[cfg(feature = "std")] fn hypot(self, other: Self) -> Self;
			#[cfg(feature = "std")] fn sin(self) -> Self;
			#[cfg(feature = "std")] fn cos(self) -> Self;
			#[cfg(feature = "std")] fn tan(self) -> Self;
			#[cfg(feature = "std")] fn asin(self) -> Self;
			#[cfg(feature = "std")] fn acos(self) -> Self;
			#[cfg(feature = "std")] fn atan(self) -> Self;
			#[cfg(feature = "std")] fn atan2(self, other: Self) -> Self;
			#[cfg(feature = "std")] fn sin_cos(self) -> (Self, Self);
			#[cfg(feature = "std")] fn exp_m1(self) -> Self;
			#[cfg(feature = "std")] fn ln_1p(self) -> Self;
			#[cfg(feature = "std")] fn sinh(self) -> Self;
			#[cfg(feature = "std")] fn cosh(self) -> Self;
			#[cfg(feature = "std")] fn tanh(self) -> Self;
			#[cfg(feature = "std")] fn asinh(self) -> Self;
			#[cfg(feature = "std")] fn acosh(self) -> Self;
			#[cfg(feature = "std")] fn atanh(self) -> Self;

			fn is_nan(self) -> bool;
			fn is_infinite(self) -> bool;
			fn is_finite(self) -> bool;
			fn is_normal(self) -> bool;
			fn classify(self) -> FpCategory;
			fn is_sign_positive(self) -> bool;
			fn is_sign_negative(self) -> bool;
			fn recip(self) -> Self;
			fn to_degrees(self) -> Self;
			fn to_radians(self) -> Self;
			fn max(self, other: Self) -> Self;
			fn min(self, other: Self) -> Self;
			fn to_bits(self) -> Self::Raw;
			fn from_bits(bits: Self::Raw) -> Self;
		}
	}
}
