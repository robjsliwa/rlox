use super::expr::*;

pub struct AstPrinter {}

impl AstPrinter {
  pub fn print<E: Expr>(&mut self, expr: E) -> String {
    expr.accept(self)
  }

  fn parenthesize_expr<E: Expr>(&mut self, name: &str, expr: &E) -> String {
    let mut text = String::from("(");
    text.push_str(name);

    text.push_str(" ");
    text.push_str(&expr.accept(self));

    text.push_str(")");

    text
  }

  fn parenthesize_expr_pair<E: Expr, R: Expr>(
    &mut self,
    name: &str,
    expr_left: &E,
    expr_right: &R,
  ) -> String {
    let mut text = String::from("(");
    text.push_str(name);

    text.push_str(" ");
    text.push_str(&expr_left.accept(self));

    text.push_str(" ");
    text.push_str(&expr_right.accept(self));

    text.push_str(")");

    text
  }
}

impl Visitor for AstPrinter {
  type Result = String;

  fn visit_binary_expr<E: Expr, R: Expr>(&mut self, expr: &Binary<E, R>) -> Self::Result {
    self.parenthesize_expr_pair(&expr.operator.lexeme, &expr.left, &expr.right)
  }

  fn visit_grouping_expr<E: Expr>(&mut self, expr: &Grouping<E>) -> Self::Result {
    self.parenthesize_expr("group", &expr.expression)
  }

  fn visit_literal_expr(&mut self, expr: &LiteralObj) -> Self::Result {
    match &expr.value {
      Some(v) => v.to_string(),
      None => String::from("nil"),
    }
  }

  fn visit_unary_expr<E: Expr>(&mut self, expr: &Unary<E>) -> Self::Result {
    self.parenthesize_expr(&expr.operator.lexeme, &expr.right)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rlox::*;

  #[test]
  fn print_simple_ast() {
    let expression = Binary::new(
      Unary::new(
        Token::new(TokenType::MINUS, String::from("-"), None, 1),
        LiteralObj::new(Some(Literal::NumberType(123.0))),
      ),
      Token::new(TokenType::STAR, String::from("*"), None, 1),
      Grouping::new(LiteralObj::new(Some(Literal::NumberType(45.67)))),
    );

    let mut ast_printer = AstPrinter {};
    let scanned_expression = ast_printer.print(expression);
    println!("Scanned expression {:?}", scanned_expression);
    assert_eq!(scanned_expression, "(* (- 123) (group 45.67))");
  }
}
