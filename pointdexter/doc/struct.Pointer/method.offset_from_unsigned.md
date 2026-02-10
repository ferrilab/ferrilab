Calculates the distance between two pointers within the same allocation, _where
it’s known that `self` is equal to or greater than `origin`_. The returned value
is in units of `T`: the distance in bytes is divided by `size_of::<T>()`.

This computes the same value that [`offset_from`] would compute, but with the
added precondition that the offset is guaranteed to be non-negative. This method
is equivalent to `usize::try_from(self.offset_from(origin)).unwrap_unchecked()`,
but it provides slightly more information to the optimizer, which can sometimes
allow it to optimize slightly better with some backends.

This method can be thought of as recovering the `count` that was passed to
[`add`] (or, with the parameters in the other order, to [`sub`]). The following
are all equivalent, assuming that their safety preconditions are met:

```rust,ignore
ptr.offset_from_unsigned(origin) == count
origin.add(count) == ptr
ptr.sub(count) == origin
```

# Original

- [`<*T>::offset_from_unsigned`][orig]
- [`NonNull::offset_from_unsigned`]

# Similar Functions

- [`Pointer::offset_from_unsigned`]
- [`NonNullPointer::offset_from_unsigned`]

# Notes

This stabilized in Rust 1.87, and so requires enabling a corresponding feature:

```toml
[dependencies.pointdexter]
features = ["rust_187"]
```

# Safety

- The distance between the pointers must be non-negative (`self >= origin`)
- All the safety conditions of [`offset_from`] apply to this method as well; see
  it for the full details.

Importantly, despite the return type of this method being able to represent a
larger offset, it’s still not permitted to pass pointers which differ by more
than `isize::MAX` bytes. As such, the result of this method will always be less
than or equal to `isize::MAX as usize`.

# Panics

This function panics if `T` is a Zero-Sized Type (“ZST”).

# Examples

```rust
use ptxr::*;

let a = [0; 5];
let ptr1: Pointer<i32, Shared> = (&a[1]).into();
let ptr2: Pointer<i32, Shared> = (&a[3]).into();
unsafe {
  assert_eq!(ptr2.offset_from_unsigned(ptr1), 2);
  assert_eq!(ptr1.add(2), ptr2);
  assert_eq!(ptr2.sub(2), ptr1);
  assert_eq!(ptr2.offset_from_unsigned(ptr2), 0);
}

// This would be incorrect, as the pointers are not correctly ordered:
// ptr1.offset_from_unsigned(ptr2)
```

[orig]: https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from_unsigned
[`add`]: Self::add
[`sub`]: Self::sub
[`offset_from`]: Self::offset_from
