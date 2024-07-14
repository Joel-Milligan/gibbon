use std::fmt::Display;

use super::Node;

#[derive(Debug)]
pub enum Expression {
    Temporary,
    Identifier(String),
    IntegerLiteral(i64),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Temporary => "".to_string(),
            Expression::Identifier(value) => value.to_string(),
            Expression::IntegerLiteral(value) => value.to_string(),
            Expression::Prefix { operator, right } => format!("{operator}{right}"),
            Expression::Infix {
                left,
                operator,
                right,
            } => format!("{left}{operator}{right}"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Prefix { .. } => write!(f, "({})", self.token_literal()),
            Expression::Infix {
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),
            _ => write!(f, "{}", self.token_literal()),
        }
    }
}
