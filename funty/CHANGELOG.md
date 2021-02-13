# Changelog

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

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
