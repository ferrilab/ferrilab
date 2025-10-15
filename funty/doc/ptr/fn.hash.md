Hash a pointer.

This can be used to hash a `&T` reference by its address rather than the value
to which it refers (which is what the `Hash for &T` implementation does).

# Original

[`core::ptr::hash`]

# Examples

```rust
use funty::ptr::{self, *};
use std::hash::{DefaultHasher, Hash, Hasher};

let five = 5;
let five_ref = &five;

let mut hasher = DefaultHasher::new();
ptr::hash(five_ref.wrap_funty(), &mut hasher);
let actual = hasher.finish();

let mut hasher = DefaultHasher::new();
(five_ref.wrap_funty()).hash(&mut hasher);
let expected = hasher.finish();

assert_eq!(actual, expected);
```
