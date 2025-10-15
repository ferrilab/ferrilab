Overwrites a memory location with the given value without reading or dropping
the old value.

Unlike [`write`], the pointer may be unaligned.

`write_unaligned` does not drop the contents of the destination. This is safe, but it
could leak allocations or resources, so care should be taken not to overwrite an
object that should be dropped.

Additionally, it does not drop the source. Semantically, the source is moved into the
location pointed to by the destination.

This is appropriate for initializing uninitialized memory, or overwriting memory
that has previously been read with [`read_unaligned`].

# Safety

Behavior is undefined if any of the following conditions are violated:

- the destination must be [valid] for writes.

# On `packed` structs

Attempting to create a raw pointer to an `unaligned` struct field with an
expression such as `&packed.unaligned as *const FieldType` creates an
intermediate unaligned reference before converting that to a raw pointer. That
this reference is temporary and immediately cast is inconsequential as the
compiler always expects references to be properly aligned. As a result, using
`&packed.unaligned as *const FieldType` causes immediate undefined behavior in
your program.

Instead, you must use the `&raw mut` syntax to create the pointer. You may use
that constructed pointer together with this function.

An example of how to do it and how this relates to `write_unaligned` is:

```rust
use funty::ptr::*;

#[repr(packed, C)]
struct Packed {
  _padding: u8,
  unaligned: u32,
}

let mut packed: Packed = unsafe { std::mem::zeroed() };

// Take the address of a 32-bit integer which is not aligned.
// In contrast to `&packed.unaligned as *mut _`, this has no undefined behavior.
let unaligned = &raw mut packed.unaligned;

unsafe { unaligned.wrap_funty().write_unaligned(42) };

// `{...}` forces copying the field instead of creating a reference.
assert_eq!({packed.unaligned}, 42);
```

Accessing unaligned fields directly with e.g. `packed.unaligned` is safe however
(as can be seen in the `assert_eq!` above).

# Examples

Write a `usize` value to a byte buffer:

```rust
use funty::ptr::*;

fn write_usize(x: &mut [u8], val: usize) {
  assert!(x.len() >= size_of::<usize>());

  let ptr = Pointer::from_mut(x.as_mut_ptr()).cast::<usize>();

  unsafe { ptr.write_unaligned(val); }
}
```

[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
[`read_unaligned`]: crate::ptr::read_unaligned
[`write`]: crate::ptr::write
