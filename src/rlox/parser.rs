use super::expr::*;
use super::literal::*;
use super::token::*;
use super::token_type::*;
use failure::{format_err, Error};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub type ParserExpr = Rc<RefCell<dyn Expr>>;
pub type ParserResult = Result<ParserExpr, Error>;

pub struct Parser {
  tokens: Vec<Token>,
  current: Cell<usize>,
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: Cell::new(0),
    }
  }

  fn expression(&self) -> ParserResult {
    self.equality()
  }

  fn equality(&self) -> ParserResult {
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

  fn comparison(&self) -> ParserResult {
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

  fn addition(&self) -> ParserResult {
    let mut expr = self.multiplication()?;

    while self.token_match(vec![TokenType::MINUS, TokenType::PLUS]) {
      let operator = self.previous();
      let right = self.multiplication()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn multiplication(&self) -> ParserResult {
    let mut expr = self.unary()?;

    while self.token_match(vec![TokenType::SLASH, TokenType::STAR]) {
      let operator = self.previous();
      let right = self.unary()?;
      expr = Rc::new(RefCell::new(Binary::new(expr, operator.clone(), right)));
    }

    Ok(expr)
  }

  fn unary(&self) -> ParserResult {
    if self.token_match(vec![TokenType::BANG, TokenType::MINUS]) {
      let operator = self.previous();
      let right = self.unary()?;
      return Ok(Rc::new(RefCell::new(Unary::new(operator.clone(), right))));
    }

    self.primary()
  }

  fn primary(&self) -> ParserResult {
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

    if self.token_match(vec![TokenType::LEFTPAREN]) {
      let expr = self.expression()?;
      self.consume(TokenType::RIGHTPAREN, "Expected ')' after expression.")?;
      return Ok(Rc::new(RefCell::new(Grouping::new(expr))));
    }

    Err(format_err!("Parser error."))
  }

  fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, Error> {
    if self.check(token_type) {
      return Ok(self.advance());
    }

    Err(format_err!("{}", message))
  }
}
