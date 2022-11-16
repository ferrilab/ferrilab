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
against a `Pointer<T, P>` and have access to behavior that is common to all
pointers (address inspection, loading from the pointed-to location), or you can
specify a `Pointer<T, Unique>` and gain access to behavior that requires mutable
or unique access permission.

## Usage

Your code should be generic over the `Permission` trait when you primarily
expect to read from memory or to hold pointers and pass them through to known
use sites. You can also explicitly demand pointers that specify a `Shared` or
`Unique` permission type parameter in order to make use of their specific
capabilities.

The `Pointer<T, Unique>` type can be temporarily downgraded to
`Pointer<T, (Shared, Unique)>`. This type acts identically to
`Pointer<T, Shared>`, except that it has memory of being derived from a `Unique`
permission and can be *safely* restored to that.

`Pointer<T, Shared>` can be *unsafely* upgraded to `Pointer<T, Unique>`, but
this operation requires you to have upheld Rust’s provenance guarantees about
write permissions to the pointed-to memory.

The `Permission` trait is intended for use only in type relation graphs. It
contains runtime functionality that cannot be expressed anywhere else in the
module, but these functions are not part of the public API and are hidden from
the documentation.

## Associated Types

The `Permission::Ptr<T: ?Sized>` type is either a `*const T` or a `*mut T` raw
pointer. This is the only value stored inside a `Pointer<T, P>`, and the
`Permission` implementation allows `Pointer` to store the correct value type and
satisfy Rust’s expectations about pointer mutability provenance.

Similarly, the `Permission::Ref<T: ?Sized>` type is either a `&T` or an
`&mut T`.

The `Pointer` APIs work primarily with these associated types rather than the
concrete primitives. Some functions operate on raw pointers directly so that you
can have direct access to them rather than relying on the `Pointer` type or
`RawPtr` trait.

## API Surface

As with the rest of `funty`, the `Pointer` type strives to match the stable API
surface of the pointer primitives. However, the pointers have a great deal of
still-unstable methods that make them actually useful, including all of the
operations on slice pointers and direct address manipulation.

Since the `bitvecto-rs` project is committed to using the stable series, these
APIs will not be provided until they stabilize. If you urgently want them
available for your project under a `cfg(feature = "nightly")` gate, please file
an issue.
