use super::{
  callable::Callable,
  interpreter::Interpreter,
  rlox_type::RloxType,
  rlox_errors::RloxError,
  rlox_instance::RloxInstance,
  rlox_function::RloxFunction,
};
use std::{
  cell::RefCell,
  rc::Rc,
  collections::HashMap,
};

pub type RloxClassMethods = Rc<RefCell<HashMap<String, RloxFunction>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct RloxClass {
  name: String,
  methods: RloxClassMethods,
}

impl RloxClass {
  pub fn new(name: &str, methods: RloxClassMethods) -> RloxClass {
    RloxClass {
      name: name.to_string(),
      methods,
    }
  }

  pub fn class_name(&self) -> String {
    self.name.clone()
  }

  pub fn find_method(&self, name: &str) -> Result<RloxFunction, RloxError> {
    let methods = self.methods.borrow();
    if methods.contains_key(name) {
      return match methods.get(name) {
        Some(m) => Ok(m.clone()),
        None => Err(RloxError::InterpreterError(format!("Interpreter internal error while looking for method {}.", name))),
      }
    }

    Err(RloxError::InterpreterError(format!("Method {} not found.", name)))
  }
}

impl Callable for RloxClass {
  fn call(&self, interpreter: &Interpreter, arguments: Vec<RloxType>) -> Result<RloxType, RloxError> {
    let instance = RloxInstance::new(self.clone());
    if let Ok(initializer) = self.find_method("init") {
      initializer.bind(&instance).call(interpreter, arguments)?;
    }
    Ok(RloxType::ClassType(instance))
  }

  fn arity(&self) -> usize {
    match self.find_method("init") {
      Ok(initializer) => initializer.arity(),
      Err(_) => 0,
    }
  }

  fn name(&self) -> String {
    self.class_name()
  }
}
