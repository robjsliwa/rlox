use super::literal::*;
use super::token::*;

#[macro_export]
macro_rules! generate_ast {
  ($(#[$attr:meta])* struct $name:ident { $($field:ident : $ftype:ty),* $(,)? }) => {
    $(#[$attr])*
    pub struct $name {
      $( $field: $ftype, )*
    }

    impl $name {
      pub fn new(
        $( $field:$ftype, )*
      ) -> $name {
        $name {
          $( $field, )*
        }
      }
    }
  };
}

pub trait Expr {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait Visitor {
  type Result;

  fn visit_binary_expr<E: Expr, R: Expr>(&mut self, expr: &Binary<E, R>) -> Self::Result;
  fn visit_grouping_expr<E: Expr>(&mut self, expr: &Grouping<E>) -> Self::Result;
  fn visit_literal_expr(&mut self, expr: &LiteralObj) -> Self::Result;
  fn visit_unary_expr<E: Expr>(&mut self, expr: &Unary<E>) -> Self::Result;
}

// generate_ast!(
//   #[derive(Debug)]
//   struct Binary<V> {
//     left: Box<dyn Expr<V>>,
//     operator: Token,
//     right: Box<dyn Expr>,
//   }
// );

pub struct Binary<E: Expr, R: Expr> {
  pub left: E,
  pub operator: Token,
  pub right: R,
}

impl<E: Expr, R: Expr> Binary<E, R> {
  pub fn new(left: E, operator: Token, right: R) -> Binary<E, R> {
    Binary {
      left,
      operator,
      right,
    }
  }
}

impl<E: Expr, R: Expr> Expr for Binary<E, R> {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
    visitor.visit_binary_expr(self)
  }
}

pub struct Grouping<E: Expr> {
  pub expression: E,
}

impl<E: Expr> Grouping<E> {
  pub fn new(expression: E) -> Grouping<E> {
    Grouping { expression }
  }
}

impl<E: Expr> Expr for Grouping<E> {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
    visitor.visit_grouping_expr(self)
  }
}

pub struct LiteralObj {
  pub value: Option<Literal>,
}

impl LiteralObj {
  pub fn new(value: Option<Literal>) -> LiteralObj {
    LiteralObj { value }
  }
}

impl Expr for LiteralObj {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
    visitor.visit_literal_expr(self)
  }
}

pub struct Unary<E: Expr> {
  pub operator: Token,
  pub right: E,
}

impl<E: Expr> Unary<E> {
  pub fn new(operator: Token, right: E) -> Unary<E> {
    Unary { operator, right }
  }
}

impl<E: Expr> Expr for Unary<E> {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
    visitor.visit_unary_expr(self)
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  generate_ast!(
    #[derive(Debug)]
    struct Stmt {
      a: String,
      b: bool,
      c: u64,
    }
  );

  #[test]
  fn create_via_new() {
    let stmt = Stmt::new(String::from("Howdy"), true, 10);
    println!("stmt {:?}", stmt);

    assert_eq!(stmt.a, String::from("Howdy"));
    assert_eq!(stmt.b, true);
    assert_eq!(stmt.c, 10);
  }
}
