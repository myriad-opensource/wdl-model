use std::path::PathBuf;
use std::process::Command;

fn main() {
    let grammar_dir = PathBuf::from("../wdl-grammar/antrl4/v1");
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
            out_dir.to_str().unwrap(),
            grammar_dir.join("WdlV1Lexer.g4").to_str().unwrap(),
            grammar_dir.join("WdlV1Parser.g4").to_str().unwrap(),
        ])
        .status()
        .expect("failed to run java; ensure Java is on PATH");

    if !status.success() {
        panic!("ANTLR4 codegen failed");
    }

    // Post-process: rename WdlV1ParserParserContext -> WdlV1ParserContext
    let parser_file = out_dir.join("wdlv1parser.rs");
    if parser_file.exists() {
        let src = std::fs::read_to_string(&parser_file).expect("failed to read generated parser");
        let patched = src.replace("WdlV1ParserParserContext", "WdlV1ParserContext");
        std::fs::write(&parser_file, patched).expect("failed to write patched parser");
    }
}
