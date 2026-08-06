# WDL Model Libraries

This repository hosts WDL model libraries for multiple languages.

At a high level, each library helps you do three things:

1. Parse WDL source into a structured model.
2. Validate WDL with semantic, static-analysis, and lint-style checks.
3. Traverse documents with processor/visitor APIs (processors are what perform traversal) to build custom tooling.

Across languages, processor APIs are the main extension point; built-in validators are layered on top of the same traversal approach, and custom checks can be implemented the same way.

The goal of this repo is to provide practical, production-usable building blocks for WDL tooling across ecosystems.

## Language And Ecosystem Libraries

- [Go](go/README.md)
- [Java](java/README.md)
- [Python](python/README.md)
- [TypeScript](typescript/README.md)

## Shared Project Assets

- `wdl-grammar/`: grammar sources, spec examples, and spec support files.
- Root `Makefile`: cross-language build, test, format, and clean workflows.

