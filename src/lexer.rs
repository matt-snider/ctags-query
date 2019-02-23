use std::str::Chars;
use crate::token::Token;

pub struct Lexer<'a> {
    input: Chars<'a>,
    ch: char,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let input = input.chars();
        let mut l = Lexer {
            input,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        return match self.ch {
            '|' => {
                self.read_char();
                Token::OR
            },
            '&' => {
                self.read_char();
                Token::AND
            }
            '!' => {
                self.read_char();
                Token::NOT
            }
            x if is_tag_char(x) =>  {
                let tag = self.read_tag();
                Token::TAG(tag)
            },
            '\0' => {
                Token::END
            },
            _   => panic!("Unsupported token: {}", self.ch),
        };
    }

    fn read_char(&mut self) {
        self.ch = match self.input.next() {
            Some(c) => c,
            None => '\0',
        };
    }

    fn read_tag(&mut self) -> String {
        let mut tag = String::new();
        while is_tag_char(self.ch) {
            tag.push(self.ch);
            self.read_char();
        }
        return tag;
    }

    fn eat_whitespace(&mut self) {
        loop {
            match self.ch {
                x if is_whitespace(x) => self.read_char(),
                _                 => break,
            }
        }
    }

}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

fn is_tag_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '/' => true,
        _ => false,
    }
}

