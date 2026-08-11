# wdl-model Python Library

This module provides a Python model and analysis layer for Workflow Description Language (WDL).

This is a general-purpose package for building WDL tools. You can use it to parse, inspect, validate, and transform WDL documents.

The Python library has its own test coverage for parsing, validation, processing, and fixture behavior.

## Using In Your Project

Use the git tag and `python` subdirectory in your dependency declaration.

`pyproject.toml` example:

```toml
[project]
dependencies = [
	"wdl-model @ git+https://github.com/myriad-opensource/wdl-model.git@python-0.0.1b0#subdirectory=python"
]
```

Poetry-style example:

```toml
[tool.poetry.dependencies]
wdl-model = { git = "https://github.com/myriad-opensource/wdl-model.git", tag = "python-0.0.1b0", subdirectory = "python" }
```

Exact syntax depends on your package manager, but the source, tag, and subdirectory are the same.
You can also point to a branch name or commit hash instead of a release tag.

## What This Library Does

This library helps you:

1. Parse WDL source into model objects (documents, tasks, workflows, types, statements, and expressions).
2. Resolve imports recursively through pluggable import resolvers.
3. Process and traverse the model through visitor-style processor interfaces and base classes.
4. Validate and lint documents with progressively stricter validators.

## Generated Parser Code

Generated ANTLR parser code is isolated in:

- `src/wdl_model/grammar/v1`

Generated parser `.py` artifacts in that package are committed in this repo.
This is required so git-based dependency consumers can build/import without
running generation steps.

For maintainers:

- Regenerate grammar bindings with `make generate-grammar`.
- Use `make clean-grammar` only when you explicitly want to remove generated grammar artifacts.
- `make publish-release` verifies `python/src/wdl_model/grammar/v1` is clean after generation and fails if generated files are out of sync or untracked.

## Loading Files And Imports

WDL source is loaded through the v1 loader APIs.

- Imports are resolved recursively and attached to the root document through imported-document mappings.
- Resolution is delegated to import resolver implementations.
- The filesystem resolver supports local and relative paths and rejects network protocols for deterministic local behavior.
- Custom resolvers can be supplied when alternate import sources are needed.

This keeps parsing independent from transport details while preserving reproducible local workflows.

## Walking Documents (Processor Core)

The package includes processor APIs for walking documents in source order and building custom behavior on top of the model.

- Base processor types provide default traversal over documents, expressions, and types.
- Function and expression processor helpers support focused analysis flows.
- Appending/rendering processor support provides a reference for source regeneration and custom transforms.

Processors are the main extension point in this package: traversal is performed by processor implementations. Built-in validators are layered on top of the same traversal approach, and custom validators/checkers can follow the same pattern.

## Validation Levels

Validation is layered so callers can choose strictness based on their use case.

1. WdlSemanticValidator (baseline semantic validation)
- Declaration assignability checks, including explicit None optionality behavior.
- Required/private call input rules.
- Invalid member/index access checks.
- Version-gated function availability checks.

2. WdlStaticAnalysisSemanticValidator (deterministic static analysis)
- Duplicate declarations and structural workflow issues.
- Unknown call targets and unknown type references.
- Function arity/signature mismatches.
- Operator/type compatibility checks.

3. WdlLintingSemanticValidator (usage and deprecation diagnostics)
- Unused workflow/task declarations.
- Unused scatter variables.
- Unreferenced call outputs.
- Deprecation warnings for parseable but discouraged constructs.

## How Diagnostics Work

Diagnostics are represented by WdlSemanticError and include:

- stable code values
- severity (ERROR or WARNING)
- warning policy control

By default, warnings still raise WdlException. If you want warnings collected but non-throwing:

```python
validator = WdlLintingSemanticValidator().setThrowOnWarnings(False)
```

Errors always raise.

## Quick Example

```python
document = WdlV1Loader.load_from_string(source)

WdlSemanticValidator().validateDocument(document)
WdlStaticAnalysisSemanticValidator().validateDocument(document)
WdlLintingSemanticValidator().setThrowOnWarnings(False).validateDocument(document)
```

If you only need the parsed model, stop after loading.

## Releasing

Set the version in `pyproject.toml`, then run:

```bash
make publish-release
```

What this does:

- Reads the version from `pyproject.toml` using `poetry version -s`.
- Creates an annotated git tag named `python-<version>`.
- Pushes the tag to origin.
- Creates a GitHub Release for that tag.

The target also checks that:

- You are on the release branch (default `main`).
- The tag does not already exist.
- `gh` CLI is installed.
- Generated grammar files are up to date and committed under `src/wdl_model/grammar/v1`.
