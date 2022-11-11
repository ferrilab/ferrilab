//! Type definitions
//!
//! This module provides `RadiumT` aliases that correspond to `AtomicT` from the
//! standard library, and resolve to `AtomicT` when it exists and `Cell<T>` when
//! it does not. In addition, newtype structs `Atom<T>` and `Isotope<T>`
//! correspond to (and contain) `AtomicT` and `RadiumT`, respectively. These
//! newtypes are designed to be used in cases where you are generic over a
//! primitive and want to plug it into a shared-mutable wrapper type without
//! having to specifically name one of the `AtomicT` or `RadiumT` individual
//! names.

#[allow(unused_imports)]
use core::{
    cell::Cell,
    fmt::{self, Debug, Formatter},
    sync::atomic::*,
};

use crate::{
    marker::{Atomic, Nuclear},
    Radium,
};

#[repr(transparent)]
#[doc = include_str!("../doc/atom.md")]
pub struct Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T>,
{
    pub(crate) inner: T::Atom,
}

impl<T> Debug for Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T> + Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, fmt)
    }
}

impl<T> Default for Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T> + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> From<T> for Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T> + From<T>,
{
    #[inline]
    fn from(val: T) -> Self {
        Self {
            inner: From::from(val),
        }
    }
}

#[repr(transparent)]
#[doc = include_str!("../doc/isotope.md")]
pub struct Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T>,
{
    pub(crate) inner: T::Nucleus,
}

impl<T> Debug for Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T> + Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, fmt)
    }
}

impl<T> Default for Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T> + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> From<T> for Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T> + From<T>,
{
    #[inline]
    fn from(val: T) -> Self {
        Self {
            inner: From::from(val),
        }
    }
}

/// Creates type aliases that resolve to either `AtomicT` or `Cell<T>` depending
/// on availability.
macro_rules! alias {
    ($($width:literal => { $(
        $(@<$t:ident>)? $base:ty => $radium:ident $(=> $atom:ident)?
    );+ $(;)? })+) => { $( $(
        alias!(atom $width $(@<$t>)? $base => $radium $(=> $atom)?);

        alias!(cell $width $(@<$t>)? $base => $radium $(=> $atom)?);
    )+ )+ };

    (atom $width:literal $(@<$t:ident>)? $base:ty => $radium:ident) => {};

    (atom $width:literal $(@<$t:ident>)? $base:ty => $radium:ident => $atom:ident) => {
        #[doc = concat!("Best-effort atomicity for `", stringify!($base), "`.")]
        ///
        /// This target has the required atomic support.
        #[cfg(target_has_atomic = $width)]
        pub type $radium$(<$t>)? = $atom$(<$t>)?;

        // If the atomic variant exists, create `Atom<T>`.
        #[cfg(target_has_atomic = $width)]
        impl$(<$t>)? Atomic for $base {
            type Atom = $atom$(<$t>)?;
        }
    };

    // When an atom is provided, be conditional on target atomics.
    (cell $width:literal $(@<$t:ident>)? $base:ty => $radium:ident => $atom:ident) => {
        #[doc = concat!("Best-effort atomicity for `", stringify!($base), "`.")]
        ///
        /// This target does not have the required atomic support, and is
        /// falling back to `Cell`.
        #[cfg(not(target_has_atomic = $width))]
        pub type $radium$(<$t>)? = Cell<$base>;

        // Create `Isotope<T>` with the generated alias.
        impl$(<$t>)? Nuclear for $base {
            type Nucleus = $radium$(<$t>)?;
        }
    };

    // When an atom is not provided, unconditionally create the alias.
    (cell $width:literal $(@<$t:ident>)? $base:ty => $radium:ident) => {
        #[doc = concat!("Best-effort atomicity for `", stringify!($base), "`.")]
        ///
        /// The required atomic support is not stabilized in `core`, so this is
        /// unconditionally a `Cell`.
        pub type $radium$(<$t>)? = Cell<$base>;

        /// Note: the standard library has an unstable atomic for this type.
        /// `radium` commits to operating on the stable release series, and so
        /// will not use its atomic variant, but is willing to prepare for
        /// assumed stabilization by acting on the `Cell`.
        impl$(<$t>)? Nuclear for $base {
            type Nucleus = $radium$(<$t>)?;
        }
    };
}

