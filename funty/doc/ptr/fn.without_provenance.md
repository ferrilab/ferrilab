Creates a pointer with the given address and no [provenance].

This is equivalent to `Pointer::null().with_addr(addr)`.

Without provenance, this pointer is not associated with any actual allocation. Such a no-provenance pointer may be used for zero-sized memory accesses (if suitably aligned), but non-zero-sized memory accesses with a no-provenance pointer are UB. No-provenance pointers are little more than a `usize` address in disguise.

This is different from `addr as *const T`, which creates a pointer that picks up a previously exposed provenance. See [`with_exposed_provenance`] for more details on that operation.

This is a [Strict Provenance][strict] API.

[provenance]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[strict]: https://doc.rust-lang.org/core/ptr/index.html#strict-provenance
[`with_exposed_provenance`]: crate::ptr::with_exposed_provenance
