# ![Bitvec](assets/bitvec.svg)

This is Ferrilab’s largest, most complex, project and likely the reason you’re
reading this guide. It provides a system that allows Rust code to treat memory
as if it were bit-addressed, rather than byte-addressed, and strives to be the
most *correct* and *capable* such bit-addressing system available. This results
in some unavoidable complexity and performance loss, and we are always working
to improve the quality of generated code.

## Introduction

`bitvec` was built out of myrrlyn’s experience and frustration with performing
I/O buffer manipulation using C, C++, and Ruby. His work required programs
capable of dynamically selecting an arbitrary region of a bit-stream (a task to
which C’s structural bitfield declarations are unsuited), and it required those
programs to be fast and portable to flight embedded systems (adjectives not
commonly associated with Ruby engines).

Furthermore, his work involved message schemas that were permitted to select a
bit ordering at the packet and field level. This is *not* a behavior that any
existing bit-stream library or language feature provides. These experiences
informed his goals and design choices from the very beginning.

`bitvec` matches, and exceeds, the functionality of every other bit-stream
implementation we have found. It is also the only Rust crate that is a drop-in
replacement for standard library types, and is able to do so while remaining
well-formed and conformant to Rust’s rules about memory access. Thanks to
excellent compiler engineering by the Rust and LLVM teams, it is able to do this
while still producing reasonably good output code.

## Goals for This Guide

We hope that this guide will explain `bitvec`’s design choices, philosophy,
backing theory, and overall goals. It is not a detailed exploration of the crate
API – this is [hosted on docs.rs][docsrs] – but instead seeks to communicate how
to think about `bitvec` so that you will know how to best use the APIs it
offers.

The best way I (myrrlyn) know how to communicate this information is as a
dialogue between me, the author, and you, the user. Since this is a book, not a
live conversation, I actively encourage you to get in contact with me with any
questions or feedback through the channels listed in Ferrilab’s [CONTRIBUTING]
document, and throughout the guide I will periodically remind you that if a
section is unclear, it is an error on my part, and I would appreciate an issue
or other contact so that I can improve it.

[CONTRIBUTING]: https://github.com/ferrilab/ferrilab/blob/main/CONTRIBUTING.md
[docsrs]: https://docs.rs/bitvec/latest/bitvec
