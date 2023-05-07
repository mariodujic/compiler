use std::fs;

use crate::parser::Parser;

mod parser;
mod lexer;
mod error;
mod symbol;
mod token;

fn main() {
    let content = fs::read_to_string("sample/main.gz").unwrap();

    let mut parser = Parser::new(content.as_str());
    let result = parser.get_symbol_table();
    match result {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => eprint!("Error: {}", error)
    }
}
