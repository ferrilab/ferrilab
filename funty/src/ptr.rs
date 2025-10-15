#![doc = include_str!("../doc/ptr.md")]

use core::{
	any,
	cmp,
	fmt,
	hash,
	mem,
};

mod details;
mod error;
mod nonnull;

pub use self::{
	details::WrapFunty,
	error::{
		MisalignedError,
		NonNullError,
		NonUniqueError,
	},
	nonnull::NonNullPointer,
};

#[doc = include_str!("../doc/ptr/trait.Permission.md")]
pub trait Permission: details::Impl {
	/// Forwards a type-hidden [`Reference`] into a callback as `&T`.
	///
	/// The callback receives a re-borrow of the original referent, and is
	/// forbidden from leaking its argument reference out through the return
	/// value.
	fn do_with_ref<'a, T, R>(
		r: &Self::Ref<'a, T>,
		func: impl FnOnce(&T) -> R,
	) -> R
	where
		T: 'a + ?Sized;

	/// Attempts to forward a type-hidden [`Reference`] into a callback as `&mut
	/// T`.
	///
	/// If the invocant permission is not [`Unique`], then this function exits
	/// with an error. If it is `Unique`, then the callback is invoked
	/// with a re-borrow of the original referent, and is forbidden from leaking
	/// its argument reference out through the return value.
	///
	/// # Details
	///
	/// This reborrowing is necessary because `&mut T` references are not
	/// `Copy`, and therefore cannot be passed through function calls by value.
	/// Literal `&mut T` instances cause implicit `&mut *val` re-borrowing
	/// de-references, but type-obscured `Reference<'a, T, Unique>` instances do
	/// not. Therefore, this function enables client code to temporarily view a
	/// `Reference` as its true `&mut T` typing without incurring a destruction
	/// of the _reference_ value (as distinct from the pointed-to referent
	/// value).
	fn try_with_mut_ref<'a, T, R>(
		r: &mut Self::Ref<'a, T>,
		func: impl FnOnce(&mut T) -> R,
	) -> Result<R, NonUniqueError<T>>
	where
		T: 'a + ?Sized;
}

/// Equivalent to `*const` pointers or `&` references.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Shared;

/// Equivalent to `*mut` pointers or `&mut` references.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unique;

impl<P> Permission for P
where P: details::Impl
{
	#[inline(always)]
	fn do_with_ref<'a, T, R>(
		r: &Self::Ref<'a, T>,
		func: impl FnOnce(&T) -> R,
	) -> R
	where
		T: 'a + ?Sized,
	{
		<P as details::Impl>::impl_do_with_ref(r, func)
	}

	#[inline(always)]
	fn try_with_mut_ref<'a, T, R>(
		r: &mut Self::Ref<'a, T>,
		func: impl FnOnce(&mut T) -> R,
	) -> Result<R, NonUniqueError<T>>
	where
		T: 'a + ?Sized,
	{
		<P as details::Impl>::impl_try_with_mut_ref(r, func)
	}
}

#[repr(transparent)]
#[doc = include_str!("../doc/ptr/struct.Pointer.md")]
pub struct Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// The underlying raw pointer. This is `*const` for `Shared` or
	/// `(Shared, P)` and `*mut` for `Unique`.
	ptr: P::Ptr<T>,
}

impl<T> Pointer<T, Shared>
where T: ?Sized
{
	/// Wraps a raw const-pointer.
	#[inline(always)]
	pub const fn from_const(ptr: *const T) -> Self {
		Self { ptr }
	}

	/// Unwraps the pointer into its underlying `*const T` primitive.
	#[inline(always)]
	pub const fn into_raw(self) -> *const T {
		self.ptr
	}
}

impl<T> Pointer<T, Unique>
where T: ?Sized
{
	/// Wraps a raw mut-pointer.
	#[inline(always)]
	pub const fn from_mut(ptr: *mut T) -> Self {
		Self { ptr }
	}

	/// Unwraps the pointer into its underlying `*mut T` primitive.
	#[inline(always)]
	pub const fn into_raw(self) -> *mut T {
		self.ptr
	}
}

