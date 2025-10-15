//! Non-public implementation details to support pointers.

use core::any;

use super::{
	NonUniqueError,
	Shared,
	Unique,
};

/// Internal implementation details for the [`Permission`] trait. Almost all
/// behavior is held in the associated `Ptr` and `Ref` types; only
/// functionality which the type system cannot express in them is held here.
///
/// [`Permission`]: super::Permission
pub trait Impl: 'static {
	/// The raw pointer fundamental, either `*const T` or `*mut T`.
	type Ptr<T>: RawPtr<T>
	where T: ?Sized;

	/// The raw reference fundamental, either `&'a T` or `&'a mut T`.
	type Ref<'a, T>: RawRef<'a, T>
	where T: 'a + ?Sized;

	type Base: Impl;

	/// Either `"*const"` or `"*mut"`; used for debug printing.
	const DEBUG_PREFIX: &'static str;

	/// Used with [`into_const`](Impl::into_const) to move pointers through
	/// different permission types without changing the underlying pointer
	/// or its provenance.
	fn from_const<T>(ptr: *const T) -> Self::Ptr<T>
	where T: ?Sized;

	/// Used with [`from_const`](Impl::from_const) to move pointers through
	/// different permission types without changing the underlying pointer
	/// or its provenance.
	fn into_const<T>(ptr: Self::Ptr<T>) -> *const T
	where T: ?Sized;

	/// Attempts to promote the pointer to `*mut T`. Implementors are required
	/// to respect the [Stacked Borrows][0] rules as laid out by the Miri team.
	///
	/// # Returns
	///
	/// - `Ok`: a `*mut T`
	/// - `Err`: a marker indicating that the orginial pointer did not have
	///   write permissions.
	///
	/// [0]: https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md
	fn try_into_mut<T>(ptr: Self::Ptr<T>) -> Result<*mut T, NonUniqueError<T>>
	where T: ?Sized;

	/// Converts the pointer to a reference without doing any validity checks.
	unsafe fn ptr_to_ref<'a, T>(ptr: Self::Ptr<T>) -> Self::Ref<'a, T>
	where T: 'a + ?Sized;

	/// Converts the reference to a pointer.
	fn ref_to_ptr<'a, T>(r: Self::Ref<'a, T>) -> Self::Ptr<T>
	where T: 'a + ?Sized;

	/// Changes the pointee type. This can remove, but not add, metadata on a
	/// pointer.
	fn cast<T, U>(ptr: Self::Ptr<T>) -> Self::Ptr<U>
	where
		T: ?Sized,
		U: Sized;

	/// Changes the permission, but not the pointee-type, on a pointer. This  is
	/// fallible because it will refuse to promote a `*const T` to a  `*mut T`
	/// if it cannot prove that the `*const T` pointer had been  previously
	/// demoted from a `*mut T`.
	///
	/// # Returns
	///
	/// - `Ok`: The original pointer with the new permission.
	/// - `Err`: A marker indicating that the new permission could not be
	///   constructed. This only occurs for
	/// `Shared::cast_permission::<_, Unique>()`; casting to `Q = Shared` is
	/// always valid, and `<(Shared, Unique)>::cast_permission::<_, Unique>()`
	/// is also valid.
	fn cast_permission<T, Q>(
		ptr: Self::Ptr<T>,
	) -> Result<Q::Ptr<T>, NonUniqueError<T>>
	where
		T: ?Sized,
		Q: Impl;
}

impl Impl for Shared {
	type Base = Self;
	type Ptr<T>
		= *const T
	where T: ?Sized;
	type Ref<'a, T>
		= &'a T
	where T: 'a + ?Sized;

	const DEBUG_PREFIX: &'static str = "*const";

	#[inline(always)]
	fn from_const<T>(ptr: *const T) -> Self::Ptr<T>
	where T: ?Sized {
		ptr
	}

	#[inline(always)]
	fn into_const<T>(ptr: Self::Ptr<T>) -> *const T
	where T: ?Sized {
		ptr
	}

