.PHONY: help fmt format clippy test check build run clean all ci install

# Default target
help:
	@echo "Available targets:"
	@echo "  make fmt       - Format code with cargo fmt"
	@echo "  make clippy    - Run clippy lints"
	@echo "  make test      - Run tests"
	@echo "  make check     - Quick compile check"
	@echo "  make build     - Build in debug mode"
	@echo "  make release   - Build in release mode"
	@echo "  make run       - Run the application"
	@echo "  make install   - Install to system"
	@echo "  make ci        - Run all checks (fmt, clippy, test)"
	@echo "  make all       - Format, check, and build"
	@echo "  make clean     - Clean build artifacts"

# Format code
fmt format:
	cargo fmt --all

# Check formatting without modifying files
fmt-check:
	cargo fmt --all -- --check

# Run clippy with pedantic lints
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	cargo test

# Quick compilation check
check:
	cargo check

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run the application
run:
	cargo run

# Install to system
install:
	cargo install --path .

# CI pipeline - run all quality checks
ci: fmt-check clippy test
	@echo "✓ All CI checks passed!"

# Run format, clippy, and build
all: fmt clippy build
	@echo "✓ Build complete!"

# Clean build artifacts
clean:
	cargo clean
