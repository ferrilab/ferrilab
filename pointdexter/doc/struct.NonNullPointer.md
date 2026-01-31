Like [`Pointer`], but guaranteed to be non-zero.

Unlike the standard-library type, it can be constructed from a `*const T` and
will remember its originating [`Permission`]. It has the same API, including
permission-casting, as `Pointer` does.

# Original

[`core::ptr::NonNull`]

# Representation

`NonNullPointer` is a `repr(transparent)` wrapper around a standard-library
`NonNull`, and inherits layout and ABI properties from it, including the
`Option::<NonNullPointer<T, P>>::None` niche representation (`None` is 0, `Some`
is anything else).

# Ongoing Stabilization

This type has APIs which either enable, or become `const fn`, in the following
Rust releases:

- 1.87: `byte_offset_from_unsigned`, `offset_from_unsigned`
- 1.88: `replace` becomes `const fn`
- 1.89:
  - `from_ref`, `from_mut`, `expose_provenance` are added
  - `with_exposed_provenance` becomes `const fn`

These can be enabled with

```toml
[dependencies.pointdexter]
features = ["rust_187", "rust_188", "rust_189"]
```
