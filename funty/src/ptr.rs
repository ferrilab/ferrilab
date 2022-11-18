#![doc = include_str!("../doc/ptr.md")]

use core::{
	any,
	cmp,
	fmt,
	hash::{
		Hash,
		Hasher,
	},
	marker::PhantomData,
	ptr::{
		self,
		NonNull,
	},
};

#[doc = include_str!("../doc/permission.md")]
pub trait Permission {
	/// The raw pointer type that drives memory accesses for this `Permission`.
	/// This is necessary because `*const T` and `*mut T` are different types,
	/// and tools such as Miri track how `*mut` pointers are created, and
	/// by using an associated type we can avoid improperly using a `*mut T`
	/// pointer inside a `Pointer<T, Shared>` that never had unique access
	/// permissions.
	type Ptr<T: ?Sized>: RawPtr<T>;

	/// The corresponding reference type.
	type Ref<'a, T: 'a + ?Sized>: RawRef<'a, T>;

	/// Either `"*const"` or `"*mut"`; used for debug printing.
	const DEBUG_PREFIX: &'static str;

	// These are utility functions that should not be called directly by public
	// consumers. They exist to provide specific behaviors that the `Pointer`
	// type needs, but cannot directly express.

	/// Casts the referent type of the pointer.
	///
	/// This can be used to cast a pointer-to-unsized to pointer-to-sized, but
	/// cannot be used to cast any pointer to pointer-to-unsized. Pointers to
	/// unsized types (currently only slices and trait objects) must be created
	/// through dedicated APIs which are not yet stabilized on the raw pointer
	/// types.
	#[doc(hidden)]
	fn cast<T: ?Sized, U>(ptr: Self::Ptr<T>) -> Self::Ptr<U>;

	/// Creates a raw `*const T` primitive.
	#[doc(hidden)]
	fn into_const<T: ?Sized>(ptr: Self::Ptr<T>) -> *const T;

	/// Attempts to create a raw `*mut T` primitive.
	///
	/// This is only allowed to succeed where `Self::Ptr<T>` is `*mut T`.
	#[doc(hidden)]
	fn try_into_mut<T: ?Sized>(ptr: Self::Ptr<T>) -> Option<*mut T>;

	/// Attempts to unwind the permission stack until it bottoms out at a raw
	/// mutable pointer.
	///
	/// The `Shared` implementation returns `None`, the `Unique` implementation
	/// returns `Some`, and the `(Shared, P)` implementation recurses into `P`.
	#[doc(hidden)]
	fn unwind_to_unique<T: ?Sized>(ptr: Self::Ptr<T>) -> Option<*mut T>;

	/// Wraps a raw `*const T` primitive as the `Raw` associated type.
	#[doc(hidden)]
	fn from_const<T: ?Sized>(ptr: *const T) -> Self::Ptr<T>;

	/// Dereferences a pointer and produces a reference to the pointed-to value.
	///
	/// ## Safety
	///
	/// The pointer must be non-null and well-aligned for `T`, and the
	/// pointed-to location must outlive the conjured `'a` lifetime and not have
	/// any other outstanding references which violate the Rust
	/// many-shared-xor-one-mutable reference rule.
	#[doc(hidden)]
	unsafe fn ptr_to_ref<'a, T: 'a + ?Sized>(
		ptr: Self::Ptr<T>,
	) -> Self::Ref<'a, T>;

	/// Combines a base pointer and a length into a slice pointer.
	///
	/// ## Safety
	///
	/// The region `ptr[.. len]` must be within a single provenance region.
	#[doc(hidden)]
	unsafe fn ptr_to_slice<T: Sized>(
		ptr: Self::Ptr<T>,
		len: usize,
	) -> Self::Ptr<[T]>;

	/// Gets the raw address of a pointer.
	#[doc(hidden)]
	fn addr<T: ?Sized>(ptr: Self::Ptr<T>) -> usize;
}

/// Equivalent to `*const` pointers or `&` references.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Shared;

/// Equivalent to `*mut` pointers or `&mut` references.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unique;

impl Permission for Shared {
	type Ptr<T: ?Sized> = *const T;
	type Ref<'a, T: 'a + ?Sized> = &'a T;

	const DEBUG_PREFIX: &'static str = "*const";

	fn cast<T: ?Sized, U>(ptr: *const T) -> *const U {
		ptr.cast::<U>()
	}

	fn into_const<T: ?Sized>(ptr: *const T) -> *const T {
		ptr
	}

	fn try_into_mut<T: ?Sized>(_: *const T) -> Option<*mut T> {
		None
	}

	fn unwind_to_unique<T: ?Sized>(_: *const T) -> Option<*mut T> {
		None
	}

	fn from_const<T: ?Sized>(ptr: *const T) -> *const T {
		ptr
	}

	unsafe fn ptr_to_ref<'a, T: 'a + ?Sized>(ptr: *const T) -> &'a T {
		&*ptr
	}

	unsafe fn ptr_to_slice<T: Sized>(ptr: *const T, len: usize) -> *const [T] {
		ptr::slice_from_raw_parts(ptr, len)
	}

	fn addr<T: ?Sized>(ptr: *const T) -> usize {
		ptr.cast::<()>() as usize
	}
}

