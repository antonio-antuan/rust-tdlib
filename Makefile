build:
	cargo build

build-no-defaults:
	cargo build --no-default-features

format:
	cargo fmt

format-check:
	cargo fmt --check

lint:
	cargo clippy --tests --examples --all-features

test:
	cargo test --all-features --bins --tests --all-targets

check: build format lint test