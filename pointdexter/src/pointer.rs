//! Implementation details for [`Pointer`]. See its type documentation.

#[cfg(doc)]
use core::ptr::NonNull;
use core::{
	any,
	cell::UnsafeCell,
	cmp,
	fmt,
	hash,
	panic,
	ptr,
};

use crate::{
	permission::{
		self,
		Impl as _,
	},
	NonNullPointer,
	NonUniqueError,
	NullPointerError,
	Permission,
	Pointer,
	Reference,
	Shared,
	Unique,
};

// region NEW_API

/// New APIs with no standard-library counterpart.
impl<T> Pointer<T, Shared>
where T: ?Sized
{
	/// Converts a raw pointer to a Pointer.
	#[inline(always)]
	pub const fn from_raw_const(ptr: *const T) -> Self {
		Self { inner: ptr }
	}
}

/// New APIs with no standard-library counterpart.
impl<T> Pointer<T, Unique>
where T: ?Sized
{
	/// Converts a raw pointer to a Pointer.
	#[inline(always)]
	pub const fn from_raw_mut(ptr: *mut T) -> Self {
		Self { inner: ptr }
	}

	/// Returns the interior raw `*mut T` pointer.
	#[inline(always)]
	pub const fn into_raw_mut(self) -> *mut T {
		self.inner
	}
}

