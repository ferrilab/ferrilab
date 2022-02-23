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

pub mod num;
pub use self::num::{
	Floating,
	Integral,
	Numeric,
	Signed,
	Unsigned,
};

/// Declare that a type is one of the language fundamental types.
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

/// Declare that a type is exactly eight bits wide.
pub trait Is8: Fundamental {}

/// Declare that a type is exactly sixteen bits wide.
pub trait Is16: Fundamental {}

/// Declare that a type is exactly thirty-two bits wide.
pub trait Is32: Fundamental {}

/// Declare that a type is exactly sixty-four bits wide.
pub trait Is64: Fundamental {}

/// Declare that a type is exactly one hundred twenty-eight bits wide.
pub trait Is128: Fundamental {}

/// Declare that a type is eight or more bits wide.
pub trait AtLeast8: Fundamental {}

/// Declare that a type is sixteen or more bits wide.
pub trait AtLeast16: Fundamental {}

/// Declare that a type is thirty-two or more bits wide.
pub trait AtLeast32: Fundamental {}

/// Declare that a type is sixty-four or more bits wide.
pub trait AtLeast64: Fundamental {}

/// Declare that a type is one hundred twenty-eight or more bits wide.
pub trait AtLeast128: Fundamental {}

/// Declare that a type is eight or fewer bits wide.
pub trait AtMost8: Fundamental {}

/// Declare that a type is sixteen or fewer bits wide.
pub trait AtMost16: Fundamental {}

/// Declare that a type is thirty-two or fewer bits wide.
pub trait AtMost32: Fundamental {}

/// Declare that a type is sixty-four or fewer bits wide.
pub trait AtMost64: Fundamental {}

/// Declare that a type is one hundred twenty-eight or fewer bits wide.
pub trait AtMost128: Fundamental {}

