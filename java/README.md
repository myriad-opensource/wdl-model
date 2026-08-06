# wdl-model Java Library

This module provides a Java model and analysis layer for Workflow Description Language (WDL).

This is a general-purpose library for building WDL tools. You can use it to read, analyze, transform, and validate WDL documents.

The Java library has its own test coverage for parsing, validation, processing, and fixture behavior.

## Using In Your Project

This library is published to GitHub Packages Maven registry.

Add the dependency:

```xml
<dependency>
	<groupId>com.myriad.wdl</groupId>
	<artifactId>wdl-model</artifactId>
	<version>0.0.1</version>
</dependency>
```

And add the GitHub Packages repository:

```xml
<repository>
	<id>github</id>
	<url>https://maven.pkg.github.com/myriad-opensource/wdl-model</url>
</repository>
```

## What This Library Does

This library helps you:

1. Parse WDL source into model objects (`WdlDocument`, tasks, workflows, types, statements, and expressions).
2. Resolve imports recursively through pluggable import resolvers.
3. Process and traverse the model through visitor-style processor interfaces and base classes.
4. Validate and lint documents with progressively stricter validators.

## Loading Files And Imports

`WdlV1Loader` is the entry point for parsing and import-aware document loading.

- Imports are resolved recursively and attached to the root `WdlDocument` through `importedDocuments`.
- Resolution is delegated to `WdlImportResolver` implementations.
- The filesystem resolver (`WdlImportResolverFilesystem`) supports local and relative paths and rejects network protocols (`http`, `https`) for deterministic local behavior.
- Custom resolvers can be used when alternative import sources are needed.

This separation keeps parsing independent from transport details while preserving reproducible test and local development behavior.

## Walking Documents (Processor Core)

The library includes processor APIs for walking documents in source order and building custom behavior on top of the model.

- `WdlProcessor` defines visitor-style callbacks for document, definition, section, and workflow statement nodes.
- `WdlProcessorBase` provides default traversal over documents, expressions, and types so consumers can override only the hooks they need.
- `WdlExpressionProcessor` and `WdlFunctionProcessor` support focused expression/function traversal workflows.
- `WdlAppendingProcessor` demonstrates rendering model objects back to WDL source and serves as a reference implementation for custom processors.

Processors are the core extension point in this library: traversal happens through processor implementations. The built-in validators use the same model-walking pattern, and custom validators/checkers can be built the same way.

## Validation Levels

Validation is layered so callers can choose strictness based on their use case.

1. `WdlValidator` (baseline semantic validation)
- Declaration assignability checks, including explicit `None` optionality behavior.
- Required/private call input rules.
- Invalid member/index access checks.
- Version-gated function availability checks.

2. `WdlStaticAnalysisValidator` (deterministic static analysis)
- Duplicate declarations and structural workflow issues.
- Unknown call targets and unknown type references.
- Function arity/signature mismatches.
- Operator/type compatibility checks.

3. `WdlLintingValidator` (usage and deprecation diagnostics)
- Unused workflow/task declarations.
- Unused scatter variables.
- Unreferenced call outputs.
- Deprecation warnings for language constructs that remain parseable but discouraged.

## Deprecation Warnings

`WdlLintingValidator` emits warning-severity diagnostics (`LINT_DEPRECATED_FEATURE`) for:

- deprecated `runtime` section usage
- deprecated `requirements` key `docker` (prefer `container`)
- deprecated `Object` type usage
- deprecated placeholder option forms
- deprecated `file://` import URI usage

## How Diagnostics Work

Diagnostics are represented by `WdlSemanticError` and include:

- stable code values
- severity (`ERROR` or `WARNING`)
- warning policy control

By default, warnings still throw `WdlException`. If you want warnings collected but non-throwing:

```java
WdlLintingValidator validator = new WdlLintingValidator().setThrowOnWarnings(false);
```

Errors always throw.

## Quick Example

```java
WdlDocument document = WdlV1Loader.load(source);

new WdlValidator().validate(document);
new WdlStaticAnalysisValidator().validate(document);
new WdlLintingValidator().setThrowOnWarnings(false).validate(document);
```

If you only need the parsed model, stop after `WdlV1Loader.load(...)`.

## Releasing

The release version comes from `pom.xml`.

For prerelease publishing, set a `-SNAPSHOT` version and run:

```bash
make publish-prerelease
```

This publishes the snapshot package to GitHub Packages and does not create a git tag or GitHub release.

For a full release, set a non-SNAPSHOT version and run:

```bash
make publish-release
```

This publishes the Maven package, creates an annotated git tag named `java-<version>`, and creates a GitHub Release for that tag.

Before publishing, load repo env vars with `direnv allow` (or `. ../.envrc` without direnv).

Note: Maven/Java + VPN certs can be tricky.
