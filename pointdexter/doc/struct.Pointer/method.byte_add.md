Adds an unsigned offset in bytes to a pointer.

`count` is in units of bytes.

This is purely a convenience for casting to a `u8` pointer and using [`add`] on
it. See that method for documentation and safety requirements.

For non-`Sized` pointees this operation changes only the data pointer, leaving
the metadata untouched.

# Original

- [`<*T>::byte_add`][orig]
- [`NonNull::byte_add`]

# Similar Functions

- [`Pointer::byte_add`]
- [`NonNullPointer::byte_add`]

# Safety

See [`add`].

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_add
[`add`]: Self::add
