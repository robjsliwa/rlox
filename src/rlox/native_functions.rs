use super::{
  callable::Callable,
  interpreter::Interpreter,
  rlox_type::RloxType,
};
use failure::Error;
use chrono;

#[derive(Clone)]
pub struct Clock {}

impl Clock {
  pub fn new() -> Clock {
    Clock {}
  }
}

impl Callable for Clock {
  fn arity(&self) -> usize {
    0
  }

  fn call(&self, _interpreter: &Interpreter, _arguments: Vec<RloxType>) -> Result<RloxType, Error> {
    Ok(RloxType::NumberType(chrono::offset::Utc::now().timestamp() as f64))
  }

  fn name(&self) -> String {
    String::from("<native function> clock")
  }
}