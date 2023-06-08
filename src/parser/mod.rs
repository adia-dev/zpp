use std::{panic, println, unimplemented};

use crate::{
    ast::{
        expressions::identifier_expression::Identifier, program::Program,
        statements::declare_statement::DeclareStatement,
    },
    enums::{keyword::Keyword, token_type::TokenType},
    lexer::Lexer,
    token::Token,
    traits::Statement,
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut new_parser = Self {
            lexer,
            current_token: None,
            next_token: None,
        };

        new_parser.next_token();
        new_parser.next_token();

        new_parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = Some(self.lexer.next_token());
    }

    pub fn cmp_next_token_type(&self, token_type: TokenType) -> bool {
        if let Some(token) = &self.next_token {
            token.t == token_type
        } else {
            false
        }
    }

    pub fn cmp_current_token_type(&self, token_type: TokenType) -> bool {
        if let Some(token) = &self.current_token {
            token.t == token_type
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut new_program = Program::new();

        loop {
            if let Some(token) = &self.current_token {
                if token.t == TokenType::EOF {
                    break;
                }

                let parsed_stmt = self.parse_statement();
                if let Some(stmt) = parsed_stmt {
                    new_program.statements.push(stmt);
                }

                self.next_token();
            } else {
                break;
            }
        }

        new_program
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        if let Some(token) = &self.current_token {
            match token.t {
                TokenType::KEYWORD(kw) => match kw {
                    Keyword::LET | Keyword::CONST | Keyword::VAR | Keyword::AUTO => {
                        self.parse_declare_statement()
                    }
                    Keyword::RETURN => todo!(),
                    Keyword::FOR => todo!(),
                    Keyword::WHILE => todo!(),
                    Keyword::FUNCTION => todo!(),
                    Keyword::IF => todo!(),
                    Keyword::ELSE => todo!(),
                    Keyword::DO => todo!(),
                    Keyword::END => todo!(),
                    Keyword::TRUE => todo!(),
                    Keyword::FALSE => todo!(),
                    Keyword::UNDEFINED => todo!(),
                },
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_declare_statement(&mut self) -> Option<Box<dyn Statement>> {
        // (LET | CONST | VAR | AUTO) INDENT ASSIGN EXPRESSION SEMICOLON
        // (LET | CONST | VAR | AUTO) INDENT COLON INDENT ASSIGN EXPRESSION SEMICOLOR
        // (LET | CONST | VAR | AUTO) INDENT SEMICOLON
        // (LET | CONST | VAR | AUTO) INDENT COLON INDENT SEMICOLON

        if !self.cmp_next_token_type(TokenType::IDENT) {
            println!(
                "Expected an Identifier Token., UNEXPECTED: {:#?}",
                self.next_token
            );
            return None;
        }

        let identifier_token = self.next_token.clone().unwrap();

        let identifier = Identifier::new(identifier_token.clone(), identifier_token.t.to_string());
        let mut stmt =
            DeclareStatement::new(self.current_token.clone().unwrap(), None, identifier, None);

        self.next_token();

        // maybe type identifier
        if self.cmp_next_token_type(TokenType::COLON) {
            self.next_token();
            if !self.cmp_next_token_type(TokenType::IDENT) {
                println!(
                    "Expected an Identifier Token., UNEXPECTED: {:#?}",
                    self.next_token
                );
                return None;
            }

            stmt.type_specifier = self.next_token.clone();
            self.next_token();
        }

        if self.cmp_next_token_type(TokenType::SEMICOLON) {
            return Some(Box::new(stmt));
        }

        if !self.cmp_next_token_type(TokenType::ASSIGN) {
            println!(
                "Expected an ASSIGN Token., UNEXPECTED: {:#?}",
                self.next_token
            );
            return None;
        }

        self.next_token();

        loop {
            if let Some(_) = &self.current_token {
                if self.cmp_current_token_type(TokenType::SEMICOLON) {
                    break;
                }

                if self.cmp_current_token_type(TokenType::EOF)
                    || self.cmp_current_token_type(TokenType::ILLEGAL)
                {
                    println!(
                        "Expected a SEMICOLON Token., UNEXPECTED: {:#?}",
                        self.current_token
                    );
                }

                self.next_token();
            } else {
                break;
            }
        }

        return Some(Box::new(stmt));
    }
}
