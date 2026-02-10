Creates a new pointer that is dangling, but non-null and well-aligned.

This is useful for initializing types which lazily allocate, like `Vec::new`
does.

Note that the pointer value may potentially represent a valid pointer to a `T`,
which means this must not be used as a “not yet initialized” sentinel value.
Types that lazily allocate must track initialization by some other means.

# Original

- [`core::ptr::dangling`]
- [`core::ptr::dangling_mut`]
- [`NonNull::dangling`]

# Similar Functions

- [`crate::dangling`]
- [`Pointer::dangling`]
- [`NonNullPointer::dangling`]

# Examples

```rust
use pointdexter::prelude::*;

let a: Pointer<i32, Unique> = ptxr::dangling();
let b = Pointer::<u64, Shared>::dangling();
let c = NonNullPointer::<i16, Unique>::dangling();
```

Remember, none of these can be dereferenced!
