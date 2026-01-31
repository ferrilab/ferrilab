Converts a reference to a pointer.

For `r: &T`, `from_ref(r)` is equivalent to `r as *const T`, and for
`r: &mut T`, `from_mut(r)` is equivalent to `r as *mut T` (except for the caveat
noted below), but are a bit safer since they will never silently change type or
mutability, in particular if the code is refactored.

The caller must ensure that the pointee outlives the pointer this function
returns, or else it will end up dangling.

When using `from_ref`, the caller must also ensure that the memory the pointer
(non-transitively) points to is never written to (except inside an `UnsafeCell`)
using this pointer or any pointer derived from it. If you need to mutate the
pointee, use [`from_mut`]. Specifically, to turn a mutable reference `m: &mut T`
into `*const T`, prefer `from_mut(m).make_shared()` to obtain a pointer that can
later be used for mutation.

# Original

- [`core::ptr::from_mut`]
- [`core::ptr::from_ref`]
- `<NonNull as From<&/mut T>>::from`

# Similar Functions

- [`crate::from_mut]
- [`crate::from_ref`]
- [`NonNullPointer::from`]

# Interaction with lifetime extension

Note that this has subtle interactions with the rules for lifetime extension of
temporaries in tail expressions. This code is valid, albeit in a non-obvious
way:

```rust
# type T = i32;
# fn foo() -> T { 42 }
// The temporary holding the return value of `foo` has its lifetime extended,
// because the surrounding expression involves no function call.
let p = &mut foo() as *mut T;
unsafe { p.write(T::default()) };
```

Naively replacing the cast with from_mut is not valid:

```rust,no_run
# type T = i32;
# fn foo() -> T { 42 }
use pointdexter::prelude::*;

// The temporary holding the return value of `foo` does *not* have its lifetime extended,
// because the surrounding expression involves a function call.
let p = ptr::from_mut(&mut foo());
unsafe { p.write(T::default()) }; // UB! Writing to a dangling pointer ⚠️
```

The recommended way to write this code is to avoid relying on lifetime extension
when raw pointers are involved:

```rust
# type T = i32;
# fn foo() -> T { 42 }
use pointdexter::prelude::*;

let mut x = foo();
let p = ptr::from_mut(&mut x);
unsafe { p.write(T::default()) };
```
