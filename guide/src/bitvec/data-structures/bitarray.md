# Arrays

While `BitSlice` describes a region of borrowed data, `BitArray` provides a
container that can hold and manage such a region.

It is most comparable to the C++ type [`std::bitset<N>`]. Unfortunately, the
Rust support for type-level integers is still experimental, so it is unable to
take the length of the `BitSlice` it contains as a type parameter. Instead, it
must take the entire region it contains as a type parameter. The full type
declaration is

```rust,ignore
# use bitvec::prelude::*;
pub struct BitArray<
  A: BitViewSized,
  O: BitOrder,
> {
  _ord: PhantomData<O>,
  data: A,
}
```

As described in the [previous chapter], the `BitView` trait is implemented on
the unsigned integers, and on arrays of them.

```admonish info
Once type-level computation stabilizes, `BitArray` will change to have the type
parameters `<T: BitStore, O: BitOrder, const N: usize>`, matching the
`std::bitset<N>` length parameter. This will require a major-version increase.
```

This array dereferences to a `BitSlice` region over its entire length. It does
not currently permit shortening its `BitSlice` from either end. If this is a
behavior you want, please file an issue.

## Declaring a `BitArray` Type

Until Rust allows type-level integer computation to affect the memory layout,
`BitArray` types remain awkward to declare. You could declare types yourself by
using the `bitvec::mem::elts` function:

```rust
use bitvec::{array::BitArray, mem, order::Lsb0};

type MyArray = BitArray<[u8; mem::elts::<u8>(50)], Lsb0>;
```

But for convenience, we provide a `BitArr!` type-declaration macro. It expands
to exactly the expression above. It accepts the following syntaxes:

```rust
use bitvec::{BitArr, order::Lsb0};

// explicit ordering
type A = BitArr!(for 50, in u16, Lsb0);
// implicit ordering defaults to Lsb0
type B = BitArr!(for 50, in u32);
// implicit store defaults to usize
type C = BitArr!(for 50);
```

## Creating a `BitArray` Value

The `::ZERO` constant is a blank `BitArray` with its memory completely zeroed.
The `::new()` function wraps an existing element or array into a `BitArray`. In
addition, the macro constructor `bitarr!` takes the exact same arguments as the
`bits!` constructor, except that it returns an array directly rather than a
reference to a buffer.

Furthermore, `BitArray` structures and references can be constructed from
`&BitSlice` references using the `TryFrom` trait, just as arrays can be
constructed in the standard library.

## Using a `BitArray`

Once constructed, `BitArray` offers the `.as_bitslice()` and
`.as_mut_bitslice()` explicit methods, as well as all the standard traits, to
borrow its data as a `BitSlice`. The array has almost no functionality of its
own, and serves primarily to own a region used as a `BitSlice`. Like standard
library arrays, it natively produces a by-value iterator

Once you are done using `BitSlice` to manipulate the array, you can remove the
array with `.into_inner()` and regain the `A` memory within.

That’s everything that the array does! Like regular arrays, it is useful
primarily for its ability to move memory through a program, and has essentially
no behavior in its own right. As a plain data structure, it is most useful for
programs that do not have access to a dynamic allocator, and do not wish to use
`static` buffers. However, if you do have access to an allocator, you will
probably prefer to use `BitVec` instead.

`BitArray` is uniquely suited as a structural field for types which implement
I/O protocols and have fixed-size buffers, such as the headers of internet
transport packets, or CPU instruction set encodings. A full example can be found
in the [`bitvec` examples], but here is a short sample of how a TCP packet
header might be represented:

```rust
use bitvec::prelude::*;

#[derive(Clone, Copy, Default)]
struct TcpHeader {
  data: BitArr!(for 160, in u8; Msb0),
}

impl TcpHeader {
  fn data_offset(&self) -> usize {
    self.data[96 .. 100].load::<u8>() as usize
  }

  fn set_data_offset(&mut self, value: u8) {
    if !(5 ..= 15).contains(&value) {
      panic!("invalid data offset value");
    }
    self.data[96 .. 100].store(value);
  }

  fn syn_flag(&self) -> bool {
    self.data[110]
  }

  fn sequence_number(&self) -> u32 {
    self.data[32 .. 64].load_be()
  }

  fn as_bytes(&self) -> &[u8] {
    self.data.as_raw_slice()
  }
}
```

This snippet shows how you can:

- use `BitArray` as the storage inside a semantic new-type
- select individual flag bits
- use sub-byte regions for small integer storage
- use multi-byte regions for large integer storage
- access the raw storage for interaction with I/O systems

[`bitvec` examples]: https://github.com/ferrilab/ferrilab/blob/main/bitvec/examples/ipv4.rs
[previous chapter]: ./bitslice.md "BitSlice region"
[`std::bitset<N>`]: https://en.cppreference.com/w/cpp/utility/bitset "C++ std::bitset documentation"
