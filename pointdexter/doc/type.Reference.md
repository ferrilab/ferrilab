Type alias for either `&T` or `&mut T`.

The reference primitive has no behavior of its own, and very few trait
implementations, so Pointdexter does not attempt to give it a newtype like the
pointers have. Client code needing to interoperate between pointers and
references should use use the interchange APIs on the pointer types, and produce
concretely-typed `&` or `&mut` references, rather than continue to use the
generic.
