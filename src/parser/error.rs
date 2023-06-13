use std::{error::Error, fmt::Display};

use crate::{enums::token_type::TokenType, token::Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorCode {
    Unexpected = 1_000,
    UnexpectedEOF,
    UnexpectedToken,
    InvalidExpression,
    MissingSemicolon,
    UnexpectedCharacter,
    InvalidAssignmentTarget,
}

impl Display for ParserErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Box<dyn Error>> for ParserError {
    fn from(error: Box<dyn Error>) -> Self {
        ParserError::unexpected(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    code: ParserErrorCode,
    message: String,
    token: Option<Token>,
    at: Option<usize>,
    to: Option<usize>,
}

impl ParserError {
    pub fn new(
        code: ParserErrorCode,
        message: String,
        token: Option<Token>,
        at: Option<usize>,
        to: Option<usize>,
    ) -> Self {
        Self {
            code,
            message,
            token,
            at,
            to,
        }
    }

    pub fn unexpected(error: String) -> ParserError {
        ParserErrorBuilder::new(
            ParserErrorCode::Unexpected,
            format!("Unexpected: {}", error),
        )
        .build()
    }

    pub fn unexpected_token(token: Token, expected: TokenType) -> Self {
        ParserErrorBuilder::new(
            ParserErrorCode::UnexpectedToken,
            format!("Unexpected token: '{}', expected: {}", token.t, expected),
        )
        .with_token(token)
        .build()
    }

    pub fn unexpected_eof() -> Self {
        ParserErrorBuilder::new(ParserErrorCode::UnexpectedEOF, "Unexpected end of file").build()
    }

    pub fn invalid_expression() -> Self {
        ParserErrorBuilder::new(ParserErrorCode::InvalidExpression, "Invalid expression").build()
    }

    pub fn missing_semicolon() -> Self {
        ParserErrorBuilder::new(ParserErrorCode::MissingSemicolon, "Missing semicolon").build()
    }

    pub fn unexpected_character(ch: char) -> Self {
        ParserErrorBuilder::new(
            ParserErrorCode::UnexpectedCharacter,
            format!("Unexpected character: '{}'", ch),
        )
        .build()
    }

    pub fn invalid_assignment_target() -> Self {
        ParserErrorBuilder::new(
            ParserErrorCode::InvalidAssignmentTarget,
            "Invalid assignment target",
        )
        .build()
    }


    pub fn code(&self) -> &ParserErrorCode {
        &self.code
    }

    pub fn set_code(&mut self, code: ParserErrorCode) {
        self.code = code;
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    pub fn set_token(&mut self, token: Option<Token>) {
        self.token = token;
    }

    pub fn at(&self) -> Option<usize> {
        self.at
    }

    pub fn set_at(&mut self, at: Option<usize>) {
        self.at = at;
    }

    pub fn to(&self) -> Option<usize> {
        self.to
    }

    pub fn set_to(&mut self, to: Option<usize>) {
        self.to = to;
    }

}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error({}): {}", self.code, self.message)
    }
}

pub struct ParserErrorBuilder {
    code: ParserErrorCode,
    message: String,
    token: Option<Token>,
    at: Option<usize>,
    to: Option<usize>,
}

impl ParserErrorBuilder {
    pub fn new(code: ParserErrorCode, message: impl Into<String>) -> Self {
        ParserErrorBuilder {
            code,
            message: message.into(),
            token: None,
            at: None,
            to: None,
        }
    }

    pub fn with_token(mut self, token: Token) -> Self {
        if let Some(line) = token.line {
            self.at = Some(line);
        }
        self.token = Some(token);
        self
    }

    pub fn with_range(mut self, at: usize, to: usize) -> Self {
        self.at = Some(at);
        self.to = Some(to);
        self
    }

    pub fn build(self) -> ParserError {
        ParserError {
            code: self.code,
            message: self.message,
            token: self.token,
            at: self.at,
            to: self.to,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_parser_error_display() {
        let error = ParserError::new(
            ParserErrorCode::UnexpectedToken,
            "Unexpected token".to_string(),
            None,
            None,
            None,
        );
        assert_eq!(
            format!("{}", error),
            "Error(UnexpectedToken): Unexpected token"
        );
    }

    #[test]
    fn test_parser_error_builder() {
        let token = Token {
            t: TokenType::INT,
            value: "42".to_string(),
            line: Some(1),
            position: Some(2),
            filename: Some("example.rs".to_string()),
        };

        let error = ParserErrorBuilder::new(ParserErrorCode::UnexpectedToken, "Unexpected token")
            .with_token(token.clone())
            .build();

        assert_eq!(error.code(), &ParserErrorCode::UnexpectedToken);
        assert_eq!(error.message(), "Unexpected token");
        assert_eq!(error.token(), Some(&token));
        assert_eq!(error.at(), Some(1));
        assert_eq!(error.to(), None);
    }

    #[test]
    fn test_parser_error_methods() {
        let token = Token {
            t: TokenType::IDENT,
            value: "foo".to_string(),
            line: Some(1),
            position: Some(2),
            filename: Some("example.rs".to_string()),
        };

        let mut error = ParserError::new(
            ParserErrorCode::UnexpectedToken,
            "Unexpected token".to_string(),
            Some(token.clone()),
            Some(3),
            Some(5),
        );

        error.set_code(ParserErrorCode::InvalidExpression);
        assert_eq!(error.code(), &ParserErrorCode::InvalidExpression);

        error.set_message("Invalid expression".to_string());
        assert_eq!(error.message(), "Invalid expression");

        let new_token = Token {
            t: TokenType::ASSIGN,
            value: "=".to_string(),
            line: Some(2),
            position: Some(4),
            filename: Some("example.rs".to_string()),
        };
        error.set_token(Some(new_token.clone()));
        assert_eq!(error.token(), Some(&new_token));

        error.set_at(Some(6));
        assert_eq!(error.at(), Some(6));

        error.set_to(Some(8));
        assert_eq!(error.to(), Some(8));
    }

    #[test]
    fn test_test_result_ok() {
        let value = 42;
        let result: TestResult<i32> = Ok(value);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);
    }

    #[test]
    fn test_test_result_err() {
        let error_message = "An error occurred";
        let result: TestResult<i32> = Err(Box::new(ParserError::new(
            ParserErrorCode::InvalidExpression,
            error_message.to_string(),
            None,
            None,
            None,
        )));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), format!("Error(InvalidExpression): {}", error_message));
        assert_eq!(
            err.downcast_ref::<ParserError>(),
            Some(&ParserError::new(
                ParserErrorCode::InvalidExpression,
                error_message.to_string(),
                None,
                None,
                None,
            ))
        );
    }

    #[test]
    fn test_result_conversion() {
        use std::convert::From;

        let value = 42;
        let ok_result: Result<i32, ParserError> = Ok(value);
        let converted_result: TestResult<i32> = match ok_result {
            Ok(value) => Ok(value),
            Err(err) => Err(Box::new(err) as Box<dyn Error>),
        };

        assert!(converted_result.is_ok());
        assert_eq!(converted_result.unwrap(), value);

        let error_message = "An error occurred";
        let err_result: Result<i32, ParserError> = Err(ParserError::new(
            ParserErrorCode::InvalidExpression,
            error_message.to_string(),
            None,
            None,
            None,
        ));
        let converted_result: TestResult<i32> = match err_result {
            Ok(value) => Ok(value),
            Err(err) => Err(Box::new(err) as Box<dyn Error>),
        };

        assert!(converted_result.is_err());
        let err = converted_result.unwrap_err();
        assert_eq!(err.to_string(), format!("Error(InvalidExpression): {}", error_message));
        assert_eq!(
            err.downcast_ref::<ParserError>(),
            Some(&ParserError::new(
                ParserErrorCode::InvalidExpression,
                error_message.to_string(),
                None,
                None,
                None,
            ))
        );
    }
}
