Performs a volatile write of a memory location with the given value without
reading or dropping the old value.

Volatile operations are intended to act on I/O memory. As such, they are
considered externally observable events (just like syscalls), and are guaranteed
to not be elided or reordered by the compiler across other externally observable
events. With this in mind, there are two cases of usage that need to be
distinguished:

- When a volatile operation is used for memory inside an [allocation], it
  behaves exactly like [`write`], except for the additional guarantee that it
  won’t be elided or reordered (see above). This implies that the operation will
  actually access memory and not e.g. be lowered to a register access. Other
  than that, all the usual rules for memory accesses apply (including
  provenance). In particular, just like in C, whether an operation is volatile
  has no bearing whatsoever on questions involving concurrent access from
  multiple threads. Volatile accesses behave exactly like non-atomic accesses in
  that regard.

- Volatile operations, however, may also be used to access memory that is
  outside of any Rust allocation. In this use-case, the pointer does not have to
  be [valid] for writes. This is typically used for CPU and peripheral registers
  that must be accessed via an I/O memory mapping, most commonly at fixed
  addresses reserved by the hardware. These often have special semantics
  associated to their manipulation, and cannot be used as general purpose
  memory. Here, any address value is possible, including 0 and [`usize::MAX`],
  so long as the semantics of such a write are well-defined by the target
  hardware. The provenance of the pointer is irrelevant, and it can be created
  with [`without_provenance`]. The access must not trap. It can cause
  side-effects, but those must not affect Rust-allocated memory in any way. This
  access is still not considered [atomic], and as such it cannot be used for
  inter-thread synchronization.

Note that volatile memory operations on zero-sized types (e.g., if a zero-sized
type is passed to `write_volatile`) are noops and may be ignored.

`write_volatile` does not drop the contents of `dst`. This is safe, but it could
leak allocations or resources, so care should be taken not to overwrite an
object that should be dropped when operating on Rust memory. Additionally, it
does not drop `src`. Semantically, `src` is moved into the location pointed to
by `dst`.

# Original

- [`core::ptr::write_volatile`]
- [`<*mut T>::write_volatile`][orig]
- [`NonNull::write_volatile`]

# Similar Functions

- [`crate::write_volatile`]
- [`Pointer::write_volatile`]
- [`NonNullPointer::write_volatile`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- `dst` must be either [valid] for writes, or it must point to memory outside of
  all Rust allocations and writing to that memory must:

  - not trap, and
  - not cause any memory inside a Rust allocation to be modified.

- `dst` must be properly aligned.

Note that even if T has size 0, the pointer must be properly aligned.

# Examples

Basic usage:

```rust
use ptxr::*;

let mut x = 0;
let y: Pointer<i32, Unique> = (&mut x).into();
let z = 12;

unsafe {
  y.write_volatile(z);
  assert_eq!(y.read_volatile(), 12);
}
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write_volatile
[allocation]: https://doc.rust-lang.org/core/ptr/index.html#allocated-object
[atomic]:
  https://doc.rust-lang.org/core/sync/atomic/index.html#memory-model-for-atomic-accesses
[valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
[`write`]: Pointer::write
[`without_provenance`]: Pointer::without_provenance