impl<T, P> Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Create a new `Pointer` with any permission from a raw const pointer.
	///
	/// # Safety
	///
	/// This is a module-internal API, and can only be used in immediate
	/// conjunction with `Self::into_raw_const()` of the **same** `Permission`
	/// type, or a type that is known to be in a valid relation to the original
	/// permission. It is not marked as `unsafe` because it is private to this
	/// module.
	#[inline(always)]
	const fn new_from_const(ptr: *const T) -> Self {
		Self {
			ptr: unsafe { mem::transmute_copy(&ptr) },
		}
	}

	/// Unwrap a `Pointer` into a raw const pointer.
	///
	/// This is only useful for accessing the `const fn` methods on `<*T>`. It
	/// is distinct from `.into_raw()` to indicate that this is an internal API
	/// and should be paired with `Self::new_from_const`, which is similarly
	/// private.
	#[inline(always)]
	const fn into_raw_const(self) -> *const T {
		unsafe { mem::transmute_copy(&self.ptr) }
	}

	/// Reversibly degrades a pointer to `Shared` permissions, by pushing a
	/// `Shared` to the front of its permission history stack.
	///
	/// There is no difference in behavior between the `Shared` and
	/// `(Shared, Shared)` permissions. This is primarily useful for blunting
	/// `Unique`-permissioned pointers for a limited period, and allowing them
	/// to recover later.
	#[inline(always)]
	pub const fn make_shared(self) -> Pointer<T, (Shared, P)> {
		Pointer::new_from_const(self.into_raw_const())
	}

	/// Inverse of [`.make_shared()`](Self::make_shared). Restores the pointer
	/// to its original `Shared` or `Unique` permission.
	///
	/// Because `Pointer<_, (Shared, _)>` has the same API as
	/// `Pointer<_, Shared>`, this method is primarily useful only for cutting
	/// down on monomorphization bloat from repeated `.make_shared()`
	/// invocations.
	#[inline(always)]
	pub const fn make_unshared(self) -> Pointer<T, P::Base> {
		Pointer::new_from_const(self.into_raw_const())
	}

	/// Similar to [`.make_shared()`](Self::make_shared), except the cast is
	/// irreversible.
	#[inline(always)]
	pub const fn make_const(self) -> Pointer<T, Shared> {
		Pointer::from_const(self.into_raw_const())
	}

	/// Converts a pointer to the `Unique` permission, **only** if `P` has a
	/// `Unique` base permission.
	///
	/// Use this instead of [`.cast_mut()`](Self::cast_mut). That function
	/// panics when the base permission is `Shared`.
	///
	/// # Returns
	///
	/// - `Ok`: a wrapped `*mut T` pointer
	/// - `Err`: a marker indicating that the pointer did not have any write
	///   permission in its history.
	#[inline]
	pub fn make_mut(self) -> Result<Pointer<T, Unique>, NonUniqueError<T>> {
		P::try_into_mut_ptr(self.ptr).map(Into::into)
	}
}

