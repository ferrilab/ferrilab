<div style="text-align: center;" align="center">

# ![`funty`][logo] <!-- omit in toc -->

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
width, or signedness can be represented in the trait system. It also provides
an abstraction trait over memory-access permission, and wrapper types which
mirror those found in the standard library for working with the fundamentals in
more meaningful ways.

## Functionality Traits

All primitive value types (`bool`, `char`, `{i,u}{8,16,32,64,128,size}`, and
`f{32,64}`) implement the [`Fundamental`] trait. This trait defines the basic
concepts available to primitives: they are plain-old-data values, and can be
`as`-casted to each other. `Fundamental` has no functionality other than
providing the basic set of traits and allowing conversion. Note that pointers do
**not** implement `Fundamental`.

The numeric primitives (everything except `bool` and `char`) implement the
following trait hierarchy, found in the [`num`](`crate::num`) module:

- [`Numeric`] exports all the trait implementations and methods found on *all*
  numeric primitives.
  - [`Integral`] exports the trait implementations and methods found on all
    integers.
    - [`Signed`] unifies all signed integers `iN`.
    - [`Unsigned`] unifies all unsigned integers `uN`.
  - [`Floating`] unifies both floating-point numbers.

Funty provides a [`NonZero<T>`](`crate::num::NonZero`) wrapper and a `Zeroable`
trait which correspond to the `core::num::NonZero` wrapper and its unstable
marker trait. The [`funty::num::Zeroable`](`crate::num::Zeroable`) trait is
**only** to be used in trait bounds, and should be expected to be replaced with
the standard-library trait when/if it stabilizes.

## Width Traits

There are three trait families for type width. For `Width` values of `8`, `16`,
`32`, `64`, and `128`:

- `IsWidth` is implemented by the numbers that are exactly this width.
- `AtLeastWidth` is implemented by all numbers that are this width or wider.
- `AtMostWidth` is implemented by all numbers that are this width or narrower.

## Usage

Type [`use funty::prelude*;`](`crate::prelude`), then declare the traits you
need as generic bounds.

The prelude exports `NonZeroFty` so that it will not collide in a scope which
has already imported `core::num::NonZero`. You can also replace
`use core::num::NonZero;` with `use funty::num::NonZero;` and have your existing
code continue to work unchanged.

## Examples

Perform bit arithmetic on some unsigned integer:

```rust
use funty::num::Unsigned;
fn invert_middle_bits<T: Unsigned>(num: T) -> T {
  let mask = (T::MAX << 2) & (T::MAX >> 2);
  num ^ mask
}
assert_eq!(invert_middle_bits(0xAAu8), 0b1001_0110u8);
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

## Version Policy

This crate’s MSRV will always be the Rust version which introduced the edition
it currently uses. APIs stabilized after that version was released will be gated
behind `"rust_<major><minor>"` features. The `"rust_now"` feature uses the
current stable version at time of release.

This crate will raise minor versions as it catches up with the standard library.
It will raise major versions as it gains significant features or makes an API
break.

At time of writing, it uses edition 2024 (MSRV 1.85), and has features for each
Rust version since MSRV which stabilized new APIs for `funty` to mirror. Each
version-feature automatically enables all earlier version-features.

- `"rust_186"`
- `"rust_187"`: adds unbounded shifting, midpoint-finding, signage casting, and
  is-multiple-of testing to the numeric traits
- `"rust_188"`
- `"rust_189"`
- `"rust_190"`: adds graceful signed subtraction to the unsigned integers
- `"rust_191"`
- `"rust_192"`
- `"rust_193"`

<style type="text/css">
  h2 > img {
    background-image: url("data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSItMTEgLTEwNCAyMjUgMTEyIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxnIHN0eWxlPSJ0cmFuc2Zvcm06c2tld1goLTE4ZGVnKSI+PHBhdGggY2xhc3M9InBlbiBwZW4tc3EiIGQ9Ik0wLTI0di02Nmw2LTZoOTBNLTEyLTcyaDM2djQ4aDI0di00OG0yNCA0OHYtNDhoMTJsMTIgMTJ2MzZtMzYgMGgtMTJ2LTcyTTI0IDBoMTU2di03Mm0tMjQgMHYzNmwyNCAyNFYwIi8+PHBhdGggZD0iTTk2LTgwaDQ0djE2aC0yOG0wIDMyaDI4bDE2IDE2aC00NCIvPjxwYXRoIGNsYXNzPSJwZW4gcGVuLXNxIiBkPSJNMCAwaDAiIHN0eWxlPSJzdHJva2U6I2ZmNGYwMCIvPjwvZz48c3R5bGU+cGF0aHtzdHJva2U6IzAwMDtmaWxsOiMwMDA7c3Ryb2tlLXdpZHRoOjB9QG1lZGlhIChwcmVmZXJzLWNvbG9yLXNjaGVtZTpkYXJrKXtwYXRoe3N0cm9rZTojZmZmO2ZpbGw6I2ZmZn19KntzaGFwZS1yZW5kZXJpbmc6Y3Jpc3BFZGdlc30ucGVue2ZpbGw6bm9uZTtzdHJva2Utd2lkdGg6MTZweDtzdHJva2UtbGluZWNhcDpzcXVhcmV9PC9zdHlsZT48L3N2Zz4=");
    background-position: center center;
    background-repeat: no-repeat;
    display: block;
  }
  h2 > img::before { display: none; }
  h1 > img, h2 > img {
    height: 4em;
  }
</style>

<!-- Badges -->
[crate_link]: https://crates.io/crates/funty "Crate Link"
[docs_link]: https://docs.rs/funty/latest/funty "Documentation"
[docs_img]: https://img.shields.io/docsrs/funty/latest.svg?style=for-the-badge "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/funty.svg?style=for-the-badge "Crate Downloads"
[license_file]: https://github.com/ferrilab/ferrilab/blob/main/funty/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/funty.svg?style=for-the-badge "License Display"
[msrv_img]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.85"
[version_img]: https://img.shields.io/crates/v/funty?color=f46623&style=for-the-badge "Funty version badge"
[logo]: assets/funty.svg
