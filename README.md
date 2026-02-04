<div style="text-align: center;" align="center">

# ![Ferrilab][all_logo]

## Redefining the Rust fundamental data model

</div>

|      [![`bitvec`][b_logo]][bcu]       |      [![`radium`][r_logo]][rcu]       |      [![`funty`][f_logo]][fcu]       |      [![`pointdexter`][p_logo]][pcu]       |
|:-------------------------------------:|:-------------------------------------:|:------------------------------------:|:------------------------------------------:|
|     [![`bitvec` crate][bci]][bcu]     |     [![`radium` crate][rci]][rcu]     |     [![`funty` crate][fci]][fcu]     |     [![`pointdexter` crate][pci]][pcu]     |
|  [![`bitvec` MSRV - 1.65][bmi]][bcu]  |  [![`radium` MSRV - 1.60][rmi]][rcu]  |  [![`funty` MSRV - 1.65][fmi]][fcu]  |  [![`pointdexter` MSRV - 1.85][pmi]][pcu]  |
|    [![`bitvec` license][bli]][blu]    |    [![`radium` license][rli]][rlu]    |    [![`funty` license][fli]][flu]    |    [![`pointdexter` license][pli]][plu]    |
| [![`bitvec` documentation][bdi]][bdu] | [![`radium` documentation][rdi]][rdu] | [![`funty` documentation][fdi]][fdu] | [![`pointdexter` documentation][pdi]][pdu] |
|  [![`bitvec` downloads][bdli]][bcu]   |  [![`radium` downloads][rdli]][rcu]   |   [![`funty` downloads][fdli]][fcu]  |   [![`pointdexter` downloads][pdli]][pcu]  |

## Introduction

The Ferrilab project is a collection of crates that provide more powerful
alternatives to many basic Rust types. `bitvec` compresses `[bool]` to use truly
single-bit storage while still matching the standard slice and vector API,
`funty` allows you to be generic over properties of integers and pointers, and
`radium` provides tools for abstracting over kinds of shared mutability.

## Organization

Since `bitvec` depends on both `funty` and `radium` for its functionality, these
three crates are developed in a single workspace. However, `funty` and `radium`
stand entirely on their own and can be used independently of it.

Each crate has a much more detailed `README` describing what it does and how to
use it. The [project guide][0] explains more about how about the theory behind
their creation and provides user stories that don’t fit in the API
documentation.

## Why the Name?

myrrlyn is from the Great Lakes region of America, and began `bitvec` while
working at Kirtland AFB in Albuquerque. Since these crates reshape the way Rust
programs interact with the fundamental data types, the name “Fermi” jumped out
as a close analogue, and from there it was only a one-letter change to make it
fit the Rustacean community.

----

Both `funty` and `radium` are type-system crates with almost no runtime logic of
their own, so test coverage is not really meaningful for them. `bitvec` is
tested heavily, and is always in need of further work on its benchmarks, use
cases, and obscure behaviors.

<div style="text-align: center;" align="center">

[![codecov][all_codecov_img]][all_codecov_url]
[![coveralls][all_coveralls_img]][all_coveralls_url]

[![codecov][all_codecov_banner]][all_codecov_url]

</div>

## Maintenance

myrrlyn has not had the time or availability for sustained maintenance and
development on this project since 2022. Work is provided on a best-effort basis.
We apologize for the inconvenience.

<style type="text/css">
  h1 img {
    max-height: 6em;
  }
  thead img {
    height: 3em;
  }
</style>

[0]: https://ferrilab.github.io/ferrilab

<!-- Badges -->

[all_codecov_banner]: https://codecov.io/github/ferrilab/ferrilab/branch/main/graphs/icicle.svg?token=KNF0XPDE93
[all_codecov_img]: https://img.shields.io/codecov/c/github/ferrilab/ferrilab?style=for-the-badge&logo=codecov&token=KNF0XPDE93
[all_codecov_url]: https://codecov.io/github/ferrilab/ferrilab
[all_coveralls_img]: https://img.shields.io/coverallsCoverage/github/ferrilab/ferrilab?style=for-the-badge&logo=coveralls
[all_coveralls_url]: https://coveralls.io/github/ferrilab/ferrilab
[all_logo]: ./assets/ferrilab.svg "Ferrilab logo"

