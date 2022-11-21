#[derive(Debug)]
pub enum TamalNumber {
  Float(f32),
  Int(i32),
}

#[derive(Debug)]
pub enum TokenKind {
  Function,
  End,
  Identifier { value: String },
  Number { value: TamalNumber },
  Assign,
  Colon,
  Addition,
  Substraction,
  Multiplication,
  EndOfFile,
  Illegal { value: String },
}