impl Permission for Unique {
	type Ptr<T: ?Sized> = *mut T;
	type Ref<'a, T: 'a + ?Sized> = &'a mut T;

	const DEBUG_PREFIX: &'static str = "*mut";

	fn cast<T: ?Sized, U>(ptr: *mut T) -> *mut U {
		ptr.cast::<U>()
	}

	fn into_const<T: ?Sized>(ptr: *mut T) -> *const T {
		ptr.cast_const()
	}

	fn try_into_mut<T: ?Sized>(ptr: *mut T) -> Option<*mut T> {
		Some(ptr)
	}

	fn unwind_to_unique<T: ?Sized>(ptr: *mut T) -> Option<*mut T> {
		Some(ptr)
	}

	fn from_const<T: ?Sized>(ptr: *const T) -> *mut T {
		ptr.cast_mut()
	}

	unsafe fn ptr_to_ref<'a, T: 'a + ?Sized>(ptr: *mut T) -> &'a mut T {
		&mut *ptr
	}

	unsafe fn ptr_to_slice<T: Sized>(ptr: *mut T, len: usize) -> *mut [T] {
		ptr::slice_from_raw_parts_mut(ptr, len)
	}

	fn addr<T: ?Sized>(ptr: *mut T) -> usize {
		ptr.cast::<()>() as usize
	}
}

impl<P> Permission for (Shared, P)
where P: Permission
{
	type Ptr<T: ?Sized> = *const T;
	type Ref<'a, T: 'a + ?Sized> = &'a T;

	const DEBUG_PREFIX: &'static str = Shared::DEBUG_PREFIX;

	fn cast<T: ?Sized, U>(ptr: *const T) -> *const U {
		ptr.cast::<U>()
	}

	fn into_const<T: ?Sized>(ptr: *const T) -> *const T {
		ptr
	}

	fn try_into_mut<T: ?Sized>(_: *const T) -> Option<*mut T> {
		None
	}

	fn unwind_to_unique<T: ?Sized>(ptr: *const T) -> Option<*mut T> {
		P::unwind_to_unique(P::from_const(ptr))
	}

	fn from_const<T: ?Sized>(ptr: *const T) -> *const T {
		ptr
	}

	unsafe fn ptr_to_ref<'a, T: 'a + ?Sized>(ptr: *const T) -> &'a T {
		&*ptr
	}

	unsafe fn ptr_to_slice<T: Sized>(ptr: *const T, len: usize) -> *const [T] {
		ptr::slice_from_raw_parts(ptr, len)
	}

	fn addr<T: ?Sized>(ptr: *const T) -> usize {
		ptr.cast::<()>() as usize
	}
}

#[repr(transparent)]
#[doc = include_str!("../doc/pointers.md")]
pub struct Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	ptr: P::Ptr<T>,
}

impl<T> Pointer<T, Shared>
where T: ?Sized
{
	/// Produces a new `Pointer` from a raw `const` pointer.
	pub const fn new(ptr: *const T) -> Self {
		Self { ptr }
	}

	/// Produces the enclosed raw pointer.
	pub const fn into_inner(self) -> *const T {
		self.ptr
	}
}

impl<T> Pointer<T, Unique>
where T: ?Sized
{
	/// Produces a new `Pointer` from a raw `mut` pointer.
	pub const fn new(ptr: *mut T) -> Self {
		Self { ptr }
	}

	/// Produces the enclosed raw pointer.
	pub const fn into_inner(self) -> *mut T {
		self.ptr
	}

	/// Produces a unique reference to the pointed-to value.
	///
	/// ## Safety
	///
	/// This function requires the following:
	///
	/// - the `Pointer` is well-aligned for `T`.
	/// - the pointed-to location contains an initialized `T` value.
	/// - no other reference or `Reference` pointing to the location exists for
	///   the duration of the produced reference’s existence.
	/// - the pointed-to location exists for the full duration of the conjured
	///   `'a` lifetime.
	pub unsafe fn as_mut<'a>(self) -> Option<&'a mut T> {
		if self.is_null() {
			return None;
		}
		Some(&mut *self.ptr)
	}

	/// Runs the destructor on the pointed-to location.
	///
	/// ## Safety
	///
	/// The value at the pointed-to location must be currently initialized.
	pub unsafe fn drop_in_place(self) {
		self.ptr.drop_in_place();
	}
}