	#[inline(always)]
	fn try_into_mut<T>(_: Self::Ptr<T>) -> Result<*mut T, NonUniqueError<T>>
	where T: ?Sized {
		Err(NonUniqueError::new())
	}

	#[inline(always)]
	unsafe fn ptr_to_ref<'a, T>(ptr: Self::Ptr<T>) -> Self::Ref<'a, T>
	where T: 'a + ?Sized {
		unsafe { &*ptr }
	}

	#[inline(always)]
	fn ref_to_ptr<'a, T>(r: Self::Ref<'a, T>) -> Self::Ptr<T>
	where T: 'a + ?Sized {
		r as *const T
	}

	#[inline(always)]
	fn cast<T, U>(ptr: Self::Ptr<T>) -> Self::Ptr<U>
	where
		T: ?Sized,
		U: Sized,
	{
		ptr.cast::<U>()
	}

	#[inline]
	fn cast_permission<T, Q>(
		ptr: Self::Ptr<T>,
	) -> Result<Q::Ptr<T>, NonUniqueError<T>>
	where
		T: ?Sized,
		Q: Impl,
	{
		if any::TypeId::of::<Q::Base>() == any::TypeId::of::<Shared>() {
			Ok(Q::from_const(Self::into_const(ptr)))
		}
		else {
			Err(NonUniqueError::new())
		}
	}
}

impl Impl for Unique {
	type Base = Self;
	type Ptr<T>
		= *mut T
	where T: ?Sized;
	type Ref<'a, T>
		= &'a mut T
	where T: 'a + ?Sized;

	const DEBUG_PREFIX: &'static str = "*mut";

	#[inline(always)]
	fn from_const<T>(ptr: *const T) -> Self::Ptr<T>
	where T: ?Sized {
		ptr.cast_mut()
	}

	#[inline(always)]
	fn into_const<T>(ptr: Self::Ptr<T>) -> *const T
	where T: ?Sized {
		<Self::Ptr<T>>::cast_const(ptr)
	}

	#[inline(always)]
	fn try_into_mut<T>(ptr: Self::Ptr<T>) -> Result<*mut T, NonUniqueError<T>>
	where T: ?Sized {
		Ok(ptr)
	}

	#[inline(always)]
	unsafe fn ptr_to_ref<'a, T>(ptr: Self::Ptr<T>) -> Self::Ref<'a, T>
	where T: 'a + ?Sized {
		unsafe { &mut *ptr }
	}

	#[inline(always)]
	fn ref_to_ptr<'a, T>(r: Self::Ref<'a, T>) -> Self::Ptr<T>
	where T: 'a + ?Sized {
		r as *mut T
	}

	#[inline(always)]
	fn cast<T, U>(ptr: Self::Ptr<T>) -> Self::Ptr<U>
	where
		T: ?Sized,
		U: Sized,
	{
		ptr.cast::<U>()
	}

	#[inline(always)]
	fn cast_permission<T, Q>(
		ptr: Self::Ptr<T>,
	) -> Result<Q::Ptr<T>, NonUniqueError<T>>
	where
		T: ?Sized,
		Q: Impl,
	{
		Ok(Q::from_const(Self::into_const(ptr)))
	}
}

