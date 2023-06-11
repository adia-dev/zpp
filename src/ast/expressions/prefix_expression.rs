use crate::{
    token::Token,
    traits::{Expression, Node},
};

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub rhs: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, rhs: Box<dyn Expression>) -> Self {
        Self { token, operator, rhs }
    }
}

impl Node for PrefixExpression {
    fn get_token(&self) -> String {
        self.token.value.clone()
    }
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.rhs.to_string())
    }
}

impl Expression for PrefixExpression {
    fn eval(&self) -> String {
        self.operator.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_integer_literal_expression() {
        let code = r#"
            -5;
            !5;
            !-5;
            -!-!-!-!-!-10000;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(program.is_ok());

        assert_eq!(program.unwrap().statements.len(), 4);
    }
}
