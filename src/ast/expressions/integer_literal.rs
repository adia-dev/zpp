use crate::{
    token::Token,
    traits::{Expression, Node},
};

#[derive(Clone, Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i32) -> Self {
        Self { token, value }
    }
}

impl Node for IntegerLiteral {
    fn get_token(&self) -> String {
        self.token.value.clone()
    }
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.token.value.clone()
    }
}

impl Expression for IntegerLiteral {
    fn eval(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_integer_literal_expression() {
        let code = r#"
            5;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        println!("{:#?}", program);

        assert!(program.is_ok());

        assert_eq!(program.unwrap().statements.len(), 1);
    }
}
