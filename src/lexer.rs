use thiserror::Error;

use crate::lexer::Token::EOF;
use crate::lexer::TokenError::UnsupportedCharacter;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    EOF,
}

#[derive(Debug, Error, PartialEq, Clone)]
pub enum TokenError {
    #[error("Unsupported character '{0}' at position {1}")]
    UnsupportedCharacter(char, usize)
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub(crate) fn get_next_token(&mut self) -> Result<Token, TokenError> {
        if self.position >= self.input.len() {
            return Ok(EOF);
        }

        let current_char = self.input[self.position];

        if current_char.is_digit(10) {
            let mut num_str = String::new();
            while self.position < self.input.len() && self.input[self.position].is_digit(10) {
                num_str.push(self.input[self.position]);
                self.advance();
            }
            Ok(Token::Number(num_str.parse().unwrap()))
        } else if current_char.is_whitespace() {
            self.advance();
            self.get_next_token()
        } else {
            match current_char {
                '+' => {
                    self.advance();
                    Ok(Token::Plus)
                }
                '-' => {
                    self.advance();
                    Ok(Token::Minus)
                }
                '*' => {
                    self.advance();
                    Ok(Token::Multiply)
                }
                '/' => {
                    self.advance();
                    Ok(Token::Divide)
                }
                _ => Err(UnsupportedCharacter(current_char, self.position))
            }
        }
    }
}