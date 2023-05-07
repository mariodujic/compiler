use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::CompilerError;
use crate::error::CompilerError::UnsupportedCharacter;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    pub(crate) fn get_next_token(&mut self) -> Result<Token, CompilerError> {

        let current_char = match self.input.peek() {
            Some(c) => *c,
            None => return Ok(Token::EOF),
        };

        if current_char.is_digit(10) {
            let mut num_str = String::new();
            while let Some(&c) = self.input.peek() {
                if c.is_digit(10) {
                    num_str.push(c);
                    self.input.next();
                } else {
                    break;
                }
            }
            Ok(Token::Number(num_str.parse().unwrap()))
        } else if current_char.is_alphabetic() {
            let mut identifier = String::new();
            while let Some(&c) = self.input.peek() {
                if is_valid_identifier(c) {
                    identifier.push(c);
                    self.input.next();
                } else {
                    break;
                }
            }
            if &identifier == "immut" {
                Ok(Token::Immutable)
            } else if &identifier == "mut" {
                Ok(Token::Mutable)
            } else {
                Ok(Token::Identifier(identifier.into_boxed_str()))
            }
        } else if current_char.is_whitespace() {
            self.input.next();
            self.get_next_token()
        } else {
            match current_char {
                '+' => {
                    self.input.next();
                    Ok(Token::Plus)
                }
                '-' => {
                    self.input.next();
                    Ok(Token::Minus)
                }
                '*' => {
                    self.input.next();
                    Ok(Token::Multiply)
                }
                '/' => {
                    self.input.next();
                    Ok(Token::Divide)
                }
                '(' => {
                    self.input.next();
                    Ok(Token::OpenParenthesis)
                }
                ')' => {
                    self.input.next();
                    Ok(Token::CloseParenthesis)
                }
                '=' => {
                    self.input.next();
                    Ok(Token::AssignmentOperator)
                }
                _ => Err(UnsupportedCharacter(current_char, self.position))
            }
        }
    }
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

fn is_valid_identifier(text: char) -> bool {
    IDENTIFIER_REGEX.is_match(&text.to_string())
}