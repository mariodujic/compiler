use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == Some(token) {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token");
        }
    }

    fn factor(&mut self) -> i32 {
        match self.current_token {
            Some(Token::Number(value)) => {
                self.eat(Token::Number(value));
                value
            }
            _ => panic!("Invalid token"),
        }
    }

    fn term(&mut self) -> i32 {
        let mut result = self.factor();
        while let Some(token) = self.current_token {
            match token {
                Token::Multiply => {
                    self.eat(Token::Multiply);
                    result *= self.factor();
                }
                Token::Divide => {
                    self.eat(Token::Divide);
                    result /= self.factor();
                }
                _ => break,
            }
        }
        result
    }

    pub fn expr(&mut self) -> i32 {
        let mut result = self.term();
        while let Some(token) = self.current_token {
            match token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    result += self.term();
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    result -= self.term();
                }
                _ => break,
            }
        }
        result
    }
}
