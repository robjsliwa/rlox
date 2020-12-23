use super::literal::*;
use super::token::*;
use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;

#[macro_export]
macro_rules! parse_ast_visitor_entry {
  ($visitor_name:ident $t:ident $g:ident) => {
    fn $visitor_name(&self, expr: &$t<$g>) -> Result<T, Error>;
  };
  ($visitor_name:ident $t:ident) => {
    fn $visitor_name(&self, expr: &$t) -> Result<T, Error>;
  };
}

#[macro_export]
macro_rules! generate_ast_visitor {
  ($name: ident {
    $($visitor_name:ident $t:ident $($g:ident)?),*,
  }) => {
    pub trait $name<T> {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error>;
    }
    
    pub trait Visitor<T> {
      $(parse_ast_visitor_entry!($visitor_name $t $($g)?);)*
    }
  };
}

generate_ast_visitor! {
  Expr {
    visit_binary_expr Binary T,
    visit_grouping_expr Grouping T,
    visit_literal_expr LiteralObj,
    visit_unary_expr Unary T,
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

#[macro_export]
macro_rules! parse_grammar_entry {
  ($visitor_name:ident $name:ident $g:ident {
    $($var_name:ident: $t:ty),+;
  }) => {
    pub struct $name<$g> {
      $(pub $var_name: $t),*,
    }
    
    impl<$g> $name<$g> {
      pub fn new(
        $($var_name: $t),*,
      ) -> $name<$g> {
        $name {
          $($var_name),*,
        }
      }
    }
    
    impl<T> Expr<T> for $name<$g> {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
        visitor.borrow().$visitor_name(self)
      }
    }
  };
  ($visitor_name:ident $name:ident {
    $($var_name:ident: $t:ty),+;
  }) => {
    pub struct $name {
      $(pub $var_name: $t),*,
    }
    
    impl $name {
      pub fn new(
        $($var_name: $t),*,
      ) -> $name {
        $name {
          $($var_name),*,
        }
      }
    }
    
    impl<T> Expr<T> for $name {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
        visitor.borrow().$visitor_name(self)
      }
    }
  };
}

#[macro_export]
macro_rules! generate_ast {
  ($root_name: ident {
    $($visitor_name:ident $name:ident $($g:ident)* => $($var_name:ident: $t:ty),+;)+
  }) => {
    $(parse_grammar_entry!($visitor_name $name $($g)* {
      $($var_name: $t),+;
    });)+
  };
}

type Expression<T> = Rc<RefCell<dyn Expr<T>>>;

generate_ast! {
  Expr {
    visit_binary_expr Binary T => left: Expression<T>, operator: Token, right: Expression<T>;
    visit_grouping_expr Grouping T => expression: Expression<T>;
    visit_literal_expr LiteralObj => value: Option<Literal>;
    visit_unary_expr Unary T => operator: Token, right: Expression<T>;
  }
}

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
