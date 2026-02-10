Compares the _addresses_ of two pointers for equality, ignoring any metadata in
fat pointers.

If the arguments are thin pointers of the same type, then this is the same as
[`eq`].

# Original

[`core::ptr::addr_eq`]

# Examples

```rust
use core::fmt::Debug;
use pointdexter::prelude::*;

let whole: &[i32; 3] = &[1, 2, 3];
let first: &i32 = &whole[0];

assert!(ptxr::addr_eq(whole.into(), first.into()));

let whole: &dyn Debug = whole;
let first: &dyn Debug = first;
assert!(!ptxr::eq(whole.into(), first.into()));
```

[`eq`]: crate::eq
