use std::io;

use crate::lexer::Lexer;
use crate::tags::Tag;
use crate::token::Token;
use crate::query::Query;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        let mut parser = Parser { 
            lexer: Lexer::new(input.clone()),
            curr_token: Token::END,
            next_token: Token::END,
        };

        // Initialize to the first two tokens
        parser.next();
        parser.next();

        parser
    }

    pub fn parse(&mut self) -> Result<Query> {
        self.parse_bool_expr()
    }

    fn next(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse_bool_expr(&mut self) -> Result<Query> {
        let mut expr = match self.curr_token.clone() {
            Token::NOT => {
                self.next();
                let tag = self.parse_tag()?;
                Query::Absent(tag)
            },
            Token::TAG(tag) => {
                self.next();
                Query::Present(tag.clone())
            },
            t => {
                return Err(Error::UnexpectedToken(format!("Unexpected token: {:?}", t)));
            },
        };

        if self.curr_token != Token::END {
            expr = match self.curr_token.clone() {
                Token::AND => {
                    self.next();
                    Query::And(
                        Box::new(expr),
                        Box::new(self.parse_bool_expr()?)
                    )
                },
                Token::OR => {
                    self.next();
                    Query::Or(
                        Box::new(expr),
                        Box::new(self.parse_bool_expr()?)
                    )
                },
                t => {
                    return Err(Error::UnexpectedToken(format!("Unexpected token: {:?}", t)));
                },
            };
        }

        Ok(expr)
    }

    fn parse_tag(&mut self) -> Result<Tag> {
        if let Token::TAG(tag) = self.curr_token.clone() {
            self.next();
            Ok(tag.clone())
        } else {
            Err(Error::UnexpectedToken(format!("Expected tag, got {:?}", self.curr_token)))
        }
    }
}


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedToken(String),
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            Error::UnexpectedToken(s) => io::Error::new(
                io::ErrorKind::InvalidInput,
                s
            ),
        }
    }
}
