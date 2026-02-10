Returns whether the pointer is properly aligned for `T`.

# Original

- [`<*T>::is_aligned][orig]
- [`NonNull::is_aligned]

# Similar Functions

- [`Pointer::is_aligned`]
- [`NonNullPointer::is_aligned`]

# Examples

```rust
use ptxr::*;

// On some platforms, the alignment of i32 is less than 4.
#[repr(align(4))]
struct AlignedI32(i32);

let data = AlignedI32(42);
let ptr: Pointer<AlignedI32, Shared> = (&data).into();

assert!(ptr.is_aligned());
assert!(!ptr.wrapping_byte_add(1).is_aligned());
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.is_aligned
