//! Rebuilds [`core::ptr::NonNull`] using the [`Permission`] system.

use core::{
	any,
	cmp,
	fmt,
	marker::PhantomData,
	mem,
	num::NonZero,
	ptr::NonNull,
};

use super::{
	Permission,
	Pointer,
	Reference,
	Shared,
	Unique,
	error::{
		MisalignedError,
		NonNullError,
	},
};

#[repr(transparent)]
#[doc = include_str!("../../doc/ptr/struct.NonNullPointer.md")]
pub struct NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	inner: NonNull<T>,
	_perm: PhantomData<P>,
}

impl<T> NonNullPointer<T, Shared>
where T: ?Sized
{
	/// Creates a new `NonNullPointer` if `ptr` is non-null.
	///
	/// The argument is a `*const` or `*mut` raw pointer.
	///
	/// # Original
	///
	/// [`NonNull::new`]
	///
	/// # API Differences
	///
	/// Returns a `Result`, rather than `Option`. The error value is a ZST that
	/// prints a useful error message when `.unwrap()`ped.
	///
	/// # Panics
	///
	/// This invokes `NonNull::new` directly, which panics during const
	/// evaluation if the pointer cannot be determined to be null or not. See
	/// [`is_null`](Pointer::is_null) for more information.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	/// use core::ptr;
	///
	/// let x = 0u32;
	/// let ptr = NonNullPointer::<u32, Shared>::new(&x as *const _).unwrap();
	///
	/// if let Ok(ptr) = NonNullPointer::<u32, Shared>::new(ptr::null()) {
	///   unreachable!();
	/// }
	/// ```
	#[inline(always)]
	pub const fn new(ptr: *const T) -> Result<Self, NonNullError<T, Shared>> {
		Self::from_pointer(Pointer::from_const(ptr))
	}

	/// Creates a new `NonNullPointer`.
	///
	/// # Original
	///
	/// [`NonNull::new_unchecked`]
	///
	/// # Safety
	///
	/// `ptr` must be non-null.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let x = 0u32;
	/// let ptr = unsafe {
	///   NonNullPointer::<u32, Shared>::new_unchecked(&x as *const _)
	/// };
	/// ```
	///
	/// _Incorrect_ usage of this function:
	///
	/// ```rust,ignore
	/// use core::ptr;
	/// use funty::ptr::*;
	///
	/// // NEVER DO THAT!!! This is undefined behavior. ⚠️
	///
	/// let ptr = unsafe {
	///   NonNullPointer::<u32, Shared>::new_unchecked(core::ptr::null())
	/// };
	/// ```
	#[inline(always)]
	pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
		Self::from_nonnull(unsafe { NonNull::new_unchecked(ptr.cast_mut()) })
	}

	/// Converts a reference to a `NonNullPointer`.
	///
	/// # Original
	///
	/// [`NonNull::from_ref`]
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub const fn from_ref(r: &T) -> Self {
		Self::from_nonnull(NonNull::from_ref(r))
	}
}

impl<T> NonNullPointer<T, Unique>
where T: ?Sized
{
	/// Creates a new `NonNullPointer` if `ptr` is non-null.
	///
	/// The argument is a `*const` or `*mut` raw pointer.
	///
	/// # Original
	///
	/// [`NonNull::new`]
	///
	/// # API Differences
	///
	/// Returns a `Result`, rather than `Option`. The error value is a ZST that
	/// prints a useful error message when `.unwrap()`ped.
	///
	/// # Panics
	///
	/// This invokes `NonNull::new` directly, which panics during const
	/// evaluation if the pointer cannot be determined to be null or not. See
	/// [`is_null`](Pointer::is_null) for more information.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	/// use core::ptr;
	///
	/// let mut x = 0u32;
	/// let ptr = NonNullPointer::<u32, Unique>::new(&mut x as *mut _).unwrap();
	///
	/// if let Ok(ptr) = NonNullPointer::<u32, Unique>::new(ptr::null_mut()) {
	///   unreachable!();
	/// }
	/// ```
	#[inline(always)]
	pub const fn new(ptr: *mut T) -> Result<Self, NonNullError<T, Unique>> {
		Self::from_pointer(Pointer::from_mut(ptr))
	}

	/// Creates a new `NonNullPointer`.
	///
	/// # Original
	///
	/// [`NonNull::new_unchecked`]
	///
	/// # Safety
	///
	/// `ptr` must be non-null.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	/// use core::ptr;
	///
	/// let mut x = 0u32;
	/// let ptr = unsafe {
	///   NonNullPointer::<u32, Unique>::new_unchecked(&mut x as *mut _)
	/// };
	/// ```
	///
	/// _Incorrect_ usage of this function:
	///
	/// ```rust,ignore
	/// use core::ptr;
	/// use funty::ptr::*;
	///
	/// // NEVER DO THAT!!! This is undefined behavior. ⚠️
	///
	/// let ptr = unsafe {
	///   NonNullPointer::<u32, Unique>::new_unchecked(core::ptr::null_mut())
	/// };
	/// ```
	#[inline(always)]
	pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
		Self::from_nonnull(unsafe { NonNull::new_unchecked(ptr) })
	}

	/// Converts a mutable reference to a `NonNullPointer`.
	///
	/// # Original
	///
	/// [`NonNull::from_mut`]
	#[inline(always)]
	pub const fn from_mut(r: &mut T) -> Self {
		Self::from_nonnull(NonNull::from_mut(r))
	}
}

