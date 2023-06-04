#[allow(dead_code)]
#[allow(unused)]
use self::{reserved::Reserved, token::Token};

pub mod reserved;
pub mod token;
mod test;

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

