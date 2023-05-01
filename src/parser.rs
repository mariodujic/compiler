use crate::lexer::{Lexer, Token, TokenError};
use crate::lexer::Token::EOF;

pub struct Parser {
    lexer: Lexer,
    current_token: Result<Token, TokenError>,
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
        match &self.current_token {
            Ok(t) if *t == token => self.current_token = self.lexer.get_next_token(),
            _ => panic!("Unsupported token result")
        }
    }

    fn factor(&mut self) -> Result<i32, TokenError> {
        match self.current_token.clone() {
            Ok(Token::Number(value)) => {
                self.eat(Token::Number(value));
                Ok(value)
            }
            Ok(Token::OpenParenthesis) => {
                self.eat(Token::OpenParenthesis);
                let result = self.expr()?;
                self.eat(Token::CloseParenthesis);
                Ok(result)
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to resolve factor")
        }
    }

    fn term(&mut self) -> Result<i32, TokenError> {
        let mut result = self.factor()?;
        while let Ok(token) = self.current_token {
            match token {
                Token::Multiply => {
                    self.eat(Token::Multiply);
                    result *= self.factor()?;
                }
                Token::Divide => {
                    self.eat(Token::Divide);
                    result /= self.factor()?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> Result<i32, TokenError> {
        let mut result = self.term()?;
        while self.current_token != Ok(EOF) {
            match &self.current_token {
                Ok(res) => {
                    match res {
                        Token::Plus => {
                            self.eat(Token::Plus);
                            result += self.term()?;
                        }
                        Token::Minus => {
                            self.eat(Token::Minus);
                            result -= self.term()?;
                        }
                        _ => break,
                    }
                }
                Err(err) => return Err(err.clone())
            }
        }
        Ok(result)
    }
}
