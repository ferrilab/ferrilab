use core::{
	error,
	fmt,
	ops,
	str::FromStr,
};

use super::{
	NonZero,
	ZeroValueError,
	Zeroable,
};
use crate::num::Signed;

impl<T> Clone for NonZero<T>
where T: Zeroable
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

/// Implements From<NonZero<Left>> for each NonZero<Right>.
macro_rules! widen_into {
	( $($src:ty => $($dst:ty),+ $(,)?);+ $(;)? ) => { $( $(
		impl From<NonZero<$src>> for NonZero<$dst> {
			#[inline(always)]
			fn from(src: NonZero<$src>) -> Self {
				Self { inner: src.inner.into() }
			}
		}
	)+ )+ };
}

widen_into! {
	i8  => i16, i32, i64, i128, isize;
	i16 =>      i32, i64, i128, isize;
	i32 =>           i64, i128;
	i64 =>                i128;
	u8  => i16, i32, i64, i128, isize, u16, u32, u64, u128, usize;
	u16 =>      i32, i64, i128,             u32, u64, u128, usize;
	u32 =>           i64, i128,                  u64, u128;
	u64 =>                i128,                       u128;
}

/// Implements `TryFrom<Left> for Left` and `TryFrom<NonZero<Left>> for-each
/// NonZero<Right>`.
macro_rules! impl_tryfrom {
	( $($src:ty $(=> $($dst:ty),+ $(,)?)?);+ $(;)? ) => { $(
		impl TryFrom<$src> for NonZero<$src> {
			type Error = ZeroValueError<$src>;

			#[inline]
			fn try_from(value: $src) -> Result<Self, Self::Error> {
				Self::new(value).ok_or_else(ZeroValueError::new)
			}
		}

		$($(
			impl TryFrom<NonZero<$src>> for NonZero<$dst> {
				type Error = ZeroValueError<$dst>;

				#[inline]
				fn try_from(value: NonZero<$src>) -> Result<Self, ZeroValueError<$dst>> {
					Self::new(value.get() as $dst).ok_or_else(ZeroValueError::new)
				}
			}
		)+)?
	)+ };
}

impl_tryfrom! {
	i8    =>                                 u8, u16, u32, u64, u128, usize;
	i16   => i8,                             u8, u16, u32, u64, u128, usize;
	i32   => i8, i16,                 isize, u8, u16, u32, u64, u128, usize;
	i64   => i8, i16, i32,            isize, u8, u16, u32, u64, u128, usize;
	i128  => i8, i16, i32, i64,       isize, u8, u16, u32, u64, u128, usize;
	isize => i8, i16, i32, i64, i128,        u8, u16, u32, u64, u128, usize;
	u8    => i8;
	u16   => i8, i16,                 isize, u8;
	u32   => i8, i16, i32,            isize, u8, u16,                 usize;
	u64   => i8, i16, i32, i64,       isize, u8, u16, u32,            usize;
	u128  => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64,       usize;
	usize => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128;
}

impl<T> fmt::Binary for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::Binary,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Binary::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::Display for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::Display,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::LowerExp for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::LowerExp,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::LowerExp::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::LowerHex for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::LowerHex,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::LowerHex::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::Octal for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::Octal,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Octal::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::UpperExp for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::UpperExp,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::UpperExp::fmt(&self.inner, fmt)
	}
}

impl<T> fmt::UpperHex for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: fmt::UpperHex,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::UpperHex::fmt(&self.inner, fmt)
	}
}

impl<T> Copy for NonZero<T> where T: Zeroable {}

unsafe impl<T> Send for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: Send,
{
}

unsafe impl<T> Sync for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: Sync,
{
}

impl<T> ops::BitOr<T> for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero: ops::BitOr<T, Output = <T as Zeroable>::NonZero>,
{
	type Output = Self;

	#[inline(always)]
	fn bitor(self, rhs: T) -> Self {
		Self::from_nonzero(self.inner | rhs)
	}
}

impl<T> ops::BitOr for NonZero<T>
where
	T: Zeroable,
	<T as Zeroable>::NonZero:
		ops::BitOr<<T as Zeroable>::NonZero, Output = <T as Zeroable>::NonZero>,
{
	type Output = Self;

	#[inline(always)]
	fn bitor(self, rhs: Self) -> Self {
		Self::from_nonzero(self.inner | rhs.inner)
	}
}

