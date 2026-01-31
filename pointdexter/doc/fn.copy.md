Copies `count * size_of::<T>()` bytes from the source to the destination. The
two regions may overlap.

If the source and destination will _never_ overlap, [`copy_nonoverlapping`] can
be used instead.

`copy` is semantically equivalent to C’s [`memmove`][0]. The free function and the
`copy_to` method swap the argument order, but the `copy_from` method matches the
`memmove` argument order. Additionally, the `count` argument is the number of
`T`s instead of the number of bytes. Copying takes place as-if the bytes were
copied from the source to a temporary array and then copied from the temporary
array to the destination.

The copy is “untyped” in the sense that data may be uninitialized or otherwise
violate the requirements of `T`. The initialization state is preserved exactly.

# Original

- [`core::ptr::copy`]
- [`<*T>::copy_to`][core_copyto]
- [`<*mut T>::copy_from`][core_copyfrom]
- [`NonNull::copy_from`]
- [`NonNull::copy_to`]

# Similar Functions

- [`crate::copy`]
- [`Pointer::copy_from`]
- [`Pointer::copy_to`]
- [`NonNullPointer::copy_from`]
- [`NonNullPointer::copy_to`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- the source must be [valid] for reads of `count * size_of::<T>()` bytes.
- the destination must be [valid] for writes of `count * size_of::<T>()`
  bytes, and must remain valid even when `src` is read for
  `count * size_of::<T>()` bytes. (This means if the memory ranges overlap, the
  destination pointer must not be invalidated by source reads.)
- Both the source and destination pointers must be properly aligned.

Like [`read`], `copy` creates a bitwise copy of `T`, regardless of whether `T`
is [`Copy`]. If `T` is not [`Copy`], using the values in both the source and
destination regions can [violate memory safety][1].

Note that even if the effectively copied size (`count * size_of::<T>()`) is `0`,
the pointers must be properly aligned.

# Examples

Efficiently create a Rust vector from an unsafe buffer:

```rust
use pointdexter::prelude::*;

/// # Safety
///
/// - `src` must be correctly aligned for its type and non-zero.
/// - `src` must be valid for reads of `elts` contiguous elements of type `T`.
/// - Those elements must not be used after calling this function unless `T: Copy`.
unsafe fn from_buf_raw<T, P: Permission>(src: Pointer<T, P>, elts: usize) -> Vec<T> {
  let mut dst = Vec::<T>::with_capacity(elts);

  // SAFETY: Our precondition ensures the source is aligned and valid,
  // and `Vec::with_capacity` ensures that we have usable space to write them.
  unsafe {
    ptr::copy(src, dst.as_mut_ptr().into(), elts);
  }

  // SAFETY: We created it with this much capacity earlier,
  // and the previous `copy` has initialized these elements.
  unsafe { dst.set_len(elts); }
  dst
}
```

[0]: https://en.cppreference.com/w/c/string/byte/memmove
[1]:
  https://doc.rust-lang.org/core/ptr/fn.read.html#ownership-of-the-returned-value
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[core_copyfrom]:
  https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from
[core_copyto]:
  https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to
