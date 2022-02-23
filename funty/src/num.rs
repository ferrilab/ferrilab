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

/// Declare that a type is an abstract number.
///
/// This unifies all of the signed-integer, unsigned-integer, and floating-point
/// types.
pub trait Numeric:
	Fundamental
	//  iter
	+ Product<Self>
	+ for<'a> Product<&'a Self>
	+ Sum<Self>
	+ for<'a> Sum<&'a Self>
	//  numeric ops
	+ Add<Self, Output = Self>
	+ for<'a> Add<&'a Self, Output = Self>
	+ AddAssign<Self>
	+ for<'a> AddAssign<&'a Self>
	+ Sub<Self, Output = Self>
	+ for<'a> Sub<&'a Self, Output = Self>
	+ SubAssign<Self>
	+ for<'a> SubAssign<&'a Self>
	+ Mul<Self, Output = Self>
	+ for<'a> Mul<&'a Self, Output = Self>
	+ MulAssign<Self>
	+ for<'a> MulAssign<&'a Self>
	+ Div<Self, Output = Self>
	+ for<'a> Div<&'a Self, Output = Self>
	+ DivAssign<Self>
	+ for<'a> DivAssign<&'a Self>
	+ Rem<Self, Output = Self>
	+ for<'a> Rem<&'a Self, Output = Self>
	+ RemAssign<Self>
	+ for<'a> RemAssign<&'a Self>
{
	/// The `[u8; N]` byte array that stores values of `Self`.
	type Bytes;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.to_be_bytes>.
	fn to_be_bytes(self) -> Self::Bytes;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.to_le_bytes>.
	fn to_le_bytes(self) -> Self::Bytes;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.to_ne_bytes>.
	fn to_ne_bytes(self) -> Self::Bytes;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_be_bytes>.
	fn from_be_bytes(bytes: Self::Bytes) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_le_bytes>.
	fn from_le_bytes(bytes: Self::Bytes) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_ne_bytes>.
	fn from_ne_bytes(bytes: Self::Bytes) -> Self;
}

