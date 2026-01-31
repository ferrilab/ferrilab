Gets the “address” portion of the pointer.

This is similar to `self as usize`, except that the [provenance] of the
pointer is discarded and not [exposed]. This means that casting the returned
address back to a pointer yields a [pointer without provenance][0], which is
undefined behavior to dereference. To properly restore the lost information and
obtain a dereferenceable pointer, use [`with_addr`] or [`map_addr`].

If using those APIs is not possible because there is no way to preserve a
pointer with the required provenance, then Strict Provenance might not be for
you. Use pointer-integer casts or expose_provenance and with_exposed_provenance
instead. However, note that this makes your code less portable and less amenable
to tools that check for compliance with the Rust memory model.

On most platforms this will produce a value with the same bytes as the original
pointer, because all the bytes are dedicated to describing the address.
Platforms which need to store additional information in the pointer may perform
a change of representation to produce a value containing only the address
portion of the pointer. What that means is up to the platform to define.

This is a [Strict Provenance][1] API.

# Original

- [`<*T>::addr`][orig]
- [`NonNull::addr`]

# Similar Functions

- [`Pointer::addr`]
- [`NonNullPointer::addr`]

[0]: crate::without_provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.addr
[exposed]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
[provenance]: https://doc.rust-lang.org/core/ptr/index.html#provenance