impl<T, P> NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Converts a [`Pointer`] to a [`NonNullPointer`], if it is not null.
	#[inline(always)]
	pub const fn from_pointer(
		ptr: Pointer<T, P>,
	) -> Result<Self, NonNullError<T, P>> {
		match NonNull::new(unsafe { mem::transmute_copy(&ptr.ptr) }) {
			| Some(nnp) => Ok(Self::from_nonnull(nnp)),
			| None => Err(NonNullError::new()),
		}
	}

	/// Wraps a standard-library [`NonNull`].
	#[inline(always)]
	pub const fn from_nonnull(src: NonNull<T>) -> Self {
		Self {
			inner: src,
			_perm: PhantomData,
		}
	}
}

impl<T, P> NonNullPointer<T, P>
where
	T: Sized,
	P: Permission,
{
	/// Converts this pointer to a reference, after checking alignment.
	#[inline]
	pub unsafe fn as_reference_checked<'a>(
		self,
	) -> Result<Reference<'a, T, P>, MisalignedError<T>> {
		if !self.is_aligned() {
			return Err(MisalignedError::new(
				self.as_pointer().make_const().into_raw(),
			));
		}
		Ok(unsafe { self.as_reference() })
	}
}

/// Mirrors of the [`core::ptr::NonNull`] API.
impl<T, P> NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Gets the “address” portion of the pointer.
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::addr`].
	///
	/// This is a [Strict Provenance][0] API.
	///
	/// # Original
	///
	/// [`NonNull::addr`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
	#[inline(always)]
	pub fn addr(self) -> NonZero<usize> {
		self.inner.addr()
	}

	/// Exposes the [“provenance”][0] part of the pointer for future use in
	/// [`with_exposed_provenance`][1] and returns the “address” portion.
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::expose_provenance`].
	///
	/// This is an [Exposed Provenance][2] API.
	///
	/// # Original
	///
	/// [`NonNull::expose_provenance`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
	/// [1]: Self::with_exposed_provenance
	/// [2]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub fn expose_provenance(self) -> NonZero<usize> {
		self.inner.expose_provenance()
	}

	/// Creates a new pointer with the given address and the provenance of
	/// `self`.
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::with_addr`].
	///
	/// This is a [Strict Provenance][0] API.
	///
	/// # Original
	///
	/// [`NonNull::with_addr`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
	#[inline(always)]
	pub fn with_addr(self, addr: NonZero<usize>) -> Self {
		Self::from_nonnull(self.inner.with_addr(addr))
	}

	/// Creates a new pointer by mapping `self`’s address to a new one,
	/// preserving the provenance of `self`.
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::map_addr`].
	///
	/// This is a [Strict Provenance][0] API.
	///
	/// # Original
	///
	/// [`NonNull::map_addr`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
	#[inline(always)]
	pub fn map_addr(
		self,
		func: impl FnOnce(NonZero<usize>) -> NonZero<usize>,
	) -> Self {
		Self::from_nonnull(self.inner.map_addr(func))
	}

	/// Acquires the underlying raw pointer.
	///
	/// This method exists only so that the standard-library spelling will still
	/// work when changing types. Use [`.as_pointer()`](`Self::as_pointer`) to
	/// get the nullable pointer, and use _that_ type’s API to get the pointer
	/// primitive if you really want it.
	#[inline(always)]
	#[deprecated = "use `.as_pointer()`"]
	pub const fn as_ptr(self) -> P::Ptr<T> {
		unsafe { mem::transmute_copy(&self.inner.as_ptr()) }
	}

	/// Acquires the underlying [`Pointer`].
	///
	/// # Original
	///
	/// [`NonNull::as_ptr`]
	///
	/// # API Differences
	///
	/// This produces a `funty::Pointer`, rather than a `*{const, mut} T`
	/// primitive.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let mut x = 0u32;
	/// let ptr = NonNullPointer::<_, Unique>::new(&mut x).unwrap();
	///
	/// let x_value = unsafe { ptr.as_pointer().read() };
	/// assert_eq!(x_value, 0);
	///
	/// unsafe { ptr.as_pointer().write(2); }
	/// let x_value = unsafe { ptr.as_pointer().read() };
	/// assert_eq!(x_value, 2);
	/// ```
	#[inline(always)]
	pub const fn as_pointer(self) -> Pointer<T, P> {
		Pointer::new_from_const(self.inner.as_ptr().cast_const())
	}

	/// Generalized equivalent to [`.as_ref()`](Self::as_ref) or
	/// [`.as_mut()`](Self::as_mut).
	///
	/// # Safety
	///
	/// 1. This must point to a validly allocated, initialized, and aligned
	///    location of type `T`.
	/// 1. There must be no outstanding references to the value at this location
	///    which would conflict with the reference this function produces.
	/// 1. The pointed-to value must outlive the conjured lifetime of the new
	///    reference.
	#[inline(always)]
	pub const unsafe fn as_reference<'a>(self) -> Reference<'a, T, P> {
		unsafe { mem::transmute_copy(&self.as_pointer().ptr) }
	}

	/// Returns a shared reference to the value. Do not use if the value is not
	/// [`MaybeUninit`](core::mem::MaybeUninit) and might be uninitialized.
	///
	/// For the mutable counterpart, see [`as_mut`](NonNullPointer::as_mut).
	///
	/// # Original
	///
	/// [`NonNull::as_ref`]
	///
	/// # Safety
	///
	/// When calling this method, you have to ensure that the pointer is
	/// [convertible to a reference][0].
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let x = 0u32;
	/// let ptr = NonNullPointer::<u32, Shared>::new(&x).unwrap();
	///
	/// let x_ref = unsafe { ptr.as_ref() };
	/// assert_eq!(*x_ref, 0);
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#pointer-to-reference-conversion
	#[inline(always)]
	pub const unsafe fn as_ref<'a>(&self) -> &'a T {
		unsafe { self.inner.as_ref() }
	}

	/// Casts to a pointer of another type.
	///
	/// # Original
	///
	/// [`NonNull::cast`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let mut x = 0u32;
	/// let ptr = NonNullPointer::<u32, Unique>::new(&mut x).unwrap();
	///
	/// let casted_ptr = ptr.cast::<i8>();
	/// let raw_ptr: *mut i8 = casted_ptr.as_pointer().into_raw();
	/// ```
	#[inline(always)]
	pub const fn cast<U>(self) -> NonNullPointer<U, P>
	where U: Sized {
		NonNullPointer::from_nonnull(self.inner.cast::<U>())
	}

	/// Calculates the offset from a pointer in bytes.
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
	/// [`NonNull::byte_offset`]
	///
	/// [0]: Self::offset
	#[inline(always)]
	pub const unsafe fn byte_offset(self, count: isize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_offset(count) })
	}

	/// Calculates the offset from a pointer in bytes (convenience for
	/// `.byte_offset(count as isize)`).
	///
	/// `count` is in units of bytes.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`add`] on it. See that method for documentation and safety
	/// requirements.
	///
	/// For non-`Sized` pointees this operation changes only the data pointer,
	/// leaving the metadata untouched.
	///
	/// # Original
	///
	/// [`NonNull::add`]
	///
	/// [`add`]: Self::add
	#[inline(always)]
	pub const unsafe fn byte_add(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_add(count) })
	}

	/// Calculates the offset from a pointer in bytes (convenience for
	/// `.byte_offset((count as isize).wrapping_neg())`).
	///
	/// `count` is in units of bytes.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`sub`] on it. See that method for documentation and safety
	/// requirements.
	///
	/// For non-`Sized` pointees this operation changes only the data pointer,
	/// leaving the metadata untouched.
	///
	/// # Original
	///
	/// [`NonNull::sub`]
	///
	/// [`sub`]: Self::sub
	#[inline(always)]
	pub const unsafe fn byte_sub(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_sub(count) })
	}

	/// Calculates the distance between two pointers within the same allocation.
	/// The returned value is in units of bytes.
	///
	/// This is purely a convenience for casting to a u8 pointer and using
	/// [`offset_from`] on it. See that method for documentation and safety
	/// requirements.
	///
	/// For non-`Sized` pointees this operation considers only the data
	/// pointers, ignoring the metadata.
	///
	/// # Original
	///
	/// [`NonNull::byte_offset_from`]
	///
	/// [`offset_from`]: Self::offset_from
	#[inline(always)]
	pub const unsafe fn byte_offset_from<U, Q>(
		self,
		origin: NonNullPointer<U, Q>,
	) -> isize
	where
		U: ?Sized,
		Q: Permission,
	{
		unsafe { self.inner.byte_offset_from(origin.inner) }
	}

	/// Calculates the distance between two pointers within the same allocation,
	/// _where it’s known that `self` is equal to or greater than _`origin`_.
	/// The returned value is in units of **bytes**.
	///
	/// This is purely a convenience for casting to a `u8` pointer and using
	/// [`offset_from_unsigned`] on it. See that method for documentation and
	/// safety requirements.
	///
	/// For non-`Sized` pointees this operation considers only the data
	/// pointers, ignoring the metadata.
	///
	/// # Original
	///
	/// [`NonNull::byte_offset_from_unsigned`]
	///
	/// [`offset_from_unsigned`]: Self::offset_from_unsigned
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub const unsafe fn byte_offset_from_unsigned<U, Q>(
		self,
		origin: NonNullPointer<U, Q>,
	) -> usize
	where
		U: ?Sized,
		Q: Permission,
	{
		unsafe { self.inner.byte_offset_from_unsigned(origin.inner) }
	}
}