/// Mirrors of the pointer fundamental API that work on all pointers.
impl<T, P> Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Tests if the pointer is null.
	///
	/// Note that unsized types may have many possible null pointers, as only
	/// the raw data pointer is considered, not their length, vtable, etc.
	/// Therefore, two pointers that are null may still not compare equal to
	/// each other.
	///
	/// # Original
	///
	/// [`<*T>::is_null`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null)
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::prelude::*;
	/// let s: &str = "Follow the rabbit";
	/// let ptr: Pointer<str, Shared> = s.into();
	/// assert!(!ptr.is_null());
	/// ```
	#[inline(always)]
	pub const fn is_null(self) -> bool {
		self.into_raw_const().is_null()
	}

	/// Casts to a pointer of another type.
	///
	/// # Original
	///
	/// [`<*T>::cast`](https://doc.rust-lang.org/std/primitive.pointer.html#method.cast)
	///
	/// # Notes
	///
	/// This can only discard, but not conjure, pointer metadata. Therefore, a
	/// slice or dyn-trait pointer can be cast to an object pointer, but an
	/// object pointer cannot be cast to a slice- or dyn-trait- pointer.
	#[inline(always)]
	pub const fn cast<U>(self) -> Pointer<U, P>
	where U: Sized {
		Pointer::new_from_const(self.into_raw_const().cast::<U>())
	}

	/// Changes constness without changing the type.
	///
	/// This is a bit safer than `as` because it wouldn’t silently change the
	/// type if the code is refactored.
	///
	/// # Original
	///
	/// [`<*T>::cast_const`](https://doc.rust-lang.org/std/primitive.pointer.html#method.cast_const)
	///
	/// # Alternatives
	///
	/// - [`.make_shared()`](Pointer::make_shared) produces a `Pointer<T,
	///   (Shared, P)>`, which acts like a const-pointer, but can later pop its
	///   permission stack and restore its original permission.
	/// - [`.make_const()`](Pointer::make_const) produces a `Pointer<T,
	///   Shared>`. It discards its permission stack entirely.
	#[inline(always)]
	#[deprecated = "prefer `make_shared`; its pointer can be safely made \
	                mutable again"]
	pub const fn cast_const(self) -> *const T {
		self.into_raw_const()
	}

	/// Changes constness without changing the type.
	///
	/// This is a bit safer than `as` because it wouldn’t silently change the
	/// type if the code is refactored.
	///
	/// # Original
	///
	/// [`<*T>::cast_mut`](https://doc.rust-lang.org/std/primitive.pointer.html#method.cast_mut)
	///
	/// # Behavior Differences
	///
	/// This **panics** if the root permission is `Shared`. This type **does
	/// not** allow conjuring uniqueness permissions from air; uniqueness
	/// **must** be recorded from the beginning of the type’s usage.
	///
	/// # Examples
	///
	/// This code succeeds, because the initial memory grant has mutability:
	///
	/// ```rust
	/// # use funty::prelude::*;
	/// let mut data: u8 = 5;
	/// let ptr_m: Pointer<u8, Unique> = (&mut data).into();
	/// let ptr_c = ptr_m.make_shared();
	/// let raw_m: *mut u8 = ptr_c.cast_mut();
	/// ```
	///
	/// This code fails, because the initial memory grant lacks mutability:
	///
	/// ```rust,should_panic
	/// # use funty::prelude::*;
	/// let data: u8 = 5;
	/// let ptr_c: Pointer<u8, Shared> = (&data).into();
	/// let raw_m: *mut u8 = ptr_c.cast_mut();
	/// ```
	#[inline]
	#[track_caller]
	#[deprecated = "use `make_mut` instead"]
	pub const fn cast_mut(self) -> *mut T {
		match self.make_mut() {
			| Ok(ptr) => ptr.into_raw(),
			| Err(_) => {
				panic!("funty refuses to conjure a *mut T from a *const T")
			},
		}
	}

	/// Gets the “address” portion of the pointer.
	///
	/// This is similar to `self.cast_const() as usize`, except that the
	/// [provenance][0] of the pointer is discarded and not [exposed][1]. This
	/// means that casting the returned address back to a pointer yields a
	/// [pointer without provenance], which is undefined behavior to
	/// dereference. To properly restore the lost information and obtain a
	/// dereferenceable pointer, use [`with_addr`][2] or [`map_addr`][3].
	///
	/// If using those APIs is not possible because there is no way to preserve
	/// a pointer with the required provenance, then Strict Provenance might not
	/// be for you. Use pointer-integer casts or [`expose_provenance`][4] and
	/// [`with_exposed_provenance`][5] instead. However, note that this makes
	/// your code less portable and less amenable to tools that check for
	/// compliance with the Rust memory model.
	///
	/// On most platforms this will produce a value with the same bytes as the
	/// original pointer, because all the bytes are dedicated to describing the
	/// address. Platforms which need to store additional information in the
	/// pointer may perform a change of representation to produce a value
	/// containing only the address portion of the pointer. What that means is
	/// up to the platform to define.
	///
	/// This is a [Strict Provenance][6] API.
	///
	/// # Original
	///
	/// [`<*T>::addr`](https://doc.rust-lang.org/std/primitive.pointer.html#method.addr)
	///
	/// [0]: https://doc.rust-lang.org/std/ptr/index.html#provenance
	/// [1]: https://doc.rust-lang.org/std/ptr/index.html#exposed-provenance
	/// [2]: without_provenance
	/// [3]: Pointer::with_addr
	/// [4]: Pointer::map_addr
	/// [5]: Pointer::expose_provenance
	/// [6]: with_exposed_provenance
	#[inline(always)]
	pub fn addr(self) -> usize {
		self.into_raw_const().addr()
	}

	/// Exposes the [“provenance”][0] part of the pointer for future use in
	/// [`with_exposed_provenance`] and returns the “address” portion.
	///
	/// This is equivalent to `self as usize`, which semantically discards
	/// provenance information. Furthermore, this (like the `as` cast) has the
	/// implicit side-effect of marking the provenance as ‘exposed’, so on
	/// platforms that support it you can later call [`with_exposed_provenance`]
	/// to reconstitute the original pointer including its provenance.
	///
	/// Due to its inherent ambiguity, [`with_exposed_provenance`] may not be
	/// supported by tools that help you to stay conformant with the Rust memory
	/// model. It is recommended to use [Strict Provenance][1] APIs such as
	/// [`with_addr`][2] wherever possible, in which case [`addr`][3] should be
	/// used instead of `expose_provenance`.
	///
	/// On most platforms this will produce a value with the same bytes as the
	/// original pointer, because all the bytes are dedicated to describing the
	/// address. Platforms which need to store additional information in the
	/// pointer may not support this operation, since the ‘expose’ side-effect
	/// which is required for [`with_exposed_provenance`] to work is typically
	/// not available.
	///
	/// This is an [Exposed Provenance][4] API.
	///
	/// # Original
	///
	/// [`<*T>::expose_provenance`](https://doc.rust-lang.org/std/primitive.pointer.html#method.expose_provenance)
	///
	/// [0]: https://doc.rust-lang.org/std/ptr/index.html#provenance
	/// [1]: https://doc.rust-lang.org/std/ptr/index.html#strict-provenance
	/// [2]: Pointer::with_addr
	/// [3]: Pointer::addr
	/// [4]: https://doc.rust-lang.org/std/ptr/index.html#exposed-provenance
	#[inline(always)]
	pub fn expose_provenance(self) -> usize {
		self.into_raw_const().expose_provenance()
	}

	/// Creates a new pointer with the given address and the [provenance][0] of
	/// `self`.
	///
	/// This is similar to a `addr as *const T` cast, but copies the
	/// _provenance_ of `self` to the new pointer. This avoids the inherent
	/// ambiguity of the unary cast.
	///
	/// This is equivalent to using [`wrapping_offset`][1] to offset `self` to
	/// the given address, and therefore has all the same capabilities and
	/// restrictions.
	///
	/// This is a [Strict Provenance][2] API.
	///
	/// # Original
	///
	/// [`<*T>::with_addr`](https://doc.rust-lang.org/std/primitive.pointer.html#method.with_addr)
	///
	/// [0]: https://doc.rust-lang.org/std/ptr/index.html#provenance
	/// [1]: Pointer::wrapping_offset
	/// [2]: https://doc.rust-lang.org/std/ptr/index.html#strict-provenance
	#[inline(always)]
	pub fn with_addr(self, addr: usize) -> Self {
		Self::new_from_const(self.into_raw_const().with_addr(addr))
	}

	/// Creates a new pointer by mapping `self`’s address to a new one,
	/// preserving the provenance of self.
	///
	/// This is a convenience for [`with_addr`][0]; see that method for details.
	///
	/// This is a [Strict Provenance][1] API.
	///
	/// # Original
	///
	/// [`<*T>::map_addr`](https://doc.rust-lang.org/std/primitive.pointer.html#method.map_addr)
	///
	/// [0]: Pointer::with_addr
	/// [1]: https://doc.rust-lang.org/std/ptr/index.html#strict-provenance
	#[inline(always)]
	pub fn map_addr(self, func: impl FnOnce(usize) -> usize) -> Self {
		Self::new_from_const(self.into_raw_const().map_addr(func))
	}

	/// Promotes this pointer to a reference with the same permission. Fails if
	/// the pointer is null.
	///
	/// # Original
	///
	/// [`<*T>::as_ref`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref)
	///
	/// # API Differencess
	///
	/// Returns `Result` instead of `Option`. The error variant is a ZST with a
	/// useful message upon `.unwrap()`.
	///
	/// # Safety
	///
	/// When calling this method, you have to ensure that _either_ the pointer
	/// is null _or_ the pointer is [convertible to a reference][0].
	///
	/// # Null-unchecked version
	///
	/// You can convert this back to a raw pointer and dereference that. To
	/// read, use [`.cast_const()`][1]; to write, use
	/// [`.cast_mut().unwrap()`][2]. The unwrap on `.cast_mut()` is for ensuring
	/// write permission, and cannot be bypassed with `funty` APIs.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::prelude::*;
	/// let data = 10u8;
	/// let ptr = Pointer::from(&data);
	///
	/// unsafe {
	///   if let Ok(val) = ptr.as_reference() {
	///     assert_eq!(val, &10);
	///   }
	/// }
	/// ```
	///
	/// ```rust
	/// use funty::prelude::*;
	/// let data = 10u8;
	/// let ptr = Pointer::from(&data);
	///
	/// unsafe {
	///   let val = &*ptr.cast_const();
	///   assert_eq!(val, &10);
	/// }
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/std/ptr/index.html#pointer-to-reference-conversion
	/// [1]: Pointer::cast_const
	/// [2]: Pointer::cast_mut
	pub const unsafe fn as_reference<'a>(
		self,
	) -> Result<Reference<'a, T, P>, NonNullError<T, P>>
	where T: 'a {
		match NonNullPointer::<T, P>::from_pointer(self) {
			| Ok(nnp) => Ok(unsafe { nnp.as_reference() }),
			| Err(e) => Err(e),
		}
	}

	/// Adds a signed offset in bytes to a pointer.
	///
	/// `count` is in units of **bytes**.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`offset`][0] on it. See that method for documentation and safety
	/// requirements.
	///
	/// For non-`Sized` pointees this operation changes only the data pointer,
	/// leaving the metadata untouched.
	///
	/// # Original
	///
	/// [`<*T>::byte_offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_offset)
	///
	/// [0]: Pointer::offset
	#[inline(always)]
	pub const unsafe fn byte_offset(self, count: isize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().byte_offset(count) })
	}

	/// Adds a signed offset in bytes to a pointer using wrapping arithmetic.
	///
	/// `count` is in units of **bytes**.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`wrapping_offset`][0] on it. See that method for documentation.
	///
	/// For non-`Sized` pointees this operation changes only the data pointer,
	/// leaving the metadata untouched.
	///
	/// # Original
	///
	/// [`<*T>::wrapping_byte_offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_byte_offset)
	///
	/// [0]: Self::wrapping_offset
	#[inline(always)]
	pub const fn wrapping_byte_offset(self, count: isize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_byte_offset(count))
	}

	/// Calculates the distance between two pointers within the same allocation.
	/// The returned value is in units of **bytes**.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`offset_from`][0] on it. See that method for documentation and safety
	/// requirements.
	///
	/// For non-`Sized` pointees this operation considers only the data
	/// pointers, ignoring the metadata.
	///
	/// # Original
	///
	/// [`<*T>::byte_offset_from`](https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_offset_from)
	///
	/// # Safety
	///
	/// `self` and `origin` **must** be derived from the same allocation and
	/// have the same provenance.
	///
	/// [0]: Self::offset_from
	pub const unsafe fn byte_offset_from<U, Q>(
		self,
		origin: Pointer<U, Q>,
	) -> isize
	where
		U: ?Sized,
		Q: Permission,
	{
		unsafe {
			self.into_raw_const()
				.byte_offset_from(origin.into_raw_const())
		}
	}

	/// Calculates the distance between two pointers within the same allocation,
	/// _where it’s known that `self` is equal to or greater than `origin`_. The
	/// returned value is in units of T: the distance in bytes is divided by
	/// `size_of::<T>()`.
	///
	/// This computes the same value that [`offset_from`][0] would compute, but
	/// with the added precondition that the offset is guaranteed to be
	/// non-negative. This method is equivalent to
	/// `usize::try_from(self.offset_from(origin)).unwrap_unchecked()`, but it
	/// provides slightly more information to the optimizer, which can sometimes
	/// allow it to optimize slightly better with some backends.
	///
	///This method can be thought of as recovering the `count` that was passed
	/// to [`add`][1] (or, with the parameters in the other order, to
	/// [`sub`][2]). The following are all equivalent, assuming that their
	/// safety preconditions are met:
	///
	/// ```rust,ignore
	/// ptr.offset_from_unsigned(origin) == count
	/// origin.add(count) == ptr
	/// ptr.sub(count) == origin
	/// ```
	///
	/// # Safety
	///
	/// - The distance between the pointers must be non-negative (`self >=
	///   origin`)
	/// - _All_ the safety conditions of [`offset_from`][0] apply to this method
	///   as well; see it for the full details.
	///
	/// Importantly, despite the return type of this method being able to
	/// represent a larger offset, it’s still not permitted to pass pointers
	/// which differ by more than `isize::MAX` bytes. As such, the result of
	/// this method will always be less than or equal to `isize::MAX as usize`.
	///
	/// # Panics
	///
	/// This function panics if T is a Zero-Sized Type (“ZST”).
	///
	/// # Examples
	///
	/// ```rust
	/// # use funty::ptr::*;
	/// let a = [0; 5];
	/// let ptr1 = Pointer::<i32, Shared>::from_const(&a[1]);
	/// let ptr2 = Pointer::<i32, Shared>::from_const(&a[3]);
	/// unsafe {
	///     assert_eq!(ptr2.offset_from_unsigned(ptr1), 2);
	///     assert_eq!(ptr1.add(2), ptr2);
	///     assert_eq!(ptr2.sub(2), ptr1);
	///     assert_eq!(ptr2.offset_from_unsigned(ptr2), 0);
	/// }
	///
	/// // This would be incorrect, as the pointers are not correctly ordered:
	/// // ptr1.offset_from_unsigned(ptr2)
	/// ```
	///
	/// [0]: Self::offset_from
	/// [1]: Self::add
	/// [2]: Self::sub
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub const unsafe fn byte_offset_from_unsigned<U, Q>(
		self,
		origin: Pointer<U, Q>,
	) -> usize
	where
		U: ?Sized,
		Q: Permission,
	{
		unsafe {
			self.into_raw_const()
				.byte_offset_from_unsigned(origin.into_raw_const())
		}
	}
}

