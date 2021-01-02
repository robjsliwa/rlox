use std::rc::Rc;
use super::{
  stmt::*,
  interpreter::Interpreter,
  callable::Callable,
  rlox_type::RloxType,
  environment::Environment,
  rlox_errors::RloxError,
};

#[derive(Clone)]
pub struct RloxFunction {
  declaration: Rc<Function<RloxType>>,
}

impl RloxFunction {
  pub fn new(decl: &Function<RloxType>) -> RloxFunction {
    let new_declaration = Function::new(decl.name.clone(), decl.params.clone(), decl.body.clone());
    RloxFunction {
      declaration: Rc::new(new_declaration),
    }
  }
}

impl Callable for RloxFunction {
  fn call(&self, interpreter: &Interpreter, arguments: Vec<RloxType>) -> Result<RloxType, RloxError> {
    let env = Environment::new_with_parent(interpreter.globals.borrow().clone());
    
    for (i, param) in self.declaration.params.iter().enumerate() {
      env.define(param.lexeme.clone(), arguments.get(i).unwrap().clone());
    }

    interpreter.execute_block(self.declaration.body.clone(), env)
  }

  fn arity(&self) -> usize {
    self.declaration.params.len()
  }

  fn name(&self) -> String {
    format!("<fn {} >", self.declaration.name.lexeme)
  }
}