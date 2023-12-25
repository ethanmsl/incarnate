# Justfile for HomeBrew hosted Rust Binary.

FILE := "incarnate"
_default:
        just --list

# Init & Update Git Submodules used in this Repo.
git-submodules:
        git submodule init
        git submodule update

# Build a release version for specific architectures.
build ARCH="aarch64-apple-darwin":
        cargo build --release --target {{ARCH}}
        tar czf target/{{ARCH}}/release/{{FILE}}-{{ARCH}}.tar.gz target/{{ARCH}}/release/{{FILE}}
        echo "Compiling for x86_64-apple-darwin from aarch64-apple-darwin appears broken, currently."


# List target architectures. (Should one want to expand build targets.)
list-archs:
        rustup target list

# Try to Bump the HomeBrew Version. URL pointing to hosted github (tar) file.
brew-bump URL SHA:
        brew bump-formula-pr {{FILE}} --url {{URL}} --sha256 {{SHA}}

# Get sha256 of release binary for a specific architecture.
sha ARCH="aarch64-apple-darwin":
        shasum --algorithm 256 target/{{ARCH}}/release/{{FILE}}-{{ARCH}}.tar.gz


# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# brew bump-formula-pr incarnate --url https://github.com/ethanmsl/incarnate/releases/download/v0.2.1/incarnate-aarch64-apple-darwin.tar.gz --sha256 64d6e78594721e8abc344895598f1d9f18d9ca89bfe10aa4bf0382d69afeb536 --version 0.2.1
#  --version {{VERSION}}
