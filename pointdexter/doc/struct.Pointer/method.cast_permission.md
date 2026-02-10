Casts from one [`Permission`] type to another.

The `Permission` type system allows Pointers to move between `Shared` and
`Unique` according to the rules of the Rust [provenance][0] system. These rules
are that a `Unique` pointer may become a `Shared` pointer without restriction,
but a `Shared` pointer may only become `Unique` if it has a provenance which
was originally `Unique`.

Pointdexter manages this by storing provenance information in the Rust compiler
session: once a `Pointer<_, Unique>` exists, rather than casting directly to
`Shared`, it instead casts to `(Shared, Unique)`. This tuple _also_ implements
`Permission`, as a `Shared`. However, it remembers that its base permission is
`Unique`, not `Shared`. As such, it can successfully cast back to `Unique`,
while a plain `Shared` permission cannot do this.

# Original

- [`<*const T>::cast_mut`][orig_m]
- [`<*mut T>::cast_const`][orig_c]

# API Differences

The `.try_cast_unique()` method returns `Result`, and `.cast_mut()`
unconditionally panics. `Pointer<_, Unique>` and `Pointer<_, (Shared, Unique)>`
both successfully cast, while `Pointer<_, Shared>` produces an error. The error
type exists only to give a useful error message on `.unwrap()`.

# Usage

Casting between permissions is most useful in a context which is generic over
`P: Permission`; however, it can also be used in e.g. scoped parallelism.

# Examples

Implementing a destructor for a maybe-writeable cache key:

```rust
use ptxr::*;

struct CacheKey<P: Permission> {
  local: u32,
  remote: Pointer<u32, P>,
}

impl<P: Permission> Drop for CacheKey<P> {
  fn drop(&mut self) {
    if let Ok(rmt) = self.remote.try_cast_unique() {
      unsafe { rmt.write(self.local); }
    }
  }
}
```

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[orig_c]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast_const
[orig_m]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast_mut
