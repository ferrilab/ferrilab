#![doc = include_str!("../README.md")]
#![no_std]
#![deny(unconditional_recursion)]

pub mod marker;
mod seal;
pub mod types;

pub use crate::types::{Atom, Isotope, Radon};

use crate::marker::*;
use core::{cell::Cell, sync::atomic::*};

#[doc = include_str!("../doc/radium.md")]
pub trait Radium: seal::Sealed {
    /// The primitive type that this implementor makes shared-mutable.
    type Item;

    /// Creates a new value of this type.
    fn new(value: Self::Item) -> Self;

    /// If the implementor is atomic, this calls [`atomic::fence`] with the
    /// given `Ordering`; otherwise, it does nothing.
    ///
    /// [`atomic::fence`]: core::sync::atomic::fence
    fn fence(order: Ordering);

    /// Returns a mutable reference to the underlying value.
    ///
    /// This is safe because the mutable reference to `self` guarantees that no
    /// other references exist to this value.
    fn get_mut(&mut self) -> &mut Self::Item;

    /// Consumes the wrapper and returns the contained value.
    ///
    /// This is safe because consuming by value ensures that no other references
    /// exist.
    fn into_inner(self) -> Self::Item;

    /// Loads a value from this object.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::load`].
    ///
    /// [`AtomicUsize::load`]: core::sync::atomic::AtomicUsize::load
    fn load(&self, order: Ordering) -> Self::Item;

    /// Stores a value into this object.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::store`].
    ///
    /// [`AtomicUsize::store`]: core::sync::atomic::AtomicUsize::store
    fn store(&self, value: Self::Item, order: Ordering);

    /// Swaps a new value with the value stored in this object.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::swap`].
    ///
    /// [`AtomicUsize::swap`]: core::sync::atomic::AtomicUsize::swap
    fn swap(&self, value: Self::Item, order: Ordering) -> Self::Item;

    /// Stores a new value into this object if (and only if) the value currently
    /// stored in it is the same as the `current` argument.
    ///
    /// The return value is always what the object contained before the call
    /// entered. If it is equal to the `current` argument, then the object has
    /// been updated to contain `new`.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::compare_and_swap`].
    ///
    /// [`AtomicUsize::compare_and_swap`]: core::sync::atomic::AtomicUsize::compare_and_swap
    #[deprecated = "Use `compare_exchange` or `compare_exchange_weak` instead"]
    fn compare_and_swap(&self, current: Self::Item, new: Self::Item, order: Ordering)
        -> Self::Item;

