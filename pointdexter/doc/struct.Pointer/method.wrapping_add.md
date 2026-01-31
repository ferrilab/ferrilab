Adds an unsigned offset to a pointer using wrapping arithmetic.

`count` is in units of `T`; e.g., a count of 3 represents a pointer offset of
`3 * size_of::<T>()` bytes.

# Original

[`<*T>::wrapping_add`][orig]

# Safety

This operation itself is always safe, but using the resulting pointer is not.

The resulting pointer “remembers” the [allocation][0] that `self` points to; it
must not be used to read or write other allocations.

In other words, `let z = x.wrapping_add((y.addr()) - (x.addr()))` does not make
`z` the same as `y` even if we assume `T` has size `1` and there is no overflow:
`z` is still attached to the object `x` is attached to, and dereferencing it is
Undefined Behavior unless `x` and `y` point into the same allocation.

Compared to [`add`], this method basically delays the requirement of staying
within the same allocation: [`add`] is immediate Undefined Behavior when
crossing object boundaries; `wrapping_add` produces a pointer but still leads to
Undefined Behavior if a pointer is dereferenced when it is out-of-bounds of the
object it is attached to. [`add`] can be optimized better and is thus preferable
in performance-sensitive code.

The delayed check only considers the value of the pointer that was dereferenced,
not the intermediate values used during the computation of the final result. For
example, `x.wrapping_add(o).wrapping_sub(o)` is always the same as x. In other
words, leaving the allocation and then re-entering it later is permitted.

# Examples

```rust
use pointdexter::*;
use std::fmt::Write as _;

// Iterate using a raw pointer in increments of two elements
let data = [1u8, 2, 3, 4, 5];
let mut ptr: Pointer<u8, Shared> = data.as_ptr().into();
let step = 2;
let end_rounded_up = ptr.wrapping_add(6);

let mut out = String::new();
while ptr != end_rounded_up {
  unsafe {
    write!(&mut out, "{}, ", ptr.read()).unwrap();
  }
  ptr = ptr.wrapping_add(step);
}
assert_eq!(out, "1, 3, 5, ");
```

[0]: https://doc.rust-lang.org/core/ptr/index.html#allocation
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add
[`add`]: Self::add
