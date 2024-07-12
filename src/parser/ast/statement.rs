use std::fmt::Display;

use crate::token::Token;

use super::{Expression, Identifer, Node};

#[derive(Debug)]
pub enum Statement {
    Let {
        name: Identifer,
        value: Expression,
    },
    Return(Expression),
    Expression {
        token: Token,
        expression: Expression,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let { .. } => "let".to_string(),
            Self::Return(_) => "return".to_string(),
            Self::Expression {
                token,
                expression: _,
            } => token.literal.to_string(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value } => {
                writeln!(f, "let {} = {};", name, value)
            }
            Statement::Return(value) => writeln!(f, "return {value};"),
            Statement::Expression {
                token: _,
                expression,
            } => writeln!(f, "{expression}"),
        }
    }
}
