.PHONY: clean build test lint format

clean:
	@cargo clean

build:
	@cargo build

test:
	@cargo test

lint:
	@cargo clippy --all-targets -- -D warnings

format-check:
	@cargo fmt --check

format:
	@cargo fmt
