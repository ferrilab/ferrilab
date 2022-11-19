# Best-Effort Atomic Primitives

This type takes a Rust primitive (`bool`, integer, or pointer) as a type
parameter and acts as the corresponding `RadiumT` type. Unlike the [`Atom<T>`]
sibling type, this type will never produce a compiler error when instantiated
(with a valid primitive). When the requisite atomic support is missing on the
target architecture, it falls back to acting as a `Cell<T>`.

This type implements the [`Radium`] API, as well as the `Debug`, `Default`, and
`From<T>` traits that the standard library atomics implement. It has no other
API surface, and in particular *does not* attempt to follow the `Cell` API.

See also [`Radon<T>`], which is always strictly non-atomic.

[`Atom<T>`]: crate::types::Atom
[`Radium`]: crate::Radium
[`Radon<T>`]: crate::types::Radon
