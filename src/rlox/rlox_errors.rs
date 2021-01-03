use thiserror::Error;
use super::rlox_type::RloxType;

#[derive(Error, Debug)]
pub enum RloxError {
  #[error("Error parsing")]
  ParserError(String),

  #[error("Interpreter error")]
  InterpreterError(String),

  #[error("Return value.")]
  ReturnValue(RloxType),

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}