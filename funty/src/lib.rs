#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "\
data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIC0xMTYgMTI4IDEyOCIgeG1sbnM9Imh0d\
HA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBzdHlsZT0idHJhbnNmb3JtOnNrZXdYKC0xOGRlZyki\
PjxwYXRoIGQ9Ik0yOC01Mmg2MiIgc3R5bGU9InN0cm9rZTojZmY0ZjAwIi8+PHBhdGggZD0iTTQgMGg\
yNHYtMTA0aDYyIiBzdHlsZT0ic3Ryb2tlOnZhcigtLWZnKSIvPjwvZz48c3R5bGU+OnJvb3R7LS1mZz\
pibGFjazstLWJnOndoaXRlfUBtZWRpYSAocHJlZmVycy1jb2xvci1zY2hlbWU6ZGFyayl7OnJvb3R7L\
S1mZzp3aGl0ZTstLWJnOmJsYWNrfX0qe3NoYXBlLXJlbmRlcmluZzpjcmlzcEVkZ2VzfXBhdGh7c3Ry\
b2tlLXdpZHRoOjI0cHg7c3Ryb2tlLWxpbmVjYXA6YnV0dDtzdHJva2UtbGluZWpvaW46bWl0ZXI7Zml\
sbDpub25lfTwvc3R5bGU+PC9zdmc+")]
#![doc(html_logo_url = "\
data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIC0xNDAgMjU2IDI1NiIgeG1sbnM9Imh0d\
HA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiBzdHlsZT0iaGVpZ2h0Ojk1MXB4Ij48cGF0aCBjbGFzcz0i\
cGVuLXNxIHN0cm9rZS1mZyIgZD0iTTAtMTJ2LTk2aDE2OG0tMjQgMjRIMG0yNCA0OGgyNHYtMjRIMjR\
2NDhoMjRtMjQgMHYtNDhoMjRtMjQgNDh2LTQ4aDI0bTI0IDQ4di00OCIgdHJhbnNmb3JtPSJ0cmFuc2\
xhdGUoNDApc2tld1goLTE4KSIvPjxwYXRoIGNsYXNzPSJwZW4tc3Egc3Ryb2tlLWFjY2VudCIgZD0iT\
TE2OC04NGgwIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSg0MClza2V3WCgtMTgpIi8+PHBhdGggY2xhc3M9\
InBlbi1zcSBzdHJva2UtZmciIGQ9Ik0wIDEydjcyaDE0NE0yNCAxMmgyNHY0OEgyNFYzNmgyNG0yNCA\
waDI0djI0SDcyVjEybTk2IDB2NDgiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDQwKXNrZXdYKC0xOCkiLz\
48cGF0aCBjbGFzcz0icGVuLXNxIHN0cm9rZS1hY2NlbnQiIGQ9Ik0xNjggODRoMCIgdHJhbnNmb3JtP\
SJ0cmFuc2xhdGUoNDApc2tld1goLTE4KSIvPjxzdHlsZT4uc3Ryb2tlLWZne3N0cm9rZTojMDAwfUBt\
ZWRpYSAocHJlZmVycy1jb2xvci1zY2hlbWU6ZGFyayl7LnN0cm9rZS1mZ3tzdHJva2U6I2ZmZn19LnN\
0cm9rZS1hY2NlbnR7c3Ryb2tlOiNmZjRmMDB9cGF0aHtmaWxsOm5vbmU7c3Ryb2tlLXdpZHRoOjE2cH\
g7c3Ryb2tlLWxpbmVjYXA6c3F1YXJlfS5wZW4tc3F7c3Ryb2tlLWxpbmVqb2luOm1pdGVyfTwvc3R5b\
GU+PC9zdmc+")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![deny(unconditional_recursion)]

use core::{
	fmt,
	str::FromStr,
};

#[macro_use]
mod macros;

pub mod num;

/// Common-use symbol exports.
pub mod prelude {
	pub use crate::num::{
		Floating,
		Integral,
		NonZero as NonZeroFty,
		Numeric,
		Signed,
		Unsigned,
		Zeroable,
	};
}

/// Tests if two types have the same layout (size and alignment).
#[inline(always)]
pub const fn layout_equal<T, U>() -> bool {
	core::mem::size_of::<T>() == core::mem::size_of::<U>()
		&& core::mem::align_of::<T>() == core::mem::align_of::<U>()
}

