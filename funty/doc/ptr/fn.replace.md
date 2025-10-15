Moves `src` into the pointed `dst`, returning the previous `dst` value.

Neither value is dropped.

This function is semantically equivalent to [`mem::replace`] except that it
operates on raw pointers instead of references. When references are available,
[`mem::replace`] should be preferred.

# Original

- [`core::ptr::replace`]
- [`<*T>::replace`][0]

# Safety

Behavior is undefined if any of the following conditions are violated:

- `dst` must be [valid][1] for both reads and writes.
- `dst` must be properly aligned.
- `dst` must point to a properly initialized value of type `T`.

Note that even if `T` has size `0`, the pointer must be properly aligned.

# Examples

```rust
use funty::ptr::*;

let mut rust = vec!['b', 'u', 's', 't'];

// `mem::replace` would have the same effect without requiring the unsafe
// block.
let b = unsafe {
  Pointer::from_mut(&mut rust[0]).replace('r')
};

assert_eq!(b, 'b');
assert_eq!(rust, &['r', 'u', 's', 't']);
```

[`mem::replace`]: core::mem::replace
[0]: https://doc.rust-lang.org/std/primitive.pointer.html#method.replace
[1]: https://doc.rust-lang.org/std/ptr/index.html#safety