/// New APIs with no standard-library counterpart.
impl<T, P> Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Converts into a raw `*const T`. Should only be used with `Self::rewrap`.
	#[inline(always)]
	pub(crate) const fn unwrap(self) -> *const T {
		unsafe { core::mem::transmute_copy(&self.inner) }
	}

	/// Converts from a raw `*const T`. Should only be used with `Self::unwrap`.
	#[inline(always)]
	pub(crate) const fn rewrap(ptr: *const T) -> Self {
		unsafe { core::mem::transmute_copy(&ptr) }
	}

	/// Converts into a raw `*mut T`. Should only be used with
	/// `Self::rewrap_mut`.`
	#[inline(always)]
	pub(crate) const fn unwrap_mut(self) -> *mut T {
		unsafe { core::mem::transmute_copy(&self.inner) }
	}

	/// Converts from a raw `*mut T`. Should only be used with
	/// `Self::unwrap_mut`.`
	#[inline(always)]
	pub(crate) const fn rewrap_mut(ptr: *mut T) -> Self {
		unsafe { core::mem::transmute_copy(&ptr) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.cast_permission.md")]
	pub const fn cast_shared(self) -> Pointer<T, (Shared, P)> {
		Pointer::rewrap(self.unwrap())
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.cast_permission.md")]
	pub const fn try_cast_unique(
		self,
	) -> Result<Pointer<T, Unique>, NonUniqueError<T>> {
		// This is safe to inline because it const-folds.
		if P::Original::IS_MUT {
			Ok(Pointer::rewrap(self.unwrap()))
		}
		else {
			Err(NonUniqueError::new())
		}
	}

	/// Returns the interior raw `*const T` pointer.
	#[inline(always)]
	pub const fn into_raw_const(self) -> *const T {
		self.unwrap()
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_reference<'a>(
		self,
	) -> Result<Reference<'a, T, P>, NullPointerError<T, P>> {
		match NonNullPointer::new(self) {
			| Err(err) => Err(err),
			| Ok(nnp) => Ok(unsafe { nnp.as_reference() }),
		}
	}
}

impl<T, P> Pointer<T, P>
where P: Permission
{
	#[inline(always)]
	#[doc = include_str!("../doc/fn.null.md")]
	pub const fn null() -> Self {
		if P::IS_MUT {
			Self::rewrap_mut(ptr::null_mut())
		}
		else {
			Self::rewrap(ptr::null())
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.dangling.md")]
	pub const fn dangling() -> Self {
		if P::IS_MUT {
			Self::rewrap_mut(ptr::dangling_mut())
		}
		else {
			Self::rewrap(ptr::dangling())
		}
	}

	#[inline(always)]
	#[cfg(not(feature = "rust_191"))]
	#[doc = include_str!("../doc/fn.with_exposed_provenance.md")]
	/// [`with_addr`]: Self::with_addr
	/// [`expose_provenance`]: Self::expose_provenance
	#[cfg_attr(
		not(feature = "rust_189"),
		doc = "
[`NonNullPointer::with_exposed_provenance`]: crate::NonNullPointer
[`NonNull::with_exposed_provenance`]: core::ptr::NonNull
		"
	)]
	pub fn with_exposed_provenance(addr: usize) -> Self {
		if P::IS_MUT {
			Self::rewrap_mut(ptr::with_exposed_provenance_mut(addr))
		}
		else {
			Self::rewrap(ptr::with_exposed_provenance(addr))
		}
	}

	#[inline(always)]
	#[cfg(feature = "rust_191")]
	#[doc = include_str!("../doc/fn.with_exposed_provenance.md")]
	/// [`with_addr`]: Self::with_addr
	/// [`expose_provenance`]: Self::expose_provenance
	pub const fn with_exposed_provenance(addr: usize) -> Self {
		if P::IS_MUT {
			Self::rewrap_mut(ptr::with_exposed_provenance_mut(addr))
		}
		else {
			Self::rewrap(ptr::with_exposed_provenance(addr))
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.without_provenance.md")]
	/// [`with_exposed_provenance`]: Self::with_exposed_provenance
	#[cfg_attr(
		not(feature = "rust_189"),
		doc = "
[`NonNullPointer::without_provenance`]: NonNullPointer
[`NonNull::without_provenance`]: core::ptr::NonNull
		"
	)]
	pub const fn without_provenance(addr: usize) -> Self {
		if P::IS_MUT {
			Self::rewrap_mut(ptr::without_provenance_mut(addr))
		}
		else {
			Self::rewrap(ptr::without_provenance(addr))
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.slice_from_raw_parts.md")]
	pub const fn make_slice(self, len: usize) -> Pointer<[T], P> {
		if P::IS_MUT {
			Pointer::rewrap_mut(ptr::slice_from_raw_parts_mut(
				self.unwrap_mut(),
				len,
			))
		}
		else {
			Pointer::rewrap(ptr::slice_from_raw_parts(self.unwrap(), len))
		}
	}
}

// endregion NEW_API

// region INHERENT_MIRROR

/// Mirrors of the standard-library `*const T`-only APIs.
impl<T> Pointer<T, Shared> where T: ?Sized {}

/// Mirrors of the standard-library `*mut T`-only APIs.
impl<T> Pointer<T, Unique> where T: ?Sized {}

/// Mirrors of the standard-library `*const T`/`*mut T` duplicated APIs.
impl<T, P> Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	pointer_dispatch! {
		#[doc = include_str!("../doc/struct.Pointer/method.is_null.md")]
		@const fn is_null(@self) -> bool;

		#[doc = include_str!("../doc/struct.Pointer/method.addr.md")]
		#[doc = "[`map_addr`]: Self::map_addr"]
		#[doc = "[`with_addr`]: Self::with_addr"]
		fn addr(@self) -> usize;

		#[doc = include_str!("../doc/struct.Pointer/method.expose_provenance.md")]
		#[cfg_attr(
			not(feature = "rust_189"),
			doc = "
[`NonNullPointer::expose_provenance`]: NonNullPointer
[`NonNull::expose_provenance`]: NonNull
			"
		)]
		fn expose_provenance(@self) -> usize;

		#[doc = include_str!("../doc/struct.Pointer/method.with_addr.md")]
		#[doc = "[`wrapping_offset`]: Self::wrapping_offset"]
		fn with_addr(@self, addr: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.map_addr.md")]
		#[doc = "[`with_addr`]: Self::with_addr"]
		fn map_addr(@self, func: impl FnOnce(usize) -> usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.byte_offset.md")]
		@const @@unsafe fn byte_offset(@self, count: isize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_byte_offset.md")]
		@const fn wrapping_byte_offset(@self, count: isize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.byte_add.md")]
		@const @@unsafe fn byte_add(@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.byte_sub.md")]
		@const @@unsafe fn byte_sub(@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_byte_add.md")]
		@const fn wrapping_byte_add(@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_byte_sub.md")]
		@const fn wrapping_byte_sub(@self, count: usize) via rewrap rewrap_mut -> Self;
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.cast.md")]
	pub const fn cast<U>(self) -> Pointer<U, P>
	where U: Sized {
		pointer_dispatch!(rewrap self.cast())
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_ref<'a>(
		self,
	) -> Result<&'a T, NullPointerError<T, P>> {
		match unsafe { pointer_dispatch!(nowrap self.as_ref()) } {
			| Some(r) => Ok(r),
			| None => Err(NullPointerError::new()),
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_offset_from.md")]
	pub const unsafe fn byte_offset_from<T2, P2>(
		self,
		origin: Pointer<T2, P2>,
	) -> isize
	where
		T2: ?Sized,
		P2: Permission,
	{
		unsafe { self.unwrap().byte_offset_from(origin.unwrap()) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_offset_from_unsigned.md")]
	pub const unsafe fn byte_offset_from_unsigned<T2, P2>(
		self,
		origin: Pointer<T2, P2>,
	) -> usize
	where
		T2: ?Sized,
		P2: Permission,
	{
		unsafe { self.unwrap().byte_offset_from_unsigned(origin.unwrap()) }
	}
}

/// Mirrors of the standard-library `*const T`-only APIs which require `T:
/// Sized`.
impl<T> Pointer<T, Shared> {
	#[deprecated = "use `.try_cast_unique()` in a generic context"]
	#[doc = include_str!("../doc/struct.Pointer/method.cast_permission.md")]
	pub const fn cast_mut(self) -> Pointer<T, Unique> {
		panic!("cannot conjure Unique permissions");
	}
}

/// Mirrors of the standard-library `*mut T`-only APIs which require `T: Sized`.
impl<T> Pointer<T, Unique> {
	#[deprecated = "use `.cast_shared()` to create a Pointer that can restore \
	                mutability"]
	#[doc = include_str!("../doc/struct.Pointer/method.cast_permission.md")]
	pub const fn cast_const(self) -> Pointer<T, Shared> {
		Pointer::from_raw_const(self.into_raw_mut().cast_const())
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_mut<'a>(
		self,
	) -> Result<&'a mut T, NullPointerError<T, Unique>> {
		match unsafe { self.inner.as_mut() } {
			| Some(r) => Ok(r),
			| None => Err(NullPointerError::new()),
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy.md")]
	/// [`copy_nonoverlapping`]: Self::copy_to_nonoverlapping
	/// [`read`]: Self::read
	pub const unsafe fn copy_from<P>(self, src: Pointer<T, P>, count: usize)
	where P: Permission {
		unsafe {
			self.inner.copy_from(src.into_raw_const(), count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy_nonoverlapping.md")]
	/// [`copy`]: Self::copy_from
	/// [`read`]: Self::read
	pub const unsafe fn copy_from_nonoverlapping<P>(
		self,
		src: Pointer<T, P>,
		count: usize,
	) where
		P: Permission,
	{
		unsafe {
			self.inner
				.copy_from_nonoverlapping(src.into_raw_const(), count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.drop_in_place.md")]
	pub unsafe fn drop_in_place(self) {
		unsafe {
			self.inner.drop_in_place();
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write.md")]
	/// [`read`]: Self::read
	/// [`write_unaligned`]: Self::write_unaligned
	pub const unsafe fn write(self, val: T) {
		unsafe {
			self.unwrap_mut().write(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_bytes.md")]
	pub const unsafe fn write_bytes(self, val: u8, count: usize) {
		unsafe {
			self.unwrap_mut().write_bytes(val, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_volatile.md")]
	pub unsafe fn write_volatile(self, val: T) {
		unsafe {
			self.unwrap_mut().write_volatile(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_unaligned.md")]
	/// [`write`]: Self::write
	/// [`read_unaligned`]: Self::read_unaligned
	pub const unsafe fn write_unaligned(self, val: T) {
		unsafe {
			self.unwrap_mut().write_unaligned(val);
		}
	}

	#[inline(always)]
	#[cfg(not(feature = "rust_188"))]
	#[doc = include_str!("../doc/fn.replace.md")]
	pub unsafe fn replace(self, src: T) -> T {
		unsafe { self.unwrap_mut().replace(src) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_188")]
	#[doc = include_str!("../doc/fn.replace.md")]
	pub const unsafe fn replace(self, src: T) -> T {
		unsafe { self.unwrap_mut().replace(src) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.swap.md")]
	pub const unsafe fn swap(self, with: Self) {
		unsafe {
			self.unwrap_mut().swap(with.unwrap_mut());
		}
	}
}

/// Mirrors of the standard-library `*const T`/`*mut T` duplicated APIs which
/// require `T: Sized`.
impl<T, P> Pointer<T, P>
where P: Permission
{
	pointer_dispatch! {
		#[doc = include_str!("../doc/struct.Pointer/method.offset.md")]
		#[doc = "[`wrapping_offset`]: Self::wrapping_offset"]
		@const @@unsafe fn offset(@self, count: isize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_offset.md")]
		@const fn wrapping_offset(@self, count: isize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.add.md")]
		#[doc = "[`offset`]: Self::offset"]
		#[doc = "[`wrapping_add`]: Self::wrapping_add"]
		@const @@unsafe fn add(@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_add.md")]
		@const fn wrapping_add (@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.sub.md")]
		#[doc = "[`offset`]: Self::offset"]
		#[doc = "[`wrapping_sub`]: Self::wrapping_sub"]
		@const @@unsafe fn sub(@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/struct.Pointer/method.wrapping_sub.md")]
		@const fn wrapping_sub (@self, count: usize) via rewrap rewrap_mut -> Self;

		#[doc = include_str!("../doc/fn.read.md")]
		#[doc = "[`read_unaligned`]: Self::read_unaligned"]
		@const @@unsafe fn read(@self) -> T;

		#[doc = include_str!("../doc/fn.read_volatile.md")]
		#[doc = "[`read`]: Self::read"]
		#[doc = "[`without_provenance`]: Self::without_provenance"]
		@@unsafe fn read_volatile(@self) -> T;

		#[doc = include_str!("../doc/fn.read_unaligned.md")]
		#[doc = "[`read`]: Self::read"]
		@const @@unsafe fn read_unaligned(@self) -> T;

		#[doc = include_str!("../doc/struct.Pointer/method.align_offset.md")]
		#[doc = "[`wrapping_add`]: Self::wrapping_add"]
		fn align_offset(@self, align: usize) -> usize;

		#[doc = include_str!("../doc/struct.Pointer/method.is_aligned.md")]
		fn is_aligned(@self) -> bool;
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.offset_from.md")]
	pub const unsafe fn offset_from<P2>(self, origin: Pointer<T, P2>) -> isize
	where P2: Permission {
		unsafe { self.unwrap().offset_from(origin.unwrap()) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	#[doc = include_str!("../doc/struct.Pointer/method.offset_from_unsigned.md")]
	pub const unsafe fn offset_from_unsigned<P2>(
		self,
		origin: Pointer<T, P2>,
	) -> usize
	where
		P2: Permission,
	{
		unsafe { self.unwrap().offset_from_unsigned(origin.unwrap()) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy.md")]
	/// [`copy_nonoverlapping`]: Self::copy_to_nonoverlapping
	/// [`read`]: Self::read
	pub const unsafe fn copy_to(self, dest: Pointer<T, Unique>, count: usize) {
		unsafe {
			self.unwrap().copy_to(dest.unwrap_mut(), count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy_nonoverlapping.md")]
	/// [`copy`]: Self::copy_to
	/// [`read`]: Self::read
	pub const unsafe fn copy_to_nonoverlapping(
		self,
		dest: Pointer<T, Unique>,
		count: usize,
	) {
		unsafe {
			self.unwrap()
				.copy_to_nonoverlapping(dest.unwrap_mut(), count);
		}
	}
}

impl<T, P> Pointer<[T], P>
where P: Permission
{
	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.len.md")]
	pub const fn len(self) -> usize {
		self.unwrap().len()
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.is_empty.md")]
	pub const fn is_empty(self) -> bool {
		self.len() == 0
	}

	#[inline(always)]
	#[cfg(feature = "rust_193")]
	#[doc = include_str!("../doc/struct.Pointer/method.as_array.md")]
	pub const fn as_array<const N: usize>(self) -> Option<Pointer<[T; N], P>> {
		match self.unwrap().as_array::<N>() {
			| Some(ptr) => Some(Pointer::rewrap(ptr)),
			| None => None,
		}
	}
}

// endregion INHERENT_MIRROR

// region CELL_MIRROR

impl<T> Pointer<UnsafeCell<T>, Shared> {
	/// # Original
	///
	/// [`core::cell::UnsafeCell::raw_get`]
	pub const fn raw_get(self) -> Pointer<T, Unique> {
		Pointer::from_raw_mut(UnsafeCell::raw_get(self.inner))
	}
}

// endregion CELL_MIRROR

// region TRAITS

// mod clone
impl<T, P> Clone for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	fn clone(&self) -> Self {
		*self
	}
}

// mod cmp

impl<T, P> Eq for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

/// Pointer comparison is by address, as produced by the [`addr`](Self::addr)
/// method.
impl<T, P> Ord for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	#[expect(
		ambiguous_wide_pointer_comparisons,
		reason = "the standard library allows it"
	)]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.into_raw_const().cmp(&other.into_raw_const())
	}
}

impl<T, P1, P2> PartialEq<Pointer<T, P2>> for Pointer<T, P1>
where
	T: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline(always)]
	fn eq(&self, rhs: &Pointer<T, P2>) -> bool {
		self.addr() == rhs.addr()
	}
}

/// Pointer comparison is by address, as produced by the [`addr`](Self::addr)
/// method.
impl<T, P1, P2> PartialOrd<Pointer<T, P2>> for Pointer<T, P1>
where
	T: ?Sized,
	P1: Permission,
	P2: Permission,
{
	#[inline]
	#[expect(
		ambiguous_wide_pointer_comparisons,
		reason = "the standard library allows it"
	)]
	fn partial_cmp(&self, other: &Pointer<T, P2>) -> Option<cmp::Ordering> {
		self.into_raw_const().partial_cmp(&other.into_raw_const())
	}
}

// mod convert

impl<T> From<*const T> for Pointer<T, Shared>
where T: ?Sized
{
	#[inline(always)]
	fn from(ptr: *const T) -> Self {
		Self::from_raw_const(ptr)
	}
}

impl<T> From<&T> for Pointer<T, Shared>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &T) -> Self {
		Self::from_raw_const(src)
	}
}

impl<T> From<*mut T> for Pointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	fn from(ptr: *mut T) -> Self {
		Self::from_raw_mut(ptr)
	}
}

impl<T> From<&mut T> for Pointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	fn from(src: &mut T) -> Self {
		Self::from_raw_mut(src)
	}
}

#[cfg(feature = "rust_188")]
impl<T> Default for Pointer<T, Shared> {
	#[inline]
	fn default() -> Self {
		Self::null()
	}
}

#[cfg(feature = "rust_188")]
impl<T> Default for Pointer<T, Unique> {
	#[inline]
	fn default() -> Self {
		Self::null()
	}
}

// mod fmt

impl<T, P> fmt::Debug for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "(*{} {})", P::NAME, any::type_name::<T>())?;
		fmt::Pointer::fmt(self, fmt)
	}
}

impl<T, P> fmt::Pointer for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Pointer::fmt(&self.into_raw_const(), fmt)
	}
}

// mod hash

impl<T, P> hash::Hash for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		core::ptr::hash(self.into_raw_const(), state);
	}
}

// mod marker

impl<T, P> Copy for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

impl<T, P> Unpin for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

// mod panic

impl<T, P> panic::UnwindSafe for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
	<P as permission::Impl>::Ptr<T>: panic::UnwindSafe,
{
}

// endregion TRAITS
