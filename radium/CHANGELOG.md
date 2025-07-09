# Changelog

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

## Version 1

### 1.1.1

- Deconflicted a symbol usage (`Atomic`) introduced by a nightly feature in the
  standard library. See [Rust #130539]

### 1.1.0

- Added `Radium` implementations for the [`portable-atomic`] crate.
- Relaxed `Radium` from being `Sealed` (forbidding implementations outside this
  crate) to merely `unsafe`. Foreign crates may now provide implementations as
  appropriate.

### 1.0.0

The `Radium` trait and `RadiumT` type aliases are carried over from version 0.
The `if_atomic!` macro is removed, as the compiler stabilized
`#[cfg(target_has_atomic)]`.

Additionally, the `Atom<T>`, `Isotope<T>`, and `Radon<T>` type families allow
user code to be generic over the integral primitive being managed. These types
only accept primitives which can be made atomic, and `Atom` will reject
primitives that do not have atomic instructions on the target.

## Version 0

The development series created the `Radium` trait and `RadiumT` type aliases. It
used a build script with manually-provided configuration values to select for
the atomics that each target actually supported.

It will receive target updates as needed for clients who need to use Rust
versions older than 1.60, but will not receive a backport of the 1.0 API.

[Rust #130539]: https://github.com/rust-lang/rust/issues/130539
[kac]: https://keepachangelog/en/1.0.0
[`portable-atomic`]: https://docs.rs/portable-atomic/1
