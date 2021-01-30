use super::{
  expr::*,
  stmt::*,
  literal::*,
  token::*,
  token_type::*,
  rlox_errors::RloxError,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub type ParserExpr<T> = Rc<RefCell<dyn Expr<T>>>;
pub type ParserExprResult<T> = Result<ParserExpr<T>, RloxError>;
pub type ParserStmt<T> = Rc<RefCell<dyn Stmt<T>>>;
pub type ParserStmtResult<T> = Result<ParserStmt<T>, RloxError>;
pub type ParserVecStmtResult<T> = Result<Vec<ParserStmt<T>>, RloxError>;

pub struct Parser {
  tokens: Vec<Token>,
  current: Cell<usize>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: Cell::new(0),
    }
  }

  fn expression<T: 'static>(&self) -> ParserExprResult<T> {
    self.assignment()
  }

  fn assignment<T: 'static>(&self) -> ParserExprResult<T> {
    let expr = self.or()?;

    if self.token_match(vec![TokenType::EQUAL]) {
      let equals = self.previous();
      let value = self.assignment()?;

      if let Some(var_expr) = expr.borrow().as_any().downcast_ref::<Variable>() {
        let name = var_expr.name.clone();
        return Ok(Rc::new(RefCell::new(Assign::new(name, value))));
      } else if let Some(get_expr) = expr.borrow().as_any().downcast_ref::<Get<T>>() {
        return Ok(Rc::new(RefCell::new(Set::new(get_expr.object.clone(), get_expr.name.clone(), value))));
      }

      return Err(RloxError::ParserError(format!("{} Invalid assignment target.", equals.lexeme)))
    }

    Ok(expr)
  }

  fn or<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.and()?;

    while self.token_match(vec![TokenType::OR]) {
      let operator = self.previous();
      let right = self.and()?;
      expr = Rc::new(RefCell::new(Logical::new(expr, operator, right)));
    }

    Ok(expr)
  }

  fn and<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.equality()?;

    while self.token_match(vec![TokenType::AND]) {
      let operator = self.previous();
      let right = self.equality()?;
      expr = Rc::new(RefCell::new(Logical::new(expr, operator, right)));
    }

    Ok(expr)
  }

  fn equality<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.comparison()?;

    while self.token_match(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
      let operator = self.previous();
      let right = self.comparison()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn token_match(&self, token_types: Vec<TokenType>) -> bool {
    for token_type in token_types {
      if self.check(token_type) {
        self.advance();
        return true;
      }
    }

    false
  }

  fn check(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().token_type == token_type
  }

  fn advance(&self) -> Token {
    if !self.is_at_end() {
      self.current.set(self.current.get() + 1);
    }
    self.previous()
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::EOF
  }

  fn peek(&self) -> Token {
    self.tokens[self.current.get()].clone()
  }

  fn previous(&self) -> Token {
    self.tokens[self.current.get() - 1].clone()
  }

  fn comparison<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.addition()?;

    while self.token_match(vec![
      TokenType::GREATER,
      TokenType::GREATEREQUAL,
      TokenType::LESS,
      TokenType::LESSEQUAL,
    ]) {
      let operator = self.previous();
      let right = self.addition()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn addition<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.multiplication()?;

    while self.token_match(vec![TokenType::MINUS, TokenType::PLUS]) {
      let operator = self.previous();
      let right = self.multiplication()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn multiplication<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.unary()?;

    while self.token_match(vec![TokenType::SLASH, TokenType::STAR]) {
      let operator = self.previous();
      let right = self.unary()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn unary<T: 'static>(&self) -> ParserExprResult<T> {
    if self.token_match(vec![TokenType::BANG, TokenType::MINUS]) {
      let operator = self.previous();
      let right = self.unary()?;
      return Ok(Rc::new(RefCell::new(Unary::new(operator.clone(), right))));
    }

    self.call()
  }

  fn call<T: 'static>(&self) -> ParserExprResult<T> {
    let mut expr = self.primary()?;

    loop {
      if self.token_match(vec![TokenType::LEFTPAREN]) {
        expr = self.finish_call(expr)?;
      } else if self.token_match(vec![TokenType::DOT]) {
        let name = self.consume(TokenType::IDENTIFIER, "Expect property name after '.'.")?;
        expr = Rc::new(RefCell::new(Get::new(expr, name)));
      } else {
        break;
      }
    }

    return Ok(expr)
  }

  fn finish_call<T: 'static>(&self, callee: Exp<T>) -> ParserExprResult<T> {
    let mut arguments: Vec<Exp<T>> = Vec::new();

    if !self.check(TokenType::RIGHTPAREN) {
      loop {
        if arguments.len() >= 255 {
          return Err(RloxError::ParserError("Can't have more than 255 arguments.".to_string()));
        }
        arguments.push(self.expression()?);
        if !self.token_match(vec![TokenType::COMMA]) {
          break;
        }
      }
    }

    let paren = self.consume(TokenType::RIGHTPAREN, "Expect ')' after arguments.")?;

    Ok(Rc::new(RefCell::new(Call::new(callee, paren, arguments))))
  }

  fn primary<T: 'static>(&self) -> ParserExprResult<T> {
    if self.token_match(vec![TokenType::FALSE]) {
      return Ok(Rc::new(RefCell::new(LiteralObj::new(Some(
        Literal::BooleanType(false),
      )))));
    }

    if self.token_match(vec![TokenType::TRUE]) {
      return Ok(Rc::new(RefCell::new(LiteralObj::new(Some(
        Literal::BooleanType(true),
      )))));
    }

    if self.token_match(vec![TokenType::NIL]) {
      return Ok(Rc::new(RefCell::new(LiteralObj::new(Some(
        Literal::NullType,
      )))));
    }

    if self.token_match(vec![TokenType::NUMBER, TokenType::STRING]) {
      return Ok(Rc::new(RefCell::new(LiteralObj::new(
        self.previous().literal,
      ))));
    }

    if self.token_match(vec![TokenType::THIS]) {
      return Ok(Rc::new(RefCell::new(This::new(self.previous()))));
    }

    if self.token_match(vec![TokenType::IDENTIFIER]) {
      return Ok(Rc::new(RefCell::new(Variable::new(self.previous()))));
    }

    if self.token_match(vec![TokenType::LEFTPAREN]) {
      let expr = self.expression()?;
      self.consume(TokenType::RIGHTPAREN, "Expected ')' after expression.")?;
      return Ok(Rc::new(RefCell::new(Grouping::new(expr))));
    }

    Err(RloxError::ParserError("Expect expression.".to_string()))
  }

  fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, RloxError> {
    if self.check(token_type) {
      return Ok(self.advance());
    }

    Err(RloxError::ParserError(format!("{}", message)))
  }

  fn synchronize(&self) {
    self.advance();

    while !self.is_at_end() {
      if self.previous().token_type == TokenType::SEMICOLON {
        return;
      }

      match self.peek().token_type {
        TokenType::CLASS
        | TokenType::FUN
        | TokenType::VAR
        | TokenType::FOR
        | TokenType::IF
        | TokenType::WHILE
        | TokenType::PRINT
        | TokenType::RETURN => return,
        _ => self.advance(),
      };
    }
  }

  fn statement<T: 'static>(&self) -> ParserStmtResult<T> {
    if self.token_match(vec![TokenType::FOR]) {
      return self.for_statement();
    }

    if self.token_match(vec![TokenType::IF]) {
      return self.if_statement();
    }

    if self.token_match(vec![TokenType::PRINT]) {
      return self.print_statement();
    }

    if self.token_match(vec![TokenType::RETURN]) {
      return self.return_statement();
    }

    if self.token_match(vec![TokenType::WHILE]) {
      return self.while_statement();
    }

    if self.token_match(vec![TokenType::LEFTBRACE]) {
      return Ok(Rc::new(RefCell::new(Block::new(self.block()?))))
    }

    self.expression_statement()
  }

  fn return_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    let keyword = self.previous();

    let mut value: Rc<RefCell<dyn Expr<T>>> = Rc::new(RefCell::new(LiteralObj::new(Some(Literal::NullType))));

    if !self.check(TokenType::SEMICOLON) {
      value = self.expression()?;
    }

    self.consume(TokenType::SEMICOLON, "Expect ';' after return value.")?;

    Ok(Rc::new(RefCell::new(Return::new(keyword, value))))
  }

  fn for_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    self.consume(TokenType::LEFTPAREN, "Expect '(' after 'for'.")?;

    let initializer: Option<Stm<T>>;
    if self.token_match(vec![TokenType::SEMICOLON]) {
      initializer = None;
    } else if self.token_match(vec![TokenType::VAR]) {
      initializer = Some(self.var_declaration()?);
    } else {
      initializer = Some(self.expression_statement()?);
    }

    let mut condition: Option<Exp<T>> = None;
    if !self.check(TokenType::SEMICOLON) {
      condition = Some(self.expression()?);
    }
    self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

    let mut increment: Option<Exp<T>> = None;
    if !self.check(TokenType::RIGHTPAREN) {
      increment = Some(self.expression()?);
    }
    self.consume(TokenType::RIGHTPAREN, "Expect ')' after for clauses.")?;

    let mut body = self.statement()?;

    if let Some(increment) = increment {
      body = Rc::new(RefCell::new(Block::new(vec![body, Rc::new(RefCell::new(Expression::new(increment)))])));
    }

    if let None = condition {
      condition = Some(Rc::new(RefCell::new(LiteralObj::new(Some(Literal::BooleanType(true))))));
    }
    body = Rc::new(RefCell::new(While::new(condition.unwrap(), body)));

    if let Some(initializer) = initializer {
      body = Rc::new(RefCell::new(Block::new(vec![initializer, body])));
    }

    Ok(body)
  }

  fn if_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    self.consume(TokenType::LEFTPAREN, "Expect '(' after if.")?;
    let condition = self.expression()?;
    self.consume(TokenType::RIGHTPAREN, "Expect ')' after if confition.")?;

    let then_branch = self.statement()?;
    let mut else_branch: Option<Stm<T>> = None;
    if self.token_match(vec![TokenType::ELSE]) {
      else_branch = Some(self.statement()?);
    }

    Ok(Rc::new(RefCell::new(If::new(condition, then_branch, else_branch))))
  }

  fn while_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    self.consume(TokenType::LEFTPAREN, "Expect '(' after 'while'.")?;
    let condition = self.expression()?;
    self.consume(TokenType::RIGHTPAREN, "Expect ')' after condition.")?;
    let body = self.statement()?;

    Ok(Rc::new(RefCell::new(While::new(condition, body))))
  }

  fn block<T: 'static>(&self) -> ParserVecStmtResult<T> {
    let mut statements = Vec::new();

    while !self.check(TokenType::RIGHTBRACE) && !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    self.consume(TokenType::RIGHTBRACE, "Expect '}' after block.")?;

    Ok(statements)
  }

  fn print_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    let value = self.expression()?;
    self.consume(TokenType::SEMICOLON, "Expected ';' after value.")?;
    Ok(Rc::new(RefCell::new(Print::new(value))))
  }

  fn expression_statement<T: 'static>(&self) -> ParserStmtResult<T> {
    let expr = self.expression()?;
    self.consume(TokenType::SEMICOLON, "Expected ';' after expression.")?;
    Ok(Rc::new(RefCell::new(Expression::new(expr))))
  }

  fn declaration_impl<T: 'static>(&self) -> ParserStmtResult<T> {
    if self.token_match(vec![TokenType::CLASS]) {
      return self.class_declaration();
    }

    if self.token_match(vec![TokenType::FUN]) {
      return self.function("function");
    }

    if self.token_match(vec![TokenType::VAR]) {
      return self.var_declaration();
    }

    self.statement()
  }

  fn declaration<T: 'static>(&self) -> ParserStmtResult<T> {
    match self.declaration_impl() {
      Ok(d) => Ok(d),
      Err(e) => {
        self.synchronize();
        Err(e)
      }
    }
  }

  fn function<T: 'static>(&self, kind: &str) -> ParserStmtResult<T> {
    let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {} name.", kind))?;
    self.consume(TokenType::LEFTPAREN, &format!("Expect '(' after {} name.", kind))?;

    let mut parameters: Vec<Token> = Vec::new();

    if !self.check(TokenType::RIGHTPAREN) {
      loop {
        if parameters.len() >= 255 {
          return Err(RloxError::ParserError("Can't have more than 255 parameters.".to_string()))
        }

        parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name.")?);

        if !self.token_match(vec![TokenType::COMMA]) {
          break;
        }
      }
    }

    self.consume(TokenType::RIGHTPAREN, "Expect ')' after parameters.")?;

    self.consume(TokenType::LEFTBRACE, &format!("Expect '{{' before {} body.", kind))?;
    let body = self.block()?;
    Ok(Rc::new(RefCell::new(Function::new(name, parameters, body))))
  }

  fn var_declaration<T: 'static>(&self) -> ParserStmtResult<T> {
    let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;

    let mut initializer: std::rc::Rc<std::cell::RefCell<dyn Expr<T>>> = Rc::new(RefCell::new(LiteralObj::new(Some(Literal::NullType))));
    if self.token_match(vec![TokenType::EQUAL]) {
      initializer = self.expression()?;
    }

    self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;
    Ok(Rc::new(RefCell::new(Var::new(name, initializer))))
  }

  fn class_declaration<T: 'static>(&self) -> ParserStmtResult<T> {
    let name = self.consume(TokenType::IDENTIFIER, "Expected class name.")?;
    self.consume(TokenType::LEFTBRACE, "Expected '{' before class body.")?;

    let mut methods = Vec::new();
    while !self.check(TokenType::RIGHTBRACE) && !self.is_at_end() {
      methods.push(self.function("method")?);
    }

    self.consume(TokenType::RIGHTBRACE, "Expect '}' adter class body.")?;

    Ok(Rc::new(RefCell::new(Class::new(name, methods))))
  }

  pub fn parse<T: 'static>(&self) -> Result<Vec<ParserStmt<T>>, RloxError> {
    let mut statements = Vec::new();
    while !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    Ok(statements)
  }
}
