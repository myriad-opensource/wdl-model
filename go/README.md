# wdl-model Go Library

This module provides a Go model and analysis layer for Workflow Description Language (WDL).

This is a general-purpose package for building WDL tools. You can use it to parse, inspect, validate, and transform WDL documents.

## What This Library Does

This library helps you:

1. Parse WDL source into model objects (documents, tasks, workflows, import metadata, and declarations).
2. Resolve imports recursively through pluggable import resolvers.
3. Process and traverse loaded documents with callback-oriented processor APIs.
4. Validate and lint documents with progressively stricter validators.

## Name And Import Path

The module path uses `wdl-model` to match repository naming:

- `github.com/myriad-opensource/wdl-model/go`

Go package identifiers cannot contain `-`, so the public package is intentionally named `wdl`.

## Using In Your Project

Add the module to your project with a pinned version:

```bash
go get github.com/myriad-opensource/wdl-model/go@v0.1.0
```

Then import it in your code:

```go
import "github.com/myriad-opensource/wdl-model/go/wdl"
```

## Generated Parser Code

Generated ANTLR parser code is isolated in a dedicated package:

- `grammar/wdl1`

Generated parser `.go` artifacts in that package are committed in this repo.
This is required so module consumers (`go get`) can compile without running
`go generate` on dependency modules.

For maintainers:

- Regenerate grammar bindings with `make generate`.
- Use `make clean-grammar` only when you explicitly want to remove generated grammar artifacts.
- `make publish-release` now verifies `go/grammar/wdl1` is clean after generation and fails if generated files are out of sync or untracked.

Handwritten library APIs live in:

- `wdl`

This follows the same generated-code split used in the Java, Python, and TypeScript libraries.

## Loading Files And Imports

`wdl.Loader` is the entry point for parsing and import-aware document loading.

- Imports are resolved recursively and attached to the root document through `ImportedDocs` and `ImportStatements`.
- Resolution is delegated to `wdl.Resolver` implementations.
- `wdl.NewDefaultResolver(...)` supports local file and HTTP(S) imports with configurable TLS behavior.

## Walking Documents (Processor Core)

The package includes processor APIs for walking a document and imported subdocuments in traversal order.

- `wdl.Processor` defines callbacks.
- `wdl.ProcessorBase` provides no-op defaults.
- `wdl.TraverseDocument(...)` performs recursive traversal.

Processors are the main extension point in this package: traversal is performed by processor implementations. Built-in validators are layered on top of the same traversal approach, and custom validators/checkers can follow the same pattern.

## Validation Levels

Validation is layered so callers can choose strictness based on their use case.

1. `wdl.SemanticValidator` (baseline semantic validation)
- Duplicate top-level definition detection.
- Structural rule requiring exactly one workflow.
- Import alias conflict checks.

2. `wdl.StaticValidator` (deterministic static analysis)
- Currently delegates to semantic validation and is the extension point for stricter static checks.

3. `wdl.LintingValidator` (usage and deprecation diagnostics)
- Unused import aliases/symbol diagnostics.
- Deprecation warnings such as `file://` import URI usage.

## Test Coverage

The Go module runs fixture-based tests from repository suites, including:

- `function_version_matrix`
- `static_function_signature_matrix`
- `type_assignability_matrix`
- `expression_operator_semantics`
- `import_validation`
- `import_edge_cases`
- `validator`
- `deprecations`
- `loader_imports`
- `resolver_filesystem`
- `processor_imports`

## How Diagnostics Work

Diagnostics are represented by semantic/syntax error types and include:

- stable code values
- severity (`ERROR` or `WARNING`)
- warning policy control

By default, warning-only lint diagnostics do not fail validation. Set `ThrowOnWarnings: true` to escalate warnings into returned errors.

## Quick Example

```go
resolver, _ := wdl.NewDefaultResolver(wdl.ResolverConfig{})
loader := wdl.NewWdlV1Loader()

doc, err := loader.LoadFile(ctx, "workflow.wdl", wdl.WithResolver(resolver))
if err != nil {
    // handle parse/load diagnostics
}

validator := wdl.NewLintingValidator(wdl.SemanticValidatorConfig{ThrowOnWarnings: false})
if err := validator.Validate(ctx, doc); err != nil {
    // handle validation diagnostics
}
```

## Releasing

Set the release version in `version` (example `v0.1.0`), then run:

Note: Go modules are versioned by git tags, not prerelease package channels. The `version` file is the next tag to publish.

```bash
make publish-release
```

What this does:

- Creates an annotated git tag named `go-<version>` (example: `go-v0.1.0`).
- Pushes the tag to origin.
- Creates a GitHub Release for that tag.

The target also checks that:

- You are on the release branch (default `main`).
- The tag does not already exist.
- `gh` CLI is installed.

Tip: you can still override the version manually when needed:

```bash
make publish-release VERSION=v0.1.0
```
