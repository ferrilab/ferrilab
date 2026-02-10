Casts to a pointer of another type

# Original

- [`<*T>::cast`][orig]
- [`NonNull::cast`]

# Similar Functions

- [`Pointer::cast`]
- [`NonNullPointer::cast`]

# Notes

The destination type must be `Sized`. Metadata (slice length, trait vtable,
etc.) cannot be conjured.

# Examples

```rust
use ptxr::*;

let mut x = 0u32;
let ptr = Pointer::from(&mut x);
let nnp = NonNullPointer::new(ptr).unwrap();

let cast_ptr = ptr.cast::<i8>();
let cast_nnp = nnp.cast::<u8>();
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast
