Swaps `count * size_of::<T>()` bytes between the two regions of memory beginning
at `x` and `y`. The two regions must _not_ overlap.

The operation is “untyped” in the sense that data may be uninitialized or
otherwise violate the requirements of `T`. The initialization state is preserved
exactly.

# Original

- [`core::ptr::swap_nonoverlapping`]

# Safety

Behavior is undefined if any of the following conditions are violated:

- Both `x` and `y` must be valid for both reads and writes of
  `count * size_of::<T>()` bytes.
- Both `x` and `y` must be properly aligned.
- The region of memory beginning at `x` with a size of `count * size_of::<T>()`
  bytes must not overlap with the region of memory beginning at `y` with the
  same size.

Note that even if the effectively copied size (`count * size_of::<T>()`) is `0`,
the pointers must be properly aligned.

# Examples

Basic usage:

```rust
use pointdexter::prelude::*;

let mut x = [1, 2, 3, 4];
let mut y = [7, 8, 9];

unsafe {
  ptxr::swap_nonoverlapping(x.as_mut_ptr().into(), y.as_mut_ptr().into(), 2);
}

assert_eq!(x, [7, 8, 3, 4]);
assert_eq!(y, [1, 2, 9]);
```

# Const evaluation limitations

If this function is invoked during const-evaluation, the current implementation
has a small (and rarely relevant) limitation: if `count` is at least 2 and the
data pointed to by `x` or `y` contains a pointer that crosses the boundary of
two `T`-sized chunks of memory, the function may fail to evaluate (similar to a
panic during const-evaluation). This behavior may change in the future.

The limitation is illustrated by the following example:

```rust,compile_fail
use core::mem;
use pointdexter::prelude::*;

const { unsafe {
  const PTR_SIZE: usize = mem::size_of::<*const i32>();
  let mut data1 = [0u8; PTR_SIZE];
  let mut data2 = [0u8; PTR_SIZE];

  let ptr1 = Pointer::from_mut(data1.as_mut_ptr());
  let ptr2 = Pointer::from_mut(data2.as_mut_ptr());

  // Store a pointer in `data1`.
  ptr1.cast::<*const i32>().write_unaligned(&42);
  // Swap the contents of `data1` and `data2` by swapping `PTR_SIZE` many `u8`-sized chunks.
  // This call will fail, because the pointer in `data1` crosses the boundary
  // between several of the 1-byte chunks that are being swapped here.
  //ptxr::swap_nonoverlapping(data1.as_mut_ptr(), data2.as_mut_ptr(), PTR_SIZE);
  // Swap the contents of `data1` and `data2` by swapping a single chunk of size
  // `[u8; PTR_SIZE]`. That works, as there is no pointer crossing the boundary between
  // two chunks.
  ptxr::swap_nonoverlapping(ptr1, ptr2, 1);
  // Read the pointer from `data2` and dereference it.
  let ptr = ptr2.cast::<*const i32>().read_unaligned();
  assert!(*ptr == 42);
} }
```
