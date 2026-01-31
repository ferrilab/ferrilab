Trait-level pointer access permissions.

This trait moves the shared/exclusive access system from unrelated language
primitives (`*const T`/`*mut T`, `&T`/`&mut T`) into the trait system. It is
implemented by two token types: [`Shared`] and [`Unique`]. These correspond to
`*const` and `*mut`, respectively.

When used in [`Pointer`], [`NonNullPointer`], or [`Reference`] types, these
markers track access controls and original provenance through the trait system,
including the capability to conditionally degrade a `Unique` accessor down to
`Shared` and back up without violating any of Rust’s requirements about region
provenance.

Like its implementor structs, this is a marker-only trait with no public API of
its own.
