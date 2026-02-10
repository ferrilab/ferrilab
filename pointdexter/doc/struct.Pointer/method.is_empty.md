Tests if the raw slice has a length of 0.

# Original

- [`<*[T]>::is_empty`][orig]
- [`NonNull::is_empty`]

# Similar Functions

- [`Pointer::is_empty`]
- [`NonNullPointer::is_empty`]

# Examples

```rust
use ptxr::*;

let slice = Pointer::<u8, Shared>::dangling().make_slice(3);
assert!(!slice.is_empty());
let slice = NonNullPointer::new(slice).unwrap();
assert!(!slice.is_empty());
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.is_empty
