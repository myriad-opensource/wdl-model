ANTLR_VERSION ?= 4.13.2
PROFILE_DIR ?= .profiles
PROFILE_ITERATIONS ?= 3000
PROFILE_WARMUP ?= 100

.PHONY: install lock clean-grammar clean build test package lint format check generate-grammar write-spec-examples profile-mem profile-mem-ui

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

profile-mem:
	mkdir -p $(PROFILE_DIR)
	poetry run python scripts/benchmark_profile.py --profile-dir $(PROFILE_DIR) --iterations $(PROFILE_ITERATIONS) --warmup $(PROFILE_WARMUP)

profile-mem-ui:
	@if [ ! -f $(PROFILE_DIR)/cpu.prof ]; then $(MAKE) profile-mem; fi
	@echo "Open interactive CPU profile with: poetry run python -m pstats $(PROFILE_DIR)/cpu.prof"
	@echo "Top summary is available at: $(PROFILE_DIR)/cpu-top.txt"
	@echo "Benchmark summary is available at: $(PROFILE_DIR)/bench.txt"
	@if command -v xdg-open >/dev/null 2>&1; then xdg-open $(PROFILE_DIR); \
	elif command -v open >/dev/null 2>&1; then open $(PROFILE_DIR); \
	elif command -v explorer.exe >/dev/null 2>&1; then explorer.exe $(PROFILE_DIR); \
	else echo "Open the profiles directory manually if needed."; fi
