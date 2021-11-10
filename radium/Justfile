default:
        just --list

# Installs every target not already installed.
install:
        rustup target list | grep -v installed > new_targets.txt
        xargs rustup target add < new_targets.txt

# Builds the library once for every target.
build_all:
        rustup target list | cut -f1 -d' ' | xargs -n1 -P1 cargo build --target

# Uninstalls only the targets added by `just install`.
uninstall:
        touch new_targets.txt
        xargs rustup target remove < new_targets.txt
        rm new_targets.txt
