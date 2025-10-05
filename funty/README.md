<div style="text-align: center;" align="center">

# `funty` <!-- omit in toc -->

## `Fun`damental `Ty`pe Unification <!-- omit in toc -->

[![Latest Version][version_img]][crate_link]
[![MSRV][msrv_img]][crate_link]
[![License][license_img]][license_file]

[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

Prior to `1.0`, Rust had traits for the numeric primitive types to permit code
to generalize over which specific type it accepted. This was never stabilized,
and eventually removed.

This library provides a set of traits that abstract over common API surfaces of
the primitive types, so that properties such as numeric behavior, register
width, or signedness can be represented in the trait system.

## Version Policy

This crate’s MSRV will always be the Rust version which introduced the edition
it currently uses. APIs stabilized after that version was released will be gated
behind `"rust_<major><minor>"` features. The `"rust_now"` feature uses the
current stable version at time of release.

This crate will raise minor versions as it catches up with the standard library.
It will raise major versions as it gains significant features or makes an API
break.

At time of writing, it uses edition 2024 (MSRV 1.85), and has features
`"rust_187"` and `"rust_190"`. Rust versions 1.86, 1.88, and 1.89 did not change
primitive APIs.

## Pointer Unification

`*const T` and `*mut T` are unified under the `Pointer<T, Shared | Unique>`
type. The `Permission` trait allows code to be generic over write permissions,
and manages propagating, downgrading, and upgrading permissions correctly
without risking violations of Rust’s provenance tracking rules.

In particular, `Pointer` uses the associated-type system to internally wrap
either `*const T` or `*mut T` according to the `Permission` type parameter it is
given, so user code is never able to (safely) improperly upgrade write
permissions on a pointer that is derived from a read-only provenance history.

See the [`ptr`] module for more details.

## Functionality Traits

All primitive types (`bool`, `char`, `{i,u}{8,16,32,64,128,size}`, and
`f{32,64}`) implement the `Fundamental` trait. This trait defines the basic
concepts available to primitives: they are plain-old-data values, and can be
`as`-casted to each other. `Fundamental` has no functionality other than
providing the basic set of traits and allowing conversion.

The numeric primitives (everything except `bool` and `char`) implement the
following trait hierarchy:

- `Numeric` exports all the trait implementations and methods found on *all*
  numeric primitives.
  - `Integral` exports the trait implementations and methods found on all
    integers.
    - `Signed` unifies all signed integers `iN`.
    - `Unsigned` unifies all unsigned integers `uN`.
  - `Floating` unifies both floating-point numbers.

## Width Traits

There are three trait families for type width. For `Width` values of `8`, `16`,
`32`, `64`, and `128`:

- `IsWidth` is implemented by the numbers that are exactly this width.
- `AtLeastWidth` is implemented by all numbers that are this width or wider.
- `AtMostWidth` is implemented by all numbers that are this width or narrower.

## Usage

Type `use funty::*;`, then declare the traits you need as generic bounds.

## Examples

Perform bit arithmetic on some unsigned integer:

```rust
use funty::Unsigned;
fn invert_middle_bits<T: Unsigned>(num: T) -> T {
  let mask = (!T::ZERO).wrapping_shl(2).wrapping_shr(4).wrapping_shl(2);
  num ^ mask
}
```

## `#![no_std]` Compatibility

The floating-point numbers offer many functions which are implemented in the
target system’s `libm`. This library is present only in `std`-targets. If you
are compiling to a `#![no_std]` target, depend on this library with

```toml
[dependencies.funty]
version = "3"
default-features = false
```

<!-- Badges -->
[crate_link]: https://crates.io/crates/funty "Crate Link"
[docs_link]: https://docs.rs/funty/latest/funty "Documentation"
[docs_img]: https://img.shields.io/docsrs/funty/latest.svg?style=for-the-badge "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/funty.svg?style=for-the-badge "Crate Downloads"
[license_file]: https://github.com/ferrilab/ferrilab/blob/master/funty/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/funty.svg?style=for-the-badge "License Display"
[msrv_img]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.85"
[version_img]: https://img.shields.io/crates/v/funty?color=f46623&style=for-the-badge "Funty version badge"

<!-- Documentation -->
[`ptr`]: https://docs.rs/funty/latest/funty/ptr "The `ptr` module API docs"
