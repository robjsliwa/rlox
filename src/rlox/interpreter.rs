use super::{expr::*, rlox_type::*, token_type::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {}

impl Interpreter {
  fn evaluate(&self, expr: Rc<RefCell<dyn Expr<RloxType>>>) -> RloxType {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }
}

impl Visitor<RloxType> for Interpreter {
  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> RloxType {
    RloxType::NullType
  }

  fn visit_grouping_expr(&self, expr: &Grouping<RloxType>) -> RloxType {
    RloxType::NullType
  }

  fn visit_literal_expr(&self, expr: &LiteralObj) -> RloxType {
    match &expr.value {
      Some(v) => v.clone(),
      None => RloxType::NullType,
    }
  }

  fn visit_unary_expr(&self, expr: &Unary<RloxType>) -> RloxType {
    let right = self.evaluate(expr.right.clone());

    match expr.operator.token_type {
      TokenType::MINUS => RloxType::NumberType(0.0),
      _ => RloxType::NullType,
    }
  }
}
