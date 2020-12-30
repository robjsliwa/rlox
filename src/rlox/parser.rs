use super::expr::*;
use super::stmt::*;
use super::literal::*;
use super::token::*;
use super::token_type::*;
use failure::{format_err, Error};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub type ParserExpr<T> = Rc<RefCell<dyn Expr<T>>>;
pub type ParserExprResult<T> = Result<ParserExpr<T>, Error>;
pub type ParserStmt<T> = Rc<RefCell<dyn Stmt<T>>>;
pub type ParserStmtResult<T> = Result<ParserStmt<T>, Error>;
pub type ParserVecStmtResult<T> = Result<Vec<ParserStmt<T>>, Error>;

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
      }

      return Err(format_err!("{} Invalid assignment target.", equals.lexeme))
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

    self.primary()
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

    if self.token_match(vec![TokenType::IDENTIFIER]) {
      return Ok(Rc::new(RefCell::new(Variable::new(self.previous()))));
    }

    if self.token_match(vec![TokenType::LEFTPAREN]) {
      let expr = self.expression()?;
      self.consume(TokenType::RIGHTPAREN, "Expected ')' after expression.")?;
      return Ok(Rc::new(RefCell::new(Grouping::new(expr))));
    }

    Err(format_err!("Expect expression."))
  }

  fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, Error> {
    if self.check(token_type) {
      return Ok(self.advance());
    }

    Err(format_err!("{}", message))
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
    if self.token_match(vec![TokenType::IF]) {
      return self.if_statement();
    }

    if self.token_match(vec![TokenType::PRINT]) {
      return self.print_statement();
    }

    if self.token_match(vec![TokenType::LEFTBRACE]) {
      return Ok(Rc::new(RefCell::new(Block::new(self.block()?))))
    }

    self.expression_statement()
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
    if self.token_match(vec![TokenType::VAR]) {
      return Ok(self.var_declaration()?);
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

  fn var_declaration<T: 'static>(&self) -> ParserStmtResult<T> {
    let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;

    let mut initializer: std::rc::Rc<std::cell::RefCell<dyn Expr<T>>> = Rc::new(RefCell::new(LiteralObj::new(Some(Literal::NullType))));
    if self.token_match(vec![TokenType::EQUAL]) {
      initializer = self.expression()?;
    }

    self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;
    Ok(Rc::new(RefCell::new(Var::new(name, initializer))))
  }

  pub fn parse<T: 'static>(&self) -> Result<Vec<ParserStmt<T>>, Error> {
    let mut statements = Vec::new();
    while !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    Ok(statements)
  }
}
