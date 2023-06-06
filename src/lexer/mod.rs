use crate::{
    enums::{
        arithmetic::Arithmetic, bitop::Bitop, cmp::Cmp, keyword::Keyword, logicop::LogicOp,
        token_type::TokenType,
    },
    token::Token,
};

mod test;

#[derive(Default, Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    next_position: usize,
    c: char,
    line: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut new_lexer = Lexer {
            input: input.to_owned(),
            position: 0,
            next_position: 0,
            c: '\0',
            line: 1,
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.maybe_read_whitespace();

        let mut new_token: Token = Token {
            t: TokenType::KEYWORD(Keyword::UNDEFINED),
            value: self.c.to_string(),
            filename: None,
            line: Some(self.line),
            position: Some(self.position),
        };

        match self.c {
            '=' => {
                if self.peek_next_char() == '=' {
                    self.read_char(); // Consume '='
                    new_token.value = "==".to_owned();
                    new_token.t = TokenType::CMP(Cmp::EQUAL);
                } else {
                    new_token.t = TokenType::ASSIGN;
                }
            }
            ';' => new_token.t = TokenType::SEMICOLON,
            ':' => {
                if self.peek_next_char() == ':' {
                    self.read_char();
                    new_token.value.push(':');
                    new_token.t = TokenType::SCOPE;
                } else {
                    new_token.t = TokenType::COLON;
                }
            }
            '(' => new_token.t = TokenType::LPAREN,
            ')' => new_token.t = TokenType::RPAREN,
            '.' => {
                if self.peek_next_char() == '.' {
                    self.read_char();

                    new_token.value = "..".to_owned();
                    if self.peek_next_char() == '=' {
                        self.read_char();

                        new_token.value.push('=');
                        new_token.t = TokenType::IRANGE;
                    } else {
                        new_token.t = TokenType::RANGE;
                    }
                } else {
                    new_token.t = TokenType::DOT;
                }
            }
            ',' => new_token.t = TokenType::COMMA,
            '+' | '-' | '*' | '/' | '%' => {
                if let Some(arithmetic) = Arithmetic::from_str(self.c.to_string().as_str()) {
                    if self.peek_next_char() == self.c {
                        let double = format!("{}{}", self.c, self.c);
                        if let Some(repeated_arithmetic) = Arithmetic::from_str(double.as_str()) {
                            self.read_char();
                            new_token.value = double;
                            new_token.t = TokenType::ARITHMETIC(repeated_arithmetic);
                        }
                    } else {
                        new_token.t = TokenType::ARITHMETIC(arithmetic);
                    }
                } else {
                    new_token.t = TokenType::ILLEGAL;
                }
            }
            '&' | '|' | '~' | '^' => {
                if let Some(bitop) = Bitop::from_str(self.c.to_string().as_str()) {
                    if self.peek_next_char() == self.c {
                        let double = format!("{}{}", self.c, self.c);
                        if let Some(logicop) = LogicOp::from_str(double.as_str()) {
                            self.read_char();
                            new_token.value = double;
                            new_token.t = TokenType::LOGICOP(logicop);
                        }
                    } else {
                        new_token.t = TokenType::BITOP(bitop);
                    }
                } else {
                    new_token.t = TokenType::ILLEGAL;
                }
            }
            '!' => {
                if self.peek_next_char() == '=' {
                    self.read_char(); // Consume '='
                    new_token.t = TokenType::CMP(Cmp::NEQUAL);
                } else {
                    new_token.t = TokenType::LOGICOP(LogicOp::NOT);
                }
            }
            '{' => new_token.t = TokenType::LBRACE,
            '}' => new_token.t = TokenType::RBRACE,
            '<' => {
                if self.peek_next_char() == '=' {
                    self.read_char(); // Consume '='
                    new_token.t = TokenType::CMP(Cmp::LE);
                } else {
                    new_token.t = TokenType::CMP(Cmp::LT);
                }
            }
            '>' => {
                if self.peek_next_char() == '=' {
                    self.read_char(); // Consume '='
                    new_token.t = TokenType::CMP(Cmp::GE);
                } else {
                    new_token.t = TokenType::CMP(Cmp::GT);
                }
            }
            '"' => new_token.t = TokenType::DQUOTE,
            '\'' => new_token.t = TokenType::QUOTE,
            '`' => new_token.t = TokenType::BACKTICK,
            '\0' => new_token.t = TokenType::EOF,
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.maybe_read_identifier();

                if let Some(keyword) = Keyword::from_str(&identifier) {
                    new_token.t = TokenType::KEYWORD(keyword);
                } else {
                    new_token.t = TokenType::IDENT;
                }

                new_token.value = identifier;

                return new_token;
            }
            '0'..='9' => {
                let number = self.maybe_read_number();
                return Token {
                    t: TokenType::INT,
                    value: number.to_string(),
                    line: Some(self.line),
                    position: Some(self.position),
                    filename: None,
                };
            }
            _ => new_token.t = TokenType::ILLEGAL,
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

    pub fn peek_next_char(&mut self) -> char {
        if self.next_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_position]
        }
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
                ' ' | '\t' => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }
}
