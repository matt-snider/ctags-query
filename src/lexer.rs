use std::str::Chars;
use crate::token::Token;

pub struct Lexer<'a> {
    input: Chars<'a>,
    ch: char,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut l = Lexer {
            input: input.chars(),
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let tok = match self.ch {
            '|'  => Token::OR,
            '&'  => Token::AND,
            '!'  => Token::NOT,
            '\0' => Token::END,
            _   => panic!("Unsupported token: {}", self.ch),
        };
        self.read_char();

        return tok;
    }

    fn read_char(&mut self) {
        self.ch = match self.input.next() {
            Some(c) => c,
            None => '\0',
        };
    }

    fn eat_whitespace(&mut self) {
        loop {
            match self.ch {
                ' ' | '\t' | '\n' => self.read_char(),
                _                 => break,
            }
        }
    }
}
