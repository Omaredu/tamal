use super::{ Position, TokenKind };

pub struct Token {
  pub kind: TokenKind,
  pub pos: Position
}