impl<T> Pointer<T, Unique>
where T: Sized
{
	/// Copies objects pointed to by `src` into the region pointed to by
	/// `self`.
	///
	/// ## Safety
	///
	/// - the memory region of `src[.. count]` must contain contiguous
	///   fully-initialized `T` values.
	/// - the memory region of `self[.. count]` must be within a single
	///   allocation.
	/// - the values stored in `src[.. count]` must not have their destructors
	///   run, as they are now alive in `self[.. count]`.
	pub unsafe fn copy_from<P: Permission>(
		self,
		src: Pointer<T, P>,
		count: usize,
	) {
		self.ptr.copy_from(
			<(Shared, P)>::into_const::<T>(src.cast_const().ptr),
			count,
		);
	}

	/// Same as `copy_from`, except that you guarantee that the memory regions
	/// `self[.. count]` and `src[.. count]` are fully disjoint.
	///
	/// ## Safety
	///
	/// The copying behavior from the source region to the self region is not
	/// specified. As such, the two regions **must not** overlap at all in the
	/// memory space.
	pub unsafe fn copy_from_nonoverlapping<P: Permission>(
		self,
		src: Pointer<T, P>,
		count: usize,
	) {
		self.ptr.copy_from_nonoverlapping(
			<(Shared, P)>::into_const::<T>(src.cast_const().ptr),
			count,
		);
	}

	/// Writes a new value into the pointed-to location.
	///
	/// The pointed-to value does not have its destructor run.
	///
	/// ## Safety
	///
	/// You must ensure that the pointed-to value is properly destroyed before
	/// overwriting it.
	pub unsafe fn write(self, value: T) {
		self.ptr.write(value);
	}

	/// Fills the memory region with a byte pattern for `count` *elements* (not
	/// bytes).
	///
	/// ## Safety
	///
	/// You must ensure that the values stored in `self[.. count]` are properly
	/// destroyed before calling this function, and that the byte pattern
	/// `[byte; mem::size_of::<T>()]` is a valid object representation for `T`
	/// values.
	pub unsafe fn write_bytes(self, byte: u8, count: usize) {
		self.ptr.write_bytes(byte, count);
	}

	/// Equivalent to `write`, with the additional guarantee that the *compiler*
	/// will not omit this store.
	///
	/// This store does not synchronize across threads, nor does it create a
	/// causal dependency with any other memory accesses in the current thread.
	/// Its only guarantee is that the compiler will emit a store instruction.
	///
	/// ## Safety
	///
	/// See `write`.
	pub unsafe fn write_volatile(self, value: T) {
		self.ptr.write_volatile(value);
	}

	/// Equivalent to `write`, except that it is tolerant of the address not
	/// being well-aligned for `T`.
	///
	/// ## Safety
	///
	/// See `write`.
	pub unsafe fn write_unaligned(self, value: T) {
		self.ptr.write_unaligned(value);
	}

	/// Exchanges the argument with the pointed-to value.
	///
	/// ## Safety
	///
	/// The pointed-to value must be properly initialized, and no other
	/// reference to it may exist when this function is called.
	pub unsafe fn replace(self, value: T) -> T {
		self.ptr.replace(value)
	}

	/// Exchanges the pointed-to values between this and another `Pointer`.
	///
	/// ## Safety
	///
	/// The pointed-to values must be properly initialized, and no other
	/// references to them may exist when this function is called.
	pub unsafe fn swap(self, with: Self) {
		self.ptr.swap(with.ptr);
	}
}

impl<T, P> Pointer<T, (Shared, P)>
where
	T: ?Sized,
	P: Permission,
{
	/// Removes the leading `Shared` from the permission stack.
	pub fn cast_unshared(self) -> Pointer<T, P> {
		Pointer {
			ptr: <P>::from_const(self.into_const_ptr()),
		}
	}

	/// Recursively unwinds the permission stack until it bottoms out at
	/// `Unique`. Returns `None` if the base permission is `Shared`.
	pub fn unwind_to_unique(self) -> Option<Pointer<T, Unique>> {
		P::unwind_to_unique(self.cast_unshared().ptr)
			.map(Pointer::<T, Unique>::new)
	}
}

