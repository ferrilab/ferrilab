# `Lsb0` Little-Endian Integer Storing

This implementation uses the `Lsb0` bit-ordering to determine *which* bits in a
partially-occupied memory element are used for storage, using little-endian
element ordering.

See the [trait method definition][orig] for an overview of what element ordering
means.

## Narrowing Behavior

Integers are truncated from the high end. When storing into a bit-slice of
length `n`, the `n` least numerically significant bits are stored, and any
remaining high bits are ignored.

Be aware of this behavior if you are storing signed integers! The signed integer
`-14i8` (bit pattern `0b1111_0010u8`) will, when stored into and loaded back
from a 4-bit slice, become the value `2i8`.

## Examples

```rust
use bitvec::prelude::*;

let mut raw = 0u8;
raw.view_bits_mut::<Lsb0>()
   [1 .. 6]
   .store_le(22u8);
assert_eq!(raw, 0b00_10110_0);
//                 76 54321 0
raw.view_bits_mut::<Lsb0>()
   [1 .. 6]
   .store_le(-10i8);
assert_eq!(raw, 0b00_10110_0);
```

In bit-slices that span multiple elements, the little-endian element ordering
means that the slice index increases with numerical significance:

```rust
use bitvec::prelude::*;

let mut raw = [!0u8; 3];
raw.view_bits_mut::<Lsb0>()
   [4 .. 20]
   .store_le(0x2018u16);
assert_eq!(raw, [
  0x8_F,
//  7 0
  0x0_1,
// 15 8
  0xF_2,
// 23 16
]);
```

Note that while these examples use `u8` storage for convenience in displaying
the literals, `BitField` operates identically with *any* storage type. As most
machines use little-endian *byte ordering* within wider element types, and
`bitvec` exclusively operates on *elements*, the actual bytes of memory may
rapidly start to behave oddly when translating between numeric literals and
in-memory representation.

The [user guide] has a chapter that translates bit indices into memory positions
for each combination of `<T: BitStore, O: BitOrder>`, and may be of additional
use when choosing a combination of type parameters and store functions.

[orig]: crate::field::BitField::store_le
[user guide]: https://ferrilab.github.io/ferrilab/bitvec/memory-representation.html
