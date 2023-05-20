use crate::lexer::Lexer;
use crate::token::{Kind, Token};

trait Node {
    fn token_literal(&self) -> String;
}

enum Statement {
    Let {
        token: Token,
        name: Identifer,
        value: Expression,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let {
                token: _,
                name: _,
                value: _,
            } => "let".to_string(),
        }
    }
}

enum Expression {
    Temporary,
}

struct Identifer {
    token: Token,
    value: String,
}

impl Node for Identifer {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

struct Program {
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

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let peek_token = lexer.next_token();
        let current_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token.kind {
            Kind::Let => Some(Statement::Let {
                token: self.current_token.clone(),
                name: Identifer {
                    token: self.current_token.clone(),
                    value: self.current_token.literal.clone(),
                },
                value: Expression::Temporary,
            }),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token.kind != Kind::Eof {
            let statement = self.parse_statement();
            if let Some(s) = statement {
                program.statements.push(s);
            }
            self.next_token();
        }

        program
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::{Node, Parser, Statement};

    fn valid_let_statement(statement: &Statement, let_name: &str) -> bool {
        if statement.token_literal() != "let" {
            return false;
        }

        match statement {
            Statement::Let {
                token: _,
                name,
                value: _,
            } => name.value != let_name && name.token_literal() != let_name,
        }
    }

    #[test]
    fn let_statement() {
        // Arrange
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#
        .to_string();

        let tests = vec!["x", "y", "foobar"];

        // Act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // Assert
        assert_eq!(program.statements.len(), 3);
        for (i, t) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            assert!(valid_let_statement(statement, t));
        }
    }
}
