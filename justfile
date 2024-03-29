# Justfile for HomeBrew hosted Rust Binary.

# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula

FILE := "incarnate"
_default:
        just --list --unsorted

# Init & Update Git Submodules used in this Repo.
git-submodules:
        git submodule init
        git submodule update
        git submodule foreach git pull origin master

# Build a release version for specific architectures.
build ARCH="aarch64-apple-darwin":
        cargo clean
        cargo build --release --target {{ARCH}}
        cd ./target/{{ARCH}}/release && tar czf {{FILE}}-{{ARCH}}.tar.gz {{FILE}}
        shasum --algorithm 256 target/{{ARCH}}/release/{{FILE}}-{{ARCH}}.tar.gz
        echo "Compiling for x86_64-apple-darwin from aarch64-apple-darwin appears broken, currently."

# note: This compresses the whole directory stack down to the file:
# tar czf target/{{ARCH}}/release/{{FILE}}-{{ARCH}}.tar.gz target/{{ARCH}}/release/{{FILE}}

# List target architectures. (Should one want to expand build targets.)
list-archs:
        rustup target list

# Get sha256 of release binary for a specific architecture.
sha ARCH="aarch64-apple-darwin":
        shasum --algorithm 256 target/{{ARCH}}/release/{{FILE}}-{{ARCH}}.tar.gz

# Check brew file info and whether PR has been made.
brew-bump:
        brew info {{FILE}}
        brew bump {{FILE}}
        brew info {{FILE}}

# Open Releases Page, Ruby Formula Page, and local release directory.
open-pages ARCH="aarch64-apple-darwin":
        open ./target/{{ARCH}}/release
        open https://github.com/ethanmsl/{{FILE}}/releases
        open https://github.com/ethanmsl/homebrew-{{FILE}}

# run current build and then remove repo that was built.  (To see behavior and tracing results.)
_test_run DEBUG_LEVEL="DEBUG" REPO_NAME=("toremove"+uuid()):
        RUST_LOG={{DEBUG_LEVEL}} cargo run -- {{REPO_NAME}}
        rm -rf {{REPO_NAME}}


### Brew Commands for debugging.  ###
# NOTE: Changes to HomeBrew process mean I need to update some of these steps.

# provide info on the Homebrew formula
_brew-info:
        brew info {{FILE}}

# provide info on the Homebrew formula upload code
_brew-debug:
        brew install --debug --verbose {{FILE}}


# NOTE: these two commands (brew-bump-formula) have never worked...
# Try to Bump the HomeBrew Version. URL pointing to hosted github (tar) file.
_brew-bump-formula URL SHA:
        echo "NOTE: these have never worked.  We've always had to do it manually and wait a long time. :shrug:"
        brew bump-formula-pr {{FILE}} --url {{URL}} --sha256 {{SHA}}

# Try to Bump the HomeBrew Version. Give Version Number.
_brew-bump-formula-vrsn URL SHA VRSN:
        echo "NOTE: these have never worked.  We've always had to do it manually and wait a long time. :shrug:"
        brew bump-formula-pr {{FILE}} --url {{URL}} --sha256 {{SHA}} --version {{VRSN}}

