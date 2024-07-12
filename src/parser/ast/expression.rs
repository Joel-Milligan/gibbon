use std::fmt::Display;

use crate::token::Token;

use super::Node;

#[derive(Debug)]
pub enum Expression {
    Temporary,
    Identifier { token: Token, value: String },
    IntegerLiteral { token: Token, value: i64 },
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        String::new()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
