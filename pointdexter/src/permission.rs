#![expect(dead_code)]

use crate::Permission;

/// Private implementation of the `Permission` trait system.
pub trait Impl: 'static {
	/// The raw-pointer type family corresponding to this `Permission`
	/// implementation. Will be either `*const T` or `*mut T`.
	type Ptr<T>: Copy
	where T: ?Sized;

	/// The raw-reference type family corresponding to this `Permission`
	/// implementation. Will be either `&T` or `&mut T`.
	type Ref<'a, T>
	where T: 'a + ?Sized;

	/// Points from a derived `Permission` to its originating type.
	type Original: Impl;

	/// Either `"const"` or `"mut"`.`
	const NAME: &'static str;

	/// Whether this implementation has write privileges.
	const IS_MUT: bool;
}

#[doc = include_str!("../doc/struct.Shared.md")]
pub struct Shared {
	inner: (),
}

#[doc = include_str!("../doc/struct.Unique.md")]
pub struct Unique {
	inner: (),
}

impl Shared {
	pub(crate) const fn new() -> Self {
		Self { inner: () }
	}
}

impl Unique {
	pub(crate) const fn new() -> Self {
		Self { inner: () }
	}
}

impl Impl for Shared {
	type Original = Self;
	type Ptr<T>
		= *const T
	where T: ?Sized;
	type Ref<'a, T>
		= &'a T
	where T: 'a + ?Sized;

	const IS_MUT: bool = false;
	const NAME: &'static str = "const";
}

impl Impl for Unique {
	type Original = Self;
	type Ptr<T>
		= *mut T
	where T: ?Sized;
	type Ref<'a, T>
		= &'a mut T
	where T: 'a + ?Sized;

	const IS_MUT: bool = true;
	const NAME: &'static str = "mut";
}

/// Any `Permission` can be degraded by prepending a `Shared` marker. Such
/// tuples act like plain `Shared`.
impl<P> Impl for (Shared, P)
where P: Impl
{
	type Original = P::Original;
	type Ptr<T>
		= <Shared as Impl>::Ptr<T>
	where T: ?Sized;
	type Ref<'a, T>
		= <Shared as Impl>::Ref<'a, T>
	where T: 'a + ?Sized;

	const IS_MUT: bool = Shared::IS_MUT;
	const NAME: &'static str = Shared::NAME;
}

impl Permission for Shared {}
impl Permission for Unique {}

/// Any `Permission` can be degraded by prepending a `Shared` marker. Such
/// tuples act like plain `Shared`.
impl<P> Permission for (Shared, P) where P: Permission {}
