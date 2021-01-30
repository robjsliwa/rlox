use super::{
  expr::*,
  stmt::*,
  rlox_type::*,
  token_type::*,
  token::*,
  literal::*,
  environment::*,
  rlox_function::RloxFunction,
  rlox_errors::RloxError,
  rlox_class::RloxClass,
};
use std::{
  cell::RefCell,
  rc::Rc,
  collections::HashMap,
};

#[derive(PartialEq, Eq, Hash)]
pub enum VarExpr {
  VariableExpr(Variable),
  AssignmentExpr(Assign<RloxType>),
  ThisExpr(This),
}

type Exp = Rc<RefCell<dyn Expr<RloxType>>>;
type Stm = Rc<RefCell<dyn Stmt<RloxType>>>;

#[derive(Clone)]
pub struct Interpreter {
  environment: Rc<RefCell<Environment>>,
  globals: Rc<RefCell<Environment>>,
  locals: Rc<RefCell<HashMap<VarExpr, usize>>>
}

impl Interpreter {
  pub fn new() -> Interpreter {
    let env_init = Rc::new(RefCell::new(Environment::new()));
    Interpreter {
      environment: env_init.clone(),
      globals: Rc::new(RefCell::new(Environment::new())),
      locals: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn interpret(&self, statements: Vec<Stm>, callback: Option<fn(resutl: Result<RloxType, RloxError>)>) {
    for statement in statements {
      let result = self.evaluate_stmt(statement);
      if let Some(f) = callback {
        f(result);
      }
    }
  }

  fn evaluate_expr(&self, expr: Exp) -> Result<RloxType, RloxError> {
    expr.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn evaluate_stmt(&self, stmt: Stm) -> Result<RloxType, RloxError> {
    stmt.borrow().accept(Rc::new(RefCell::new(self.clone())))
  }

  fn is_truthy(&self, rlox_type: RloxType) -> Result<RloxType, RloxError> {
    match rlox_type {
      RloxType::NullType => Ok(RloxType::BooleanType(false)),
      RloxType::BooleanType(b) => Ok(RloxType::BooleanType(b)),
      _ => Ok(RloxType::BooleanType(true)),
    }
  }

  fn is_equal(&self, left: Literal, right: Literal) -> Result<RloxType, RloxError> {
    if left == Literal::NullType && right == Literal::NullType {
      return Ok(RloxType::BooleanType(true));
    }
    if left == Literal::NullType {
      return Ok(RloxType::BooleanType(false));
    }

    Ok(RloxType::BooleanType(left == right))
  }

  fn not(&self, b: RloxType) -> Result<RloxType, RloxError> {
    if let RloxType::BooleanType(b) = b {
      return Ok(RloxType::BooleanType(!b));
    }

    Err(RloxError::InterpreterError("invalid type: expected boolean".to_string()))
  }

  fn compute_binary_operand(&self, token_type: &TokenType, left: Literal, right: Literal) -> Result<RloxType, RloxError> {
    if let RloxType::NumberType(left_number) = left {
      if let RloxType::NumberType(right_number) = right {
        return match token_type {
          TokenType::MINUS => Ok(RloxType::NumberType(left_number - right_number)),
          TokenType::PLUS => Ok(RloxType::NumberType(left_number + right_number)),
          TokenType::SLASH => Ok(RloxType::NumberType(left_number / right_number)),
          TokenType::STAR => Ok(RloxType::NumberType(left_number * right_number)),
          TokenType::GREATER => Ok(RloxType::BooleanType(left_number > right_number)),
          TokenType::GREATEREQUAL => Ok(RloxType::BooleanType(left_number >= right_number)),
          TokenType::LESS => Ok(RloxType::BooleanType(left_number < right_number)),
          TokenType::LESSEQUAL => Ok(RloxType::BooleanType(left_number <= right_number)),
          TokenType::BANGEQUAL => self.not(self.is_equal(left, right)?),
          TokenType::EQUALEQUAL => self.is_equal(left, right),
          _ => Err(RloxError::InterpreterError(format!("unimplemented operand {}", token_type.name()))),
        }
      }
    }

    if let RloxType::StringType(left_number) = left {
      if let RloxType::StringType(right_number) = right {
        return match token_type {
          TokenType::PLUS => Ok(RloxType::StringType(format!("{}{}", left_number, right_number))),
          _ => Err(RloxError::InterpreterError(format!("unsupported operand type(s) for {}: both operand types must be string", token_type.name()))),
        }
      }
    }

    return Err(RloxError::InterpreterError(format!("unsupported operand type(s) for {}: both operand types must be number", token_type.name())));
  }

  pub fn execute_block(&self, statements: Vec<Stm>, environment: Environment)-> Result<RloxType, RloxError> {
    let previous = self.environment.replace(environment);

    for statement in statements {
      if let Err(e) = self.evaluate_stmt(statement) {
        self.environment.replace(previous);
        return Err(e);
      }
    }

    self.environment.replace(previous);
    Ok(RloxType::NullType)
  }

  pub fn resolve(&self, var_expr: VarExpr, depth: usize) {
    self.locals.borrow_mut().insert(var_expr, depth);
  }

  fn lookup_variable(&self, name: Token, expr: &VarExpr) -> Result<RloxType, RloxError> {
    match self.locals.borrow().get(expr) {
      Some(distance) => self.environment.borrow().get_at(*distance, &name.lexeme),
      None => self.globals.borrow().get(&name.lexeme),
    }
  }

  fn prepare_klass(&self, stmt: &Class<RloxType>, env: &Environment) -> Result<RloxClass, RloxError> {
    let mut methods = HashMap::new();
    for method in &stmt.methods {
      if let Some(func_method) = method.borrow().as_any().downcast_ref::<Function<RloxType>>() {
        methods.insert(func_method.name.lexeme.clone(), RloxFunction::new(func_method, env));
      } else {
        return Err(RloxError::InterpreterError("Expected method.".to_string()));
      }
    }
    Ok(RloxClass::new(&stmt.name.lexeme, Rc::new(RefCell::new(methods))))
  }
}

impl super::stmt::Visitor<RloxType> for Interpreter {
  fn visit_while_stmt(&self, stmt: &While<RloxType>) -> Result<RloxType, RloxError> {
    while self.is_truthy(self.evaluate_expr(stmt.condition.clone())?)? == Literal::BooleanType(true) {
      self.evaluate_stmt(stmt.body.clone())?;
    }

    Ok(RloxType::NullType)
  }

  fn visit_block_stmt(&self, stmt: &Block<RloxType>) -> Result<RloxType, RloxError> {
    let env = Environment::new_with_parent(self.environment.borrow().clone());
    Ok(self.execute_block(stmt.statements.clone(), env)?)
  }

  fn visit_expression_stmt(&self, stmt: &Expression<RloxType>) -> Result<RloxType, RloxError> {
    Ok(self.evaluate_expr(stmt.expression.clone())?)
  }

  fn visit_if_stmt(&self, stmt: &If<RloxType>) -> Result<RloxType, RloxError> {
    if self.is_truthy(self.evaluate_expr(stmt.condition.clone())?)? == Literal::BooleanType(true) {
      self.evaluate_stmt(stmt.then_branch.clone())?;
    } else if let Some(eb) = stmt.else_branch.clone() {
      self.evaluate_stmt(eb)?;
    }

    Ok(RloxType::NullType)
  }

  fn visit_print_stmt(&self, stmt: &Print<RloxType>) -> Result<RloxType, RloxError> {
    let value = self.evaluate_expr(stmt.expression.clone())?;
    println!("{}", value);
    Ok(RloxType::NullType)
  }

  fn visit_var_stmt(&self, stmt: &Var<RloxType>) -> Result<RloxType, RloxError> {
    let value = self.evaluate_expr(stmt.initializer.clone())?;

    let env = self.environment.borrow();
    if env.is_top_level() {
      self.globals.borrow().define(stmt.name.lexeme.clone(), value);
    } else {
      env.define(stmt.name.lexeme.clone(), value);
    }

    Ok(RloxType::NullType)
  }

  fn visit_function_stmt(&self, stmt: &Function<RloxType>) -> Result<RloxType, RloxError> {
    let env = self.environment.borrow();
    let function = RloxFunction::new(stmt, &env);

    if env.is_top_level() {
      self.globals.borrow().define(stmt.name.lexeme.clone(), RloxType::CallableType(Box::new(function)));
    } else {
      env.define(stmt.name.lexeme.clone(), RloxType::CallableType(Box::new(function)));
    }

    Ok(RloxType::NullType)
  }

  fn visit_return_stmt(&self, stmt: &Return<RloxType>) -> Result<RloxType, RloxError> {
    let value = self.evaluate_expr(stmt.value.clone())?;

    Err(RloxError::ReturnValue(value))
  }

  fn visit_class_stmt(&self, stmt: &Class<RloxType>) -> Result<RloxType, RloxError> {
    let env = self.environment.borrow();

    match env.is_top_level() {
      true => {
        let globals = self.globals.borrow();
        globals.define(stmt.name.lexeme.clone(), RloxType::NullType);
        let klass = self.prepare_klass(stmt, &globals)?;
        globals.assign(&stmt.name.lexeme, Literal::CallableType(Box::new(klass)))?;
      }
      false => {
        env.define(stmt.name.lexeme.clone(), RloxType::NullType);
        let klass = self.prepare_klass(stmt, &env)?;
        env.assign(&stmt.name.lexeme, Literal::CallableType(Box::new(klass)))?;
      }
    }

    Ok(RloxType::NullType)
  }
}

impl super::expr::Visitor<RloxType> for Interpreter {
  fn visit_binary_expr(&self, expr: &Binary<RloxType>) -> Result<RloxType, RloxError> {
    let left = self.evaluate_expr(expr.left.clone())?;
    let right = self.evaluate_expr(expr.right.clone())?;

    self.compute_binary_operand(&expr.operator.token_type, left, right)
  }

  fn visit_grouping_expr(&self, _: &Grouping<RloxType>) -> Result<RloxType, RloxError> {
    Ok(RloxType::NullType)
  }

  fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<RloxType, RloxError> {
    match &expr.value {
      Some(v) => Ok(v.clone()),
      None => Err(RloxError::InterpreterError("missing value".to_string())),
    }
  }

  fn visit_unary_expr(&self, expr: &Unary<RloxType>) -> Result<RloxType, RloxError> {
    let right = self.evaluate_expr(expr.right.clone())?;

    match expr.operator.token_type {
      TokenType::MINUS => {
        if let RloxType::NumberType(n) = right {
          return Ok(RloxType::NumberType(-1.0 * n));
        }
        return Err(RloxError::InterpreterError("Invalid type".to_string()));
      }
      TokenType::BANG => self.is_truthy(right),
      _ => Err(RloxError::InterpreterError("unsupported operand".to_string())),
    }
  }

  fn visit_variable_expr(&self, expr: &Variable) -> Result<RloxType, RloxError> {
    // self.environment.borrow().get(&expr.name.lexeme)
    self.lookup_variable(expr.name.clone(), &VarExpr::VariableExpr(expr.clone()))
  }

  fn visit_assign_expr(&self, expr: &Assign<RloxType>) -> Result<RloxType, RloxError> {
    let value = self.evaluate_expr(expr.value.clone())?;
    match self.locals.borrow().get(&VarExpr::AssignmentExpr(expr.clone())) {
      Some(distance) => self.environment.borrow().assign_at(*distance, &expr.name.lexeme, value.clone())?,
      None => self.globals.borrow().assign(&expr.name.lexeme, value.clone())?,
    }
    Ok(value)
  }

  fn visit_logical_expr(&self, expr: &Logical<RloxType>) -> Result<RloxType, RloxError> {
    let left = self.evaluate_expr(expr.left.clone())?;

    if expr.operator.token_type == TokenType::OR {
      if self.is_truthy(left.clone())? == Literal::BooleanType(true) {
        return Ok(left.clone());
      }
    } else {
      if self.is_truthy(left.clone())? == Literal::BooleanType(false) {
        return Ok(left.clone())
      }
    }

    Ok(self.evaluate_expr(expr.right.clone())?)
  }

  fn visit_call_expr(&self, expr: &Call<RloxType>) -> Result<RloxType, RloxError> {
    let callee = self.evaluate_expr(expr.callee.clone())?;

    let mut arguments = Vec::new();
    for argument in expr.arguments.clone() {
      arguments.push(self.evaluate_expr(argument)?);
    }

    match callee {
      RloxType::CallableType(c) => {
        if arguments.len() != c.arity() {
          return Err(RloxError::InterpreterError(format!("Expected {} arguments but got {}.", c.arity(), arguments.len())))
        }
        Ok(c.call(self, arguments)?)
      }
      _ => Err(RloxError::InterpreterError("Can only call functions and classes.".to_string()))
    }
  }

  fn visit_get_expr(&self, expr: &Get<RloxType>) -> Result<RloxType, RloxError> {
    let object = self.evaluate_expr(expr.object.clone())?;

    match object {
      RloxType::ClassType(instance) => {
        instance.get(&expr.name)
      }
      _ => Err(RloxError::InterpreterError("Only instances have properties.".to_string()))
    }
  }

  fn visit_set_expr(&self, expr: &Set<RloxType>) -> Result<RloxType, RloxError> {
    let object = self.evaluate_expr(expr.object.clone())?;

    match object {
      RloxType::ClassType(instance) => {
        let value = self.evaluate_expr(expr.value.clone())?;
        instance.set(&expr.name, &value);
        Ok(value)
      }
      _ => Err(RloxError::InterpreterError("Only instances have flields.".to_string()))
    }
  }

  fn visit_this_expr(&self, expr: &This) -> Result<RloxType, RloxError> {
    self.lookup_variable(expr.keyword.clone(), &VarExpr::ThisExpr(expr.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rlox::*;
  use crate::scanners::Scanner;
  use std::collections::HashMap;

  fn run(input: &str) -> Result<RloxType, RloxError> {
    let data = input.chars().collect();

    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let interpreter = Interpreter::new();

    let resolver = Resolver::new(interpreter.clone());
    resolver.resolve_statements(statements.clone())?;

    let mut final_result = RloxType::NullType;

    for statement in statements {
      final_result = interpreter.evaluate_stmt(statement.clone())?;
    }

    Ok(final_result)
  }

  #[test]
  fn test_basic_arithmetic() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("1 + 2;", 3.0),
      ("-1 + 2;", 1.0),
      ("-1 + -2;", -3.0),
      ("5+5;", 10.0),
      ("25 - 1;", 24.0),
      ("-3 - 3;", -6.0),
      ("-3 - -3;", 0.0),
      ("5*5;", 25.0),
      ("25 /5;", 5.0),
      ("1 - 4 * 4;", -15.0),
      ("25 / 5 + 2 * 4;", 13.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_truthiness() -> Result<(), RloxError> {
    let test_input: HashMap<&str, bool> = [
      ("1 < 2;", true),
      ("-1 <= 2;", true),
      ("25 >= 25;", true),
      ("5 > 5;", false),
      ("-25 > 1;", false),
      ("-3 != 3;", true),
      ("-3 == -3;", true),
      ("5==5;", true),
      ("25 >= 5;", true),
      ("1 - 4 > 4;", false),
      ("25 / 5 == 2 * 4 - 3;", true),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::BooleanType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_global_vars() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var t=5; t;", 5.0),
      ("var t=5; t=t+1; t;", 6.0),
      ("var p=5; p=10; p;", 10.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_var_scopes() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var t=5; t; {var p=10; t=p;} t;", 10.0),
      ("var k=1; {var k=10; k=k+1;} k;", 1.0),
      ("var s=5; {var d=10; d=d+5; s=s+d;} s;", 20.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_if_statements() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var t=3; var p=1; if (t>p) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } t;", 8.0),
      ("var t=3; var p=1; if (t<p) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } p;", 5.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_logical_operators() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var t=3; var p=1; if (t>1 and p<10) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } t;", 8.0),
      ("var t=3; var p=1; if (t<1 and p<10) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } p;", 5.0),
      ("var t=3; var p=1; if (t<1 or p<10) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } t;", 8.0),
      ("var t=3; var p=1; if (t<1 or p>10) { t=t+1; t=t*2; } else { p=p+9; p=p/2; } p;", 5.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    let test_input2: HashMap<&str, &str> = [
      ("\"hi\" or 2;", "hi"),
      ("nil or \"yes\";", "yes"),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input2.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::StringType(expected_result.to_string()).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_while_loop() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var p=0; while(p<5) { print p; p=p+1; } p;", 5.0),
      ("var l=0; var i=0; while(i+l < 10) { print i; print l; i=i+1; l=l+2; } i;", 4.0),
      ("var l=0; var i=0; while(i+l < 10) { print i; print l; i=i+1; l=l+2; } l;", 8.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_for_loop() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("fun fib(n) { if (n <= 1) return n; return fib(n - 2) + fib(n - 1); } var r = 0; for (var i = 0; i < 20; i = i + 1) { r = fib(i); } r;", 4181.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_functions() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("var a = 0; var temp; for (var b = 1; a < 10000; b = temp + b) { print a; temp = a; a = b; } a;", 10946.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_closures() -> Result<(), RloxError> {
    let test_input: HashMap<&str, f64> = [
      ("fun makeCounter() { var i = 0; fun count() { i=i+1; return i; } return count; } var counter=makeCounter(); counter(); counter();", 2.0),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::NumberType(expected_result).to_string());
    }

    Ok(())
  }

  #[test]
  fn test_closure_scopes() -> Result<(), RloxError> {
    let test_input: HashMap<&str, &str> = [
      ("var a = \"global\"; { fun showA() { print a; } showA(); var a = \"block\"; showA(); } fun test() { print a; a=a+\"!\"; return a; } test();", "global!"),
    ].iter().cloned().collect();

    for (&input, &expected_result) in test_input.iter() {
      let val = run(input)?;
      assert_eq!(val.to_string(), RloxType::StringType(expected_result.to_string()).to_string());
    }

    Ok(())
  }
}
