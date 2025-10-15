# Access Permissions

This trait moves the shared/exclusive access system from disparate fundamentals
(`*const T`/`&T`, `*mut T`/`&mut T`) into the trait and generic system. It is
implemented by two token types: [`Shared`] and [`Unique`], which correspond to
`*const`/`&` and `*mut`/`&mut`, respectively.

When combined with the [`Pointer`] and [`Reference`] types, these can be used to
track access controls through the trait system, including implementing a
permission history stack in the type parameters.

This is primarily a _marker_ trait, intended to be consumed only as a type bound
or through the marker structs which implement it. The `Impl` supertrait contains
private implementation details that allow `Pointer<T, impl Permission>` to
forward to the real language fundamental types, but are not otherwise
interesting.

## Implementors

- [`Shared`] corresponds to the `*const T` pointer
- [`Unique`] corresponds to the `*mut T` pointer
- `(Shared, impl Permission)` corresponds to a _derived_ pointer which remembers
  the original `Permission` from which it was derived. Client code can therefore
  use these recursive tuples as a type-level history-stack and be guaranteed
  correct behavior at runtime when attempting to restore derived pointers to
  their original permissions.

## The History Stack

The [Stacked Borrows][sb] system that Ralf Jung is implementing in the Rust
Abstract Machine interpreters (Miri, `rustc`) implements a _much_ more robust
version of provenance-tracking than `funty` attempts to do. Its `Permission`
system is an `enum` which has the same three levels that `funty` provides, and
with nearly the same names:

- `Unique` is the same
- their `SharedReadOnly` is our `Shared`, `(Shared, Shared)`, etc.
- their `SharedReadWrite` is our `(Shared, Unique)`,
  `(Shared, (Shared, Unique))`, etc.

Non-generic code does not need to worry about the stack; it can just directly
type a `Pointer` or `NonNullPointer` as the appropriate permission and use it
without issue. Where the history stack shines is in generic library code that
needs to be blanket-implemented over both kinds of pointer. Library code that
is generic over `P: Permission` can always unwind any `P` to `Shared`, and can
attempt to unwind it to `Unique`, and be guaranteed that it will succeed only
when the pointer _entered into `funty`_ as a `*mut T`.

Any permissioned type can be made into a shared-read-write state by calling
`make_shared` on it. This pushes the `Permission` type variable from
`P -> (Shared, P)`. Limitations in the Rust type solver require this to be a
universal implementation, so a stack can be grown as deep as you want to force
it to before the compiler errors. However, there is no reason to only partially
unwind a history-stack. Therefore, the `make_unshared` function also available
on all permissioned types fully unwinds back to the original `P: Permission`.

## Usage

This trait’s primary public API is only its existence as a marker. It also
provides functions which help work with references, as inside a context which is
generic over `<P: Permission>`, the `P::Ref` type is opaque and cannot be
dereferenced, even though it is an alias for reference primitives.

You should use it for pointer-like types which contain an indirect reference to
other data storage. It can be used, for instance, as a local lens into a cache,
or to have one type which generically guards what kind of network request it can
send.

`bitvec` uses it to implement the `BitRef` type which acts as a library-level
reïmplementation of `&/mut bool`. By being generic over `P: Permission`, a
single `BitRef` type can be read-only or write-through; in particular, it needs
to access the runtime affordances of conditional `P -> Unique` unwinding because
`Drop` cannot be partially implemented on a type with generics:

```rust
use funty::ptr::*;

struct CacheKey<'a, T: 'a + Copy, P: Permission> {
  local: T,
  remote: Reference<'a, T, P>,
}

impl<T: Copy, P: Permission> Drop for CacheKey<'_, T, P> {
  fn drop(&mut self) {
    P::try_with_mut_ref(&mut self.remote, |r| *r = self.local);
  }
}
```

[sb]: https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md
