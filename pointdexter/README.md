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

This library was extracted from `bitvec`, where it was first built to support
the custom pointer type that powers it. That crate’s source code is likely the
best (and also likely only) example of how Pointdexter is meant to be used as an
implementation detail of other libraries.

Pointdexter is a long name. Its prelude aliases the crate to `ptxr`. If you feel
very confident, you can alias it to `ptr` and completely shadow the core module.

```rust
use pointdexter::prelude::*;

let mut data = 5i32;

let cptr: ptxr::Pointer<i32, Shared> = (&data).into();
let mptr: ptxr::Pointer<i32, Unique> = (&mut data).into();
```

## Rust Version Compatibility

This crate begins its Rust support at 1.85, and uses feature gates to add APIs
for successive releases.

If you want to use this crate on earlier Rust versions, please file an issue and
I will lower the floor accordingly.

Enable the feature `rust_1xy`, where `xy` is the minor version of Rust you use,
to enable the pointer APIs that are stable in your compiler. Use `rust_now` if
you track the stable release train. Features begin at `rust_186`, as Rust 1.85
is the baseline and is always available.

Note that `rust_now` is a **default feature**! If you are pinning a Rust rather
than floating on the stable series, you must set

```toml
[dependencies.pointdexter]
default-features = false
features = ["rust_1xy"]
```

<style type="text/css">
  h2 > img {
    background-image: url("data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSItMTkgLTgwIDUxNCAxMTIiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGcgc3R5bGU9InRyYW5zZm9ybTpza2V3WCgtMThkZWcpIj48cGF0aCBjbGFzcz0icGVuIiBkPSJNMCAyNHYtNzJoMjRWMEgwbTQ4IDB2LTQ4aDI0VjBINDhtNDggMHYtNDhtMjQgNDh2LTQ4aDEybDEyIDEyVjBtMjQtNzJWMGgxMm00OC03MlYwaC0yNHYtNDhoMjRtMjQgMjRoMjR2LTI0aC0yNFYwaDI0bTI0IDB2LTEybDI0LTI0di0xMm0tMjQgMHYxMmwyNCAyNFYwbTQ4IDBoLTEydi03Mm0tMTIgMjRoMjRtMjQgMjRoMjR2LTI0aC0yNFYwaDI0bTI0IDB2LTQ4aDI0Ii8+PHBhdGggZD0iTTE0NC01Nmg0NHYxNmgtMjgiLz48cGF0aCBjbGFzcz0icGVuIiBkPSJNOTYtNzJoMCIgc3R5bGU9InN0cm9rZTojZmY0ZjAwIi8+PC9nPjxzdHlsZT5wYXRoe3N0cm9rZTojMDAwO2ZpbGw6IzAwMH1AbWVkaWEgKHByZWZlcnMtY29sb3Itc2NoZW1lOmRhcmspe3BhdGh7c3Ryb2tlOiNmZmY7ZmlsbDojZmZmfX0ucGVue2ZpbGw6bm9uZTtzdHJva2Utd2lkdGg6MTZweDtzdHJva2UtbGluZWNhcDpzcXVhcmV9PC9zdHlsZT48L3N2Zz4=");
    background-position: center center;
    background-repeat: no-repeat;
    display: block;
  }
  h2 > img::before { display: none; }
  h1 > img, h2 > img {
    height: 4em;
  }
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

[logo]: assets/pointdexter.svg
