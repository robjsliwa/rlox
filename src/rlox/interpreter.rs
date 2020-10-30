use super::{expr::*, rlox_type::*, token_type::*};
use failure::{format_err, Error};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {}

impl Interpreter {
  fn evaluate(&self, expr: Rc<RefCell<dyn Expr<RloxType>>>) -> Result<RloxType, Error> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }
}

impl Visitor<RloxType> for Interpreter {
  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> Result<RloxType, Error> {
    Ok(RloxType::NullType)
  }

  fn visit_grouping_expr(&self, expr: &Grouping<RloxType>) -> Result<RloxType, Error> {
    Ok(RloxType::NullType)
  }

  fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<RloxType, Error> {
    match &expr.value {
      Some(v) => Ok(v.clone()),
      None => Err(format_err!("missing value")),
    }
  }

  fn visit_unary_expr(&self, expr: &Unary<RloxType>) -> Result<RloxType, Error> {
    let right = self.evaluate(expr.right.clone());

    match expr.operator.token_type {
      TokenType::MINUS => Ok(RloxType::NumberType(0.0)),
      _ => Err(format_err!("unsupported operation")),
    }
  }
}
