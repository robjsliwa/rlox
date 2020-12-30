use super::literal::*;
use super::token::*;
use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;
use crate::generate_ast;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

pub type Exp<T> = Rc<RefCell<dyn Expr<T>>>;

generate_ast! {
  Expr {
    visit_assign_expr Assign T => name: Token, value: Exp<T>;
    visit_binary_expr Binary T => left: Exp<T>, operator: Token, right: Exp<T>;
    visit_grouping_expr Grouping T => expression: Exp<T>;
    visit_literal_expr LiteralObj => value: Option<Literal>;
    visit_logical_expr Logical T => left: Exp<T>, operator: Token, right: Exp<T>;
    visit_unary_expr Unary T => operator: Token, right: Exp<T>;
    visit_variable_expr Variable => name: Token;
  }
}

// Above generate the following:

// pub trait Expr<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error>;
// }

// pub trait Visitor<T> {
//   fn visit_binary_expr(&self, expr: &Binary<T>) -> Result<T, Error>;
//   fn visit_grouping_expr(&self, expr: &Grouping<T>) -> Result<T, Error>;
//   fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<T, Error>;
//   fn visit_unary_expr(&self, expr: &Unary<T>) -> Result<T, Error>;
// }

// pub struct Binary<T> {
//   pub left: Rc<RefCell<dyn Expr<T>>>,
//   pub operator: Token,
//   pub right: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Binary<T> {
//   pub fn new(
//     left: Rc<RefCell<dyn Expr<T>>>,
//     operator: Token,
//     right: Rc<RefCell<dyn Expr<T>>>,
//   ) -> Binary<T> {
//     Binary {
//       left,
//       operator,
//       right,
//     }
//   }
// }

// impl<T> Expr<T> for Binary<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_binary_expr(self)
//   }
// }

// pub struct Grouping<T> {
//   pub expression: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Grouping<T> {
//   pub fn new(expression: Rc<RefCell<dyn Expr<T>>>) -> Grouping<T> {
//     Grouping { expression }
//   }
// }

// impl<T> Expr<T> for Grouping<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_grouping_expr(self)
//   }
// }

// pub struct LiteralObj {
//   pub value: Option<Literal>,
// }

// impl LiteralObj {
//   pub fn new(value: Option<Literal>) -> LiteralObj {
//     LiteralObj { value }
//   }
// }

// impl<T> Expr<T> for LiteralObj {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_literal_expr(self)
//   }
// }

// pub struct Unary<T> {
//   pub operator: Token,
//   pub right: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Unary<T> {
//   pub fn new(operator: Token, right: Rc<RefCell<dyn Expr<T>>>) -> Unary<T> {
//     Unary { operator, right }
//   }
// }

// impl<T> Expr<T> for Unary<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_unary_expr(self)
//   }
// }

// #[cfg(test)]
// mod tests {
//   // use super::*;

//   generate_ast!(
//     #[derive(Debug)]
//     struct Stmt {
//       a: String,
//       b: bool,
//       c: u64,
//     }
//   );

//   #[test]
//   fn create_via_new() {
//     let stmt = Stmt::new(String::from("Howdy"), true, 10);
//     println!("stmt {:?}", stmt);

//     assert_eq!(stmt.a, String::from("Howdy"));
//     assert_eq!(stmt.b, true);
//     assert_eq!(stmt.c, 10);
//   }
// }
