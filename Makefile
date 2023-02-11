.PHONY: format build test clippy test-ci

format:
	cargo fmt

f: format

build:
	cargo build

b: build

test-ci:
	make test

test:
	cargo test --all-features
	cargo test --all-features -- --ignored

t: test

clippy:
	cargo clippy --all-features

c: clippy
