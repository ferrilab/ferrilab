# ![Introduction to Ferrilab][all_logo]

Ferrilab is an umbrella project for crates that experiment with reshaping the
Rust data model about primitive types. It began with the `bitvec` crate; as
`bitvec` expanded, it eventually spun off its integer-generalization into
`funty`, and `radium` was created independently but swiftly brought into the
fold.

Ferrilab is currently focused on just these crates, but may expand in the future
as we discover new ideas to try out.

## Components

|      [![`bitvec`][b_logo]][bcu]       |      [![`radium`][r_logo]][rcu]       |      [![`funty`][f_logo]][fcu]       |      [![`pointdexter`][p_logo]][pcu]       |
|:-------------------------------------:|:-------------------------------------:|:------------------------------------:|:------------------------------------------:|
|     [![`bitvec` crate][bci]][bcu]     |     [![`radium` crate][rci]][rcu]     |     [![`funty` crate][fci]][fcu]     |     [![`pointdexter` crate][pci]][pcu]     |
|  [![`bitvec` MSRV - 1.65][bmi]][bcu]  |  [![`radium` MSRV - 1.60][rmi]][rcu]  |  [![`funty` MSRV - 1.65][fmi]][fcu]  |  [![`pointdexter` MSRV - 1.85][pmi]][pcu]  |
|    [![`bitvec` license][bli]][blu]    |    [![`radium` license][rli]][rlu]    |    [![`funty` license][fli]][flu]    |    [![`pointdexter` license][pli]][plu]    |
| [![`bitvec` documentation][bdi]][bdu] | [![`radium` documentation][rdi]][rdu] | [![`funty` documentation][fdi]][fdu] | [![`pointdexter` documentation][pdi]][pdu] |
|  [![`bitvec` downloads][bdli]][bcu]   |  [![`radium` downloads][rdli]][rcu]   |   [![`funty` downloads][fdli]][fcu]  |   [![`pointdexter` downloads][pdli]][pcu]  |

## Behind the Name

The primary maintainer, myrrlyn, is from the Great Lakes region of the Americas
and the crates here all reshape the fundamental types in the Rust language. We
looked for names that had to do with early modern physics, and settled on Enrico
Fermi, as he worked in atomic physics, has an eponymous laboratory near Chicago,
and Fermilab was a single edit step away from Rust’s mascot.

<small>Plus, `bitvec` began while myrrlyn was working for the US Government in
New Mexico…</small>

<style type="text/css">
  thead th img { height: 3em; }
</style>

[all_logo]: ./assets/ferrilab.svg "Ferrilab logo"

[bci]: https://img.shields.io/crates/v/bitvec.svg?style=for-the-badge&color=f46623 "bitvec crate badge"
[bcu]: https://crates.io/crates/bitvec "bitvec crate"
[bdi]: https://img.shields.io/docsrs/bitvec/latest.svg?style=for-the-badge "bitvec documentation badge"
[bdu]: https://docs.rs/bitvec/latest/bitvec "bitvec documentation"
[bdli]: https://img.shields.io/crates/dv/bitvec.svg?style=for-the-badge "bitvec downloads"
[blu]: https://github.com/ferrilab/ferrilab/blob/main/bitvec/LICENSE.txt "bitvec license"
[bli]: https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge "bitvec license badge"
[b_logo]: ./assets/bitvec.svg "bitvec logo"
[bmi]: https://img.shields.io/badge/MSRV-1.65-f46623?style=for-the-badge&color=f46623&logo=rust "bitvec MSRV badge"

[fci]: https://img.shields.io/crates/v/funty.svg?style=for-the-badge&color=f46623 "funty crate badge"
[fcu]: https://crates.io/crates/funty "funty crate"
[fdi]: https://img.shields.io/docsrs/funty/latest.svg?style=for-the-badge "funty documentation badge"
[fdu]: https://docs.rs/funty/latest/funty "funty documentation"
[fdli]: https://img.shields.io/crates/dv/funty.svg?style=for-the-badge "funty downloads"
[flu]: https://github.com/ferrilab/ferrilab/blob/main/funty/LICENSE.txt "funty license"
[fli]: https://img.shields.io/crates/l/funty.svg?style=for-the-badge "funty license badge"
[f_logo]: ./assets/funty.svg "funty logo"
[fmi]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&color=f46623&logo=rust "funty MSRV badge"

[pci]: https://img.shields.io/crates/v/pointdexter.svg?style=for-the-badge&color=f46623 "pointdexter crate badge"
[pcu]: https://crates.io/crates/pointdexter "pointdexter crate"
[pdi]: https://img.shields.io/docsrs/pointdexter/latest.svg?style=for-the-badge "pointdexter documentation badge"
[pdu]: https://docs.rs/pointdexter/latest/pointdexter "pointdexter documentation"
[pdli]: https://img.shields.io/crates/dv/pointdexter.svg?style=for-the-badge "pointdexter downloads"
[plu]: https://github.com/ferrilab/ferrilab/blob/main/pointdexter/LICENSE.txt "pointdexter license"
[pli]: https://img.shields.io/crates/l/pointdexter.svg?style=for-the-badge "pointdexter license badge"
[p_logo]: ./assets/pointdexter.svg "pointdexter logo"
[pmi]: https://img.shields.io/badge/MSRV-1.85-f46623?style=for-the-badge&color=f46623&logo=rust "pointdexter MSRV badge"

[rci]: https://img.shields.io/crates/v/radium.svg?style=for-the-badge&color=f46623 "radium crate badge"
[rcu]: https://crates.io/crates/radium "radium crate"
[rdi]: https://img.shields.io/docsrs/radium/latest.svg?style=for-the-badge "radium documentation badge"
[rdu]: https://docs.rs/radium/latest/radium "radium documentation"
[rdli]: https://img.shields.io/crates/dv/radium.svg?style=for-the-badge "radium downloads"
[rlu]: https://github.com/ferrilab/ferrilab/blob/main/radium/LICENSE.txt "radium license"
[rli]: https://img.shields.io/crates/l/radium.svg?style=for-the-badge "radium license badge"
[r_logo]: ./assets/radium.svg "radium logo"
[rmi]: https://img.shields.io/badge/MSRV-1.60-f46623?style=for-the-badge&color=f46623&logo=rust "radium MSRV badge"
