use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum Literal {
  StringType(String),
  NumberType(f64),
  BooleanType(bool),
  NullType,
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Literal::StringType(s) => write!(f, "{}", s),
      Literal::NumberType(n) => write!(f, "{}", n),
      Literal::BooleanType(b) => write!(f, "{}", b),
      Literal::NullType => write!(f, "null"),
    }
  }
}
