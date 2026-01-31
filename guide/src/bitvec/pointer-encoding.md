# Bit Slice Pointer Encoding

`bitvec`’s core value proposition rests on the fact that it is capable of
defining an unsized slice type, and controlling references to it. The Rust
language rests heavily on the two reference types `&` and `&mut`, and does not
ordinarily allow these to be faked or created by anything other than the
compiler.

## Rust Reference Rules

It so happens that not only does Rust strongly guarantee the [in-memory layout]
of a reference to a slice, it also provides a stable API for
[constructing values] of `&[T]` type without using `mem::transmute`. Subject to
certain value requirements imposed by types, slice references can be constructed
through these functions and the compiler will accept them as valid.

These requirements traditionally make it difficult to encode non-address
information into a bare reference, since the compiler has a very firm
expectation that a reference to a type is immediately dereferenceäble to a value
of that type, but if your type happens to be zero-sized, then it can never exist
in memory, no loads or stores to it can ever be produced, and the compiler no
longer concerns itself with the actual bit-pattern value of references to it.

Which is why the definition of `BitSlice` is

```rust
//  src/slice.rs

#[repr(transparent)]
pub struct BitSlice<T, O>
where
  T: BitStore,
  O: BitOrder,
{
  _mem: [()],
  _typ: PhantomData<T>,
  _ord: PhantomData<O>,
}
```

`BitSlice` is `[()]` (a slice of the unit value) with some markers that only the
type-checker can see. `&BitSlice` is thus `&[()]`, and `&[()]` can have any
values it wants (except, of course, null) – the unit value has no alignment
requirements, can be placed anywhere in memory without worrying about whether
there is a backing allocation, and can have as many instances of itself as
desired.

Zero-sized types are an *absurdly* powerful concept when working with memory
that the language expects to be able to manifest at any time.

## Pointer Encoding

Slice references contain two pieces of information: the address of the base
element, and the number of elements, starting at the base, contained in the
slice region. Theoretically, bit-slice references have the same pair of
information: the address of the first bit, and the number of bits in the region.

However, computers are byte-addressed, not bit-addressed, so we need to store
three more bits (to select a bit in the base byte) somewhere in the reference.
Since slice references are defined as `{ base: *T, elts: usize }`, and there are
no[^1] spare bits in `*const _`, the bits to store the base bit are taken out of
the length counter.

Reference address values are also required to be integer multiples of the
alignment of the referent type `T`. This alignment is, on all supported targets,
the width in bytes of the referent type. As a result, there are as many low bits
in the address of any `T` that are *guaranteed* to be the `0` value, as there
are bits needed to select a byte within the element. The end result is that the
length counter must always use three bits to store the starting bit, and the
base address will be composed of an aligned `T` address and an index of the
starting byte within it.

As Rust does not have bitfield syntax, a definition of the pointer structure in
C++ looks like something like this[^2]:

```cpp
template <typename T>
struct BitSpan {
  static_assert(
    std::is_unsigned<T>()
    && sizeof(T) <= sizeof(std::size_t)
    && sizeof(T) <= alignof(T)
  );

  // on little-endian systems, bitfields are
  // allocated from LSbit and move towards MSbit
  uintptr_t ptr_head : __builtin_ctzll(alignof(T));
  uintptr_t ptr_addr : (sizeof(uintptr_t) * 8)
                     - __builtin_ctzll(alignof(T));

  size_t len_head : 3;
  size_t len_bits : (sizeof(size_t) * 8) - 3;
};
```

In Rust, the structure is declared as

```rust
// src/pointer.rs

#[repr(C)]
pub struct BitSpan<T, O>
where
  T: BitStore,
  O: BitOrder,
{
  ptr: NonNull<u8>,
  len: usize,
  _ty: PhantomData<T>,
  _or: PhantomData<O>,
}
```

and the logical components must be accessed through get/set functions, rather
than through compiler-generated field stubs.

By marking the pointer as `NonNull`, `BitSpan` declares that it will never be a
null pointer and becomes subject to the same peephole optimization that allows
`mem::size_of::<Option<&T>>() == mem::size_of::<&T>()`. By marking it as
unconditionally a pointer to `u8`, we declare that all low bits of the address
value are in use, and none can be used as slots for anything else (since our
encoding is using them to select a byte within the `T`).

## Significant Values

`BitSpan<T, O>` does not have any sentinel values of its own, but inherits from
`NonNull<T>`. The completely zero value is not a valid member of the `BitSpan`
type, but rather indicates `Option::<BitSpan<_, _>>::None`, and it uses the
dangling `NonNull` pointer value to indicate an instantiated pointer object
without an associated allocation.

