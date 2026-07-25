// Test runner para Lexer y Parser
// Fase 39 - Parser Real para scripts .bf

use forge_editor::bakeforge_parser::{Lexer, Parser};

fn main() {
    let source = include_str!("test_ok.bf");
    
    println!("=== Lexer Test ===");
    println!("Source:\n{}\n", source);
    
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };
    
    println!("Tokens ({}):", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, token);
    }
    
    println!("\nFirst 10 tokens:");
    for (i, token) in tokens.iter().take(10).enumerate() {
        println!("  {}: {:?}", i, token);
    }
    
    println!("\n=== Parser Test ===");
    
    let mut parser = Parser::new(tokens.clone());
    println!("Parser initialized with {} tokens", tokens.len());
    println!("First token: {:?}", tokens[0]);
    match parser.parse() {
        Ok(ast) => {
            println!("AST:\n{:?}", ast);
        }
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
        }
    }
}
