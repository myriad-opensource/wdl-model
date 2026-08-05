// Package wdl provides Go APIs for loading and validating WDL documents.
//
// The package is intentionally split into small, Go-idiomatic entry points:
//
//   - Loader for parsing source and resolving imports.
//   - Resolver for local file and HTTP(S) import retrieval.
//   - Validator for semantic/static/lint pass integration.
//
// Generated parser sources are isolated in the grammar/wdl1 package to mirror
// the same generated-code separation used in the Java, Python, and TypeScript
// implementations.
package wdl