Not all zero-length regions are dead: a cleared `BitVec` region has zero length
but owns an allocation, so it cannot discard its address information.

## Summary

Rust requires that slice references have a specific ABI, but makes no
requirements about the encoding of values of those references for certain types.
We can supply our own ABI-equivalent structure, define functions that use the
structural encoding to compute the information needed to actually interact with
memory, and convert our structures into Rust-accepted slices through the
provided compiler API in `core`.

If and when the `ptr_metadata` feature stabilizes, `bitvec` will experiment with
discarding this packed encoding in favor of a three-word pointer. If the
unpacked pointer results in better performance by eliminating the need for the
special encoding, `bitvec` will release a new **minor** version with the changed
structure.

```admonish info
`bitvec`’s MSRV policy is that raising compiler requirements is a minor change,
not major, and the pointer ABI is **not public interface**! You are already
forbidden from moving bit-region pointers out of a program, so this change will
not affect your program’s behavior.
```

## Footnotes

[^1]: On AMD64, pointers are actually aggregates of [MMU translation pages], and
  processors only decode the low 48 or 57 bits of them, leaving the high 16 or 7
  bits available for other information not part of the memory addressing system.
  However, these processors also trap when attempting to dereference a pointer
  whose high `[48:64]` or `[57:64]` bits do not have the same bit value as bit
  `[47]` or `[56]`, and that bit is typically used to differentiate unprivileged
  user memory from privileged kernel memory. Furthermore, this dead region does
  not exist on 32-bit architectures, x86 or otherwise, and since `bitvec`
  explicitly supports 32-bit systems, the use of dead bits only present on a
  subset of supported targets and subject to their own extra rules is not
  worthwhile.

[^2]: Here is a full code listing which you can also [view on Godbolt][godbolt]:
  ```cpp
  // compiles on x86-64 clang 15.0.0
  #include <climits>
  #include <cstddef>
  #include <cstdint>
  #include <type_traits>

  static_assert(CHAR_BIT == 8, "this target is not supported");

  template <typename T>
  struct BitSpan {
    static_assert(
      std::is_unsigned<T>()
      && sizeof(T) <= sizeof(std::size_t)
      && sizeof(T) <= alignof(T),
      "this type is not supported as BitSpan storage"
    );

    uintptr_t ptr_head : __builtin_ctzll(alignof(T));
    uintptr_t ptr_addr : (sizeof(uintptr_t) * 8)
                      - __builtin_ctzll(alignof(T));

    size_t len_head : 3;
    size_t len_bits : (sizeof(size_t) * 8) - 3;
  };

  template <>
  struct BitSpan<uint8_t> {
    // ptr_head is zero bits wide when targeting bytes
    uintptr_t ptr_addr;
    size_t len_head : 3;
    size_t len_bits : (sizeof(size_t) * 8) - 3;
  };

  static uint64_t data[4];

  BitSpan<uint8_t> one() {
    return {
      .ptr_addr = (uintptr_t)&data[0],
      .len_head = 1,
      .len_bits = 6,
    };
  }

  BitSpan<uint16_t> two() {
    return {
      .ptr_head = 1,
      .ptr_addr = (uintptr_t)&data[1],
      .len_head = 1,
      .len_bits = 5,
    };
  }
  BitSpan<uint32_t> four() {
    return {
      .ptr_head = 2,
      .ptr_addr = (uintptr_t)&data[2],
      .len_head = 3,
      .len_bits = 10,
    };
  }
  BitSpan<uint64_t> eight() {
    return {
      .ptr_head = 4,
      .ptr_addr = (uintptr_t)&data[3],
      .len_head = 5,
      .len_bits = 25,
    };
  }
  ```

[MMU translation pages]: https://en.wikipedia.org/wiki/X86-64#Virtual_address_space_details
[constructing values]: https://github.com/rust-lang/rust/blob/8558ccd/src/libcore/slice/mod.rs#L5642-L5739
[in-memory layout]: https://github.com/rust-lang/rust/blob/8558ccd/src/libcore/ptr/mod.rs#L220-L231

