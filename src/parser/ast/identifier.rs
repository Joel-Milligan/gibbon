use std::fmt::Display;

use crate::token::Token;

use super::Node;

#[derive(Debug)]
pub struct Identifer {
    pub token: Token,
    pub value: String,
}

impl Node for Identifer {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Display for Identifer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
