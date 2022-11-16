#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]

use core::{
	fmt::{
		Debug,
		Display,
	},
	num::{
		FpCategory,
		ParseIntError,
	},
	str::FromStr,
};

#[macro_use]
mod macros;

pub mod num;
pub mod ptr;

pub use crate::{
	num::{
		Floating,
		Integral,
		Numeric,
		Signed,
		Unsigned,
	},
	ptr::{
		Permission,
		Pointer,
		Reference,
		Shared,
		Unique,
	},
};

/// Declares that a type is one of the language fundamental types.
pub trait Fundamental:
	'static
	+ Sized
	+ Send
	+ Sync
	+ Unpin
	+ Clone
	+ Copy
	+ Default
	+ FromStr
	//  cmp
	+ PartialEq<Self>
	+ PartialOrd<Self>
	//  fmt
	+ Debug
	+ Display
{
	/// Tests `self != 0`.
	fn as_bool(self) -> bool;

	/// Represents `self` as a Unicode Scalar Value, if possible.
	fn as_char(self) -> Option<char>;

	/// Performs `self as i8`.
	fn as_i8(self) -> i8;

	/// Performs `self as i16`.
	fn as_i16(self) -> i16;

	/// Performs `self as i32`.
	fn as_i32(self) -> i32;

	/// Performs `self as i64`.
	fn as_i64(self) -> i64;

	/// Performs `self as i128`.
	fn as_i128(self) -> i128;

	/// Performs `self as isize`.
	fn as_isize(self) -> isize;

	/// Performs `self as u8`.
	fn as_u8(self) -> u8;

	/// Performs `self as u16`.
	fn as_u16(self) -> u16;

	/// Performs `self as u32`.
	fn as_u32(self) -> u32;

	/// Performs `self as u64`.
	fn as_u64(self) -> u64;

	/// Performs `self as u128`.
	fn as_u128(self) -> u128;

	/// Performs `self as usize`.
	fn as_usize(self) -> usize;

	/// Performs `self as f32`.
	fn as_f32(self) -> f32;

	/// Performs `self as f64`.
	fn as_f64(self) -> f64;
}

/// Declares that a type is exactly eight bits wide.
pub trait Is8: Fundamental {}

/// Declares that a type is exactly sixteen bits wide.
pub trait Is16: Fundamental {}

/// Declares that a type is exactly thirty-two bits wide.
pub trait Is32: Fundamental {}

/// Declares that a type is exactly sixty-four bits wide.
pub trait Is64: Fundamental {}

/// Declares that a type is exactly one hundred twenty-eight bits wide.
pub trait Is128: Fundamental {}

/// Declares that a type is eight or more bits wide.
pub trait AtLeast8: Fundamental {}

/// Declares that a type is sixteen or more bits wide.
pub trait AtLeast16: Fundamental {}

/// Declares that a type is thirty-two or more bits wide.
pub trait AtLeast32: Fundamental {}

/// Declares that a type is sixty-four or more bits wide.
pub trait AtLeast64: Fundamental {}

/// Declares that a type is one hundred twenty-eight or more bits wide.
pub trait AtLeast128: Fundamental {}

/// Declares that a type is eight or fewer bits wide.
pub trait AtMost8: Fundamental {}

/// Declares that a type is sixteen or fewer bits wide.
pub trait AtMost16: Fundamental {}

/// Declares that a type is thirty-two or fewer bits wide.
pub trait AtMost32: Fundamental {}

/// Declares that a type is sixty-four or fewer bits wide.
pub trait AtMost64: Fundamental {}

/// Declares that a type is one hundred twenty-eight or fewer bits wide.
pub trait AtMost128: Fundamental {}

