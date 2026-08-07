# Rust WDL Library — Implementation Plan

## Overview

Mirror the Java implementation in `rust/` as a single Cargo crate, using ANTLR4 (via the
`antlr-rust` runtime + a forked generator JAR) to build the same parser, model, traversal,
validator, and import resolution layers — translated to idiomatic Rust.

The Java implementation lives in `java/` and serves as the authoritative reference for
behavior, test coverage, and API shape. All tests are patterned directly on the Java test
suite and share the same fixture files (`wdl_tests/`, `wdl-grammar/spec_examples/`).

---

## Repository Placement

```
wdl-model/
├── java/                      # existing Java implementation
├── rust/                      # new Rust implementation
│   ├── Cargo.toml
│   ├── build.rs
│   ├── antlr4-rust-tool.jar   # forked ANTLR4 generator JAR (downloaded once)
│   ├── src/
│   └── tests/
├── wdl-grammar/               # shared grammar + spec examples (referenced by Rust tests)
└── wdl_tests/                 # shared test fixtures (referenced by Rust tests)
```

The root `Makefile` gains `rust` targets mirroring the existing `java` targets
(`build`, `test`, `format`, `clean`).

---

## Key Java → Rust Translation Decisions

| Java Pattern | Rust Idiom |
|---|---|
| Interface hierarchy (`WdlExpression` + subtypes) | `enum WdlExpression { Bool(WdlBoolLiteral), Int(WdlIntLiteral), ... }` |
| Abstract class + subclasses (`WdlType`) | `enum WdlType { Primitive(WdlPrimitiveType), Array(WdlArrayType), ... }` |
| Marker interfaces (`WdlNode`, `WdlDocumentElement`) | Marker traits; or enum variants on a `WdlDocumentElement` enum |
| Lombok `@Getter` / `@Setter` | Public fields or `pub fn` accessors; derive `Debug`, `Clone`, `PartialEq` |
| `ArrayDeque<WdlNode>` builder stack in loader | `Vec<WdlNode>` used as a stack |
| `LinkedHashMap<String, WdlDocument>` | `IndexMap<String, WdlDocument>` (via `indexmap` crate) |
| Checked exceptions (`WdlException`) | `Result<T, WdlError>` using `thiserror` |
| `WdlSemanticError.Code` enum (11 codes, stable) | Mirror as `WdlErrorCode` Rust enum with same 11 variants |
| `WdlProcessor` interface + `WdlProcessorBase` default traversal | `trait WdlProcessor` with default method bodies providing full traversal |
| Validator inheritance chain (3 levels) | Validators implement `WdlProcessor`; compose via delegation |
| Apache HttpClient5 for HTTP import resolution | `reqwest::blocking` behind `http-resolver` feature flag |
| ANTLR4 Visitor: `WdlV1ParserBaseVisitor<Void>` | ANTLR4 Rust visitor pattern via `antlr-rust` crate |

---

## `Cargo.toml` Dependencies

```toml
[dependencies]
antlr-rust = "0.3"
thiserror = "2"
indexmap = "2"        # insertion-ordered HashMap; mirrors Java LinkedHashMap
log = "0.4"
url = "2"

[dependencies.reqwest]
version = "0.12"
features = ["blocking"]
optional = true

[features]
default = []
http-resolver = ["dep:reqwest"]

[dev-dependencies]
rstest = "0.23"       # parameterized tests; mirrors JUnit @ParameterizedTest

[build-dependencies]
# build.rs invokes antlr4-rust-tool.jar via std::process::Command (requires Java at build time)
```

---

## Module Structure (`src/`)

