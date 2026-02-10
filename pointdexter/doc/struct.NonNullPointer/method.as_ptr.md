Unwraps the `NonNull` back to a regular `Pointer`.

# Original

[`NonNull::as_ptr`]

# Examples

```rust
use ptxr::*;

let mut x = 0u32;
let ptr = NonNullPointer::new((&mut x).into()).unwrap();

let x_ptr: Pointer<u32, Unique> = ptr.as_ptr();
let x_val = unsafe { x_ptr.read() };
assert_eq!(x_val, 0);

unsafe { x_ptr.write(2); }
let x_val = unsafe { ptr.read() };
assert_eq!(x_val, 2);
```
