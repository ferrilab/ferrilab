Subtracts an unsigned offset in bytes from a pointer.

`count` is in units of bytes.

This is purely a convenience for casting to a `u8` pointer and using [`sub`] on
it. See that method for documentation and safety requirements.

For non-`Sized` pointees this operation changes only the data pointer, leaving
the metadata untouched.

# Original

- [`<*T>::byte_sub`][orig]
- [`NonNull::byte_sub`]

# Similar Functions

- [`Pointer::byte_sub`]
- [`NonNullPointer::byte_sub`]

# Safety

See [`sub`].

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_sub
[`sub`]: Self::sub
