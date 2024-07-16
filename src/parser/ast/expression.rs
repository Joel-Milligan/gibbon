use std::fmt::Display;

use super::{BlockStatement, Node};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Temporary,
    Identifier(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Temporary => "".to_string(),
            Expression::Identifier(value) => value.to_string(),
            Expression::IntegerLiteral(value) => value.to_string(),
            Expression::BooleanLiteral(value) => value.to_string(),
            Expression::Prefix { operator, right } => format!("{operator}{right}"),
            Expression::Infix {
                left,
                operator,
                right,
            } => format!("{left}{operator}{right}"),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                if let Some(alternative) = alternative {
                    return format!("if{condition} {consequence}else {alternative}");
                } else {
                    return format!("if{condition} {consequence}");
                }
            }
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
