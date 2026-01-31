Calculates the distance between two pointers within the same allocation, _where
it’s known that `self` is equal to or greater than `origin`_. The returned value
is in units of **bytes**.

This is purely a convenience for casting to a `u8` pointer and using
[`offset_from_unsigned`] on it. See that method for documentation and safety
requirements.

For non-`Sized` pointees this operation considers only the data pointers,
ignoring the metadata.

# Original

- [`<*T>::byte_offset_from_unsigned`][orig]
- [`NonNull::byte_offset_from_unsigned`]

# Similar Functions

- [`Pointer::byte_offset_from_unsigned`]
- [`NonNullPointer::byte_offset_from_unsigned`]

# Notes

This stabilized in Rust 1.87, and so requires enabling a corresponding feature:

```toml
[dependencies.pointdexter]
features = ["rust_187"]
```

# Safety

See [`offset_from_unsigned`].

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.byte_offset_from_unsigned
[`offset_from_unsigned`]: Self::offset_from_unsigned
