Copies `count * size_of::<T>()` bytes from the source to the destination. The
two regions must _not_ overlap.

For regions of memory which might overlap, use [`copy`] instead.

`copy_nonoverlapping` is semantically equivalent to C’s [`memcpy`]. The free
function and the `copy_to_nonoverlapping` method swap the argument order, but
the `copy_from_nonoverlapping` method matches the `memcpy` argument order.
Additionally, the `count` argument is the number of `T`s instead of the number
of bytes. Copying takes place as-if the bytes were copied from the source to a
temporary array and then copied from the temporary array to the destination.

The copy is “untyped” in the sense that data may be uninitialized or otherwise
violate the requirements of `T`. The initialization state is preserved exactly.

# Original

- [`core::ptr::copy_nonoverlapping`]
- [`<*T>::copy_to_nonoverlapping`][core_copyto]
- [`<*mut T>::copy_from_nonoverlapping`][core_copyfrom]
- [`NonNull::copy_from`]
- [`NonNull::copy_to]

# Similar Functions

- [`crate::copy_nonoverlapping`]
- [`Pointer::copy_from_nonoverlapping`]
- [`Pointer::copy_to_nonoverlapping`]
- [`NonNullPointer::copy_from_nonoverlapping`]
- [`NonNullPointer::copy_to_nonoverlapping`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- the source must be [valid] for reads of `count * size_of::<T>()` bytes.
- the destination must be [valid] for writes of `count * size_of::<T>()` bytes,
  and must remain valid even when `src` is read for `count * size_of::<T>()`
  bytes. (This means if the memory ranges overlap, the destination pointer must
  not be invalidated by source reads.)
- Both the source and destination pointers must be properly aligned.
- The region of memory beginning at the source with a size of
  `count * size_of::<T>()` bytes must _not_ overlap with the region of memory
  beginning at the destination with the same size.

Like [`read`], `copy` creates a bitwise copy of `T`, regardless of whether `T`
is [`Copy`]. If `T` is not [`Copy`], using the values in both the source and
destination regions can [violate memory safety][0].

Note that even if the effectively copied size (`count * size_of::<T>()`) is `0`,
the pointers must be properly aligned.

# Examples

Manually implement [`Vec::append`][1]:

```rust
use ptxr::*;

/// Moves all the elements of `src` into `dst`, leaving `src` empty.
fn append<T>(dst: &mut Vec<T>, src: &mut Vec<T>) {
  let src_len = src.len();
  let dst_len = dst.len();

  // Ensure that `dst` has enough capacity to hold all of `src`.
  dst.reserve(src_len);

  unsafe {
    // The call to add is always safe because `Vec` will never
    // allocate more than `isize::MAX` bytes.
    let dst_base: Pointer<T, Unique> = dst.as_mut_ptr().into();
    let dst_ptr = dst_base.add(dst_len);
    let src_ptr: Pointer<T, Shared> = src.as_ptr().into();

    // Truncate `src` without dropping its contents. We do this first,
    // to avoid problems in case something further down panics.
    src.set_len(0);

    // The two regions cannot overlap because mutable references do
    // not alias, and two different vectors cannot own the same
    // memory.
    ptxr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);

    // Notify `dst` that it now holds the contents of `src`.
    dst.set_len(dst_len + src_len);
  }
}

let mut a = vec!['r'];
let mut b = vec!['u', 's', 't'];

append(&mut a, &mut b);

assert_eq!(a, &['r', 'u', 's', 't']);
assert!(b.is_empty());
```

[0]:
  https://doc.rust-lang.org/core/ptr/fn.read.html#ownership-of-the-returned-value
[1]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.append
[core_copyfrom]:
  https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from
[core_copyto]:
  https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`memcpy`]: https://en.cppreference.com/w/c/string/byte/memcpy
