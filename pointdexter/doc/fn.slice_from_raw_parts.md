Forms a raw slice from a pointer and a length.

The `len` argument is the number of **elements**, not the number of bytes.

This function is safe, but actually using the return value is unsafe. See the
documentation of [`slice::from_raw_parts`][0] for slice safety requirements.

This exists both as a constructor on the pointers, or as a transform from a
pointer-to-scalar to pointer-to-slice called `.make_slice()`.

# Original

- [`core::ptr::slice_from_raw_parts`]
- [`core::ptr::slice_from_raw_parts_mut`]
- [`NonNull::slice_from_raw_parts]

# Similar Functions

- [`crate::slice_from_raw_parts`]
- [`Pointer::make_slice`]
- [`NonNullPointer::slice_from_raw_parts`]
- [`NonNullPointer::make_slice`]

# Examples

```rust
use pointdexter::prelude::*;

// create a slice pointer when starting out with a pointer to the first element
let x = [5, 6, 7];
let raw = Pointer::from_raw_const(x.as_ptr());
let slice = ptxr::slice_from_raw_parts(raw, 3);
assert_eq!(unsafe { &*slice.into_raw_const() }[2], 7);
let nnp = NonNullPointer::new(raw).unwrap();
let slice = nnp.make_slice(3);
```

You must ensure that the pointer is valid and not null before dereferencing the
raw slice. A slice reference must never have a null pointer, even if it’s empty.

```rust,should_panic
use pointdexter::prelude::*;

let danger = ptxr::null::<u8, Shared>().make_slice(0);
unsafe {
    danger.as_ref().expect("references must not be null");
}
```

[0]: core::slice::from_raw_parts
