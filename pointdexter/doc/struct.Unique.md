# Unique Provenance

This type denotes a [provenance][0] which begins with only one accessor, which
has total write capability. This accessor can then be degraded by casting to
`Shared` (irrevocable) or `(Shared, Unique)` (revocable).

While this is the top-most permission in a Pointdexter type, that type keeps
access to its unconditional-write APIs. It will lose access once this is no
longer the top-most permission.

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
