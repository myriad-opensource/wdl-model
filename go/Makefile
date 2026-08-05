.PHONY: clean build test lint lint-strict format generate tidy

clean:
	@go clean ./...

build:
	@go build ./...

test:
	@go test ./...

lint:
	@go vet -unreachable=false ./wdl/...

lint-strict:
	@golangci-lint run ./...

format:
	@gofmt -w ./wdl/*.go

generate:
	@go generate ./...

tidy:
	@go mod tidy
