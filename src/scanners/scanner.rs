use super::error_handler::report;
use super::literal::Literal;
use super::token::Token;
use super::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {
  source: Vec<char>,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: Vec<char>) -> Scanner {
    Scanner {
      source,
      tokens: Vec::<Token>::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source[self.current - 1]
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.add_token_with_literal(token_type, None);
  }

  fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let v = &self.source[self.start..self.current];
    let text: String = v.into_iter().collect();
    self
      .tokens
      .push(Token::new(token_type, text, literal, self.line));
  }

  fn is_next_match(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }

    if self.source[self.current] != expected {
      return false;
    }

    self.current += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.source[self.current]
  }

  fn process_string_literal(&mut self) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }

    // unterminated string
    if self.is_at_end() {
      report(self.line, "Unterminated string.");
      return;
    }

    // the closing "
    self.advance();

    // trim the surrounding quotes
    let l = &self.source[self.start + 1..self.current - 1];
    let literal_string: Literal = Literal::StringType(l.into_iter().collect());
    self.add_token_with_literal(TokenType::STRING, Some(literal_string));
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0';
    }
    return self.source[self.current + 1];
  }

  fn is_digit(&self, c: char) -> bool {
    c >= '0' && c <= '9'
  }

  fn process_number_literal(&mut self) {
    while self.is_digit(self.peek()) {
      self.advance();
    }

    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      // consume the "."
      self.advance();

      while self.is_digit(self.peek()) {
        self.advance();
      }
    }

    let l = &self.source[self.start..self.current];
    let number_as_string: String = l.into_iter().collect();
    if let Ok(n) = number_as_string.parse::<f64>() {
      let literal_number: Literal = Literal::NumberType(n);
      self.add_token_with_literal(TokenType::NUMBER, Some(literal_number));
    } else {
      report(self.line, "Invalid number");
    }
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      '(' => self.add_token(TokenType::LEFTPAREN),
      ')' => self.add_token(TokenType::RIGHTPAREN),
      '{' => self.add_token(TokenType::LEFTBRACE),
      '}' => self.add_token(TokenType::RIGHTBRACE),
      ',' => self.add_token(TokenType::COMMA),
      '.' => self.add_token(TokenType::DOT),
      '-' => self.add_token(TokenType::MINUS),
      '+' => self.add_token(TokenType::PLUS),
      ';' => self.add_token(TokenType::SEMICOLON),
      '*' => self.add_token(TokenType::STAR),
      '!' => {
        if self.is_next_match('=') {
          self.add_token(TokenType::BANGEQUAL)
        } else {
          self.add_token(TokenType::BANG)
        }
      }
      '=' => {
        if self.is_next_match('=') {
          self.add_token(TokenType::EQUALEQUAL)
        } else {
          self.add_token(TokenType::EQUAL)
        }
      }
      '<' => {
        if self.is_next_match('=') {
          self.add_token(TokenType::LESSEQUAL)
        } else {
          self.add_token(TokenType::LESS)
        }
      }
      '>' => {
        if self.is_next_match('=') {
          self.add_token(TokenType::GREATEREQUAL)
        } else {
          self.add_token(TokenType::GREATER)
        }
      }
      '/' => {
        if self.is_next_match('/') {
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        } else {
          self.add_token(TokenType::SLASH);
        }
      }
      ' ' | '\r' | '\t' => {
        // ignore whitespace
      }
      '\n' => self.line += 1,
      '"' => self.process_string_literal(),
      '0'..='9' => self.process_number_literal(),
      _ => report(self.line, "Unexpected character."),
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token::new(
      TokenType::EOF,
      String::from(""),
      None,
      self.line,
    ));

    self.tokens.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scan_simple_tokens() {
    let text = String::from("*+-{}");
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let assert_tokens = vec![
      TokenType::STAR,
      TokenType::PLUS,
      TokenType::MINUS,
      TokenType::LEFTBRACE,
      TokenType::RIGHTBRACE,
      TokenType::EOF,
    ];

    for (i, t) in tokens.iter().enumerate() {
      assert_eq!(assert_tokens[i], t.token_type);
    }
  }

  #[test]
  fn scan_more_complex_tokens() {
    let text =
      String::from("// this is a comment\n(( )){} // grouping stuff\n!*+-/=<> <= == // operators");
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let assert_tokens = vec![
      TokenType::LEFTPAREN,
      TokenType::LEFTPAREN,
      TokenType::RIGHTPAREN,
      TokenType::RIGHTPAREN,
      TokenType::LEFTBRACE,
      TokenType::RIGHTBRACE,
      TokenType::BANG,
      TokenType::STAR,
      TokenType::PLUS,
      TokenType::MINUS,
      TokenType::SLASH,
      TokenType::EQUAL,
      TokenType::LESS,
      TokenType::GREATER,
      TokenType::LESSEQUAL,
      TokenType::EQUALEQUAL,
      TokenType::EOF,
    ];

    for (i, t) in tokens.iter().enumerate() {
      assert_eq!(assert_tokens[i], t.token_type);
    }
  }

  #[test]
  fn scan_string_literal_tokens() {
    let text =
      String::from("// this is a comment\n(( )){} // grouping stuff\n!*+-/=<> \"test string literal\" <= == // operators");
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let assert_tokens = vec![
      TokenType::LEFTPAREN,
      TokenType::LEFTPAREN,
      TokenType::RIGHTPAREN,
      TokenType::RIGHTPAREN,
      TokenType::LEFTBRACE,
      TokenType::RIGHTBRACE,
      TokenType::BANG,
      TokenType::STAR,
      TokenType::PLUS,
      TokenType::MINUS,
      TokenType::SLASH,
      TokenType::EQUAL,
      TokenType::LESS,
      TokenType::GREATER,
      TokenType::STRING,
      TokenType::LESSEQUAL,
      TokenType::EQUALEQUAL,
      TokenType::EOF,
    ];

    for (i, t) in tokens.iter().enumerate() {
      assert_eq!(assert_tokens[i], t.token_type);
    }
  }

  #[test]
  fn scan_number_literals_tokens() {
    let text = String::from(
      "// this is a comment\n((25.77)){} // grouping stuff\n!*+-/=<20> <= == // operators",
    );
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let assert_tokens = vec![
      TokenType::LEFTPAREN,
      TokenType::LEFTPAREN,
      TokenType::NUMBER,
      TokenType::RIGHTPAREN,
      TokenType::RIGHTPAREN,
      TokenType::LEFTBRACE,
      TokenType::RIGHTBRACE,
      TokenType::BANG,
      TokenType::STAR,
      TokenType::PLUS,
      TokenType::MINUS,
      TokenType::SLASH,
      TokenType::EQUAL,
      TokenType::LESS,
      TokenType::NUMBER,
      TokenType::GREATER,
      TokenType::LESSEQUAL,
      TokenType::EQUALEQUAL,
      TokenType::EOF,
    ];

    for (i, t) in tokens.iter().enumerate() {
      assert_eq!(assert_tokens[i], t.token_type);
    }
  }
}
