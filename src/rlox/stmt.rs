use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;
use super::expr::*;
use crate::generate_ast;

generate_ast! {
  Stmt {
    visit_expression_stmt Expression T => expression: Exp<T>;
    visit_print_stmt Print T => expression: Exp<T>;
  }
}