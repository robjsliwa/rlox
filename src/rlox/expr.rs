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

pub trait Expr<T> {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> T;
}

pub trait Visitor<T> {
  fn visit_binary_expr(&self, expr: &Binary<T>) -> T;
  fn visit_grouping_expr(&self, expr: &Grouping<T>) -> T;
  fn visit_literal_expr(&self, expr: &LiteralObj) -> T;
  fn visit_unary_expr(&self, expr: &Unary<T>) -> T;
}

// generate_ast!(
//   #[derive(Debug)]
//   struct Binary<V> {
//     left: Box<dyn Expr<V>>,
//     operator: Token,
//     right: Box<dyn Expr>,
//   }
// );

pub struct Binary<T> {
  pub left: Rc<RefCell<dyn Expr<T>>>,
  pub operator: Token,
  pub right: Rc<RefCell<dyn Expr<T>>>,
}

impl<T> Binary<T> {
  pub fn new(
    left: Rc<RefCell<dyn Expr<T>>>,
    operator: Token,
    right: Rc<RefCell<dyn Expr<T>>>,
  ) -> Binary<T> {
    Binary {
      left,
      operator,
      right,
    }
  }
}

impl<T> Expr<T> for Binary<T> {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> T {
    visitor.borrow().visit_binary_expr(self)
  }
}

pub struct Grouping<T> {
  pub expression: Rc<RefCell<dyn Expr<T>>>,
}

impl<T> Grouping<T> {
  pub fn new(expression: Rc<RefCell<dyn Expr<T>>>) -> Grouping<T> {
    Grouping { expression }
  }
}

impl<T> Expr<T> for Grouping<T> {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> T {
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

impl<T> Expr<T> for LiteralObj {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> T {
    visitor.borrow().visit_literal_expr(self)
  }
}

pub struct Unary<T> {
  pub operator: Token,
  pub right: Rc<RefCell<dyn Expr<T>>>,
}

impl<T> Unary<T> {
  pub fn new(operator: Token, right: Rc<RefCell<dyn Expr<T>>>) -> Unary<T> {
    Unary { operator, right }
  }
}

impl<T> Expr<T> for Unary<T> {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> T {
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
