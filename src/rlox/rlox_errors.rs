use thiserror::Error;

#[derive(Error, Debug)]
pub enum RloxError {
  #[error("Error parsing")]
  ParserError(String),

  #[error("Interpreter error")]
  InterpreterError(String),

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}