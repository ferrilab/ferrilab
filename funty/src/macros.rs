//! Codegen helpers

/// Produces a doc-string that forwards to a standard library item.
macro_rules! doc_url {
	(const $t:ty => $c:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/primitive.",
			stringify!($t),
			".html#associatedconstant.",
			stringify!($c),
			">."
		)
	};
	(mod const $t:ty => $c:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/",
			stringify!($t),
			"/consts/constant.",
			stringify!($c),
			".html>."
		)
	};
	(fn $t:ty => $f:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/primitive.",
			stringify!($t),
			".html#method.",
			stringify!($f),
			">.",
		)
	};
}

/// Produces a trait definition whose items are linked to their corresponding
/// `i32` API.
macro_rules! new_trait {
	// The outer trait declaration, including all its requirements.
	//
	// Note: due to quirks of Rust's macro rules, trait requirements cannot use
	// their standard `+` separator, and HRTBs need decoration.
	(
		$(#[$attr:meta])*
		$name:ident
		$(: $req:path $(, $(@for<$lt:lifetime>)? $reqs:path)*)?
		{
			$($rest:tt)*
		}
	) => {
		$(#[$attr])*
		pub trait $name
		$(: $req $(+ $(for<$lt>)? $reqs)*)?
		{
			$($rest)*
		}
	};

	// Associated types.
	($($(#[$attr:meta])* type $name:ident $(: $req:path)?;)+) => { $(
		$(#[$attr])* type $name$(: $req)?;
	)+ };

	// Constants that do not exist in the standard library.
	($($(#[$attr:meta])* @new const $name:ident : $type:ty;)+ ) => { $(
		$(#[$attr])* const $name: $type;
	)+ };
	// Freestanding constants.
	($basis:ident @ $(
		$(#[$attr:meta])*
		mod const $name:ident: $type:ty;
	)+) => { $(
		$(#[$attr:meta])*
		#[doc = doc_url!(mod const $basis => $name)]
		const $name: $type;
	)+ };
	// Associated constants.
	($basis:ident @ $(
		$(#[$attr:meta])*
		const $name:ident: $type:ty;
	)+) => { $(
		$(#[$attr])*
		#[doc = doc_url!(const $basis => $name)]
		const $name: $type;
	)+ };

	// Functions.
	($basis:ident @ $(
		$(#[$attr:meta])*
		$(@ $unsafety:ident)? fn $name:ident
		($($args:tt)*)
		$(-> $ret:ty)?;
	)+) => { $(
		#[doc = doc_url!(fn $basis => $name)]
		$(#[$attr])*
		$($unsafety)? fn $name ($($args)*) $(-> $ret)?;
	)+ };
}

/// Creates new wrapper items that forward to the corresponding items in the
/// standard library.
///
/// This macro can accept multiple item declarations in a single invocation, but
/// all of its contents *must* be of the same form. Different forms require
/// different invocations.
macro_rules! items {
	// Associated constants.
	($typ:ty => $(const $name:ident: $t:ty;)+) => { $(
		#[doc = doc_url!(const $typ => $name)]
		const $name: $t = <$typ>::$name;
	)+ };
	// Freestanding constants.
	($typ:ident => $(mod const $name:ident: $t:ty;)+) => { $(
		#[doc = doc_url!(mod const $typ => $name)]
		const $name: $t = core::$typ::consts::$name;
	)+ };

	// Methods that take `self` by value.
	($typ:ty => $(
		$(#[$attr:meta])*
		$(@ $unsafety:ident)? fn $name:ident
		(self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+ ) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		$($unsafety)? fn $name(self$(, $arg: $t)*) $(-> $ret)? {
			$($unsafety)? { Self::$name(self$(, $arg)*) }
		}
	)+ };
	// Methods that take `&self` by reference.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		(&self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name(&self$(, $arg: $t)*) $(-> $ret)? {
			<Self>::$name(&self$(, $arg )*)
		}
	)+ };
	// Methods that take `&mut self` by mutable reference.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		(&mut self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name(&mut self$(, $arg: $t)*) $(-> $ret)? {
			<Self>::$name(&mut self$(, $arg)*)
		}
	)+ };
	// Functions that do not take `self` at all.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		($($arg:ident: $t:ty),* $(,)?)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name($($arg: $t),*) $(-> $ret)? {
			<Self>::$name($($arg),*)
		}
	)+ };
}

macro_rules! impl_for {
	(Fundamental => $($t:ty => $is_zero:expr),+ $(,)?) => { $(
		impl crate::seal::Sealed for $t {}

		impl Fundamental for $t {
			#[doc = doc_url!(const $t => BITS)]
			const BITS: u32 = core::mem::size_of::<$t>() as u32 * 8;
			#[doc = doc_url!(const $t => MIN)]
			const MIN: Self = <$t>::MIN;
			#[doc = doc_url!(const $t => MAX)]
			const MAX: Self = <$t>::MAX;

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
				@unsafe fn unchecked_add(self, rhs: Self) -> Self;
				@unsafe fn unchecked_sub(self, rhs: Self) -> Self;
				@unsafe fn unchecked_mul(self, rhs: Self) -> Self;
				#[cfg(feature = "rust_187")]
				fn unbounded_shl(self, rhs: u32) -> Self;
				#[cfg(feature = "rust_187")]
				fn unbounded_shr(self, rhs: u32) -> Self;
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
				fn isqrt(self) -> Self;
				fn div_euclid(self, rhs: Self) -> Self;
				fn rem_euclid(self, rhs: Self) -> Self;
				fn ilog(self, base: Self) -> u32;
				fn ilog2(self) -> u32;
				fn ilog10(self) -> u32;

				#[cfg(feature = "rust_187")]
				fn midpoint(self, rhs: Self) -> Self;
			}
		}
	)+ };
	(Signed => $($t:ty),+ $(,)?) => { $(
		impl Signed for $t {
			items! { $t =>
				#[cfg(feature = "rust_187")]
				fn cast_unsigned(self) -> Self::Unsigned;

				fn checked_add_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;
				fn checked_sub_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;
				fn checked_abs(self) -> Option<Self>;
				fn checked_isqrt(self) -> Option<Self>;

				fn saturating_add_unsigned(self, rhs: Self::Unsigned) -> Self;
				fn saturating_sub_unsigned(self, rhs: Self::Unsigned) -> Self;
				fn saturating_neg(self) -> Self;
				fn saturating_abs(self) -> Self;

				fn wrapping_add_unsigned(self, rhs: Self::Unsigned) -> Self;
				fn wrapping_sub_unsigned(self, rhs: Self::Unsigned) -> Self;
				fn wrapping_abs(self) -> Self;

				fn unsigned_abs(self) -> Self::Unsigned;

				fn overflowing_add_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);
				fn overflowing_sub_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);
				fn overflowing_abs(self) -> (Self, bool);

				fn checked_ilog(self, base: Self) -> Option<u32>;
				fn checked_ilog2(self) -> Option<u32>;
				fn checked_ilog10(self) -> Option<u32>;

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
				#[cfg(feature = "rust_187")]
				fn cast_signed(self) -> Self::Signed;

				fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self>;
				#[cfg(feature = "rust_190")]
				fn checked_sub_signed(self, rhs: Self::Signed) -> Option<Self>;
				fn saturating_add_signed(self, rhs: Self::Signed) -> Self;
				#[cfg(feature = "rust_190")]
				fn saturating_sub_signed(self, rhs: Self::Signed) -> Self;
				fn wrapping_add_signed(self, rhs: Self::Signed) -> Self;
				#[cfg(feature = "rust_190")]
				fn wrapping_sub_signed(self, rhs: Self::Signed) -> Self;

				#[cfg(feature = "rust_187")]
				fn is_multiple_of(self, rhs: Self) -> bool;
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
				#[cfg(feature = "std")] fn round_ties_even(self) -> Self;
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
				fn next_up(self) -> Self;
				fn next_down(self) -> Self;
				fn recip(self) -> Self;
				fn to_degrees(self) -> Self;
				fn to_radians(self) -> Self;
				fn max(self, other: Self) -> Self;
				fn min(self, other: Self) -> Self;
				fn midpoint(self, other: Self) -> Self;
				fn to_bits(self) -> Self::Raw;
				fn clamp(self, min: Self, max: Self) -> Self;
			}
			items! { $t =>
				fn total_cmp(&self, other: &Self) -> cmp::Ordering;
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
