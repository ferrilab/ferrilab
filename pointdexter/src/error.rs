use core::{
	any,
	error::Error,
	fmt,
	marker::PhantomData,
};

use crate::Permission;

/// Emitted when a `Shared` provenance tries to upgrade to `Unique` without a
/// pedigree.
pub struct NonUniqueError<T>
where T: ?Sized
{
	_ty: PhantomData<*const T>,
}

impl<T> NonUniqueError<T>
where T: ?Sized
{
	pub(crate) const fn new() -> Self {
		Self { _ty: PhantomData }
	}
}

impl<T> fmt::Debug for NonUniqueError<T>
where T: ?Sized
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "NonUniqueError<{}>", any::type_name::<T>(),)
	}
}

impl<T> fmt::Display for NonUniqueError<T>
where T: ?Sized
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"tried to promote a *const {0} that was never derived from a *mut \
			 {0}",
			any::type_name::<T>(),
		)
	}
}

impl<T> Error for NonUniqueError<T> where T: ?Sized {}

/// Emitted when a null-pointer is provided to an API that requires non-null
/// pointers.
pub struct NullPointerError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	_type: PhantomData<*const T>,
	_perm: PhantomData<P>,
}

impl<T, P> NullPointerError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	pub(crate) const fn new() -> Self {
		Self {
			_type: PhantomData,
			_perm: PhantomData,
		}
	}
}

impl<T, P> fmt::Debug for NullPointerError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"NullPointerError<*{} {}>",
			P::NAME,
			any::type_name::<T>(),
		)
	}
}

impl<T, P> fmt::Display for NullPointerError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"provided a null address to a non-null `*{} {}`",
			P::NAME,
			any::type_name::<T>(),
		)
	}
}

impl<T, P> Error for NullPointerError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}
