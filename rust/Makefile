.PHONY: clean build test lint format format-check

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