macro_rules! impl_for {
	(Fundamental => $($t:ty => $is_zero:expr),+ $(,)?) => { $(
		impl Fundamental for $t {
			#[inline(always)]
			#[allow(clippy::redundant_closure_call)]
			fn as_bool(self) -> bool { ($is_zero)(self) }

			#[inline(always)]
			fn as_char(self) -> Option<char> {
				core::char::from_u32(self as u32)
			}

			#[inline(always)]
			fn as_i8(self) -> i8 { self as i8 }

			#[inline(always)]
			fn as_i16(self) -> i16 { self as i16 }

			#[inline(always)]
			fn as_i32(self) -> i32 { self as i32 }

			#[inline(always)]
			fn as_i64(self) -> i64 { self as i64 }

			#[inline(always)]
			fn as_i128(self) -> i128 { self as i128 }

			#[inline(always)]
			fn as_isize(self) -> isize { self as isize }

			#[inline(always)]
			fn as_u8(self) -> u8 { self as u8 }

			#[inline(always)]
			fn as_u16(self) -> u16 { self as u16 }

			#[inline(always)]
			fn as_u32(self) -> u32 { self as u32 }

			#[inline(always)]
			fn as_u64(self) -> u64 { self as u64 }

			#[inline(always)]
			fn as_u128(self) ->u128 { self as u128 }

			#[inline(always)]
			fn as_usize(self) -> usize { self as usize }

			#[inline(always)]
			fn as_f32(self) -> f32 { self as f32 }

			#[inline(always)]
			fn as_f64(self) -> f64 { self as f64 }
		}
	)+ };
	(Numeric => $($t:ty),+ $(,)?) => { $(
		impl Numeric for $t {
			type Bytes = [u8; core::mem::size_of::<Self>()];

			items! { $t =>
				fn to_be_bytes(self) -> Self::Bytes;
				fn to_le_bytes(self) -> Self::Bytes;
				fn to_ne_bytes(self) -> Self::Bytes;
			}
			items! { $t =>
				fn from_be_bytes(bytes: Self::Bytes) -> Self;
				fn from_le_bytes(bytes: Self::Bytes) -> Self;
				fn from_ne_bytes(bytes: Self::Bytes) -> Self;
			}
		}
	)+ };
	(Integral => { $($t:ty, $s:ty, $u:ty);+ $(;)? }) => { $(
		impl Integral for $t {
			type Signed = $s;
			type Unsigned = $u;

			const ZERO: Self = 0;
			const ONE: Self = 1;

			items! { $t =>
				const MIN: Self;
				const MAX: Self;
				const BITS: u32;
			}

			items! { $t =>
				fn min_value() -> Self;
				fn max_value() -> Self;
				fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
			}
			items! { $t =>
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
				fn from_be(self) -> Self;
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
				fn abs_diff(self, rhs: Self) -> Self::Unsigned;
				fn overflowing_mul(self, rhs: Self) -> (Self, bool);
				fn overflowing_div(self, rhs: Self) -> (Self, bool);
				fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
				fn overflowing_rem(self, rhs: Self) -> (Self, bool);
				fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
				fn overflowing_neg(self) -> (Self, bool);
				fn overflowing_shl(self, rhs: u32) -> (Self, bool);
				fn overflowing_shr(self, rhs: u32) -> (Self, bool);
				fn overflowing_pow(self, rhs: u32) -> (Self, bool);
				fn pow(self, rhs: u32) -> Self;
				fn div_euclid(self, rhs: Self) -> Self;
				fn rem_euclid(self, rhs: Self) -> Self;
			}
		}
	)+ };
	(Signed => $($t:ty),+ $(,)?) => { $(
		impl Signed for $t {
			items! { $t =>
				fn checked_abs(self) -> Option<Self>;
				fn wrapping_abs(self) -> Self;
				fn overflowing_abs(self) -> (Self, bool);
				fn abs(self) -> Self;
				fn signum(self) -> Self;
				fn is_positive(self) -> bool;
				fn is_negative(self) -> bool;
			}
		}
	)+ };
	(Unsigned => $($t:ty),+ $(,)?) => { $(
		impl Unsigned for $t {
			items! { $t =>
				fn is_power_of_two(self) -> bool;
				fn next_power_of_two(self) -> Self;
				fn checked_next_power_of_two(self) -> Option<Self>;
			}
		}
	)+ };
	(Floating => $($t:ident | $u:ty),+ $(,)?) => { $(
		impl Floating for $t {
			type Raw = $u;

			items! { $t =>
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

			items! { $t =>
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

			items! { $t =>
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
			}
			items! { $t =>
				fn from_bits(bits: Self::Raw) -> Self;
			}
		}
	)+ };
	($which:ty => $($t:ty),+ $(,)?) => { $(
		impl $which for $t {}
	)+ };
}

impl Fundamental for bool {
	#[inline(always)]
	fn as_bool(self) -> bool {
		self
	}

	#[inline(always)]
	fn as_char(self) -> Option<char> {
		Some(self as u8 as char)
	}

	#[inline(always)]
	fn as_i8(self) -> i8 {
		self as i8
	}

	#[inline(always)]
	fn as_i16(self) -> i16 {
		self as i16
	}

	#[inline(always)]
	fn as_i32(self) -> i32 {
		self as i32
	}

	#[inline(always)]
	fn as_i64(self) -> i64 {
		self as i64
	}

	#[inline(always)]
	fn as_i128(self) -> i128 {
		self as i128
	}

	#[inline(always)]
	fn as_isize(self) -> isize {
		self as isize
	}

	#[inline(always)]
	fn as_u8(self) -> u8 {
		self as u8
	}

	#[inline(always)]
	fn as_u16(self) -> u16 {
		self as u16
	}

	#[inline(always)]
	fn as_u32(self) -> u32 {
		self as u32
	}

	#[inline(always)]
	fn as_u64(self) -> u64 {
		self as u64
	}

	#[inline(always)]
	fn as_u128(self) -> u128 {
		self as u128
	}

	#[inline(always)]
	fn as_usize(self) -> usize {
		self as usize
	}

	#[inline(always)]
	fn as_f32(self) -> f32 {
		self as u8 as f32
	}

	#[inline(always)]
	fn as_f64(self) -> f64 {
		self as u8 as f64
	}
}

impl Fundamental for char {
	#[inline(always)]
	fn as_bool(self) -> bool {
		self != '\0'
	}

	#[inline(always)]
	fn as_char(self) -> Option<char> {
		Some(self)
	}

	#[inline(always)]
	fn as_i8(self) -> i8 {
		self as i8
	}

	#[inline(always)]
	fn as_i16(self) -> i16 {
		self as i16
	}

	#[inline(always)]
	fn as_i32(self) -> i32 {
		self as i32
	}

	#[inline(always)]
	fn as_i64(self) -> i64 {
		self as i64
	}

	#[inline(always)]
	fn as_i128(self) -> i128 {
		self as i128
	}

	#[inline(always)]
	fn as_isize(self) -> isize {
		self as isize
	}

	#[inline(always)]
	fn as_u8(self) -> u8 {
		self as u8
	}

	#[inline(always)]
	fn as_u16(self) -> u16 {
		self as u16
	}

	#[inline(always)]
	fn as_u32(self) -> u32 {
		self as u32
	}

	#[inline(always)]
	fn as_u64(self) -> u64 {
		self as u64
	}

	#[inline(always)]
	fn as_u128(self) -> u128 {
		self as u128
	}

	#[inline(always)]
	fn as_usize(self) -> usize {
		self as usize
	}

	#[inline(always)]
	fn as_f32(self) -> f32 {
		self as u32 as f32
	}

	#[inline(always)]
	fn as_f64(self) -> f64 {
		self as u32 as f64
	}
}

impl_for!(Fundamental =>
	i8 => |this| this != 0,
	i16 => |this| this != 0,
	i32 => |this| this != 0,
	i64 => |this| this != 0,
	i128 => |this| this != 0,
	isize => |this| this != 0,
	u8 => |this| this != 0,
	u16 => |this| this != 0,
	u32 => |this| this != 0,
	u64 => |this| this != 0,
	u128 => |this| this != 0,
	usize => |this| this != 0,
	f32 => |this: f32| (-Self::EPSILON ..= Self::EPSILON).contains(&this),
	f64 => |this: f64| (-Self::EPSILON ..= Self::EPSILON).contains(&this),
);

impl_for!(Numeric =>
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
	f32, f64,
);

impl_for!(Integral => {
	i8, i8, u8;
	i16, i16, u16;
	i32, i32, u32;
	i64, i64, u64;
	i128, i128, u128;
	isize, isize, usize;
	u8, i8, u8;
	u16, i16, u16;
	u32, i32, u32;
	u64, i64, u64;
	u128, i128, u128;
	usize, isize, usize;
});

impl_for!(Signed => i8, i16, i32, i64, i128, isize);

impl_for!(Unsigned => u8, u16, u32, u64, u128, usize);

impl_for!(Floating => f32 | u32, f64 | u64);

impl_for!(Is8 => i8, u8);
impl_for!(Is16 => i16, u16);
impl_for!(Is32 => i32, u32, f32);
impl_for!(Is64 => i64, u64, f64);
impl_for!(Is128 => i128, u128);

#[cfg(target_pointer_width = "16")]
impl_for!(Is16 => isize, usize);

#[cfg(target_pointer_width = "32")]
impl_for!(Is32 => isize, usize);

#[cfg(target_pointer_width = "64")]
impl_for!(Is64 => isize, usize);

impl_for!(AtLeast8 =>
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
	f32, f64,
);
impl_for!(AtLeast16 => i16, i32, i64, i128, u16, u32, u64, u128, f32, f64);
impl_for!(AtLeast32 => i32, i64, i128, u32, u64, u128, f32, f64);
impl_for!(AtLeast64 => i64, i128, u64, u128, f64);
impl_for!(AtLeast128 => i128, u128);

#[cfg(any(
	target_pointer_width = "16",
	target_pointer_width = "32",
	target_pointer_width = "64"
))]
impl_for!(AtLeast16 => isize, usize);

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for!(AtLeast32 => isize, usize);

