.PHONY: clean build test lint format

clean:
	@cargo clean

build:
	@cargo build

test:
	@cargo test

lint:
	@cargo clippy -- -D warnings

format:
	@cargo fmt --check

format-fix:
	@cargo fmt
