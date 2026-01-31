Exposes the “[provenance]” part of the pointer for future use in
[`with_exposed_provenance`] and returns the “address” portion.

This is equivalent to [self as usize], which semantically discards provenance
information. Furthermore, this (like the `as` cast) has the implicit side-effect
of marking the provenance as ‘exposed’, so on platforms that support it you can
later call [`with_exposed_provenance`] to reconstitute the original pointer
including its provenance.

Due to its inherent ambiguity, [`with_exposed_provenance`] may not be supported
by tools that help you to stay conformant with the Rust memory model. It is
recommended to use [Strict Provenance][0] APIs such as [`with_addr`] wherever
possible, in which case [`addr`] should be used instead of `expose_provenance`.

On most platforms this will produce a value with the same bytes as the original
pointer, because all the bytes are dedicated to describing the address.
Platforms which need to store additional information in the pointer may not
support this operation, since the ‘expose’ side-effect which is required for
[`with_exposed_provenance`] to work is typically not available.

This is an [Exposed Provenance][1] API.

# Original

- [`<*T>::expose_provenance`][orig]
- [`NonNull::expose_provenance`]

# Similar Functions

- [`Pointer::expose_provenance`]
- [`NonNullPointer::expose_provenance`]

[0]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.expose_provenance
[provenance]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[`addr`]: Self::addr
[`with_addr`]: Self::with_addr
[`with_exposed_provenance`]: Self::with_exposed_provenance
