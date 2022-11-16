# Access Permissions

This trait moves the shared/exclusive access system from disparate fundamentals
(`*const T`/`&T`, `*mut T`/`&mut T`) into the trait and generic system. It is
implemented by two token types: `Shared` and `Unique`, which correspond to
`*const`/`&` and `*mut`/`&mut`, respectively.

When combined with the [`Pointer`] and [`Reference`] types, these can be used to
track access controls through the trait system, including implementing a
permission history stack in the type parameters.

This trait is not expected to be public API except as a bound for generic type
parameters. It contains functionality required to bridge between `Pointer` and
the raw pointers, but is otherwise not itself notable.
