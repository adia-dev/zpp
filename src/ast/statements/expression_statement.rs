use crate::{
    token::Token,
    traits::{Expression, Node, Statement},
};

pub struct ExpressionStatement {
    token: Token,
    expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl core::fmt::Debug for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExpressionStatement{{{}}}", self.get_token())
    }
}

impl Node for ExpressionStatement {
    fn get_token(&self) -> String {
        format!("return EXPR;")
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_return_statement() {
        let code = r#"
            foobar;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        println!("{:#?}", program);

        assert!(program.is_ok());

        assert_eq!(program.unwrap().statements.len(), 1);
    }
}