impl<T, P> Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Constructs a `Pointer` from its enclosed associated type.
	///
	/// This takes a raw pointer when `P` is known, but is only the `P::Ptr`
	/// associated type when `P` is generic.
	pub fn from_ptr(ptr: P::Ptr<T>) -> Self {
		Self { ptr }
	}

	/// Converts this `Pointer` to its enclosed associated type.
	///
	/// This is a raw pointer when `P` is known, but remains the `P::Ptr`
	/// associated type when `P` is generic.
	pub fn as_ptr(self) -> P::Ptr<T> {
		self.ptr
	}

	/// Produces a raw immutable pointer.
	pub fn into_const_ptr(self) -> *const T {
		P::into_const::<T>(self.ptr)
	}

	/// Attempts to produce a raw mutable pointer.
	///
	/// Returns `None` when `P` is not `Unique`.
	pub fn try_into_mut_ptr(self) -> Option<*mut T> {
		P::try_into_mut::<T>(self.ptr)
	}

	/// Tests if this points to the null address.
	pub fn is_null(self) -> bool {
		self.ptr.is_null()
	}

	/// Changes the apparent type of the pointed-to value.
	///
	/// Pointer metadata such as region length or trait vtable can only be
	/// discarded, not transformed or conjured, by this function.
	pub fn cast<U: Sized>(self) -> Pointer<U, P> {
		let Self { ptr } = self;
		Pointer {
			ptr: <P as Permission>::cast::<T, U>(ptr),
		}
	}

	/// Creates a shared reference to the pointed-to value.
	///
	/// ## Safety
	///
	/// This function requires the following:
	///
	/// - the `Pointer` is well-aligned for `T`.
	/// - the pointed-to location contains an initialized `T` value.
	/// - no `&mut T` or `Reference<T, Unique>` pointing to the location exists
	///   for the duration of the produced reference’s existence.
	/// - the pointed-to location exists for the full duration of the conjured
	///   `'a` lifetime.
	pub unsafe fn as_ref<'a>(self) -> Option<&'a T> {
		if self.is_null() {
			return None;
		};
		Some(&*P::into_const(self.ptr))
	}

	/// Produces a reference to the pointed-to value.
	///
	/// ## Safety
	///
	/// This function requires the following:
	///
	/// - the `Pointer` is well-aligned for `T`.
	/// - the pointed-to location contains an initialized `T` value.
	/// - if `P` is `Unique`, no other reference to the pointed-to value may
	///   exist for the duration of the produced reference’s existence.
	/// - if `P` is `Shared`, then no `Unique` or `&mut` reference to the
	///   pointed-to value may exist for the duration.
	/// - the pointed-to location exists for the full duration of the conjured
	///   `'a` lifetime.
	pub unsafe fn as_reference<'a>(self) -> Option<Reference<'a, T, P>> {
		if self.is_null() {
			return None;
		}
		Some(P::ptr_to_ref(self.ptr))
	}

	/// Forcibly converts this to a pointer with `Shared` permissions.
	///
	/// This is always safe to do.
	pub fn cast_const(self) -> Pointer<T, Shared> {
		Pointer {
			ptr: self.into_const_ptr(),
		}
	}

	/// Forcibly converts this to a pointer with `Unique` permissions.
	///
	/// ## Safety
	///
	/// You must ensure that this pointer was originally drawn from a region
	/// with `*mut T` permissions. The penalty for violation of this rule on
	/// memory access is currently not specified, but cannot be relied upon to
	/// always work.
	pub unsafe fn cast_mut(self) -> Pointer<T, Unique> {
		Pointer {
			ptr: self.into_const_ptr().cast_mut(),
		}
	}

	/// Degrades the pointer to no longer have `Unique` permissions.
	///
	/// This prepends `Shared` to the `Permission` stack. The resulting pointer
	/// can remove this `Shared` and restore the current permission stack by
	/// calling `.cast_unshared()`.
	///
	/// Only the permission stack `(Shared, Unique)` has the method
	/// `.cast_mut()`, which directly produces a `Pointer<T, Unique>`.
	pub fn cast_shared(self) -> Pointer<T, (Shared, P)> {
		let ptr = P::into_const(self.ptr);
		let ptr = <(Shared, P)>::from_const(ptr);
		Pointer { ptr }
	}

	/// Gets the raw value of the address to which this points.
	pub fn addr(self) -> usize {
		P::addr::<T>(self.ptr)
	}
}

