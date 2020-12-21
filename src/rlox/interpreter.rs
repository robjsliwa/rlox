use super::{expr::*, rlox_type::*, token_type::*, literal::*};
use failure::{format_err, Error};
use std::cell::RefCell;
use std::rc::Rc;

type Expression = Rc<RefCell<dyn Expr<RloxType>>>;

#[derive(Debug, Clone)]
pub struct Interpreter {}

impl Interpreter {
  pub fn interpret(expression: Expression) -> Result<RloxType, Error> {
    let interpreter = Interpreter {};
    interpreter.evaluate(expression)
  }

  fn evaluate(&self, expr: Expression) -> Result<RloxType, Error> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn is_truthy(&self, rlox_type: RloxType) -> Result<RloxType, Error> {
    match rlox_type {
      RloxType::NullType => Ok(RloxType::BooleanType(false)),
      RloxType::BooleanType(b) => Ok(RloxType::BooleanType(!b)),
      _ => Ok(RloxType::BooleanType(true)),
    }
  }

  fn is_equal(&self, left: Literal, right: Literal) -> Result<RloxType, Error> {
    if left == Literal::NullType && right == Literal::NullType {
      return Ok(RloxType::BooleanType(true));
    }
    if left == Literal::NullType {
      return Ok(RloxType::BooleanType(false));
    }

    Ok(RloxType::BooleanType(left == right))
  }

  fn not(&self, b: RloxType) -> Result<RloxType, Error> {
    if let RloxType::BooleanType(b) = b {
      return Ok(RloxType::BooleanType(!b));
    }

    Err(format_err!("invalid type: expected boolean"))
  }

  fn compute_binary_operand(&self, token_type: &TokenType, left: Literal, right: Literal) -> Result<RloxType, Error> {
    if let RloxType::NumberType(left_number) = left {
      if let RloxType::NumberType(right_number) = right {
        return match token_type {
          TokenType::MINUS => Ok(RloxType::NumberType(left_number - right_number)),
          TokenType::PLUS => Ok(RloxType::NumberType(left_number + right_number)),
          TokenType::SLASH => Ok(RloxType::NumberType(left_number / right_number)),
          TokenType::STAR => Ok(RloxType::NumberType(left_number * right_number)),
          TokenType::GREATER => Ok(RloxType::BooleanType(left_number > right_number)),
          TokenType::GREATEREQUAL => Ok(RloxType::BooleanType(left_number >= right_number)),
          TokenType::LESS => Ok(RloxType::BooleanType(left_number < right_number)),
          TokenType::LESSEQUAL => Ok(RloxType::BooleanType(left_number <= right_number)),
          TokenType::BANGEQUAL => self.not(self.is_equal(left, right)?),
          TokenType::EQUALEQUAL => self.is_equal(left, right),
          _ => Err(format_err!("unimplemented operand {}", token_type.name())),
        }
      }
    }

    if let RloxType::StringType(left_number) = left {
      if let RloxType::StringType(right_number) = right {
        return match token_type {
          TokenType::PLUS => Ok(RloxType::StringType(format!("{}{}", left_number, right_number))),
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rlox::*;
  use crate::scanners::Scanner;
  use std::collections::HashMap;

  fn run(input: &str) -> Result<RloxType, Error> {
    let data = input.chars().collect();

    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let expression = parser.parse()?;
    let val = Interpreter::interpret(expression)?;
    Ok(val)
  }

  #[test]
  fn test_basic_arithmetic() -> Result<(), Error> {
    let test_input: HashMap<&str, f64> = [
      ("1 + 2", 3.0),
      ("-1 + 2", 1.0),
      ("-1 + -2", -3.0),
      ("5+5", 10.0),
      ("25 - 1", 24.0),
      ("-3 - 3", -6.0),
      ("-3 - -3", 0.0),
      ("5*5", 25.0),
      ("25 /5", 5.0),
      ("1 - 4 * 4", -15.0),
      ("25 / 5 + 2 * 4", 13.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val, RloxType::NumberType(expected_result));
    }

    Ok(())
  }

  #[test]
  fn test_truthiness() -> Result<(), Error> {
    let test_input: HashMap<&str, bool> = [
      ("1 < 2", true),
      ("-1 <= 2", true),
      ("25 >= 25", true),
      ("5 > 5", false),
      ("-25 > 1", false),
      ("-3 != 3", true),
      ("-3 == -3", true),
      ("5==5", true),
      ("25 >= 5", true),
      ("1 - 4 > 4", false),
      ("25 / 5 == 2 * 4 - 3", true),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val, RloxType::BooleanType(expected_result));
    }

    Ok(())
  }
}