/// Creates new wrapper functions that forward to inherent items of the same
/// name and signature.
macro_rules! func {
	(
		$(@$std:literal)?
		$name:ident (self$(, $arg:ident: $t:ty)*) $(-> $ret:ty)?;
		$($tt:tt)*
	) => {
		$(#[cfg(feature = $std)])?
		fn $name(self$(, $arg: $t)*) $(-> $ret)?
		{
			<Self>::$name(self$(, $arg)*)
		}

		func!($($tt)*);
	};
	(
		$(@$std:literal)?
		$name:ident(&self$(, $arg:ident: $t:ty)*) $(-> $ret:ty)?;
		$($tt:tt)*
	) => {
		$(#[cfg(feature = $std)])?
		fn $name(&self$(, $arg: $t)*) $(-> $ret)?
		{
			<Self>::$name(&self$(, $arg )*)
		}

		func!($($tt)*);
	};
	(
		$(@$std:literal)?
		$name:ident(&mut self$(, $arg:ident: $t:ty)*) $(-> $ret:ty)?;
		$($tt:tt)*
	) => {
		$(#[cfg(feature = $std)])?
		fn $name(&mut self$(, $arg: $t)*) $(-> $ret)?
		{
			<Self>::$name(&mut self$(, $arg)*)
		}

		func!($($tt)*);
	};
	(
		$(@$std:literal)?
		$name:ident($($arg:ident: $t:ty),* $(,)?) $(-> $ret:ty)?;
		$($tt:tt)*
	) => {
		$(#[cfg(feature = $std)])?
		fn $name($($arg: $t),*) $(-> $ret)?
		{
			<Self>::$name($($arg),*)
		}

		func!($($tt)*);
	};
	() => {};
}

macro_rules! impl_for {
	( Fundamental => $($t:ty => $is_zero:expr),+ $(,)? ) => { $(
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
	( Numeric => $($t:ty),+ $(,)? ) => { $(
		impl Numeric for $t {
			type Bytes = [u8; core::mem::size_of::<Self>()];

			func! {
				to_be_bytes(self) -> Self::Bytes;
				to_le_bytes(self) -> Self::Bytes;
				to_ne_bytes(self) -> Self::Bytes;
				from_be_bytes(bytes: Self::Bytes) -> Self;
				from_le_bytes(bytes: Self::Bytes) -> Self;
				from_ne_bytes(bytes: Self::Bytes) -> Self;
			}
		}
	)+ };
	( Integral => $($t:ty),+ $(,)? ) => { $(
		impl Integral for $t {
			const ZERO: Self = 0;
			const ONE: Self = 1;
			const MIN: Self = <Self>::min_value();
			const MAX: Self = <Self>::max_value();

			const BITS: u32 = <Self>::BITS;

			func! {
				min_value() -> Self;
				max_value() -> Self;
				from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
				count_ones(self) -> u32;
				count_zeros(self) -> u32;
				leading_zeros(self) -> u32;
				trailing_zeros(self) -> u32;
				leading_ones(self) -> u32;
				trailing_ones(self) -> u32;
				rotate_left(self, n: u32) -> Self;
				rotate_right(self, n: u32) -> Self;
				swap_bytes(self) -> Self;
				reverse_bits(self) -> Self;
				from_be(self) -> Self;
				from_le(self) -> Self;
				to_be(self) -> Self;
				to_le(self) -> Self;
				checked_add(self, rhs: Self) -> Option<Self>;
				checked_sub(self, rhs: Self) -> Option<Self>;
				checked_mul(self, rhs: Self) -> Option<Self>;
				checked_div(self, rhs: Self) -> Option<Self>;
				checked_div_euclid(self, rhs: Self) -> Option<Self>;
				checked_rem(self, rhs: Self) -> Option<Self>;
				checked_rem_euclid(self, rhs: Self) -> Option<Self>;
				checked_neg(self) -> Option<Self>;
				checked_shl(self, rhs: u32) -> Option<Self>;
				checked_shr(self, rhs: u32) -> Option<Self>;
				checked_pow(self, rhs: u32) -> Option<Self>;
				saturating_add(self, rhs: Self) -> Self;
				saturating_sub(self, rhs: Self) -> Self;
				saturating_mul(self, rhs: Self) -> Self;
				saturating_pow(self, rhs: u32) -> Self;
				wrapping_add(self, rhs: Self) -> Self;
				wrapping_sub(self, rhs: Self) -> Self;
				wrapping_mul(self, rhs: Self) -> Self;
				wrapping_div(self, rhs: Self) -> Self;
				wrapping_div_euclid(self, rhs: Self) -> Self;
				wrapping_rem(self, rhs: Self) -> Self;
				wrapping_rem_euclid(self, rhs: Self) -> Self;
				wrapping_neg(self) -> Self;
				wrapping_shl(self, rhs: u32) -> Self;
				wrapping_shr(self, rhs: u32) -> Self;
				wrapping_pow(self, rhs: u32) -> Self;
				overflowing_add(self, rhs: Self) -> (Self, bool);
				overflowing_sub(self, rhs: Self) -> (Self, bool);
				overflowing_mul(self, rhs: Self) -> (Self, bool);
				overflowing_div(self, rhs: Self) -> (Self, bool);
				overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
				overflowing_rem(self, rhs: Self) -> (Self, bool);
				overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
				overflowing_neg(self) -> (Self, bool);
				overflowing_shl(self, rhs: u32) -> (Self, bool);
				overflowing_shr(self, rhs: u32) -> (Self, bool);
				overflowing_pow(self, rhs: u32) -> (Self, bool);
				pow(self, rhs: u32) -> Self;
				div_euclid(self, rhs: Self) -> Self;
				rem_euclid(self, rhs: Self) -> Self;
			}
		}
	)+ };
	( Signed => $($t:ty),+ $(,)? ) => { $(
		impl Signed for $t {
			func! {
				checked_abs(self) -> Option<Self>;
				wrapping_abs(self) -> Self;
				overflowing_abs(self) -> (Self, bool);
				abs(self) -> Self;
				signum(self) -> Self;
				is_positive(self) -> bool;
				is_negative(self) -> bool;
			}
		}
	)+ };
	( Unsigned => $($t:ty),+ $(,)? ) => { $(
		impl Unsigned for $t {
			func! {
				is_power_of_two(self) -> bool;
				next_power_of_two(self) -> Self;
				checked_next_power_of_two(self) -> Option<Self>;
			}
		}
	)+ };
	( Floating => $($t:ident | $u:ty),+ $(,)? ) => { $(
		impl Floating for $t {
			type Raw = $u;

			const RADIX: u32 = core::$t::RADIX;
			const MANTISSA_DIGITS: u32 = core::$t::MANTISSA_DIGITS;
			const DIGITS: u32 = core::$t::DIGITS;
			const EPSILON: Self = core::$t::EPSILON;
			const MIN: Self = core::$t::MIN;
			const MIN_POSITIVE: Self = core::$t::MIN_POSITIVE;
			const MAX: Self = core::$t::MAX;
			const MIN_EXP: i32 = core::$t::MIN_EXP;
			const MAX_EXP: i32 = core::$t::MAX_EXP;
			const MIN_10_EXP: i32 = core::$t::MIN_10_EXP;
			const MAX_10_EXP: i32 = core::$t::MAX_10_EXP;
			const NAN: Self = core::$t::NAN;
			const INFINITY: Self = core::$t::INFINITY;
			const NEG_INFINITY: Self = core::$t::NEG_INFINITY;

			const PI: Self = core::$t::consts::PI;
			const FRAC_PI_2: Self = core::$t::consts::FRAC_PI_2;
			const FRAC_PI_3: Self = core::$t::consts::FRAC_PI_3;
			const FRAC_PI_4: Self = core::$t::consts::FRAC_PI_4;
			const FRAC_PI_6: Self = core::$t::consts::FRAC_PI_6;
			const FRAC_PI_8: Self = core::$t::consts::FRAC_PI_8;
			const FRAC_1_PI: Self = core::$t::consts::FRAC_1_PI;
			const FRAC_2_PI: Self = core::$t::consts::FRAC_2_PI;
			const FRAC_2_SQRT_PI: Self = core::$t::consts::FRAC_2_SQRT_PI;
			const SQRT_2: Self = core::$t::consts::SQRT_2;
			const FRAC_1_SQRT_2: Self = core::$t::consts::FRAC_1_SQRT_2;
			const E: Self = core::$t::consts::E;
			const LOG2_E: Self = core::$t::consts::LOG2_E;
			const LOG10_E: Self = core::$t::consts::LOG10_E;
			const LN_2: Self = core::$t::consts::LN_2;
			const LN_10: Self = core::$t::consts::LN_10;

			func! {
				@"std" floor(self) -> Self;
				@"std" ceil(self) -> Self;
				@"std" round(self) -> Self;
				@"std" trunc(self) -> Self;
				@"std" fract(self) -> Self;
				@"std" abs(self) -> Self;
				@"std" signum(self) -> Self;
				@"std" copysign(self, sign: Self) -> Self;
				@"std" mul_add(self, a: Self, b: Self) -> Self;
				@"std" div_euclid(self, rhs: Self) -> Self;
				@"std" rem_euclid(self, rhs: Self) -> Self;
				@"std" powi(self, n: i32) -> Self;
				@"std" powf(self, n: Self) -> Self;
				@"std" sqrt(self) -> Self;
				@"std" exp(self) -> Self;
				@"std" exp2(self) -> Self;
				@"std" ln(self) -> Self;
				@"std" log(self, base: Self) -> Self;
				@"std" log2(self) -> Self;
				@"std" log10(self) -> Self;
				@"std" cbrt(self) -> Self;
				@"std" hypot(self, other: Self) -> Self;
				@"std" sin(self) -> Self;
				@"std" cos(self) -> Self;
				@"std" tan(self) -> Self;
				@"std" asin(self) -> Self;
				@"std" acos(self) -> Self;
				@"std" atan(self) -> Self;
				@"std" atan2(self, other: Self) -> Self;
				@"std" sin_cos(self) -> (Self, Self);
				@"std" exp_m1(self) -> Self;
				@"std" ln_1p(self) -> Self;
				@"std" sinh(self) -> Self;
				@"std" cosh(self) -> Self;
				@"std" tanh(self) -> Self;
				@"std" asinh(self) -> Self;
				@"std" acosh(self) -> Self;
				@"std" atanh(self) -> Self;
				is_nan(self) -> bool;
				is_infinite(self) -> bool;
				is_finite(self) -> bool;
				is_normal(self) -> bool;
				classify(self) -> FpCategory;
				is_sign_positive(self) -> bool;
				is_sign_negative(self) -> bool;
				recip(self) -> Self;
				to_degrees(self) -> Self;
				to_radians(self) -> Self;
				max(self, other: Self) -> Self;
				min(self, other: Self) -> Self;
				to_bits(self) -> Self::Raw;
				from_bits(bits: Self::Raw) -> Self;
			}
		}
	)+ };
	( $which:ty => $($t:ty),+ $(,)? ) => { $(
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

impl_for!(Numeric => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_for!(Integral => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
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

impl_for!(AtLeast8 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
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
impl_for!(AtMost64 => i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
impl_for!(AtMost128 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

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
