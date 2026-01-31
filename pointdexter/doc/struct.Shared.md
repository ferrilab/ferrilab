# Shared Provenance

This type denotes a [provenance][0] which permits many accessors to interact
with it at the same time. These accessors are _usually_, though _not always_,
read-only. Types derived from [`UnsafeCell`][1] can write through shared
provenances.

While this is the top-most permission in a Pointdexter type, that type loses
access to its unconditional-write APIs, but is still able to perform shared
accesses and convert to `*const T`.

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[1]: core::cell::UnsafeCell
