use crate::ast::{Expression, Identifer, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Kind, Token};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        println!("START STATEMENT: {:?}", self.current_token);
        match &self.current_token.kind {
            Kind::Let => {
                if !self.expect_peek(Kind::Ident) {
                    return None;
                }

                let name = Identifer {
                    token: self.current_token.clone(),
                    value: self.current_token.literal.clone(),
                };

                if !self.expect_peek(Kind::Assign) {
                    return None;
                }

                while self.current_token.kind != Kind::SemiColon {
                    self.next_token();
                }

                Some(Statement::Let {
                    token: self.current_token.clone(),
                    name,
                    value: Expression::Temporary,
                })
            }
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        println!("START PROGRAM: {:?}", self.current_token);

        while self.current_token.kind != Kind::Eof {
            let statement = self.parse_statement();
            println!("STATEMENT: {statement:?}");
            if let Some(s) = statement {
                program.statements.push(s);
            }
            self.next_token();
        }

        program
    }

    fn peek_error(&mut self, token_kind: Kind) {
        self.errors.push(format!(
            "expected next token to be {token_kind:?}, got {:?} instead",
            self.peek_token.kind
        ));
    }

    fn expect_peek(&mut self, expected_token: Kind) -> bool {
        println!("PEEK: {:?}", self.peek_token);
        if expected_token == self.peek_token.kind {
            println!("TOKEN SUCCESS: {expected_token:?}");
            self.next_token();
            return true;
        } else {
            println!("TOKEN FAIL: {expected_token:?}");
            self.peek_error(expected_token);
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Node, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn valid_let_statement(statement: &Statement, let_name: &str) -> bool {
        if statement.token_literal() != "let" {
            return false;
        }

        match statement {
            Statement::Let {
                token: _,
                name,
                value: _,
            } => name.value == let_name && name.token_literal() == let_name,
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.len() != 0 {
            println!("Got {} errors.", parser.errors.len());
            for error in &parser.errors {
                println!("ERROR: {error}");
            }

            assert!(false);
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
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 3);
        for (i, t) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            assert!(valid_let_statement(statement, t));
        }
    }
}
