# Changelog

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

## 3.0

Added `Integral::Signed` and `Integral::Unsigned` associated types, as some
arithmetic functions (`abs_diff`) perform a sign change.

The `Pointer` and `NonNullPtr` newtypes generalize over `*const`/`*mut` pointers
and allow users to be generic over write permissions. In addition, the
`Reference` type alias allows the same patterns to be written with references
(which do not need a newtype, as references have no inherent behavior and must
remain dereferenceäble).

The MSRV is now 1.65, as the pointer system requires the Generic Associated
Types functionality.

## 2.0

The `Is{Characteristic}` traits were renamed to be adjectives, rather than
statements of being, to fit the Rust style.

In addition, a new baseline trait, `Fundamental`, generalizes over **all** the
Rust fundamentals, including `bool` and `char`, and provides conversions between
them. The remaining trait hierarchy, as implemented only on the numerics,
remains in place.

The MSRV is now 1.53, with introduction of the `::BITS` constant in the standard
library.

## 1.2.1, 1.1.1, and 1.0.1

`IsNumber` now requires a `'static` lifetime. This is not a breaking change, as
all implementors are already `'static`.

## 1.2

Add `BITS` in preparation for [`rust-lang/rust#76904`].

## 1.1

Add `leading_ones` and `trailing_ones`, as these methods were introduced in Rust
1.46.

## 1.0

Library creation.

### Added

The `IsNumber`, `IsInteger`, `IsFloat`, `IsSigned`, and `IsUnsigned` traits
generalize over the primitive integer types, forwarding all of their stable
constants, methods, and trait implementations.

The width relation traits allow users to constrain the widths of numbers they
require.

[kac]: //keepachangelog.com/en/1.0.0/
[`rust-lang/rust#76904`]: https://github.com/rust-lang/rust/issues/76904
