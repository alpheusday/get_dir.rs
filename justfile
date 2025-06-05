set shell := ["bash", "-cu"]
set windows-shell := ["powershell"]

# Default action
_:
    just lint
    just fmt
    just test

# Setup the project
setup:
    brew install ls-lint typos-cli

# Lint code
lint:
    ls-lint
    typos
    cargo check
    cargo clippy
    cargo test -p get_dir --features all -- --nocapture

# Format code
fmt:
    cargo fmt

# Run test
test:
    cargo test -p tests -- --nocapture

# Clean up
clean:
    cargo clean
