Forms a raw slice from a pointer and a length.

The `len` argument is the number of **elements**, not the number of bytes.

This function is safe, but actually using the return value is unsafe. See the
documentation of [`slice::from_raw_parts`][0] for slice safety requirements.

# Original

- [`core::ptr::slice_from_raw_parts`]
- [`core::ptr::slice_from_raw_parts_mut`]

# Examples

```rust
use funty::ptr::*;

// create a slice pointer when starting out with a pointer to the first element
let x = [5, 6, 7];
let raw = Pointer::from_const(x.as_ptr());
let slice = raw.make_slice(3);
assert_eq!(unsafe { &*slice.into_raw() }[2], 7);
```

You must ensure that the pointer is valid and not null before dereferencing the
raw slice. A slice reference must never have a null pointer, even if it’s empty.

```rust,should_panic
use funty::ptr;

let danger = ptr::null::<u8>().make_slice(0);
unsafe {
    danger.as_ref().expect("references must not be null");
}
```

[0]: core::slice::from_raw_parts
