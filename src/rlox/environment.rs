use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::{
  rlox_type::*,
  native_functions::*,
  rlox_errors::RloxError,
};

#[derive(Debug, Clone)]
pub struct Environment {
  values: Rc<RefCell<HashMap<String, RloxType>>>,
  enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub fn new() -> Environment {
    let mut env_map = HashMap::new();

    env_map.insert("clock".to_string(), RloxType::CallableType(Box::new(Clock::new())));

    Environment {
      values: Rc::new(RefCell::new(env_map)),
      enclosing: None,
    }
  }

  pub fn new_with_parent(enclosing: Environment)-> Environment {
    Environment {
      values: Rc::new(RefCell::new(HashMap::new())),
      enclosing: Some(Rc::new(RefCell::new(enclosing))),
    }
  }

  pub fn define(&self, name: String, expr: RloxType) {
    self.values.borrow_mut().insert(name, expr);
  }

  pub fn get(&self, name: &str) -> Result<RloxType, RloxError> {
    match self.values.borrow().get(name) {
      Some(v) => Ok(v.clone()),
      None => {
        if let Some(encl) = &self.enclosing {
          return encl.borrow().get(name);
        }

        Err(RloxError::InterpreterError(format!("Undefined variable '{}'.", name)))
      }
    }
  }

  pub fn assign(&self, name: &str, value: RloxType) -> Result<(), RloxError> {
    let mut values = self.values.borrow_mut();
    match values.get(name) {
      Some(_) => {
        values.insert(name.to_string(), value);
        Ok(())
      }
      None => {
        if let Some(encl) = &self.enclosing {
          return encl.borrow().assign(name, value);
        }

        Err(RloxError::InterpreterError(format!("Undefined variable '{}'.", name)))
      }
    }
  }

  pub fn get_at(&self, distance: usize, name: &str) -> Result<RloxType, RloxError> {
    match self.ancestor(distance) {
      Some(env) => match env.values.borrow().get(name) {
        Some(v) => Ok(v.clone()),
        None => Err(RloxError::InterpreterError(format!("Variable {} not found in environment.", name)))
      }
      None => Err(RloxError::InterpreterError("Internal interpreter error, invalid environment distance.".to_string())),
    }
  }

  fn ancestor(&self, distance: usize) -> Option<Environment> {
    let mut current_env = self.clone();

    for _ in 0..distance {
      current_env = match current_env.enclosing {
        Some(e) => e.borrow().clone(),
        None => return None,
      }
    }

    Some(current_env)
  }
}
