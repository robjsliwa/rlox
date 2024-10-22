use super::{
  literal::*,
  token::*,
};
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
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( "(" arguments? ")" )* ;
// arguments      → expression ( "," expression )* ;
// primary        → "true" | "false" | "nil" | "this"
//                | NUMBER | STRING | IDENTIFIER | "(" expression ")"
//                | "super" "." IDENTIFIER ;

pub type Exp<T> = Rc<RefCell<dyn Expr<T>>>;

generate_ast! {
  Expr {
    visit_assign_expr Assign T => name: Token, value: Exp<T>;
    visit_binary_expr Binary T => left: Exp<T>, operator: Token, right: Exp<T>;
    visit_call_expr Call T => callee: Exp<T>, parent: Token, arguments: Vec<Exp<T>>;
    visit_get_expr Get T => object: Exp<T>, name: Token;
    visit_grouping_expr Grouping T => expression: Exp<T>;
    visit_literal_expr LiteralObj => value: Option<Literal>;
    visit_logical_expr Logical T => left: Exp<T>, operator: Token, right: Exp<T>;
    visit_set_expr Set T => object: Exp<T>, name: Token, value: Exp<T>;
    visit_super_expr Super => keyword: Token, method: Token;
    visit_this_expr This => keyword: Token;
    visit_unary_expr Unary T => operator: Token, right: Exp<T>;
    visit_variable_expr Variable => name: Token;
  }
}

// Following is one off implemetation to support storying
// Variable, Assign, Get in HashMap.
impl PartialEq for Variable {
  fn eq(&self, other: &Self) -> bool {
      // self.name.lexeme == other.name.lexeme
      self.id == other.id
  }
}

impl Eq for Variable {}

impl std::hash::Hash for Variable {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    // self.name.lexeme.hash(state);
    self.id.hash(state);
  }
}

impl Clone for Variable {
  fn clone(&self) -> Self {
    Variable {
      name: self.name.clone(),
      id: self.id,
    }
  }
}

impl std::fmt::Display for Variable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "variable {}", self.name.lexeme)
  }
}

impl<T> PartialEq for Assign<T> {
  fn eq(&self, other: &Self) -> bool {
      // self.name.lexeme == other.name.lexeme
      self.id == other.id
  }
}

impl<T> Eq for Assign<T> {}

impl<T> std::hash::Hash for Assign<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    // self.name.lexeme.hash(state);
    self.id.hash(state);
  }
}

impl<T> Clone for Assign<T> {
  fn clone(&self) -> Self {
    Assign {
      name: self.name.clone(),
      value: self.value.clone(),
      id: self.id,
    }
  }
}

impl<T> std::fmt::Display for Assign<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "assign {}", self.name.lexeme)
  }
}

impl PartialEq for This {
  fn eq(&self, other: &Self) -> bool {
      // self.name.lexeme == other.name.lexeme
      self.id == other.id
  }
}

impl Eq for This {}

impl std::hash::Hash for This {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    // self.name.lexeme.hash(state);
    self.id.hash(state);
  }
}

impl Clone for This {
  fn clone(&self) -> Self {
    This {
      keyword: self.keyword.clone(),
      id: self.id,
    }
  }
}

impl std::fmt::Display for This {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "this {}", self.keyword.lexeme)
  }
}

impl PartialEq for Super {
  fn eq(&self, other: &Self) -> bool {
      // self.name.lexeme == other.name.lexeme
      self.id == other.id
  }
}

impl Eq for Super {}

impl std::hash::Hash for Super {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    // self.name.lexeme.hash(state);
    self.id.hash(state);
  }
}

impl Clone for Super {
  fn clone(&self) -> Self {
    Super {
      keyword: self.keyword.clone(),
      method: self.method.clone(),
      id: self.id,
    }
  }
}

impl std::fmt::Display for Super {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "this {}", self.keyword.lexeme)
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
