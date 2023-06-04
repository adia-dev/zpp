#[allow(dead_code)]
#[allow(unused)]
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

    pub fn next_token(&mut self) -> Token {
        self.maybe_read_whitespace();

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
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.maybe_read_identifier();
                return Token {
                    t: Reserved::dispatch_keyword(&identifier).as_str().to_string(),
                    value: identifier,
                    line: None,
                    filename: None,
                };
            }
            '0'..='9' => {
                let number = self.maybe_read_number();
                return Token {
                    t: Reserved::INT.as_str().to_string(),
                    value: number.to_string(),
                    line: None,
                    filename: None,
                };
            }
            _ => Token::new(Reserved::ILLEGAL.as_str(), &self.c.to_string()),
        };

        self.read_char();

        new_token
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

    fn maybe_read_identifier(&mut self) -> String {
        let position = self.position;

        while self.c.is_alphanumeric() {
            self.read_char();
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn maybe_read_number(&mut self) -> String {
        let position = self.position;

        while self.c.is_numeric() {
            self.read_char();
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn maybe_read_until<F>(&mut self, mut condition: F) -> String
    where
        F: FnMut() -> bool,
    {
        let position = self.position;

        while condition() {
            self.read_char();
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn maybe_read_whitespace(&mut self) {
        loop {
            match self.c {
                ' ' | '\t' | '\n' | '\r' => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod lexer_tests {

    use std::assert_eq;

    use crate::lexer::{reserved::Reserved, token::Token, Lexer};

    const INPUT: &'static str = "=+(){},;";
    const ZPP_FILES_DIR: &'static str = "data/";
    const CODE: &'static str = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y
        }

        let result = add(five, ten);"#;

    #[test]
    fn test_next_token_in_sample() {
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
    }

    #[test]
    fn test_next_token_in_code() {
        let mut tokens: Vec<(&str, &str)> = Vec::new();

        tokens.push((Reserved::LET.as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "five"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::INT.as_str(), "5"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        tokens.push((Reserved::LET.as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "ten"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::INT.as_str(), "10"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        tokens.push((Reserved::LET.as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "add"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::FUNCTION.as_str(), "fn"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::IDENT.as_str(), "x"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::IDENT.as_str(), "y"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::IDENT.as_str(), "x"));
        tokens.push((Reserved::PLUS.as_str(), "+"));
        tokens.push((Reserved::IDENT.as_str(), "y"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));

        tokens.push((Reserved::LET.as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "result"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::IDENT.as_str(), "add"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::IDENT.as_str(), "five"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::IDENT.as_str(), "ten"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::EOF.as_str(), "\0"));

        let mut lexer = Lexer::new(CODE.chars().collect());

        for (key, value) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            println!("type: {}, value: {}", tok.t, tok.value);

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
        }
    }
}
