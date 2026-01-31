Calculates the distance between two pointers within the same allocation. The
returned value is in units of bytes.

This is purely a convenience for casting to a `u8` pointer and using
[`offset_from`] on it. See that method for documentation and safety
requirements.

For non-`Sized` pointees this operation considers only the data pointers,
ignoring the metadata.

# Original

- [`<*T>::byte_offset_from`][orig]
- [`NonNull::byte_offset_from`]

# Similar Functions

- [`Pointer::byte_offset_from`]
- [`NonNullPointer::byte_offset_from`]

# Safety

See [`offset_from`].

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_offset_from
[`offset_from`]: Self::offset_from
