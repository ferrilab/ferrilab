########################################################################
#                       Rust Development Recipes                       #
#                                                                      #
# Utilities for Rust development. Typically runs each command twice,   #
# once on the MSRV with no default features, and once on stable with   #
# default features. The recipes in this module run in the directory of #
# their caller, not necessarily in the project root.                   #
########################################################################

[no-cd]
default:
	just rust --list

# Runs the benchmark suite.
bench *ARGS: check
	cargo +nightly bench {{ARGS}}

# Builds the crate
[no-cd]
build:
	cargo         build --no-default-features
	cargo +stable build

# Runs Clippy
[no-cd]
check:
	cargo         clippy --no-default-features
	cargo +stable clippy

# Destroys all compiled artifacts
[no-cd]
clean:
	cargo clean

# Usual development routine: Clippy, documentation, test suite.
[no-cd]
dev: format check doc test

# Documents the crate
[no-cd]
doc:
	cargo         doc --no-default-features --document-private-items
	cargo +stable doc                       --document-private-items

[no-cd]
format:
	cargo +nightly fmt

# Checks that the crate is ready for publication
[no-cd]
package: check build doc test
	cargo         package --no-default-features --allow-dirty
	cargo +stable package                       --allow-dirty

# Publishes the crate to crates.io
[no-cd]
publish: package
	cargo publish

# Runs the test suite
[no-cd]
test:
	cargo         test --no-default-features -q
	cargo +stable test                       -q