alias! {
    "8" => {
        bool => RadiumBool => AtomicBool;
        i8 => RadiumI8 => AtomicI8;
        u8 => RadiumU8 => AtomicU8;
    }
    "16" => {
        i16 => RadiumI16 => AtomicI16;
        u16 => RadiumU16 => AtomicU16;
    }
    "32" => {
        i32 => RadiumI32 => AtomicI32;
        u32 => RadiumU32 => AtomicU32;
    }
    "64" => {
        i64 => RadiumI64 => AtomicI64;
        u64 => RadiumU64 => AtomicU64;
    }
    "128" => {
        i128 => RadiumI128; // => AtomicI128; // when this stabilizes
        u128 => RadiumU128; // => AtomicU128; // when this stabilizes
    }
    "ptr" => {
        isize => RadiumIsize => AtomicIsize;
        usize => RadiumUsize => AtomicUsize;
        @<T> *mut T => RadiumPtr => AtomicPtr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    #[test]
    fn atom_impls() {
        #[cfg(target_has_atomic = "8")]
        {
            assert_impl_all!(Atom<bool>: Debug, Default, From<bool>, Sync);
            assert_impl_all!(Atom<i8>: Debug, Default, From<i8>, Sync);
            assert_impl_all!(Atom<u8>: Debug, Default, From<u8>, Sync);
        }
        #[cfg(target_has_atomic = "16")]
        {
            assert_impl_all!(Atom<i16>: Debug, Default, From<i16>, Sync);
            assert_impl_all!(Atom<u16>: Debug, Default, From<u16>, Sync);
        }
        #[cfg(target_has_atomic = "32")]
        {
            assert_impl_all!(Atom<i32>: Debug, Default, From<i32>, Sync);
            assert_impl_all!(Atom<u32>: Debug, Default, From<u32>, Sync);
        }
        #[cfg(target_has_atomic = "64")]
        {
            assert_impl_all!(Atom<i64>: Debug, Default, From<i64>, Sync);
            assert_impl_all!(Atom<u64>: Debug, Default, From<u64>, Sync);
        }
        #[cfg(target_has_atomic = "ptr")]
        {
            assert_impl_all!(Atom<isize>: Debug, Default, From<isize>, Sync);
            assert_impl_all!(Atom<usize>: Debug, Default, From<usize>, Sync);
            assert_impl_all!(Atom<*mut ()>: Debug, Default, From<*mut ()>, Sync);
        }
    }

    #[test]
    fn isotope_impls() {
        assert_impl_all!(Isotope<bool>: Debug, Default, From<bool>);
        assert_impl_all!(Isotope<i8>: Debug, Default, From<i8>);
        assert_impl_all!(Isotope<u8>: Debug, Default, From<u8>);
        assert_impl_all!(Isotope<i16>: Debug, Default, From<i16>);
        assert_impl_all!(Isotope<u16>: Debug, Default, From<u16>);
        assert_impl_all!(Isotope<i32>: Debug, Default, From<i32>);
        assert_impl_all!(Isotope<u32>: Debug, Default, From<u32>);
        assert_impl_all!(Isotope<i64>: Debug, Default, From<i64>);
        assert_impl_all!(Isotope<u64>: Debug, Default, From<u64>);
        assert_impl_all!(Isotope<i128>: Debug, Default, From<i128>);
        assert_impl_all!(Isotope<u128>: Debug, Default, From<u128>);
        assert_impl_all!(Isotope<isize>: Debug, Default, From<isize>);
        assert_impl_all!(Isotope<usize>: Debug, Default, From<usize>);
        assert_impl_all!(Isotope<*mut ()>: Debug, Default, From<*mut ()>);
    }

    #[test]
    fn isotope_atomic() {
        #[cfg(target_has_atomic = "8")]
        {
            assert_impl_all!(Isotope<bool>: Sync);
            assert_impl_all!(Isotope<i8>: Sync);
            assert_impl_all!(Isotope<u8>: Sync);
        }
        #[cfg(not(target_has_atomic = "8"))]
        {
            assert_not_impl_any!(Isotope<bool>: Sync);
            assert_not_impl_any!(Isotope<i8>: Sync);
            assert_not_impl_any!(Isotope<u8>: Sync);
        }

        #[cfg(target_has_atomic = "16")]
        {
            assert_impl_all!(Isotope<i16>: Sync);
            assert_impl_all!(Isotope<u16>: Sync);
        }
        #[cfg(not(target_has_atomic = "16"))]
        {
            assert_not_impl_any!(Isotope<i16>: Sync);
            assert_not_impl_any!(Isotope<u16>: Sync);
        }

        #[cfg(target_has_atomic = "32")]
        {
            assert_impl_all!(Isotope<i32>: Sync);
            assert_impl_all!(Isotope<u32>: Sync);
        }
        #[cfg(not(target_has_atomic = "32"))]
        {
            assert_not_impl_any!(Isotope<i32>: Sync);
            assert_not_impl_any!(Isotope<u32>: Sync);
        }

        #[cfg(target_has_atomic = "64")]
        {
            assert_impl_all!(Isotope<i64>: Sync);
            assert_impl_all!(Isotope<u64>: Sync);
        }
        #[cfg(not(target_has_atomic = "64"))]
        {
            assert_not_impl_any!(Isotope<i64>: Sync);
            assert_not_impl_any!(Isotope<u64>: Sync);
        }

        #[cfg(target_has_atomic = "ptr")]
        {
            assert_impl_all!(Isotope<isize>: Sync);
            assert_impl_all!(Isotope<usize>: Sync);
            assert_impl_all!(Isotope<*mut ()>: Sync);
        }
        #[cfg(not(target_has_atomic = "ptr"))]
        {
            assert_not_impl_any!(Isotope<isize>: Sync);
            assert_not_impl_any!(Isotope<usize>: Sync);
            assert_not_impl_any!(Isotope<*mut ()>: Sync);
        }

        // These are always non-atomic until `Atomic*128` stabilizes.
        assert_not_impl_any!(Isotope<i128>: Sync);
        assert_not_impl_any!(Isotope<u128>: Sync);
    }
}
