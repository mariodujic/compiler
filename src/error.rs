use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum CompilerError {
    #[error("Unsupported character '{0}' at position {1}")]
    UnsupportedCharacter(char, usize),
    #[error("Invalid variable declaration '{0}'")]
    UndeclaredVariable(String),
    #[error("Trying to assign new value to immutable variable '{0}'")]
    ImmutableVariable(String),
    #[error("Unable to assign {0} to '{1}' as {2} is required")]
    IncompatibleVariableType(String, String, String)
}