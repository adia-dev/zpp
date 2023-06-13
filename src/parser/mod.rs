use std::{collections::HashMap, panic, println, unimplemented};
pub mod error;
pub mod precedence;

use crate::{
    ast::{
        expressions::{
            identifier_expression::Identifier, integer_literal::IntegerLiteral,
            prefix_expression::PrefixExpression,
        },
        program::Program,
        statements::{
            declare_statement::DeclareStatement, expression_statement::ExpressionStatement,
            return_statement::ReturnStatement,
        },
    },
    enums::{arithmetic::Arithmetic, keyword::Keyword, logicop::LogicOp, token_type::TokenType},
    lexer::Lexer,
    token::Token,
    traits::{Expression, Statement},
};

use self::{error::ParserError, precedence::Precedence};

type Result<T> = std::result::Result<T, ParserError>;
type ExpressionParserFn<'a> = fn(&mut Parser<'a>) -> Result<Box<dyn Expression>>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    errors: Vec<ParserError>,
    current_token: Option<Token>,
    next_token: Option<Token>,
    prefix_funs: HashMap<TokenType, ExpressionParserFn<'a>>,
    infix_funs: HashMap<TokenType, ExpressionParserFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut prefix_funs: HashMap<TokenType, ExpressionParserFn<'a>> = HashMap::new();
        prefix_funs.insert(TokenType::IDENT, Self::parse_identifier);
        prefix_funs.insert(TokenType::INT, Self::parse_integer_literal_expression);
        prefix_funs.insert(
            TokenType::ARITHMETIC(Arithmetic::MINUS),
            Self::parse_prefix_expression,
        );
        prefix_funs.insert(
            TokenType::LOGICOP(LogicOp::NOT),
            Self::parse_prefix_expression,
        );

        let mut new_parser = Self {
            lexer,
            errors: Vec::new(),
            current_token: None,
            next_token: None,
            prefix_funs,
            infix_funs: HashMap::new(), // later
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

                let parsed_stmt = match self.parse_statement() {
                    Ok(stmt) => stmt,
                    Err(err) => {
                        self.errors.push(err.into());
                        return Err(ParserError::unexpected("An error occurred.".to_owned()));
                    }
                };
                new_program.statements.push(parsed_stmt);

                self.next_token();
            } else {
                return Err(ParserError::unexpected_eof());
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
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(ParserError::unexpected_eof())
        }
    }

    fn parse_identifier(&mut self) -> Result<Box<dyn Expression>> {
        if let Some(tok) = &self.current_token {
            if tok.t != TokenType::IDENT {
                return Err(ParserError::unexpected_token(tok.clone(), TokenType::IDENT));
            }

            Ok(Box::new(Identifier::new(tok.clone(), tok.t.to_string())))
        } else {
            Err(ParserError::unexpected_eof())
        }
    }

    fn parse_integer_literal_expression(&mut self) -> Result<Box<dyn Expression>> {
        if let Some(token) = &self.current_token {
            match token.value.parse::<i32>() {
                Ok(int) => {
                    let integer = IntegerLiteral::new(token.clone(), int);
                    Ok(Box::new(integer))
                }
                Err(_) => Err(ParserError::invalid_assignment_target()),
            }
        } else {
            Err(ParserError::unexpected_eof())
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>> {
        if let Some(token) = &self.current_token.clone() {
            self.next_token();
            let rhs = self.parse_expression(Precedence::Prefix)?;

            Ok(Box::new(PrefixExpression::new(
                token.clone(),
                token.value.to_string(),
                rhs,
            )))
        } else {
            Err(ParserError::unexpected_eof())
        }
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

        let identifier: Option<Box<dyn Expression>>;
        if let Some(tok) = &self.next_token {
            if tok.t != TokenType::IDENT {
                return Err(ParserError::unexpected_token(tok.clone(), TokenType::IDENT));
            }

            identifier = Some(Box::new(Identifier::new(tok.clone(), tok.t.to_string())))
        } else {
            return Err(ParserError::unexpected_eof());
        }

        let mut stmt = DeclareStatement::new(
            self.current_token.clone().unwrap(),
            None,
            identifier.unwrap(),
            None,
        );

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
            if let Some(_) = &self.next_token {
                return Err(ParserError::unexpected_token(
                    self.next_token.clone().unwrap(),
                    TokenType::ASSIGN,
                ));
            } else {
                return Err(ParserError::unexpected_eof());
            }
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
                    if let Some(_) = &self.current_token {
                        return Err(ParserError::unexpected_token(
                            self.current_token.clone().unwrap(),
                            TokenType::SEMICOLON,
                        ));
                    } else {
                        return Err(ParserError::unexpected_eof());
                    }
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
                    if let Some(_) = &self.current_token {
                        return Err(ParserError::unexpected_token(
                            self.current_token.clone().unwrap(),
                            TokenType::SEMICOLON,
                        ));
                    } else {
                        return Err(ParserError::unexpected_eof());
                    }
                }

                self.next_token();
            } else {
                break;
            }
        }

        let stmt = ReturnStatement::new(self.current_token.clone().unwrap(), None);

        return Ok(Box::new(stmt));
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>> {
        if self.current_token.is_none() {
            return Err(ParserError::unexpected_eof());
        }

        let expression = self.parse_expression(Precedence::Lowest)?;
        let stmt = ExpressionStatement::new(self.current_token.clone().unwrap(), expression);

        if self.cmp_next_token_type(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Ok(Box::new(stmt));
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Result<Box<dyn Expression>> {
        if let Some(token) = &self.current_token {
            if let Some(fun) = self
                .prefix_funs
                .get(&token.t)
            {
                return fun(self);
            }else {
                Err(ParserError::invalid_expression())
            }
        } else {
            Err(ParserError::unexpected_eof())
        }
    }

    pub fn errors(&self) -> &[ParserError] {
        self.errors.as_ref()
    }

    pub fn set_errors(&mut self, errors: Vec<ParserError>) {
        self.errors = errors;
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
