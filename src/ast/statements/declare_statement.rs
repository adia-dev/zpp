use crate::{
    ast::expressions::identifier_expression::Identifier,
    enums::{keyword::Keyword, token_type::TokenType},
    token::Token,
    traits::{Expression, Node, Statement},
};

pub struct DeclareStatement {
    token: Token,
    pub type_specifier: Option<Token>,
    pub identifier: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl DeclareStatement {
    pub fn new(
        token: Token,
        type_specifier: Option<Token>,
        identifier: Identifier,
        value: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            type_specifier,
            identifier,
            value,
        }
    }
}

impl core::fmt::Debug for DeclareStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeclareStatement{{{}}}", self.get_token())
    }
}

impl Node for DeclareStatement {
    fn get_token(&self) -> String {
        if let Some(type_specifier) = &self.type_specifier {
            format!(
                "{} {}: {} = EXPR;",
                self.token.value, self.identifier.token.value, type_specifier.value
            )
        } else {
            // self.token.value.clone()
            format!(
                "{} {} = EXPR;",
                self.token.value, self.identifier.token.value
            )
        }
    }
}

impl Statement for DeclareStatement {
    fn execute(&self) {}
}

#[cfg(test)]
mod tests {

    use std::println;

    use crate::{
        ast::{
            expressions::identifier_expression::Identifier,
            statements::declare_statement::DeclareStatement,
        },
        enums::{keyword::Keyword, token_type::TokenType},
        lexer::Lexer,
        parser::Parser,
        token::Token,
    };

    #[test]
    pub fn test_declare_statements() {
        let code = r#"
            let x = 5;
            let name: string = Abdoulaye Dia;
            const WINDOW_WIDTH: uint16_t = 1440;
            let later;
            const PI: float = 3.14;
            let is_active: bool = true;
            auto is_active= user.is_active();
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(program.is_ok());

        assert_eq!(program.unwrap().statements.len(), 7);
    }

    #[test]
    fn test_declare_statement_values() {
        let token = Token::new(TokenType::KEYWORD(Keyword::LET), "let", None, None, None);
        let type_specifier = Some(Token::new(TokenType::IDENT, "int", None, None, None));
        let identifier = Identifier::new(
            Token::new(TokenType::IDENT, "x", None, None, None),
            "x".to_string(),
        );
        let declare_statement = DeclareStatement::new(token, type_specifier, identifier, None);

        assert_eq!(declare_statement.token.value, "let");
        assert_eq!(declare_statement.type_specifier.unwrap().value, "int");
        assert_eq!(declare_statement.identifier.token.value, "x");
        assert!(declare_statement.value.is_none());
    }
}
