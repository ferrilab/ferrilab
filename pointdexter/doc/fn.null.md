Creates a null pointer.

This function is equivalent to zero-initializing the pointer:
`MaybeUninit::<*const T>::zeroed().assume_init()`. The resulting pointer has the
address 0.

# Original

- [`core::ptr::null`]
- [`core::ptr::null_mut`]

# Similar Functions

- [`crate::null`]
- [`Pointer::null`]

# Examples

```rust
use pointdexter::prelude::*;

let p: Pointer<i32, Shared> = ptxr::null();
assert!(p.is_null());
assert_eq!(p.addr(), 0); // this pointer has the address 0
```
