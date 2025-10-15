Compares pointers for equality.

This is the same as using the `==` operator, but less generic: the arguments
have to be `Pointer`s, not anything that implements PartialEq.

Because `funty` types do not have any implicit type coërcion, this is less
necessary than the corresponding `core::ptr` function.

When comparing wide pointers, both the address and the metadata are tested for
equality. However, note that comparing trait object pointers
(`*const dyn Trait`) is unreliable: pointers to values of the same underlying
type can compare inequal (because vtables are duplicated in multiple codegen
units), and pointers to values of _different_ underlying type can compare equal
(since identical vtables can be deduplicated within a codegen unit).

# Original

[`core::ptr::eq`]

# Examples

```rust
use funty::ptr::{self, *};

let five = 5;
let other_five = 5;
let five_ref = &five;
let same_five_ref = &five;
let other_five_ref = &other_five;

assert!(five_ref == same_five_ref);
assert!(ptr::eq(five_ref.wrap_funty(), same_five_ref.wrap_funty()));

assert!(five_ref == other_five_ref);
assert!(!ptr::eq(five_ref.wrap_funty(), other_five_ref.wrap_funty()));
```

Slices are also compared by their length (fat pointers):

```rust
use funty::ptr::{self, *};

let a = [1, 2, 3];
assert!(ptr::eq((&a[..3]).wrap_funty(), (&a[..3]).wrap_funty()));
assert!(!ptr::eq((&a[..2]).wrap_funty(), (&a[..3]).wrap_funty()));
assert!(!ptr::eq((&a[0..2]).wrap_funty(), (&a[1..3]).wrap_funty()));
```
