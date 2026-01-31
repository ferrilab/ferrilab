Creates a new pointer by mapping `self`’s address to a new one, preserving the
[provenance][0] of `self`.

This is a convenience for [`with_addr`]; see that method for details.

This is a [Strict Provenance][1] API.

# Original

- [`<*T>::map_addr`][orig]
- [`NonNull::map_addr]

# Similar Functions

- [`Pointer::map_addr`]
- [`NonNullPointer::map_addr`]

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.map_addr
