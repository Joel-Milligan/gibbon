use std::fmt::Display;

use super::{BlockStatement, Identifer, Node};

#[derive(Debug, PartialEq)]
pub enum Expression {
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
    FunctionLiteral {
        parameters: Vec<Identifer>,
        body: BlockStatement,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(value) => value.to_string(),
            Expression::IntegerLiteral(value) => value.to_string(),
            Expression::BooleanLiteral(value) => value.to_string(),
            Expression::Prefix { operator, right: _ } => operator.to_string(),
            Expression::Infix {
                left: _,
                operator,
                right: _,
            } => operator.to_string(),
            Expression::If { .. } => "if".to_string(),
            Expression::FunctionLiteral { .. } => "fn".to_string(),
            Expression::Call { .. } => "(".to_string(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Prefix { operator, right } => write!(f, "({operator}{right})"),
            Expression::Infix {
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                if let Some(alternative) = alternative {
                    write!(f, "if{condition} {consequence}else {alternative}")
                } else {
                    write!(f, "if{condition} {consequence}")
                }
            }
            Expression::FunctionLiteral { parameters, body } => {
                let parameters = parameters
                    .iter()
                    .map(|p| p.token_literal())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "fn({parameters}) {body}")
            }
            Expression::Call {
                function,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{function}({arguments})")
            }
            _ => write!(f, "{}", self.token_literal()),
        }
    }
}
