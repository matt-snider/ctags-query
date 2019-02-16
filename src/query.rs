use crate::lexer::Lexer;
use crate::token::Token;

pub struct Query { }


impl Query {
    pub fn from(input: &str) -> Query {
        let mut l = Lexer::new(&input);

        let mut t = l.next_token();
        while t != Token::END {
            println!("Lexer.next_token(): {:?}", t);
            t = l.next_token();
        }

        Query {}
    }
}

