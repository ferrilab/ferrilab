# Pointers and References

As with the other primitive data types, the Rust language does not unify raw
pointers or references in any way, and insists that users treat `*const T`,
`*mut T`, `*const T: !Sized`, and `*mut T: !Sized` as essentially four
completely independent types that happen to look similar.

This has worked so far, but ongoing efforts with pointer provenance and
mutability tracking make improper choice of pointer storage increasingly
hazardous to well-formed programs. Furthermore, advances in the Rust type
system, including the recently-stabilized Generic Associated Types, allow
increasingly expressive type declarations that can nevertheless result in code
generation that uses the ordinary primitives.

This module implements indirect-access permission tracking in the type system,
and allows code to be generic over permissions. This means that you can write
against a [`Pointer<T, P>`](`Pointer`) and have access to behavior that is
common to all pointers (address inspection, loading from the pointed-to
location), or you can specify a `Pointer<T, Unique>` and gain access to behavior
that requires mutable or unique access permission.

## Usage

Each fundamental or `core::ptr` type has a corresponding `funty` type:

- `*const T` is `Pointer<T, Shared>`
- `*mut T` is `Pointer<T, Unique>`
- A `*const T` that was _derived_ from a `*mut T` is
  `Pointer<T, (Shared, Unique)>`. This is elaborated in more detail in the
  [`Permission`] documentation.
- `NonNull<T>` is [`NonNullPointer<T, impl Permission>`][`NonNullPointer`]. The
  Rust `core::ptr::NonNull` is a wrapper around `*mut T`, and cannot safely
  express pointers which are not null but also are not exclusive/writable. You
  can instantiate this with `Shared` or `Unique` permissions as needed.

In application code, you should generally use fully-qualified `Pointer` and
`NonNullPointer` types. In library code, you _may_ choose to become generic over
`P: Permission`. Once generic over `Permission`, a `Pointer<T, P>` always has
access to the `*const`/`Shared` APIs, but loses access to the `*mut`/`Unique`
APIs. Library code can manipulate pointer permissions by using the `.make_perm`
family of methods:

- [`Pointer::make_shared`] changes the permission from `P` to `(Shared, P)` (see
  `Permission` docs)
- [`Pointer::make_unshared`] changes the permission from `(Shared, P)` to `P`
- [`Pointer::make_const`] makes the permission unconditionally `Shared`
- [`Pointer::make_mut`] produces an `Option<Pointer<T, Unique>>`. It will
  fail if the base permission of a pointer is `Shared`, but succeed if the base
  permission is `Unique`. When you write a function which is generic over
  `Permission`, then calls to `make_unique` will monomorphize out to one of the
  two branches of the `Option`, and the other will be deleted.

## API Surface

As with the rest of `funty`, the `Pointer` type strives to match the stable API
surface of the pointer primitives. However, the pointers have a great deal of
still-unstable methods that make them actually useful, including all of the
operations on slice pointers and direct address manipulation.

Since the Ferrilab project is committed to using the stable series, these APIs
will not be provided until they stabilize. If you urgently want them available
for your project under a `cfg(feature = "nightly")` gate, please file an issue.
