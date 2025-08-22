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

# Run benchmark
bench:
    cargo bench -p bench

# Publish package as dry-run
publish-try:
    cd ./package && cargo publish --allow-dirty --dry-run

# Publish package
publish:
    cd ./package && cargo publish

# Clean up
clean:
    cargo clean