/// Mirrors of the `NonNull<T: Sized>` standard library APIs.
impl<T, P> NonNullPointer<T, P>
where
	T: Sized,
	P: Permission,
{
	/// Creates a pointer with the given address and no [provenance][0].
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::without_provenance`].
	///
	/// This is a [Strict Provenance][1] API.
	///
	/// # Original
	///
	/// [`NonNull::without_provenance`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
	/// [1]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub const fn without_provenance(addr: NonZero<usize>) -> Self {
		Self::from_nonnull(NonNull::without_provenance(addr))
	}

	/// Creates a new non-null poniter that is dangling, but well-aligned.
	///
	/// This is useful for initializing types which lazily allocate, like
	/// `Vec::new` does.
	///
	/// Note that the pointer value may potentially represent a valid pointer to
	/// a `T`, which means this must not be used as a “not yet initialized”
	/// sentinel value. Types that lazily allocate must track initialization by
	/// some other means.
	///
	/// # Original
	///
	/// [`NonNull::dangling`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let ptr = NonNullPointer::<u32, Unique>::dangling();
	/// // Important: don't try to access the value of `ptr` without
	/// // initializing it first! The pointer is not null but isn't valid either!
	/// ```
	#[inline(always)]
	pub const fn dangling() -> Self {
		Self::from_nonnull(NonNull::dangling())
	}

	/// Converts an address back to a mutable pointer, picking up some
	/// previously ‘exposed’ [provenance][0].
	///
	/// For more details, see the equivalent method on a raw pointer,
	/// [`Pointer::with_exposed_provenance`].
	///
	/// This is an [Exposed Provenance][1] API.
	///
	/// # Original
	///
	/// [`NonNull::with_exposed_provenance`]
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
	/// [1]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub fn with_exposed_provenance(addr: NonZero<usize>) -> Self {
		Self::from_nonnull(NonNull::with_exposed_provenance(addr))
	}

	/// Adds an offset to a pointer.
	///
	/// `count` is in units of T; e.g., a `count` of 3 represents a pointer
	/// offset of `3 * size_of::<T>()` bytes.
	///
	/// # Original
	///
	/// [`NonNull::offset`]
	///
	/// # Safety
	///
	/// If any of the following conditions are violated, the result is Undefined
	/// Behavior:
	///
	/// - The computed offset, `count * size_of::<T>()` bytes, must not overflow
	///   `isize`.
	///
	/// - If the computed offset is non-zero, then self must be derived from a
	///   pointer to some [allocation][0], and the entire memory range between
	///   `self` and the result must be in bounds of that allocation. In
	///   particular, this range must not “wrap around” the edge of the address
	///   space.
	///
	/// Allocations can never be larger than `isize::MAX` bytes, so if the
	/// computed offset stays in bounds of the allocation, it is guaranteed to
	/// satisfy the first requirement. This implies, for instance, that
	/// `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always safe.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let s = [1, 2, 3];
	/// let ptr = NonNullPointer::<i32, Shared>::new(s.as_ptr()).unwrap();
	///
	/// unsafe {
	///   assert_eq!(ptr.offset(1).read(), 2);
	///   assert_eq!(ptr.offset(2).read(), 3);
	/// }
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#allocation
	#[inline(always)]
	pub const unsafe fn offset(self, count: isize) -> Self {
		Self::from_nonnull(unsafe { self.inner.offset(count) })
	}

	/// Adds an offset to a pointer (convenience for `.offset(count as isize)`).
	///
	/// `count` is in units of T; e.g. a `count` of 3 represents a pointer
	/// offset of `3 * size_of::<T>()`.
	///
	/// # Original
	///
	/// [`NonNull::add`]
	///
	/// # Safety
	///
	/// If any of the following conditions are violated, the result is Undefined
	/// Behavior:
	///
	/// - The computed offset, `count * size_of::<T>()` bytes, must not overflow
	///   `isize`.
	/// - If the computed offset is non-zero, then `self` must be derived from a
	///   pointer to some [allocation][0], and the entire memory range between
	///   `self` and the result must be in bounds of that allocation. In
	///   particular, this range must not “wrap around” the edge of the address
	///   space.
	///
	/// Allocations can never be larger than `isize::MAX` bytes, so if the
	/// computed offset stays in bounds of the allocation, it is guaranteed to
	/// satisfy the first requirement. This implies, for instance, that
	/// `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always safe.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let s: &str = "123";
	/// let ptr = NonNullPointer::<u8, Shared>::new(s.as_bytes().as_ptr()).unwrap();
	///
	/// unsafe {
	///   assert_eq!(ptr.add(1).read() as char, '2');
	///   assert_eq!(ptr.add(2).read() as char, '3');
	/// }
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#allocation
	#[inline(always)]
	pub const unsafe fn add(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.add(count) })
	}

	/// Subtracts an offset from a pointer (convenience for `.offset((count as
	/// isize).wrapping_neg())`).
	///
	///`count` is in units of T; e.g., a count of 3 represents a pointer offset
	/// of `3 * size_of::<T>()` bytes.
	///
	/// # Original
	///
	/// [`NonNull::sub`]
	///
	/// # Safety
	///
	/// If any of the following conditions are violated, the result is Undefined
	/// Behavior:
	///
	/// - The computed offset, `count * size_of::<T>()` bytes, must not overflow
	///   `isize`.
	/// - If the computed offset is non-zero, then `self` must be derived from a
	///   pointer to some [allocation][0], and the entire memory range between
	///   `self` and the result must be in bounds of that allocation. In
	///   particular, this range must not “wrap around” the edge of the address
	///   space.
	///
	/// Allocations can never be larger than `isize::MAX` bytes, so if the
	/// computed offset stays in bounds of the allocation, it is guaranteed to
	/// satisfy the first requirement. This implies, for instance, that
	/// `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always safe.
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let s: &str = "123";
	///
	/// unsafe {
	///   let end = NonNullPointer::<u8, Shared>::new(s.as_bytes().as_ptr()).unwrap().add(3);
	///   assert_eq!(end.sub(1).read() as char, '3');
	///   assert_eq!(end.sub(2).read() as char, '2');
	/// }
	/// ```
	///
	/// Allocations always include the _address_ `&base[size]`, although that
	/// final address is not _dereferenceable_. In this example, it is not UB to
	/// compute the address immediately following `b'3'`, but it _would_ be UB
	/// to dereference a pointer to that address.
	#[inline(always)]
	pub const unsafe fn sub(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.sub(count) })
	}

	/// Calculates the distance between two pointers within the same allocation.
	/// The returned value is in units of T: the distance in bytes divided by
	/// `size_of::<T>()`.
	///
	/// This is equivalent to `(self as isize - origin as isize) /
	/// (size_of::<T>() as isize)`, except that it has a lot more opportunities
	/// for UB, in exchange for the compiler better understanding what you are
	/// doing.
	///
	/// The primary motivation of this method is for computing the `len` of an
	/// array/slice of `T` that you are currently representing as a “start” and
	/// “end” pointer (and “end” is “one past the end” of the array). In that
	/// case, `end.offset_from(start)` gets you the length of the array.
	///
	/// All of the following safety requirements are trivially satisfied for
	/// this usecase.
	///
	/// # Original
	///
	/// [`NonNull::offset_from`]
	///
	/// # Safety
	///
	/// If any of the following conditions are violated, the result is Undefined
	/// Behavior:
	///
	/// - `self` and `origin` must either
	///   - point to the same address, or
	///   - both be _derived from_ a pointer to the same [allocation][0], and
	///     the memory range between the two pointers must be in bounds of that
	///     object. (See below for an example.)
	/// - The distance between the pointers, in bytes, must be an exact multiple
	///   of the size of `T`.
	///
	/// As a consequence, the absolute distance between the pointers, in bytes,
	/// computed on mathematical integers (without “wrapping around”), cannot
	/// overflow an `isize`. This is implied by the in-bounds requirement, and
	/// the fact that no allocation can be larger than `isize::MAX` bytes.
	///
	/// The requirement for pointers to be derived from the same allocation is
	/// primarily needed for `const`-compatibility: the distance between
	/// pointers into different allocated objects is not known at compile-time.
	/// However, the requirement also exists at runtime and may be exploited by
	/// optimizations. If you wish to compute the difference between pointers
	/// that are not guaranteed to be from the same allocation, use `(self as
	/// isize - origin as isize) / size_of::<T>()`.
	///
	/// # Panics
	///
	/// This function panics if `T` is a Zero-Sized Type (“ZST”).
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let a = [0u32; 5];
	/// let ptr1 = NonNullPointer::<u32, Shared>::from(&a[1]);
	/// let ptr2 = NonNullPointer::<u32, Shared>::from(&a[3]);
	/// unsafe {
	///   assert_eq!(ptr2.offset_from(ptr1), 2);
	///   assert_eq!(ptr1.offset_from(ptr2), -2);
	///   assert_eq!(ptr1.offset(2), ptr2);
	///   assert_eq!(ptr2.offset(-2), ptr1);
	/// }
	/// ```
	///
	/// _Incorrect_ usage:
	///
	/// ```rust,ignore
	/// use funty::ptr::*;
	///
	/// let ptr1 = NonNullPointer::<u8, Shared>::new(Box::into_raw(Box::new(0u8))).unwrap();
	/// let ptr2 = NonNullPointer::<u8, Shared>::new(Box::into_raw(Box::new(1u8))).unwrap();
	/// let diff = (ptr2.addr().get() as isize).wrapping_sub(ptr1.addr().get() as isize);
	/// // Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
	/// let ptr2_other = NonNullPointer::<u8, Shared>::new(ptr1.as_ptr().wrapping_byte_offset(diff)).unwrap();
	/// assert_eq!(ptr2.addr(), ptr2_other.addr());
	/// // Since ptr2_other and ptr2 are derived from pointers to different objects,
	/// // computing their offset is undefined behavior, even though
	/// // they point to addresses that are in-bounds of the same object!
	///
	/// let one = unsafe { ptr2_other.offset_from(ptr2) }; // Undefined Behavior! ⚠️
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#allocation
	#[inline(always)]
	pub const unsafe fn offset_from<Q>(
		self,
		origin: NonNullPointer<T, Q>,
	) -> isize
	where
		Q: Permission,
	{
		unsafe { self.inner.offset_from(origin.inner) }
	}

	/// Calculates the distance between two pointers within the same allocation,
	/// _where it’s known that `self` is equal to or greater than `origin`_. The
	/// returned value is in units of T: the distance in bytes is divided by
	/// `size_of::<T>()`.
	///
	/// This computes the same value that [`offset_from`] would compute, but
	/// with the added precondition that the offset is guaranteed to be
	/// non-negative. This method is equivalent to
	/// `usize::try_from(self.offset_from(origin)).unwrap_unchecked()`, but it
	/// provides slightly more information to the optimizer, which can sometimes
	/// allow it to optimize slightly better with some backends.
	///
	/// This method can be though of as recovering the `count` that was passed
	/// to [`add`] (or, with the parameters in the other order, to [`sub`]).
	/// The following are all equivalent, assuming that their safety
	/// preconditions are met:
	///
	/// ```rust,ignore
	/// ptr.offset_from_unsigned(origin) == count
	/// origin.add(count) == ptr
	/// ptr.sub(count) == origin
	/// ```
	///
	/// # Original
	///
	/// [`NonNull::offset_from_unsigned`]
	///
	/// # Safety
	///
	/// - The distance between the pointers must be non-negative (`self >=
	/// origin`)
	///
	/// - _All_ the safety conditions of `offset_from` apply to this method as
	/// well; see it for the full details.
	///
	/// Importantly, despite the return type of this method being able to
	/// represent a larger offset, it’s still not permitted to pass pointers
	/// which differ by more than `isize::MAX` _bytes_. As such, the result of
	/// this method will always be less than or equal to `isize::MAX as usize`.
	///
	/// # Panics
	///
	/// This function panics if T is a Zero-Sized Type (“ZST”).
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let a = [0; 5];
	/// let ptr1 = NonNullPointer::<i32, Shared>::from(&a[1]);
	/// let ptr2 = NonNullPointer::<i32, Shared>::from(&a[3]);
	/// unsafe {
	///   assert_eq!(ptr2.offset_from_unsigned(ptr1), 2);
	///   assert_eq!(ptr1.add(2), ptr2);
	///   assert_eq!(ptr2.sub(2), ptr1);
	///   assert_eq!(ptr2.offset_from_unsigned(ptr2), 0);
	/// }
	///
	/// // This would be incorrect, as the pointers are not correctly ordered:
	/// // ptr1.offset_from_unsigned(ptr2)
	/// ```
	///
	/// [`offset_from`]: Self::offset_from
	/// [`add`]: Self::add
	/// [`sub`]: Self::sub
	#[inline(always)]
	#[cfg(feature = "rust_187")]
	pub const unsafe fn offset_from_unsigned<Q>(
		self,
		subtracted: NonNullPointer<T, Q>,
	) -> usize
	where
		Q: Permission,
	{
		unsafe { self.inner.offset_from_unsigned(subtracted.inner) }
	}

	/// Reads the value from `self` without moving it. This leaves the memory in
	/// `self` unchanged.
	///
	/// See [`Pointer::read`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::read`]
	///
	/// # Safety
	///
	/// This must point to an allocated, alive, value, and the value must be
	/// dropped the correct number of times. The address value must be aligned
	/// for `T`.
	#[inline(always)]
	pub const unsafe fn read(self) -> T {
		unsafe { self.inner.read() }
	}

	/// Performs a volatile read of the value from `self` without moving it.
	/// This leaves the memory in `self` unchanged.
	///
	/// Volatile operations are intended to act on I/O memory, and are
	/// guaranteed to not be elided or reordered by the compiler across other
	/// volatile operations.
	///
	/// See [`Pointer::read_volatile`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::read_volatile`]
	///
	/// # Safety
	///
	/// The Rust Abstract Machine does not check this address for
	/// dereferenceability. The address must be valid in the program’s run-time
	/// address space.
	#[inline(always)]
	pub unsafe fn read_volatile(self) -> T {
		unsafe { self.inner.read_volatile() }
	}

	/// Reads the value from `self` without moving it. This leaves the memory in
	/// `self` unchanged.
	///
	/// Unlike `read`, the pointer may be unaligned.
	///
	/// See [`Pointer::read_unaligned`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::read_unaligned`]
	///
	/// # Safety
	///
	/// The address must be valid in the program’s run-time address space. It is
	/// not checked by the Rust Abstract Machine.
	#[inline(always)]
	pub const unsafe fn read_unaligned(self) -> T {
		unsafe { self.inner.read_unaligned() }
	}

	/// Copies `count * size_of::<T>()` bytes from `self` to `dest`. The source
	/// and destination may overlap.
	///
	/// NOTE: this has the _same_ argument order as [`ptr::copy`].
	///
	/// See [`ptr::copy`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::copy_to`]
	///
	/// [`ptr::copy`]: crate::ptr::copy
	#[inline(always)]
	pub const unsafe fn copy_to(
		self,
		dest: NonNullPointer<T, Unique>,
		count: usize,
	) {
		unsafe {
			self.inner.copy_to(dest.inner, count);
		}
	}

	/// Copies `count * size_of::<T>()` bytes from `self` to `dest`. The source
	/// and destination may _not_ overlap.
	///
	/// NOTE: this has the _same_ argument order as
	/// [`ptr::copy_nonoverlapping`].
	///
	/// See [`ptr::copy_nonoverlapping`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::copy_to_nonoverlapping`]
	///
	/// [`ptr::copy_nonoverlapping`]: crate::ptr::copy_nonoverlapping
	#[inline(always)]
	pub const unsafe fn copy_to_nonoverlapping(
		self,
		dest: NonNullPointer<T, Unique>,
		count: usize,
	) {
		unsafe {
			self.inner.copy_to_nonoverlapping(dest.inner, count);
		}
	}

	/// Computes the offset that needs to be applied to the pointer in order to
	/// make it aligned to `align`.
	///
	/// If it is not possible to align the pointer, the implementation returns
	/// `usize::MAX`.
	///
	/// The offset is expressed in number of `T` elements, and not bytes.
	///
	/// There are no guarantees whatsoever that offsetting the pointer will not
	/// overflow or go beyond the allocation that the pointer points into. It is
	/// up to the caller to ensure that the returned offset is correct in all
	/// terms other than alignment.
	///
	/// When this is called during compile-time evaluation (which is unstable),
	/// the implementation may return `usize::MAX` in cases where that can never
	/// happen at runtime. This is because the actual alignment of pointers is
	/// not known yet during compile-time, so an offset with guaranteed
	/// alignment can sometimes not be computed. For example, a buffer declared
	/// as `[u8; N]` might be allocated at an odd or an even address, but at
	/// compile-time this is not yet known, so the execution has to be correct
	/// for either choice. It is therefore impossible to find an offset that is
	/// guaranteed to be 2-aligned. (This behavior is subject to change, as
	/// usual for unstable APIs.)
	///
	/// # Original
	///
	/// [`NonNull::align_offset`]
	///
	/// # Panics
	///
	/// The function panics if `align` is not a power-of-two.
	///
	/// # Examples
	///
	/// Accessing adjacent `u8` as `u16`:
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let x = [5_u8, 6, 7, 8, 9];
	/// let ptr = NonNullPointer::<u8, Shared>::new(x.as_ptr()).unwrap();
	/// let offset = ptr.align_offset(align_of::<u16>());
	///
	/// if offset < x.len() - 1 {
	/// # unsafe {
	///   let u16_ptr = ptr.add(offset).cast::<u16>();
	///   assert!(u16_ptr.read() == u16::from_ne_bytes([5, 6]) || u16_ptr.read() == u16::from_ne_bytes([6, 7]));
	/// # }
	/// } else {
	///   // while the pointer can be aligned via `offset`, it would point
	///   // outside the allocation
	/// }
	/// ```
	#[inline(always)]
	pub fn align_offset(self, align: usize) -> usize {
		self.inner.align_offset(align)
	}

	/// Tests if the pointer is properly aligned for `T`.
	///
	/// # Original
	///
	/// [`NonNull::is_aligned`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// // On some platforms, the alignment of i32 is less than 4.
	/// #[repr(align(4))]
	/// struct AlignedI32(i32);
	///
	/// let data = AlignedI32(42);
	/// let ptr = NonNullPointer::from(&data);
	/// assert!(ptr.is_aligned());
	/// assert!(!unsafe { ptr.byte_add(1) }.is_aligned());
	/// ```
	#[inline(always)]
	pub fn is_aligned(self) -> bool {
		self.inner.is_aligned()
	}
}