mod seal {
	pub trait Sealed {}
}

/// Declares that a type is one of the language fundamental types.
pub trait Fundamental:
	'static
	+ seal::Sealed
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
	+ fmt::Debug
	+ fmt::Display
{
	/// The width, in bits, of the fundamental type.
	const BITS: u32;

	/// The numeric minimum legal value of the type.
	const MIN: Self;

	/// The numeric maximum legal value of the type.
	const MAX: Self;

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

impl seal::Sealed for bool {}

impl Fundamental for bool {
	const BITS: u32 = 8;
	const MAX: bool = true;
	const MIN: bool = false;

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

impl seal::Sealed for char {}

impl Fundamental for char {
	const BITS: u32 = 32;
	const MAX: Self = <char>::MAX;
	const MIN: Self = <char>::MIN;

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

/// Indicates that the implementor is exactly `SIZE_EQU` bits wide.
pub trait SizeEquals<const SIZE_EQU: usize>: Fundamental {}

/// Indicates that the implementor is at least `SIZE_MIN` bits wide (inclusive).
pub trait SizeGreater<const SIZE_MIN: usize>: Fundamental {}

/// Indicates that the implementor is at most `SIZE_MAX` bits wide (inclusive).
pub trait SizeLesser<const SIZE_MAX: usize>: Fundamental {}

/// Declares that a type is exactly eight bits wide.
#[deprecated = "use SizeEquals<8>"]
pub trait Is8: SizeEquals<8> {}

/// Declares that a type is exactly sixteen bits wide.
#[deprecated = "use SizeEquals<16>"]
pub trait Is16: SizeEquals<16> {}

/// Declares that a type is exactly thirty-two bits wide.
#[deprecated = "use SizeEquals<32>"]
pub trait Is32: SizeEquals<32> {}

/// Declares that a type is exactly sixty-four bits wide.
#[deprecated = "use SizeEquals<64>"]
pub trait Is64: SizeEquals<64> {}

/// Declares that a type is exactly one hundred twenty-eight bits wide.
#[deprecated = "use SizeEquals<128>"]
pub trait Is128: SizeEquals<128> {}

/// Declares that a type is eight or more bits wide.
#[deprecated = "use SizeGreater<8>"]
pub trait AtLeast8: SizeGreater<8> {}

/// Declares that a type is sixteen or more bits wide.
#[deprecated = "use SizeGreater<16>"]
pub trait AtLeast16: SizeGreater<16> {}

/// Declares that a type is thirty-two or more bits wide.
#[deprecated = "use SizeGreater<32>"]
pub trait AtLeast32: SizeGreater<32> {}

/// Declares that a type is sixty-four or more bits wide.
#[deprecated = "use SizeGreater<64>"]
pub trait AtLeast64: SizeGreater<64> {}

/// Declares that a type is one hundred twenty-eight or more bits wide.
#[deprecated = "use SizeGreater<128>"]
pub trait AtLeast128: SizeGreater<128> {}

/// Declares that a type is eight or fewer bits wide.
#[deprecated = "use SizeLesser<8>"]
pub trait AtMost8: SizeLesser<8> {}

/// Declares that a type is sixteen or fewer bits wide.
#[deprecated = "use SizeLesser<16>"]
pub trait AtMost16: SizeLesser<16> {}

/// Declares that a type is thirty-two or fewer bits wide.
#[deprecated = "use SizeLesser<32>"]
pub trait AtMost32: SizeLesser<32> {}

/// Declares that a type is sixty-four or fewer bits wide.
#[deprecated = "use SizeLesser<64>"]
pub trait AtMost64: SizeLesser<64> {}

/// Declares that a type is one hundred twenty-eight or fewer bits wide.
#[deprecated = "use SizeLesser<128>"]
pub trait AtMost128: SizeLesser<128> {}

mod deprecations {
	#![allow(deprecated)]
	use super::*;
	impl<T> Is8 for T where T: SizeEquals<8> {}
	impl<T> Is16 for T where T: SizeEquals<16> {}
	impl<T> Is32 for T where T: SizeEquals<32> {}
	impl<T> Is64 for T where T: SizeEquals<64> {}
	impl<T> Is128 for T where T: SizeEquals<128> {}
	impl<T> AtLeast8 for T where T: SizeGreater<8> {}
	impl<T> AtLeast16 for T where T: SizeGreater<16> {}
	impl<T> AtLeast32 for T where T: SizeGreater<32> {}
	impl<T> AtLeast64 for T where T: SizeGreater<64> {}
	impl<T> AtLeast128 for T where T: SizeGreater<128> {}
	impl<T> AtMost8 for T where T: SizeLesser<8> {}
	impl<T> AtMost16 for T where T: SizeLesser<16> {}
	impl<T> AtMost32 for T where T: SizeLesser<32> {}
	impl<T> AtMost64 for T where T: SizeLesser<64> {}
	impl<T> AtMost128 for T where T: SizeLesser<128> {}
}

impl_for!(SizeEquals<8> => i8, u8);
impl_for!(SizeEquals<16> => i16, u16);
impl_for!(SizeEquals<32> => i32, u32, f32);
impl_for!(SizeEquals<64> => i64, u64, f64);
impl_for!(SizeEquals<128> => i128, u128);

#[cfg(target_pointer_width = "16")]
impl_for!(SizeEquals<16> => isize, usize);

#[cfg(target_pointer_width = "32")]
impl_for!(SizeEquals<32> => isize, usize);

#[cfg(target_pointer_width = "64")]
impl_for!(SizeEquals<64> => isize, usize);

impl_for!(SizeGreater<8> =>
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
	f32, f64,
);
impl_for!(SizeGreater<16> => i16, i32, i64, i128, u16, u32, u64, u128, f32, f64);
impl_for!(SizeGreater<32> => i32, i64, i128, u32, u64, u128, f32, f64);
impl_for!(SizeGreater<64> => i64, i128, u64, u128, f64);
impl_for!(SizeGreater<128> => i128, u128);

#[cfg(any(
	target_pointer_width = "16",
	target_pointer_width = "32",
	target_pointer_width = "64"
))]
impl_for!(SizeGreater<16> => isize, usize);

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for!(SizeGreater<32> => isize, usize);

