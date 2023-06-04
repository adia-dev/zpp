use self::{reserved::Reserved, token::Token};

pub mod reserved;
pub mod token;

#[derive(Default, Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    next_position: usize,
    c: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut new_lexer = Lexer {
            input: input.to_owned(),
            position: 0,
            next_position: 0,
            c: '\0',
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn read_char(&mut self) -> &Lexer {
        if self.next_position >= self.input.len() {
            self.c = '\0';
        } else {
            self.c = self.input[self.next_position];
        }

        self.position = self.next_position;
        self.next_position += 1;
        self
    }

    pub fn next_token(&mut self) -> Token {
        let new_token: Token = match self.c {
            '=' => Token::new(Reserved::ASSIGN.as_str(), &self.c.to_string()),
            ';' => Token::new(Reserved::SEMICOLON.as_str(), &self.c.to_string()),
            '(' => Token::new(Reserved::LPAREN.as_str(), &self.c.to_string()),
            ')' => Token::new(Reserved::RPAREN.as_str(), &self.c.to_string()),
            ',' => Token::new(Reserved::COMMA.as_str(), &self.c.to_string()),
            '+' => Token::new(Reserved::PLUS.as_str(), &self.c.to_string()),
            '{' => Token::new(Reserved::LBRACE.as_str(), &self.c.to_string()),
            '}' => Token::new(Reserved::RBRACE.as_str(), &self.c.to_string()),
            '\0' => Token::new(Reserved::EOF.as_str(), &self.c.to_string()),
            _ => Token::new(Reserved::ILLEGAL.as_str(), &self.c.to_string()),
        };

        self.read_char();

        new_token
    }
}

#[cfg(test)]
mod lexer_tests {

    use std::assert_eq;

    use crate::lexer::{reserved::Reserved, token::Token, Lexer};

    const INPUT: &'static str = "=+(){},;";

    #[test]
    fn test_next_token() {
        let mut tokens: Vec<(&str, &str)> = Vec::new();
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::PLUS.as_str(), "+"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        let mut lexer = Lexer::new(INPUT.chars().collect());

        for (key, value) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
        }

        assert!(true)
    }
}
