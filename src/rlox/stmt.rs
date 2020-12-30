use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;
use super::token::*;
use super::expr::*;
use crate::generate_ast;

pub type Stm<T> = Rc<RefCell<dyn Stmt<T>>>;

// program        → statement* EOF ;

// statement      → exprStmt
//                | printStmt
//                | block ;
//
// ifStmt         → "if" "(" expression ")" statement
//                ( "else" statement )? ;
// block          → "{" declaration* "}" ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

generate_ast! {
  Stmt {
    visit_block_stmt Block T => statements: Vec<Stm<T>>;
    visit_expression_stmt Expression T => expression: Exp<T>;
    visit_if_stmt If T => condition: Exp<T>, then_branch: Stm<T>, else_branch: Option<Stm<T>>;
    visit_print_stmt Print T => expression: Exp<T>;
    visit_var_stmt Var T => name: Token, initializer: Exp<T>;
  }
}