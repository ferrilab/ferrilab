<div style="text-align: center;" align="center">

# ![Pointdexter][logo] <!-- omit in toc -->

## *Point*er *Dexter*ity <!-- omit in toc -->

[![Latest Version][version_img]][crate_link]
[![MSRV][msrv_img]][crate_link]
[![License][license_img]][license_file]

[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

In the Rust standard library, `*const T` and `*mut T` are wholly discrete and
unrelated types, except that `*mut T` coerces into `*const T` via a compiler
builtin. `*mut T` is notionally a subtype of `*const T`, but this relation is
not expressed in the type system available to Rust user code.

This crate produces a unifying wrapper type over both pointers, called
[`Pointer`]. It, like the raw pointer primitives in the standard library, is
generic over the pointee `T`. Unlike the raw pointer primitives, it is also
generic over a [`P: Permission`][Permission] token.

## The Permission System

The `Permission` trait and its implementors, [`Shared`] and [`Unique`], provide
a library-level reification of the [Stacked Borrows] model being built inside
the Rust compiler’s [provenance] system. You can see in the Stacked Borrows
documentation that it has a similar permission system that describes Unique and
two different kinds of Shared. Pointdexter views the `SharedReadWrite` and
`SharedReadOnly` values not as discrete states, but as stateful lenses over an
original provenance.

In particular, any `Unique` permission in Pointdexter can be degraded to an
equivalent of the Stacked-Borrows `SharedReadWrite` by pushing it into a type
stack, `(Shared, Unique)`. Pointdexter implements the `Permission` trait on all
`(Shared, impl Permission)` tuples, which allows client code to always be able
to push and unwind sharing information and change the *type* of a `Pointer`,
rather than add runtime state that causes ABI changes.

## Usage

This crate is *primarily* useful for other library or intermediate code that
needs to generalize over write permissions. Applications tend to not need to be
general in the same way and may benefit less from it.

All pointers and references can be converted into their corresponding `Pointer`,
`Reference`, or `NonNullPointer` using `.into()`, `.try_into()`, or `::new()`.
Once converted, all stable standard-library APIs continue to exist.

## Rust Version Compatibility

This crate begins its Rust support at 1.85, and uses feature gates to add APIs
for successive releases.

If you want to use this crate on earlier Rust versions, please file an issue and
I will lower the floor accordingly.

Enable the feature `rust_1min`, where `min` is the minor version of Rust you
use, to enable the pointer APIs that are stable in your compiler. Use `rust_now`
if you track the stable release train. Features begin at `rust_186`, as Rust
1.85 is the baseline and is always available.

<style type="text/css">
  h1 img, h2 img { height: 4em; }
</style>

<!-- Documentation -->
[Stacked Borrows]: https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md
[provenance]: https://doc.rust-lang.org/core/ptr/index.html#provenance

<!-- Badges -->
[crate_link]: https://crates.io/crates/pointdexter "Crate Link"
[docs_link]: https://docs.rs/pointdexter/latest/pointdexter "Documentation"
[docs_img]: https://img.shields.io/docsrs/pointdexter/latest.svg?style=for-the-badge "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/pointdexter.svg?style=for-the-badge "Crate Downloads"
[license_file]: https://github.com/ferrilab/ferrilab/blob/main/pointdexter/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/pointdexter.svg?style=for-the-badge "License Display"
[msrv_img]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.85"
[version_img]: https://img.shields.io/crates/v/pointdexter?color=f46623&style=for-the-badge "pointdexter version badge"

[logo]: ../assets/pointdexter.svg
