use std::cell::RefCell;
use std::rc::Rc;
use super::{
  token::*,
  expr::*,
};
use crate::generate_ast;

pub type Stm<T> = Rc<RefCell<dyn Stmt<T>>>;

// program        → statement* EOF ;
//
// declaration    → funDecl
//                | varDecl
//                | statement ;
//
// statement      → exprStmt
//                | forStmt
//                | ifStmt
//                | printStmt
//                | returnStmt
//                | whileStmt
//                | block ;
//
// returnStmt     → "return" expression? ";" ;
// funDecl        → "fun" function ;
// function       → IDENTIFIER "(" parameters? ")" block ;
// parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
// forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
//                  expression? ";"
//                  expression? ")" statement ;
// whileStmt      → "while" "(" expression ")" statement ;
// ifStmt         → "if" "(" expression ")" statement
//                ( "else" statement )? ;
// block          → "{" declaration* "}" ;
// classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )?
//                  "{" function* "}" ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

generate_ast! {
  Stmt {
    visit_block_stmt Block T => statements: Vec<Stm<T>>;
    visit_class_stmt Class T => name: Token, superclass: Option<Variable>, methods: Vec<Stm<T>>;
    visit_expression_stmt Expression T => expression: Exp<T>;
    visit_function_stmt Function T => name: Token, params: Vec<Token>, body: Vec<Stm<T>>;
    visit_if_stmt If T => condition: Exp<T>, then_branch: Stm<T>, else_branch: Option<Stm<T>>;
    visit_print_stmt Print T => expression: Exp<T>;
    visit_return_stmt Return T => keyword: Token, value: Exp<T>;
    visit_var_stmt Var T => name: Token, initializer: Exp<T>;
    visit_while_stmt While T => condition: Exp<T>, body: Stm<T>;
  }
}
