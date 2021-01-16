use super::{
  expr::*,
  stmt::*,
  rlox_type::*,
  rlox_errors::RloxError,
  token::Token,
  interpreter::{Interpreter, VarExpr},
};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Resolver {
  scopes: Rc<RefCell<Vec<HashMap<String, bool>>>>,
  interpreter: Interpreter,
}

impl<'a> Resolver {
  pub fn new(interpreter: Interpreter) -> Resolver {
    Resolver {
      scopes: Rc::new(RefCell::new(Vec::new())),
      interpreter
    }
  }

  fn begin_scope(&self) {
    self.scopes.borrow_mut().push(HashMap::new());
  }

  pub fn resolve_statements(&self, statements: Vec<Stm<RloxType>>) -> Result<(), RloxError> {
    for statement in statements {
      self.resolve_stmt(statement)?;
    }

    Ok(())
  }

  fn resolve_stmt(&self, stmt: Stm<RloxType>) -> Result<RloxType, RloxError> {
    stmt.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn resolve_expr(&self, expr: Exp<RloxType>) -> Result<RloxType, RloxError> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn end_scope(&self) {
    self.scopes.borrow_mut().pop();
  }

  fn declare(&self, name: Token) -> Result<(), RloxError> {
    let mut scopes = self.scopes.borrow_mut();
    if scopes.is_empty() {
      return Ok(());
    }

    if let Some(back_scope) = scopes.last_mut() {
      if back_scope.contains_key(&name.lexeme) {
        return Err(RloxError::ResolverError("Already variable with this name in this scope.".to_string()));
      }
      back_scope.insert(name.lexeme, false);
    }

    Ok(())
  }

  fn define(&self, name: Token) {
    let mut scopes = self.scopes.borrow_mut();
    if scopes.is_empty() {
      return;
    }

    if let Some(back_scope) = scopes.last_mut() {
      back_scope.insert(name.lexeme, true);
    }
  }

  fn resolve_local(&self, var_expr: VarExpr, name: Token) {
    let scopes = self.scopes.borrow();
    for (i, scope) in scopes.iter().enumerate().rev() {
      if scope.contains_key(&name.lexeme) {
        self.interpreter.resolve(var_expr, scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn resolve_function(&self, stmt: &Function<RloxType>) -> Result<(), RloxError> {
    self.begin_scope();
    for param in stmt.params.clone() {
      self.declare(param.clone())?;
      self.define(param);
    }

    for body_stmt in stmt.body.clone() {
      self.resolve_stmt(body_stmt)?;
    }
    self.end_scope();

    Ok(())
  }
}

impl super::stmt::Visitor<RloxType> for Resolver {
  fn visit_block_stmt(&self, stmt: &Block<RloxType>) -> Result<RloxType, RloxError> {
    self.begin_scope();
    self.resolve_statements(stmt.statements.clone())?;
    self.end_scope();

    Ok(RloxType::NullType)
  }

  fn visit_var_stmt(&self, stmt: &Var<RloxType>) -> Result<RloxType, RloxError> {
    self.declare(stmt.name.clone())?;
    self.resolve_expr(stmt.initializer.clone())?;
    self.define(stmt.name.clone());

    Ok(RloxType::NullType)
  }

  fn visit_function_stmt(&self, stmt: &Function<RloxType>) -> Result<RloxType, RloxError> {
    self.declare(stmt.name.clone())?;
    self.define(stmt.name.clone());

    self.resolve_function(stmt)?;

    Ok(RloxType::NullType)
  }

  fn visit_expression_stmt(&self, stmt: &Expression<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(stmt.expression.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_if_stmt(&self, stmt: &If<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(stmt.condition.clone())?;
    self.resolve_stmt(stmt.then_branch.clone())?;
    if let Some(else_branch) = stmt.else_branch.clone() {
      self.resolve_stmt(else_branch)?;
    }

    Ok(RloxType::NullType)
  }

  fn visit_print_stmt(&self, stmt: &Print<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(stmt.expression.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_return_stmt(&self, stmt: &Return<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(stmt.value.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_while_stmt(&self, stmt: &While<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(stmt.condition.clone())?;
    self.resolve_stmt(stmt.body.clone())?;

    Ok(RloxType::NullType)
  }
}

impl super::expr::Visitor<RloxType> for Resolver {
  fn visit_variable_expr(&self, expr: &Variable) -> Result<RloxType, RloxError> {
    {
      let scopes = self.scopes.borrow_mut();
      if !scopes.is_empty() {
        if let Some(back_scope) = scopes.last() {
          match back_scope.get(&expr.name.lexeme) {
            Some(lexeme) => {
              if *lexeme == false {
                return Err(RloxError::ResolverError(format!("Can't read local variable in its own initializer - {}.", expr.name.lexeme)));
              }
            }
            None => (),
          }
        }
      }
    }
    self.resolve_local(VarExpr::VariableExpr(expr.clone()), expr.name.clone());

    Ok(RloxType::NullType)
  }

  fn visit_assign_expr(&self, expr: &Assign<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.value.clone())?;
    self.resolve_local(VarExpr::AssignmentExpr(expr.clone()), expr.name.clone());

    Ok(RloxType::NullType)
  }

  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.left.clone())?;
    self.resolve_expr(expr.right.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_call_expr(&self, expr: &Call<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.callee.clone())?;

    for argument in expr.arguments.clone() {
      self.resolve_expr(argument)?;
    }

    Ok(RloxType::NullType)
  }

  fn visit_grouping_expr(&self, expr: &Grouping<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.expression.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_literal_expr(&self, _: &LiteralObj) -> Result<RloxType, RloxError> {
    Ok(RloxType::NullType)
  }

  fn visit_logical_expr(&self, expr: &Logical<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.left.clone())?;
    self.resolve_expr(expr.right.clone())?;

    Ok(RloxType::NullType)
  }

  fn visit_unary_expr(&self, expr: &Unary<RloxType>) -> Result<RloxType, RloxError> {
    self.resolve_expr(expr.right.clone())?;

    Ok(RloxType::NullType)
  }
}