impl<T, P> Pointer<T, P>
where
	T: Sized,
	P: Permission,
{
	/// Produces the canonical dangling pointer for `T`.
	///
	/// Note that the dangling pointer is always considered to be conjured from
	/// nothing and thus does not have provenance, so this pointer cannot be
	/// dereferenced until it has been overwritten with a different value.
	pub fn dangling() -> Self {
		Self {
			ptr: P::from_const(NonNull::<T>::dangling().as_ptr().cast_const()),
		}
	}

	/// Tests if this points to the canonical “sentinel” address for a type.
	///
	/// Since most targets Rust supports do not permit addressing the zero page,
	/// Rust considers addresses which match the alignment of a type (that is,
	/// the addresses `0x1`, `0x2`, `0x4`, `0x8`, `0x10`, ... probably `0x1000`)
	/// to be “unlikely” to point to validly-initialized or dereferenceable
	/// memory, and uses these values to indicate that some structure which
	/// contains a pointer has been validly initialized, but has not been
	/// granted a region of memory to which it can usefully point.
	///
	/// This is a convenience function to support that use case, but should not
	/// be considered to indicate anything else about the pointed-to location.
	/// This behavior is determined entirely by the user, not by this type.
	pub fn is_dangling(self) -> bool {
		self == Self::dangling()
	}

	/// Adjusts the memory address by some number of `T` elements.
	///
	/// ## Safety
	///
	/// The resulting pointer must be within the same provenance region as the
	/// source pointer.
	pub unsafe fn offset(self, by: isize) -> Self {
		Self {
			ptr: self.ptr.offset(by),
		}
	}

	/// Adjusts the memory address by some number of `T` elements.
	///
	/// Note: the resulting pointer is permitted to be outside the provenance
	/// region of the source pointer. However, it will not be safe to
	/// dereference until it has been brought back within the original
	/// provenance bounds.
	pub fn wrapping_offset(self, by: isize) -> Self {
		Self {
			ptr: self.ptr.wrapping_offset(by),
		}
	}

	/// Produces the number of `T` elements between this pointer and the
	/// provided origin.
	///
	/// This is positive when `self` is higher in the memory space than `origin`
	/// and negative when `self` is lower in the memory space than `origin`.
	///
	/// # Safety
	///
	/// Both pointers must be within the same provenance region. This is most
	/// likely to be the case when `self` has been produced by calling
	/// `origin.offset()`.
	pub unsafe fn offset_from(self, origin: Self) -> isize {
		self.ptr.offset_from(origin.ptr)
	}

	/// Adjusts the memory address upwards in the memory space by some number of
	/// `T` elements.
	///
	/// ## Safety
	///
	/// The resulting pointer must be within the same provenance region as the
	/// source pointer.
	pub unsafe fn add(self, count: usize) -> Self {
		debug_assert!(
			count < (isize::MAX as usize),
			"Addend ({}) exceeds maximum ({})",
			count,
			isize::MAX
		);
		self.offset(count as isize)
	}

	/// Adjusts the memory address downwards in the memory space by some number
	/// of `T` elements.
	///
	/// ## Safety
	///
	/// The resulting pointer must be within the same provenance region as the
	/// source pointer.
	pub unsafe fn sub(self, count: usize) -> Self {
		debug_assert!(
			count < (isize::MAX as usize),
			"Subtrahend ({}) exceeds maximum ({})",
			count,
			isize::MAX
		);
		self.offset(-(count as isize))
	}

	/// Adjusts the memory address upwards in the memory space by some number of
	/// `T` elements.
	///
	/// Note: the resulting pointer is permitted to be outside the provenance
	/// region of the source pointer. However, it will not be safe to
	/// dereference until it has been brought back within the original
	/// provenance bounds.
	pub fn wrapping_add(self, count: usize) -> Self {
		debug_assert!(
			count < (isize::MAX as usize),
			"Addend ({}) exceeds maximum ({})",
			count,
			isize::MAX
		);
		self.wrapping_offset(count as isize)
	}

	/// Adjusts the memory address downwards in the memory space by some number
	/// of `T` elements.
	///
	/// Note: the resulting pointer is permitted to be outside the provenance
	/// region of the source pointer. However, it will not be safe to
	/// dereference until it has been brought back within the original
	/// provenance bounds.
	pub fn wrapping_sub(self, count: usize) -> Self {
		debug_assert!(
			count < (isize::MAX as usize),
			"Subtrahend ({}) exceeds maximum ({})",
			count,
			isize::MAX
		);
		self.wrapping_offset(-(count as isize))
	}

	/// Reads the value out of the pointed-to location.
	///
	/// ## Safety
	///
	/// The pointed-to location is now de-initialized, and must not have its
	/// destructor run unless the location is re-initialized with a new value,
	/// such as through `write`.
	pub unsafe fn read(self) -> T {
		self.ptr.read()
	}

	/// Equivalent to `read`, with the additional guarantee that the *compiler*
	/// will not omit this load.
	///
	/// This load does not synchronize across threads, nor does it create a
	/// causal dependency with any other memory accesses in the current thread.
	/// Its only guarantee is that the compiler will emit a load instruction.
	///
	/// ## Safety
	///
	/// See `read`.
	pub unsafe fn read_volatile(self) -> T {
		self.ptr.read_volatile()
	}

	/// Equivalent to `read`, except that it is tolerant of the address not
	/// being well-aligned for `T`.
	///
	/// ## Safety
	///
	/// See `read`.
	pub unsafe fn read_unaligned(self) -> T {
		self.ptr.read_unaligned()
	}

	/// Copies objects pointed to by `self` into the region pointed to by
	/// `dest`.
	///
	/// ## Safety
	///
	/// - the memory region of `self[.. count]` must contain contiguous
	///   fully-initialized `T` values.
	/// - the memory region of `dest[.. count]` must be within a single
	///   allocation.
	///
	/// ## Non-Effects
	///
	/// This does not run destructors on values stored in `dest[.. count]`, and
	/// any values stored there will become leaked.
	pub unsafe fn copy_to(self, dest: Pointer<T, Unique>, count: usize) {
		self.ptr.copy_to(dest.ptr, count)
	}

	/// Same as `copy_t`, except that you guarantee that the memory regions
	/// `self[.. count]` and `dest[.. count`] are fully disjoint.
	///
	/// ## Safety
	///
	/// The copying behavior from the self region to the destination region is
	/// not specified. As such, the two regions **must not** overlap at all in
	/// the memory space.
	pub unsafe fn copy_to_nonoverlapping(
		self,
		dest: Pointer<T, Unique>,
		count: usize,
	) {
		self.ptr.copy_to_nonoverlapping(dest.ptr, count)
	}

	/// Computes the positive offset, measured in `T` elements, that must be
	/// applied to this pointer in order to bring its address to the requested
	/// alignment.
	///
	/// The requested alignment is measured in bytes, and is most likely to be
	/// produced by calling `mem::align_of::<U>()`.
	pub fn align_offset(self, align: usize) -> usize {
		self.ptr.align_offset(align)
	}

	/// Converts this into a slice pointer, beginning at the pointed-to element
	/// and extending upwards in memory for `len` total elements.
	///
	/// ## Safety
	///
	/// `self[.. len]` must be within a single contiguous provenance region.
	pub unsafe fn make_slice(self, len: usize) -> Pointer<[T], P> {
		Pointer::from_raw_parts(self, len)
	}
}

