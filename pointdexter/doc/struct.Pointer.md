Unifying bridge over `*const T` and `*mut T`.

This pointer is generic over both the pointed-to type and also the mutability of
the pointed-to type. The [`Permission`] trait and the main crate docs explain
this in more detail, but a `Pointer<T, Unique>` is equivalent to a `*mut T`, but
a `Pointer<T, Shared>` is a _limited_ `*const T` in that it _will not_ convert
to `Pointer<T, Unique>`. Furthermore, `Unique`-permissioned pointers can degrade
temporarily to write-incapable by becoming `Pointer<T, (Shared, Unique)>`, and
_that_ permission can later be unwound to `Unique`.

`Pointer<T, impl Permission>` implements all APIs which are present on both
`*const T` and `*mut T` pointers, while APIs which are unique to `*mut T`
(anything that involves writing through the pointer) are _only_ present on
`Pointer<T, Unique>`.

If you want to encode a non-null value restriction into the type system, see
[`NonNullPointer`], which behaves exactly like `Pointer` but is, as the name
indicates, guaranteed not to be the zero address.

# Original

- [`*const T`][raw_ptr]
- [`*mut T`][raw_ptr]

# Permission Changes

Pointers can change their permission by using one of four casting APIs:

- [`cast_const`](Self::cast_const) unconditionally creates a
  `Pointer<T, Shared>`, which can never change to anything else.
- [`cast_mut`](Self::cast_mut) unconditionally panics. It is marked as
  deprecated, and provided only so that code porting over from standard-library
  pointers will continue to compile, and show the correction message.
- [`cast_shared`](Self::cast_shared) converts `Pointer<T, P>` to
  `Pointer<T, (Shared, P)>`, which acts like `Pointer<T, Shared>`.
- [`try_cast_unique`](Self::try_cast_unique) is a fallible method which will
  throw away all stacked permissions until it reaches the original `Shared` or
  `Unique`, then either produce `Pointer<T, Unique>` or an error value.

# Representation

`Pointer<T, Unique>` is a `repr(transparent)` wrapper over `*mut T`. All other
`Pointer`s are transparent wrappers over `*const T`. Hopefully, this makes it
easier for the compiler to understand how `Pointer`s are used and simplifes
provenance tracking.

# Ongoing Stabilization

This type has APIs which either enable, or become `const fn`, in the following
Rust releases:

- 1.87: `byte_offset_from_unsigned`, `offset_from_unsigned` are added
- 1.88: `replace` becomes `const fn`, `impl Default` added

These can be enabled with

```toml
[dependencies.ptxr]
features = ["rust_187", "rust_188"]
```

[raw_ptr]: https://doc.rust-lang.org/std/primitive.pointer.html
