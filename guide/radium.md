# Radium

The [`radium`] crate provides a unifying model for shared-mutability over the
primitives. It does *not* handle more complex shared-mutability topics such as
mutices or locks: if you need to manage large structured data, you will need to
look elsewhere.

## `Radium` Trait

This trait allows your code to generically accept either an atomic type or a
`Cell` and interact with it through a unified API. It is implemented by all of
the standard library atomics, `Cell<{bool,{i,u}{8,16,32,64,128,size},*mut T}>`,
and the type families that Radium provides (described below).

The primitive type that a `Radium` implementor encloses is indicated by the
`Radium::Item` associated type. You can use this as a constraint in your trait
bounds (i.e. `<R: Radium<Item = i32>>`) in order to gain direct access to the
primitive, or you can require that `R::Item` implement other traits and interact
with it through them.

## Type Aliases

Radium provides type aliases for each primitive which *could* be atomic in the
standard library. The `Radium{Bool,{I,U}{8,16,32,64,128,size},Ptr}` symbols all
forward to their corresponding `AtomicType` when that symbol exists, or to
`Cell<Type>` when it does not.

Since these are type aliases rather than newtypes, you can globally replace the
`AtomicT` symbols with their `RadiumT` equivalent without any other changes to
your codebase.

## Type Families

Radium provides three type families which accept fundamentals as type
parameters. These families have no inherent API, and only implement `Radium`,
`Debug`, `Default`, and `From<T>`. They may or may not have a `Sync`
implementation, depending on whether they have atomic behavior.

1. The `Atom<T>` family corresponds to (and wraps) the standard library’s
   `core::sync::atomic::*` types. This family only accepts `T` parameters where
   an equivalent `AtomicT` symbol exists for the target; this means that on
   targets which do not have, for instance, `AtomicU64`, `Atom<u64>` will fail
   to compile. These are always `Sync`.

   `Atom` requires that type arguments implement `radium::marker::Atomic`.

2. The `Isotope<T>` family functions similarly to `Atom<T>`, except that it
   wraps Radium’s `RadiumT` type aliases. As such, `Isotope` is portable across
   targets with different atomic supports, and will never fail to compile;
   however, it will silently degrade from atomic to `Cell` behavior (including
   loss of `Sync`) when the requisite atomic types are missing.

   `Isotope` requires that type arguments implement `radium::marker::Nuclear`.

3. The `Radon<T>` family is a wrapper over `Cell<T>`. Like `Isotope`, it
   requires that type arguments implement `radium::marker::Nuclear`. It is never
   `Sync`.

## Examples

This contrived example is taken from `radium/examples/schroedinger.rs`. It
shows how the `Radium` trait can be used by a worker function to manipulate
data, and how the different types can be used to work in sequence or in
parallel.

> Note: Radium’s MSRV is 1.60, while the scoped-threads API used here stabilized
> in 1.63.

```rust
use radium::{Radium, types::{RadiumU64, Atom, Isotope, Radon}};
use std::{
  cell::Cell,
  sync::atomic::{AtomicU64, Ordering},
  thread,
  time::Duration,
};

fn do_work<R: Radium<Item = u64>>(this: &R, ident: u8) {
  let on_entry = this.load(Ordering::SeqCst);
  println!("{: >2} step 0 sees: {: >2}", ident, on_entry);

  let before_add = this.fetch_add(10, Ordering::SeqCst);
  println!("{: >2} step 1 sees: {: >2}", ident, before_add);

  let after_add = this.load(Ordering::SeqCst);
  println!("{: >2} step 2 sees: {: >2}", ident, after_add);

  thread::sleep(Duration::from_millis(after_add));

  let before_sub = this.fetch_sub(3, Ordering::SeqCst);
  println!("{: >2} step 3 sees: {: >2}", ident, before_sub);

  let on_exit = this.load(Ordering::SeqCst);
  println!("{: >2} step 4 sees: {: >2}", ident, on_exit);
}

static ATOM: AtomicU64 = AtomicU64::new(0);
static RADIUM: RadiumU64 = RadiumU64::new(0);

fn main() {
  let cell = Cell::new(0u64);

  let atom = Atom::new(0u64);
  let isotope = Isotope::new(0u64);
  let radon = Radon::new(0u64);

  println!("atoms");
  thread::scope(|s| for ident in 0 .. 3 {
    s.spawn(move || do_work(&ATOM, ident));
  });
  println!();
  thread::scope(|s| for ident in 3 .. 6 {
    let atom = &atom;
    s.spawn(move || do_work(atom, ident));
  });
  println!();

  println!("isotopes");
  thread::scope(|s| for ident in 6 .. 9 {
    s.spawn(move || do_work(&RADIUM, ident));
  });
  println!();
  for ident in 9 .. 12 {
    do_work(&isotope, ident);
  }
  println!();

  println!("cells");
  for ident in 12 .. 15 {
    do_work(&cell, ident);
  }
  println!();
  for ident in 15 .. 18 {
    do_work(&radon, ident);
  }
  println!();
}
```

[`radium`]: https://crates.io/crates/radium
