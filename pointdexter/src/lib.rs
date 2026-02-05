#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "\
data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIC0xMTYgMTI4IDEyOCIgeG1sbnM9Imh0d\
HA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBzdHlsZT0idHJhbnNmb3JtOnNrZXdYKC0xOGRlZyki\
PjxwYXRoIGQ9Ik0yOC01Mmg2MiIgc3R5bGU9InN0cm9rZTojZmY0ZjAwIi8+PHBhdGggZD0iTTQgMGg\
yNHYtMTA0aDYyIiBzdHlsZT0ic3Ryb2tlOnZhcigtLWZnKSIvPjwvZz48c3R5bGU+OnJvb3R7LS1mZz\
pibGFjazstLWJnOndoaXRlfUBtZWRpYSAocHJlZmVycy1jb2xvci1zY2hlbWU6ZGFyayl7OnJvb3R7L\
S1mZzp3aGl0ZTstLWJnOmJsYWNrfX0qe3NoYXBlLXJlbmRlcmluZzpjcmlzcEVkZ2VzfXBhdGh7c3Ry\
b2tlLXdpZHRoOjI0cHg7c3Ryb2tlLWxpbmVjYXA6YnV0dDtzdHJva2UtbGluZWpvaW46bWl0ZXI7Zml\
sbDpub25lfTwvc3R5bGU+PC9zdmc+")]
#![doc(html_logo_url = "\
data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSItNDAgLTE0MCAyNTYgMjU2IiB4bWxucz0ia\
HR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik0wLTEydi05NmgxNjhtLTI0IDI0SDBt\
MjQgNDhoMjR2LTI0SDI0djQ4aDI0bTI0IDB2LTQ4aDI0bTI0IDQ4di00OGgyNG0yNCA0OHYtNDhNMCA\
xMnY3MmgxNDRNMjQgMTJoMjR2NDhIMjRWMzZoMjRtMjQgMGgyNHYyNEg3MlYxMm05NiAwdjQ4IiB0cm\
Fuc2Zvcm09InNrZXdYKC0xOCkiLz48cGF0aCBkPSJNMTY4IDg0djBtMC0xNjh2MCIgc3R5bGU9InN0c\
m9rZTojZmY0ZjAwIiB0cmFuc2Zvcm09InNrZXdYKC0xOCkiLz48c3R5bGU+cGF0aHtzdHJva2U6IzAw\
MDtmaWxsOm5vbmU7c3Ryb2tlLXdpZHRoOjE2cHg7c3Ryb2tlLWxpbmVjYXA6c3F1YXJlfUBtZWRpYSA\
ocHJlZmVycy1jb2xvci1zY2hlbWU6ZGFyayl7cGF0aHtzdHJva2U6I2ZmZn19KntzaGFwZS1yZW5kZX\
Jpbmc6Y3Jpc3BFZGdlc308L3N0eWxlPjwvc3ZnPg==")]
#![no_std]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![allow(
	clippy::incompatible_msrv,
	reason = "advances beyond MSRV are gated behind features"
)]
#![deny(unconditional_recursion)]

use core::{
	hash,
	marker::PhantomData,
	ptr::NonNull,
};

#[macro_use]
mod macros;

mod error;
mod nonnull;
mod permission;
mod pointer;

pub use error::{
	NonUniqueError,
	NullPointerError,
};
pub use permission::{
	Shared,
	Unique,
};

/// Aliases `pointdexter` to `ptr`, and exports common symbols into the local
/// scope.
pub mod prelude {
	pub use crate::{
		self as ptr,
		NonNullPointer,
		NonUniqueError as PdxNonUniqueError,
		NullPointerError as PdxNullPointerError,
		Permission,
		Pointer,
		Reference,
		Shared,
		Unique,
	};
}

#[doc = include_str!("../doc/trait.Permission.md")]
pub trait Permission: permission::Impl {}

#[doc = include_str!("../doc/struct.Pointer.md")]
#[repr(transparent)]
pub struct Pointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	inner: P::Ptr<T>,
}

