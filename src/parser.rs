use std::collections::HashMap;

use crate::lexer::{Lexer, Token, TokenError};
use crate::lexer::Token::EOF;

pub struct Parser {
    lexer: Lexer,
    current_token: Result<Token, TokenError>,
    symbol_table: HashMap<Box<str>, i32>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
            symbol_table: HashMap::new(),
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
            _ => Ok(0)
        }
    }

    fn term(&mut self) -> Result<i32, TokenError> {
        let mut result = self.factor()?;
        while let Ok(token) = self.current_token.clone() {
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

    fn parse_identifier(&mut self) -> Result<Box<str>, TokenError> {
        match self.current_token.clone() {
            Ok(Token::Identifier(value)) => {
                self.eat(Token::Identifier(value.clone()));
                Ok(value)
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to parse identifier"),
        }
    }

    fn parse_assignment(&mut self) -> Result<(), TokenError> {
        match self.current_token.clone() {
            Ok(Token::AssignmentOperator) => {
                self.eat(Token::AssignmentOperator);
                Ok(())
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to parse assignment operator"),
        }
    }

    fn expr(&mut self) -> Result<i32, TokenError> {
        let mut result = self.term().unwrap_or(0);
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
                        Token::Identifier(_) => {
                            let identifier = self.parse_identifier()?;
                            self.parse_assignment()?;
                            let value = self.expr()?;
                            self.symbol_table.entry(identifier).or_insert(value);
                        }
                        _ => break,
                    }
                }
                Err(err) => return Err(err.clone())
            }
        }
        Ok(result)
    }

    pub fn get_symbol_table(&mut self) -> Result<HashMap<Box<str>, i32>, TokenError> {
        self.expr()?;
        return Ok(self.symbol_table.clone());
    }
}