impl<T> NonNullPointer<T, Unique>
where T: ?Sized
{
	/// Returns a unique reference to the value. Do not use if the value is not
	/// [`MaybeUninit`](core::mem::MaybeUninit) and might be uninitialized.
	///
	/// For the shared counterpart, see [`as_ref`](NonNullPointer::as_ref).
	///
	/// # Original
	///
	/// [`NonNull::as_mut`]
	///
	/// # Safety
	///
	/// When calling this method, you have to ensure that the pointer is
	/// [convertible to a reference][0].
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let mut x = 0u32;
	/// let mut ptr = NonNullPointer::<u32, Unique>::new(&mut x).unwrap();
	///
	/// let x_ref = unsafe { ptr.as_mut() };
	/// assert_eq!(*x_ref, 0);
	/// *x_ref += 2;
	/// assert_eq!(*x_ref, 2);
	/// ```
	///
	/// [0]: https://doc.rust-lang.org/core/ptr/index.html#pointer-to-reference-conversion
	#[inline(always)]
	pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
		unsafe { self.inner.as_mut() }
	}
}

impl<T> NonNullPointer<T, Unique>
where T: Sized
{
	/// Copies `count * size_of::<T>()` bytes from `src` to `self`. The source
	/// and destination may overlap.
	///
	/// NOTE: this has the _opposite_ argument order of [`ptr::copy`].
	///
	/// See [`ptr::copy`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::copy_from`]
	///
	/// [`ptr::copy`]: crate::ptr::copy
	#[inline(always)]
	pub const unsafe fn copy_from<Q>(
		self,
		src: NonNullPointer<T, Q>,
		count: usize,
	) where
		Q: Permission,
	{
		unsafe { self.inner.copy_from(src.inner, count) }
	}

	/// Copies `count * size_of::<T>()` bytes from `src` to `self`. The source
	/// and destination may _not_ overlap.
	///
	/// NOTE: this has the _opposite_ argument order of
	/// [`ptr::copy_nonoverlapping`].
	///
	/// See [`ptr::copy_nonoverlapping`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::copy_from_nonoverlapping`]
	///
	/// [`ptr::copy_nonoverlapping`]: crate::ptr::copy_nonoverlapping
	#[inline(always)]
	pub const unsafe fn copy_from_nonoverlapping<Q>(
		self,
		src: NonNullPointer<T, Q>,
		count: usize,
	) where
		Q: Permission,
	{
		unsafe {
			self.inner.copy_from_nonoverlapping(src.inner, count);
		}
	}

	/// Executes the destructor (if any) of the pointed-to value.
	///
	/// See [`Pointer::drop_in_place`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::drop_in_place`]
	#[inline(always)]
	pub unsafe fn drop_in_place(self) {
		unsafe {
			self.inner.drop_in_place();
		}
	}

	///	Overwrites a memory location with the given value without reading or
	/// dropping the old value.
	///
	/// See [`Pointer::write`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::write`]
	#[inline(always)]
	pub const unsafe fn write(self, val: T) {
		unsafe {
			self.inner.write(val);
		}
	}

	/// Invokes memset on the specified pointer, setting `count *
	/// size_of::<T>()` bytes of memory starting at `self` to `val`.
	///
	/// See [`Pointer::write_bytes`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::write_bytes`]
	#[inline(always)]
	pub const unsafe fn write_bytes(self, val: u8, count: usize) {
		unsafe {
			self.inner.write_bytes(val, count);
		}
	}

	/// Performs a volatile write of a memory location with the given value
	/// without reading or dropping the old value.
	///
	/// Volatile operations are intended to act on I/O memory, and are
	/// guaranteed to not be elided or reordered by the compiler across other
	/// volatile operations.
	///
	/// See [`Pointer::write_volatile`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::write_volatile`]
	#[inline(always)]
	pub unsafe fn write_volatile(self, val: T) {
		unsafe {
			self.inner.write_volatile(val);
		}
	}

	/// Overwrites a memory location with the given value without reading or
	/// dropping the old value.
	///
	/// Unlike `write`, the pointer may be unaligned.
	///
	/// See [`Pointer::write_unaligned`] for safety concerns and examples.
	#[inline(always)]
	pub const unsafe fn write_unaligned(self, val: T) {
		unsafe {
			self.inner.write_unaligned(val);
		}
	}

	/// Replaces the value at `self` with `src`, returning the old value,
	/// without dropping either.
	///
	/// See [`Pointer::replace`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::replace`]
	#[inline(always)]
	pub const unsafe fn replace(self, src: T) -> T {
		unsafe { self.inner.replace(src) }
	}

	/// Swaps the values at two mutable locations of the same type, without
	/// deinitializing either. They may overlap, unlike `mem::swap` which is
	/// otherwise equivalent.
	///
	/// See [`Pointer::swap`] for safety concerns and examples.
	///
	/// # Original
	///
	/// [`NonNull::swap`]
	#[inline(always)]
	pub const unsafe fn swap(self, with: Self) {
		unsafe {
			self.inner.swap(with.inner);
		}
	}
}

