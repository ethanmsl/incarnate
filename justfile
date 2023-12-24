

_default:
        just --list

# Init & Update Git Submodules used in this Repo.
git-submodules:
        git submodule init
        git submodule update

# Build a release version for specific architectures.
build:
        cargo build --release --target aarch64-apple-darwin
        echo "Compiling for x86_64-apple-darwin from aarch64-apple-darwin appears broken, currently."


# List target architectures. (Should one want to expand build targets.)
list-archs:
        rustup target list
