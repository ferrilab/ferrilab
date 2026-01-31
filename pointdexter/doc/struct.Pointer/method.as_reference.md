Tries to produce a [`Reference`] to the pointed-to value.

Returns `Err` if the pointer is null, or else returns a Reference to the value
wrapped in `Ok`.

# Original

- [`<*T>::as_ref`][orig_c]
- [`<*mut T>::as_mut`][orig_m]
- [`NonNull::as_ref`]
- [`NonNull::as_mut`]

# Similar Functions

- [`Pointer::as_ref`]
- [`Pointer::as_mut`]
- [`Pointer::as_reference`]
- [`NonNullPointer::as_ref`]
- [`NonNullPointer::as_mut`]
- [`NonNullPointer::as_reference`]

# API Differences

Returns a `Result`, rather than `Option`, with useful error messages on the
error value.

# Safety

When calling this method, you have to ensure that _either_ the pointer is null
_or_ the pointer is [convertible to a reference][0].

# Panics During `const` Evaluation

This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See [`is_null`][1] for more information.

# Examples

```rust
use pointdexter::*;

let ptr: Pointer<u8, Shared> = (&10u8).into();

unsafe {
  if let Ok(val_back) = ptr.as_reference() {
    assert_eq!(val_back, &10);
  }
}
```

# Null-Unchecked Version

If you are sure the pointer can never be null, use [`.into_raw_const()`][2] or
[`.into_raw_mut()`][3] and dereference the raw pointer.

[0]: https://doc.rust-lang.org/core/ptr/index.html#pointer-to-reference-conversion
[1]: Pointer::is_null
[2]: Pointer::into_raw_const
[3]: Pointer::into_raw_mut
[orig_c]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
[orig_m]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut
