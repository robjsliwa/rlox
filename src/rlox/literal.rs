use std::fmt::{Display, Formatter, Result};
use super::{
  callable::Callable,
  rlox_class::RloxClass,
};

#[derive(Clone, Debug)]
pub enum Literal {
  StringType(String),
  NumberType(f64),
  BooleanType(bool),
  CallableType(Box<dyn Callable>),
  ClassType(RloxClass),
  NullType,
}

impl PartialEq for Literal {
  fn eq(&self, other: &Self) -> bool {
      match (self, other) {
          (Literal::StringType(sl), Literal::StringType(sr)) => sl == sr,
          (Literal::NumberType(nl), Literal::NumberType(nr)) => nl == nr,
          (Literal::BooleanType(bl), Literal::BooleanType(br)) => bl == br,
          (Literal::CallableType(cl), Literal::CallableType(cr)) => cl == cr,
          (Literal::ClassType(kl), Literal::ClassType(kr)) => kl == kr,
          (Literal::NullType, Literal::NullType) => true,
          (_, _) => false,
      }
  }
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Literal::StringType(s) => write!(f, "{}", s),
      Literal::NumberType(n) => write!(f, "{}", n),
      Literal::BooleanType(b) => write!(f, "{}", b),
      Literal::CallableType(_) => write!(f, "callable"),
      Literal::ClassType(k) => write!(f, "Class {}", k.name()),
      Literal::NullType => write!(f, "null"),
    }
  }
}
