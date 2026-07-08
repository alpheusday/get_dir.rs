set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

# Default action
_:
    just --list -u

# Format code
fmt:
    cargo fmt

# Lint code with ls-lint
ls-lint:
    ls-lint -config ./.ls-lint.yaml

# Lint code with ls-lint
lslint: ls-lint

# Lint code with typos-cli
typos:
    typos

# Lint code
lint:
    cargo clippy
    cargo test -p get_dir --features all -- --nocapture

# Run test
test:
    cargo test -p tests -- --nocapture

# Check code
check: fmt ls-lint typos lint test

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
