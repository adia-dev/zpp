use std::write;
use crate::parser::Parser;

pub trait Node {
    fn get_token(&self) -> String;
}

pub trait Statement: Node {
    fn execute(&self);
}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement{{{}}}", self.get_token())
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