/// This allows history-stacking: `(Shared, Unique)` denotes an
/// originally-unique pointer that has been degraded to shared, but could be
/// restored in the future. Because this tuple is itself a `Permission`
/// implementor, `(Shared, (Shared, Unique))` and all recursive extensions
/// continue to work correctly, and can all be unwound back to their root
/// `Shared` or `Unique` permission.
impl<P> Impl for (Shared, P)
where P: Impl
{
	type Base = <P as Impl>::Base;
	type Ptr<T>
		= <Shared as Impl>::Ptr<T>
	where T: ?Sized;
	type Ref<'a, T>
		= <Shared as Impl>::Ref<'a, T>
	where T: 'a + ?Sized;

	const DEBUG_PREFIX: &'static str = Shared::DEBUG_PREFIX;

	#[inline(always)]
	fn from_const<T>(ptr: *const T) -> Self::Ptr<T>
	where T: ?Sized {
		ptr
	}

	#[inline(always)]
	fn into_const<T>(ptr: Self::Ptr<T>) -> *const T
	where T: ?Sized {
		ptr
	}

	#[inline(always)]
	fn try_into_mut<T>(ptr: Self::Ptr<T>) -> Result<*mut T, NonUniqueError<T>>
	where T: ?Sized {
		<P as Impl>::try_into_mut(P::from_const(ptr))
	}

	#[inline(always)]
	unsafe fn ptr_to_ref<'a, T>(ptr: Self::Ptr<T>) -> Self::Ref<'a, T>
	where T: 'a + ?Sized {
		unsafe { Shared::ptr_to_ref(ptr) }
	}

	#[inline(always)]
	fn ref_to_ptr<'a, T>(r: Self::Ref<'a, T>) -> Self::Ptr<T>
	where T: 'a + ?Sized {
		r as *const T
	}

	#[inline(always)]
	fn cast<T, U>(ptr: Self::Ptr<T>) -> Self::Ptr<U>
	where
		T: ?Sized,
		U: Sized,
	{
		Shared::cast::<T, U>(ptr)
	}

	#[inline]
	fn cast_permission<T, Q>(
		ptr: Self::Ptr<T>,
	) -> Result<Q::Ptr<T>, NonUniqueError<T>>
	where
		T: ?Sized,
		Q: Impl,
	{
		P::cast_permission::<T, Q>(P::from_const(Self::into_const(ptr)))
	}
}

/// Forwards trait methods to the underlying fundamental method.
macro_rules! trait_items {
	(rawptr @ $(
		$(@ $unsafety:ident)? fn $func:ident $(<$($typarm:ident),+>)? (self $(, $argn:ident : $argty:ty)*) $(-> $ret:ty)? $(where ($($tybnd:tt)+))?;
	)+ ) => { $(
		#[inline(always)]
		$($unsafety)? fn $func $(<$($typarm),+>)? (self $(, $argn : $argty)*) $(-> $ret)? $(where $($tybnd)+)? {
			$($unsafety)? { <Self>::$func$(::<$($typarm),*>)? (self $(, $argn)*) }
		}
	)+ };
}

/// Makes raw-pointer APIs accessible to the `funty` type system.
///
/// All methods here are either methods on the raw-pointer fundamentals or
/// functions in the `core::ptr` module.
pub trait RawPtr<T>: Copy
where T: ?Sized
{
	fn is_null(self) -> bool;

	fn addr(self) -> usize;

	fn expose_provenance(self) -> usize;

	fn with_exposed_provenance(addr: usize) -> Self
	where T: Sized;

	fn without_provenance(addr: usize) -> Self
	where T: Sized;

	fn with_addr(self, addr: usize) -> Self;

	fn map_addr(self, func: impl FnOnce(usize) -> usize) -> Self;

	unsafe fn offset(self, count: isize) -> Self
	where T: Sized;

	unsafe fn byte_offset(self, count: isize) -> Self;

	fn wrapping_offset(self, count: isize) -> Self
	where T: Sized;

	fn wrapping_byte_offset(self, count: isize) -> Self;

	unsafe fn offset_from(self, other: impl RawPtr<T>) -> isize
	where T: Sized;

	unsafe fn byte_offset_from<U>(self, other: impl RawPtr<U>) -> isize
	where U: ?Sized;

	#[cfg(feature = "rust_187")]
	unsafe fn offset_from_unsigned(self, other: impl RawPtr<T>) -> usize
	where T: Sized;

	#[cfg(feature = "rust_187")]
	unsafe fn byte_offset_from_unsigned<U>(self, other: impl RawPtr<U>) -> usize
	where U: ?Sized;

	unsafe fn add(self, count: usize) -> Self
	where T: Sized;

	unsafe fn byte_add(self, count: usize) -> Self
	where T: Sized;

	unsafe fn sub(self, count: usize) -> Self
	where T: Sized;

	unsafe fn byte_sub(self, count: usize) -> Self
	where T: Sized;

	fn wrapping_add(self, count: usize) -> Self
	where T: Sized;

	fn wrapping_byte_add(self, count: usize) -> Self
	where T: Sized;

	fn wrapping_sub(self, count: usize) -> Self
	where T: Sized;

	fn wrapping_byte_sub(self, count: usize) -> Self
	where T: Sized;

	unsafe fn read(self) -> T
	where T: Sized;

	unsafe fn read_volatile(self) -> T
	where T: Sized;

	unsafe fn read_unaligned(self) -> T
	where T: Sized;

	unsafe fn copy_to(self, dest: *mut T, count: usize)
	where T: Sized;

	unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)
	where T: Sized;

	fn align_offset(self, align: usize) -> usize
	where T: Sized;

	fn is_aligned(self) -> bool
	where T: Sized;
}