impl<T, P> NonNullPointer<[T], P>
where
	T: Sized,
	P: Permission,
{
	/// Creates a non-null raw slice from a thin pointer and a length.
	///
	/// The `len` argument is the number of **elements**, not the number of
	/// bytes.
	///
	/// This function is safe, but dereferencing the return value is unsafe. See
	/// the documentation of [`slice::from_raw_parts`][0] for slice safety
	/// requirements.
	///
	/// # Original
	///
	/// [`NonNull::slice_from_raw_parts`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// // create a slice pointer when starting out with a pointer to the first element
	/// let x = [5, 6, 7];
	/// let nnp = NonNullPointer::<i32, Shared>::new(x.as_ptr()).unwrap();
	/// let slice = NonNullPointer::slice_from_raw_parts(nnp, 3);
	/// assert_eq!(unsafe{ slice.as_ref()[2] }, 7);
	/// ```
	///
	/// (Note that this example artificially demonstrates a use of this method,
	/// but `let slice = NonNull::from(&x[..]);`` would be a better way to write
	/// code like this.)
	///
	/// [0]: https://doc.rust-lang.org/core/slice/fn.from_raw_parts.html
	#[inline(always)]
	pub const fn slice_from_raw_parts(
		data: NonNullPointer<T, P>,
		len: usize,
	) -> Self {
		Self::from_nonnull(NonNull::slice_from_raw_parts(data.inner, len))
	}

	/// Returns the length of a non-null raw slice.
	///
	/// The returned value is the number of **elements**, not the number of
	/// bytes.
	///
	/// This function is safe, even when the non-null raw slice cannot be
	/// dereferenced to a slice because the pointer does not have a valid
	/// address.
	///
	/// # Original
	///
	/// [`NonNull::len`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let ptr = NonNullPointer::<u8, Shared>::dangling();
	/// let slice = NonNullPointer::slice_from_raw_parts(ptr, 3);
	/// assert_eq!(slice.len(), 3);
	/// ```
	#[inline(always)]
	pub const fn len(self) -> usize {
		self.inner.len()
	}

	/// Tests if the non-null raw slice has a length of 0.
	///
	/// # Original
	///
	/// [`NonNull::is_empty`]
	///
	/// # Examples
	///
	/// ```rust
	/// use funty::ptr::*;
	///
	/// let ptr = NonNullPointer::<u8, Shared>::dangling();
	/// let slice = NonNullPointer::slice_from_raw_parts(ptr, 3);
	/// assert!(!slice.is_empty());
	/// ```
	#[inline(always)]
	pub const fn is_empty(self) -> bool {
		self.inner.is_empty()
	}
}

