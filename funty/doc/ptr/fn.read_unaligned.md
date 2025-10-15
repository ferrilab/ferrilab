Reads the value from the source without moving it. This leaves the memory in the
source unchanged.

Unlike [`read`], `read_unaligned` works with unaligned pointers.

# Original

- [`core::ptr::read_unaligned`]
- [`<*T>::read_unaligned`][ptr_read_unaligned]

Safety

Behavior is undefined if any of the following conditions are violated:

- the source must be [valid] for reads.
- the source must point to a properly initialized value of type `T`.

Like [`read`], `read_unaligned` creates a bitwise copy of `T`, regardless of
whether `T` is [`Copy`]. If `T` is not [`Copy`], using both the returned value
and the value at the source can [violate memory safety][0].

On `packed` structs

Attempting to create a raw pointer to an `unaligned` struct field with an
expression such as `&packed.unaligned as *const FieldType` creates an
intermediate unaligned reference before converting that to a raw pointer. That
this reference is temporary and immediately cast is inconsequential as the
compiler always expects references to be properly aligned. As a result, using
`&packed.unaligned as *const FieldType` causes immediate undefined behavior in
your program.

Instead you must use the `&raw const` syntax to create the pointer. You may use
that constructed pointer together with this function.

An example of what not to do and how this relates to `read_unaligned` is:

```rust
use funty::ptr::*;

#[repr(packed, C)]
struct Packed {
  _padding: u8,
  unaligned: u32,
}

let packed = Packed {
  _padding: 0x00,
  unaligned: 0x01020304,
};

// Take the address of a 32-bit integer which is not aligned.
// In contrast to `&packed.unaligned as *const _`, this has no undefined behavior.
let unaligned = Pointer::from_const(&raw const packed.unaligned);

let v = unsafe { unaligned.read_unaligned() };
assert_eq!(v, 0x01020304);
```

Accessing unaligned fields directly with e.g. `packed.unaligned` is safe
however.

# Examples

Read a `usize` value from a byte buffer:

```rust
use funty::ptr::*;

fn read_usize(x: &[u8]) -> usize {
  assert!(x.len() >= size_of::<usize>());

  let ptr = Pointer::from_const(x.as_ptr()).cast::<usize>();

  unsafe { ptr.read_unaligned() }
}
```

[0]:
  https://doc.rust-lang.org/core/ptr/fn.read.html#ownership-of-the-returned-value
[ptr_read_unaligned]:
  https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`read`]: crate::ptr::read
