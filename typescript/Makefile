NPM ?= npm
PROFILE_DIR ?= .profiles
RELEASE_BRANCH ?= main

.PHONY: install clean build test package publish publish-prerelease publish-release lint format generate-grammar profile-mem profile-mem-ui

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

package: build
	@VERSION=$$(node -p "require('./package.json').version"); \
	if echo "$$VERSION" | grep -qi 'beta'; then \
		TAG=beta; \
	else \
		TAG=latest; \
	fi; \
	echo "Dry run for version $$VERSION with tag '$$TAG'..."; \
	$(NPM) publish --tag $$TAG --dry-run

publish: publish-prerelease

publish-prerelease: lint test
	@VERSION=$$(node -p "require('./package.json').version"); \
	if ! echo "$$VERSION" | grep -qi 'beta'; then \
		echo "ERROR: publish-prerelease expects a beta version, got $$VERSION"; \
		exit 1; \
	fi; \
	echo "Publishing npm package $$VERSION with tag beta..."; \
	$(NPM) publish --tag beta

publish-release: lint test
	@VERSION=$$(node -p "require('./package.json').version"); \
	if echo "$$VERSION" | grep -qi 'beta'; then \
		echo "ERROR: publish-release requires a non-beta version, got $$VERSION"; \
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
	TAG_NAME=typescript-$$VERSION; \
	if git rev-parse "$$TAG_NAME" >/dev/null 2>&1 || git ls-remote --tags origin "refs/tags/$$TAG_NAME" | grep -q .; then \
		echo "ERROR: tag $$TAG_NAME already exists. Bump version before publishing again."; \
		exit 1; \
	fi; \
	echo "Publishing npm package $$VERSION with tag latest..."; \
	$(NPM) publish --tag latest; \
	echo "Creating git tag $$TAG_NAME and GitHub release..."; \
	git tag -a "$$TAG_NAME" -m "$$TAG_NAME"; \
	git push origin "$$TAG_NAME"; \
	gh release create "$$TAG_NAME" --title "$$TAG_NAME" --generate-notes

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
