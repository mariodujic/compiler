#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Identifier(Box<str>),
    Immutable,
    Mutable,
    AssignmentOperator,
    StringLiteral(Box<str>),
    EOF,
}