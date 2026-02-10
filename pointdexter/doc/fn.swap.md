Swaps the values at two mutable locations of the same type, without
deinitializing either.

But for the following exceptions, this function is semantically equivalent to
[`mem::swap`]:

- It operates on raw pointers instead of references. When references are
  available, [`mem::swap`] should be preferred.
- The two pointed-to values may overlap. If the values do overlap, then the
  overlapping region of memory from `x` will be used. This is demonstrated in
  the second example below.
- The operation is “untyped” in the sense that data may be uninitialized or
  otherwise violate the requirements of `T`. The initialization state is
  preserved exactly.

# Original

- [`core::ptr::swap`]
- [`<*T>::swap`][ptr_swap]
- [`NonNull::swap`]

# Similar Functions

- [`crate::swap`]
- [`Pointer::swap`]
- [`NonNullPointer::swap`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- Both `x` and `y` must be [valid] for both reads and writes. They must remain
  valid even when the other pointer is written. (This means if the memory ranges
  overlap, the two pointers must not be subject to aliasing restrictions
  relative to each other.)
- Both `x` and `y` must be properly aligned.

Note that even if `T` has size `0`, the pointers must be properly aligned.

# Examples

Swapping two non-overlapping regions:

```rust
use ptxr::*;

let mut array = [0, 1, 2, 3];

let (x, y) = array.split_at_mut(2);
let x = Pointer::from_raw_mut(x).cast::<[u32; 2]>(); // this is `array[0..2]`
let y = Pointer::from_raw_mut(y).cast::<[u32; 2]>(); // this is `array[2..4]`

unsafe {
  x.swap(y);
  assert_eq!([2, 3, 0, 1], array);
}
```

Swapping two overlapping regions:

```rust
use ptxr::*;

let mut array: [i32; 4] = [0, 1, 2, 3];

let array_ptr = Pointer::from_raw_mut(array.as_mut_ptr());

// this is `array[0..3]`
let x = array_ptr.cast::<[i32; 3]>();
// this is `array[1..4]`
let y = unsafe { array_ptr.add(1) }.cast::<[i32; 3]>();

unsafe {
  x.swap(y);
  // The indices `1..3` of the slice overlap between `x` and `y`.
  // Reasonable results would be for to them be `[2, 3]`, so that indices `0..3` are
  // `[1, 2, 3]` (matching `y` before the `swap`); or for them to be `[0, 1]`
  // so that indices `1..4` are `[0, 1, 2]` (matching `x` before the `swap`).
  // This implementation is defined to make the latter choice.
  assert_eq!([1, 0, 1, 2], array);
}
```

[ptr_swap]: https://doc.rust-lang.org/std/primitive.pointer.html#method.swap
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`mem::swap`]: core::mem::swap
