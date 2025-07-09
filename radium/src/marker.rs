//! Marker traits used to gate `Radium` methods and newtype parameters.

use core::ops::{
	Add,
	BitAnd,
	BitOr,
	BitXor,
	Not,
	Sub,
};

use crate::Radium;

/// Indicates that the type supports bit-wise operations.
pub trait BitOps:
	Sized
	+ BitAnd<Output = Self>
	+ BitOr<Output = Self>
	+ BitXor<Output = Self>
	+ Not<Output = Self>
{
}

/// Indicates that the type supports integer operations.
pub trait NumericOps:
	BitOps + Add<Output = Self> + Sub<Output = Self> + PartialEq + Ord
{
}

macro_rules! mark {
		($($t:ty => $($u:ty),+ $(,)?);+ $(;)?) => { $( $(
			impl $t for $u {}
		)+ )+ };
	}

mark! {
	BitOps => bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize;
	NumericOps => i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize;
}

/// Relates a primitive type to its corresponding atomic type.
///
/// This is only implemented when the corresponding atomic type exists.
///
/// Note that Rust #130539 introduced an `Atomic<T>` type alias that conflicts
/// with this trait name.
pub trait Atomic: Copy {
	/// The `AtomicT` type corresponding to `Self`.
	type Atom: Radium<Item = Self> + Send + Sync;
}

/// Relates a primitive type to its corresponding best-effort atomic type.
///
/// This is always implemented; however, because `Nucleus` uses `RadiumT` rather
/// than `AtomicT`, the destination type might wind up being `Cell<T>`.
///
/// ## Behind the Name
///
/// Atoms and (eukaryotic) cells both have a nucleus. Technically each
/// *implementor* of this trait is the nucleus, and the destination of the
/// associated type is the nuclear thing that possesses a nucleus. Sorry this
/// codebase isn’t a perfect reflection of biology and physics.
pub trait Nuclear: Copy {
	/// The `RadiumT` type corresponding to `Self`.
	type Nucleus: Radium<Item = Self>;
}
