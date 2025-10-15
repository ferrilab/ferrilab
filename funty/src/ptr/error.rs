use core::{
	any,
	cmp,
	error,
	fmt,
	marker::PhantomData,
	mem,
};

use super::{
	Permission,
	Shared,
};

/// Emitted when a context requires that a pointer be well-aligned for its
/// pointee type, but the pointer value is not.
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MisalignedError<T>
where T: Sized
{
	ptr: *const T,
}

impl<T> MisalignedError<T> {
	pub(crate) const fn new(ptr: *const T) -> Self {
		Self { ptr }
	}
}

impl<T> Clone for MisalignedError<T> {
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> error::Error for MisalignedError<T> {}

impl<T> fmt::Debug for MisalignedError<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "MisalignedError<{}>", any::type_name::<T>())?;
		fmt.debug_tuple("").field(&self.ptr).finish()
	}
}

impl<T> fmt::Display for MisalignedError<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"type {} requires alignment {}, but address {:p} is only aligned \
			 to {}",
			any::type_name::<T>(),
			mem::align_of::<T>(),
			self.ptr,
			1 << self.ptr.addr().trailing_zeros(),
		)
	}
}

impl<T> Copy for MisalignedError<T> {}

/// Emitted when a null pointer is provided to an API that does not accept it.
pub struct NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	_type: PhantomData<*const T>,
	_perm: PhantomData<P>,
}

impl<T, P> NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Creates a new null-pointer error instance.
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			_type: PhantomData,
			_perm: PhantomData,
		}
	}

	/// Discards the `Permission` data.
	#[inline(always)]
	pub const fn make_const(self) -> NonNullError<T, Shared> {
		NonNullError::new()
	}
}

impl<T, P> Clone for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, P> Eq for NonNullError<T, P>
where
	T: 'static + ?Sized,
	P: 'static + Permission,
{
}

impl<T, P> Ord for NonNullError<T, P>
where
	T: 'static + ?Sized,
	P: 'static + Permission,
{
	#[inline(always)]
	fn cmp(&self, _: &Self) -> cmp::Ordering {
		cmp::Ordering::Equal
	}
}

impl<T1, T2, P1, P2> PartialEq<NonNullError<T2, P2>> for NonNullError<T1, P1>
where
	T1: 'static + ?Sized,
	T2: 'static + ?Sized,
	P1: 'static + Permission,
	P2: 'static + Permission,
{
	#[inline]
	fn eq(&self, _: &NonNullError<T2, P2>) -> bool {
		any::TypeId::of::<Self>() == any::TypeId::of::<NonNullError<T2, P2>>()
	}
}

impl<T1, T2, P1, P2> PartialOrd<NonNullError<T2, P2>> for NonNullError<T1, P1>
where
	T1: 'static + ?Sized,
	T2: 'static + ?Sized,
	P1: 'static + Permission,
	P2: 'static + Permission,
{
	#[inline]
	fn partial_cmp(&self, _: &NonNullError<T2, P2>) -> Option<cmp::Ordering> {
		any::TypeId::of::<Self>()
			.partial_cmp(&any::TypeId::of::<NonNullError<T2, P2>>())
	}
}

impl<T, P> fmt::Debug for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"NullPointerError<{}, {}>",
			any::type_name::<T>(),
			any::type_name::<P>(),
		)
	}
}

impl<T, P> fmt::Display for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"provided a null address to a non-null `{} {}`",
			P::DEBUG_PREFIX,
			any::type_name::<T>(),
		)
	}
}

impl<T, P> Copy for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

unsafe impl<T, P> Send for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

unsafe impl<T, P> Sync for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

impl<T, P> error::Error for NonNullError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

/// Emitted when a `*const` pointer tries to promote to `*mut` without proof of
/// origin.
pub struct NonUniqueError<T>
where T: ?Sized
{
	_type: PhantomData<*const T>,
}

impl<T> NonUniqueError<T>
where T: ?Sized
{
	/// Creates a new non-unique error instance.
	pub const fn new() -> Self {
		Self { _type: PhantomData }
	}
}

impl<T> Clone for NonUniqueError<T>
where T: ?Sized
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> fmt::Debug for NonUniqueError<T>
where T: ?Sized
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "NonUniqueError<{}>", any::type_name::<T>())
	}
}

impl<T> fmt::Display for NonUniqueError<T>
where T: ?Sized
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let t = any::type_name::<T>();
		write!(
			fmt,
			"tried to cast *const {t} to *mut {t} without good provenance"
		)
	}
}

impl<T> error::Error for NonUniqueError<T> where T: ?Sized {}

impl<T> Copy for NonUniqueError<T> where T: ?Sized {}