/// Declare that a type is a fixed-point integer.
///
/// This unifies all of the signed and unsigned integral types.
pub trait Integral:
	Numeric
	+ Hash
	+ Eq
	+ Ord
	+ Binary
	+ LowerHex
	+ UpperHex
	+ Octal
	+ BitAnd<Self, Output = Self>
	+ for<'a> BitAnd<&'a Self, Output = Self>
	+ BitAndAssign<Self>
	+ for<'a> BitAndAssign<&'a Self>
	+ BitOr<Self, Output = Self>
	+ for<'a> BitOr<&'a Self, Output = Self>
	+ BitOrAssign<Self>
	+ for<'a> BitOrAssign<&'a Self>
	+ BitXor<Self, Output = Self>
	+ for<'a> BitXor<&'a Self, Output = Self>
	+ BitXorAssign<Self>
	+ for<'a> BitXorAssign<&'a Self>
	+ Not<Output = Self>
	+ TryFrom<i8>
	+ TryFrom<u8>
	+ TryFrom<i16>
	+ TryFrom<u16>
	+ TryFrom<i32>
	+ TryFrom<u32>
	+ TryFrom<i64>
	+ TryFrom<u64>
	+ TryFrom<i128>
	+ TryFrom<u128>
	+ TryFrom<isize>
	+ TryFrom<usize>
	+ TryInto<i8>
	+ TryInto<u8>
	+ TryInto<i16>
	+ TryInto<u16>
	+ TryInto<i32>
	+ TryInto<u32>
	+ TryInto<i64>
	+ TryInto<u64>
	+ TryInto<i128>
	+ TryInto<u128>
	+ TryInto<isize>
	+ TryInto<usize>
	+ Shl<Self, Output = Self>
	+ for<'a> Shl<&'a Self, Output = Self>
	+ ShlAssign<Self>
	+ for<'a> ShlAssign<&'a Self>
	+ Shr<Self, Output = Self>
	+ for<'a> Shr<&'a Self, Output = Self>
	+ ShrAssign<Self>
	+ for<'a> ShrAssign<&'a Self>
	+ Shl<i8, Output = Self>
	+ for<'a> Shl<&'a i8, Output = Self>
	+ ShlAssign<i8>
	+ for<'a> ShlAssign<&'a i8>
	+ Shr<i8, Output = Self>
	+ for<'a> Shr<&'a i8, Output = Self>
	+ ShrAssign<i8>
	+ for<'a> ShrAssign<&'a i8>
	+ Shl<u8, Output = Self>
	+ for<'a> Shl<&'a u8, Output = Self>
	+ ShlAssign<u8>
	+ for<'a> ShlAssign<&'a u8>
	+ Shr<u8, Output = Self>
	+ for<'a> Shr<&'a u8, Output = Self>
	+ ShrAssign<u8>
	+ for<'a> ShrAssign<&'a u8>
	+ Shl<i16, Output = Self>
	+ for<'a> Shl<&'a i16, Output = Self>
	+ ShlAssign<i16>
	+ for<'a> ShlAssign<&'a i16>
	+ Shr<i16, Output = Self>
	+ for<'a> Shr<&'a i16, Output = Self>
	+ ShrAssign<i16>
	+ for<'a> ShrAssign<&'a i16>
	+ Shl<u16, Output = Self>
	+ for<'a> Shl<&'a u16, Output = Self>
	+ ShlAssign<u16>
	+ for<'a> ShlAssign<&'a u16>
	+ Shr<u16, Output = Self>
	+ for<'a> Shr<&'a u16, Output = Self>
	+ ShrAssign<u16>
	+ for<'a> ShrAssign<&'a u16>
	+ Shl<i32, Output = Self>
	+ for<'a> Shl<&'a i32, Output = Self>
	+ ShlAssign<i32>
	+ for<'a> ShlAssign<&'a i32>
	+ Shr<i32, Output = Self>
	+ for<'a> Shr<&'a i32, Output = Self>
	+ ShrAssign<i32>
	+ for<'a> ShrAssign<&'a i32>
	+ Shl<u32, Output = Self>
	+ for<'a> Shl<&'a u32, Output = Self>
	+ ShlAssign<u32>
	+ for<'a> ShlAssign<&'a u32>
	+ Shr<u32, Output = Self>
	+ for<'a> Shr<&'a u32, Output = Self>
	+ ShrAssign<u32>
	+ for<'a> ShrAssign<&'a u32>
	+ Shl<i64, Output = Self>
	+ for<'a> Shl<&'a i64, Output = Self>
	+ ShlAssign<i64>
	+ for<'a> ShlAssign<&'a i64>
	+ Shr<i64, Output = Self>
	+ for<'a> Shr<&'a i64, Output = Self>
	+ ShrAssign<i64>
	+ for<'a> ShrAssign<&'a i64>
	+ Shl<u64, Output = Self>
	+ for<'a> Shl<&'a u64, Output = Self>
	+ ShlAssign<u64>
	+ for<'a> ShlAssign<&'a u64>
	+ Shr<u64, Output = Self>
	+ for<'a> Shr<&'a u64, Output = Self>
	+ ShrAssign<u64>
	+ for<'a> ShrAssign<&'a u64>
	+ Shl<i128, Output = Self>
	+ for<'a> Shl<&'a i128, Output = Self>
	+ ShlAssign<i128>
	+ for<'a> ShlAssign<&'a i128>
	+ Shr<i128, Output = Self>
	+ for<'a> Shr<&'a i128, Output = Self>
	+ ShrAssign<i128>
	+ for<'a> ShrAssign<&'a i128>
	+ Shl<u128, Output = Self>
	+ for<'a> Shl<&'a u128, Output = Self>
	+ ShlAssign<u128>
	+ for<'a> ShlAssign<&'a u128>
	+ Shr<u128, Output = Self>
	+ for<'a> Shr<&'a u128, Output = Self>
	+ ShrAssign<u128>
	+ for<'a> ShrAssign<&'a u128>
	+ Shl<isize, Output = Self>
	+ for<'a> Shl<&'a isize, Output = Self>
	+ ShlAssign<isize>
	+ for<'a> ShlAssign<&'a isize>
	+ Shr<isize, Output = Self>
	+ for<'a> Shr<&'a isize, Output = Self>
	+ ShrAssign<isize>
	+ for<'a> ShrAssign<&'a isize>
	+ Shl<usize, Output = Self>
	+ for<'a> Shl<&'a usize, Output = Self>
	+ ShlAssign<usize>
	+ for<'a> ShlAssign<&'a usize>
	+ Shr<usize, Output = Self>
	+ for<'a> Shr<&'a usize, Output = Self>
	+ ShrAssign<usize>
	+ for<'a> ShrAssign<&'a usize>
{
	/// The type’s zero value.
	const ZERO: Self;

	/// The type’s step value.
	const ONE: Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN>.
	const MIN: Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX>.
	const MAX: Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.BITS>.
	const BITS: u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.min_value>.
	#[deprecated = "Deprecating in a future Rust version: replaced by the `MIN` \
	                associated constant on this type"]
	fn min_value() -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.max_value>.
	#[deprecated = "Deprecating in a future Rust version: replaced by the `MAX` \
	                associated constant on this type"]
	fn max_value() -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_str_radix>.
	fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.count_ones>.
	fn count_ones(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.count_zeros>.
	fn count_zeros(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.leading_zeros>.
	fn leading_zeros(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.trailing_zeros>.
	fn trailing_zeros(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.leading_ones>.
	fn leading_ones(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.trailing_ones>.
	fn trailing_ones(self) -> u32;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.rotate_left>.
	fn rotate_left(self, n: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.rotate_right>.
	fn rotate_right(self, n: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.swap_bytes>.
	fn swap_bytes(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.reverse_bits>.
	fn reverse_bits(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_be>.
	#[allow(clippy::wrong_self_convention)]
	fn from_be(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.from_le>.
	#[allow(clippy::wrong_self_convention)]
	fn from_le(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.to_be>.
	fn to_be(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.to_le>.
	fn to_le(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add>.
	fn checked_add(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_sub>.
	fn checked_sub(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_mul>.
	fn checked_mul(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_div>.
	fn checked_div(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_div_euclid>.
	fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_rem>.
	fn checked_rem(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_rem_euclid>.
	fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_neg>.
	fn checked_neg(self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_shl>.
	fn checked_shl(self, rhs: u32) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_shr>.
	fn checked_shr(self, rhs: u32) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_pow>.
	fn checked_pow(self, rhs: u32) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_add>.
	fn saturating_add(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_sub>.
	fn saturating_sub(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_mul>.
	fn saturating_mul(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_pow>.
	fn saturating_pow(self, rhs: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_add>.
	fn wrapping_add(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_sub>.
	fn wrapping_sub(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_mul>.
	fn wrapping_mul(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_div>.
	fn wrapping_div(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_div_euclid>.
	fn wrapping_div_euclid(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_rem>.
	fn wrapping_rem(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_rem_euclid>.
	fn wrapping_rem_euclid(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_neg>.
	fn wrapping_neg(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_shl>.
	fn wrapping_shl(self, rhs: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_shr>.
	fn wrapping_shr(self, rhs: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_pow>.
	fn wrapping_pow(self, rhs: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_add>.
	fn overflowing_add(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_sub>.
	fn overflowing_sub(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_mul>.
	fn overflowing_mul(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_div>.
	fn overflowing_div(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_div_euclid>.
	fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_rem>.
	fn overflowing_rem(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_rem_euclid>.
	fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_neg>.
	fn overflowing_neg(self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_shl>.
	fn overflowing_shl(self, rhs: u32) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_shr>.
	fn overflowing_shr(self, rhs: u32) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_pow>.
	fn overflowing_pow(self, rhs: u32) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.pow>.
	fn pow(self, rhs: u32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.div_euclid>.
	fn div_euclid(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid>.
	fn rem_euclid(self, rhs: Self) -> Self;
}

/// Declare that a type is a signed integer.
pub trait Signed: Integral + Neg {
	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.checked_abs>.
	fn checked_abs(self) -> Option<Self>;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_abs>.
	fn wrapping_abs(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_abs>.
	fn overflowing_abs(self) -> (Self, bool);

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.abs>.
	fn abs(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.signum>.
	fn signum(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.is_positive>.
	fn is_positive(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.i32.html#method.is_negative>.
	fn is_negative(self) -> bool;
}

/// Declare that a type is an unsigned integer.
pub trait Unsigned: Integral {
	/// See <https://doc.rust-lang.org/std/primitive.u32.html#method.is_power_of_two>.
	fn is_power_of_two(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.u32.html#method.next_power_of_tow>.
	fn next_power_of_two(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.u32.html#method.checked_next_power_of_two>.
	fn checked_next_power_of_two(self) -> Option<Self>;
}

/// Declare that a type is a floating-point number.
pub trait Floating:
	Numeric
	+ LowerExp
	+ UpperExp
	+ Neg
	+ From<f32>
	+ From<i8>
	+ From<i16>
	+ From<u8>
	+ From<u16>
{
	/// The unsigned integer type of the same width as `Self`.
	type Raw: Unsigned;

	/// See <https://doc.rust-lang.org/std/primitive.i64.html#associatedconstant.RADIX>.
	const RADIX: u32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MANTISSA_DIGITS>.
	const MANTISSA_DIGITS: u32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.DIGITS>.
	const DIGITS: u32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.EPSILON>.
	const EPSILON: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MIN>.
	const MIN: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MIN_POSITIVE>.
	const MIN_POSITIVE: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX>.
	const MAX: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MIN_EXP>.
	const MIN_EXP: i32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX_EXP>.
	const MAX_EXP: i32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MIN_10_EXP>.
	const MIN_10_EXP: i32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX_10_EXP>.
	const MAX_10_EXP: i32;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.NAN>.
	const NAN: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.INFINITY>.
	const INFINITY: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.NEG_INFINITY>.
	const NEG_INFINITY: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.PI>.
	const PI: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_PI_2>.
	const FRAC_PI_2: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_PI_3>.
	const FRAC_PI_3: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_PI_4>.
	const FRAC_PI_4: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_PI_6>.
	const FRAC_PI_6: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_PI_8>.
	const FRAC_PI_8: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_1_PI>.
	const FRAC_1_PI: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_2_PI>.
	const FRAC_2_PI: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_2_SQRT_PI>.
	const FRAC_2_SQRT_PI: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.SQRT_2>.
	const SQRT_2: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.FRAC_1_SQRT_2>.
	const FRAC_1_SQRT_2: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.E>.
	const E: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.LOG2_E>.
	const LOG2_E: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.LOG10_E>.
	const LOG10_E: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.LN_2>.
	const LN_2: Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#associated_constant.LN_10>.
	const LN_10: Self;

	//  These functions are only available in `std`, because they rely on the
	//  system math library `libm` which is not provided by `core`.

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.floor>.
	#[cfg(feature = "std")]
	fn floor(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.ceil>.
	#[cfg(feature = "std")]
	fn ceil(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.round>.
	#[cfg(feature = "std")]
	fn round(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.trunc>.
	#[cfg(feature = "std")]
	fn trunc(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.fract>.
	#[cfg(feature = "std")]
	fn fract(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.abs>.
	#[cfg(feature = "std")]
	fn abs(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.signum>.
	#[cfg(feature = "std")]
	fn signum(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.copysign>.
	#[cfg(feature = "std")]
	fn copysign(self, sign: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add>.
	#[cfg(feature = "std")]
	fn mul_add(self, a: Self, b: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.div_euclid>.
	#[cfg(feature = "std")]
	fn div_euclid(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.rem_euclid>.
	#[cfg(feature = "std")]
	fn rem_euclid(self, rhs: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.powi>.
	#[cfg(feature = "std")]
	fn powi(self, n: i32) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.powf>.
	#[cfg(feature = "std")]
	fn powf(self, n: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt>.
	#[cfg(feature = "std")]
	fn sqrt(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.exp>.
	#[cfg(feature = "std")]
	fn exp(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.exp2>.
	#[cfg(feature = "std")]
	fn exp2(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.ln>.
	#[cfg(feature = "std")]
	fn ln(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.log>.
	#[cfg(feature = "std")]
	fn log(self, base: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.log2>.
	#[cfg(feature = "std")]
	fn log2(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.log10>.
	#[cfg(feature = "std")]
	fn log10(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.cbrt>.
	#[cfg(feature = "std")]
	fn cbrt(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.hypot>.
	#[cfg(feature = "std")]
	fn hypot(self, other: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.sin>.
	#[cfg(feature = "std")]
	fn sin(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.cos>.
	#[cfg(feature = "std")]
	fn cos(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.tan>.
	#[cfg(feature = "std")]
	fn tan(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.asin>.
	#[cfg(feature = "std")]
	fn asin(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.acos>.
	#[cfg(feature = "std")]
	fn acos(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.atan>.
	#[cfg(feature = "std")]
	fn atan(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.atan2>.
	#[cfg(feature = "std")]
	fn atan2(self, other: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.sin_cos>.
	#[cfg(feature = "std")]
	fn sin_cos(self) -> (Self, Self);

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.exp_m1>.
	#[cfg(feature = "std")]
	fn exp_m1(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.ln_1p>.
	#[cfg(feature = "std")]
	fn ln_1p(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.sinh>.
	#[cfg(feature = "std")]
	fn sinh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.cosh>.
	#[cfg(feature = "std")]
	fn cosh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.tanh>.
	#[cfg(feature = "std")]
	fn tanh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.asinh>.
	#[cfg(feature = "std")]
	fn asinh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.acosh>.
	#[cfg(feature = "std")]
	fn acosh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.atanh>.
	#[cfg(feature = "std")]
	fn atanh(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan>.
	fn is_nan(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_infinite>.
	fn is_infinite(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite>.
	fn is_finite(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_normal>.
	fn is_normal(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.classify>.
	fn classify(self) -> FpCategory;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_sign_positive>.
	fn is_sign_positive(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.is_sign_negative>.
	fn is_sign_negative(self) -> bool;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.recip>.
	fn recip(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.to_degrees>.
	fn to_degrees(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians>.
	fn to_radians(self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.max>.
	fn max(self, other: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.min>.
	fn min(self, other: Self) -> Self;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits>.
	fn to_bits(self) -> Self::Raw;

	/// See <https://doc.rust-lang.org/std/primitive.f64.html#method.from_bits>.
	fn from_bits(bits: Self::Raw) -> Self;
}
