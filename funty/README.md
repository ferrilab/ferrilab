<div class="title-block" style="text-align: center;" align="center">

# `funty` <!-- omit in toc -->

## `Fun`damental `Ty`pe Unification <!-- omit in toc -->

[![Crate][crate_img]][crate]
[![Documentation][docs_img]][docs]
[![License][license_img]][license_file]

[![Crate Downloads][downloads_img]][crate]
[![Crate Size][loc_img]][loc]

</div>

Prior to `1.0`, Rust had traits for the numeric primitive types to permit code
to generalize over which specific type it accepted. This was never stabilized,
and eventually removed.

This library reïnstates these traits.

## MSRV

Each minor version of this library raises the MSRV to the current stable at the
time of release.

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
version = "1"
default-features = false
```

<!-- Badges -->
[crate]: https://crates.io/crates/funty "Crate Link"
[crate_img]: https://img.shields.io/crates/v/funty.svg?logo=rust "Crate Page"
[docs]: https://docs.rs/funty "Documentation"
[docs_img]: https://docs.rs/funty/badge.svg "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/funty.svg?logo=rust "Crate Downloads"
[license_file]: https://github.com/myrrlyn/funty/blob/master/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/funty.svg "License Display"
[loc]: https://github.com/myrrlyn/funty "Repository"
[loc_img]: https://tokei.rs/b1/github/myrrlyn/funty?category=code "Repository Size"
