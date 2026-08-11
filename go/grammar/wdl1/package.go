// Package wdl1 contains the WDL v1 ANTLR grammar bindings for Go.
//
// The .g4 files in this directory are symlinks to the shared repository grammar
// source. Generated Go parser artifacts are committed so `go get` consumers can
// build without requiring `go generate` in dependency modules.
package wdl1

//go:generate npx -y antlr4ng-cli@2.0.0 -Dlanguage=Go -visitor -listener -package wdl1 -o . ./WdlV1Lexer.g4 ./WdlV1Parser.g4