impl<T> ops::BitOrAssign<T> for NonZero<T>
where
	Self: ops::BitOr<T, Output = Self>,
	T: Zeroable,
{
	#[inline]
	fn bitor_assign(&mut self, rhs: T) {
		*self = *self | rhs;
	}
}

impl<T> ops::BitOrAssign for NonZero<T>
where
	T: Zeroable,
	NonZero<T>: ops::BitOr<Output = NonZero<T>>,
{
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

macro_rules! nzdiv {
	( $($t:ty),+ $(,)? ) => { $(
		impl ops::Div<NonZero<$t>> for $t {
			type Output = $t;

			#[inline]
			fn div(self, denom: NonZero<$t>) -> Self {
				self / denom.inner
			}
		}

		impl ops::DivAssign<NonZero<$t>> for $t {
			#[inline]
			fn div_assign(&mut self, denom: NonZero<$t>) {
				*self /= denom.inner;
			}
		}

		impl ops::Rem<NonZero<$t>> for $t {
			type Output = $t;

			#[inline]
			fn rem(self, denom: NonZero<$t>) -> Self {
				self % denom.inner
			}
		}

		impl ops::RemAssign<NonZero<$t>> for $t {
			#[inline]
			fn rem_assign(&mut self, denom: NonZero<$t>) {
				*self %= denom.inner;
			}
		}
	)+ };
}

nzdiv!(u8, u16, u32, u64, u128, usize);

impl<T> ops::Neg for NonZero<T>
where
	T: Zeroable + Signed,
	<T as Zeroable>::NonZero: ops::Neg<Output = <T as Zeroable>::NonZero>,
{
	type Output = Self;

	#[inline(always)]
	fn neg(self) -> Self {
		Self::from_nonzero(-self.inner)
	}
}

#[derive(Debug)]
pub enum ParseNonZeroIntError<T>
where
	T: Zeroable + FromStr,
	<T as FromStr>::Err: error::Error,
{
	NotIntegerString(<T as FromStr>::Err),
	ZeroValue(ZeroValueError<T>),
}

impl<T> From<ZeroValueError<T>> for ParseNonZeroIntError<T>
where
	T: Zeroable + FromStr,
	<T as FromStr>::Err: error::Error,
{
	#[inline(always)]
	fn from(src: ZeroValueError<T>) -> Self {
		Self::ZeroValue(src)
	}
}

impl<T> fmt::Display for ParseNonZeroIntError<T>
where
	T: Zeroable + FromStr,
	<T as FromStr>::Err: error::Error,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match self {
			| Self::NotIntegerString(err) => fmt::Display::fmt(err, fmt),
			| Self::ZeroValue(err) => fmt::Display::fmt(err, fmt),
		}
	}
}

impl<T> error::Error for ParseNonZeroIntError<T>
where
	T: Zeroable + FromStr,
	<T as FromStr>::Err: error::Error,
{
	#[inline]
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			| Self::NotIntegerString(err) => Some(err),
			| Self::ZeroValue(err) => Some(err),
		}
	}
}

impl<T> FromStr for NonZero<T>
where
	T: Zeroable + FromStr,
	<T as FromStr>::Err: error::Error,
{
	type Err = ParseNonZeroIntError<T>;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		let val =
			T::from_str(src).map_err(ParseNonZeroIntError::NotIntegerString)?;
		Self::new(val)
			.ok_or_else(ZeroValueError::<T>::new)
			.map_err(Into::into)
	}
}

#[cfg(test)]
mod tests {
	use static_assertions::*;

	use super::*;

	macro_rules! polyassert_integers {
		( $($tr:ty),+ $(,)? ) => {
			assert_impl_all!(NonZero<i8>: $($tr),+);
			assert_impl_all!(NonZero<i16>: $($tr),+);
			assert_impl_all!(NonZero<i32>: $($tr),+);
			assert_impl_all!(NonZero<i64>: $($tr),+);
			assert_impl_all!(NonZero<i128>: $($tr),+);
			assert_impl_all!(NonZero<isize>: $($tr),+);

			assert_impl_all!(NonZero<u8>: $($tr),+);
			assert_impl_all!(NonZero<u16>: $($tr),+);
			assert_impl_all!(NonZero<u32>: $($tr),+);
			assert_impl_all!(NonZero<u64>: $($tr),+);
			assert_impl_all!(NonZero<u128>: $($tr),+);
			assert_impl_all!(NonZero<usize>: $($tr),+);
		};
	}

	polyassert_integers!(
		fmt::Display,
		Copy,
		Sized,
		Unpin,
		ops::BitOr,
		ops::BitOrAssign
	);
}
