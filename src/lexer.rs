#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
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

    pub(crate) fn get_next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.input[self.position];

        if current_char.is_digit(10) {
            let mut num_str = String::new();
            while self.position < self.input.len() && self.input[self.position].is_digit(10) {
                num_str.push(self.input[self.position]);
                self.advance();
            }
            Some(Token::Number(num_str.parse().unwrap()))
        } else if current_char.is_whitespace() {
            self.advance();
            self.get_next_token()
        } else {
            match current_char {
                '+' => {
                    self.advance();
                    Some(Token::Plus)
                }
                '-' => {
                    self.advance();
                    Some(Token::Minus)
                }
                '*' => {
                    self.advance();
                    Some(Token::Multiply)
                }
                '/' => {
                    self.advance();
                    Some(Token::Divide)
                }
                _ => panic!("Unsupported character {:?}", current_char),
            }
        }
    }
}