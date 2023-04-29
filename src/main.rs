use crate::parser::Parser;

mod parser;
mod lexer;

fn main() {
    let mut parser = Parser::new("33+4*3-4/2");
    let result = parser.expr();
    println!("Result: {}", result);
}
