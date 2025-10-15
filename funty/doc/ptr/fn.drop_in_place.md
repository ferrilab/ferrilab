Executes the destructor (if any) of the pointed-to value.

This is almost the same as calling [`ptr::read`] and discarding the result, but
has the following advantages:

- It is _required_ to use `drop_in_place` to drop unsized types like trait
  objects, because they can’t be read out onto the stack and dropped normally.
- It is friendlier to the optimizer to do this over [`ptr::read`] when dropping
  manually-allocated memory (e.g., in the implementations of `Box`/`Rc`/`Vec`),
  as the compiler doesn’t need to prove that it’s sound to elide the copy.
- It can be used to drop [pinned] data when `T` is not `repr(packed)` (pinned
  data must not be moved before it is dropped).

Unaligned values cannot be dropped in place; they must be copied to an aligned
location first using [`ptr::read_unaligned`]. For packed structs, this move is
done automatically by the compiler. This means the fields of packed structs are
not dropped in place.

# Original

- [`core::ptr::drop_in_place`] &rarr;
  [`funty::ptr::drop_in_place`](crate::ptr::drop_in_place)
- [`<*mut T>::drop_in_place`][0] &rarr; [`Pointer::drop_in_place`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- `to_drop` must be [valid] for both reads and writes.
- `to_drop` must be properly aligned, even if T has size 0.
- `to_drop` must be nonnull, even if T has size 0.
- The value `to_drop` points to must be valid for dropping, which may mean it
  must uphold additional invariants. These invariants depend on the type of the
  value being dropped. For instance, when dropping a Box, the box’s pointer to
  the heap must be valid.
- While drop_in_place is executing, the only way to access parts of `to_drop` is
  through the &mut self references supplied to the Drop::drop methods that
  drop_in_place invokes.

Additionally, if T is not [`Copy`], using the pointed-to value after calling
drop_in_place can cause undefined behavior. Note that `*to_drop = foo` counts as
a use because it will cause the value to be dropped again. [`ptr::write`] can be
used to overwrite data without causing it to be dropped.

# Examples

```rust
use funty::ptr::{self, *};
use std::rc::Rc;

let last = Rc::new(1);
let weak = Rc::downgrade(&last);

let mut v = vec![Rc::new(0), last];

unsafe {
  // Get a raw pointer to the last element in `v`.
  let ptr = (&mut v[1]).wrap_funty();
  // Shorten `v` to prevent the last item from being dropped. We do that first,
  // to prevent issues if the `drop_in_place` below panics.
  v.set_len(1);
  // Without a call `drop_in_place`, the last item would never be dropped,
  // and the memory it manages would be leaked.
  ptr::drop_in_place(ptr);
}

assert_eq!(v, &[0.into()]);

// Ensure that the last item was dropped.
assert!(weak.upgrade().is_none());
```

[0]: https://doc.rust-lang.org/std/primitive.pointer.html#method.drop_in_place
[pinned]: https://doc.rust-lang.org/core/pin/index.html
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`ptr::read`]: crate::ptr::read
[`ptr::read_unaligned`]: crate::ptr::read_unaligned
[`ptr::write`]: crate::ptr::write
