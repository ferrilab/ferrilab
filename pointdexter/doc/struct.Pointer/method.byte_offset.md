Adds a signed offset in bytes to a pointer.

`count` is in units of **bytes**.

This is purely a convenience for casting to a `u8` pointer and using [`offset`]
on it. See that method for documentation and safety requirements.

For non-`Sized` pointees this operation changes only the data pointer, leaving
the metadata untouched.

# Original

- [`<*T>::byte_offset`][orig]
- [`NonNull::byte_offset`]

# Similar Functions

- [`Pointer::byte_offset`]
- [`NonNullPointer::byte_offset`]

# Safety

See [`offset`].

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_offset
[`offset`]: Self::offset
