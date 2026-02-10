Returns the length of a raw slice.

The returned value is the number of **elements**, not the number of bytes.

This function is safe, even when the raw slice cannot be cast to a slice
reference because the pointer is null or unaligned.

# Original

- [`<*[T]>::len`][orig]
- [`NonNull::len`]

# Similar Functions

- [`Pointer::len`]
- [`NonNullPointer::len`]

# Examples

```rust
use pointdexter::prelude::*;

let slice = ptxr::null::<i8, Shared>().make_slice(3);
assert_eq!(slice.len(), 3);
let slice = NonNullPointer::<i8, Shared>::dangling().make_slice(3);
assert_eq!(slice.len(), 3);
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.len