impl<T, P> Clone for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, P> Eq for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

impl<T, P> Ord for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	fn cmp(&self, rhs: &Self) -> cmp::Ordering {
		self.partial_cmp(rhs)
			.expect("NonNullPointer has a total ordering")
	}
}

impl<T1, T2, P1, P2> PartialEq<NonNullPointer<T2, P2>> for NonNullPointer<T1, P1>
where
	T1: ?Sized,
	T2: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline]
	fn eq(&self, rhs: &NonNullPointer<T2, P2>) -> bool {
		self.addr() == rhs.addr()
	}
}

impl<T1, T2, P1, P2> PartialOrd<NonNullPointer<T2, P2>>
	for NonNullPointer<T1, P1>
where
	T1: ?Sized,
	T2: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline]
	fn partial_cmp(
		&self,
		rhs: &NonNullPointer<T2, P2>,
	) -> Option<cmp::Ordering> {
		self.as_pointer().partial_cmp(&rhs.as_pointer())
	}
}

impl<T> From<&T> for NonNullPointer<T, Shared>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &T) -> Self {
		Self::from_ref(src)
	}
}

impl<T> From<&mut T> for NonNullPointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &mut T) -> Self {
		Self::from_mut(src)
	}
}

impl<T> TryFrom<*const T> for NonNullPointer<T, Shared>
where T: ?Sized
{
	type Error = NonNullError<T, Shared>;

	#[inline]
	fn try_from(ptr: *const T) -> Result<Self, Self::Error> {
		Self::new(ptr)
	}
}

impl<T> TryFrom<*mut T> for NonNullPointer<T, Unique>
where T: ?Sized
{
	type Error = NonNullError<T, Unique>;

	#[inline]
	fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
		Self::new(ptr)
	}
}

impl<T, P> TryFrom<Pointer<T, P>> for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	type Error = NonNullError<T, P>;

	#[inline(always)]
	fn try_from(ptr: Pointer<T, P>) -> Result<Self, Self::Error> {
		NonNull::new(ptr.make_const().into_raw().cast_mut())
			.map(Self::from_nonnull)
			.ok_or_else(Self::Error::new)
	}
}

impl<T, P> fmt::Debug for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		if fmt.alternate() {
			write!(
				fmt,
				"(non-null {} {})",
				P::DEBUG_PREFIX,
				any::type_name::<T>(),
			)?;
		}
		fmt::Debug::fmt(&self.inner.as_ptr(), fmt)
	}
}

impl<T, P> fmt::Pointer for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Pointer::fmt(&self.inner.as_ptr(), fmt)
	}
}

impl<T, P> Copy for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}
