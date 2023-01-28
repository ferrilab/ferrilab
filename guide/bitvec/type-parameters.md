# Type Parameters

`bitvec` uses type parameters to permit precise user control of its behavior and
in-memory representation. The Rust generic system permits `bitvec` to have a
more powerful and capable behavior than any other bitstream library yet
implemented in any language.

All `bitvec` types take two type parameters:

1. `T`: the storage type. The slice types (`BitSlice`, `BitVec`, `BitBox`)
   select an unsigned integer, since as slices they store the length in their
   runtime state. `BitArray` uses either a bare integer or an integer array as
   its storage parameter, since it has no runtime state of its own and stores
   all of its information in the type system.
1. `O`: the ordering of bits within a single `T` element. We provide two
   orderings, `Msb0` and `Lsb0`.

The combination of these two type parameters governs how `bitvec` translates its
abstract storage (`usize -> bool`) into real memory; if you do not care about
real-memory representation, then the default type parameters `<usize, Lsb0>`
will give you the best performance. If you do care about this, then the
[memory representation][mr] chapter goes into detail about all the combinations
and can help you select which one best fits your needs.

The [`BitOrder`] trait is open for third-party implementation. It describes its
requirements in great detail in its API documentation, so if you have a memory
representation that is neither `Lsb0` nor `Msb0`, you can implement the ordering
yourself and `bitvec` will use it without complaint.

----

Rust syntax requires explicitly choosing type parameters when using generic
expressions, such as `BitVec::<Store, Order>::new()`, and will not substitute in
the default parameters when attempting to elide the parameters with
`BitVec::new()`. However, Rust *will* use the default type parameters in
patterns: `let bv: BitVec = BitVec::new();` will use the default type parameters
in the `: BitVec` type annotation, which then completes the type of the
expression on the right side of the assignment `=`.

[`BitOrder`]: https://docs.rs/bitvec/latest/bitvec/order/trait.BitOrder.html
[mr]: ./memory-representation.md
