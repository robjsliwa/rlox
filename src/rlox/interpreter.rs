use super::{expr::*, rlox_type::*, token_type::*, literal::*};
use failure::{format_err, Error};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {}

impl Interpreter {
  fn evaluate(&self, expr: Rc<RefCell<dyn Expr<RloxType>>>) -> Result<RloxType, Error> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn is_truthy(&self, rlox_type: RloxType) -> Result<RloxType, Error> {
    match rlox_type {
      RloxType::NullType => Ok(RloxType::BooleanType(false)),
      RloxType::BooleanType(b) => Ok(RloxType::BooleanType(!b)),
      _ => Ok(RloxType::BooleanType(true)),
    }
  }

  fn compute_binary_operand(&self, token_type: &TokenType, left: Literal, right: Literal) -> Result<RloxType, Error> {
    if let RloxType::NumberType(left_number) = left {
      if let RloxType::NumberType(right_number) = right {
        return match token_type {
          TokenType::MINUS => Ok(RloxType::NumberType(left_number - right_number)),
          TokenType::PLUS => Ok(RloxType::NumberType(left_number + right_number)),
          TokenType::SLASH => Ok(RloxType::NumberType(left_number / right_number)),
          TokenType::STAR => Ok(RloxType::NumberType(left_number * right_number)),
          _ => Err(format_err!("unimplemented operand {}", token_type.name())),
        }
      }
    }

    if let RloxType::StringType(left_number) = left {
      if let RloxType::StringType(right_number) = right {
        return match token_type {
          TokenType::PLUS => Ok(RloxType::NumberType(format!("{}{}", left_number, right_number))),
          _ => Err(format_err!("unsupported operand type(s) for {}: both operand types must be string", token_type.name())),
        }
      }
    }

    return Err(format_err!("unsupported operand type(s) for {}: both operand types must be number", token_type.name()));
  }
}

impl Visitor<RloxType> for Interpreter {
  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> Result<RloxType, Error> {
    let left = self.evaluate(expr.left.clone())?;
    let right = self.evaluate(expr.right.clone())?;

    self.compute_binary_operand(&expr.operator.token_type, left, right)
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
    let right = self.evaluate(expr.right.clone())?;

    match expr.operator.token_type {
      TokenType::MINUS => {
        if let RloxType::NumberType(n) = right {
          return Ok(RloxType::NumberType(-1.0 * n));
        }
        return Err(format_err!("Invalid type"));
      }
      TokenType::BANG => self.is_truthy(right),
      _ => Err(format_err!("unsupported operand")),
    }
  }
}
