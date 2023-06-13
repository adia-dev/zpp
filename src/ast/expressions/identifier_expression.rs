use crate::{
    token::Token,
    traits::{Expression, Node},
};

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}

impl Node for Identifier {
    fn get_token(&self) -> String {
        self.token.value.clone()
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.token.value.clone()
    }
}

impl Expression for Identifier {
    fn eval(&self) -> String {
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_identifier_expression() {
        let code = r#"
            let x = 5;
            var name: String;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(program.is_ok());

        let statements = program.unwrap().statements;

        assert_eq!(statements.len(), 2);
    }
}
