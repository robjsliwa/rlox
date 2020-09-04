use super::expr::*;
use super::literal::*;
use super::token::*;
use super::token_type::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens, current: 0 }
  }

  fn expression<E: Expr>(&self) -> E {
    self.equality()
  }

  fn equality<E: Expr>(&mut self) -> Rc<RefCell<E>> {
    let expr = Rc::new(RefCell::new(self.comparison()));

    while self.token_match(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
      let operator = self.previous();
      let right = self.comparison();
      expr = Rc::new(RefCell::new(Binary::new(expr, operator, right)));
    }

    expr
  }

  fn token_match(&mut self, token_types: Vec<TokenType>) -> bool {
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

  fn advance(&mut self) -> Token {
    if !self.is_at_end() {
      self.current += 1;
    }
    self.previous()
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::EOF
  }

  fn peek(&self) -> Token {
    self.tokens[self.current]
  }

  fn previous(&self) -> Token {
    self.tokens[self.current - 1]
  }

  fn comparison<E: Expr>(&mut self) -> E {
    let expr = self.addition();

    while self.token_match(vec![
      TokenType::GREATER,
      TokenType::GREATEREQUAL,
      TokenType::LESS,
      TokenType::LESSEQUAL,
    ]) {
      let operator = self.previous();
      let right = self.addition();
      expr = Binary::new(expr, operator, right);
    }

    expr
  }

  fn addition<E: Expr>(&mut self) -> E {
    let expr = self.multiplication();

    while self.token_match(
      TokenType::MINUS,
      TokenType::PLUS,
    ) {
      let operator = self.previous();
      let right = self.multiplication();
      expr = Binary::new(expr, operator, right);
    }

    expr
  }

  fn multiplication<E: Expr>(&mut self) -> E {
    let expr = self.unary();

    while self.token_match(
      TokenType::SLASH,
      TokenType::STAR,
    ) {
      let operator = self.previous();
      let right = self.unary();
      expr = Binary::new(expr, operator, right);
    }

    expr
  }

  fn unary<E: Expr>(&mut self) -> E {
    if self.token_match(TokenType::BANG, TokenType::MINUS) {
      let operator = self.previous();
      let right = self.unary();
      return Unary::new(operator, right);
    }

    self.primary()
  }

  fn primary<E: Expr> primary(&mut self) -> E {
    if self.token_match(TokenType::FALSE) {
      return LiteralObj::new()
    }
  }
}
