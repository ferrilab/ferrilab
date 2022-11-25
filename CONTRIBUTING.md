# Contributing Guide

Contributions, feature requests, usage questions, and general contacts of any
kind are *absolutely* welcome!

Please remember that this is a hobby project, and the maintainers work on it
solely in their free time. We are not sponsored or employed to maintain this
project, and cannot guarantee availability or rapid responsiveness.

## Contact Information

The primary author of the project is myrrlyn (Alexander Payne). If you need to
contact a maintainer, please reach out to him first.

In roughly descending order of likelihood that myrrlyn will receive and respond
to your contact, he can be reached through:

- email: [self@myrrlyn.dev](mailto:self@myrrlyn.dev)
- GitHub: [@myrrlyn](https://github.com/myrrlyn)
- Mastodon: [@myrrlyn@hachyderm.io](https://hachyderm.io/myrrlyn)
- Discord: `@myrrlyn#0611`. He is present in the Rust Programming Language and
  Rust Programming Language Community servers. If you are in either, you can
  @-mention him in them, or DM him.
- Reddit: [/u/myrrlyn](https://reddit.com/u/myrrlyn)

## Expectations and Non-Expectations

The `bitvec` crate intends to power the lowest level of memory manipulation
while offering a convenient, powerful, and idiomatic high-level API. We welcome
and actively encourage inputs on aspects of its construction, usage, and
documentation. We are intimately familiar with its operation and use, and as
such are likely to overlook aspects that are not obvious to newcomers. We ask
that you read the documentation we provide, but you do not need to dig through
our source code before asking a question or filing a PR.

## Contributing

If you have a patch you think is worth inspecting right away, opening a pull
request without prelude is fine, although we would certainly appreciate an
accompanying explanation of what the patch does and why.

If you have questions, bugs, suggestions, or other contributions of any kind
that do not immediately touch the codebase, you can reach myrrlyn informally to
talk about them, or open an issue.

We will do our best to respond to all contacts in a timely manner.

## Environment

Ferrilab provides a `Justfile`. Please install [`just`] and use `just format`,
`just check`, and `just test` to ensure that your PR matches the rest of
Ferrilab.

[`just`]: https://github.com/casey/just
