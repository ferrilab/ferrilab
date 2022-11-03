# Unified Shared-Mutable API

The `Radium` type is a common interface for shared-mutable types. It is only
implemented by `AtomicT` and `Cell<T>` variants of Rust primitives, as well as
the `radium` [alternative types][types]. It mirrors the API of the Rust [atomic]
types.

You should consult the Rust standard library documentation for the correct usage
of all methods. We do not attempt to keep our documentation up to date with the
Rust standard library.

Some of the `Radium` methods are gated on marker types in order to prevent their
use when the underlying primitive does not support them. For instance, pointers
do not (at time of writing) support atomic bit-wise or numeric operations, and
so cannot be used with any of the `.fetch_modify()` methods. Attempting to call
these methods will cause a compiler error when the `Self::Item` type is
unsuitable.

## Usage

You should use this trait as a type parameter in your API when you want to
accept *something* that supports shared-mutability, but you don’t need to care
about what it is. You will likely want to specify the `Item` to be a known type,
by writing this bound: `<R: Radium<Item = T>>` where `T` is another generic
parameter or a named primitive.

The `radium` project does *not* provide any unified trait system for the Rust
primitives! If you want to accept `Radium::Item` as a generic parameter, you
will need to use another crate (for instance, [`funty`]) to describe behavior
over generic primitives.

## Non-Usage

If you do not wish to expose caller-specified shared-mutability in your API, you
should instead use the [`radium::types`][types] module. The types in that module
all implement `Radium` as their only behavior, but may be easier to use when you
are describing particular data types.

[atomic]: core::sync::atomic
[types]: crate::types
[`funty`]: //crates.io/crates/funty
