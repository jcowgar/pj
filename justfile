version := `grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'`

default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test

check:
    cargo check

build:
    cargo build

release:
    cargo build --release

run:
    cargo run

install:
    cargo install --path .

ci: fmt-check clippy test

all: fmt clippy build

clean:
    cargo clean

package-deb: release _ensure-cargo-deb
    cargo deb --no-build
    @echo "Built: target/debian/pj_{{version}}-1_amd64.deb"

package-rpm: release _ensure-cargo-rpm
    cargo generate-rpm
    @echo "Built: target/generate-rpm/pj-{{version}}-1.x86_64.rpm"

package-all: package-deb package-rpm

_ensure-cargo-deb:
    @command -v cargo-deb >/dev/null 2>&1 || (echo "Installing cargo-deb..." && cargo install cargo-deb)

_ensure-cargo-rpm:
    @command -v cargo-generate-rpm >/dev/null 2>&1 || (echo "Installing cargo-generate-rpm..." && cargo install cargo-generate-rpm)