#[cfg(target_pointer_width = "64")]
impl_for!(SizeGreater<64> => isize, usize);

impl_for!(SizeLesser<8> => i8, u8);
impl_for!(SizeLesser<16> => i8, i16, u8, u16);
impl_for!(SizeLesser<32> => i8, i16, i32, u8, u16, u32, f32);
impl_for!(SizeLesser<64> =>
	i8, i16, i32, i64, isize,
	u8, u16, u32, u64, usize,
	f32, f64,
);
impl_for!(SizeLesser<128> =>
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
	f32, f64,
);

#[cfg(target_pointer_width = "16")]
impl_for!(SizeLesser<16> => isize, usize);

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
impl_for!(SizeLesser<32> => isize, usize);

#[cfg(test)]
mod tests {
	use static_assertions::*;

	use super::*;
	use crate::num::{
		Floating,
		Integral,
		Signed,
		Unsigned,
	};

	assert_impl_all!(bool: Fundamental);
	assert_impl_all!(char: Fundamental);

	assert_impl_all!(i8: Integral, Signed, SizeEquals<8>);
	assert_impl_all!(i16: Integral, Signed, SizeEquals<16>);
	assert_impl_all!(i32: Integral, Signed, SizeEquals<32>);
	assert_impl_all!(i64: Integral, Signed, SizeEquals<64>);
	assert_impl_all!(i128: Integral, Signed, SizeEquals<128>);
	assert_impl_all!(isize: Integral, Signed);

	assert_impl_all!(u8: Integral, Unsigned, SizeEquals<8>);
	assert_impl_all!(u16: Integral, Unsigned, SizeEquals<16>);
	assert_impl_all!(u32: Integral, Unsigned, SizeEquals<32>);
	assert_impl_all!(u64: Integral, Unsigned, SizeEquals<64>);
	assert_impl_all!(u128: Integral, Unsigned, SizeEquals<128>);
	assert_impl_all!(usize: Integral, Unsigned);

	assert_impl_all!(f32: Floating, SizeEquals<32>);
	assert_impl_all!(f64: Floating, SizeEquals<64>);
}
