use std::io::{ BufReader };
use std::fs::File;

mod tamal;
use tamal::lexer::Lexer;

use crate::tamal::shared::TokenKind;

fn main() {
    let mut source_file = BufReader::new(File::open("main.tamal").expect("Unable to open file"));
    let mut lexer = Lexer::new(&mut source_file);

    loop {
        let token = lexer.lex();
        println!("position: [{}:{}]; kind: {:?}", token.pos.ln, token.pos.col, token.kind);

        match token.kind {
            TokenKind::EndOfFile => break,
            _ => ()
        }
    }
}
