Subtracts an unsigned offset from a pointer.

This can only move the pointer backward (or not move it). If you need to move
forward or backward depending on the value, then you might want [`offset`]
instead which takes a signed offset.

`count` is in units of `T`; e.g., a count of 3 represents a pointer offset of
`3 * size_of::<T>()` bytes.

# Original

- [`<*T>::sub`][orig]
- [`NonNull::sub`]

# Similar Functions

- [`Pointer::sub`]
- [`NonNullPointer::sub`]

# Safety

If any of the following conditions are violated, the result is Undefined
Behavior:

- The offset in bytes, `count * size_of::<T>()`, computed on mathematical
  integers (without “wrapping around”), must fit in an `isize`.
- If the computed offset is non-zero, then `self` must be [derived from][0] a
  pointer to some [allocation][1], and the entire memory range between `self`
  and the result must be in bounds of that allocation. In particular, this range
  must not “wrap around” the edge of the address space.

Allocations can never be larger than `isize::MAX` bytes, so if the computed
offset stays in bounds of the allocation, it is guaranteed to satisfy the first
requirement. This implies, for instance, that `vec.as_ptr().add(vec.len())` (for
vec: `Vec<T>`) is always safe.

Consider using [`wrapping_sub`] instead if these constraints are difficult to
satisfy. The only advantage of this method is that it enables more aggressive
compiler optimizations.

# Examples

```rust
use pointdexter::prelude::*;
let s: &str = "123";

unsafe {
  let end: Pointer<u8, Shared> = s.as_ptr().add(3).into();
  assert_eq!(end.sub(1).read(), b'3');
  assert_eq!(end.sub(2).read(), b'2');
}
```

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#allocation
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.sub
