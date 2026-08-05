ANTLR_VERSION ?= 4.13.2

.PHONY: install lock clean-grammar clean build test package lint format check generate-grammar write-spec-examples

install:
	poetry install

lock:
	poetry lock

clean:
	rm -rf .pytest_cache .ruff_cache dist build src/wdl_model/grammar/v1
	find . -type d -name __pycache__ -prune -exec rm -rf {} +

build: generate-grammar

test:
	poetry run pytest -q

package: build
	poetry build

lint:
	poetry run ruff check src tests

format:
	poetry run ruff format src tests

generate-grammar:
	mkdir -p src/wdl_model/grammar/v1
	poetry run antlr4 \
		-v "$(ANTLR_VERSION)" \
		-Dlanguage=Python3 \
		-visitor \
		-no-listener \
		-Xexact-output-dir \
		-o src/wdl_model/grammar/v1 \
		antlr4/v1/WdlV1Lexer.g4
	poetry run antlr4 \
		-v "$(ANTLR_VERSION)" \
		-Dlanguage=Python3 \
		-visitor \
		-no-listener \
		-Xexact-output-dir \
		-lib src/wdl_model/grammar/v1 \
		-o src/wdl_model/grammar/v1 \
		antlr4/v1/WdlV1Parser.g4
