Computes the offset that needs to be applied to the pointer in order to make it
aligned to `align`.

If it is not possible to align the pointer, the implementation returns
`usize::MAX`.

The offset is expressed in number of `T` elements, and not bytes. The value
returned can be used with the [`wrapping_add`] method.

There are no guarantees whatsoever that offsetting the pointer will not overflow
or go beyond the allocation that the pointer points into. It is up to the caller
to ensure that the returned offset is correct in all terms other than alignment.

# Original

- [`<*T>::align_offset][orig]
- [`NonNull::align_offset`]

# Similar Functions

- [`Pointer::align_offset`]
- [`NonNullPointer::align_offset`]

# Panics

The function panics if align is not a power-of-two.

# Examples

Accessing adjacent `u8` as `u16`:

```rust
use pointdexter::*;

let x = [5_u8, 6, 7, 8, 9];
let ptr = Pointer::from_raw_const(x.as_ptr());
let offset = ptr.align_offset(align_of::<u16>());

if offset < x.len() - 1 {
  unsafe {
    let u16_ptr = ptr.add(offset).cast::<u16>();
    assert!(u16_ptr.read() == u16::from_ne_bytes([5, 6])
         || u16_ptr.read() == u16::from_ne_bytes([6, 7]));
  }
} else {
  // while the pointer can be aligned via `offset`, it would point
  // outside the allocation
}
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.align_offset
