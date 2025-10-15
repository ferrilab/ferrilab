use core::{
	any,
	error,
	fmt,
	marker::PhantomData,
};

use super::Zeroable;

/// You supplied `0` to `NonZero`. Don’t do that.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZeroValueError<T>
where T: Zeroable
{
	_ty: PhantomData<T>,
}

impl<T> ZeroValueError<T>
where T: Zeroable
{
	/// Creates a ZVE.
	pub const fn new() -> Self {
		Self { _ty: PhantomData }
	}
}

impl<T> error::Error for ZeroValueError<T> where T: Zeroable {}

impl<T> fmt::Display for ZeroValueError<T>
where T: Zeroable
{
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"provided ({ty})0 to NonZero<{ty}>",
			ty = any::type_name::<T>(),
		)
	}
}