#[cfg(target_pointer_width = "64")]
impl_for!(AtLeast64 => isize, usize);

impl_for!(AtMost8 => i8, u8);
impl_for!(AtMost16 => i8, i16, u8, u16);
impl_for!(AtMost32 => i8, i16, i32, u8, u16, u32, f32);
impl_for!(AtMost64 =>
	i8, i16, i32, i64, isize,
	u8, u16, u32, u64, usize,
	f32, f64,
);
impl_for!(AtMost128 =>
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
	f32, f64,
);

#[cfg(target_pointer_width = "16")]
impl_for!(AtMost16 => isize, usize);

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
impl_for!(AtMost32 => isize, usize);

#[cfg(test)]
mod tests {
	use super::*;
	use static_assertions::*;

	assert_impl_all!(bool: Fundamental);
	assert_impl_all!(char: Fundamental);

	assert_impl_all!(i8: Integral, Signed, Is8);
	assert_impl_all!(i16: Integral, Signed, Is16);
	assert_impl_all!(i32: Integral, Signed, Is32);
	assert_impl_all!(i64: Integral, Signed, Is64);
	assert_impl_all!(i128: Integral, Signed, Is128);
	assert_impl_all!(isize: Integral, Signed);

	assert_impl_all!(u8: Integral, Unsigned, Is8);
	assert_impl_all!(u16: Integral, Unsigned, Is16);
	assert_impl_all!(u32: Integral, Unsigned, Is32);
	assert_impl_all!(u64: Integral, Unsigned, Is64);
	assert_impl_all!(u128: Integral, Unsigned, Is128);
	assert_impl_all!(usize: Integral, Unsigned);

	assert_impl_all!(f32: Floating, Is32);
	assert_impl_all!(f64: Floating, Is64);
}
