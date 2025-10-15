Reads the value from the source without moving it. This leaves the memory in the
source unchanged.

# Original

- [`core::ptr::read`]
- [`<*T>::read`][ptr_read]

# Safety

Behavior is undefined if any of the following conditions are violated:

- the source must be [valid] for reads.
- the source must be properly aligned. Use [`read_unaligned`] if this is not the
  case.
- the source must point to a properly initialized value of type `T`.

Note that even if `T` has size `0`, the pointer must be properly aligned.

# Safety

Basic usage:

```rust
use funty::ptr::*;

let x = 12;
let y = Pointer::from_const(&x);

unsafe {
  assert_eq!(y.read(), 12);
}
```

Manually implement [`mem::swap`]:

```rust
use funty::ptr::{self, *};

fn swap<T>(a: &mut T, b: &mut T) {
  let a = Pointer::from_mut(a);
  let b = Pointer::from_mut(b);
  unsafe {
    // Create a bitwise copy of the value at `a` in `tmp`.
    let tmp = a.read();

    // Exiting at this point (either by explicitly returning or by
    // calling a function which panics) would cause the value in `tmp` to
    // be dropped while the same value is still referenced by `a`. This
    // could trigger undefined behavior if `T` is not `Copy`.

    // Create a bitwise copy of the value at `b` in `a`.
    // This is safe because mutable references cannot alias.
    ptr::copy_nonoverlapping(b, a, 1);

    // As above, exiting here could trigger undefined behavior because
    // the same value is referenced by `a` and `b`.

    // Move `tmp` into `b`.
    b.write(tmp);

    // `tmp` has been moved (`write` takes ownership of its second argument),
    // so nothing is dropped implicitly here.
  }
}

let mut foo = "foo".to_owned();
let mut bar = "bar".to_owned();

swap(&mut foo, &mut bar);

assert_eq!(foo, "bar");
assert_eq!(bar, "foo");
```

# Ownership of the Returned Value

`read` creates a bitwise copy of `T`, regardless of whether `T` is [`Copy`]. If
T is not [`Copy`], using both the returned value and the value at `*src` can
violate memory safety. Note that assigning to `*src` counts as a use because it
will attempt to drop the value at `*src`.

[`write`] can be used to overwrite data without causing it to be dropped.

```rust
use funty::ptr::{self, *};

let mut s = String::from("foo");
let s_ptr = Pointer::from_mut(&mut s);
unsafe {
  // `s2` now points to the same underlying memory as `s`.
  let mut s2: String = s_ptr.read();

  assert_eq!(s2, "foo");

  // Assigning to `s2` causes its original value to be dropped. Beyond
  // this point, `s` must no longer be used, as the underlying memory has
  // been freed.
  s2 = String::default();
  assert_eq!(s2, "");

  // Assigning to `s` would cause the old value to be dropped again,
  // resulting in undefined behavior.
  // s = String::from("bar"); // ERROR

  // `ptr::write` can be used to overwrite a value without dropping it.
  s_ptr.write(String::from("bar"));
}

assert_eq!(s, "bar");
```

[ptr_read]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`mem::swap`]: core::mem::swap
[`read_unaligned`]: crate::ptr::read_unaligned
[`write`]: crate::ptr::write
