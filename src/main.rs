use crate::parser::Parser;

mod parser;
mod lexer;

fn main() {
    let mut parser = Parser::new("(33+4)*3+1");
    let result = parser.expr();
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprint!("Error: {}", error)
    }
}
