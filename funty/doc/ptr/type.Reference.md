# Reference Management

The `Reference` symbol is a type alias rather than a wrapping struct, as
references have essentially no inherent APIs to mirror and are most useful when
directly exposed.

You can use them as either `<P as Permission>::Ref<'a, T>` or
`Reference<'a, T, P>`.
