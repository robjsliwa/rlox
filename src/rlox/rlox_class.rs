use super::{
  callable::Callable,
  interpreter::Interpreter,
  rlox_type::RloxType,
  rlox_errors::RloxError,
  rlox_instance::RloxInstance,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RloxClass {
  name: String,
}

impl RloxClass {
  pub fn new(name: &str) -> RloxClass {
    RloxClass {
      name: name.to_string(),
    }
  }

  pub fn class_name(&self) -> String {
    self.name.clone()
  }
}

impl Callable for RloxClass {
  fn call(&self, interpreter: &Interpreter, arguments: Vec<RloxType>) -> Result<RloxType, RloxError> {
    let instance = RloxInstance::new(self.clone());
    Ok(RloxType::ClassType(instance))
  }

  fn arity(&self) -> usize {
    0
  }

  fn name(&self) -> String {
    self.class_name()
  }
}
