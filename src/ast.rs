use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let { name: Identifer, value: Expression },
    Return { value: Expression },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let { .. } => "let".to_string(),
            Self::Return { .. } => "return".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Temporary,
}

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

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
