Adds an unsigned offset in bytes to a pointer using wrapping arithmetic.

`count` is in units of bytes.

This is purely a convenience for casting to a `u8` pointer and using
[`wrapping_add`] on it. See that method for documentation.

For non-`Sized` pointees this operation changes only the data pointer, leaving
the metadata untouched.

# Original

[`<*T>::wrapping_byte_add`][orig]

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_byte_add
[`wrapping_add`]: Self::wrapping_add
