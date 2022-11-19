use crate::{Atom, Atomic, Isotope, Nuclear, Radium, Radon};
use core::{cell::Cell, sync::atomic::*};

/// Forbid external implementation of `radium` traits. This crate *only* works
/// on the standard-library `AtomicT` and `Cell<T>` types, as well as its own
/// `Atom<T>` and `Isotope<T>`. We do not support third-party types, as only the
/// standard library can reasonably guarantee atomic behavior.
pub trait Sealed {}

macro_rules! sealed {
    ($($w:expr => {
        $($(@<$g:ident>)? $t:ty),+;
        $($($(@<$g2:ident>)? $a:ident),+;)?
    })+) => { $( $(
        impl$(<$g>)? Sealed for $t {}
        impl$(<$g>)? Sealed for Cell<$t> {}
    )+ $($(
        #[cfg(target_has_atomic = $w)]
        impl$(<$g2>)? Sealed for $a$(<$g2>)? {}
    )+)? )+ };
}

sealed! {
    "8" => {
        bool, i8, u8;
        AtomicBool, AtomicI8, AtomicU8;
    }
    "16" => {
        i16, u16;
        AtomicI16, AtomicU16;
    }
    "32" => {
        i32, u32;
        AtomicI32, AtomicU32;
    }
    "64" => {
        i64, u64;
        AtomicI64, AtomicU64;
    }
    "128" => {
        i128, u128;
    }
    "ptr" => {
        isize, usize, @<T> *mut T;
        AtomicIsize, AtomicUsize, @<T> AtomicPtr;
    }
}

impl<T> Sealed for Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T>,
{
}

impl<T> Sealed for Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T>,
{
}

impl<T> Sealed for Radon<T>
where
    T: Nuclear,
    Cell<T>: Radium<Item = T>,
{
}
