//! Guard macros
//!
//! The build-script-defined `cfg` flags are not available outside this crate,
//! and so client crates cannot use them to conditionally compile their code on
//! the presence or absence of atomic types of a certain width.
//!
//! Instead, the `radium::if_atomic!` macro implements conditional compilation
//! based on target atomic availability. It takes an `if`-expression that tests
//! the `atomic(WIDTH)` condition, and produces code depending on whether the
//! requested width has atomic instructions. An optional `else`-block allows you
//! to write fallback code used when the requested width does not have atomic
//! instructions.
//!
//! The acceptable arguments to `atomic` are:
//!
//! - `8`
//! - `16`
//! - `32`
//! - `64`
//! - `ptr`
//! - `bool`: alias for `8`
//! - `size`: alias for `ptr`
//!
//! In addition, the `atomic()` test can be inverted, as `!atomic()`, to reverse
//! the preserve/destroy behavior of the `if` and `else` blocks.
//!
//! This macro can be used in any position.
//!
//! # Examples
//!
//! This demonstrates that the `if_atomic!` macro can be called in any position.
//!
//! Multiple `if` clauses can be placed in the same macro call to create
//! multiple items or statements, but only one `if`/`else` clause may be present
//! when the macro is used in expression or type position.
//!
//! ```rust
//! radium::if_atomic! {
//!   if atomic(size) { use core::sync::atomic::AtomicUsize; }
//!   if !atomic(size) { use core::cell::Cell; }
//! }
//!
//! struct RadiumRc<T: ?Sized> {
//!   strong: radium::if_atomic! {
//!     if atomic(ptr) { AtomicUsize }
//!     else { Cell<usize> }
//!   },
//!   weak: radium::types::RadiumUsize,
//!   data: T,
//! }
//! ```

#[doc(hidden)]
mod inner {
    #[macro_export]
    #[cfg(radium_atomic_8)]
    macro_rules! __radium_if_atomic_8 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($a)* }
    }

    #[macro_export]
    #[cfg(not(radium_atomic_8))]
    macro_rules! __radium_if_atomic_8 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($b)* }
    }

    #[macro_export]
    #[cfg(radium_atomic_16)]
    macro_rules! __radium_if_atomic_16 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($a)* }
    }

    #[macro_export]
    #[cfg(not(radium_atomic_16))]
    macro_rules! __radium_if_atomic_16 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($b)* }
    }

    #[macro_export]
    #[cfg(radium_atomic_32)]
    macro_rules! __radium_if_atomic_32 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($a)* }
    }

    #[macro_export]
    #[cfg(not(radium_atomic_32))]
    macro_rules! __radium_if_atomic_32 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($b)* }
    }

    #[macro_export]
    #[cfg(radium_atomic_64)]
    macro_rules! __radium_if_atomic_64 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($a)* }
    }

    #[macro_export]
    #[cfg(not(radium_atomic_64))]
    macro_rules! __radium_if_atomic_64 {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($b)* }
    }

    #[macro_export]
    #[cfg(radium_atomic_ptr)]
    macro_rules! __radium_if_atomic_ptr {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($a)* }
    }

    #[macro_export]
    #[cfg(not(radium_atomic_ptr))]
    macro_rules! __radium_if_atomic_ptr {
        ( [ $( $a:tt )* ] [ $( $b:tt )* ] ) => { $($b)* }
    }
}

/// Conditional compilation based on the presence of atomic instructions.
///
/// This macro allows you to write `if`/`else` clauses, evaluated at
/// compile-time, that test the presence of atomic instructions and preserve or
/// destroy their guarded code accordingly.
///
/// The `if atomic(WIDTH)` test preserves the contents of its block when the
/// target architecture has atomic instructions for the requested `WIDTH`, and
/// removes them from the syntax tree when the target does not. If an `else`
/// clause is provided, the contents of the `else` block are used as a
/// substitute when the `if` is destroyed.
///
/// This macro can be used in any position. When it is used in item or statement
/// position, it can contain multiple `if` clauses, and each will be evaluated
/// in turn. Expression and type positions can only accept exactly one code
/// span, and so may only have exactly one `if`/`else` clause. An `else` clause
/// is required here so that the macro will always expand to something; an empty
/// expansion is a parse error.
///
/// # Macro Syntax
///
/// The macro contents `if atomic() {} else {}` are part of the macro
/// invocation. Only the contents of the two blocks are actual Rust code.
///
/// The acceptable arguments to `atomic()` are:
///
/// - `8`
/// - `16`
/// - `32`
/// - `64`
/// - `ptr`
/// - `bool`: alias for `8`
/// - `size`: alias for `ptr`
///
/// In addition, the `atomic()` test can be inverted, as `!atomic()`, to reverse
/// the preserve/destroy behavior of the `if` and `else` blocks.
///
/// # Examples
///
/// This demonstrates the use of `if_atomic!` to produce multiple statements,
/// and then to produce a single type-name.
///
/// ```rust
/// radium::if_atomic! {
///   if atomic(size) { use core::sync::atomic::AtomicUsize; }
///   if !atomic(size) { use core::cell::Cell; }
/// }
///
/// struct RadiumRc<T: ?Sized> {
///   strong: radium::if_atomic! {
///     if atomic(ptr) { AtomicUsize }
///     else { Cell<usize> }
///   },
///   weak: radium::types::RadiumUsize,
///   data: T,
/// }
/// ```
#[macro_export]
macro_rules! if_atomic {
    ( if atomic(8) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::__radium_if_atomic_8! {
            [ $($a)* ] [ $( $($b)* )? ]
        }
        $($crate::if_atomic! { if $($rest)* })?
    };

    ( if atomic(16) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::__radium_if_atomic_16! {
            [ $($a)* ] [ $( $($b)* )? ]
        }
        $( $crate::if_atomic! { if $($rest)* } )?
    };

    ( if atomic(32) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::__radium_if_atomic_32! {
            [ $($a)* ] [ $( $($b)* )? ]
        }
        $( $crate::if_atomic! { if $($rest)* } )?
    };

    ( if atomic(64) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::__radium_if_atomic_64! {
            [ $($a)* ] [ $( $($b)* )? ]
        }
        $( $crate::if_atomic! { if $($rest)* } )?
    };

    ( if atomic(ptr) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::__radium_if_atomic_ptr! {
            [ $($a)* ] [ $( $($b)* )? ]
        }
        $( $crate::if_atomic! { if $($rest)* } )?
    };

    ( if atomic(bool) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::if_atomic! {
            if atomic(8) { $($a)* } $( else { $($b)* } )? $( if $($rest)* )?
        }
    };

    ( if atomic(size) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::if_atomic! {
            if atomic(ptr) { $($a)* } $( else { $($b)* } )? $( if $($rest)* )?
        }
    };

    ( if ! atomic( $t:tt ) { $($a:tt)* } $( else { $($b:tt)* } )? $( if $($rest:tt)* )? ) => {
        $crate::if_atomic! {
            if atomic($t) { $( $($b)* )? } else { $($a)* } $( if $($rest)* )?
        }
    };
}
