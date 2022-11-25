# Permission Changes

`funty` uses the `Permission` trait to create a graph of safe transitions
between raw pointers and `Pointer`, and between different type parameters
attached to `Pointer` values.

`Pointer`s can be constructed from both raw pointers: `*const T` produces
`Pointer<T, Shared>`, and `*mut T` produces `Pointer<T, Unique>`. Once
constructed, all `Pointer<T, P: Permission>` values have access to the
introspective and read-only memory accesses defined on the raw pointers. The
memory-write APIs are only available on `Pointer<T, Unique>`.

Additionally, the method `.cast_shared()` moves `Pointer`s from `P` to
`(Shared, P)`. The `(Shared, P: Permission)` tuple is itself an implementor of
`Permission`, and can continue to be used as a read-only pointer.
`Pointer<T, (Shared, P)>` also provides `.cast_unshared()`, which undoes
`.cast_shared()` and transitions from `(Shared, P)` back to `P`.

All `Pointer`s can produce `*const` raw pointers, but only the `Unique`
permission can produce `*mut` raw pointers. If you need access to `*mut`
raw pointers but are in generic code where you cannot satisfactorily prove to
the compiler that you have a `Unique`, you have two options. The
`.unwind_to_unique()` method recursively unwinds a `(Shared, P)` history stack
until it reaches the base, then produces `Some` pointer if the original
permission was `Unique` or `None` if it was `Shared`. The `unsafe` method
`.cast_mut()` unconditionally produces a pointer with `Unique` permissions, but
may violate Rust’s provenance rules and invoke undefined behavior.
