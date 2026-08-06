NPM ?= npm
PROFILE_DIR ?= .profiles

.PHONY: install clean build test package lint format generate-grammar profile-mem profile-mem-ui

install:
	$(NPM) install

clean:
	rm -rf dist
	rm -rf src/generated
	rm -rf .vitest

build:
	$(NPM) run build

test: build
	$(NPM) test

package:
	$(NPM) run build
	$(NPM) pack

lint:
	$(NPM) run lint

format:
	$(NPM) run format

generate-grammar:
	$(NPM) run generate:antlr

profile-mem:
	mkdir -p $(PROFILE_DIR)
	node --diagnostic-dir=$(PROFILE_DIR) --cpu-prof --cpu-prof-name=cpu.cpuprofile --heap-prof --heap-prof-name=heap.heapprofile ./node_modules/vitest/vitest.mjs bench --run
	@echo "Wrote TypeScript profiles to $(PROFILE_DIR)/"

profile-mem-ui:
	@if [ ! -f $(PROFILE_DIR)/cpu.cpuprofile ] || [ ! -f $(PROFILE_DIR)/heap.heapprofile ]; then $(MAKE) profile-mem; fi
	@npx --yes speedscope $(PROFILE_DIR)/cpu.cpuprofile
	@echo "CPU profile opened with speedscope."
	@echo "To inspect heap profile: Chrome DevTools -> Memory -> Load profile... -> $(PROFILE_DIR)/heap.heapprofile"
	@echo "Profiles directory: $(PROFILE_DIR)"
	@if command -v xdg-open >/dev/null 2>&1; then xdg-open $(PROFILE_DIR); \
	elif command -v open >/dev/null 2>&1; then open $(PROFILE_DIR); \
	elif command -v explorer.exe >/dev/null 2>&1; then explorer.exe $(PROFILE_DIR); \
	else echo "Open the profiles directory manually if needed."; fi
