use crate::lexer::Lexer;
use crate::parser::ast::{Expression, Identifer};
use crate::token::{Kind, Token};

use super::ast::{Program, Statement};

const LOWEST: i32 = 0;

struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            errors: vec![],
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_expression(&mut self, precendence: i32) -> Option<Expression> {
        let prefix = self.parse_prefix_expression();
        Some(prefix)
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
                    name,
                    value: Expression::Temporary,
                })
            }
            Kind::Return => {
                while self.current_token.kind != Kind::SemiColon {
                    self.next_token();
                }

                Some(Statement::Return(Expression::Temporary))
            }
            _ => {
                let statement = Statement::Expression {
                    token: self.current_token.clone(),
                    expression: self.parse_expression(LOWEST).unwrap(),
                };

                if self.peek_token.kind == Kind::SemiColon {
                    self.next_token();
                }

                Some(statement)
            }
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

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }
    }
    fn parse_prefix_expression(&mut self) -> Expression {
        match &self.current_token.kind {
            Kind::Ident => self.parse_identifier(),
            _ => unimplemented!(),
        }
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
    use crate::lexer::Lexer;
    use crate::parser::ast::{Expression, Node, Statement};

    use super::Parser;

    fn valid_let_statement(statement: &Statement, let_name: &str) -> bool {
        if statement.token_literal() != "let" {
            return false;
        }

        match statement {
            Statement::Let { name, .. } => {
                name.value == let_name && name.token_literal() == let_name
            }
            _ => false,
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

    #[test]
    fn return_statement() {
        // Arrange
        let input = r#"
            return 5; 
            return 10; 
            return 838383;
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
        for i in 0..tests.len() {
            let statement = &program.statements[i];
            assert_eq!(statement.token_literal(), "return");
        }
    }

    #[test]
    fn identifier_expression() {
        // Arrange
        let input = "foobar;".to_string();

        // Act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // Assert
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        let expression_statement = match &program.statements[0] {
            Statement::Expression {
                token: _,
                expression,
            } => expression,
            s => panic!("{s} is not an expression statement"),
        };

        match expression_statement {
            Expression::Identifier { token, value } => {
                assert_eq!(token.literal, "foobar");
                assert_eq!(value, "foobar");
            }
            e => panic!("{e} is not an identifier"),
        };
    }
}
