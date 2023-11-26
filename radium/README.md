<div style="text-align: center;" align="center">

# Radium

[![Latest Version][version_img]][crate_link]
[![MSRV][msrv_img]][crate_link]

[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

Radium provides unifying abstractions and graceful degradation for code that
requires shared-mutability, but does not necessarily require hardware-level
atomicity to provide it.

The primary export is the [`Radium`] trait. This is implemented on all of the
types in the standard library’s [`atomic`] module, as well as the [`Cell`]
wrappers over `bool`, the integers, and mutable pointers. Your code can be
generic over the `Radium` trait and use a stable and consistent API, and permit
client code to provide atomic or non-atomic types as they are able.

Additionally, Radium provides three type families with varying guarantees of
atomic behavior: [`Atom<T>`] wraps the standard library atomics, and only
accepts `T` parameters where the target has an `AtomicT` type; [`Isotope<T>`]
accepts any of the types which could be atomic, and wraps atomics where they
exist and silently decays to `Cell<T>` where they do not; and [`Radon<T>`] wraps
`Cell<T>`. All three of these types have no API except for implementing
`Radium`, `Debug`, `Default`, and `From<T>`, so your code can switch between
them without needing to worry about changing usage.

Lastly, Radium provides `RadiumT` type aliases matching all of the `AtomicT`
type names in the standard library. Each of these aliases forwards to its atomic
variant when it exists, and to `Cell<T>` when it does not. Your code can use
these names to be portable across targets with varying levels of atomic support
without having to worry about the fact that `AtomicT` symbols vanish on targets
that do not have the requisite atomic instructions.

The Rust compiler [stabilized][0] the `cfg(target_has_atomic)` test in version
1.60. This is now the MSRV for Radium 1.0. The version-0 series will stay
supported for the indeterminate future to allow for pre-1.60 projects to
continue to use it. The `radium::if_atomic!` macro allows projects to simulate
`#[cfg(target_has_atomic)]` in version-0, but is removed in version-1.

This crate is `#![no_std]`-compatible, as it relies solely on the
`core::sync::atomic` and `core::cell` modules.

## Versioning

Radium is by definition attached to the Rust standard library. As the atomic API
evolves, Radium will follow it. MSRV raising is always at least a minor-version
increase.

As of Rust 1.60, support for 128-bit atomics is still unstable. Since Radium
commits to being usable on the stable release series, it does not support
128-bit atomics. As a compromise, `Cell<{i,u}128>` *is* integrated with Radium
to prepare for stabilizaation in the future.

If 128-bit atomics are removed from the standard library without stabilization,
Radium will remove support for `Cell<{i,u}128>` in a major-version increase.

## Non-Standard Implementors

In addition to the Rust standard library `Cell` and `Atomic` types, we also
provide an implementation for the [`portable-atomic`] crate. However, the
`portable-atomic` implementation cannot compile on a select few targets. As of
1.60, they are:

- `thumbv6m-none-eabi`
- `riscv32i-unknown-none-elf`
- `riscv32imc-unknown-none-elf`

These targets have 32-bit atomic load and store instructions, but do not have
read/modify/write instructions. Since `Radium` demands RMU behavior, and
`portable-atomic` does not provide it even in software (the `.fetch_action`
methods are all missing), we do not attempt to handle these targets gracefully
and simply allow the compile to fail.

Do not use the `portable-atomic` feature when compiling for these targets.

We disable all `portable-atomic` features, including the default-on `fallback`
feature. This causes `portable-atomic` to only generate symbols that match what
the standard library provides on that target. If you enable
`portable-atomic/fallback` in your own crate, then these symbols will exist, but
`radium` will not be able to see them because `#[cfg(feature = "...")]` cannot
query *other* crates’ enabled feature set. You will need to set radium’s
`portable-atomic-fallback` feature to get `Radium` implementations for atomic
operations wider than what the target instruction set supports.

## Pre-1.60 Target Discovery

Because the compiler did not make atomic support on targets accessible to
libraries, Radium used a build script to detect the target architecture and emit
its own directives that marked the presence or absence of an atomic integer. We
accomplished this by reading the compiler’s target information records and
copying the information directly into our build script.

If Radium v0 does not work for your architecture, please update the build script
to handle your target string and submit a pull request against the v0 branch.
We write the build script on an as-needed basis; it is not proactively filled
with all of the information listed in the compiler.

**NOTE**: The build script receives information through two environment
variables: `TARGET` and `CARGO_CFG_TARGET_ARCH`. The latter is equivalent to the
value in `cfg(target_arch)`; however, this value **does not** contain enough
information to fully disambiguate the target. The build script attempts to do
rudimentary parsing of the `env!(TARGET)` string; if this does not work for
your target, consider using the `TARGET_ARCH` matcher, or match on the full
`TARGET` string rather than the attempted parse.

----

## Project Origins

**@kneecaw** - <https://twitter.com/kneecaw/status/1132695060812849154>
> Feelin' lazy: Has someone already written a helper trait abstracting
> operations over `AtomicUsize` and `Cell<usize>` for generic code which may not
> care about atomicity?

**@ManishEarth** - <https://twitter.com/ManishEarth/status/1132706585300496384>
> no but call the crate radium
>
> (since people didn't care that it was radioactive and used it in everything)

<!-- Badges -->
[crate_link]: https://crates.io/crates/radium "Crates.io package"
[docs_img]: https://img.shields.io/docsrs/radium/latest.svg?style=for-the-badge "Radium documentation badge"
[docs_link]: https://docs.rs/radium "Radium documentation"
[downloads_img]: https://img.shields.io/crates/dv/radium.svg?style=for-the-badge "Crate downloads"
[msrv_img]: https://img.shields.io/badge/MSRV-1.60-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.60"
[version_img]: https://img.shields.io/crates/v/radium?color=f46623&style=for-the-badge "Radium version badge"

<!-- Documentation -->
[`Atom<T>`]: https://docs.rs/radium/latest/radium/types/struct.Atom.html
[`Cell`]: https://doc.rust-lang.org/core/cell/struct.Cell.html
[`Isotope<T>`]: https://docs.rs/radium/latest/radium/types/struct.Isotope.html
[`Radium`]: https://docs.rs/radium/latest/radium/trait.Radium.html
[`Radon<T>`]: https://docs.rs/radium/latest/radium/types/struct.Radon.html
[`atomic`]: https://doc.rust-lang.org/core/sync/atomic
[`portable-atomic`]: https://docs.rs/portable-atomic/1

<!-- External links -->
[0]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1600-2022-04-07
