//! Guard macros
//!
//! The build-script-defined `cfg` flags are not available outside this crate,
//! and so client crates cannot use them to conditionally compile their code on
//! the presence or absence of atomic types of a certain width.
//!
//! Instead, these `macro_rules!` macros are configured *within `radium`* to
//! either preserve or discard their contents, so that callers can wrap code in
//! them to make the contents conditional on the presence of the requested
//! atomic width.
//!
//! For each width in `8`, `16`, `32`, `64`, and `ptr`, the macro
//! `has_atomic_WIDTH` preserves its contents when `cfg(radium_atomic_WIDTH)` is
//! set and destroys them when it is not. Its counterpart macro,
//! `not_atomic_WIDTH`, destroys its contents when this `cfg` flag is set and
//! preserves them when it is not.

/// Preserves its contents, as your target supports 8-bit atomics.
#[macro_export]
#[cfg(radium_atomic_8)]
macro_rules! has_atomic_8 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Destroys its contents, as your target does not support 8-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_8))]
macro_rules! has_atomic_8 {
    ( $($t:tt)* ) => {};
}

/// Destroys its contents, as your target supports 8-bit atomics.
#[macro_export]
#[cfg(radium_atomic_8)]
macro_rules! not_atomic_8 {
    ( $($t:tt)* ) => {};
}

/// Preserves its contents, as your target does not support 8-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_8))]
macro_rules! not_atomic_8 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Preserves its contents, as your target supports 16-bit atomics.
#[macro_export]
#[cfg(radium_atomic_16)]
macro_rules! has_atomic_16 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Destroys its contents, as your target does not 16-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_16))]
macro_rules! has_atomic_16 {
    ( $($t:tt)* ) => {};
}

/// Destroys its contents, as your target supports 16-bit atomics.
#[macro_export]
#[cfg(radium_atomic_16)]
macro_rules! not_atomic_16 {
    ( $($t:tt)* ) => {};
}

/// Preserves its contents, as your target does not support 16-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_16))]
macro_rules! not_atomic_16 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Preserves its contents, as your target supports 32-bit atomics.
#[macro_export]
#[cfg(radium_atomic_32)]
macro_rules! has_atomic_32 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Destroys its contents, as your target does not 32-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_32))]
macro_rules! has_atomic_32 {
    ( $($t:tt)* ) => {};
}

/// Destroys its contents, as your target supports 32-bit atomics.
#[macro_export]
#[cfg(radium_atomic_32)]
macro_rules! not_atomic_32 {
    ( $($t:tt)* ) => {};
}

/// Preserves its contents, as your target does not support 32-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_32))]
macro_rules! not_atomic_32 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Preserves its contents, as your target supports 64-bit atomics.
#[macro_export]
#[cfg(radium_atomic_64)]
macro_rules! has_atomic_64 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Destroys its contents, as your target does not 64-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_64))]
macro_rules! has_atomic_64 {
    ( $($t:tt)* ) => {};
}

/// Destroys its contents, as your target supports 64-bit atomics.
#[macro_export]
#[cfg(radium_atomic_64)]
macro_rules! not_atomic_64 {
    ( $($t:tt)* ) => {};
}

/// Preserves its contents, as your target does not support 64-bit atomics.
#[macro_export]
#[cfg(not(radium_atomic_64))]
macro_rules! not_atomic_64 {
    ( $($t:tt)* ) => { $($t)* };
}

/// Preserves its contents, as your target supports pointer-width atomics.
#[macro_export]
#[cfg(radium_atomic_ptr)]
macro_rules! has_atomic_ptr {
    ( $($t:tt)* ) => { $($t)* };
}

/// Destroys its contents, as your target does not pointer-width atomics.
#[macro_export]
#[cfg(not(radium_atomic_ptr))]
macro_rules! has_atomic_ptr {
    ( $($t:tt)* ) => {};
}

/// Destroys its contents, as your target supports pointer-width atomics.
#[macro_export]
#[cfg(radium_atomic_ptr)]
macro_rules! not_atomic_ptr {
    ( $($t:tt)* ) => {};
}

/// Preserves its contents, as your target does not support pointer-width
/// atomics.
#[macro_export]
#[cfg(not(radium_atomic_ptr))]
macro_rules! not_atomic_ptr {
    ( $($t:tt)* ) => { $($t)* };
}

#[macro_export]
#[cfg(any(
    radium_atomic_8,
    radium_atomic_16,
    radium_atomic_32,
    radium_atomic_64,
    radium_atomic_ptr
))]
macro_rules! has_atomic_any {
    ( $($t:tt)* ) => { $($t)* };
}

#[macro_export]
#[cfg(not(any(
    radium_atomic_8,
    radium_atomic_16,
    radium_atomic_32,
    radium_atomic_64,
    radium_atomic_ptr
)))]
macro_rules! has_atomic_any {
    ( $($t:tt)* ) => {};
}

#[macro_export]
#[cfg(not(any(
    radium_atomic_8,
    radium_atomic_16,
    radium_atomic_32,
    radium_atomic_64,
    radium_atomic_ptr
)))]
macro_rules! not_atomic_any {
    ( $($t:tt)* ) => {};
}

#[macro_export]
#[cfg(any(
    radium_atomic_8,
    radium_atomic_16,
    radium_atomic_32,
    radium_atomic_64,
    radium_atomic_ptr
))]
macro_rules! not_atomic_any {
    ( $($t:tt)* ) => { $($t)* };
}

#[cfg(test)]
mod tests {
    #[test]
    fn atomic_8() {
        let mut counter = 0;
        crate::has_atomic_8! {
            counter += 1;
        }
        crate::not_atomic_8! {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn atomic_16() {
        let mut counter = 0;
        crate::has_atomic_8! {
            counter += 1;
        }
        crate::not_atomic_8! {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn atomic_32() {
        let mut counter = 0;
        crate::has_atomic_8! {
            counter += 1;
        }
        crate::not_atomic_8! {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn atomic_64() {
        let mut counter = 0;
        crate::has_atomic_8! {
            counter += 1;
        }
        crate::not_atomic_8! {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn atomic_ptr() {
        let mut counter = 0;
        crate::has_atomic_8! {
            counter += 1;
        }
        crate::not_atomic_8! {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }
}
