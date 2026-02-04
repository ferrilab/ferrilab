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
data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIC0xNDAgMjU2IDI1NiIgeG1sbnM9Imh0d\
HA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiBzdHlsZT0iaGVpZ2h0Ojk1MXB4Ij48cGF0aCBjbGFzcz0i\
cGVuLXNxIHN0cm9rZS1mZyIgZD0iTTAtMTJ2LTk2aDE2OG0tMjQgMjRIMG0yNCA0OGgyNHYtMjRIMjR\
2NDhoMjRtMjQgMHYtNDhoMjRtMjQgNDh2LTQ4aDI0bTI0IDQ4di00OCIgdHJhbnNmb3JtPSJ0cmFuc2\
xhdGUoNDApc2tld1goLTE4KSIvPjxwYXRoIGNsYXNzPSJwZW4tc3Egc3Ryb2tlLWFjY2VudCIgZD0iT\
TE2OC04NGgwIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSg0MClza2V3WCgtMTgpIi8+PHBhdGggY2xhc3M9\
InBlbi1zcSBzdHJva2UtZmciIGQ9Ik0wIDEydjcyaDE0NE0yNCAxMmgyNHY0OEgyNFYzNmgyNG0yNCA\
waDI0djI0SDcyVjEybTk2IDB2NDgiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDQwKXNrZXdYKC0xOCkiLz\
48cGF0aCBjbGFzcz0icGVuLXNxIHN0cm9rZS1hY2NlbnQiIGQ9Ik0xNjggODRoMCIgdHJhbnNmb3JtP\
SJ0cmFuc2xhdGUoNDApc2tld1goLTE4KSIvPjxzdHlsZT4uc3Ryb2tlLWZne3N0cm9rZTojMDAwfUBt\
ZWRpYSAocHJlZmVycy1jb2xvci1zY2hlbWU6ZGFyayl7LnN0cm9rZS1mZ3tzdHJva2U6I2ZmZn19LnN\
0cm9rZS1hY2NlbnR7c3Ryb2tlOiNmZjRmMDB9cGF0aHtmaWxsOm5vbmU7c3Ryb2tlLXdpZHRoOjE2cH\
g7c3Ryb2tlLWxpbmVjYXA6c3F1YXJlfS5wZW4tc3F7c3Ryb2tlLWxpbmVqb2luOm1pdGVyfTwvc3R5b\
GU+PC9zdmc+")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
	debug_assertions,
	warn(missing_docs, clippy::missing_docs_in_private_items)
)]
#![cfg_attr(
	not(debug_assertions),
	deny(missing_docs, clippy::missing_docs_in_private_items)
)]
#![deny(unconditional_recursion)]
#![allow(
	clippy::declare_interior_mutable_const,
	clippy::type_complexity,
	unknown_lints
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod devel;

#[macro_use]
pub mod macros;

pub mod access;
pub mod array;
pub mod boxed;
pub mod domain;
pub mod field;
pub mod index;
pub mod mem;
pub mod order;
pub mod ptr;
mod serdes;
pub mod slice;
pub mod store;
pub mod vec;
pub mod view;

#[doc = include_str!("../doc/prelude.md")]
pub mod prelude {
	pub use crate::{
		array::BitArray,
		bitarr,
		bits,
		field::BitField as _,
		order::{
			BitOrder,
			LocalBits,
			Lsb0,
			Msb0,
		},
		ptr::{
			BitPtr,
			BitPtrRange,
			BitRef,
		},
		slice::BitSlice,
		store::BitStore,
		view::{
			AsBits,
			AsMutBits,
			BitView as _,
			BitViewSized as _,
		},
		BitArr,
	};
	#[cfg(feature = "alloc")]
	pub use crate::{
		bitbox,
		bitvec,
		boxed::BitBox,
		vec::BitVec,
	};
}
