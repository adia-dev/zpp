use crate::{
    token::Token,
    traits::{Expression, Node, Statement},
};

pub struct ReturnStatement {
    token: Token,
    value: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        Self { token, value }
    }
}

impl core::fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReturnStatement{{{}}}", self.get_token())
    }
}

impl Node for ReturnStatement {
    fn get_token(&self) -> String {
        self.token.value.to_string()
    }
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        if let Some(v) = &self.value {
            format!("return {};", v.to_string())
        } else {
            "return;".to_string()
        }
    }
}

impl Statement for ReturnStatement {
    fn execute(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    pub fn test_return_statement() {
        let code = r#"
            let x = 0;
            return x;


            return true;
            return;

            return true;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(program.is_ok());

        assert_eq!(program.unwrap().statements.len(), 5);
    }
}
