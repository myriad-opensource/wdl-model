# wdl-model TypeScript Library

This module provides a TypeScript model and analysis layer for Workflow Description Language (WDL).

This is a general-purpose package for building WDL tools. You can use it to parse, inspect, validate, and transform WDL documents.

The TypeScript library has its own test coverage for parsing, validation, processing, and fixture behavior.

## What This Library Does

This library helps you:

1. Parse WDL source into model objects (documents, tasks, workflows, types, statements, and expressions).
2. Resolve imports recursively through pluggable import resolvers.
3. Process and traverse the model through visitor-style processor interfaces and base classes.
4. Validate and lint documents with progressively stricter validators.

## Loading Files And Imports

WdlV1Loader is the core parser/model builder and supports import-aware loading.

- Imports are resolved recursively and attached to the root document through imported-document mappings.
- Resolution is delegated to resolver implementations.
- Node loader defaults to filesystem import resolution for deterministic local behavior.
- Web loader does not resolve imports unless a resolver is explicitly provided.

This separation keeps parsing independent from transport details while preserving reproducible behavior across environments.

## Loader Options By Environment

The package exposes loader entry points for different execution environments:

1. WdlV1Loader

- environment-agnostic parsing from CharStream or source text

2. WdlV1NodeLoader

- Node-oriented file loading with default filesystem import resolution

3. WdlV1WebLoader

- browser/web loading helpers for File/Blob and Response inputs

## Walking Documents (Processor Core)

The package includes processor APIs for walking documents in source order and building custom behavior on top of the model.

- WdlProcessor defines visitor-style callbacks.
- WdlProcessorBase provides default traversal and import helper functionality.
- Expression/function processor helpers support focused analysis flows.
- Appending processor support provides a reference for source regeneration and custom transforms.

Processors are the main extension point in this package: traversal is performed by processor implementations. Built-in validators are layered on top of the same traversal approach, and custom validators/checkers can follow the same pattern.

## Validation Levels

Validation is layered so callers can choose strictness based on their use case.

1. WdlSemanticValidator (baseline semantic validation)

- Declaration assignability checks, including explicit None optionality behavior.
- Required/private call input rules.
- Invalid member/index access checks.
- Version-gated function availability checks.
- Import declaration and visibility checks.

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

By default, warnings still throw WdlException. If you want warnings collected but non-throwing:

```ts
new WdlLintingSemanticValidator().setThrowOnWarnings(false);
```

Errors always throw.

## Testing

The TypeScript package includes fixture suites for import resolution, import edge cases, static signature checks, operator semantics, type assignability, deprecation linting, and broad spec-example coverage.
