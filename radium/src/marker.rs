//! Marker traits used to gate `Radium` methods and newtype parameters.

use crate::{seal::Sealed, Radium};
/// Indicates that the type supports bit-wise operations.
pub trait BitOps: Sealed {}

/// Indicates that the type supports integer operations.
pub trait NumericOps: BitOps + Sealed {}

macro_rules! mark {
        ($($t:ty => $($u:ty),+ $(,)?);+ $(;)?) => { $( $(
            impl $t for $u {}
        )+ )+ };
    }

mark! {
    BitOps => bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize;
    NumericOps => i8, u8, i16, u16, i32, u32, i64, u64, isize, usize;
}

/// Relates a primitive type to its corresponding atomic type.
///
/// This is only implemented when the corresponding atomic type exists.
pub trait Atomic: Sealed {
    /// The `core::sync::atomic` type corresponding to `Self`.
    type Atom: Radium<Item = Self>;
}

/// Relates a primitive type to its corresponding best-effort atomic type.
///
/// This is always implemented; however, because `Nucleus` uses `RadiumT` rather
/// than `AtomicT`, the destination type might wind up being `Cell<T>`.
pub trait Nuclear: Sealed {
    /// The `radium` type corresponding to `Self`.
    type Nucleus: Radium<Item = Self>;
}
