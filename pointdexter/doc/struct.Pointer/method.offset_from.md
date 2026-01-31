Calculates the distance between two pointers within the same allocation. The
returned value is in units of `T`: the distance in bytes divided by
`size_of::<T>()`.

This is equivalent to
`(self.addr() - origin.addr()) / (size_of::<T>().addr())`, except that it has a
lot more opportunities for UB, in exchange for the compiler better understanding
what you are doing.

The primary motivation of this method is for computing the `len` of an
array/slice of `T` that you are currently representing as a “start” and “end”
pointer (and “end” is “one past the end” of the array). In that case,
`end.offset_from(start)` gets you the length of the array.

All of the following safety requirements are trivially satisfied for this
usecase.

# Original

- [`<*T>::offset_from`][orig]
- [`NonNull::offset_from`]

# Similar Functions

- [`Pointer::offset_from`]
- [`NonNullPointer::offset_from`]

# Safety

If any of the following conditions are violated, the result is Undefined Behavior:

- self and origin must either
  - point to the same address, or
  - both be [derived from][0] a pointer to the same [allocation][1], and the
    memory range between the two pointers must be in bounds of that object. (See
    below for an example.)
- The distance between the pointers, in bytes, must be an exact multiple of the
  size of `T`.

As a consequence, the absolute distance between the pointers, in bytes, computed
on mathematical integers (without “wrapping around”), cannot overflow an
`isize`. This is implied by the in-bounds requirement, and the fact that no
allocation can be larger than `isize::MAX` bytes.

The requirement for pointers to be derived from the same allocation is primarily
needed for `const`-compatibility: the distance between pointers into different
allocated objects is not known at compile-time. However, the requirement also
exists at runtime and may be exploited by optimizations. If you wish to compute
the difference between pointers that are not guaranteed to be from the same
allocation, use `(self.addr() - origin.addr()) / size_of::<T>()`.

# Panics

This function panics if `T` is a Zero-Sized Type (“ZST”).

# Examples

Basic usage:

```rust
use pointdexter::*;

let a = [0; 5];
let ptr1: Pointer<i32, Shared> = (&a[1]).into();
let ptr2: Pointer<i32, Shared> = (&a[3]).into();
unsafe {
  assert_eq!(ptr2.offset_from(ptr1), 2);
  assert_eq!(ptr1.offset_from(ptr2), -2);
  assert_eq!(ptr1.offset(2), ptr2);
  assert_eq!(ptr2.offset(-2), ptr1);
}
```

Incorrect usage:

```rust,no_run
use pointdexter::*;

let ptr1: Pointer<u8, Unique> = Box::into_raw(Box::new(0u8)).into();
let ptr2: Pointer<u8, Unique> = Box::into_raw(Box::new(1u8)).into();

let diff = (ptr2.addr() as isize).wrapping_sub(ptr1.addr() as isize);
// Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
let ptr2_other = ptr1.wrapping_offset(diff).wrapping_offset(1);
assert_eq!(ptr2.wrapping_add(1), ptr2_other);
// Since ptr2_other and ptr2 are derived from pointers to different objects,
// computing their offset is undefined behavior, even though
// they point to addresses that are in-bounds of the same object!
unsafe {
  let one = ptr2_other.offset_from(ptr2); // Undefined Behavior! ⚠️
}
```

[0]: https://doc.rust-lang.org/core/ptr/index.html#provenance
[1]: https://doc.rust-lang.org/core/ptr/index.html#allocation
[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from
