use crate::{
    ast::expressions::identifier_expression::Identifier,
    enums::{keyword::Keyword, token_type::TokenType},
    token::Token,
    traits::{Expression, Node, Statement},
};

pub struct DeclareStatement {
    token: Token,
    pub type_specifier: Option<Token>,
    pub identifier: Box<dyn Expression>,
    pub value: Option<Box<dyn Expression>>,
}

impl DeclareStatement {
    pub fn new(
        token: Token,
        type_specifier: Option<Token>,
        identifier: Box<dyn Expression>,
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
        self.token.value.to_string()
    }
}

impl ToString for DeclareStatement {
    fn to_string(&self) -> String {
        let mut s = format!("{} {}", self.token.value.to_string(), self.identifier.to_string());

        if let Some(t) = &self.type_specifier {
            s.push_str(format!(": {}", t.value.to_string()).as_str());
        };

        if let Some(v) = &self.value {
            s.push_str(format!(" = {}", v.to_string()).as_str());
        }

        s.push(';');

        s
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
        let code = r#"let x = 5;
            let name: string = Abdoulaye Dia;
            const WINDOW_WIDTH: uint16_t = 1440;
            let later;
            const PI: float = 3.14;
            let is_active: bool = true;
            auto is_active= user.is_active();"#;

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

        println!("identifier: {:#?}", identifier);

        let declare_statement = DeclareStatement::new(token, type_specifier, Box::new(identifier), None);
        println!("declare_statement: {:#?}", declare_statement);


        assert_eq!(declare_statement.token.value, "let");
        assert_eq!(declare_statement.type_specifier.unwrap().value, "int");
        assert!(declare_statement.value.is_none());
    }
}
