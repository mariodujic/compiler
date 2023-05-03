use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum CompilerError {
    #[error("Unsupported character '{0}' at position {1}")]
    UnsupportedCharacter(char, usize),
    #[error("Invalid variable declaration '{0}'")]
    UndeclaredVariable(String),
    #[error("Trying to assign new value to immutable variable '{0}'")]
    ImmutableVariable(String),
}