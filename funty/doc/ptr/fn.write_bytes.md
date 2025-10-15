Sets `count * size_of::<T>()` bytes of memory starting at the destination to `val`.

`write_bytes` is similar to C’s [`memset`], but sets `count * size_of::<T>()` bytes to val.

# Original

- [`core::ptr::write_bytes`]
- [`<*mut T>::write_bytes`][ptr_write_bytes]

# Safety

Behavior is undefined if any of the following conditions are violated:

- the destination must be valid for writes of `count * size_of::<T>()` bytes.
- the destination must be properly aligned.

Note that even if the effectively copied size (`count * size_of::<T>()`) is 0, the pointer must be properly aligned.

Additionally, note that changing the destination in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`. For instance, the following is an incorrect use of this function:

```rust
use funty::ptr::*;

unsafe {
  let mut value: u8 = 0;
  let ptr = Pointer::from_mut(&mut value).cast::<bool>();
  let _bool = ptr.read(); // This is fine, `ptr` points to a valid `bool`.
  ptr.write_bytes(42u8, 1); // This function itself does not cause UB...
  let _bool = ptr.read(); // ...but it makes this operation UB! ⚠️
}
```

# Examples

Basic usage:

```rust
use funty::ptr::*;

let mut vec = vec![0u32; 4];
unsafe {
  let vec_ptr = Pointer::from_mut(vec.as_mut_ptr());
  vec_ptr.write_bytes(0xfe, 2);
}
assert_eq!(vec, [0xfefefefe, 0xfefefefe, 0, 0]);
```

[ptr_write_bytes]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write_bytes
[`memset`]: https://en.cppreference.com/w/c/string/byte/memset
