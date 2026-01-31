Creates a new pointer with the given address and the [provenance][0] of `self`.

This is similar to a `addr as *const T` cast, but copies the _provenance_ of
`self` to the new pointer. This avoids the inherent ambiguity of the unary cast.

This is equivalent to using [`wrapping_offset`] to offset `self` to the given
address, and therefore has all the same capabilities and restrictions.

This is a [Strict Provenance][1] API.

# Original

- [`<*T>::with_addr`][orig]
- [`NonNull::with_addr`]

# Similar Functions

- [`Pointer::with_addr`]
- [`NonNullPointer::with_addr`]

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.with_addr
