use crate::traits::{Node, Statement};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn get_token(&self) -> String {
        if self.statements.is_empty() {
            "".to_owned()
        } else {
            self.statements[0].get_token()
        }
    }
}
