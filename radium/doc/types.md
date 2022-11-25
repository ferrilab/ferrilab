# Aliases and Type Families

This module provides `RadiumT` aliases that correspond to `AtomicT` from the
standard library, and resolve to `AtomicT` when it exists and `Cell<T>` when it
does not.

In addition, newtype structs `Atom<T>` and `Isotope<T>` correspond to (and
contain) `AtomicT` and `RadiumT`, respectively. These newtypes are designed to
be used in cases where you are generic over a primitive and want to plug it into
a shared-mutable wrapper type without having to specifically name one of the
`AtomicT` or `RadiumT` individual names.

Lastly, the `Radon<T>` newtype struct wraps `Cell<T>` and only implements the
`Radium` API, mirroring `Atom<T>` and `Isotope<T>`. This type exists so that
client crates can switch out types based on a crate feature to disable atomics,
and guarantee that their API will continue to function in every regard (except
for losing `Sync` impls).

## Examples

```rust
use radium::{Radium, types::*};

#[cfg(target_has_atomic = "ptr")]
let a = Atom::new(0usize);

let b = Isotope::new(1usize);

let c = Radon::new(2usize);

// when atomics are not disabled, use best-effort
#[cfg(feature = "atomics")]
pub type MyIsotope<T> = Isotope<T>;

// when atomics are fully disabled, enforce use of `Cell`
#[cfg(not(feature = "atomics"))]
pub type MyIsotope<T> = Radon<T>;
```
