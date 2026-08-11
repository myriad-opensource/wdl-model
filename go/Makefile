.PHONY: clean build test lint lint-strict format generate tidy publish-release profile-mem profile-mem-ui

PROFILE_DIR ?= .profiles
BENCH_PKG ?= ./wdl
BENCH_PATTERN ?= .
BENCH_COUNT ?= 5
RELEASE_BRANCH ?= main
VERSION ?= $(shell tr -d '[:space:]' < version 2>/dev/null)

clean:
	@go clean ./...
	@find ./grammar -name '*.go' ! -name 'package.go' -delete

build: generate
	@go build ./...

test: generate
	@go test ./...

lint:
	@go vet -unreachable=false ./wdl/...

format:
	@gofmt -w ./wdl/*.go

generate:
	@go generate ./...

tidy:
	@go mod tidy

publish-release: lint test
	@VERSION="$(VERSION)"; \
	if [ -z "$$VERSION" ]; then \
		echo "ERROR: set version to a value like v0.1.0"; \
		echo "You can also override on the command line: make publish-release VERSION=v0.1.0"; \
		exit 1; \
	fi; \
	if ! echo "$$VERSION" | grep -Eq '^v[0-9]+\.[0-9]+\.[0-9]+'; then \
		echo "ERROR: VERSION must look like vX.Y.Z (example: v0.1.0), got $$VERSION"; \
		exit 1; \
	fi; \
	BRANCH=$$(git rev-parse --abbrev-ref HEAD); \
	if [ "$$BRANCH" != "$(RELEASE_BRANCH)" ]; then \
		echo "ERROR: releases must be cut from '$(RELEASE_BRANCH)'; current branch is '$$BRANCH'"; \
		exit 1; \
	fi; \
	if ! command -v gh >/dev/null 2>&1; then \
		echo "ERROR: GitHub CLI (gh) is required to cut a release."; \
		exit 1; \
	fi; \
	TAG_NAME=go-$$VERSION; \
	if git rev-parse "$$TAG_NAME" >/dev/null 2>&1 || git ls-remote --tags origin "refs/tags/$$TAG_NAME" | grep -q .; then \
		echo "ERROR: tag $$TAG_NAME already exists."; \
		exit 1; \
	fi; \
	echo "Creating annotated tag $$TAG_NAME and GitHub release..."; \
	git tag -a "$$TAG_NAME" -m "$$TAG_NAME"; \
	git push origin "$$TAG_NAME"; \
	gh release create "$$TAG_NAME" --title "$$TAG_NAME" --generate-notes

profile-mem:
	@mkdir -p $(PROFILE_DIR)
	@go test $(BENCH_PKG) -run '^$$' -bench '$(BENCH_PATTERN)' -benchmem -count=$(BENCH_COUNT) -memprofile $(PROFILE_DIR)/mem.pprof | tee $(PROFILE_DIR)/benchmem.txt
	@go tool pprof -top $(PROFILE_DIR)/mem.pprof > $(PROFILE_DIR)/mem-top.txt
	@echo "Wrote benchmark+memory artifacts to $(PROFILE_DIR)/"

profile-mem-ui:
	@if [ ! -f $(PROFILE_DIR)/mem.pprof ]; then $(MAKE) profile-mem; fi
	@go tool pprof -http=:8080 $(PROFILE_DIR)/mem.pprof
