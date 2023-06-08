use std::{panic, println, unimplemented};

use crate::{
    ast::{
        expressions::identifier_expression::Identifier,
        program::Program,
        statements::{declare_statement::DeclareStatement, return_statement::ReturnStatement},
    },
    enums::{keyword::Keyword, token_type::TokenType},
    lexer::Lexer,
    token::Token,
    traits::Statement,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    pub fn parse(&mut self) -> Result<Program> {
        let mut new_program = Program::new();

        loop {
            if let Some(token) = &self.current_token {
                if token.t == TokenType::EOF {
                    break;
                }

                let parsed_stmt = self.parse_statement()?;
                new_program.statements.push(parsed_stmt);

                self.next_token();
            } else {
                return Err("Error parsing an apparant `None` token in the program".into());
            }
        }

        Ok(new_program)
    }

    pub fn parse_statement(&mut self) -> Result<Box<dyn Statement>> {
        if let Some(token) = &self.current_token {
            match token.t {
                TokenType::KEYWORD(kw) => match kw {
                    Keyword::LET | Keyword::CONST | Keyword::VAR | Keyword::AUTO => {
                        self.parse_declare_statement()
                    }
                    Keyword::RETURN => self.parse_return_statement(),
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
                _ => Err(format!("Unexpected keyword token encountered: {:#?}", token).into()),
            }
        } else {
            Err("Expected a TOKEN, None was given.".into())
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier> {
        if !self.cmp_next_token_type(TokenType::IDENT) {
            return Err("Could not find a valid IDENTIFIER".into());
        }

        let identifier_token = self.next_token.clone().unwrap();

        Ok(Identifier::new(
            identifier_token.clone(),
            identifier_token.t.to_string(),
        ))
    }

    fn parse_type_specifier(&mut self) -> Option<Token> {
        if self.cmp_next_token_type(TokenType::COLON) {
            self.next_token();
            if !self.cmp_next_token_type(TokenType::IDENT) {
                return None;
            }

            self.next_token.clone()
        } else {
            None
        }
    }

    fn parse_declare_statement(&mut self) -> Result<Box<dyn Statement>> {
        // (LET | CONST | VAR | AUTO) INDENT ASSIGN EXPRESSION SEMICOLON
        // (LET | CONST | VAR | AUTO) INDENT COLON INDENT ASSIGN EXPRESSION SEMICOLOR
        // (LET | CONST | VAR | AUTO) INDENT SEMICOLON
        // (LET | CONST | VAR | AUTO) INDENT COLON INDENT SEMICOLON

        let identifier = self.parse_identifier()?;
        let mut stmt =
            DeclareStatement::new(self.current_token.clone().unwrap(), None, identifier, None);

        self.next_token();

        if let Some(type_specifier_token) = self.parse_type_specifier() {
            stmt.type_specifier = Some(type_specifier_token);
            self.next_token();
        }

        if self.cmp_next_token_type(TokenType::SEMICOLON) {
            self.next_token();
            return Ok(Box::new(stmt));
        }

        if !self.cmp_next_token_type(TokenType::ASSIGN) {
            return Err(format!(
                "Expected an ASSIGN Token, UNEXPECTED: {:#?}",
                self.next_token
            )
            .into());
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
                    return Err(format!(
                        "Expected a SEMICOLON Token, UNEXPECTED: {:#?}",
                        self.current_token
                    )
                    .into());
                }

                self.next_token();
            } else {
                break;
            }
        }

        return Ok(Box::new(stmt));
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>> {
        loop {
            if let Some(_) = &self.current_token {
                if self.cmp_current_token_type(TokenType::SEMICOLON) {
                    break;
                }

                if self.cmp_current_token_type(TokenType::EOF)
                    || self.cmp_current_token_type(TokenType::ILLEGAL)
                {
                    return Err(format!(
                        "Expected a SEMICOLON Token, UNEXPECTED: {:#?}",
                        self.current_token
                    )
                    .into());
                }

                self.next_token();
            } else {
                break;
            }
        }

        let stmt = ReturnStatement::new(self.current_token.clone().unwrap(), None);

        return Ok(Box::new(stmt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let code = "let x = 5;";
        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_error() {
        let code = "let x 5;";
        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();

        assert!(result.is_err());
    }
}
