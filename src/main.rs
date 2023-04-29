use crate::lexer::Lexer;

mod lexer;

fn main() {
    let mut lexer = Lexer::new("33+4*3-5/2");
    while let Some(token) = lexer.get_next_token() {
        println!("{:?}", token)
    }
}
