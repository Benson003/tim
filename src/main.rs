mod cmd;
mod diagnostics;
mod generator;
mod parser;
mod source_map;
mod tokenizer;

use std::fs;

use crate::cmd::cli::CliConfig;
use crate::diagnostics::diagnostics::DiagnosticPrinter;
use crate::generator::template::TemplateEngine;
use crate::generator::walker::Compiler;
use crate::parser::parser::Parser;
use crate::source_map::source_map::SourceMap;
use crate::tokenizer::tokenizer::TokenList;

fn main() {
    let config = CliConfig::parse();

    let content = match fs::read_to_string(&config.input_file) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", config.input_file, e);
            std::process::exit(1);
        }
    };

    let source_map = SourceMap::new(content);
    let printer = DiagnosticPrinter::new(&source_map);

    let mut lexer = TokenList::new();
    lexer.tokenize(&source_map);
    let mut parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    let mut diagnostics = lexer.diagnostics;
    diagnostics.extend(parser.diagnostics);

    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| matches!(d.severity, diagnostics::diagnostics::Severity::Error))
        .collect();
    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| matches!(d.severity, diagnostics::diagnostics::Severity::Warning))
        .collect();

    // Print warnings first
    if !warnings.is_empty() {
        printer.print_multiple(&warnings);
    }

    // Print errors
    if !errors.is_empty() {
        printer.print_multiple(&errors);
        std::process::exit(1);
    }

    if config.dry_run {
        println!("--- GENERATED AST ---");
        println!("{:#?}", ast);
        return;
    }

    let compiler = Compiler::new();
    let mut compiled_html = compiler.compile(ast);

    if let Some(ref tag) = config.wrap_tag {
        compiled_html = format!("<{}>\n{}</{}>", tag, compiled_html, tag);
    }

    let final_output = if config.output_file.is_some() {
        TemplateEngine::wrap_to_full_page("Compiled Document", &compiled_html)
    } else {
        compiled_html
    };

    match config.output_file {
        Some(ref dest_path) => {
            if let Err(e) = fs::write(dest_path, final_output) {
                eprintln!("Error saving to output file '{}': {}", dest_path, e);
                std::process::exit(1);
            }
            println!("Successfully compiled! Output written to {}", dest_path);
        }
        None => {
            print!("{}", final_output);
        }
    }
}
