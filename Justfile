########################################################################
#                               Justfile                               #
#                                                                      #
# Set of routines to execute for project development and management.   #
# Written against `just 1.46.0`.                                       #
########################################################################

# Utility recipes for individual-crate development.
mod rust

default:
	just --list

book: book_install
	mdbook build guide

@book_install:
	cargo +nightly install mdbook --vers ^0.4
	cargo +nightly install mdbook-admonish --vers ^1.20
	cargo +nightly install mdbook-mermaid --vers ^0.16

book_serve: book_install
	mdbook serve guide

cloc *ARGS: util_install
	tokei -e 'assets/'  {{ARGS}}

# Produces coverage reports for the test suite.
cover *ARGS: util_install
	cargo +nightly tarpaulin --all-features -- {{ARGS}}

cover_docker *ARGS:
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin:0.35.1-slim cargo-tarpaulin tarpaulin -- {{ARGS}}

# Continually runs some recipe from this file.
[no-cd]
loop +ACTIONS: util_install
	watchexec -i target -- "just {{ACTIONS}}"

miri *ARGS: miri_install
	cargo +nightly miri test --lib --tests {{ARGS}}

# Installs Miri and ensures that it is able to run without interaction.
miri_install:
	rustup toolchain add nightly --component miri
	cargo +nightly miri setup

# The API documentation, user manual, and code coverage reports all produce HTML
# artifacts inside `target/`.
#
# Spawns an HTTP file server to easily view compiled artifacts.
serve: util_install
	miniserve . -p 7878 -i 0.0.0.0 -D -rzq

[unix]
@util_install:
	which cargo-tarpaulin >/dev/null || cargo +nightly install cargo-tarpaulin --vers ^0.35
	which miniserve       >/dev/null || cargo +nightly install miniserve
	which watchexec       >/dev/null || cargo +nightly install watchexec
	which tokei           >/dev/null || cargo +nightly install tokei
