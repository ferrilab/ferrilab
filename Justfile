########################################################################
#                               Justfile                               #
#                                                                      #
# Set of routines to execute for project development and management.   #
# Written against `just 1.8.0`.                                        #
########################################################################

default:
	just --list

build:
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
cover *ARGS: test
	cargo +nightly tarpaulin --all-features -- {{ARGS}}
	@# just cover_docker
	@just cloc

cover_docker *ARGS:
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin:0.22.0-slim cargo-tarpaulin tarpaulin -- {{ARGS}}

doc:
	cargo doc --all-features --document-private-items

format:
	cargo +nightly fmt

miri:
	cargo +nightly miri test

package:
	cargo package --allow-dirty

test: check
	just bitvec/test
	cargo test -p funty
	cargo test -p radium
