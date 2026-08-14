use std::path::{Path, PathBuf};
use std::process::Command;

/// Filenames the crate actually consumes (see src/grammar/mod.rs). Anything else
/// the codegen tool emits (.interp/.tokens files, or nested package directories
/// mirroring the -lib path) is scratch output we don't need to keep.
const GENERATED_FILES: &[&str] = &[
    "wdlv1lexer.rs",
    "wdlv1parser.rs",
    "wdlv1parserbaselistener.rs",
    "wdlv1parserbasevisitor.rs",
    "wdlv1parserlistener.rs",
    "wdlv1parservisitor.rs",
];

/// Recursively search `root` for `name`, returning the first match.
/// The antlr4-rust-tool sometimes mirrors the `-lib` path's directory
/// structure under `-o` instead of writing flat into it, so we can't assume
/// a fixed location.
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

fn main() {
    // Grammar sources are consumed via the `antlr4` symlink (-> ../wdl-grammar/antlr4),
    // mirroring the python/ dir's layout. The wdl-grammar submodule must be checked
    // out (`git submodule update --init wdl-grammar`) for this path to exist.
    let grammar_dir = PathBuf::from("antlr4/v1");
    let out_dir = PathBuf::from("src/grammar");
    let jar = PathBuf::from("antlr4-rust-tool.jar");

    // Rerun if grammar files or the JAR change
    println!(
        "cargo:rerun-if-changed={}",
        grammar_dir.join("WdlV1Lexer.g4").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        grammar_dir.join("WdlV1Parser.g4").display()
    );
    println!("cargo:rerun-if-changed={}", jar.display());

    // Only regenerate if the JAR is present (skip in CI environments without Java)
    if !jar.exists() {
        println!("cargo:warning=antlr4-rust-tool.jar not found; using pre-generated grammar files");
        return;
    }

    // Only regenerate if the grammar submodule has been checked out (skip in
    // environments where `git submodule update --init` hasn't been run yet).
    if !grammar_dir.join("WdlV1Lexer.g4").exists() || !grammar_dir.join("WdlV1Parser.g4").exists() {
        println!(
            "cargo:warning=wdl-grammar submodule not initialized ({} missing); using pre-generated grammar files",
            grammar_dir.display()
        );
        return;
    }

    // Skip codegen if Java is not on PATH (e.g. no JRE installed); fall back to
    // the pre-generated grammar files checked into src/grammar.
    let java_available = Command::new("java")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !java_available {
        println!("cargo:warning=java not found on PATH; using pre-generated grammar files");
        return;
    }

    // Generate into a scratch directory under Cargo's OUT_DIR (not src/grammar
    // directly): the codegen tool sometimes mirrors the `-lib` path's directory
    // structure into its output instead of writing flat files, which would
    // otherwise pollute the tracked src/grammar/ directory with a stray nested
    // `antlr4/v1/...` copy. We scan the scratch dir afterward and copy over
    // only the files the crate actually uses.
    let scratch_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set by cargo"))
        .join("antlr4-codegen");
    if scratch_dir.exists() {
        std::fs::remove_dir_all(&scratch_dir).expect("failed to clear stale codegen scratch dir");
    }
    std::fs::create_dir_all(&scratch_dir).expect("failed to create codegen scratch dir");

    let status = Command::new("java")
        .args([
            "-jar",
            jar.to_str().unwrap(),
            "-Dlanguage=Rust",
            "-visitor",
            "-listener",
            "-lib",
            grammar_dir.to_str().unwrap(),
            "-o",
            scratch_dir.to_str().unwrap(),
            grammar_dir.join("WdlV1Lexer.g4").to_str().unwrap(),
            grammar_dir.join("WdlV1Parser.g4").to_str().unwrap(),
        ])
        .status()
        .expect("failed to run java; ensure Java is on PATH");

    if !status.success() {
        println!(
            "cargo:warning=ANTLR4 codegen failed; falling back to pre-generated grammar files"
        );
        return;
    }

    // Copy only the files the crate consumes out of the scratch dir, flat into
    // src/grammar. If any expected file is missing, bail out without touching
    // the tracked pre-generated sources (mirrors the tool's habit of exiting 0
    // even when it fails to produce complete output, e.g. `error(114): cannot
    // find tokens file`).
    let mut located = Vec::with_capacity(GENERATED_FILES.len());
    for name in GENERATED_FILES {
        match find_file(&scratch_dir, name) {
            Some(path) => located.push(path),
            None => {
                println!(
                    "cargo:warning=ANTLR4 codegen did not produce {name}; falling back to pre-generated grammar files"
                );
                return;
            }
        }
    }
    for path in &located {
        let dest = out_dir.join(path.file_name().unwrap());
        std::fs::copy(path, &dest).unwrap_or_else(|e| {
            panic!(
                "failed to copy {} to {}: {e}",
                path.display(),
                dest.display()
            )
        });
    }

    // Post-process: rename WdlV1ParserParserContext -> WdlV1ParserContext
    let parser_file = out_dir.join("wdlv1parser.rs");
    if parser_file.exists() {
        let src = std::fs::read_to_string(&parser_file).expect("failed to read generated parser");
        let patched = src.replace("WdlV1ParserParserContext", "WdlV1ParserContext");
        std::fs::write(&parser_file, patched).expect("failed to write patched parser");
    }
}
