use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum Literal {
  StringType(String),
  NumberType(f64),
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Literal::StringType(s) => write!(f, "{}", s),
      Literal::NumberType(n) => write!(f, "{}", n),
    }
  }
}
