<div style="text-align: center;" align="center">

# Radium

[![Latest Version][version_img]][crate_link]
[![MSRV][msrv_img]][crate_link]

[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

Radium provides abstractions and graceful degradation for behavior that *must*
be shared-mutable, but merely *may* use atomic instructions to do so.

The primary export is the [`Radium`] trait. This is implemented on all symbols
in the [`atomic`] module, and on their [`Cell<T>`] equivalents, and presents the
atomic inherent API as a trait. Your code can be generic over `Radium`, use a
stable and consistent API, and permit callers to select atomic or `Cell`
behavior as they need.

Additionally, the types [`Atom<T>`] and [`Isotope<T>`] provide strong and weak
guarantees of atomic behavior, respectively, while being generic over the
underlying primitive type. `Atom<T>` will fail to compile when instantiated with
a primitive which does not have a corresponding `AtomicT` type defined for the
target, while `Isotope<T>` always compiles, and falls back to enclosing a
`Cell<T>` when the required atomic support is missing.

Since the type symbols in the `atomic` module are conditionally defined
according to the target architecture’s atomic support, portable code cannot use
these names directly without placing them behind a `#[cfg(target_has_atomic)]`
gate. Instead, `radium` provides `RadiumT` type aliases which resolve to
`AtomicT` when available and `Cell<T>` when not.

The Rust compiler [stabilized] the `cfg(target_has_atomic)` attribute in version
1.60. This is the MSRV for Radium 1.0. The version-0 series will stay supported
for the indeterminate future to allow for pre-1.60 projects to continue to use
it. The `radium::if_atomic!` macro allows projects to simulate
`#[cfg(target_has_atomic)]` in version-0, but is removed in version-1.

This crate is `#![no_std]`-compatible, as it relies solely on the
`core::sync::atomic` and `core::cell` modules.

## Versioning

`radium` is by definition attached to the Rust standard library. As the atomic
API evolves, `radium` will follow it. MSRV raising is always at least a
minor-version increase.

As of Rust 1.60, support for 128-bit atomics is unstable. Since `radium` commits
to being usable on the stable release series, it does not support 128-bit
atomics. As a compromise, `Cell<{i,u}128>` *is* integrated with `radium` to
prepare for stabilization in the future.

If 128-bit atomics are removed from the standard library without stabilization,
`radium` will similarly remove support for `Cell<{i,u}128>` in a major-version
increase.

----

## Pre-1.60 Target Discovery

Because the compiler did not easily expose this information to libraries,
`radium` used a build script to detect the target architecture and emit its own
directives that mark the presence or absence of an atomic integer. We accomplish
this by reading the compiler’s target information records and copying the
information directly into the build script.

If `radium` does not work for your architecture, please update the build script
to handle your target string and submit a pull request. We write the build
script on an as-needed basis; it is not proactively filled with all of the
information listed in the compiler.

**NOTE**: The build script receives information through two variables: `TARGET`
and `CARGO_CFG_TARGET_ARCH`. The latter is equivalent to the value in
`cfg!(target_arch = "")`; however, this value **does not** contain enough
information to fully disambiguate the target. The build script attempts to do
rudimentary parsing of the `env!(TARGET)` string; if this does not work for your
target, consider using the `TARGET_ARCH` matcher, or match on the full `TARGET`
string rather than the parse attempt.

----

## Project Origins

**@kneecaw** - <https://twitter.com/kneecaw/status/1132695060812849154>
> Feelin' lazy: Has someone already written a helper trait abstracting
> operations over `AtomicUsize` and `Cell<usize>` for generic code which may
> not care about atomicity?

**@ManishEarth** - <https://twitter.com/ManishEarth/status/1132706585300496384>
> no but call the crate radium
>
> (since people didn't care that it was radioactive and used it in everything)

<!-- Badges -->
[crate_link]: https://crates.io/crates/raidum "Crates.io package"
[docs_img]: https://img.shields.io/docsrs/radium/latest.svg?style=for-the-badge "Radium documentation badge"
[docs_link]: https://docs.rs/radium "Radium documentation"
[downloads_img]: https://img.shields.io/crates/dv/radium.svg?style=for-the-badge "Crate downloads"
[msrv_img]: https://img.shields.io/badge/MSRV-1.60-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.60"
[version_img]: https://img.shields.io/crates/v/radium?color=f46623&style=for-the-badge "Radium version badge"

<!-- Documentation -->
[`Atom<T>`]: https://docs.rs/radium/latest/radium/types/struct.Atom.html
[`Cell<T>`]: https://doc.rust-lang.org/core/cell/struct.Cell.html
[`Isotope<T>`]: https://docs.rs/radium/latest/radium/types/struct.Isotope.html
[`Radium`]: https://docs.rs/radium/latest/radium/trait.Radium.html
[`atomic`]: https://doc.rust-lang.org/core/sync/atomic

<!-- External links -->
[stabilized]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1600-2022-04-07
