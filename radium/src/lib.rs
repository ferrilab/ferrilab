//! `radium` provides a series of helper traits providing a uniform API for
//! interacting with both atomic types like
//! [`AtomicUsize`], and non-atomic types like [`Cell<T>`].
//!
//! This crate is `#![no_std]`-compatible, and uses no non-core types.
//!
//! For more details, see each trait's documentation.
//!
//! ---
//!
//! **@kneecaw** - <https://twitter.com/kneecaw/status/1132695060812849154>
//! > Feelin' lazy: Has someone already written a helper trait abstracting
//! > operations over `AtomicUsize` and `Cell<usize>` for generic code which may
//! > not care about atomicity?
//!
//! **@ManishEarth** - <https://twitter.com/ManishEarth/status/1132706585300496384>
//! > no but call the crate radium
//! >
//! > (since people didn't care that it was radioactive and used it in everything)
//!
//! [`AtomicUsize`]: core::sync::atomic::AtomicUsize
//! [`Cell<T>`]: core::cell::Cell

#![no_std]
#![deny(unconditional_recursion)]

use core::cell::Cell;
use core::sync::atomic::{
    self, AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16,
    AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

/// A maybe-atomic shared mutable fundamental type `T`.
///
/// This trait is implemented by both the [atomic wrapper] type for `T`, and by
/// [`Cell<T>`], providing a consistent interface for interacting with the two
/// types.
///
/// This trait only provides atomic load/store access, and does not perform
/// type-specific operations on the values. For types that can be used in
/// bitwise arithmetic, see [`RadiumBits`]; for types that can be used as
/// integers, see [`RadiumInteger`].
///
/// [atomic wrapper]: core::sync::atomic
/// [`Cell<T>`]: core::cell::Cell
/// [`RadiumBits`]: crate::RadiumBits
/// [`RadiumInteger`]: crate::RadiumInteger
pub trait Radium<Item> {
    /// Creates a new value of this type.
    fn new(value: Item) -> Self;

    /// If the underlying value is atomic, calls [`fence`] with the given
    /// [`Ordering`]. Otherwise, does nothing.
    ///
    /// [`Ordering`]: core::sync::atomic::Ordering
    /// [`fence`]: core::sync::atomic::fence
    fn fence(order: Ordering);

    /// Returns a mutable reference to the underlying value.
    ///
    /// This is safe because the mutable reference to `self` guarantees that no
    /// other references exist to this value.
    fn get_mut(&mut self) -> &mut Item;

    /// Consumes the wrapper and returns the contained value.
    ///
    /// This is safe as passing by value ensures no other references exist.
    fn into_inner(self) -> Item;

    /// Load a value from this object.
    ///
    /// Ordering values are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::load`].
    ///
    /// [`AtomicUsize::load`]: core::sync::atomic::AtomicUsize::load
    fn load(&self, order: Ordering) -> Item;

    /// Store a value in this object.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::store`].
    ///
    /// [`AtomicUsize::store`]: core::sync::atomic::AtomicUsize::store
    fn store(&self, value: Item, order: Ordering);

    /// Swap with the value stored in this object.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::swap`].
    ///
    /// [`AtomicUsize::swap`]: core::sync::atomic::AtomicUsize::swap
    fn swap(&self, value: Item, order: Ordering) -> Item;

    /// Stores a value into this object if the currently-stored value is the
    /// same as the `current` value.
    ///
    /// The return value is always the previously-stored value. If it is equal to
    /// `current`, then the value was updated with `new`.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::compare_and_swap`].
    ///
    /// [`AtomicUsize::compare_and_swap`]: core::sync::atomic::AtomicUsize::compare_and_swap
    fn compare_and_swap(&self, current: Item, new: Item, order: Ordering) -> Item;

    /// Stores a value into this object if the currently-stored value is the
    /// same as the `current` value.
    ///
    /// The return value is a `Result` indicating whether the new value was
    /// written, and containing the previously-stored value. On success, this
    /// value is guaranteed to be equal to `current`.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::compare_exchange`].
    ///
    /// [`AtomicUsize::compare_exchange`]: core::sync::atomic::AtomicUsize::compare_exchange
    fn compare_exchange(
        &self,
        current: Item,
        new: Item,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Item, Item>;

    /// Stores a value into this object if the currently-stored value is the
    /// same as the `current` value.
    ///
    /// Unlike `compare_exchange`, this function is allowed to spuriously fail
    /// even when the comparison succeeds, which can result in more efficient
    /// code on some platforms. The return value is a `Result` indicating
    /// whether the new value was written, and containing the previously-stored
    /// value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::compare_exchange_weak`].
    ///
    /// [`AtomicUsize::compare_exchange_weak`]: core::sync::atomic::AtomicUsize::compare_exchange_weak
    fn compare_exchange_weak(
        &self,
        current: Item,
        new: Item,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Item, Item>;
}

/// Performs bitwise arithmetic on a maybe-atomic value.
pub trait RadiumBits<Item>: Radium<Item> {
    /// Performs a bitwise "and" on the currently-stored value and the argument
    /// `value`, and stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_and`].
    ///
    /// [`AtomicUsize::fetch_and`]: core::sync::atomic::AtomicUsize::fetch_and
    fn fetch_and(&self, value: Item, order: Ordering) -> Item;

    /// Performs a bitwise "nand" on the currently-stored value and the argument
    /// `value`, and stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_nand`].
    ///
    /// [`AtomicUsize::fetch_nand`]: core::sync::atomic::AtomicUsize::fetch_nand
    fn fetch_nand(&self, value: Item, order: Ordering) -> Item;

    /// Performs a bitwise "or" on the currently-stored value and the argument
    /// `value`, and stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_or`].
    ///
    /// [`AtomicUsize::fetch_or`]: core::sync::atomic::AtomicUsize::fetch_or
    fn fetch_or(&self, value: Item, order: Ordering) -> Item;

    /// Performs a bitwise "xor" on the currently-stored value and the argument
    /// `value`, and stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_xor`].
    ///
    /// [`AtomicUsize::fetch_xor`]: core::sync::atomic::AtomicUsize::fetch_xor
    fn fetch_xor(&self, value: Item, order: Ordering) -> Item;
}

/// Performs integer arithmetic on a maybe-atomic value.
///
/// All integer types implement `RadiumBits`, so `RadiumInteger` may specify
/// `RadiumBits` as a supertrait, and users may specify `RadiumInteger` alone to
/// access `RadiumBits` as well.
pub trait RadiumInteger<Item>: RadiumBits<Item> {
    /// Adds `value` to the currently-stored value, wrapping on overflow, and
    /// stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_add`].
    ///
    /// [`AtomicUsize::fetch_add`]: core::sync::atomic::AtomicUsize::fetch_add
    fn fetch_add(&self, value: Item, order: Ordering) -> Item;

    /// Subtracts `value` from the currently-stored value, wrapping on
    /// underflow, and stores the result in `self`.
    ///
    /// Returns the previously-stored value.
    ///
    /// Ordering arguments are ignored by non-atomic types.
    ///
    /// See also: [`AtomicUsize::fetch_sub`].
    ///
    /// [`AtomicUsize::fetch_sub`]: core::sync::atomic::AtomicUsize::fetch_sub
    fn fetch_sub(&self, value: Item, order: Ordering) -> Item;
}

macro_rules! radium {
    //  Emit `Radium` trait function implementations where `self` is one of the
    //  standard library atomic types
    ( atom $base:ty ) => {
        #[inline]
        fn new(value: $base) -> Self {
            Self::new(value)
        }

        #[inline]
        fn fence(order: Ordering) {
            atomic::fence(order);
        }

        #[inline]
        fn get_mut(&mut self) -> &mut $base {
            self.get_mut()
        }

        #[inline]
        fn into_inner(self) -> $base {
            self.into_inner()
        }

        #[inline]
        fn load(&self, order: Ordering) -> $base {
            self.load(order)
        }

        #[inline]
        fn store(&self, value: $base, order: Ordering) {
            self.store(value, order);
        }

        #[inline]
        fn swap(&self, value: $base, order: Ordering) -> $base {
            self.swap(value, order)
        }

        #[inline]
        fn compare_and_swap(
            &self,
            current: $base,
            new: $base,
            order: Ordering,
        ) -> $base {
            self.compare_and_swap(current, new, order)
        }

        #[inline]
        fn compare_exchange(
            &self,
            current: $base,
            new: $base,
            success: Ordering,
            failure: Ordering,
        ) -> Result<$base, $base> {
            self.compare_exchange(current, new, success, failure)
        }

        #[inline]
        fn compare_exchange_weak(
            &self,
            current: $base,
            new: $base,
            success: Ordering,
            failure: Ordering,
        ) -> Result<$base, $base> {
            self.compare_exchange_weak(current, new, success, failure)
        }
    };

    //  Emit `RadiumBits` trait function implementations where `self` is one of
    //  the standard library atomic types
    ( atom_bits $base:ty ) => {
        #[inline]
        fn fetch_and(&self, value: $base, order: Ordering) -> $base {
            self.fetch_and(value, order)
        }

        #[inline]
        fn fetch_nand(&self, value: $base, order: Ordering) -> $base {
            self.fetch_nand(value, order)
        }

        #[inline]
        fn fetch_or(&self, value: $base, order: Ordering) -> $base {
            self.fetch_or(value, order)
        }

        #[inline]
        fn fetch_xor(&self, value: $base, order: Ordering) -> $base {
            self.fetch_xor(value, order)
        }
    };

    //  Emit `RadiumInteger` trait function implementations where `self` is one
    //  of the standard library atomic types
    ( atom_int $base:ty, $atom:ty ) => {
        #[inline]
        fn fetch_add(&self, value: $base, order: Ordering) -> $base {
            self.fetch_add(value, order)
        }

        #[inline]
        fn fetch_sub(&self, value: $base, order: Ordering) -> $base {
            self.fetch_sub(value, order)
        }
    };

    //  Implement `Radium` and `RadiumBits` for `bool` wrappers
    ( bits $( $base:ty , $atom:ty );* ) => { $(
        impl Radium<$base> for $atom {
            radium!(atom $base);
        }

        impl Radium<$base> for Cell<$base> {
            radium!(cell $base);
        }

        impl RadiumBits<$base> for $atom {
            radium!(atom_bits $base);
        }

        impl RadiumBits<$base> for Cell<$base> {
            radium!(cell_bits $base);
        }
    )* };

    //  Emit `Radium` trait function implementations where `self` is `Cell`
    ( cell $base:ty ) => {
        #[inline]
        fn new(value: $base) -> Self {
            Cell::new(value)
        }

        #[inline]
        fn fence(_: Ordering) {}

        #[inline]
        fn get_mut(&mut self) -> &mut $base {
            self.get_mut()
        }

        #[inline]
        fn into_inner(self) -> $base {
            self.into_inner()
        }

        #[inline]
        fn load(&self, _: Ordering) -> $base {
            self.get()
        }

        #[inline]
        fn store(&self, value: $base, _: Ordering) {
            self.set(value);
        }

        #[inline]
        fn swap(&self, value: $base, _: Ordering) -> $base {
            self.replace(value)
        }

        #[inline]
        fn compare_and_swap(
            &self,
            current: $base,
            new: $base,
            _: Ordering,
        ) -> $base {
            if self.get() == current {
                self.replace(new)
            }
            else {
                self.get()
            }
        }

        #[inline]
        fn compare_exchange(
            &self,
            current: $base,
            new: $base,
            _: Ordering,
            _: Ordering,
        ) -> Result<$base, $base> {
            if self.get() == current {
                Ok(self.replace(new))
            }
            else {
                Err(self.get())
            }
        }

        #[inline]
        fn compare_exchange_weak(
            &self,
            current: $base,
            new: $base,
            success: Ordering,
            failure: Ordering,
        ) -> Result<$base, $base> {
            Radium::compare_exchange(self, current, new, success, failure)
        }
    };

    //  Emit `RadiumBits` trait function implementations where `self` is `Cell`
    ( cell_bits $base:ty ) => {
        #[inline]
        fn fetch_and(&self, value: $base, _: Ordering) -> $base {
            self.replace(self.get() & value)
        }

        #[inline]
        fn fetch_nand(&self, value: $base, _: Ordering) -> $base {
            self.replace(!(self.get() & value))
        }

        #[inline]
        fn fetch_or(&self, value: $base, _: Ordering) -> $base {
            self.replace(self.get() | value)
        }

        #[inline]
        fn fetch_xor(&self, value: $base, _: Ordering) -> $base {
            self.replace(self.get() ^ value)
        }
    };

    //  Emit `RadiumInteger` trait function implementations where `self` is
    //  `Cell`
    ( cell_int $base:ty ) => {
        #[inline]
        fn fetch_add(&self, value: $base, _: Ordering) -> $base {
            self.replace(self.get().wrapping_add(value))
        }

        #[inline]
        fn fetch_sub(&self, value: $base, _: Ordering) -> $base {
            self.replace(self.get().wrapping_sub(value))
        }
    };

    //  Emit `RadiumInteger` trait implementations for both atomic and cell
    //  wrappers around a base type
    ( int $( $base:ty , $atom:ty ; )* ) => { $(
        radium!(bits $base, $atom);

        impl RadiumInteger<$base> for $atom {
            radium!(atom_int $base, $atom);
        }

        impl RadiumInteger<$base> for Cell<$base> {
            radium!(cell_int $base);
        }
    )* };

    //  Emit `Radium` trait implementations for pointers.
    ( ptr ) => {
        impl<T> Radium<*mut T> for AtomicPtr<T> {
            radium!(atom *mut T);
        }

        impl<T> Radium<*mut T> for Cell<*mut T> {
            radium!(cell *mut T);
        }
    };
}

radium![
    int
    i8, AtomicI8;
    i16, AtomicI16;
    i32, AtomicI32;
    i64, AtomicI64;
    isize, AtomicIsize;
    u8, AtomicU8;
    u16, AtomicU16;
    u32, AtomicU32;
    u64, AtomicU64;
    usize, AtomicUsize;
];

radium!(bits bool, AtomicBool);

radium!(ptr);
