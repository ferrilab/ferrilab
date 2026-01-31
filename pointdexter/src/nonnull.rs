//! Implementation details for [`NonNullPointer`]. See its type documentation..

use core::{
	any,
	cmp,
	fmt,
	hash,
	marker::PhantomData,
	num::NonZero,
	ptr::NonNull,
};

use crate::{
	NonNullPointer,
	NullPointerError,
	Permission,
	Pointer,
	Reference,
	Shared,
	Unique,
};

// region NEW_API

impl<T, P> NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	const fn from_nonnull(nonnull: NonNull<T>) -> Self {
		Self {
			inner: nonnull,
			_perm: PhantomData,
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_reference<'a>(self) -> Reference<'a, T, P> {
		unsafe { core::mem::transmute_copy(&self.inner) }
	}
}

impl<T, P> NonNullPointer<T, P>
where P: Permission
{
	#[inline(always)]
	#[doc = include_str!("../doc/fn.slice_from_raw_parts.md")]
	pub const fn make_slice(self, len: usize) -> NonNullPointer<[T], P> {
		NonNullPointer::slice_from_raw_parts(self, len)
	}
}

// endregion NEW_API

// region STD_UNSIZED

impl<T> NonNullPointer<T, Shared>
where T: ?Sized
{
	/// Converts a reference to a `NonNullPointer`.
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub const fn from_ref(r: &T) -> Self {
		Self::from_nonnull(NonNull::from_ref(r))
	}
}

impl<T> NonNullPointer<T, Unique>
where T: ?Sized
{
	/// Converts a unique reference to a `NonNullPointer`.
	#[inline(always)]
	#[cfg(feature = "rust_189")]
	pub const fn from_mut(r: &mut T) -> Self {
		Self::from_nonnull(NonNull::from_mut(r))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
		unsafe { self.inner.as_mut() }
	}
}

impl<T, P> NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline(always)]
	#[doc = include_str!("../doc/struct.NonNullPointer/method.new_unchecked.md")]
	pub const unsafe fn new_unchecked(ptr: Pointer<T, P>) -> Self {
		Self::from_nonnull(unsafe {
			NonNull::new_unchecked(ptr.into_raw_const().cast_mut())
		})
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.NonNullPointer/method.new.md")]
	pub const fn new(
		ptr: Pointer<T, P>,
	) -> Result<Self, NullPointerError<T, P>> {
		match NonNull::new(ptr.into_raw_const().cast_mut()) {
			| Some(nn) => Ok(Self::from_nonnull(nn)),
			| None => Err(NullPointerError::new()),
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.addr.md")]
	#[doc = "[`map_addr`]: Self::map_addr"]
	#[doc = "[`with_addr`]: Self::with_addr"]
	pub fn addr(self) -> NonZero<usize> {
		self.inner.addr()
	}

	#[inline(always)]
	#[cfg(feature = "rust_189")]
	#[doc = include_str!("../doc/struct.Pointer/method.expose_provenance.md")]
	pub fn expose_provenance(self) -> NonZero<usize> {
		self.inner.expose_provenance()
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.with_addr.md")]
	#[doc = "[`wrapping_offset`]: Pointer::wrapping_offset"]
	pub fn with_addr(self, addr: NonZero<usize>) -> Self {
		Self::from_nonnull(self.inner.with_addr(addr))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.map_addr.md")]
	#[doc = "[`with_addr`]: Self::with_addr"]
	pub fn map_addr(
		self,
		func: impl FnOnce(NonZero<usize>) -> NonZero<usize>,
	) -> Self {
		Self::from_nonnull(self.inner.map_addr(func))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.NonNullPointer/method.as_ptr.md")]
	pub const fn as_ptr(self) -> Pointer<T, P> {
		Pointer::rewrap_mut(self.inner.as_ptr())
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.as_reference.md")]
	pub const unsafe fn as_ref<'a>(&self) -> &'a T {
		unsafe { self.inner.as_ref() }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.cast.md")]
	pub const fn cast<U>(self) -> NonNullPointer<U, P> {
		NonNullPointer::from_nonnull(self.inner.cast::<U>())
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_offset.md")]
	pub const unsafe fn byte_offset(self, count: isize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_offset(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_add.md")]
	pub const unsafe fn byte_add(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_add(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_sub.md")]
	pub const unsafe fn byte_sub(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.byte_sub(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_offset_from.md")]
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

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	#[doc = include_str!("../doc/struct.Pointer/method.byte_offset_from_unsigned.md")]
	pub const unsafe fn byte_offset_from_unsigned<Q>(
		self,
		subtracted: NonNullPointer<T, Q>,
	) -> usize
	where
		Q: Permission,
	{
		unsafe { self.inner.byte_offset_from_unsigned(subtracted.inner) }
	}
}

impl<T> NonNullPointer<T, Unique>
where T: ?Sized
{
	#[inline(always)]
	#[doc = include_str!("../doc/fn.drop_in_place.md")]
	pub unsafe fn drop_in_place(self) {
		unsafe {
			self.inner.drop_in_place();
		}
	}
}

// endregion STD_UNSIZED

// region STD_SIZED

impl<T, P> NonNullPointer<T, P>
where P: Permission
{
	#[inline(always)]
	#[doc = include_str!("../doc/fn.dangling.md")]
	pub const fn dangling() -> Self {
		Self::from_nonnull(NonNull::dangling())
	}

	#[inline(always)]
	#[cfg(feature = "rust_189")]
	#[doc = include_str!("../doc/fn.without_provenance.md")]
	#[doc = "[`with_exposed_provenance`]: Self::with_exposed_provenance"]
	pub const fn without_provenance(addr: NonZero<usize>) -> Self {
		Self::from_nonnull(NonNull::without_provenance(addr))
	}

	#[inline(always)]
	#[cfg(feature = "rust_189")]
	#[doc = include_str!("../doc/fn.with_exposed_provenance.md")]
	#[doc = "[`expose_provenance`]: Self::expose_provenance"]
	#[doc = "[`with_addr`]: Self::with_addr"]
	#[doc = "[`without_provenance`]: Self::without_provenance"]
	pub fn with_exposed_provenance(addr: NonZero<usize>) -> Self {
		Self::from_nonnull(NonNull::with_exposed_provenance(addr))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.offset.md")]
	#[doc = "[`wrapping_offset`]: Pointer::wrapping_offset"]
	pub const unsafe fn offset(self, count: isize) -> Self {
		Self::from_nonnull(unsafe { self.inner.offset(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.add.md")]
	#[doc = "[`offset`]: Self::offset"]
	#[doc = "[`wrapping_add`]: Pointer::wrapping_add"]
	pub const unsafe fn add(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.add(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.sub.md")]
	#[doc = "[`offset`]: Self::offset"]
	#[doc = "[`wrapping_sub`]: Pointer::wrapping_sub"]
	pub const unsafe fn sub(self, count: usize) -> Self {
		Self::from_nonnull(unsafe { self.inner.sub(count) })
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.offset_from.md")]
	pub const unsafe fn offset_from<Q>(
		self,
		origin: NonNullPointer<T, Q>,
	) -> isize
	where
		Q: Permission,
	{
		unsafe { self.inner.offset_from(origin.inner) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_187")]
	#[doc = include_str!("../doc/struct.Pointer/method.offset_from_unsigned.md")]
	pub const unsafe fn offset_from_unsigned<Q>(
		self,
		subtracted: NonNullPointer<T, Q>,
	) -> usize
	where
		Q: Permission,
	{
		unsafe { self.inner.offset_from_unsigned(subtracted.inner) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.read.md")]
	#[doc = "[`read_unaligned`]: Self::read_unaligned"]
	pub const unsafe fn read(self) -> T {
		unsafe { self.inner.read() }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.read_volatile.md")]
	#[doc = "[`read`]: Self::read"]
	#[cfg_attr(
		not(feature = "rust_189"),
		doc = "[`without_provenance`]: Pointer::without_provenance"
	)]
	#[cfg_attr(
		feature = "rust_189",
		doc = "[`without_provenance`]: Self::without_provenance"
	)]
	pub unsafe fn read_volatile(self) -> T {
		unsafe { self.inner.read_volatile() }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.read_unaligned.md")]
	#[doc = "[`read`]: Self::read"]
	pub const unsafe fn read_unaligned(self) -> T {
		unsafe { self.inner.read_unaligned() }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy.md")]
	#[doc = "[`copy_nonoverlapping`]: Self::copy_to_nonoverlapping"]
	#[doc = "[`read`]: Self::read"]
	pub const unsafe fn copy_to(
		self,
		dest: NonNullPointer<T, Unique>,
		count: usize,
	) {
		unsafe {
			self.inner.copy_to(dest.inner, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy_nonoverlapping.md")]
	#[doc = "[`copy`]: Self::copy_to`"]
	#[doc = "[`read`]: Self::read"]
	pub const unsafe fn copy_to_nonoverlapping(
		self,
		dest: NonNullPointer<T, Unique>,
		count: usize,
	) {
		unsafe {
			self.inner.copy_to_nonoverlapping(dest.inner, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.align_offset.md")]
	#[doc = "[`wrapping_add`]: Pointer::wrapping_add"]
	pub fn align_offset(self, align: usize) -> usize {
		self.inner.align_offset(align)
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.is_aligned.md")]
	pub fn is_aligned(self) -> bool {
		self.inner.is_aligned()
	}
}

impl<T> NonNullPointer<T, Unique> {
	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy.md")]
	#[doc = "[`copy_nonoverlapping`]: Self::copy_from_nonoverlapping"]
	#[doc = "[`read`]: Self::read"]
	pub const unsafe fn copy_from<P>(
		self,
		src: NonNullPointer<T, P>,
		count: usize,
	) where
		P: Permission,
	{
		unsafe {
			self.inner.copy_from(src.inner, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.copy_nonoverlapping.md")]
	#[doc = "[`copy`]: Self::copy_from`"]
	#[doc = "[`read`]: Self::read"]
	pub const unsafe fn copy_from_nonoverlapping<P>(
		self,
		src: NonNullPointer<T, P>,
		count: usize,
	) where
		P: Permission,
	{
		unsafe {
			self.inner.copy_from_nonoverlapping(src.inner, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write.md")]
	#[doc = "[`read`]: Self::read"]
	#[doc = "[`write_unaligned`]: Self::write_unaligned"]
	pub const unsafe fn write(self, val: T) {
		unsafe {
			self.inner.write(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_bytes.md")]
	pub const unsafe fn write_bytes(self, val: u8, count: usize) {
		unsafe {
			self.inner.write_bytes(val, count);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_volatile.md")]
	pub unsafe fn write_volatile(self, val: T) {
		unsafe {
			self.inner.write_volatile(val);
		}
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.write_unaligned.md")]
	#[doc = "[`read_unaligned`]: Self::read_unaligned"]
	pub const unsafe fn write_unaligned(self, val: T) {
		unsafe {
			self.inner.write_unaligned(val);
		}
	}

	#[inline(always)]
	#[cfg(not(feature = "rust_188"))]
	#[doc = include_str!("../doc/fn.replace.md")]
	pub unsafe fn replace(self, src: T) -> T {
		unsafe { self.inner.replace(src) }
	}

	#[inline(always)]
	#[cfg(feature = "rust_188")]
	#[doc = include_str!("../doc/fn.replace.md")]
	pub const unsafe fn replace(self, src: T) -> T {
		unsafe { self.inner.replace(src) }
	}

	#[inline(always)]
	#[doc = include_str!("../doc/fn.swap.md")]
	pub const unsafe fn swap(self, with: Self) {
		unsafe {
			self.inner.swap(with.inner);
		}
	}
}

// endregion STD_SIZED

// region STD_SLICE

impl<T, P> NonNullPointer<[T], P>
where P: Permission
{
	#[inline(always)]
	#[doc = include_str!("../doc/fn.slice_from_raw_parts.md")]
	pub const fn slice_from_raw_parts(
		data: NonNullPointer<T, P>,
		len: usize,
	) -> Self {
		Self::from_nonnull(NonNull::slice_from_raw_parts(data.inner, len))
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.len.md")]
	pub const fn len(self) -> usize {
		self.inner.len()
	}

	#[inline(always)]
	#[doc = include_str!("../doc/struct.Pointer/method.is_empty.md")]
	pub const fn is_empty(self) -> bool {
		self.inner.is_empty()
	}
}

// endregion STD

// region TRAITS

impl<T, P> Clone for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
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
	#[expect(
		ambiguous_wide_pointer_comparisons,
		reason = "the standard library allows it"
	)]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.inner.cmp(&other.inner)
	}
}

impl<T, P1, P2> PartialEq<NonNullPointer<T, P2>> for NonNullPointer<T, P1>
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
	fn eq(&self, other: &NonNullPointer<T, P2>) -> bool {
		self.inner.eq(&other.inner)
	}
}

impl<T, P1, P2> PartialOrd<NonNullPointer<T, P2>> for NonNullPointer<T, P1>
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
	fn partial_cmp(
		&self,
		other: &NonNullPointer<T, P2>,
	) -> Option<cmp::Ordering> {
		self.inner.partial_cmp(&other.inner)
	}
}

impl<T> From<&T> for NonNullPointer<T, Shared> {
	#[inline]
	fn from(src: &T) -> Self {
		Self::from_nonnull(NonNull::from(src))
	}
}
impl<T> From<&mut T> for NonNullPointer<T, Unique> {
	#[inline]
	fn from(src: &mut T) -> Self {
		Self::from_nonnull(NonNull::from(src))
	}
}

impl<T, P> fmt::Debug for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "NonNull<*{} {}>", P::NAME, any::type_name::<T>())?;
		fmt.debug_tuple("").field(&self.inner).finish()
	}
}

impl<T, P> fmt::Pointer for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Pointer::fmt(&self.as_ptr().into_raw_const(), fmt)
	}
}

impl<T, P> hash::Hash for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T, P> Copy for NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

// All pointers are !Send and !Sync, so no overrides may be done here.

// endregion TRAITS