#[doc = include_str!("../doc/type.Reference.md")]
pub type Reference<'a, T, P> = <P as permission::Impl>::Ref<'a, T>;

#[repr(transparent)]
#[doc = include_str!("../doc/struct.NonNullPointer.md")]
pub struct NonNullPointer<T, P>
where
	T: ?Sized,
	P: Permission,
{
	inner: NonNull<T>,
	_perm: PhantomData<P>,
}

// region Constructors

#[inline(always)]
#[doc = include_str!("../doc/fn.null.md")]
pub const fn null<T, P>() -> Pointer<T, P>
where P: Permission {
	Pointer::null()
}

#[inline(always)]
#[doc = include_str!("../doc/fn.dangling.md")]
pub const fn dangling<T, P>() -> Pointer<T, P>
where P: Permission {
	Pointer::dangling()
}

#[inline(always)]
#[doc = include_str!("../doc/fn.with_exposed_provenance.md")]
#[doc = "[`with_addr`]: Pointer::with_addr"]
#[doc = "[`expose_provenance`]: Pointer::expose_provenance"]
#[cfg_attr(
	not(feature = "rust_189"),
	doc = "[`NonNullPointer::with_exposed_provenance`]: crate::NonNullPointer"
)]
pub fn with_exposed_provenance<T, P>(addr: usize) -> Pointer<T, P>
where P: Permission {
	Pointer::with_exposed_provenance(addr)
}

#[inline(always)]
#[doc = include_str!("../doc/fn.without_provenance.md")]
#[cfg_attr(
	not(feature = "rust_189"),
	doc = "[`NonNullPointer::without_provenance`]: crate::NonNullPointer"
)]
pub const fn without_provenance<T, P>(addr: usize) -> Pointer<T, P>
where P: Permission {
	Pointer::without_provenance(addr)
}

#[inline(always)]
#[doc = include_str!("../doc/fn.slice_from_raw_parts.md")]
pub const fn slice_from_raw_parts<T, P>(
	ptr: Pointer<T, P>,
	len: usize,
) -> Pointer<[T], P>
where
	P: Permission,
{
	ptr.make_slice(len)
}

// endregion Constructors

// region IO

