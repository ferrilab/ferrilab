Creates a new `NonNullPointer` from any value.

# Original

[`NonNull::new_unchecked`]

# Safety

`ptr` must not be null.

# Examples

```rust
use pointdexter::prelude::*;

let mut x = 0u32;
let y = Pointer::from(&mut x);
let z = unsafe { NonNullPointer::new_unchecked(y) };
```

_Incorrect_ usage of this function:

```rust,should_panic
use pointdexter::prelude::*;

let ptr = unsafe { NonNullPointer::<u32, Unique>::new_unchecked(ptxr::null()) };
// NEVER DO THIS!!! This is undefined behavior. ⚠️
```
