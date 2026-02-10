Tests if the pointer is null.

Note that for `T: !Sized` there are many possible null pointers, as only the raw
data pointer is considered, not the length, vtable, or other metadata.
Therefore, two pointers that are null may still not compare equal to each other.

# Original

[`<*T>::is_null`][orig]

# Panics During `const` Evaluation

If this method is used during const evaluation, and `self` is a pointer that is
offset beyond the bounds of the memory it initially pointed to, then there might
not be enough information to determine whether the pointer is null. This is
because the absolute address in memory is not known at compile time. If the
nullness of the pointer cannot be determined, this method will panic.

In-bounds pointers are never null, so the method will never panic for such
pointers.

# Examples

```rust
use ptxr::*;

let s: &str = "Follow the rabbit";
let ptr: Pointer<u8, Shared> = s.as_ptr().into();
assert!(!ptr.is_null());
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null
