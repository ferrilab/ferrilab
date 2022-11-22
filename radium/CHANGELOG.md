# Changelog

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

## Version 1

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

[kac]: https://keepachangelog/en/1.0.0
