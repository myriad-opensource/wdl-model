# TODO — Java

Items found during Rust parity work. **No Java files were edited** — those
sessions were scoped to `rust/` only. Each part records the diagnosis and the
exact proposed change so someone with Java scope can apply it.

| Part | Subject | Task |
|---|---|---|
| [1](#part-1--build-environment) | `mvn` fails when the ambient `JAVA_HOME` is not JDK 11+ | E1 |
| [2](#part-2--validator-correctness) | A test blind spot and four validator defects | B4 |

---

# Part 1 — Build environment

## `mvn` fails out of the box when the ambient `JAVA_HOME` is not JDK 11+

Tracked as **Task E1** in `rust/.context/updated_plan_20260928.md`.
Written up 2026-10-01.

---

## 1. Problem

Maven honors `JAVA_HOME`, not `PATH`. On a machine where the two disagree, Maven
silently runs on the wrong JDK.

| | Value |
|---|---|
| `JAVA_HOME` (exported in shell) | `~/.asdf/installs/java/zulu-javafx-8.86.0.25` → **Java 1.8.0_452** |
| `which java` (asdf shim, per `.tool-versions`) | `temurin-11.0.27+6` → **Java 11** |
| `.tool-versions` (repo root) | `java temurin-11.0.27+6` |

> **The JDK 8 `JAVA_HOME` is deliberate.** It serves a separate, older project on
> the same machine. It is not stale, and it is not a misconfiguration to be
> corrected.
>
> This is the single most important constraint on the fix: **"just point
> `JAVA_HOME` at JDK 11" is not an available remedy**, because it would break
> that other project. The build has to stop depending on the ambient value
> instead. That rules out a documentation-only fix and makes Edit 3 the real
> solution rather than a convenience.

So the shell reports Java 11 while Maven uses Java 8. `antlr4-maven-plugin:4.13.2`
is compiled for Java 11 (class file version 55), and dies during
`generate-sources` — before a single line of project code compiles:

```
$ mvn -B -s .mvn/settings.xml generate-sources

java.lang.UnsupportedClassVersionError: org/antlr/v4/Tool has been compiled by a
more recent version of the Java Runtime (class file version 55.0), this version
of the Java Runtime only recognizes class file versions up to 52.0
[INFO] BUILD FAILURE
[ERROR] Failed to execute goal org.antlr:antlr4-maven-plugin:4.13.2:antlr4
        (generate-parser) on project wdl-model: ...
```

Class file 55 = Java 11; 52 = Java 8.

**The error points at ANTLR, which is not the problem.** A developer whose
ambient `JAVA_HOME` targets a pre-11 JDK — a normal, legitimate state on a
machine serving several projects — hits this with no obvious path to the cause.

### Current workaround

Per-invocation only. It must **not** be made global, for the reason in the
callout above:

```sh
JAVA_HOME=/Users/bvaisvil/.asdf/installs/java/temurin-11.0.27+6 \
  mvn -B -s .mvn/settings.xml test
```

Verified: `BUILD SUCCESS`.

### Why `pom.xml` does not already catch this

`java/pom.xml:55-57` already sets:

```xml
<maven.compiler.release>11</maven.compiler.release>
<maven.compiler.source>11</maven.compiler.source>
<maven.compiler.target>11</maven.compiler.target>
```

Those govern the *bytecode Maven emits*. They say nothing about which JVM Maven
itself is running on, so they cannot detect or prevent this.

---

## 2. Proposed changes

Three edits across two files. **Edit 3 is the one that actually fixes the
failure; Edits 1–2 only turn the error message into something legible.** If you
only do one, do Edit 3 — it is also the lower-risk of the two changes, and it is
the only one that works without asking developers to change a `JAVA_HOME` they
may need for other projects.

### Edit 1 — `java/pom.xml`: add the version property

Anchor: `<properties>` block, after line 62 (`maven-surefire.version`).

```diff
 		<maven-compiler.version>3.15.0</maven-compiler.version>
 		<maven-source.version>3.4.0</maven-source.version>
 		<maven-javadoc.version>3.12.0</maven-javadoc.version>
 		<maven-surefire.version>3.5.6</maven-surefire.version>
+		<maven-enforcer.version>3.1.0</maven-enforcer.version>
 		<spotless.version>2.44.5</spotless.version>
 		<google-java-format.version>1.24.0</google-java-format.version>
```

See §3 for why `3.1.0` and not the newest release.

### Edit 2 — `java/pom.xml`: add the enforcer plugin

Anchor: first entry in `<build><plugins>`, line 127 — **before** the
`antlr4-maven-plugin` block.

```diff
 	<build>
 		<plugins>
+			<plugin>
+				<groupId>org.apache.maven.plugins</groupId>
+				<artifactId>maven-enforcer-plugin</artifactId>
+				<version>${maven-enforcer.version}</version>
+				<executions>
+					<execution>
+						<id>enforce-java-version</id>
+						<!-- The `enforce` goal binds to `validate` by default,
+						     which precedes `generate-sources`. This must fail
+						     before antlr4-maven-plugin tries to load
+						     org.antlr.v4.Tool, or the legible message never
+						     appears. Do not add an explicit <phase>. -->
+						<goals>
+							<goal>enforce</goal>
+						</goals>
+						<configuration>
+							<rules>
+								<requireJavaVersion>
+									<version>[11,)</version>
+									<message>This build requires JDK 11 or newer.
+
+Maven honors JAVA_HOME, not PATH, so a shell where `java -version` reports 11 can
+still run Maven on JDK 8. When that happens, antlr4-maven-plugin fails later with
+an opaque UnsupportedClassVersionError (class file version 55.0 vs 52.0).
+
+If JAVA_HOME must stay on an older JDK for other projects, do not change it
+globally -- run `make test` (which pins JAVA_HOME per-invocation from
+.tool-versions), or prefix a single command:
+
+  JAVA_HOME=$(asdf where java) mvn ...</message>
+								</requireJavaVersion>
+							</rules>
+							<fail>true</fail>
+						</configuration>
+					</execution>
+				</executions>
+			</plugin>
 			<plugin>
 				<groupId>org.antlr</groupId>
 				<artifactId>antlr4-maven-plugin</artifactId>
```

Two details that matter:

- **Placement is load-bearing.** `enforce` binds to `validate`, which runs before
  `generate-sources`. That ordering is what converts the classloader stack trace
  into a readable message.
- **The message deliberately avoids `${java.version}`.** Line 54 of this pom
  defines a *project property* `<java.version>11</java.version>`, which would
  shadow the JVM's real version and print a misleading "11" on a Java 8 run.
  Enforcer reports the detected version in its own output anyway.

`java/pom.xml` is **tab-indented**; the diffs above preserve that.

### Edit 3 — `java/Makefile`: pin `JAVA_HOME`

Anchor: top of file, before line 1 (`PROFILE_DIR ?= .profiles`).

```diff
+# Resolve a JDK independent of the caller's ambient JAVA_HOME.
+#
+# Maven honors JAVA_HOME, not PATH. When the ambient JAVA_HOME targets a pre-11
+# JDK -- a legitimate state on a machine serving several projects -- Maven runs
+# on it and antlr4-maven-plugin:4.13.2 fails with UnsupportedClassVersionError
+# during generate-sources, before any project code compiles. See TODO.md.
+#
+# Scoped to this Makefile's recipes, so developers who need an older JAVA_HOME
+# for other work do not have to change it globally.
+#
+# Prefer the asdf-resolved toolchain named in .tool-versions (temurin-11); fall
+# back to the ambient JAVA_HOME when asdf is absent, so CI images that manage
+# their own JDK are unaffected.
+ASDF_JAVA_HOME := $(shell command -v asdf >/dev/null 2>&1 && asdf where java 2>/dev/null)
+ifneq ($(ASDF_JAVA_HOME),)
+export JAVA_HOME := $(ASDF_JAVA_HOME)
+endif
+
 PROFILE_DIR ?= .profiles
 PROFILE_ITERATIONS ?= 2000
 PROFILE_WARMUP ?= 100
```

No changes are needed to the 12 `mvn` invocations below it — `export` in GNU Make
propagates to every recipe subshell. This also covers the root `Makefile`, whose
`test-java` target is just `(cd java && $(MAKE) test)`.

`asdf where java` resolves `.tool-versions` by walking up from the working
directory, so running it from `java/` finds the repo-root file correctly.

---

## 3. Choosing the enforcer version — read before applying

`3.1.0` is proposed deliberately, **not** because it is current.

What is known about the local Maven cache:

- `~/.m2/repository/.../maven-enforcer-plugin/` contains only **1.4.1** and
  **3.1.0**. For 3.1.0, the full stack is cached: `maven-enforcer-plugin`,
  `enforcer-api`, and `enforcer-rules`.
- `3.5.0` is **not** cached.

Picking an already-cached version avoids a first-build download. Any version
works for a normal online build (see §4), so this is a convenience choice, not a
constraint — bump it freely if you prefer a current release.

### ⚠️ Unresolved: does the enforcer plugin itself run on JDK 8?

**This is the one thing that could invalidate Edits 1–2 entirely, and it was not
verified.**

The whole point of the enforcer is to produce a readable error *while running on
the wrong JDK*. If the chosen enforcer version is itself compiled for Java 11,
it will fail on Java 8 with the very same `UnsupportedClassVersionError` it
exists to explain — just naming `enforcer` instead of `antlr`. No improvement.

Confirm before trusting Edit 2:

```sh
# Should print "major version: 52" (Java 8) or lower.
unzip -p ~/.m2/repository/org/apache/maven/enforcer/enforcer-rules/3.1.0/enforcer-rules-3.1.0.jar \
  'org/apache/maven/plugins/enforcer/RequireJavaVersion.class' > /tmp/rjv.class
javap -verbose -cp /tmp /tmp/rjv.class | grep major
```

Or just test end-to-end, which is the real acceptance check anyway:

```sh
JAVA_HOME=~/.asdf/installs/java/zulu-javafx-8.86.0.25 \
  mvn -B -s .mvn/settings.xml validate
```

You want a clean enforcer failure naming JDK 11, **not** an
`UnsupportedClassVersionError`. If you get the latter, drop to an older enforcer
(1.4.1 is cached and definitely runs on Java 8) or abandon Edit 2 and keep only
Edit 3.

---

## 4. Verification status

Recorded honestly so the next person knows what to re-check.

### Verified

- The failure reproduces exactly as described (`BUILD FAILURE`, ANTLR
  `UnsupportedClassVersionError`).
- The `JAVA_HOME` override workaround produces `BUILD SUCCESS`.
- `java/pom.xml:55-57` sets compiler release/source/target to 11, and this does
  not and cannot prevent the failure.
- `java/Makefile` contains **zero** `-o` flags, i.e. the repo never builds
  offline by design.
- `~/.m2/settings.xml` defines a repository id `externalNexus`; `java/.mvn/settings.xml`
  defines only `github` (for deploys), so builds fall back to `central`.
- `mvn -s <file>` **replaces** the user-level `settings.xml` rather than merging
  it. This repo's builds therefore never see `externalNexus`.

### Not verified

- **That `maven-enforcer-plugin` resolves from Maven Central.** Could not be
  tested: the session was sandboxed, Maven does not read `http_proxy`/`https_proxy`
  environment variables (it uses `<proxies>` in `settings.xml`), so DNS for
  `repo.maven.apache.org` failed regardless of VPN state. This should be a
  non-issue for a normal online build, but it is untested.
- **Whether the enforcer plugin runs on JDK 8.** See §3 — this is the material
  one.
- **That `asdf where java` works on this asdf installation.** The `ifneq` guard
  makes a failure degrade to current behaviour rather than break the build, so
  the blast radius is nil, but the fix would silently not apply.
- End-to-end enforcer rule behaviour.

### A red herring, documented so it is not rediscovered

While investigating, `mvn -o` (offline) rejected the cached enforcer 3.1.0 jars:

```
Cannot access central (https://repo.maven.apache.org/maven2) in offline mode and
the artifact org.apache.maven.plugins:maven-enforcer-plugin:jar:3.1.0 has not
been downloaded from it before.
```

Cause: `_remote.repositories` for those artifacts records only `externalNexus=`
(cached 2026-07-15 by some other project using the default user settings), while
this repo's build sees only `central`. Maven's offline mode refuses an artifact
not sourced from a currently-configured repository. By contrast,
`antlr4-maven-plugin` records both `central=` and `externalNexus=` and works
offline.

**This is an artifact of forcing `-o`, which this repo never does.** It is not a
defect in the proposal and needs no action.

---

## 5. Acceptance

- `make test-java` succeeds in a shell whose `JAVA_HOME` points at a JDK 8,
  **without modifying that `JAVA_HOME`** — it must remain available to other
  projects on the machine.
- With a JDK 8 `JAVA_HOME` and Edit 3 reverted, `mvn -s .mvn/settings.xml validate`
  fails with the enforcer's "requires JDK 11 or newer" message rather than
  `UnsupportedClassVersionError`.

Estimated effort: ~0.25 day.

---

## 6. Adjacent, not a defect

`mvn test` reports 3 errors in `WdlImportResolverTest`
(`Could not initialize plugin: MockMaker`). These are **sandbox artifacts** — the
Mockito inline mock maker could not self-attach its ByteBuddy agent, and surefire
could not create its temp directory. Baseline is otherwise **658 tests, 0
failures**. No action; re-verify outside a sandbox before filing anything.

---

# Part 2 — Validator correctness

Found 2026-10-01 during **Task B4** of the Rust parity work
(`rust/.context/B4_plan.md`), while using this validator as the reference
implementation. Again, **no Java files were edited.**

Item 1 is a gap in the *tests*, and it is the reason items 2–5 have gone
unnoticed. Fixing item 1 first is recommended: it turns the rest from "claims in
a markdown file" into failing tests.

| # | Item | Kind | Impact |
|---|---|---|---|
| 1 | Spec examples are never validated, only parsed | Test gap | Hides ~13 real failures |
| 2 | Struct literals may not omit optional members | Validator bug | Rejects valid WDL |
| 3 | `input {}` declarations are never type-checked | Validator gap | Misses invalid WDL |
| 4 | Scatter/conditional output rewrapping not applied | Validator gap | Rejects valid WDL |
| 5 | `as_map` rejects a valid spec example | Validator bug | Rejects valid WDL |

---

## 1. `testParseSpecExample` never runs the validator

**This is the highest-value item in this document.**

`WdlV11SpecExamplesTest.testParseSpecExample` (`:53-74`) loads every spec
example and asserts only that it parses:

```java
WdlDocument doc =
    (hasImports && !hasRemoteImports)
        ? WdlV1Loader.load(wdlContent, filePath.toUri())
        : WdlV1Loader.load(wdlContent);
```

Neither overload takes a validator. The only place `new WdlValidator()` appears
in these suites is `testParseAndValidateFailSpecExample` (`:81-85`), whose
`@MethodSource` is `v11FailExamples` — filtered to `_fail.wdl` (`:45-47`).

**So the validator is exercised against examples that are *supposed* to fail,
and never against the examples that are supposed to pass.** Any false positive —
the validator rejecting valid WDL — is invisible to this suite by construction.

Identical in `WdlV12SpecExamplesTest` (`:59`, `:67-68`, `:91-95`) and
`WdlV13SpecExamplesTest` (`:62`, `:70-71`, `:93-97`).

### Evidence

Running `WdlV1Loader.load(content, new WdlValidator())` directly over the valid
v1_1 examples rejects at least these 13:

| File | Diagnostic | Cause |
|---|---|---|
| `test_struct.wdl` | `Declaration 'john' type is incompatible with expression` | Item 2 |
| `serde_pair.wdl` | `as_map expects Array[Pair[K,V]]` | Item 5 |
| `map_to_array.wdl` | `Declaration 'aout' ...` | Item 4 |
| `test_scatter.wdl` | `Declaration 'messages' ...` | Item 4 |
| `nested_scatter.wdl` | `Declaration 'used_honorifics' ...` | Item 4 |
| `test_conditional.wdl` | `Declaration 'maybe_result2' ...` | Item 4 |
| `serialize_map.wdl` | `Call input 'args' ...` | Item 4 |
| `test_map_ordering.wdl` | `Declaration 'ints' ...` | Item 4 |
| `test_keys.wdl` | `Declaration 'str_to_files_keys' ...` | Item 4 |
| `test_range.wdl` | `Declaration 'result' ...` | Item 4 |
| `test_values.wdl` (v1_2) | `Declaration 'sums' ...` | Item 4 |
| `chunk_array.wdl` (v1_2) | `Declaration 'concats' ...` | Item 4 |
| `allow_nested.wdl` | `Declaration 'incrs' ...` | Item 4 |
| `main.wdl` | `Declaration 'echo_array' ...` | Item 4 |

These are normative examples lifted from the WDL specification. Every one of
them is valid WDL, so each diagnostic is a false positive.

### Proposed change

Pass a validator in `testParseSpecExample`, and introduce an explicit
skip-list for the known-failing files so the suite can go green immediately and
shrink as items 2–5 are fixed:

```java
WdlValidator validator = KNOWN_VALIDATION_GAPS.contains(filename) ? null : new WdlValidator();
```

> ⚠️ **Expect more than 13.** The list above was gathered with
> `WdlV1Loader.load(content, new WdlValidator())`, which supplies **no base
> URI**, so imports do not resolve and importing files emit extra
> `UNKNOWN_REFERENCE` noise (`'msg' is not an output field of call
> 'say_hello'`). A correct implementation must mirror the existing
> `hasImports && !hasRemoteImports` branch and pass `filePath.toUri()`. Re-measure
> the skip-list that way; do not copy the table above verbatim.

The Rust port already does this — see `rust/tests/spec_validation_test.rs` and
its `P1_INFERENCE_GAP` skip list, which is the direct analogue.

---

## 2. Struct, object, and map literals may not omit optional members

`WdlExpressionValidator.keyedEntriesMatchExpectedMembers` (`:820-836`) compares
the literal's key set to the struct's member set for **equality**:

```java
if (!actualEntries.keySet().equals(expectedMembers.keySet())) {
  return false;
}
```

The map-literal branch of `isStructAssignableFromExpression` repeats the same
rule inline (`:806-808`). Consequence: a literal that omits *any* member is
rejected, including a member the struct declares as optional.

### Evidence

`spec_examples/v1_1/test_struct.wdl` omits the optional `String? username` and
says so in its own comment:

```wdl
Person john = Person {
  name: "John",
  # it's okay to leave out username since it's optional
  account: BankAccount { ... }
}
```

Running the validator over it:

```
FAIL test_struct.wdl : Declaration 'john' type is incompatible with expression (TYPE_MISMATCH)
```

Reproduced on both v1_1 and v1_2. The spec example documents the intended rule
in a comment; the validator contradicts it.

### The correct rule

1. Every **required** member must be present.
2. **Optional** members (`T?`) may be omitted.
3. A key naming no declared member is an error.

The current code gets (3) right and (1)–(2) wrong, by collapsing them into one
equality test.

### Proposed change

Replace the key-set equality in both places with:

```java
for (String key : actualEntries.keySet()) {
  if (!expectedMembers.containsKey(key)) {
    return false; // unknown member
  }
}
for (Map.Entry<String, WdlType> expected : expectedMembers.entrySet()) {
  WdlExpression actual = actualEntries.get(expected.getKey());
  if (actual == null) {
    if (!expected.getValue().isOptional()) {
      return false; // missing required member
    }
    continue;
  }
  if (!isAssignableFrom(expected.getValue(), actual)) {
    return false;
  }
}
return true;
```

Note the existing loop relies on `isAssignableFrom(type, null)` returning `true`
(`:201-203`) for absent members; the rewrite makes the absence case explicit
instead, which is why the required/optional distinction becomes expressible.

### Risk

**Rule (1) is a tightening and may newly reject files.** Introduce it behind
the same change as rules (2)–(3) only if item 1 is fixed first, so the spec
corpus can tell you what moved.

The Rust port implements (2) and (3) but deliberately **not** (1) — see
`ValidatorRunner::keyed_entries_match_members` in `rust/src/validators/mod.rs`,
which documents the omission and the reasoning.

### A dead end, recorded so it is not rediscovered

`incomplete_struct_fail.wdl` looks like it pins rule (1) from the other side
(`# error! missing required account_number`). It does not. It writes its struct
literal with **quoted string keys**:

```wdl
Person fail1 = Person {
  "name": "Sam",
  ...
}
```

The grammar requires `IDENTIFIER` for struct-literal keys, so the file dies at
parse time (`mismatched input '{' expecting {IDENTIFIER, QUESTION_MARK}`) and
never reaches semantic validation. It passes `testParseAndValidateFailSpecExample`
for the wrong reason. **Nothing in the corpus currently exercises the
required-member rule**; a new fixture would be needed to test rule (1).

---

## 3. `input {}` bound declarations are never assignability-checked

`WdlValidator.processWorkflowInput` (`:775-786`) validates and evaluates each
bound declaration's *expression*, but never calls `validateBoundDeclaration`:

```java
if (declaration instanceof WdlBoundDeclaration && declaration.getName() != null) {
  WdlBoundDeclaration bound = (WdlBoundDeclaration) declaration;
  expressionValidator.validate(bound.getExpression());
  scopeValues.put(declaration.getName(), expressionValidator.evaluate(bound.getExpression()));
}
```

Compare its siblings, which both do:

- `processWorkflowDeclaration` (`:789-791`) → `validateBoundDeclaration(node)`
- `processWorkflowOutput` (`:793-798`) → `validateBoundDeclaration(declaration)`

`validateBoundDeclaration` (`:912`) is what compares the declared type against
the expression. So a default value in an `input {}` block is never checked
against its own declared type, while the identical declaration one block over
is.

This also **masks item 2**: `import_structs.wdl` declares
`PatientIncome average_income = PatientIncome { ... }` inside an `input {}`
block, omitting an optional member. It passes only because the check never runs.
Fixing item 2 without noticing this would be confusing; fixing item 3 alone
would newly surface item 2 there.

### Proposed change

Add the missing call, mirroring `processWorkflowOutput`:

```java
if (declaration instanceof WdlBoundDeclaration && declaration.getName() != null) {
  WdlBoundDeclaration bound = (WdlBoundDeclaration) declaration;
  validateBoundDeclaration(bound);
  scopeValues.put(declaration.getName(), expressionValidator.evaluate(bound.getExpression()));
}
```

`validateBoundDeclaration` already calls `expressionValidator.validate(...)`
internally — confirm before applying, to avoid double-reporting.

### Open question

There is **no `processTaskInput` override in `WdlValidator` at all** —
`processWorkflowInput` is the only input hook (grep for `WdlInput node`). Whether
task inputs are covered by some other path, or simply unvalidated, was not
determined. Worth checking while here.

---

## 4. Scatter/conditional output rewrapping is not applied

Inside a `scatter {}`, a declaration or call output has scalar type `T`; from
the enclosing scope the same symbol is `Array[T]`. Inside an `if {}` it becomes
`T?`. The validator does not apply that rewrapping, so an outer reference is
checked against the inner scalar type and a correct declaration looks wrong:

```wdl
scatter (i in indices) {
  Array[Int] a = ...
}
Array[Array[Int]] aout = a   # rejected: 'aout' type is incompatible with expression
```

It surfaces at two sites — as a declaration mismatch (`map_to_array.wdl`) and as
a call-input mismatch (`serialize_map.wdl`, `Call input 'args' ...`). Same gap.

This accounts for 12 of the 13+ false positives in item 1's table.

### Status

**The Rust port has the identical gap**, and has deliberately deferred it — see
the `P1_INFERENCE_GAP` skip list in `rust/tests/spec_validation_test.rs`.
Recorded here for symmetry: whichever implementation fixes it first should be
the reference for the other. This is the largest of the items and the one most
likely to need design discussion rather than a patch.

---

## 5. `as_map` rejects a valid spec example

```
FAIL serde_pair.wdl : as_map expects Array[Pair[K,V]] (INVALID_FUNCTION_ARGUMENTS)
```

`spec_examples/v1_1/serde_pair.wdl` is a normative example, so this is a false
positive — most likely the argument's inferred type is `Array[Pair[...]]` but
the check is comparing something stricter, or the element type failed to infer
and the check treats "unknown" as "wrong".

Not investigated further; it was out of scope for B4. The Rust port does not
implement this `as_map` argument check at all, so it has no corresponding
failure.

**Suggested first step:** log the actual inferred argument type at the rejection
site and compare against `Array[Pair[K,V]]`.

---

## Verification status for Part 2

### Verified

- Items 1, 2, 3: read directly from the Java sources at the line numbers cited.
- Item 2's `test_struct.wdl` rejection: reproduced on v1_1 and v1_2.
- Items 4 and 5: reproduced by running `WdlValidator` over each file.
- `incomplete_struct_fail.wdl` fails at parse, not validation: reproduced.

All runs used a single-file source launch against the prebuilt `target/classes`
(Maven was unavailable in the sandbox):

```sh
java --class-path java/target/classes:"$HOME"/.m2/repository/org/antlr/antlr4-runtime/4.13.2/antlr4-runtime-4.13.2.jar \
  Probe.java <files...>
```

### Not verified

- **The exact size of item 1's skip list.** Measured without a base URI, so the
  numbers include import-resolution noise. See the callout in item 1.
- **Whether task inputs are validated anywhere.** See item 3's open question.
- **Item 5's root cause.** Only the symptom was reproduced.
- **That the proposed patches compile.** They are illustrative, written without
  the ability to build.

---

## Suggested order

1. **Item 1** — pure test change, no behaviour risk, and it makes everything
   else measurable. Land with a generous skip list.
2. **Item 2** rules (2)–(3), then **item 3**, then **item 2** rule (1). Items 2
   and 3 interact (see item 3), so sequence them and re-measure between.
3. **Item 5** — small and self-contained once someone looks at it.
4. **Item 4** — largest; shared with the Rust port; coordinate.

Estimated effort: item 1 ~0.5 day, items 2–3 ~0.5 day, item 5 unknown, item 4
multiple days.