impl<T, P> Pointer<[T], P>
where
	T: Sized,
	P: Permission,
{
	/// Produces a slice pointer from its raw parts.
	///
	/// ## Safety
	///
	/// The provided element pointer must point to a region of memory that is
	/// within a single allocation for at least `len` contiguous elements of
	/// `T`. The pointed-to region does not need to be initialized.
	pub unsafe fn from_raw_parts(ptr: Pointer<T, P>, len: usize) -> Self {
		Self {
			ptr: P::ptr_to_slice::<T>(ptr.ptr, len),
		}
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
		fmt::Debug::fmt(&P::into_const(self.ptr), fmt)
	}
}

impl<T, P> fmt::Pointer for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt::Pointer::fmt(&P::into_const(self.ptr), fmt)
	}
}

impl<T, P> Clone for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
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
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		let this = P::into_const(self.ptr);
		let that = P::into_const(other.ptr);
		this.cmp(&that)
	}
}

impl<T, P1, P2> PartialEq<Pointer<T, P2>> for Pointer<T, P1>
where
	T: ?Sized,
	P1: Permission,
	P2: Permission,
{
	fn eq(&self, other: &Pointer<T, P2>) -> bool {
		let this = P1::into_const(self.ptr);
		let that = P2::into_const(other.ptr);
		this == that
	}
}

impl<T, P1, P2> PartialOrd<Pointer<T, P2>> for Pointer<T, P1>
where
	T: ?Sized,
	P1: Permission,
	P2: Permission,
{
	fn partial_cmp(&self, other: &Pointer<T, P2>) -> Option<cmp::Ordering> {
		let this = P1::into_const(self.ptr);
		let that = P2::into_const(other.ptr);
		this.partial_cmp(&that)
	}
}

impl<T> From<*const T> for Pointer<T, Shared>
where T: ?Sized
{
	fn from(src: *const T) -> Self {
		Self::new(src)
	}
}

impl<T> From<&T> for Pointer<T, Shared>
where T: ?Sized
{
	fn from(src: &T) -> Self {
		Self::new(src)
	}
}

impl<T> From<*mut T> for Pointer<T, Unique>
where T: ?Sized
{
	fn from(src: *mut T) -> Self {
		Self::new(src)
	}
}

impl<T> From<&mut T> for Pointer<T, Unique>
where T: ?Sized
{
	fn from(src: &mut T) -> Self {
		Self::new(src)
	}
}

impl<T, P> Hash for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		P::into_const(self.ptr).hash(hasher);
	}
}

impl<T, P> Copy for Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

#[repr(transparent)]
#[doc = include_str!("../doc/nonnull.md")]
pub struct NonNullPtr<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// The enclosed non-null pointer. Only the standard library can guarantee
	/// the presence of niches on types, or this type would just wrap `Pointer`.
	inner: NonNull<T>,
	/// Remember the `Permission`.
	_perm: PhantomData<P>,
}

impl<T> NonNullPtr<T, Shared>
where T: ?Sized
{
	/// Wraps a raw pointer, returning `None` if it is null.
	pub fn new(ptr: *const T) -> Option<Self> {
		NonNull::new(ptr.cast_mut()).map(Self::from_nonnull)
	}

	/// Wraps a raw pointer.
	///
	/// ## Safety
	///
	/// The pointer must not be null.
	pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
		Self::from_nonnull(NonNull::new_unchecked(ptr.cast_mut()))
	}
}

impl<T> NonNullPtr<T, Unique>
where T: ?Sized
{
	/// Wraps a raw pointer, returning `None` if it is null.
	pub fn new(ptr: *mut T) -> Option<Self> {
		NonNull::new(ptr).map(Self::from_nonnull)
	}

	/// Wraps a raw pointer.
	///
	/// ## Safety
	///
	/// The pointer must not be null.
	pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
		Self::from_nonnull(NonNull::new_unchecked(ptr))
	}
}

