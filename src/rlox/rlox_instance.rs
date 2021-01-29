use std::{
  collections::HashMap,
  cell::RefCell,
  rc::Rc,
};
use super::{
  rlox_class::RloxClass,
  rlox_type::RloxType,
  rlox_errors::RloxError,
  token::Token,
  expr::*,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RloxInstance {
  klass: RloxClass,
  fields: Rc<RefCell<HashMap<String, RloxType>>>,
}

impl RloxInstance {
  pub fn new(klass: RloxClass) -> RloxInstance {
    RloxInstance {
      klass,
      fields: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn as_string(&self) -> String {
    format!("{} instance", self.klass.class_name())
  }

  pub fn get(&self, name: &Token) -> Result<RloxType, RloxError> {
    let fields = self.fields.borrow();
    if fields.contains_key(&name.lexeme) {
      return match fields.get(&name.lexeme) {
        Some(v) => Ok(v.clone()),
        None => Err(RloxError::InterpreterError(format!("Interpreter internal error while looking for field {}.", &name.lexeme)))
      }
    }

    let method = self.klass.find_method(&name.lexeme)?;
    Ok(RloxType::CallableType(Box::new(method)))
  }

  pub fn set(&self, name: &Token, value: &RloxType) -> Result<(), RloxError> {
    let mut fields = self.fields.borrow_mut();
    fields.insert(name.lexeme.clone(), value.clone());
    Ok(())
  }
}
