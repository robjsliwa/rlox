use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use failure::{Error, format_err};
use super::literal::*;

#[derive(Debug, Clone)]
pub struct Environment {
  values: Rc<RefCell<HashMap<String, Literal>>>,
  enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      values: Rc::new(RefCell::new(HashMap::new())),
      enclosing: None,
    }
  }

  pub fn new_with_parent(enclosing: Environment)-> Environment {
    Environment {
      values: Rc::new(RefCell::new(HashMap::new())),
      enclosing: Some(Rc::new(RefCell::new(enclosing))),
    }
  }

  pub fn define(&self, name: String, expr: Literal) {
    self.values.borrow_mut().insert(name, expr);
  }

  pub fn get(&self, name: &str) -> Result<Literal, Error> {
    match self.values.borrow().get(name) {
      Some(v) => Ok(v.clone()),
      None => {
        if let Some(encl) = &self.enclosing {
          return encl.borrow().get(name);
        }

        Err(format_err!("Undefined variable '{}'.", name))
      }
    }
  }

  pub fn assign(&self, name: &str, value: Literal) -> Result<(), Error> {
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

        Err(format_err!("Undefined variable '{}'.", name))
      }
    }
  }
}