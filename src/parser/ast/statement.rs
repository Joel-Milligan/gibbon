use std::fmt::Display;

use crate::token::Token;

use super::{Expression, Identifer, Node};

#[derive(Debug, PartialEq)]
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
                write!(f, "let {} = {};", name, value)
            }
            Statement::Return(value) => write!(f, "return {value};"),
            Statement::Expression {
                token: _,
                expression,
            } => write!(f, "{expression}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        "{".to_string()
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }

        Ok(())
    }
}