[godbolt]: https://clang.godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DIApACYAQuYukl9ZATwDKjdAGFUtAK4sGe1wAyeAyYAHI%2BAEaYxCAAbADspAAOqAqETgwe3r56KWmOAkEh4SxRMQm2mPYFDEIETMQEWT5%2BXJXVGXUNBEVhkdF6CvWNzTltQ929JWUSAJS2qF7EyOwc5gDMwcjeWADUJutu23gshAoH2CYaAIIbWzuY%2B4fIQ%2BhYVBdXt2abDNteewOR1ewQInxudz%2BDyebgIAE8kpgAPoEYhMM7g243CaOZBIpgKJSNCBuAAS1wASkiLABJAAqTwAIgdGbsABykfZmMwEBB4BS7YbATAEXb83YMVCihReJIpRqYdDmMyzA5WCE3AiYFhJAxamHwxHMNi7OmYobELwOXYWQhCJKGfbxdXXXa7HF4PEEokECBfN1u14gED8pFeBhpYAhJWHM3rbAQVU3ANc2LmWLuvAAL0wqCoEDpsxhLMzObzECDIDSOZRSddAfT6dLufzheL61ZYjwUfLhdI/ob3N54sNj3FkulsvlWvQuwJNrtDoY7qIaOFyoHqvWLoHXlBSVRKN2B%2BISIQmCYs5AuyRSIie9ojgYSIcWdotAgXZ7rdmW5dbr3QQTyPYDL3QYhdmvCtsxbCBAIIYCCCLAAqdk6xTDDMMwgBaG87wfJ8XwIN8Py/SUfz/L4B2rZFRXoZ9z0vSDdnWNVqJgo96LvM5mOgst8xo2tdlQtki1w1jtyo%2BJmUkjVri1HU9UeIFzVRK1RVtYQlyBeC2RRC4nX/XYAHpjOPQ9GNncUc2IVBdgiHiAHc8D2Rzz2XIURWCYB7LhLVzmTXZ4MQ8zTzA4g2MCwS6MYM8LyvFjIvraLdi4hyCAFKCaPLaKULQ3ZxKSkxpKK7F6lxILQViSQj3QJh6hMABWCxJCamSdxuTT7UMHTQT0sF412ARMETQyB2IEUlmXYqjLdAA6UC3ggks4P3Q8kPTOqGuajQ2v7QL5q4yymV2NoB0O2L0oFEtYn2%2BtivaqTmTkrrtMOeCuFifTBoIRzUFGmbxsm4hpudc7dgWiz4pOs6DohxbwJO1agPW1UzFiLamCaiwuD28G5qO6GS1h%2BsLufK6Tsau63QeoqSs6xceve0F1jMb7sF2fglgBsHAomggprGuHIdPY6SzMamAxF/ElqR4LUc2%2Bqseasw8eFwmmJLdZJbJ7iMphjRJdp2Tbnp65XqZtx4Oq9ndkwbsEF9ItAb54HQdm%2BGoc1jtdmkfGEeWn3kYQhX0cx7HWMaxkdYhjXZxLKn8bSnjxcTwLjZ3M2OHmWhOEa3g/A4LRSFQTg3Gsax3UWZZlJ%2BHhSAITRs/mABrEBGsN3OOEkAum5LzheAUEBDcbovs9IOBYBgRAUFQHU6GichKDQef6BibZDGALgO8NrAADdPUwAA1PBMEcgB5I1OHrmhH2iIeIAiPuHOYYg4Wv3gX4aOFz4ibRMAcB/UgK82CCHPgwWg78x6kCwCwTe4hoH4Amg4PAe9MBD2gZgVQACvBaiAaCKofdaB4AiGiN%2BHgsB91RCcD%2B8wqAGGAAoE%2BZ9L6MCAfwQQIgxDsCkDIQQigVDqGgboNoBgjAoArpYfQJCh6QHmKgA8GQMGDyqAAmoLgGDuE8C0fwmipj9BiG0PI6QBCjFaMkVIJiGD6NKAMcYqiUECC6CMbRYx2hqM6MMHowQ%2Bi2MMbYLxZjBheJsTMLg8wFDVxWHMfQede7QNLhwXYqg2SxGwtVXYG8jCnUanNDQeTdgQFwIQEgXJ1jhN4KPLQv5SBtw7rE7uvAWDt0NoXYuiTB7Dwbk3eYk8Z6LAQrgpeEAV5JAXsQUIrBVgpLSRkrJPlt55LybwRUJTiAuT0Bw4QohxC8K2QItQfcRGkEcmiJItCGn51IG03giTz64KSLgoaVBkmpPSZITJYiFm5PyRoQpHhV7RDKRU7pY8amWQGImBpPdSDNPqTc/uHBOkjx6a3FpDT1jxPaQPUF1Sc6cDMFi25OKqnN1IGg4gaRnCSCAA
