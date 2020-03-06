mod scanner;
mod lexer;

use crate::ast::*;

use scanner::Scanner;
use lexer::{Lexer, TokenKind};

pub fn parse_module(source: &str) -> Module {
    let scanner = Scanner::new(source.as_bytes());
    let mut lexer = Lexer::new(scanner);
    loop {
        let token = lexer.next();
        println!("{:?} {:?}", token.kind, std::str::from_utf8(token.span).unwrap());
        if token.kind == TokenKind::Eof {
            break;
        }
    }
    todo!()
}
