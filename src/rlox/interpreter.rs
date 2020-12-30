use super::{
  expr::*,
  stmt::*,
  rlox_type::*,
  token_type::*,
  literal::*,
  environment::*,
};
use failure::{format_err, Error};
use std::{
  cell::RefCell,
  rc::Rc,
};

type Exp = Rc<RefCell<dyn Expr<RloxType>>>;
type Stm = Rc<RefCell<dyn Stmt<RloxType>>>;

#[derive(Clone)]
pub struct Interpreter {
  environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      environment: Rc::new(RefCell::new(Environment::new())),
    }
  }

  pub fn interpret(&self, statements: Vec<Stm>, callback: Option<fn(resutl: Result<RloxType, Error>)>) {
    for statement in statements {
      let result = self.evaluate_stmt(statement);
      if let Some(f) = callback {
        f(result);
      }
    }
  }

  fn evaluate_expr(&self, expr: Exp) -> Result<RloxType, Error> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn evaluate_stmt(&self, stmt: Stm) -> Result<RloxType, Error> {
    stmt.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn is_truthy(&self, rlox_type: RloxType) -> Result<RloxType, Error> {
    match rlox_type {
      RloxType::NullType => Ok(RloxType::BooleanType(false)),
      RloxType::BooleanType(b) => Ok(RloxType::BooleanType(b)),
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

  fn execute_block(&self, statements: Vec<Stm>, environment: Environment)-> Result<RloxType, Error> {
    let previous = self.environment.replace(environment);

    for statement in statements {
      if let Err(e) = self.evaluate_stmt(statement) {
        self.environment.replace(previous);
        return Err(e);
      }
    }

    self.environment.replace(previous);
    Ok(RloxType::NullType)
  }
}

impl super::stmt::Visitor<RloxType> for Interpreter {
  fn visit_block_stmt(&self, stmt: &Block<RloxType>) -> Result<RloxType, Error> {
    let env = Environment::new_with_parent(self.environment.borrow().clone());
    Ok(self.execute_block(stmt.statements.clone(), env)?)
  }

  fn visit_expression_stmt(&self, stmt: &Expression<RloxType>) -> Result<RloxType, Error> {
    Ok(self.evaluate_expr(stmt.expression.clone())?)
  }

  fn visit_if_stmt(&self, stmt: &If<RloxType>) -> Result<RloxType, Error> {
    if self.is_truthy(self.evaluate_expr(stmt.condition.clone())?)? == Literal::BooleanType(true) {
      self.evaluate_stmt(stmt.then_branch.clone())?;
    } else if let Some(eb) = stmt.else_branch.clone() {
      self.evaluate_stmt(eb)?;
    }

    Ok(RloxType::NullType)
  }

  fn visit_print_stmt(&self, stmt: &Print<RloxType>) -> Result<RloxType, Error> {
    let value = self.evaluate_expr(stmt.expression.clone())?;
    println!("{}", value);
    Ok(RloxType::NullType)
  }

  fn visit_var_stmt(&self, stmt: &Var<RloxType>) -> Result<RloxType, Error> {
    let value = self.evaluate_expr(stmt.initializer.clone())?;

    self.environment.borrow().define(stmt.name.lexeme.clone(), value);

    Ok(RloxType::NullType)
  }
}

impl super::expr::Visitor<RloxType> for Interpreter {
  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> Result<RloxType, Error> {
    let left = self.evaluate_expr(expr.left.clone())?;
    let right = self.evaluate_expr(expr.right.clone())?;

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
    let right = self.evaluate_expr(expr.right.clone())?;

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

  fn visit_variable_expr(&self, expr: &Variable) -> Result<RloxType, Error> {
    self.environment.borrow().get(&expr.name.lexeme)
  }

  fn visit_assign_expr(&self, expr: &Assign<RloxType>) -> Result<RloxType, Error> {
    let value = self.evaluate_expr(expr.value.clone())?;
    self.environment.borrow().assign(&expr.name.lexeme, value.clone())?;
    Ok(value)
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
    let statements = parser.parse()?;
    let interpreter = Interpreter::new();

    let mut final_result = RloxType::NullType;

    for statement in statements {
      final_result = interpreter.evaluate_stmt(statement.clone())?;
    }

    Ok(final_result)
  }

  #[test]
  fn test_basic_arithmetic() -> Result<(), Error> {
    let test_input: HashMap<&str, f64> = [
      ("1 + 2;", 3.0),
      ("-1 + 2;", 1.0),
      ("-1 + -2;", -3.0),
      ("5+5;", 10.0),
      ("25 - 1;", 24.0),
      ("-3 - 3;", -6.0),
      ("-3 - -3;", 0.0),
      ("5*5;", 25.0),
      ("25 /5;", 5.0),
      ("1 - 4 * 4;", -15.0),
      ("25 / 5 + 2 * 4;", 13.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_truthiness() -> Result<(), Error> {
    let test_input: HashMap<&str, bool> = [
      ("1 < 2;", true),
      ("-1 <= 2;", true),
      ("25 >= 25;", true),
      ("5 > 5;", false),
      ("-25 > 1;", false),
      ("-3 != 3;", true),
      ("-3 == -3;", true),
      ("5==5;", true),
      ("25 >= 5;", true),
      ("1 - 4 > 4;", false),
      ("25 / 5 == 2 * 4 - 3;", true),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::BooleanType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_global_vars() -> Result<(), Error> {
    let test_input: HashMap<&str, f64> = [
      ("var t=5; t;", 5.0),
      ("var t=5; t=t+1; t;", 6.0),
      ("var p=5; p=10; p;", 10.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_var_scopes() -> Result<(), Error> {
    let test_input: HashMap<&str, f64> = [
      ("var t=5; t; {var p=10; t=p;} t;", 10.0),
      ("var k=1; {var k=10; k=k+1;} k;", 1.0),
      ("var s=5; {var d=10; d=d+5; s=s+d;} s;", 20.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }
}