    /// Stores a new value into this object if (and only if) the value currently
    /// stored in it is the same as the `current` argument.
    ///
    /// The return value is a `Result` indicating whether the new value was
    /// written into this object, and containing the value this object contained
    /// when the call entered. On success, this value is guaranteed to be equal
    /// to `current`.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::compare_exchange`].
    ///
    /// [`AtomicUsize::compare_exchange`]: core::sync::atomic::AtomicUsize::compare_exchange
    fn compare_exchange(
        &self,
        current: Self::Item,
        new: Self::Item,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Item, Self::Item>;

    /// Stores a new value into this object if (and only if) the value currently
    /// stored in it is the same as the `current` argument.
    ///
    /// Unlike `compare_exchange`, this function is allowed to spuriously fail
    /// even when the comparison succeeds, which can result in more efficient
    /// code on some platforms. The return value is a `Result` indicating
    /// whether the comparison succeeded and the new value was written, and
    /// containing the value this object contained when the call entered.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::compare_exchange_weak`].
    ///
    /// [`AtomicUsize::compare_exchange_weak`]: core::sync::atomic::AtomicUsize::compare_exchange_weak
    fn compare_exchange_weak(
        &self,
        current: Self::Item,
        new: Self::Item,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Item, Self::Item>;

    /// Performs a bit-wise AND on the currently-stored value and the argument.
    /// The result is stored into this object, and the previous value is
    /// returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_and`].
    ///
    /// [`AtomicUsize::fetch_and`]: core::sync::atomic::AtomicUsize::fetch_and
    fn fetch_and(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: BitOps;

    /// Performs a bit-wise NAND on the currently-stored value and the argument.
    /// The result is stored into this object, and the previous value is
    /// returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_nand`].
    ///
    /// [`AtomicUsize::fetch_nand`]: core::sync::atomic::AtomicUsize::fetch_nand
    fn fetch_nand(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: BitOps;

    /// Performs a bit-wise OR on the currently-stored value and the argument.
    /// The result is stored into this object, and the previous value is
    /// returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_or`].
    ///
    /// [`AtomicUsize::fetch_or`]: core::sync::atomic::AtomicUsize::fetch_or
    fn fetch_or(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: BitOps;

    /// Performs a bit-wise XOR on the currently-stored value and the argument.
    /// The result is stored into this object, and the previous value is
    /// returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_xor`].
    ///
    /// [`AtomicUsize::fetch_xor`]: core::sync::atomic::AtomicUsize::fetch_xor
    fn fetch_xor(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: BitOps;

    /// Adds the argument into the currently-stored value, wrapping on overflow.
    /// The result is stored into this object, and the previous value is
    /// returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_add`].
    ///
    /// [`AtomicUsize::fetch_add`]: core::sync::atomic::AtomicUsize::fetch_add
    fn fetch_add(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: NumericOps;

    /// Subtracts the argument from the currently-stored value, wrapping on
    /// overflow. The result is stored into this object, and the previous value
    /// is returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_sub`].
    ///
    /// [`AtomicUsize::fetch_sub`]: core::sync::atomic::AtomicUsize::fetch_sub
    fn fetch_sub(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: NumericOps;

    /// Finds the maximum of the currently-stored value and the argument. The
    /// result is stored into this object, and the previous value is returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_max`].
    ///
    /// [`AtomicUsize::fetch_max`]: core::sync::atomic::AtomicUsize::fetch_max
    fn fetch_max(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: NumericOps;

    /// Finds the minimum of the currently-stored value and the argument. The
    /// result is stored into this object, and the previous value is returned.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_min`].
    ///
    /// [`AtomicUsize::fetch_min`]: core::sync::atomic::AtomicUsize::fetch_min
    fn fetch_min(&self, value: Self::Item, order: Ordering) -> Self::Item
    where
        Self::Item: NumericOps;

    /// Fetches the value, and applies a function to it that may produce a new
    /// value.
    ///
    /// Note: this may call the generator function multiple times if the stored
    /// value is updated in between the fetch and store. However, when a store
    /// occurs successfully, the generator will have been applied only once to
    /// the fetched value. That is, this function will never store
    /// `f(f(self.load()))`.
    ///
    /// Returns `Ok(fetched)` if the generator produces `Some` new value which
    /// is successfully stored, or `Err(fetched)` if it produces `None`.
    ///
    /// Non-atomic implementors ignore the ordering value.
    ///
    /// See also: [`AtomicUsize::fetch_update`].
    ///
    /// [`AtomicUsize::fetch_update`]: core::sync::atomic::AtomicUsize::fetch_update
    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::Item, Self::Item>
    where
        F: FnMut(Self::Item) -> Option<Self::Item>;
}

/// Generates `Radium` implementation bodies.
macro_rules! radium {
    ($($width:literal => $bit:ident $num:ident => {
        $($(@<$t:ident>)? $base:ty $(=> $atom:ident)?;)+
    } )+) => { $( $(
        radium!(atom $width $bit $num $(@<$t>)? $base $(=> $atom)?);

        radium!(cell $width $bit $num $(@<$t>)? $base);
    )+ )+ };

    // Trap the branch that has no named atom.
    (atom $width:literal $bit:ident $num:ident $(@<$t:ident>)? $base:ty) => {};

    // Generate an implementation for the named atom.
    (
        atom $width:literal $bit:ident $num:ident
        $(@<$t:ident>)? $base:ty => $atom:ident
    ) => {
        #[cfg(target_has_atomic = $width)]
        impl$(<$t>)? Radium for $atom$(<$t>)? {
            type Item = $base;

            #[inline]
            fn new(value: $base) -> Self {
                $atom::new(value)
            }

            #[inline]
            fn fence(order: Ordering) {
                core::sync::atomic::fence(order);
            }

            #[inline]
            fn get_mut(&mut self) -> &mut $base {
                $atom::get_mut(self)
            }

            #[inline]
            fn into_inner(self) -> $base {
                $atom::into_inner(self)
            }

            #[inline]
            fn load(&self, order: Ordering) -> $base {
                $atom::load(self, order)
            }

            #[inline]
            fn store(&self, value: $base, order: Ordering) {
                $atom::store(self, value, order);
            }

            #[inline]
            fn swap(&self, value: $base, order: Ordering) -> $base {
                $atom::swap(self, value, order)
            }

            #[inline]
            #[allow(deprecated)]
            fn compare_and_swap(
                &self,
                current: $base,
                new: $base,
                order: Ordering,
            ) -> $base {
                $atom::compare_and_swap(self, current, new, order)
            }

            #[inline]
            fn compare_exchange(
                &self,
                current: $base,
                new: $base,
                success: Ordering,
                failure: Ordering
            ) -> Result<$base, $base> {
                $atom::compare_exchange(self, current, new, success, failure)
            }

            #[inline]
            fn compare_exchange_weak(
                &self,
                current: $base,
                new: $base,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$base, $base> {
                $atom::compare_exchange_weak(
                    self,
                    current,
                    new,
                    success,
                    failure,
                )
            }

            radium!(atom $bit $atom => $base);

            radium!(atom $num $atom => $base);

            #[inline]
            fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                func: F,
            ) -> Result<$base, $base>
            where
                F: FnMut($base) -> Option<$base>,
            {
                $atom::fetch_update(self, set_order, fetch_order, func)
            }
        }
    };

    // Generate an implementation for the Cell.
    (cell $width:literal $bit:ident $num:ident $(@<$t:ident>)? $base:ty) => {
        impl$(<$t>)? Radium for Cell<$base> {
            type Item = $base;

            #[inline]
            fn new(value: $base) -> Self {
                Cell::new(value)
            }

            #[inline]
            fn fence(_: Ordering) {}

            #[inline]
            fn get_mut(&mut self) -> &mut $base {
                Cell::get_mut(self)
            }

            #[inline]
            fn into_inner(self) -> $base {
                Cell::into_inner(self)
            }

            #[inline]
            fn load(&self, _: Ordering) -> $base {
                Cell::get(self)
            }

            #[inline]
            fn store(&self, value: $base, _: Ordering) {
                Cell::set(self, value);
            }

            #[inline]
            fn swap(&self, value: $base, _: Ordering) -> $base {
                Cell::replace(self, value)
            }

            #[inline]
            #[allow(deprecated)]
            fn compare_and_swap(
                &self,
                current: $base,
                new: $base,
                _: Ordering,
            ) -> $base {
                let old = Cell::get(self);
                if old == current {
                    Cell::set(self, new);
                }
                old
            }

            #[inline]
            fn compare_exchange(
                &self,
                current: $base,
                new: $base,
                _: Ordering,
                _: Ordering
            ) -> Result<$base, $base> {
                let old = Cell::get(self);
                if old == current {
                    Cell::set(self, new);
                    Ok(old)
                } else {
                    Err(old)
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

            radium!(cell $bit $base);

            radium!(cell $num $base);

            #[inline]
            fn fetch_update<F>(&self, _: Ordering, _: Ordering, mut func: F)
                -> Result<$base, $base>
            where
                F: FnMut($base) -> Option<$base>,
            {
                let old = Cell::get(self);
                match func(old) {
                    Some(new) => {
                        Cell::set(self, new);
                        Ok(old)
                    },
                    None => Err(old),
                }
            }
        }
    };

    // Forward to the atomic RMU functions.

    (atom bit $atom:ident => $base:ty) => {
        #[inline]
        fn fetch_and(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_and(self, value, order)
        }

        #[inline]
        fn fetch_nand(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_nand(self, value, order)
        }

        #[inline]
        fn fetch_or(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_or(self, value, order)
        }

        #[inline]
        fn fetch_xor(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_xor(self, value, order)
        }
    };

    (atom num $atom:ident => $base:ty) => {
        #[inline]
        fn fetch_add(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_add(self, value, order)
        }

        #[inline]
        fn fetch_sub(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_sub(self, value, order)
        }

        #[inline]
        fn fetch_max(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_max(self, value, order)
        }

        #[inline]
        fn fetch_min(&self, value: $base, order: Ordering) -> $base {
            $atom::fetch_min(self, value, order)
        }
    };

    // Implement non-atomic RMU functions.

    (cell bit $base:ty) => {
        #[inline]
        fn fetch_and(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, old & value);
            old
        }

        #[inline]
        fn fetch_nand(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, !(old & value));
            old
        }

        #[inline]
        fn fetch_or(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, old | value);
            old
        }

        #[inline]
        fn fetch_xor(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, old ^ value);
            old
        }
    };

    (cell num $base:ty) => {
        #[inline]
        fn fetch_add(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, old.wrapping_add(value));
            old
        }

        #[inline]
        fn fetch_sub(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, old.wrapping_sub(value));
            old
        }

        #[inline]
        fn fetch_max(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, core::cmp::max(old, value));
            old
        }

        #[inline]
        fn fetch_min(&self, value: $base, _: Ordering) -> $base {
            let old = Cell::get(self);
            Cell::set(self, core::cmp::min(old, value));
            old
        }
    };

    // Handle stubbed-out RMU functions.

    (atom no_bit $atom:ident => $base:ty) => {
        radium!(unreachable fetch_and, fetch_nand, fetch_or, fetch_xor);
    };

    (atom no_num $atom:ident => $base:ty) => {
        radium!(unreachable fetch_add, fetch_sub, fetch_max, fetch_min);
    };

    (cell no_bit $base:ty) => {
        radium!(unreachable fetch_and, fetch_nand, fetch_or, fetch_xor);
    };

    (cell no_num $base:ty) => {
        radium!(unreachable fetch_add, fetch_sub, fetch_max, fetch_min);
    };

    // Generate forwarding functions for `Atom<T>` and `Isotope<T>`.
    (wrappers) => {
        #[inline]
        fn new(value: T) -> Self {
            Self {
                inner: Radium::new(value),
            }
        }

        #[inline]
        fn get_mut(&mut self) -> &mut T {
            Radium::get_mut(&mut self.inner)
        }

        #[inline]
        fn into_inner(self) -> T {
            Radium::into_inner(self.inner)
        }

        #[inline]
        fn load(&self, order: Ordering) -> T {
            Radium::load(&self.inner, order)
        }

        #[inline]
        fn store(&self, value: T, order: Ordering) {
            Radium::store(&self.inner, value, order);
        }

        #[inline]
        fn swap(&self, value: T, order: Ordering) -> T {
            Radium::swap(&self.inner, value, order)
        }

        #[inline]
        #[allow(deprecated)]
        fn compare_and_swap(
            &self,
            current: T,
            new: T,
            order: Ordering,
        ) -> T {
            Radium::compare_and_swap(&self.inner, current, new, order)
        }

        #[inline]
        fn compare_exchange(
            &self,
            current: T,
            new: T,
            success: Ordering,
            failure: Ordering,
        ) -> Result<T, T> {
            Radium::compare_exchange(
                &self.inner,
                current,
                new,
                success,
                failure,
            )
        }

        #[inline]
        fn compare_exchange_weak(
            &self,
            current: T,
            new: T,
            success: Ordering,
            failure: Ordering,
        ) -> Result<T, T> {
            Radium::compare_exchange_weak(
                &self.inner,
                current,
                new,
                success,
                failure,
            )
        }

        #[inline]
        fn fetch_and(&self, value: T, order: Ordering) -> T
        where
            T: BitOps,
        {
            Radium::fetch_and(&self.inner, value, order)
        }

        #[inline]
        fn fetch_nand(&self, value: T, order: Ordering) -> T
        where
            T: BitOps,
        {
            Radium::fetch_nand(&self.inner, value, order)
        }

        #[inline]
        fn fetch_or(&self, value: T, order: Ordering) -> T
        where
            T: BitOps,
        {
            Radium::fetch_or(&self.inner, value, order)
        }

        #[inline]
        fn fetch_xor(&self, value: T, order: Ordering) -> T
        where
            T: BitOps,
        {
            Radium::fetch_xor(&self.inner, value, order)
        }

        #[inline]
        fn fetch_add(&self, value: T, order: Ordering) -> T
        where
            T: NumericOps,
        {
            Radium::fetch_add(&self.inner, value, order)
        }

        #[inline]
        fn fetch_sub(&self, value: T, order: Ordering) -> T
        where
            T: NumericOps,
        {
            Radium::fetch_sub(&self.inner, value, order)
        }

        #[inline]
        fn fetch_max(&self, value: T, order: Ordering) -> T
        where
            T: NumericOps,
        {
            Radium::fetch_max(&self.inner, value, order)
        }

        #[inline]
        fn fetch_min(&self, value: T, order: Ordering) -> T
        where
            T: NumericOps,
        {
            Radium::fetch_min(&self.inner, value, order)
        }

        #[inline]
        fn fetch_update<F>(
            &self,
            set_order: Ordering,
            fetch_order: Ordering,
            func: F,
        ) -> Result<T, T>
        where
            F: FnMut(T) -> Option<T>,
        {
            Radium::fetch_update(&self.inner, set_order, fetch_order, func)
        }
    };

    // Generate stubs for the conditionally-unreachable methods.
    (unreachable $($n:ident),+ $(,)?) => { $(
        fn $n(&self, _: Self::Item, _: Ordering) -> Self::Item {
            unreachable!(
                "This function is statically guaranteed to never be callable",
            )
        }
    )+ };
}

radium! {
    "8" => bit no_num => {
        bool => AtomicBool;
    }
    "8" => bit num => {
        i8 => AtomicI8;
        u8 => AtomicU8;
    }
    "16" => bit num => {
        i16 => AtomicI16;
        u16 => AtomicU16;
    }
    "32" => bit num => {
        i32 => AtomicI32;
        u32 => AtomicU32;
    }
    "64" => bit num => {
        i64 => AtomicI64;
        u64 => AtomicU64;
    }
    "128" => bit num => {
        i128; // => AtomicI128; // when this stabilizes
        u128; // => AtomicU128; // when this stabilizes
    }
    "ptr" => bit num => {
        isize => AtomicIsize;
        usize => AtomicUsize;
    }
    "ptr" => no_bit no_num => {
        @<T> *mut T => AtomicPtr;
    }
}

impl<T> Radium for Atom<T>
where
    T: Atomic,
    T::Atom: Radium<Item = T>,
{
    type Item = T;

    fn fence(order: Ordering) {
        core::sync::atomic::fence(order);
    }

    radium!(wrappers);
}

impl<T> Radium for Isotope<T>
where
    T: Nuclear,
    T::Nucleus: Radium<Item = T>,
{
    type Item = T;

    fn fence(order: Ordering) {
        <T::Nucleus as Radium>::fence(order);
    }

    radium!(wrappers);
}

impl<T> Radium for Radon<T>
where
    T: Nuclear,
    Cell<T>: Radium<Item = T>,
{
    type Item = T;

    fn fence(_: Ordering) {}

    radium!(wrappers);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    #[allow(unused_imports)]
    use core::sync::atomic::*;
    use static_assertions::*;

    #[test]
    fn absent_traits() {
        assert_not_impl_any!(bool: NumericOps);
        assert_not_impl_any!(*mut u8: BitOps, NumericOps);
    }

    #[test]
    fn present_traits() {
        assert_impl_all!(bool: BitOps);
        assert_impl_all!(usize: BitOps, NumericOps);
    }

    #[test]
    fn always_cell() {
        assert_impl_all!(Cell<bool>: Radium<Item = bool>);
        assert_impl_all!(Cell<i8>: Radium<Item = i8>);
        assert_impl_all!(Cell<u8>: Radium<Item = u8>);
        assert_impl_all!(Cell<i16>: Radium<Item = i16>);
        assert_impl_all!(Cell<u16>: Radium<Item = u16>);
        assert_impl_all!(Cell<i32>: Radium<Item = i32>);
        assert_impl_all!(Cell<u32>: Radium<Item = u32>);
        assert_impl_all!(Cell<i64>: Radium<Item = i64>);
        assert_impl_all!(Cell<u64>: Radium<Item = u64>);
        assert_impl_all!(Cell<isize>: Radium<Item = isize>);
        assert_impl_all!(Cell<usize>: Radium<Item = usize>);
        assert_impl_all!(Cell<*mut ()>: Radium<Item = *mut ()>);
    }

    #[test]
    fn always_alias() {
        assert_impl_all!(RadiumBool: Radium<Item = bool>);
        assert_impl_all!(RadiumI8: Radium<Item = i8>);
        assert_impl_all!(RadiumU8: Radium<Item = u8>);
        assert_impl_all!(RadiumI16: Radium<Item = i16>);
        assert_impl_all!(RadiumU16: Radium<Item = u16>);
        assert_impl_all!(RadiumI32: Radium<Item = i32>);
        assert_impl_all!(RadiumU32: Radium<Item = u32>);
        assert_impl_all!(RadiumI64: Radium<Item = i64>);
        assert_impl_all!(RadiumU64: Radium<Item = u64>);
        assert_impl_all!(RadiumIsize: Radium<Item = isize>);
        assert_impl_all!(RadiumUsize: Radium<Item = usize>);
        assert_impl_all!(RadiumPtr<()>: Radium<Item = *mut ()>);

        assert_impl_all!(Isotope<bool>: Radium<Item = bool>);
        assert_impl_all!(Isotope<i8>: Radium<Item = i8>);
        assert_impl_all!(Isotope<u8>: Radium<Item = u8>);
        assert_impl_all!(Isotope<i16>: Radium<Item = i16>);
        assert_impl_all!(Isotope<u16>: Radium<Item = u16>);
        assert_impl_all!(Isotope<i32>: Radium<Item = i32>);
        assert_impl_all!(Isotope<u32>: Radium<Item = u32>);
        assert_impl_all!(Isotope<i64>: Radium<Item = i64>);
        assert_impl_all!(Isotope<u64>: Radium<Item = u64>);
        assert_impl_all!(Isotope<isize>: Radium<Item = isize>);
        assert_impl_all!(Isotope<usize>: Radium<Item = usize>);
        assert_impl_all!(Isotope<*mut ()>: Radium<Item = *mut ()>);
    }

    #[test]
    fn maybe_atom() {
        #[cfg(target_has_atomic = "8")]
        {
            assert_impl_all!(AtomicBool: Radium<Item = bool>);
            assert_impl_all!(AtomicI8: Radium<Item = i8>);
            assert_impl_all!(AtomicU8: Radium<Item = u8>);

            assert_impl_all!(Atom<bool>: Radium<Item = bool>);
            assert_impl_all!(Atom<i8>: Radium<Item = i8>);
            assert_impl_all!(Atom<u8>: Radium<Item = u8>);
        }
        #[cfg(target_has_atomic = "16")]
        {
            assert_impl_all!(AtomicI16: Radium<Item = i16>);
            assert_impl_all!(AtomicU16: Radium<Item = u16>);

            assert_impl_all!(Atom<i16>: Radium<Item = i16>);
            assert_impl_all!(Atom<u16>: Radium<Item = u16>);
        }
        #[cfg(target_has_atomic = "32")]
        {
            assert_impl_all!(AtomicI32: Radium<Item = i32>);
            assert_impl_all!(AtomicU32: Radium<Item = u32>);

            assert_impl_all!(Atom<i32>: Radium<Item = i32>);
            assert_impl_all!(Atom<u32>: Radium<Item = u32>);
        }
        #[cfg(target_has_atomic = "64")]
        {
            assert_impl_all!(AtomicI64: Radium<Item = i64>);
            assert_impl_all!(AtomicU64: Radium<Item = u64>);

            assert_impl_all!(Atom<i64>: Radium<Item = i64>);
            assert_impl_all!(Atom<u64>: Radium<Item = u64>);
        }
        #[cfg(target_has_atomic = "ptr")]
        {
            assert_impl_all!(AtomicIsize: Radium<Item = isize>);
            assert_impl_all!(AtomicUsize: Radium<Item = usize>);
            assert_impl_all!(AtomicPtr<()>: Radium<Item = *mut ()>);

            assert_impl_all!(Atom<isize>: Radium<Item = isize>);
            assert_impl_all!(Atom<usize>: Radium<Item = usize>);
            assert_impl_all!(Atom<*mut ()>: Radium<Item = *mut ()>);
        }
    }
}
