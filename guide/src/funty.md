# ![Funty](assets/funty.svg)

The `funty` crate (***fun***damental ***ty***pes) provides traits that unify
the Rust non-pointer primitives. It also unifies pointers and references by
lifting access permissions into the trait system.

## Fundamental Unification

The Rust primitives implement the following trait hierarchy and replicate their
standard-library inherent API and trait implementations.

- **`Fundamental`**: this is implemented by all primitives: `bool`, `char`, all
  integers, and both floats. It requires all traits that *all* primitives
  implement, and provides `.as_other()` methods that can replace `as`-casts.
  - **`Numeric`**: this is implemented by all integers and both floats. It adds
    the arithmetic operator traits, and methods for converting between the
    integer and its raw byte representation.
    - **`Integral`**: this is implemented by all integers. It adds bit-wise
      operator traits, attempted conversions between the other integers, and
      bit-shifts. It also provides most of the integer inherent API, as most of
      these methods are sign-agnostic.
      - **`Signed`**: this is implemented only by signed integers. It adds the
        absolute-value and sign-testing functions that unsigned integers don’t
        support.
      - **`Unsigned`**: this is implemented only by unsigned integers. It
        provides the `{is,next}_power_of_two` one-hot methods that only make
        sense on unsigned integers.
    - **`Floating`**: this is implemented by the floating-point numbers. Unlike
      the integral traits, it has a great deal of methods that only exist when
      `cfg(feature = "std")` is active, as they require the platform `libm`
      mathematics library and are not provided by Rust’s `core` crate. It also
      provides both all of the associated constants, as well as all of the
      constants stored in eponymous modules but *not* associated with the actual
      floating-point primitive types.

Additionally, `funty` provides marker traits for selecting bit-width: for `N`
in `8`, `16`, `32`, `64`, and `128`, the `IsN` trait is implemented by types
that are exactly that wide, `AtLeastN` is implemented by types that are that
width or more, and `AtMostN` is implemented by types that are that width or
less.

You can use these traits as generic constraints in code that needs to accept a
range of different primitives. The integral traits provide Peano constants (zero
and one), and can be constructed from literals for non-`const` work.
