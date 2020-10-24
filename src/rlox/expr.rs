use super::literal::*;
use super::token::*;
use std::cell::RefCell;
use std::rc::Rc;

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
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) -> String;
}

pub trait Visitor {
  fn visit_binary_expr(&self, expr: &Binary) -> String;
  fn visit_grouping_expr(&self, expr: &Grouping) -> String;
  fn visit_literal_expr(&self, expr: &LiteralObj) -> String;
  fn visit_unary_expr(&self, expr: &Unary) -> String;
}

// generate_ast!(
//   #[derive(Debug)]
//   struct Binary<V> {
//     left: Box<dyn Expr<V>>,
//     operator: Token,
//     right: Box<dyn Expr>,
//   }
// );

pub struct Binary {
  pub left: Rc<RefCell<dyn Expr>>,
  pub operator: Token,
  pub right: Rc<RefCell<dyn Expr>>,
}

impl Binary {
  pub fn new(left: Rc<RefCell<dyn Expr>>, operator: Token, right: Rc<RefCell<dyn Expr>>) -> Binary {
    Binary {
      left,
      operator,
      right,
    }
  }
}

impl Expr for Binary {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) -> String {
    visitor.borrow().visit_binary_expr(self)
  }
}

pub struct Grouping {
  pub expression: Rc<RefCell<dyn Expr>>,
}

impl Grouping {
  pub fn new(expression: Rc<RefCell<dyn Expr>>) -> Grouping {
    Grouping { expression }
  }
}

impl Expr for Grouping {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) -> String {
    visitor.borrow().visit_grouping_expr(self)
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
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) -> String {
    visitor.borrow().visit_literal_expr(self)
  }
}

pub struct Unary {
  pub operator: Token,
  pub right: Rc<RefCell<dyn Expr>>,
}

impl Unary {
  pub fn new(operator: Token, right: Rc<RefCell<dyn Expr>>) -> Unary {
    Unary { operator, right }
  }
}

impl Expr for Unary {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) -> String {
    visitor.borrow().visit_unary_expr(self)
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
