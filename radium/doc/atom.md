# Atomic Primitives

This type takes a Rust primitive (`bool`, integer, or pointer) as a type
parameter and acts as the corresponding `AtomicT` type. Attempting to construct
an `Atom<T>` on a target that has no `AtomicT` will cause a compilation failure
stating that `T` does not implement the `Atomic` trait. The [`Isotope<T>`]
sibling type will never fail to compile, but in exchange does not guarantee
atomic behavior.

This type implements the [`Radium`] API, as well as the `Debug`, `Default`, and
`From<T>` traits that the standard library atomics implement. It has no other
API surface.

[`Isotope<T>`]: crate::types::Isotope
[`Radium`]: crate::Radium
