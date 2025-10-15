Overwrites a memory location with the given value without reading or dropping
the old value.

`write` does not drop the contents of the destination. This is safe, but it
could leak allocations or resources, so care should be taken not to overwrite an
object that should be dropped.

Additionally, it does not drop the source. Semantically, the source is moved
into the location pointed to by the destination.

This is appropriate for initializing uninitialized memory, or overwriting memory
that has previously been [`read`] from.

# Original

- [`core::ptr::write`]
- [`<*mut T>::write`][ptr_write]

# Safety

Behavior is undefined if any of the following conditions are violated:

- the destination must be [valid] for writes.
- the destination must be properly aligned. Use [`write_unaligned`] if this is
  not the case.

Note that even if `T` has size `0`, the pointer must be properly aligned.

# Examples

Basic usage:

```rust
use funty::ptr::*;

let mut x = 0;
let y: Pointer::<i32, Unique> = (&mut x).into();
let z = 12;

unsafe {
  y.write(z);
  assert_eq!(y.read(), 12);
}
```

Manually implement [`mem::swap`]:

```rust
use funty::ptr::*;

fn swap<T>(a: &mut T, b: &mut T) {
  let a = Pointer::from_mut(a);
  let b = Pointer::from_mut(b);
  unsafe {
    // Create a bitwise copy of `*a` in `tmp`.
    let tmp = a.read();

    // Exiting here (by returning or panicking) would cause `tmp` to
    // drop while the same bit-pattern is still in `*a`. This could
    // trigger undefined behavior if `T` is not `Copy`.

    // Create a bitwise copy of `*b` in `*a`.
    // This is safe because mutable references cannot alias.
    a.copy_from_nonoverlapping(b, 1);

    // As above, exiting here could trigger undefined behavior because
    // the same value is referenced by `a` and `b`.

    // Move `tmp` into `b`.
    b.write(tmp);

    // `tmp` has been moved (`write` takes ownership of its second
    // argument), so nothing is dropped implicitly here.
  }
}

let mut foo = b"foo";
let mut bar = b"bar";

swap(&mut foo, &mut bar);

assert_eq!(foo, b"bar");
assert_eq!(bar, b"foo");
```

[ptr_write]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write
[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
[`mem::swap`]: core::mem::swap
[`read`]: crate::ptr::read
[`write_unaligned`]: crate::ptr::write_unaligned
