# Unified Shared-Mutable API

The `Radium` trait is a common interface for shared-mutable types. We provide
implementations for the `AtomicT` and `Cell<T>` types in the standard library,
as well as our [alternative types][types]. It mirrors the API of the Rust
[atomic] types.

You should consult the Rust standard library documentation for the correct usage
of all methods. While we refer to, and draw on, the Rust project’s work, we do
not guarantee keeping our language up to date with it.

Some of the `Radium` methods are gated on marker types in order to prevent their
use when the underlying primitive does not support them. For instance, pointers
do not (at time of writing) support atomic bit-wise or numeric operations, and
so cannot be used with any of the `.fetch_modify()` methods. Attempting to call
these methods will cause a compiler error when the underlying primitive type is
unsuitable.

## Usage

You should use this trait as a type parameter in your API when you want to
accept *something* that supports shared-mutability, but you don’t need to care
about what it is. You will likely want to specify the `Item` to be a known type,
by writing this bound: `<R: Radium<Item = T>>` where `T` is another generic
parameter or a named primitive.

Radium does *not* provide any unified trait system for the Rust primitives! If
you want to accept `Radium::Item` as a generic parameter, you will need to use
another crate (for instance, [`funty`]) to describe behavior over generic
primitives.

## Non-Usage

If you do not wish to expose caller-specified shared-mutability in your API, you
should instead use the [`radium::types`][types] module. The types in that module
all implement `Radium` as their only behavior, but may be easier to use when you
are describing particular data types.

## Safety

This trait is marked as `unsafe` to implement, because it abstracts over types
which can be mutated through a shared reference. The implementor is required to
correctly synchronize writes that occur through `Radium` methods, and in
particular, to never violate the Rust language’s rules about the invalid
production of unique references.

## Implementation

While we do not enforce any restrictions on `Radium` implementors, only types
which are transparent wrappers over a Rust primitive should implement this.
Out-of-line guards such as `Mutex` can technically satisfy its API requirements,
but are not likely to be useful candidate types for these uses.

[atomic]: core::sync::atomic
[types]: crate::types
[`funty`]: //crates.io/crates/funty
