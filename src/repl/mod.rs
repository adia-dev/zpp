use std::io::{stdout, Write};

use crate::{enums::token_type::TokenType, lexer::Lexer, queue::Queue, token::Token};

pub struct REPL {
    tokens: Queue<Token>,
    buffer: String,
    index: usize,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            tokens: Queue::new(),
            buffer: String::new(),
            index: 1,
        }
    }

    pub fn start(&mut self) {
        println!("Welcome to the ZPP REPL - Rust Interpreter Version 0.1.0");

        loop {
            if !self.read() {
                break;
            }

            self.eval();
            self.print();
        }
    }

    fn read(&mut self) -> bool {
        print!("({}) >> ", self.index);
        stdout().flush().unwrap();

        self.tokens.clear();
        self.buffer.clear();
        self.index += 1;

        let bytes_read = std::io::stdin().read_line(&mut self.buffer);

        return bytes_read.is_ok();
    }

    fn eval(&mut self) {
        let mut lexer = Lexer::new(self.buffer.chars().collect());

        loop {
            let token = lexer.next_token();
            match token.t {
                TokenType::ILLEGAL => {
                    panic!("ILLEGAL TOKEN ENCOUNTERED");
                }
                TokenType::EOF => {
                    println!("End of file (EOF)");
                    break;
                }
                _ => self.tokens.enqueue(token),
            }
        }
    }

    fn print(&mut self) {
        let tokens: Vec<Token> = self.tokens.clone().into_iter().collect();

        for t in tokens {
            println!("Type: {:?}, Literal: {}", t.t, t.value);
        }
    }
}
