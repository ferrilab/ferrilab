# Pointer Management

Continuing the theme of Rust’s disjoint fundamental types, raw pointers and
references can be controlled generically by their referent type, but not by
their mutability permission.

This type unifies the `*const T` and `*mut T` raw pointers under one single name
and controls their memory access through the `P: Permission` type parameter.

It is a `repr(transparent)` wrapper over one of the raw pointers, and should
have the same machine behavior in all respects. Furthermore, the API is
restricted to only allow transitions between `Pointer<T, Shared>` and
`Pointer<T, Unique>` that are known to obey Rust’s pointer provenance rules (or
be forced through `unsafe` functions). This allows you to confidently use
`Pointer` in data structures rather than other types like `NonNull` or raw
pointers, which have variance and provenance concerns.

It mirrors the `*const T` and `*mut T` stable APIs, and delegates directly to
them. New APIs unique to `funty` exist to handle transitioning between element
and slice pointers, pointers and references, or changing the mutability
permission.
