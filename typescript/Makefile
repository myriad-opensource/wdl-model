NPM ?= npm

.PHONY: install clean build test package lint format generate-grammar

install:
	$(NPM) install

clean:
	rm -rf dist
	rm -rf src/generated
	rm -rf .vitest

build:
	$(NPM) run build

test:
	$(NPM) test

package:
	echo FIXME

lint:
	$(NPM) run lint

format:
	$(NPM) run format

generate-grammar:
	$(NPM) run generate:antlr
