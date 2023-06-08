use crate::{
    token::Token,
    traits::{Expression, Node},
};

#[derive(Clone)]
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

impl Expression for Identifier {
    fn eval(&self) -> String {
        "".to_owned()
    }
}
