use crate::parser::Parser;

mod parser;
mod lexer;

fn main() {
    let mut parser = Parser::new("calculation=(33+4)*3+1");
    let result = parser.get_symbol_table();
    match result {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => eprint!("Error: {}", error)
    }
}
