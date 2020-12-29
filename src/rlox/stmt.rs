use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;
use super::token::*;
use super::expr::*;
use crate::generate_ast;

// program        → statement* EOF ;

// statement      → exprStmt
//                | printStmt
//                | block ;

// block          → "{" declaration* "}" ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

// generate_expr_ast! {
//   Stmt {
//     visit_expression_stmt Expression T => expression: Exp<T>;
//     visit_print_stmt Print T => expression: Exp<T>;
//   }
// }

generate_ast! {
  Stmt {
    visit_block_stmt Block T => statements: Vec<Rc<RefCell<dyn Stmt<T>>>>;
    visit_expression_stmt Expression T => expression: Exp<T>;
    visit_print_stmt Print T => expression: Exp<T>;
    visit_var_stmt Var T => name: Token, initializer: Exp<T>;
  }
}