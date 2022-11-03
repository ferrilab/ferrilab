# For some reason, rustup does not appear to propagate correctly in Just's
# subshells, so we explicitly force the toolchain throughout this file.
TOOLCHAIN := "1.60.0"

default:
        just --list

# Runs the entire build sequence for all targets.
do_all: install build_all uninstall
        @true

# Installs every target not already installed.
install:
        rustup toolchain install {{TOOLCHAIN}}
        @# egrep exits with failure if no input line matches.
        rustup target list --toolchain {{TOOLCHAIN}} | egrep -v '(installed|fortanix)' >> new_targets.txt || true
        xargs rustup target add --toolchain {{TOOLCHAIN}} < new_targets.txt

# Builds the library once for every target.
build_all: install
        rustup target list --toolchain {{TOOLCHAIN}} | grep 'installed' | cut -f1 -d' ' | xargs -n1 -P1 just build_one

# Checks, builds, and documents one target.
build_one TARGET:
        # {{TARGET}}
        @cargo +{{TOOLCHAIN}} check --target {{TARGET}} || (echo Failed to check for {{TARGET}} && exit 255)
        @cargo +{{TOOLCHAIN}} build --target {{TARGET}} || (echo Failed to build for {{TARGET}} && exit 255)
        @cargo +{{TOOLCHAIN}} doc --target {{TARGET}} || (echo Failed to doc for {{TARGET}} && exit 255)

# Uninstalls only the targets added by `just install`.
uninstall:
        @touch new_targets.txt
        xargs rustup target remove --toolchain {{TOOLCHAIN}} < new_targets.txt
        @rm new_targets.txt
