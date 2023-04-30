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

This is a fairly large, complex, and *difficult* project. myrrlyn has been
working on this more or less continuously since mid-2018. We neither expect, nor
even want, you to try to come close to that level of familiarity as a
requirement for showing up.

> If you want to learn it well enough to join the team, we’d love that too! It’s
> just a lot of work that we don’t ask of you unless you really want to do it.

The intended audience for this project is a Rust programmer who is familiar with
the use of borrowed vs. owned types, the use of traits to allow switching
between different implementations of the same behavior, and the basics of data
layout in memory. This is not an ideal project for a novice to use when learning
the Rust language or a first foray into memory and I/O protocol manipulation. We
are happy to help you learn these things, but [the Rust book][rust-book] is a
great resource for the language, and the
[Rust Embedded WG has a book][embed-book] to help you learn more about the
“bottom of the stack” of computer operation.

Additionally, we ask that you read our documentation before asking questions
about usage. We take pride in having comprehensive docs that answer the common
questions, and if they *don’t* answer yours, we want to know that so we can
improve them!

However, we do *not* ask that you read our Rust source code before making
feature requests. The source is complex and not straightforward. You do not need
to be familiar with the project internals to tell us what you need the project
to do.

## Contributing

Ferrilab is more than Rust source code! We accept, and actively desire,
contributions to *any* and *all* parts of the repository: source code, API
documentation, the user guide, even these meta-documents talking *about* the
rest of the project.

If you have a patch you think is worth inspecting right away, opening a pull
request without prelude is fine, although we would certainly appreciate an
accompanying explanation of what the patch does and why.

If you have questions, bugs, suggestions, or other contributions of any kind
that do not immediately touch the codebase, you can reach myrrlyn informally to
talk about them, or open an issue.

We will do our best to respond to all contacts in a timely manner.

## Getting Started

You’ll need to install the Rust language tools. Even if you’re not working on
the Rust source code, we use tools like `just` and `mdbook` which are most
easily installed through Rust rather than through your system package manager.

You can find instructions on how to install Rust and get it set up on your
computer at <https://rustup.rs/>.

We use three toolchains:

- the pinned MSRV in each crate,
- `stable` (all Ferrilab projects *must* work without nightly features)
- `nightly` (some tools, like `cargo fmt`, only work properly in the nightly
  version)

Please install *at least* `stable` and `nightly`. Rust will handle getting the
specific versions we use automatically.

## Environment

Ferrilab provides a `Justfile`. Please install [`just`] and use `just format`,
`just check`, and `just test` to ensure that your PR matches the rest of
Ferrilab.

To work on the user guide, you will need to install `mdBook`. You can do this
with

```shell
cargo +nightly install mdbook --vers 0.4.28
```

or you can search for it in your system package manager.

We also use `mdbook-admonish`, so no matter how you install the book generator,
you’ll need to use `cargo +nightly install` to get that plugin.

All tool versions and installation instructions are kept in their relevant
files: `book.toml` has everything needed to build the book, `rustfmt.toml` has
the Rustfmt version, etc.

If you are not certain how to get your environment set up, please *ask us*
*informally*, not through a GitHub issue. We do accept PRs in the event that our
tool documentation is stale or unhelpful.

[`just`]: https://github.com/casey/just
[embed-book]: https://docs.rust-embedded.org/book/
[rust-book]: https://doc.rust-lang.org/book/
