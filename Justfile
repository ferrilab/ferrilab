########################################################################
#                               Justfile                               #
#                                                                      #
# Set of routines to execute for project development and management.   #
# Written against `just 1.8.0`.                                        #
########################################################################

default:
	just --list

build:
	just bitvec/build
	cargo build --no-default-features
	cargo build        --all-features

check:
	cargo clippy --no-default-features
	cargo clippy        --all-features

clean:
	cargo clean

cloc *ARGS:
	tokei -e 'guide/assets/*' {{ARGS}}

# Produces coverage reports for the test suite.
cover *ARGS:
	cargo +nightly tarpaulin --all-features -- {{ARGS}}

cover_docker *ARGS:
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin:0.22.0-slim cargo-tarpaulin tarpaulin -- {{ARGS}}

# Runs the development routines.
dev: check doc test
	echo miri cover | xargs -n1 -P2 just
	@echo "Complete at $(date)"

doc:
	cargo doc --all-features --document-private-items

format:
	cargo +nightly fmt

# Continually runs some recipe from this file.
loop +ACTIONS:
	watchexec -i target -- "just {{ACTIONS}}"

miri *ARGS: miri_install
	cargo +nightly miri test --lib --tests {{ARGS}}

# Installs Miri and ensures that it is able to run uninteractively.
miri_install:
	rustup toolchain add nightly --component miri
	cargo +nightly miri setup

package:
	cargo package --allow-dirty

# Spawns an HTTP file server to easily view compiled artifacts.
#
# The API documentation, user manual, and code coverage reports all produce HTML
# artifacts inside `target/`.
serve:
	miniserve . -p 7878 -n 0.0.0.0 -Drzq

test: check
	just bitvec/test
	cargo test -p funty
	cargo test -p radium