impl<T> RawPtr<T> for *const T
where T: ?Sized
{
	trait_items! { rawptr @
		fn is_null(self) -> bool;
		fn addr(self) -> usize;
		fn expose_provenance(self) -> usize;
		fn with_addr(self, addr: usize) -> Self;
		fn map_addr(self, func: impl FnOnce(usize) -> usize) -> Self;
		@unsafe fn offset(self, count: isize) -> Self where (T: Sized);
		@unsafe fn byte_offset(self, count: isize) -> Self;
		fn wrapping_offset(self, count: isize) -> Self where (T: Sized);
		fn wrapping_byte_offset(self, count: isize) -> Self;
		@unsafe fn add(self, count: usize) -> Self where (T: Sized);
		@unsafe fn byte_add(self, count: usize) -> Self where (T: Sized);
		@unsafe fn sub(self, count: usize) -> Self where (T: Sized);
		@unsafe fn byte_sub(self, count: usize) -> Self where (T: Sized);
		fn wrapping_add(self, count: usize) -> Self where (T: Sized);
		fn wrapping_byte_add(self, count: usize) -> Self where (T: Sized);
		fn wrapping_sub(self, count: usize) -> Self where (T: Sized);
		fn wrapping_byte_sub(self, count: usize) -> Self where (T: Sized);
		@unsafe fn read(self) -> T where (T: Sized);
		@unsafe fn read_volatile(self) -> T where (T: Sized);
		@unsafe fn read_unaligned(self) -> T where (T: Sized);
		@unsafe fn copy_to(self, dest: *mut T, count: usize) where (T: Sized);
		@unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize) where (T: Sized);
		fn align_offset(self, align: usize) -> usize where (T: Sized);
		fn is_aligned(self) -> bool where (T: Sized);
	}

	#[inline(always)]
	fn with_exposed_provenance(addr: usize) -> Self
	where T: Sized {
		core::ptr::with_exposed_provenance(addr)
	}

	#[inline(always)]
	fn without_provenance(addr: usize) -> Self
	where T: Sized {
		core::ptr::without_provenance(addr)
	}

	#[inline(always)]
	unsafe fn offset_from(self, origin: impl RawPtr<T>) -> isize
	where T: Sized {
		unsafe { self.offset_from(self.with_addr(origin.addr())) }
	}

	#[inline(always)]
	unsafe fn byte_offset_from<U>(self, origin: impl RawPtr<U>) -> isize
	where U: ?Sized {
		unsafe { self.byte_offset_from(self.with_addr(origin.addr())) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	unsafe fn offset_from_unsigned(self, origin: impl RawPtr<T>) -> usize
	where T: Sized {
		unsafe { self.offset_from_unsigned(self.with_addr(origin.addr())) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	unsafe fn byte_offset_from_unsigned<U>(
		self,
		origin: impl RawPtr<U>,
	) -> usize
	where
		U: ?Sized,
	{
		unsafe { self.byte_offset_from_unsigned(self.with_addr(origin.addr())) }
	}
}

impl<T> RawPtr<T> for *mut T
where T: ?Sized
{
	trait_items! { rawptr @
		fn is_null(self) -> bool;
		fn addr(self) -> usize;
		fn expose_provenance(self) -> usize;
		fn with_addr(self, addr: usize) -> Self;
		fn map_addr(self, func: impl FnOnce(usize) -> usize) -> Self;
		@unsafe fn offset(self, count: isize) -> Self where (T: Sized);
		@unsafe fn byte_offset(self, count: isize) -> Self;
		fn wrapping_offset(self, count: isize) -> Self where (T: Sized);
		fn wrapping_byte_offset(self, count: isize) -> Self;
		@unsafe fn add(self, count: usize) -> Self where (T: Sized);
		@unsafe fn byte_add(self, count: usize) -> Self where (T: Sized);
		@unsafe fn sub(self, count: usize) -> Self where (T: Sized);
		@unsafe fn byte_sub(self, count: usize) -> Self where (T: Sized);
		fn wrapping_add(self, count: usize) -> Self where (T: Sized);
		fn wrapping_byte_add(self, count: usize) -> Self where (T: Sized);
		fn wrapping_sub(self, count: usize) -> Self where (T: Sized);
		fn wrapping_byte_sub(self, count: usize) -> Self where (T: Sized);
		@unsafe fn read(self) -> T where (T: Sized);
		@unsafe fn read_volatile(self) -> T where (T: Sized);
		@unsafe fn read_unaligned(self) -> T where (T: Sized);
		@unsafe fn copy_to(self, dest: *mut T, count: usize) where (T: Sized);
		@unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize) where (T: Sized);
		fn align_offset(self, align: usize) -> usize where (T: Sized);
		fn is_aligned(self) -> bool where (T: Sized);
	}

	/// Conjures a pointer from nothing but a bare memory address, relying on
	/// the program having previously exposed the provenance at that address.
	///
	/// See [Exposed Provenance][0] in the standard library docs.
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
	#[inline(always)]
	fn with_exposed_provenance(addr: usize) -> Self
	where T: Sized {
		core::ptr::with_exposed_provenance_mut(addr)
	}

	/// Conjures a pointer from nothing other than a bare memory address.
	///
	/// See [Exposed Provenance][0] in the standard library docs.
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
	#[inline(always)]
	fn without_provenance(addr: usize) -> Self
	where T: Sized {
		core::ptr::without_provenance_mut(addr)
	}

	#[inline(always)]
	unsafe fn offset_from(self, origin: impl RawPtr<T>) -> isize
	where T: Sized {
		unsafe { RawPtr::<T>::offset_from(self.cast_const(), origin) }
	}

	#[inline(always)]
	unsafe fn byte_offset_from<U>(self, origin: impl RawPtr<U>) -> isize
	where U: ?Sized {
		unsafe { RawPtr::<T>::byte_offset_from::<U>(self.cast_const(), origin) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	unsafe fn offset_from_unsigned(self, origin: impl RawPtr<T>) -> usize
	where T: Sized {
		unsafe { RawPtr::<T>::offset_from_unsigned(self.cast_const(), origin) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	unsafe fn byte_offset_from_unsigned<U>(
		self,
		origin: impl RawPtr<U>,
	) -> usize
	where
		U: ?Sized,
	{
		unsafe {
			RawPtr::<T>::byte_offset_from_unsigned::<U>(
				self.cast_const(),
				origin,
			)
		}
	}
}

/// Wraps `&T` and `&mut T` in the `funty` type system.
pub trait RawRef<'a, T>
where T: 'a + ?Sized
{
	#[cfg(feature = "rust_189")]
	fn cast_const(self) -> &'a T;
}

impl<'a, T> RawRef<'a, T> for &'a T
where T: 'a + ?Sized
{
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	fn cast_const(self) -> &'a T {
		self
	}
}

impl<'a, T> RawRef<'a, T> for &'a mut T
where T: 'a + ?Sized
{
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	fn cast_const(self) -> &'a T {
		&*self
	}
}
