Hash a pointer.

This can be used to hash a `&T` reference by its address rather than the value
to which it refers (which is what the `Hash for &T` implementation does).

# Original

[`core::ptr::hash`]

# Examples

```rust
use pointdexter::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

let five = 5;
let five_ref = &five;

let mut hasher = DefaultHasher::new();
ptxr::hash(five_ref.into(), &mut hasher);
let actual = hasher.finish();

let mut hasher = DefaultHasher::new();
Pointer::from(five_ref).hash(&mut hasher);
let expected = hasher.finish();

assert_eq!(actual, expected);
```