```
src/
├── lib.rs
├── version.rs              # WdlVersion enum: V1_0, V1_1, V1_2, V1_3
├── document.rs             # WdlDocument struct (root node; holds imported docs)
├── loader.rs               # WdlV1Loader: ANTLR4 visitor → model
│
├── base/
│   └── mod.rs              # WdlNode marker trait; WdlKeyValue generic struct
│
├── types/
│   └── mod.rs              # WdlType enum (Primitive, Array, Map, Pair, TypeRef)
│                           # WdlPrimitiveKind enum (Boolean, Int, Float, String,
│                           #   File, Directory, Object)
│                           # isOptional flag on all type variants
│
├── expressions/
│   └── mod.rs              # WdlExpression enum (17+ ComponentType variants)
│                           # WdlStringLiteral struct + WdlStringComponent enum
│                           #   (Text, Escape, Placeholder, Token)
│                           # WdlBinaryOperator enum (arithmetic, comparison, logical)
│                           # WdlUnaryOperator enum
│                           # WdlMapEntry, WdlObjectEntry, WdlStructEntry, WdlPairLiteral
│
├── statements/
│   └── mod.rs              # WdlStatement enum
│                           #   (Declaration, BoundDeclaration, Import, Call,
│                           #    Scatter, Conditional)
│                           # WdlImport variants (Standard, Star, Members)
│                           # WdlCall struct (+ WdlCallInput)
│                           # WdlScatter struct
│                           # WdlConditional struct (+ WdlConditionalElseIf)
│
├── sections/
│   └── mod.rs              # WdlCommand struct (braced + multiline)
│                           # WdlInput, WdlOutput structs
│                           # WdlRequirements, WdlRuntime structs
│                           # WdlHints (task + workflow variants)
│                           # WdlMetadata, WdlParameterMetadata, WdlMetadataEntry
│
├── definitions/
│   └── mod.rs              # WdlTask struct (+ WdlTaskElement enum)
│                           # WdlWorkflow struct (+ WdlWorkflowElement enum)
│                           # WdlStruct struct (+ WdlStructMember, WdlStructElement)
│                           # WdlEnum struct
│
├── errors/
│   └── mod.rs              # WdlError (top-level thiserror enum)
│                           # WdlErrorCode enum (11 stable codes, 6 ERROR + 5 WARNING)
│                           # WdlSemanticError { code, severity, message, location }
│                           # WdlSyntaxError (wraps ANTLR parse/lex errors)
│                           # WdlImportError { uri, source }
│
├── processors/
│   └── mod.rs              # WdlProcessor trait (30+ callbacks, all with default bodies)
│                           # WdlExpressionProcessor trait + WdlExpressionProcessorBase
│                           # WdlFunctionProcessor trait + WdlFunctionProcessorBase
│                           # WdlAppendingProcessor struct (renders model → WDL text)
│                           # helper fns: expression_to_wdl, type_to_wdl,
│                           #             declaration_to_wdl, resolve_imported_*
│
├── resolvers/
│   └── mod.rs              # WdlImportResolver trait (URI scheme dispatch)
│                           # WdlFilesystemResolver (always compiled)
│                           # WdlHttpResolver (requires http-resolver feature)
│
└── validators/
    └── mod.rs              # WdlValidator (Level 1 — implements WdlProcessor)
                            # WdlStaticAnalysisValidator (Level 2 — wraps Level 1)
                            # WdlLintingValidator (Level 3 — wraps Level 2)
                            # WdlExpressionValidator
                            # WdlStaticAnalysisExpressionValidator
                            # WdlFunctionValidator
                            # WdlStaticAnalysisFunctionValidator
                            # set_throw_on_warnings(bool) option on validator
```

---

## `build.rs` — ANTLR4 Code Generation

The `build.rs` script:

1. Checks whether generated parser files in `src/grammar/` are already up to date.
2. Invokes the forked ANTLR4 generator JAR (`antlr4-rust-tool.jar`) via `java -jar`:
   ```
   java -jar antlr4-rust-tool.jar -Dlanguage=Rust \
       -visitor -listener \
       -o src/grammar/ \
       ../../wdl-grammar/antrl4/v1/WdlV1Lexer.g4 \
       ../../wdl-grammar/antrl4/v1/WdlV1Parser.g4
   ```
3. Emits `cargo:rerun-if-changed` directives for both `.g4` files and the JAR.

> **Note on the forked generator JAR:** The standard ANTLR4 4.13.2 tool does not emit Rust
> code. The `antlr-rust` crate (`antlr4rust` on GitHub) requires a fork of the ANTLR4
> generator maintained at `rrevenantt/antlr4` (`rust-target` branch). Pre-built JARs are
> available from the releases page of `rrevenantt/antlr4rust`. The JAR requires Java at
> build time, which is already a requirement of the Java implementation in this repo.
> The JAR will be committed to `rust/` or documented as a one-time download step.

