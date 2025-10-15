# Non-Null Pointers

[`Pointer<T, P>`](`Pointer`) but non-zero and [covariant][0].

See the original [`NonNull`] docs for more information about variance. Because
the standard-library type exclusively wraps `*mut T`, it unconditionally exposes
write functionality that is not appropriate when describing read-only memory.
This type does not have that problem, as the [`Shared`] permission does not
provide write APIs.

This `repr(transparent)`ly wraps a Rust `NonNull<T>` to take advantage of its
niche optimization and ABI guarantees, while still providing the same
[`Permission`] tracking that [`Pointer`] does.

It mirrors the full `NonNull<T>` API, and is also interconvertible with
`Pointer` if you need it.

## Original

[`core::ptr::NonNull`][`NonNull`]

## API Differences

- The additional `P: Permission` type parameter governs whether values of this
  type can write through to their pointee or not.
- Original APIs that return `Option<NonNull<T>>` here return
  `Result<NonNullPointer<T, P>, NonNullError<T, P>>`. The error type is an empty
  marker which prints a useful message, so that client code can just use
  `.unwrap()` instead of writing out a custom message for `.expect("null")`.

## Representation

Thanks to the [null pointer optimization][1], `NonNullPointer<T, P>` and
`Option<NonNullPointer<T, P>>` (and
`Result<NonNullPointer<T, P>, NonNullError<T, P>>`) are guaranteed to have the
same size and alignment:

```rust
use funty::ptr::*;
use core::mem;

type Nnp<T> = NonNullPointer<T, Shared>;
type Opt<T> = Option<Nnp<T>>;
type Res<T> = Result<Nnp<T>, NonNullError<T, Shared>>;

assert_eq!(mem::size_of::<Nnp<i16>>(), mem::size_of::<Opt<i16>>());
assert_eq!(mem::size_of::<Opt<i16>>(), mem::size_of::<Res<i16>>());

assert_eq!(mem::size_of::<Nnp<str>>(), mem::size_of::<Opt<str>>());
assert_eq!(mem::size_of::<Opt<str>>(), mem::size_of::<Res<str>>());
```

[0]: https://doc.rust-lang.org/reference/subtyping.html
[1]: https://doc.rust-lang.org/core/option/index.html#representation
[`NonNull`]: core::ptr::NonNull
[`Shared`]: funty::ptr::Shared
