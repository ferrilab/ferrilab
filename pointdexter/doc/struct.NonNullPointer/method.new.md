Creates a new `NonNullPointer` if `ptr` is in fact not null.

# Original

[`NonNull::new`]

# API Differences

This returns a `Result`, not an `Option`, where the `Err` variant has a useful
default message.

# Panics During `const` Evaluation

This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See [`is_null`] for more information.

# Examples

```rust
use pointdexter::prelude::*;

let mut x = 0u32;
let ptr = NonNullPointer::new((&mut x).into()).unwrap();

let Err(_) = NonNullPointer::<u32, Unique>::new(ptxr::null()) else {
  unreachable!()
};
```

[`is_null`]: Pointer::is_null