#[inline(always)]
#[doc = include_str!("../doc/fn.copy.md")]
// #[doc = "[`read`]: crate::read"]
pub const unsafe fn copy<T, P>(
	src: Pointer<T, P>,
	dst: Pointer<T, Unique>,
	count: usize,
) where
	P: Permission,
{
	unsafe {
		core::ptr::copy(src.into_raw_const(), dst.into_raw_mut(), count);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.copy_nonoverlapping.md")]
pub const unsafe fn copy_nonoverlapping<T, P>(
	src: Pointer<T, P>,
	dst: Pointer<T, Unique>,
	count: usize,
) where
	P: Permission,
{
	unsafe {
		core::ptr::copy_nonoverlapping(
			src.into_raw_const(),
			dst.into_raw_mut(),
			count,
		);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.drop_in_place.md")]
pub unsafe fn drop_in_place<T>(ptr: Pointer<T, Unique>) {
	unsafe {
		core::ptr::drop_in_place(ptr.into_raw_mut());
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.read.md")]
pub const unsafe fn read<T, P>(ptr: Pointer<T, P>) -> T
where P: Permission {
	unsafe { core::ptr::read(ptr.into_raw_const()) }
}

#[inline(always)]
#[doc = include_str!("../doc/fn.read_unaligned.md")]
pub const unsafe fn read_unaligned<T, P>(ptr: Pointer<T, P>) -> T
where P: Permission {
	unsafe { core::ptr::read_unaligned(ptr.into_raw_const()) }
}

#[inline(always)]
#[doc = include_str!("../doc/fn.read_volatile.md")]
pub unsafe fn read_volatile<T, P>(ptr: Pointer<T, P>) -> T
where P: Permission {
	unsafe { core::ptr::read_volatile(ptr.into_raw_const()) }
}

#[inline(always)]
#[doc = include_str!("../doc/fn.write.md")]
pub const unsafe fn write<T>(ptr: Pointer<T, Unique>, src: T) {
	unsafe {
		core::ptr::write(ptr.into_raw_mut(), src);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.write_unaligned.md")]
#[doc = "[`write`]: crate::write"]
pub const unsafe fn write_unaligned<T>(ptr: Pointer<T, Unique>, src: T) {
	unsafe {
		core::ptr::write_unaligned(ptr.into_raw_mut(), src);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.write_volatile.md")]
pub unsafe fn write_volatile<T>(ptr: Pointer<T, Unique>, src: T) {
	unsafe {
		core::ptr::write_volatile(ptr.into_raw_mut(), src);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.write_bytes.md")]
pub const unsafe fn write_bytes<T>(
	ptr: Pointer<T, Unique>,
	val: u8,
	count: usize,
) {
	unsafe {
		core::ptr::write_bytes(ptr.into_raw_mut(), val, count);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.replace.md")]
pub const unsafe fn replace<T>(dst: Pointer<T, Unique>, src: T) -> T {
	unsafe { core::ptr::replace(dst.into_raw_mut(), src) }
}

#[inline(always)]
#[doc = include_str!("../doc/fn.swap.md")]
pub const unsafe fn swap<T>(x: Pointer<T, Unique>, y: Pointer<T, Unique>) {
	unsafe {
		core::ptr::swap(x.into_raw_mut(), y.into_raw_mut());
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.swap_nonoverlapping.md")]
#[cfg(not(feature = "rust_188"))]
pub unsafe fn swap_nonoverlapping<T>(
	x: Pointer<T, Unique>,
	y: Pointer<T, Unique>,
	count: usize,
) {
	unsafe {
		core::ptr::swap_nonoverlapping(
			x.into_raw_mut(),
			y.into_raw_mut(),
			count,
		);
	}
}

#[inline(always)]
#[doc = include_str!("../doc/fn.swap_nonoverlapping.md")]
#[cfg(feature = "rust_188")]
pub const unsafe fn swap_nonoverlapping<T>(
	x: Pointer<T, Unique>,
	y: Pointer<T, Unique>,
	count: usize,
) {
	unsafe {
		core::ptr::swap_nonoverlapping(
			x.into_raw_mut(),
			y.into_raw_mut(),
			count,
		);
	}
}

// endregion IO

#[inline(always)]
#[doc = include_str!("../doc/fn.addr_eq.md")]
pub fn addr_eq<T, U, P, Q>(p: Pointer<T, P>, q: Pointer<U, Q>) -> bool
where
	T: ?Sized,
	U: ?Sized,
	P: Permission,
	Q: Permission,
{
	core::ptr::addr_eq(p.into_raw_const(), q.into_raw_const())
}

#[inline(always)]
#[doc = include_str!("../doc/fn.eq.md")]
pub fn eq<T, P, Q>(a: Pointer<T, P>, b: Pointer<T, Q>) -> bool
where
	T: ?Sized,
	P: Permission,
	Q: Permission,
{
	core::ptr::eq(a.into_raw_const(), b.into_raw_const())
}

#[inline(always)]
#[doc = include_str!("../doc/fn.from_reference.md")]
pub const fn from_mut<T>(r: &mut T) -> Pointer<T, Unique>
where T: ?Sized {
	Pointer::from_raw_mut(r)
}

#[inline(always)]
#[doc = include_str!("../doc/fn.from_reference.md")]
pub const fn from_ref<T>(r: &T) -> Pointer<T, Shared>
where T: ?Sized {
	Pointer::from_raw_const(r)
}

#[inline(always)]
#[doc = include_str!("../doc/fn.hash.md")]
pub fn hash<T, P, S>(hashee: Pointer<T, P>, into: &mut S)
where
	T: ?Sized,
	P: Permission,
	S: hash::Hasher,
{
	core::ptr::hash(hashee.into_raw_const(), into);
}
