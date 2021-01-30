use std::rc::Rc;
use std::cell::RefCell;
use super::{
  stmt::*,
  interpreter::Interpreter,
  callable::Callable,
  rlox_type::RloxType,
  environment::Environment,
  rlox_errors::RloxError,
  rlox_instance::RloxInstance,
};

#[derive(Clone)]
pub struct RloxFunction {
  declaration: Rc<Function<RloxType>>,
  closure: Rc<RefCell<Environment>>,
}

impl RloxFunction {
  pub fn new(decl: &Function<RloxType>, closure: &Environment) -> RloxFunction {
    let new_declaration = Function::new(decl.name.clone(), decl.params.clone(), decl.body.clone());
    RloxFunction {
      declaration: Rc::new(new_declaration),
      // This is the environment that is active when
      // the function is declared not when it’s called,
      // which is what we want.
      closure: Rc::new(RefCell::new(closure.clone())),
    }
  }

  pub fn bind(&self, instance: &RloxInstance) -> RloxFunction {
    let environment = Environment::new_with_parent(self.closure.borrow().clone());
    environment.define("this".to_string(), RloxType::ClassType(instance.clone()));
    RloxFunction {
      declaration: self.declaration.clone(),
      closure: Rc::new(RefCell::new(environment)),
    }
  }
}

impl Callable for RloxFunction {
  fn call(&self, interpreter: &Interpreter, arguments: Vec<RloxType>) -> Result<RloxType, RloxError> {
    let env = Environment::new_with_parent(self.closure.borrow().clone());
    for (i, param) in self.declaration.params.iter().enumerate() {
      env.define(param.lexeme.clone(), arguments.get(i).unwrap().clone());
    }

    match interpreter.execute_block(self.declaration.body.clone(), env) {
      Ok(r) => Ok(r),
      Err(e) => {
        match e {
          RloxError::ReturnValue(v) => Ok(v),
          _ => Err(e),
        }
      }
    }
  }

  fn arity(&self) -> usize {
    self.declaration.params.len()
  }

  fn name(&self) -> String {
    format!("<fn {} >", self.declaration.name.lexeme)
  }
}

impl std::fmt::Debug for RloxFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Function {}", self.name())
  }
}

impl PartialEq for RloxFunction {
  fn eq(&self, other: &Self) -> bool {
      self.name() == other.name()
  }
}
