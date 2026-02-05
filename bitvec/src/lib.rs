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
