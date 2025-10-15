Performs a volatile read of the value from the source without moving it.

Volatile operations are intended to act on I/O memory. As such, they are
considered externally observable events (just like syscalls, but less opaque),
and are guaranteed to not be elided or reordered by the compiler across other
externally observable events. With this in mind, there are two cases of usage
that need to be distinguished:

- When a volatile operation is used for memory inside an [allocation], it
  behaves exactly like [`read`], except for the additional guarantee that it
  won’t be elided or reordered (see above). This implies that the operation will
  actually access memory and not e.g. be lowered to reusing data from a previous
  read. Other than that, all the usual rules for memory accesses apply
  (including provenance). In particular, just like in C, whether an operation is
  volatile has no bearing whatsoever on questions involving concurrent accesses
  from multiple threads. Volatile accesses behave exactly like non-atomic
  accesses in that regard.
- Volatile operations, however, may also be used to access memory that is
  outside of any Rust allocation. In this use-case, the pointer does not have to
  be [valid] for reads. This is typically used for CPU and peripheral registers
  that must be accessed via an I/O memory mapping, most commonly at fixed
  addresses reserved by the hardware. These often have special semantics
  associated to their manipulation, and cannot be used as general purpose
  memory. Here, any address value is possible, including 0 and [`usize::MAX`],
  so long as the semantics of such a read are well-defined by the target
  hardware. The provenance of the pointer is irrelevant, and it can be created
  with [`without_provenance`]. The access must not trap. It can cause
  side-effects, but those must not affect Rust-allocated memory in any way. This
  access is still not considered [atomic], and as such it cannot be used for
  inter-thread synchronization.

Note that volatile memory operations where T is a zero-sized type are noops and
may be ignored. Safety

Like [`read`], `read_volatile` creates a bitwise copy of `T`, regardless of
whether `T` is [`Copy`]. If `T` is not [`Copy`], using both the returned value
and the value at the source can [violate memory safety][0]. However, storing
non-[`Copy`] types in volatile memory is almost certainly incorrect.

Behavior is undefined if any of the following conditions are violated:

- the source must be either [valid] for reads, or it must point to memory
  outside of all Rust allocations and reading from that memory must:
  - not trap, and
  - not cause any memory inside a Rust allocation to be modified.
- the source must be properly aligned.
- Reading from the source must produce a properly initialized value of type `T`.

Note that even if `T` has size `0`, the pointer must be properly aligned.

# Original

- [`core::ptr::read_volatile`]
- [`<*T>::read_volatile`][ptr_read_volatile]

# Examples

Basic usage:

```rust
use funty::ptr::*;

let x = 12;
let y = Pointer::from_const(&x);

unsafe {
    assert_eq!(y.read_volatile(), 12);
}
```

[0]:
  https://doc.rust-lang.org/core/ptr/fn.read.html#ownership-of-the-returned-value
[allocation]: https://doc.rust-lang.org/core/ptr/index.html#allocated-object
[atomic]:
  https://doc.rust-lang.org/core/sync/atomic/index.html#memory-model-for-atomic-accesses
[ptr_read_volatile]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`read`]: crate::ptr::read
