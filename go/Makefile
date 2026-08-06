.PHONY: clean build test lint lint-strict format generate tidy profile-mem profile-mem-ui

PROFILE_DIR ?= .profiles
BENCH_PKG ?= ./wdl
BENCH_PATTERN ?= .
BENCH_COUNT ?= 5

clean:
	@go clean ./...

build:
	@go build ./...

test:
	@go test ./...

lint:
	@go vet -unreachable=false ./wdl/...

format:
	@gofmt -w ./wdl/*.go

generate:
	@go generate ./...

tidy:
	@go mod tidy

profile-mem:
	@mkdir -p $(PROFILE_DIR)
	@go test $(BENCH_PKG) -run '^$$' -bench '$(BENCH_PATTERN)' -benchmem -count=$(BENCH_COUNT) -memprofile $(PROFILE_DIR)/mem.pprof | tee $(PROFILE_DIR)/benchmem.txt
	@go tool pprof -top $(PROFILE_DIR)/mem.pprof > $(PROFILE_DIR)/mem-top.txt
	@echo "Wrote benchmark+memory artifacts to $(PROFILE_DIR)/"

profile-mem-ui:
	@if [ ! -f $(PROFILE_DIR)/mem.pprof ]; then $(MAKE) profile-mem; fi
	@go tool pprof -http=:8080 $(PROFILE_DIR)/mem.pprof
