Converts an address back to a pointer, picking up some previously ‘exposed’
[provenance].

This is fully equivalent to `addr as *const T`. The provenance of the returned
pointer is that of some pointer that was previously exposed by passing it to
[`expose_provenance`], or a `ptr as usize` cast. In addition, memory which is
outside the control of the Rust abstract machine (MMIO registers, for example)
is always considered to be accessible with an exposed provenance, so long as
this memory is disjoint from memory that will be used by the abstract machine
such as the stack, heap, and statics.

The exact provenance that gets picked is not specified. The compiler will do its
best to pick the “right” provenance for you (whatever that may be), but
currently we cannot provide any guarantees about which provenance the resulting
pointer will have – and therefore there is no definite specification for which
memory the resulting pointer may access.

If there is no previously ‘exposed’ provenance that justifies the way the
returned pointer will be used, the program has undefined behavior. In
particular, the aliasing rules still apply: pointers and references that have
been invalidated due to aliasing accesses cannot be used anymore, even if they
have been exposed!

Due to its inherent ambiguity, this operation may not be supported by tools that
help you to stay conformant with the Rust memory model. It is recommended to use
[Strict Provenance][strict] APIs such as [`with_addr`] wherever possible.

On most platforms this will produce a value with the same bytes as the address.
Platforms which need to store additional information in a pointer may not
support this operation, since it is generally not possible to actually compute
which provenance the returned pointer has to pick up.

This is an [Exposed Provenance][exposed] API.

# Original

- [`core::ptr::with_exposed_provenance`]
- [`core::ptr::with_exposed_provenance_mut`]
- [`NonNull::with_exposed_provenance`]

# Similar Functions

- [`crate::with_exposed_provenance`]
- [`Pointer::with_exposed_provenance`]
- [`NonNullPointer::with_exposed_provenance`]

[exposed]: https://doc.rust-lang.org/core/ptr/index.html#exposed-provenance
[provenance]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[strict]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
