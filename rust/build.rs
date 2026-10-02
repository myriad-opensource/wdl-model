use std::path::{Path, PathBuf};
use std::process::Command;

/// Filenames the crate actually consumes (see src/grammar/mod.rs). Anything else
/// the codegen tool emits (.interp/.tokens files, or nested package directories
/// mirroring the input path) is scratch output we don't need to keep.
const GENERATED_FILES: &[&str] = &[
    "wdlv1lexer.rs",
    "wdlv1parser.rs",
    "wdlv1parserbaselistener.rs",
    "wdlv1parserbasevisitor.rs",
    "wdlv1parserlistener.rs",
    "wdlv1parservisitor.rs",
];

/// Recursively search `root` for `name`, returning the first match.
/// The antlr4-rust-tool mirrors the input grammar's path under `-o` instead of
/// writing flat into it, so we can't assume a fixed location.
fn find_file(root: &Path, name: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(root).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_file(&path, name) {
                return Some(found);
            }
        } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
            return Some(path);
        }
    }
    None
}

/// Runs one codegen pass over a single grammar file.
///
/// `lib_dir` supplies `-lib`, which is how the parser pass finds the
/// `WdlV1Lexer.tokens` emitted by the lexer pass (required by
/// `WdlV1Parser.g4`'s `options { tokenVocab=WdlV1Lexer; }`).
///
/// Note: a successful exit status is *not* sufficient evidence that codegen
/// worked. The tool exits 0 even on hard errors such as
/// `error(114): cannot find tokens file`. The completeness check in `main` is
/// the authoritative signal; this only catches outright crashes.
fn run_codegen(jar: &Path, grammar: &Path, out: &Path, lib_dir: Option<&Path>) {
    let mut cmd = Command::new("java");
    cmd.args([
        "-jar",
        jar.to_str().expect("jar path is not valid UTF-8"),
        "-Dlanguage=Rust",
        "-visitor",
        "-listener",
    ]);
    if let Some(lib) = lib_dir {
        cmd.arg("-lib")
            .arg(lib.to_str().expect("lib path is not valid UTF-8"));
    }
    cmd.arg("-o")
        .arg(out.to_str().expect("out path is not valid UTF-8"))
        .arg(grammar.to_str().expect("grammar path is not valid UTF-8"));

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("failed to run java for {}: {e}", grammar.display()));
    if !status.success() {
        panic!(
            "ANTLR4 codegen failed for {} (exit status {status})",
            grammar.display()
        );
    }
}

fn main() {
    // Grammar sources are consumed via the `antlr4` symlink (-> ../wdl-grammar/antlr4),
    // mirroring how java/src/main/antlr4 symlinks the same files. The wdl-grammar
    // submodule must be checked out for this path to exist.
    let grammar_dir = PathBuf::from("antlr4/v1");
    let lexer_g4 = grammar_dir.join("WdlV1Lexer.g4");
    let parser_g4 = grammar_dir.join("WdlV1Parser.g4");
    let jar = PathBuf::from("antlr4-rust-tool.jar");

    println!("cargo:rerun-if-changed={}", lexer_g4.display());
    println!("cargo:rerun-if-changed={}", parser_g4.display());
    println!("cargo:rerun-if-changed={}", jar.display());

    // The parser is generated on every build and is deliberately not checked
    // into the repository (mirroring the java/ module, where antlr4-maven-plugin
    // regenerates into target/generated-sources). That makes it impossible for
    // the generated code to drift from wdl-grammar. The flip side is that the
    // toolchain is a hard build requirement, so every missing piece below is a
    // build failure rather than a silent fallback.
    if !jar.exists() {
        panic!(
            "{} not found; it is vendored in-tree and required to generate the parser",
            jar.display()
        );
    }

    if !lexer_g4.exists() || !parser_g4.exists() {
        panic!(
            "wdl-grammar submodule not initialized ({} missing); run `git submodule update --init wdl-grammar`",
            grammar_dir.display()
        );
    }

    let java_available = Command::new("java")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !java_available {
        panic!("java not found on PATH; a JRE is required to generate the parser from wdl-grammar");
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set by cargo"));

    // Generate into a scratch subdirectory rather than OUT_DIR directly: the
    // tool mirrors the input grammar's path (antlr4/v1/...) under -o, so its
    // output is nested. We flatten the files we need out of it afterward.
    let scratch_dir = out_dir.join("antlr4-codegen");
    if scratch_dir.exists() {
        std::fs::remove_dir_all(&scratch_dir).expect("failed to clear stale codegen scratch dir");
    }
    std::fs::create_dir_all(&scratch_dir).expect("failed to create codegen scratch dir");

    // Pass 1: lexer. Emits WdlV1Lexer.tokens, which pass 2 needs.
    run_codegen(&jar, &lexer_g4, &scratch_dir, None);

    // Pass 2: parser. `WdlV1Parser.g4` declares `tokenVocab=WdlV1Lexer`, and
    // ANTLR resolves that against -lib *before* generating anything, so the
    // tokens file has to already exist. Generating both grammars in a single
    // invocation cannot work for this reason.
    let tokens = find_file(&scratch_dir, "WdlV1Lexer.tokens").unwrap_or_else(|| {
        panic!(
            "lexer codegen did not produce WdlV1Lexer.tokens under {}",
            scratch_dir.display()
        )
    });
    let tokens_dir = tokens
        .parent()
        .expect("generated tokens file has no parent directory");
    run_codegen(&jar, &parser_g4, &scratch_dir, Some(tokens_dir));

    // Flatten the files the crate consumes into OUT_DIR. A missing file here is
    // the real failure signal, since the tool exits 0 even when it bails out
    // (e.g. `error(114): cannot find tokens file`).
    let mut generated = Vec::with_capacity(GENERATED_FILES.len());
    for name in GENERATED_FILES {
        let src = find_file(&scratch_dir, name).unwrap_or_else(|| {
            panic!(
                "ANTLR4 codegen did not produce {name}; the grammar in {} may be malformed",
                grammar_dir.display()
            )
        });
        let dest = out_dir.join(name);
        std::fs::copy(&src, &dest).unwrap_or_else(|e| {
            panic!(
                "failed to copy {} to {}: {e}",
                src.display(),
                dest.display()
            )
        });
        generated.push(dest);
    }

    // Post-process: rename WdlV1ParserParserContext -> WdlV1ParserContext.
    let parser_file = out_dir.join("wdlv1parser.rs");
    let src = std::fs::read_to_string(&parser_file).expect("failed to read generated parser");
    let patched = src.replace("WdlV1ParserParserContext", "WdlV1ParserContext");
    std::fs::write(&parser_file, patched).expect("failed to write patched parser");

    // Emit the module declarations that src/grammar/mod.rs includes.
    //
    // `#[path]` + `mod` (rather than `include!` of the sources directly) is
    // deliberate: the generated files start with inner attributes
    // (`#![allow(dead_code)]` etc.), which are illegal in an `include!`ed
    // fragment but fine in a file loaded as a module. Module paths stay
    // `crate::grammar::*`, so the generated `use super::wdlv1parservisitor::*;`
    // cross-references keep resolving.
    let mut decls = String::from("// @generated by build.rs - do not edit\n");
    for path in &generated {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("generated file has no stem");
        decls.push_str("#[allow(clippy::all)]\n");
        decls.push_str(&format!("#[path = {:?}]\n", path.to_str().unwrap()));
        decls.push_str(&format!("pub mod {stem};\n"));
    }
    std::fs::write(out_dir.join("grammar_modules.rs"), decls)
        .expect("failed to write grammar module declarations");
}
