use crate::error::CompilerError;
use crate::error::CompilerError::{ImmutableVariable, IncompatibleVariableType, UndeclaredVariable};
use crate::lexer::Lexer;
use crate::symbol::{Symbol, SymbolTable, Value};
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Result<Token, CompilerError>,
    symbol_table: SymbolTable,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
            symbol_table: SymbolTable::new(),
        }
    }

    fn eat(&mut self, token: Token) {
        match &self.current_token {
            Ok(t) if *t == token => self.current_token = self.lexer.get_next_token(),
            _ => panic!("Unsupported token result")
        }
    }

    fn factor(&mut self) -> Result<i32, CompilerError> {
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

    fn parse_number(&mut self) -> Result<i32, CompilerError> {
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

    fn parse_string_literal(&mut self) -> Result<Box<str>, CompilerError> {
        while let Ok(token) = self.current_token.clone() {
            match token {
                Token::StringLiteral(s) => {
                    self.eat(Token::StringLiteral(s.clone()));
                    return Ok(s);
                }
                _ => break,
            }
        }
        panic!("Invalid String literal")
    }

    fn parse_identifier(&mut self) -> Result<Box<str>, CompilerError> {
        match self.current_token.clone() {
            Ok(Token::Identifier(value)) => {
                self.eat(Token::Identifier(value.clone()));
                Ok(value)
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to parse identifier"),
        }
    }

    fn parse_mutability(&mut self) -> Result<bool, CompilerError> {
        match self.current_token.clone() {
            Ok(Token::Mutable) => {
                self.eat(Token::Mutable);
                Ok(true)
            }
            Ok(Token::Immutable) => {
                self.eat(Token::Immutable);
                Ok(false)
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to parse mutability"),
        }
    }

    fn parse_assignment(&mut self) -> Result<(), CompilerError> {
        match self.current_token.clone() {
            Ok(Token::AssignmentOperator) => {
                self.eat(Token::AssignmentOperator);
                Ok(())
            }
            Err(error) => Err(error.clone()),
            _ => panic!("Unable to parse assignment operator"),
        }
    }

    fn expr(&mut self) -> Result<i32, CompilerError> {
        let mut result = self.parse_number().unwrap_or(0);
        while self.current_token != Ok(Token::EOF) {
            match &self.current_token {
                Ok(res) => {
                    match res {
                        Token::Plus => {
                            self.eat(Token::Plus);
                            result += self.parse_number()?;
                        }
                        Token::Minus => {
                            self.eat(Token::Minus);
                            result -= self.parse_number()?;
                        }
                        Token::Mutable | Token::Immutable => {
                            let mutable = self.parse_mutability()?;
                            let identifier = self.parse_identifier()?;
                            self.parse_assignment()?;
                            match &self.current_token {
                                Ok(res) => {
                                    match res {
                                        Token::OpenParenthesis | Token::Number(_) => {
                                            let value = self.parse_number()?;
                                            let symbol = Symbol::new(identifier, Value::Int(value), mutable);
                                            self.symbol_table.add(symbol);
                                        }
                                        Token::StringLiteral(_) => {
                                            let value = self.parse_string_literal()?;
                                            let symbol = Symbol::new(identifier, Value::String(value), mutable);
                                            self.symbol_table.add(symbol);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        Token::Identifier(_) => {
                            let identifier = self.parse_identifier()?;
                            self.parse_assignment()?;
                            match &self.current_token {
                                Ok(res) => {
                                    match res {
                                        Token::OpenParenthesis | Token::Number(_) => {
                                            let value = self.parse_number()?;
                                            let symbol = self.symbol_table.get(&identifier);
                                            if symbol.is_some() {
                                                let mut symbol = symbol.unwrap().clone();

                                                if let Value::Int(_) = symbol.value {} else {
                                                    return Err(
                                                        IncompatibleVariableType(
                                                            value.to_string(),
                                                            identifier.to_string(),
                                                            symbol.value.to_string(),
                                                        )
                                                    );
                                                }

                                                let mutable_variable = symbol.mutable;
                                                if !mutable_variable {
                                                    return Err(ImmutableVariable((*identifier).to_string()));
                                                }
                                                symbol.value = Value::Int(value);
                                                self.symbol_table.replace_with_same_name(symbol);
                                            } else {
                                                return Err(UndeclaredVariable((*identifier).to_string()));
                                            }
                                        }
                                        Token::StringLiteral(_) => {
                                            let value = self.parse_string_literal()?;
                                            let symbol = self.symbol_table.get(&identifier);
                                            if symbol.is_some() {
                                                let mut symbol = symbol.unwrap().clone();
                                                let mutable_variable = symbol.mutable;
                                                if !mutable_variable {
                                                    return Err(ImmutableVariable((*identifier).to_string()));
                                                }
                                                if let Value::String(_) = symbol.value {} else {
                                                    return Err(
                                                        IncompatibleVariableType(
                                                            value.to_string(),
                                                            identifier.to_string(),
                                                            symbol.value.to_string(),
                                                        )
                                                    );
                                                }
                                                symbol.value = Value::String(value);
                                                self.symbol_table.replace_with_same_name(symbol);
                                            } else {
                                                return Err(UndeclaredVariable((*identifier).to_string()));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => break,
                    }
                }
                Err(err) => return Err(err.clone())
            }
        }
        Ok(result)
    }

    pub fn get_symbol_table(&mut self) -> Result<SymbolTable, CompilerError> {
        self.expr()?;
        return Ok(self.symbol_table.clone());
    }
}
