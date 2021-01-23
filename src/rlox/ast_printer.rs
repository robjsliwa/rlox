use super::expr::*;
use super::rlox_errors::RloxError;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AstPrinter {}

impl AstPrinter {
  pub fn print(self, expr: Rc<RefCell<dyn Expr<String>>>) -> Result<String, RloxError> {
    expr.borrow().accept(Rc::new(RefCell::new(self)))
  }

  fn parenthesize_expr(
    &self,
    name: &str,
    expr: Rc<RefCell<dyn Expr<String>>>,
  ) -> Result<String, RloxError> {
    let mut text = String::from("(");
    text.push_str(name);

    text.push_str(" ");
    text.push_str(&expr.borrow().accept(Rc::new(RefCell::new(self.clone())))?);

    text.push_str(")");

    Ok(text)
  }

  fn parenthesize_expr_pair(
    &self,
    name: &str,
    expr_left: Rc<RefCell<dyn Expr<String>>>,
    expr_right: Rc<RefCell<dyn Expr<String>>>,
  ) -> Result<String, RloxError> {
    let mut text = String::from("(");
    text.push_str(name);

    text.push_str(" ");
    text.push_str(
      &expr_left
        .borrow()
        .accept(Rc::new(RefCell::new(self.clone())))?,
    );

    text.push_str(" ");
    text.push_str(
      &expr_right
        .borrow()
        .accept(Rc::new(RefCell::new(self.clone())))?,
    );

    text.push_str(")");

    Ok(text)
  }
}

impl Visitor<String> for AstPrinter {
  fn visit_binary_expr(&self, expr: &Binary<String>) -> Result<String, RloxError> {
    self.parenthesize_expr_pair(&expr.operator.lexeme, expr.left.clone(), expr.right.clone())
  }

  fn visit_grouping_expr(&self, expr: &Grouping<String>) -> Result<String, RloxError> {
    self.parenthesize_expr("group", expr.expression.clone())
  }

  fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<String, RloxError> {
    match &expr.value {
      Some(v) => Ok(v.to_string()),
      None => Err(RloxError::ParserError("missing value".to_string())),
    }
  }

  fn visit_unary_expr(&self, expr: &Unary<String>) -> Result<String, RloxError> {
    self.parenthesize_expr(&expr.operator.lexeme, expr.right.clone())
  }

  fn visit_variable_expr(&self, _: &Variable) -> Result<String, RloxError> {
    // TODO: fix this
    Err(RloxError::ParserError("Not implemented".to_string()))
  }

  fn visit_assign_expr(&self, _: &Assign<String>) -> Result<String, RloxError> {
    // TODO: fix this
    Err(RloxError::ParserError("Not implemented".to_string()))
  }

  fn visit_logical_expr(&self, _: &Logical<String>) -> Result<String, RloxError> {
    Err(RloxError::ParserError("Not implemented".to_string()))
  }

  fn visit_call_expr(&self, _: &Call<String>) -> Result<String, RloxError> {
    Err(RloxError::ParserError("Not implemented".to_string()))
  }

  fn visit_get_expr(&self, _: &Get<String>) -> Result<String, RloxError> {
    Err(RloxError::ParserError("Not implemented".to_string()))
  }

  fn visit_set_expr(&self, _: &Set<String>) -> Result<String, RloxError> {
    Err(RloxError::ParserError("Not implemented".to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rlox::*;

  #[test]
  fn print_simple_ast() -> Result<(), RloxError> {
    let expression = Rc::new(RefCell::new(Binary::new(
      Rc::new(RefCell::new(Unary::new(
        Token::new(TokenType::MINUS, String::from("-"), None, 1),
        Rc::new(RefCell::new(LiteralObj::new(Some(Literal::NumberType(
          123.0,
        ))))),
      ))),
      Token::new(TokenType::STAR, String::from("*"), None, 1),
      Rc::new(RefCell::new(Grouping::new(Rc::new(RefCell::new(
        LiteralObj::new(Some(Literal::NumberType(45.67))),
      ))))),
    )));

    let ast_printer = AstPrinter {};
    let scanned_expression = ast_printer.print(expression)?;
    println!("Scanned expression {:?}", scanned_expression);
    assert_eq!(scanned_expression, "(* (- 123) (group 45.67))");
    Ok(())
  }
}
