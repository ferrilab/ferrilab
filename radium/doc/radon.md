# Non-Atomic Primitives

This type takes a Rust primitive (`bool`, integer, or pointer) as a type
parameter and wraps it in a `Cell`. Like `Atom<T>` and `Isotope<T>`, it
implements *only* the [`Radium`] API, and as such is suitable for cases where a
crate wants to turn off atomic usage entirely, while guaranteeing that swapping
out types will not cause a compilation failure.

Like `Atom` and `Isotope`, this type also implements `Debug`, `Default`, and
`From<T>`.

## Examples

Consider a crate with an `"atomic"` feature. It might decide to *attempt* atomic
behavior when this flag is on, and unconditionally deny it when the flag is off:

```rust
#[cfg(feature = "atomic")]
pub type MyAtom<T> = radium::types::Isotope<T>;

#[cfg(not(feature = "atomic"))]
pub type MyAtom<T> = radium::types::Radon<T>;
```

## Behind the Name

Radium decays into radon, and the `Radon` type is a “decayed” `Radium`
implementor. Radon gas is also poisonous, and `Radon` poisons your codebase
against multithreading.

[`Radium`]: crate::Radium