impl<T, P> NonNullPtr<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Encloses an existing non-null pointer.
	pub const fn from_nonnull(ptr: NonNull<T>) -> Self {
		Self {
			inner: ptr,
			_perm: PhantomData,
		}
	}

	/// Conwerts a `Pointer` to a `NonNullPtr`, returning `None` if it was null.
	pub fn from_pointer(ptr: Pointer<T, P>) -> Option<Self> {
		NonNull::new(ptr.into_const_ptr().cast_mut()).map(Self::from_nonnull)
	}

	/// Marks a `Pointer` as being non-null, without inspecting its value.
	///
	/// ## Safety
	///
	/// The pointer must not be null.
	pub unsafe fn from_pointer_unchecked(ptr: Pointer<T, P>) -> Self {
		Self::from_nonnull(NonNull::new_unchecked(
			ptr.into_const_ptr().cast_mut(),
		))
	}

	/// Shortcut for `Self::from_pointer(Pointer::from_ptr(P::Ptr<T>))`.
	pub fn from_permission_ptr(ptr: P::Ptr<T>) -> Option<Self> {
		Self::from_pointer(Pointer::from_ptr(ptr))
	}

	/// Produces the internal non-null pointer, discarding the permission type.
	///
	/// You are responsible for ensuring that this pointer is not used in
	/// violation of the provenance from which it was constructed.
	pub const fn into_inner(self) -> NonNull<T> {
		self.inner
	}

	/// Casts this to point to a different type at the same address.
	pub const fn cast<U: Sized>(self) -> NonNullPtr<U, P> {
		let Self { inner, _perm } = self;
		NonNullPtr {
			inner: inner.cast::<U>(),
			_perm,
		}
	}

	/// Overwrites the permission type with `Shared`.
	pub const fn cast_const(self) -> NonNullPtr<T, Shared> {
		NonNullPtr {
			inner: self.inner,
			_perm: PhantomData,
		}
	}

	/// Overwrites the permission type with `Unique`.
	///
	/// ## Safety
	///
	/// The original pointer must have been drawn from a provenance region with
	/// mutable permissions.
	pub const unsafe fn cast_mut(self) -> NonNullPtr<T, Unique> {
		NonNullPtr {
			inner: self.inner,
			_perm: PhantomData,
		}
	}

	/// Prepends `Shared` to the permission history.
	pub const fn cast_shared(self) -> NonNullPtr<T, (Shared, P)> {
		NonNullPtr {
			inner: self.inner,
			_perm: PhantomData,
		}
	}

	/// Discards the non-null guarantee.
	pub fn as_pointer(self) -> Pointer<T, P> {
		Pointer::from_ptr(P::from_const(self.inner.as_ptr().cast_const()))
	}

	/// Unconditionally produces a reference to the pointed-to value.
	///
	/// ## Safety
	///
	/// The pointed-to value must be initialized, must outlive the conjured `'a`
	/// lifetime`, and must have no other references to it that violate Rust’s
	/// rules. If `P` is `Unique`, no other references may exist at all; if `P`
	/// is shared, then no `Unique` reference may exist.
	pub unsafe fn as_reference<'a>(self) -> Reference<'a, T, P> {
		P::ptr_to_ref(self.as_pointer().as_ptr())
	}

	/// Gets the bare address to which the pointer points.
	pub fn addr(self) -> usize {
		self.inner.as_ptr().cast::<()>() as usize
	}
}

impl<T, P> NonNullPtr<T, P>
where
	T: Sized,
	P: Permission,
{
	/// Produces the canonical dangling pointer for `T`.
	pub const fn dangling() -> Self {
		Self {
			inner: NonNull::dangling(),
			_perm: PhantomData,
		}
	}

	/// Converts a base pointer into a slice pointer.
	///
	/// ## Safety
	///
	/// The memory region `self[.. len]` must be a single contiguous provenance
	/// region.
	pub unsafe fn make_slice(self, len: usize) -> NonNullPtr<[T], P> {
		NonNullPtr::from_pointer_unchecked(self.as_pointer().make_slice(len))
	}
}

impl<T, P> Clone for NonNullPtr<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> From<&T> for NonNullPtr<T, Shared>
where T: ?Sized
{
	fn from(src: &T) -> Self {
		unsafe { Self::new_unchecked(src) }
	}
}

impl<T> From<&mut T> for NonNullPtr<T, Unique>
where T: ?Sized
{
	fn from(src: &mut T) -> Self {
		unsafe { Self::new_unchecked(src) }
	}
}

impl<T> TryFrom<*const T> for NonNullPtr<T, Shared>
where T: ?Sized
{
	type Error = NullPtrError<T, Shared>;

	fn try_from(ptr: *const T) -> Result<Self, Self::Error> {
		Self::new(ptr).ok_or_else(NullPtrError::<T, Shared>::new)
	}
}

impl<T> TryFrom<*mut T> for NonNullPtr<T, Unique>
where T: ?Sized
{
	type Error = NullPtrError<T, Unique>;

	fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
		Self::new(ptr).ok_or_else(NullPtrError::<T, Unique>::new)
	}
}

impl<T, P> TryFrom<Pointer<T, P>> for NonNullPtr<T, P>
where
	T: ?Sized,
	P: Permission,
{
	type Error = NullPtrError<T, P>;

	fn try_from(ptr: Pointer<T, P>) -> Result<Self, Self::Error> {
		let const_ptr = ptr.into_const_ptr();
		let nonnull = NonNull::new(const_ptr.cast_mut())
			.ok_or_else(NullPtrError::<T, P>::new)?;
		Ok(Self::from_nonnull(nonnull))
	}
}

impl<T, P> Copy for NonNullPtr<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

#[doc = include_str!("../doc/references.md")]
pub type Reference<'a, T, P> = <P as Permission>::Ref<'a, T>;

/// Emitted when a NULL pointer is provided to an API that does not accept it.
pub struct NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	_type: PhantomData<*const T>,
	_perm: PhantomData<P>,
}

