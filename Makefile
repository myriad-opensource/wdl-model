# -----------------------------------------------------------------------------
# Shared fixtures
# -----------------------------------------------------------------------------

.PHONY: spec_examples

spec_examples:
	@(cd wdl-grammar && python3 specs/extract_spec_examples.py)

# -----------------------------------------------------------------------------
# Aggregated workflows
# -----------------------------------------------------------------------------

.PHONY: build test package lint format clean

build: spec_examples build-go build-java build-python build-typescript build-rust

test: spec_examples test-go test-java test-python test-typescript test-rust

package: spec_examples package-go package-java package-python package-typescript

lint: lint-go lint-java lint-python lint-typescript lint-rust

format: format-go format-java format-python format-typescript format-rust

clean: clean-go clean-java clean-python clean-typescript clean-rust

# -----------------------------------------------------------------------------
# Rust
# -----------------------------------------------------------------------------

.PHONY: build-rust test-rust lint-rust format-rust clean-rust

build-rust: spec_examples
	@(cd rust && $(MAKE) build)

test-rust: spec_examples
	@(cd rust && $(MAKE) test)

lint-rust:
	@(cd rust && $(MAKE) lint)

format-rust:
	@(cd rust && $(MAKE) format)

clean-rust:
	@(cd rust && $(MAKE) clean)

# -----------------------------------------------------------------------------
# Go
# -----------------------------------------------------------------------------

.PHONY: build-go test-go package-go lint-go format-go clean-go

build-go: spec_examples
	@(cd go && $(MAKE) generate)
	@(cd go && $(MAKE) build)

test-go: spec_examples
	@(cd go && $(MAKE) test)

package-go: spec_examples
	@(cd go && $(MAKE) build)

lint-go:
	@(cd go && $(MAKE) lint)

format-go:
	@(cd go && $(MAKE) format)

clean-go:
	@(cd go && $(MAKE) clean)

# -----------------------------------------------------------------------------
# Java
# -----------------------------------------------------------------------------

.PHONY: build-java test-java package-java lint-java format-java clean-java

build-java: spec_examples
	@(cd java && $(MAKE) build)

test-java: spec_examples
	@(cd java && $(MAKE) test)

package-java: spec_examples
	@(cd java && $(MAKE) package)

lint-java:
	@(cd java && $(MAKE) lint)

format-java:
	@(cd java && $(MAKE) format)

clean-java:
	@(cd java && $(MAKE) clean)

# -----------------------------------------------------------------------------
# Python
# -----------------------------------------------------------------------------

.PHONY: build-python test-python package-python lint-python format-python clean-python

build-python: spec_examples
	@(cd python && $(MAKE) install)
	@(cd python && $(MAKE) build)

test-python: spec_examples
	@(cd python && $(MAKE) test)

package-python: spec_examples
	@(cd python && $(MAKE) package)

lint-python:
	@(cd python && $(MAKE) lint)

format-python:
	@(cd python && $(MAKE) format)

clean-python:
	@(cd python && $(MAKE) clean)

# -----------------------------------------------------------------------------
# TypeScript
# -----------------------------------------------------------------------------

.PHONY: build-typescript test-typescript package-typescript lint-typescript format-typescript clean-typescript

build-typescript: spec_examples
	@(cd typescript && $(MAKE) install)
	@(cd typescript && $(MAKE) build)

test-typescript: spec_examples
	@(cd typescript && $(MAKE) test)

package-typescript: spec_examples
	@(cd typescript && $(MAKE) package)

lint-typescript:
	@(cd typescript && $(MAKE) lint)

format-typescript:
	@(cd typescript && $(MAKE) format)

clean-typescript:
	@(cd typescript && $(MAKE) clean)