/// Mirrors of the pointer fundamental API that require a `Sized` pointee.
impl<T, P> Pointer<T, P>
where
	T: Sized,
	P: Permission,
{
	/// Adds a signed offset to a pointer.
	///
	/// `count` is in units of T; e.g., a count of 3 represents a pointer offset
	/// of `3 * size_of::<T>()` bytes.
	///
	/// # Original
	///
	/// [`<*T>::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset)
	///
	/// # Safety
	///
	/// If any of the following conditions are violated, the result is Undefined
	/// Behavior:
	///
	/// - The offset in bytes, `count * size_of::<T>()`, computed on
	///   mathematical integers (without “wrapping around”), must fit in an
	///   `isize`.
	///
	/// - If the computed offset is non-zero, then `self` must be [derived
	///   from][0] a pointer to some [allocation][1], and the entire memory
	///   range between `self` and the result must be in bounds of that
	///   allocation. In particular, this range must not “wrap around” the edge
	///   of the address space. Note that “range” here refers to a half-open
	///   range as usual in Rust, i.e., `self..result` for non-negative offsets
	///   and `result..self` for negative offsets.
	///
	/// Allocations can never be larger than `isize::MAX` bytes, so if the
	/// computed offset stays in bounds of the allocation, it is guaranteed to
	/// satisfy the first requirement. This implies, for instance, that
	/// `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always safe.
	///
	/// Consider using [`wrapping_offset`][2] instead if these constraints are
	/// difficult to satisfy. The only advantage of this method is that it
	/// enables more aggressive compiler optimizations.
	///
	/// [0]: https://doc.rust-lang.org/std/ptr/index.html#provenance
	/// [1]: https://doc.rust-lang.org/std/ptr/index.html#allocation
	/// [2]: Pointer::wrapping_offset
	#[inline(always)]
	pub const unsafe fn offset(self, count: isize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().offset(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/struct.Pointer.fn.wrapping_offset.md")]
	pub const fn wrapping_offset(self, count: isize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_offset(count))
	}

	#[inline(always)]
	pub const unsafe fn offset_from<Q>(self, origin: Pointer<T, Q>) -> isize
	where Q: Permission {
		unsafe { self.into_raw_const().offset_from(origin.into_raw_const()) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub const unsafe fn offset_from_unsigned<Q>(
		self,
		origin: Pointer<T, Q>,
	) -> usize
	where
		Q: Permission,
	{
		unsafe {
			self.into_raw_const()
				.offset_from_unsigned(origin.into_raw_const())
		}
	}

	#[inline(always)]
	pub const unsafe fn add(self, count: usize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().add(count) })
	}

	#[inline(always)]
	pub const unsafe fn byte_add(self, count: usize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().byte_add(count) })
	}

	#[inline(always)]
	pub const unsafe fn sub(self, count: usize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().sub(count) })
	}

	#[inline(always)]
	pub const unsafe fn byte_sub(self, count: usize) -> Self {
		Self::new_from_const(unsafe { self.into_raw_const().byte_sub(count) })
	}

	#[inline(always)]
	pub const fn wrapping_add(self, count: usize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_add(count))
	}

	#[inline(always)]
	pub const fn wrapping_byte_add(self, count: usize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_byte_add(count))
	}

	#[inline(always)]
	pub const fn wrapping_sub(self, count: usize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_sub(count))
	}

	#[inline(always)]
	pub const fn wrapping_byte_sub(self, count: usize) -> Self {
		Self::new_from_const(self.into_raw_const().wrapping_byte_sub(count))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.read.md")]
	pub const unsafe fn read(self) -> T {
		unsafe { self.into_raw_const().read() }
	}

	#[inline(always)]
	pub unsafe fn read_volatile(self) -> T {
		unsafe { self.into_raw_const().read_volatile() }
	}

	#[inline(always)]
	pub const unsafe fn read_unaligned(self) -> T {
		unsafe { self.into_raw_const().read_unaligned() }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.copy.md")]
	pub const unsafe fn copy_to(self, dest: Pointer<T, Unique>, count: usize) {
		unsafe { dest.copy_from(self.make_const(), count) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.copy_nonoverlapping.md")]
	pub const unsafe fn copy_to_nonoverlapping(
		self,
		dest: Pointer<T, Unique>,
		count: usize,
	) {
		unsafe { dest.copy_from_nonoverlapping(self.make_const(), count) }
	}

	#[inline(always)]
	pub fn align_offset(self, align: usize) -> usize {
		self.into_raw_const().align_offset(align)
	}

	#[inline(always)]
	pub fn is_aligned(self) -> bool {
		self.into_raw_const().is_aligned()
	}
}

impl<T> Pointer<T, Unique> {
	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.copy.md")]
	pub const unsafe fn copy_from<Q>(self, src: Pointer<T, Q>, count: usize)
	where Q: Permission {
		unsafe { self.into_raw().copy_from(src.into_raw_const(), count) }
	}

	/// Copies `count * size_of::<T>()` bytes from `src` to `self`. The source
	/// and destination may _not_ overlap.
	///
	/// NOTE this has the _opposite_ argument order to
	/// [`copy_to_nonoverlapping`].
	///
	/// See [`copy_to_nonoverlapping`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`<*mut T>::copy_from_nonoverlapping`][0]
	///
	/// [0]: https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from_nonoverlapping
	/// [`copy_to_nonoverlapping`]: Self::copy_to_nonoverlapping
	#[inline(always)]
	pub const unsafe fn copy_from_nonoverlapping<Q>(
		self,
		src: Pointer<T, Q>,
		count: usize,
	) where
		Q: Permission,
	{
		unsafe {
			self.into_raw()
				.copy_from_nonoverlapping(src.into_raw_const(), count)
		}
	}

	#[doc = include_str!("../doc/ptr/fn.write.md")]
	pub const unsafe fn write(self, val: T) {
		unsafe {
			self.into_raw().write(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.write_bytes.md")]
	pub const unsafe fn write_bytes(self, val: u8, count: usize) {
		unsafe {
			self.into_raw().write_bytes(val, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.write_volatile.md")]
	pub unsafe fn write_volatile(self, val: T) {
		unsafe {
			self.into_raw().write_volatile(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.write_unaligned.md")]
	pub const unsafe fn write_unaligned(self, val: T) {
		unsafe {
			self.into_raw().write_unaligned(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.replace.md")]
	pub const unsafe fn replace(self, src: T) -> T {
		unsafe { self.into_raw().replace(src) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/ptr/fn.swap.md")]
	pub const unsafe fn swap(self, with: Self) {
		unsafe {
			self.into_raw().swap(with.into_raw());
		}
	}
}

impl<T, P> Pointer<[T], P>
where
	T: Sized,
	P: Permission,
{
	#[inline(always)]
	pub const fn slice_from_raw_parts(ptr: Pointer<T, P>, len: usize) -> Self {
		Self::new_from_const(core::ptr::slice_from_raw_parts(
			ptr.into_raw_const(),
			len,
		))
	}

	/// Returns the length of a raw slice.
	///
	/// The returned value is the number of elements, not the number of bytes.
	///
	/// This function is safe, even when the raw slice cannot be cast to a slice
	/// reference because the pointer is null or unaligned.
	///
	/// # Original
	///
	/// [`<*const [T]>::len`](https://doc.rust-lang.org/std/primitive.pointer.html#method.len)
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let slice = Pointer::slice_from_raw_parts(Pointer::<i8, Shared>::null(), 3);
	/// assert_eq!(slice.len(), 3);
	/// ```
	#[inline(always)]
	pub const fn len(self) -> usize {
		self.into_raw_const().len()
	}

	/// Tests if the raw slice has a length of 0.
	///
	/// # Original
	///
	/// [`<*const [T]>`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_empty)
	#[inline(always)]
	pub const fn is_empty(self) -> bool {
		self.into_raw_const().is_empty()
	}
}

impl<T, P> fmt::Debug for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		if fmt.alternate() {
			write!(fmt, "({} {})", P::DEBUG_PREFIX, any::type_name::<T>())?;
		}
		fmt::Debug::fmt(&self.into_raw_const(), fmt)
	}
}

impl<T, P> fmt::Pointer for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Pointer::fmt(&self.into_raw_const(), fmt)
	}
}

impl<T, P> Clone for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, P> Eq for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

impl<T, P> Ord for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.into_raw_const()
			.cast::<()>()
			.cmp(&other.into_raw_const().cast::<()>())
	}
}

impl<T1, T2, P1, P2> PartialEq<Pointer<T2, P2>> for Pointer<T1, P1>
where
	T1: ?Sized,
	T2: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline]
	fn eq(&self, other: &Pointer<T2, P2>) -> bool {
		core::ptr::eq(
			self.into_raw_const().cast::<()>(),
			other.into_raw_const().cast::<()>(),
		)
	}
}

impl<T1, T2, P1, P2> PartialOrd<Pointer<T2, P2>> for Pointer<T1, P1>
where
	T1: ?Sized,
	T2: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline]
	fn partial_cmp(&self, other: &Pointer<T2, P2>) -> Option<cmp::Ordering> {
		self.into_raw_const()
			.cast::<()>()
			.partial_cmp(&other.into_raw_const().cast::<()>())
	}
}

impl<T> From<*const T> for Pointer<T, Shared>
where T: ?Sized
{
	#[inline(always)]
	fn from(ptr: *const T) -> Self {
		Self::from_const(ptr)
	}
}

impl<T> From<&T> for Pointer<T, Shared>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &T) -> Self {
		Self::from_const(src)
	}
}

impl<T> From<*mut T> for Pointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	fn from(ptr: *mut T) -> Self {
		Self::from_mut(ptr)
	}
}

impl<T> From<&mut T> for Pointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &mut T) -> Self {
		Self::from_mut(src)
	}
}

impl<T, P> hash::Hash for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	fn hash<H>(&self, hasher: &mut H)
	where H: hash::Hasher {
		self.into_raw_const().hash(hasher);
	}
}

impl<T, P> Copy for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

#[doc = include_str!("../doc/ptr/type.Reference.md")]
pub type Reference<'a, T, P> = <P as details::Impl>::Ref<'a, T>;

#[cfg(test)]
mod tests {
	use static_assertions::*;

	use super::*;

	#[test]
	fn permission_stack() {
		// Ordinary permissions
		assert_impl_all!(Shared: Permission);
		assert_impl_all!(Unique: Permission);

		// Prepend `Shared` to some permission to create a stack
		assert_impl_all!((Shared, Shared): Permission);
		assert_impl_all!((Shared, Unique): Permission);

		// Prepend `Shared` to an existing stack
		assert_impl_all!((Shared, (Shared, Shared)): Permission);
		assert_impl_all!((Shared, (Shared, Unique)): Permission);

		let mut data = 0usize;
		let data_ptr = &mut data as *mut usize;
		let base: Pointer<usize, Unique> = Pointer::from(data_ptr);
		let one: Pointer<usize, (Shared, Unique)> = base.make_shared();
		let two: Pointer<usize, (Shared, (Shared, Unique))> = one.make_shared();

		assert!(matches!(two.make_mut(), Ok(p) if p == base));
	}
}