impl<T, P> NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	/// Creates a `NullPtrError` value.
	pub const fn new() -> Self {
		Self {
			_type: PhantomData,
			_perm: PhantomData,
		}
	}

	/// Downgrade the permission to just be `Shared`.
	pub const fn cast_const(self) -> NullPtrError<T, Shared> {
		NullPtrError::new()
	}
}

impl<T, P> Clone for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, P> Eq for NullPtrError<T, P>
where
	T: 'static + ?Sized,
	P: 'static + Permission,
{
}

impl<T, P> Ord for NullPtrError<T, P>
where
	T: 'static + ?Sized,
	P: 'static + Permission,
{
	fn cmp(&self, _: &Self) -> cmp::Ordering {
		cmp::Ordering::Equal
	}
}

impl<T1, T2, P1, P2> PartialEq<NullPtrError<T2, P2>> for NullPtrError<T1, P1>
where
	T1: 'static + ?Sized,
	T2: 'static + ?Sized,
	P1: 'static + Permission,
	P2: 'static + Permission,
{
	fn eq(&self, _: &NullPtrError<T2, P2>) -> bool {
		(any::TypeId::of::<T1>(), any::TypeId::of::<P1>())
			== (any::TypeId::of::<T2>(), any::TypeId::of::<P2>())
	}
}

impl<T1, T2, P1, P2> PartialOrd<NullPtrError<T2, P2>> for NullPtrError<T1, P1>
where
	T1: 'static + ?Sized,
	T2: 'static + ?Sized,
	P1: 'static + Permission,
	P2: 'static + Permission,
{
	fn partial_cmp(&self, _: &NullPtrError<T2, P2>) -> Option<cmp::Ordering> {
		(any::TypeId::of::<T1>(), any::TypeId::of::<P1>())
			.partial_cmp(&(any::TypeId::of::<T2>(), any::TypeId::of::<P2>()))
	}
}

impl<T, P> fmt::Debug for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"NullPtrError<{}, {}>",
			any::type_name::<T>(),
			any::type_name::<P>(),
		)
	}
}

impl<T, P> fmt::Display for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(
			fmt,
			"provided a NULL pointer to a non-null `{} {}`",
			P::DEBUG_PREFIX,
			any::type_name::<T>(),
		)
	}
}

impl<T, P> Copy for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

unsafe impl<T, P> Send for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

unsafe impl<T, P> Sync for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}

#[cfg(feature = "std")]
impl<T, P> std::error::Error for NullPtrError<T, P>
where
	T: ?Sized,
	P: Permission,
{
}
/// Unifying bridge over `*const T` and `*mut T`.
#[doc(hidden)]
pub trait RawPtr<T: ?Sized>: Copy {
	fn is_null(self) -> bool;

	unsafe fn offset(self, by: isize) -> Self
	where T: Sized;

	fn wrapping_offset(self, by: isize) -> Self
	where T: Sized;

	unsafe fn offset_from(self, origin: Self) -> isize
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
}

macro_rules! impl_raw_ptr {
	($($t:ty),+) => { $(
		impl<T: ?Sized> RawPtr<T> for $t {
			fn is_null(self) -> bool {
				self.is_null()
			}

			unsafe fn offset(self, by: isize) -> Self
			where T: Sized {
				self.offset(by)
			}

			fn wrapping_offset(self, by: isize) -> Self
			where T: Sized {
				self.wrapping_offset(by)
			}

			unsafe fn offset_from(self, origin: Self) -> isize
			where T: Sized {
				self.offset_from(origin)
			}

			unsafe fn read(self) -> T
			where T: Sized {
				self.read()
			}

			unsafe fn read_volatile(self) -> T
			where T: Sized {
				self.read_volatile()
			}

			unsafe fn read_unaligned(self) -> T
			where T: Sized {
				self.read_unaligned()
			}

			unsafe fn copy_to(self, dest: *mut T, count: usize)
			where T: Sized {
				self.copy_to(dest, count);
			}

			unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)
			where T: Sized {
				self.copy_to_nonoverlapping(dest, count);
			}

			fn align_offset(self, align: usize) -> usize
			where T: Sized {
				self.align_offset(align)
			}
		}
	)+ };
}

impl_raw_ptr!(*const T, *mut T);

/// Unifying bridge over `&T` and `&mut T`.
#[doc(hidden)]
pub trait RawRef<'a, T: ?Sized> {}

impl<'a, T: 'a + ?Sized> RawRef<'a, T> for &'a T {
}

impl<'a, T: 'a + ?Sized> RawRef<'a, T> for &'a mut T {
}

#[cfg(test)]
mod tests {
	use super::*;
	use static_assertions::*;

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
		let base: Pointer<usize, Unique> =
			Pointer::<usize, Unique>::new(data_ptr);
		let one: Pointer<usize, (Shared, Unique)> = base.cast_shared();
		let two: Pointer<usize, (Shared, (Shared, Unique))> = one.cast_shared();

		assert!(matches!(two.unwind_to_unique(), Some(p) if p == base));
	}
}