[bci]: https://img.shields.io/crates/v/bitvec.svg?style=for-the-badge&color=f46623 "bitvec crate badge"
[bcu]: https://crates.io/crates/bitvec "bitvec crate"
[bdi]: https://img.shields.io/docsrs/bitvec/latest.svg?style=for-the-badge "bitvec documentation badge"
[bdu]: https://docs.rs/bitvec/latest/bitvec "bitvec documentation"
[bdli]: https://img.shields.io/crates/dv/bitvec.svg?style=for-the-badge "bitvec downloads"
[blu]: https://github.com/ferrilab/ferrilab/blob/main/bitvec/LICENSE.txt "bitvec license"
[bli]: https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge "bitvec license badge"
[b_logo]: bitvec/assets/bitvec.svg "bitvec logo"
[bmi]: https://img.shields.io/badge/MSRV-1.65-f46623?style=for-the-badge&color=f46623&logo=rust "bitvec MSRV badge"

[fci]: https://img.shields.io/crates/v/funty.svg?style=for-the-badge&color=f46623 "funty crate badge"
[fcu]: https://crates.io/crates/funty "funty crate"
[fdi]: https://img.shields.io/docsrs/funty/latest.svg?style=for-the-badge "funty documentation badge"
[fdu]: https://docs.rs/funty/latest/funty "funty documentation"
[fdli]: https://img.shields.io/crates/dv/funty.svg?style=for-the-badge "funty downloads"
[flu]: https://github.com/ferrilab/ferrilab/blob/main/funty/LICENSE.txt "funty license"
[fli]: https://img.shields.io/crates/l/funty.svg?style=for-the-badge "funty license badge"
[f_logo]: funty/assets/funty.svg "funty logo"
[fmi]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&color=f46623&logo=rust "funty MSRV badge"

[pci]: https://img.shields.io/crates/v/pointdexter.svg?style=for-the-badge&color=f46623 "pointdexter crate badge"
[pcu]: https://crates.io/crates/pointdexter "pointdexter crate"
[pdi]: https://img.shields.io/docsrs/pointdexter/latest.svg?style=for-the-badge "pointdexter documentation badge"
[pdu]: https://docs.rs/pointdexter/latest/pointdexter "pointdexter documentation"
[pdli]: https://img.shields.io/crates/dv/pointdexter.svg?style=for-the-badge "pointdexter downloads"
[plu]: https://github.com/ferrilab/ferrilab/blob/main/pointdexter/LICENSE.txt "pointdexter license"
[pli]: https://img.shields.io/crates/l/pointdexter.svg?style=for-the-badge "pointdexter license badge"
[p_logo]: pointdexter/assets/pointdexter.svg "pointdexter logo"
[pmi]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&color=f46623&logo=rust "pointdexter MSRV badge"

[rci]: https://img.shields.io/crates/v/radium.svg?style=for-the-badge&color=f46623 "radium crate badge"
[rcu]: https://crates.io/crates/radium "radium crate"
[rdi]: https://img.shields.io/docsrs/radium/latest.svg?style=for-the-badge "radium documentation badge"
[rdu]: https://docs.rs/radium/latest/radium "radium documentation"
[rdli]: https://img.shields.io/crates/dv/radium.svg?style=for-the-badge "radium downloads"
[rlu]: https://github.com/ferrilab/ferrilab/blob/main/radium/LICENSE.txt "radium license"
[rli]: https://img.shields.io/crates/l/radium.svg?style=for-the-badge "radium license badge"
[r_logo]: radium/assets/radium.svg "radium logo"
[rmi]: https://img.shields.io/badge/MSRV-1.60-f46623?style=for-the-badge&color=f46623&logo=rust "radium MSRV badge"