---

## Test Structure (`tests/`)

Mirrors every Java test class 1:1. All fixture files are referenced via relative paths to
the shared `wdl_tests/` and `wdl-grammar/spec_examples/` directories at repo root.

```
tests/
├── common/
│   └── mod.rs                              # shared helpers: load_wdl_file, collect_spec_examples
│
├── v1/
│   ├── v11_spec_examples.rs               # parameterized over wdl-grammar/spec_examples/v1_1/**
│   ├── v12_spec_examples.rs               # parameterized over wdl-grammar/spec_examples/v1_2/**
│   └── v13_spec_examples.rs               # parameterized over wdl-grammar/spec_examples/v1_3/**
│
├── expressions/
│   ├── function_call_test.rs              # WdlFunctionCallOperationTest
│   └── function_test.rs                   # WdlFunctionTest
│
├── processors/
│   ├── appending_processor_test.rs        # WdlAppendingProcessorTest (round-trip)
│   ├── expression_processor_test.rs       # WdlExpressionProcessorBaseTest
│   ├── function_processor_test.rs         # WdlFunctionProcessorBaseTest
│   └── processor_import_resolution_test.rs # WdlProcessorBaseImportResolutionTest
│
├── resolvers/
│   ├── resolver_filesystem_test.rs        # WdlImportResolverFilesystemTest
│   └── resolver_test.rs                   # WdlImportResolverTest
│
├── loader_import_resolution_test.rs       # WdlV1LoaderImportResolutionTest
│
└── validators/
    ├── deprecation_test.rs                # WdlDeprecationValidationTest
    ├── expression_operator_semantics_test.rs # WdlExpressionOperatorSemanticsTest
    ├── function_version_matrix_test.rs    # WdlFunctionVersionMatrixTest
    ├── import_edge_cases_test.rs          # WdlImportEdgeCasesTest
    ├── import_validation_test.rs          # WdlImportValidationTest
    ├── non_runtime_completion_test.rs     # WdlNonRuntimeCompletionValidationTest
    ├── static_function_signature_matrix_test.rs # WdlStaticFunctionSignatureMatrixTest
    ├── type_assignability_matrix_test.rs  # WdlTypeAssignabilityMatrixTest
    └── validator_test.rs                  # WdlValidatorTest
```

### Test Patterns (Rust equivalents of Java patterns)

| Java | Rust |
|---|---|
| `@Test` | `#[test]` |
| `@ParameterizedTest` + `@MethodSource` | `#[rstest]` with `#[case(...)]` or `#[values(...)]` |
| `@DisplayName` | test function name (snake_case) |
| `assertDoesNotThrow(...)` | `assert!(result.is_ok())` |
| `assertThrows(WdlException.class, ...)` | `assert!(result.is_err())` |
| `assertEquals(expected, actual)` | `assert_eq!(expected, actual)` |
| `WdlTestResources.load(...)` | `common::load_wdl_file(path)` helper |
| JUnit Platform Suite | `mod` grouping + Cargo test filtering |
| `std::fs::read_dir` over spec examples | enumerate `.wdl` files in `wdl-grammar/spec_examples/v1_*/` |

---

## Implementation Phases

### Phase 1 — Infrastructure
- Create `rust/` directory with `Cargo.toml`, `build.rs`, module skeleton (`lib.rs` + empty `mod.rs` files)
- Download and commit `antlr4-rust-tool.jar`
- Verify ANTLR4 codegen produces compilable Rust from the WDL grammar files
- Add `rust` targets to root `Makefile`
- Establish `cargo fmt` + `cargo clippy` baseline

### Phase 2 — Model
- Implement all model types:
  - `base`: `WdlNode` marker trait, `WdlKeyValue`
  - `types`: `WdlType` enum and all type variants
  - `expressions`: `WdlExpression` enum and all expression types, string components, operators
  - `statements`: `WdlStatement` enum and all statement types
  - `sections`: all section structs
  - `definitions`: `WdlTask`, `WdlWorkflow`, `WdlStruct`, `WdlEnum`
  - `document`: `WdlDocument` root struct
  - `version`: `WdlVersion` enum
