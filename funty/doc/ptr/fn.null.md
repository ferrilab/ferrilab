Creates a null pointer.

This function is equivalent to zero-initializing the pointer:
`MaybeUninit::<*const T>::zeroed().assume_init()`. The resulting pointer has the
address 0.

# Original

- [`core::ptr::null`]
- [`core::ptr::null_mut`]

# Examples

```rust
use funty::ptr::{self, *};

let p: Pointer<i32, Shared> = ptr::null();
assert!(p.is_null());
assert_eq!(p.addr(), 0); // this pointer has the address 0
```
