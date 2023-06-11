use crate::parser::Parser;
use std::write;

pub trait Node: ToString {
    fn get_token(&self) -> String;
}

pub trait Statement: Node {
    fn execute(&self);
}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement{{{}}}", self.to_string())
    }
}

pub trait Expression: Node {
    fn eval(&self) -> String;
}

impl core::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression{{{}}}", self.eval())
    }
}