- All model types derive `Debug`, `Clone`, `PartialEq`

### Phase 3 — Loader
- Implement `WdlV1Loader` as an ANTLR4 visitor over the generated parse tree
- Use `Vec` as a build stack (mirrors Java `ArrayDeque`)
- Handle all `visitXxx` methods mirroring the Java `WdlV1Loader`
- Implement `load` entry points: `load_str`, `load_file`, `load_uri`
- Wire up import resolution during load

### Phase 4 — Processors
- Implement `WdlProcessor` trait with all ~30 callbacks and default traversal bodies
- Implement `WdlExpressionProcessor` and `WdlFunctionProcessor` sub-traits
- Implement `WdlAppendingProcessor` (renders model back to WDL text; used for round-trip tests)
- Implement helper functions: `expression_to_wdl`, `type_to_wdl`, `declaration_to_wdl`
- Implement `resolve_imported_tasks`, `resolve_imported_workflows`, etc.

### Phase 5 — Resolvers
- Define `WdlImportResolver` trait (URI scheme dispatch)
- Implement `WdlFilesystemResolver` (handles `file://` and bare paths; rejects `http://`/`https://`)
- Implement `WdlHttpResolver` (uses `reqwest::blocking`; behind `http-resolver` feature flag)
- Import cycle detection via URI cache (`IndexMap<String, WdlDocument>`)

### Phase 6 — Validators
- Implement `WdlFunctionValidator` (stdlib function arity/signature; version-gated availability)
- Implement `WdlExpressionValidator` (type checking for expressions)
- Implement `WdlValidator` Level 1 (baseline semantic checks; implements `WdlProcessor`)
- Implement `WdlStaticAnalysisExpressionValidator` and `WdlStaticAnalysisFunctionValidator`
- Implement `WdlStaticAnalysisValidator` Level 2 (deterministic static analysis)
- Implement `WdlLintingValidator` Level 3 (deprecation + usage diagnostics)
- Mirror the stable `WdlErrorCode` enum (11 codes) exactly

### Phase 7 — Tests
- Implement all test files mirroring the Java test classes
- Parameterize spec example tests over all 200+ `.wdl` files in `wdl-grammar/spec_examples/`
- Implement matrix tests: type assignability, function version matrix, function signature matrix,
  operator semantics, deprecation, import edge cases, import validation
- Verify round-trip fidelity via `WdlAppendingProcessor` tests
- All tests share fixture files via relative paths to `../../wdl_tests/` and
  `../../wdl-grammar/spec_examples/`

---

## Open Questions / Risks

### 1. `antlr4rust` Maturity
The `antlr-rust` crate is at `0.3.0-beta` and requires a **custom fork** of the ANTLR4
generator, not the official 4.13.2 release. If the generated Rust code from the WDL grammar
has issues (lifetime errors, unsupported grammar features), the grammar files may need
minor adaptations or the generated code may need manual patching. This is the primary
technical risk of the chosen approach.

**Mitigation:** Validate ANTLR4 codegen against the WDL grammar files early in Phase 1
before committing to deeper implementation work.

### 2. Ownership in the Loader
The `antlr-rust` runtime uses `Rc<>` heavily internally. The `WdlV1Loader` builds an
owned model from the parse tree, which means cloning or extracting data from ANTLR
contexts carefully. The Java loader uses a mutable stack with reference semantics
that doesn't translate directly.

**Mitigation:** Use `Vec<Box<dyn Any>>` or a typed enum stack in the loader to mirror the
Java `ArrayDeque<WdlNode>` pattern while remaining safe.

### 3. `WdlProcessor` Default Methods with Mutable State
Rust trait default methods that call other trait methods work fine, but validators accumulate
errors into a `Vec` (requiring `&mut self`). The trait callbacks must be `&mut self` throughout,
and blanket implementations need care to avoid borrow conflicts during traversal.

**Mitigation:** Design the traversal to pass `&mut self` to all callbacks from the start;
avoid holding references to model nodes across callback boundaries.

### 4. Feature Flag Hygiene
The `http-resolver` feature gates `reqwest`. Tests that exercise HTTP import resolution must
be `#[cfg(feature = "http-resolver")]` annotated to avoid CI failures when the feature is
not enabled.
