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
    line: usize
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut new_lexer = Lexer {
            input: input.to_owned(),
            position: 0,
            next_position: 0,
            c: '\0',
            line: 1
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn next_token(&mut self) -> Token {

        self.maybe_read_whitespace();

        let new_token: Token = match self.c {
            '=' => Token{t: Reserved::ASSIGN.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            ';' => Token{t: Reserved::SEMICOLON.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            '(' => Token{t: Reserved::LPAREN.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            ')' => Token{t: Reserved::RPAREN.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            ',' => Token{t: Reserved::COMMA.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            '+' => Token{t: Reserved::PLUS.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            '{' => Token{t: Reserved::LBRACE.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            '}' => Token{t: Reserved::RBRACE.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            '\0' => Token{t: Reserved::EOF.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.maybe_read_identifier();
                return Token {
                    t: Reserved::dispatch_keyword(&identifier).to_string().to_string(),
                    value: identifier,
                    line: Some(self.line),
                    position: Some(self.position),
                    filename: None,
                };
            }
            '0'..='9' => {
                let number = self.maybe_read_number();
                return Token {
                    t: Reserved::INT.to_string(),
                    value: number.to_string(),
                    line: Some(self.line),
                    position: Some(self.position),
                    filename: None,
                };
            }
            _ => Token{t: Reserved::ILLEGAL.to_string(), value: self.c.to_string(), filename: None, line: Some(self.line), position: Some(self.position)},
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
                '\n' | '\r' => {
                    self.line += 1;
                    self.read_char();
                }
                ' ' | '\t'  => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }
}

