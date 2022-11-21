use std::collections::VecDeque;
use std::io::{ BufReader, Read };
use std::fs::File;

use super::shared::{ Position, Token, TokenKind, TamalNumber };

pub struct Lexer<'a> {
  pos: Position,
  source_reader: &'a mut BufReader<File>,
  source: VecDeque<char>,
  curr_char: char
}

impl Lexer<'_> {
  pub fn new(source_reader: &mut BufReader<File>) -> Lexer {
    let mut lexer_position = Position { ln: 1, col: 0 };
    let mut source_buffer = String::new();
    
    source_reader.read_to_string(&mut source_buffer);

    Lexer {
      source_reader,
      pos: lexer_position,
      source: VecDeque::from_iter(source_buffer.chars()),
      curr_char: ' '
    }
  }

  pub fn lex(&mut self) -> Token {
    // define the kind of the token along with the value of the token
    self.pos.col += 1;

    let start_position = self.pos;
    let curr_char = match self.source.pop_front() {
      Some(c) => c,
      None => return Token { kind: TokenKind::EndOfFile, pos: start_position }
    };

    self.curr_char = curr_char;

    let token_kind = match curr_char
    {
      '+' => TokenKind::Addition,
      '*' => TokenKind::Multiplication,
      '-' => TokenKind::Substraction,
      ':' => TokenKind::Colon,
      '=' => TokenKind::Assign,
      '_' => TokenKind::Identifier { value: self.read_identifier() },
      '.' => TokenKind::Number { value: self.read_number() },
      ' ' => return self.lex(),
      '\n' => { self.reset_position(); return self.lex(); },
      _ if curr_char.is_alphabetic() => { let ident = self.read_identifier(); self.lookup_token_kind(ident) },
      _ if curr_char.is_numeric() => TokenKind::Number { value: self.read_number() },
      _ => TokenKind::Illegal { value: (curr_char.to_string()) }
    };
    
    Token { kind: token_kind, pos: start_position }
  }

  fn lookup_token_kind(&mut self, ident: String) -> TokenKind {
    match ident.as_str() {
      "fn" => TokenKind::Function,
      "end" => TokenKind::End,
      _ => TokenKind::Identifier { value: ident }
    }
  }

  fn read_identifier(&mut self) -> String {
    let mut identifier = String::new();
    identifier.push(self.curr_char);
    
    loop {
      let curr_char = match self.source.get(0) {
        Some(c) => 
          match c {
            '_' => self.source.pop_front().unwrap(),
            _ if c.is_alphanumeric() => self.source.pop_front().unwrap(),
            _ => return identifier
          },
        None => return identifier
      };

      self.pos.col += 1;
      self.curr_char = curr_char;
      identifier.push(curr_char)
    }
  }

  fn read_number(&mut self) -> TamalNumber {
    let mut literal = String::new();
    let mut is_float = false;
    
    match self.curr_char {
      '.' => { 
        literal.push('0'); 
        is_float = true;
      },
      _ => ()
    }
    literal.push(self.curr_char);

    loop {
      let curr_char = match self.source.get(0) {
        Some(c) => 
          match c {
            '.' => self.source.pop_front().unwrap(),
            _ if c.is_numeric() => self.source.pop_front().unwrap(),
            _ => break 
          },
        None => break
      };

      match curr_char {
        '.' if !is_float => { is_float = true },
        _ if curr_char.is_numeric() => (),
        '.' if is_float => break,
        _ => break
      }

      self.pos.col += 1;
      self.curr_char = curr_char;
      literal.push(curr_char)
    }

    if is_float {
      TamalNumber::Float(literal.parse().unwrap_or(0.0))
    } else {
      TamalNumber::Int(literal.parse().unwrap_or(0))
    }
  }

  fn reset_position(&mut self) {
    self.pos.ln += 1;
    self.pos.col = 0;
  }
}