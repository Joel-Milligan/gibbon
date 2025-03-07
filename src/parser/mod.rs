mod ast;

use crate::lexer::Lexer;
use crate::token::{Kind, Token};

use ast::{BlockStatement, Expression, Identifer, Program, Statement};

const LOWEST: i32 = 0;
const EQUALITY: i32 = 1;
const LESS_GREATER: i32 = 2;
const SUM: i32 = 3;
const PRODUCT: i32 = 4;
const PREFIX: i32 = 5;
const CALL: i32 = 6;

pub struct Parser {
    lexer: Lexer,
    pub errors: Vec<String>,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token.kind != Kind::Eof {
            program.statements.push(self.parse_statement());
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Statement {
        match &self.current_token.kind {
            Kind::Let => {
                if !self.expect_peek(Kind::Ident) {
                    panic!("let must be follow by an identifier")
                }

                let name = Identifer {
                    token: self.current_token.clone(),
                    value: self.current_token.literal.clone(),
                };

                if !self.expect_peek(Kind::Assign) {
                    panic!("let identifier must be follow by an assign token")
                }

                self.next_token();

                let value = self.parse_expression(LOWEST);

                if self.peek_token.kind == Kind::SemiColon {
                    self.next_token();
                }

                Statement::Let { name, value }
            }
            Kind::Return => {
                self.next_token();

                let value = self.parse_expression(LOWEST);

                if self.peek_token.kind == Kind::SemiColon {
                    self.next_token();
                }

                Statement::Return(value)
            }
            _ => {
                let statement = Statement::Expression {
                    token: self.current_token.clone(),
                    expression: self.parse_expression(LOWEST),
                };

                if self.peek_token.kind == Kind::SemiColon {
                    self.next_token();
                }

                statement
            }
        }
    }

    fn parse_expression(&mut self, precendence: i32) -> Expression {
        let mut left = match &self.current_token.kind {
            Kind::Ident => self.parse_identifier(),
            Kind::Int => self.parse_integer_literal(),
            Kind::True | Kind::False => self.parse_boolean_literal(),
            Kind::Bang | Kind::Minus => self.parse_prefix(),
            Kind::LParen => self.parse_grouped_expression(),
            Kind::If => self.parse_if_expression(),
            Kind::Function => self.parse_function_literal(),
            token => panic!("can't parse expression that starts with {token:?}"),
        };

        while self.peek_token.kind != Kind::SemiColon && precendence < self.peek_precedence() {
            left = match &self.peek_token.kind {
                Kind::Plus
                | Kind::Minus
                | Kind::Asterix
                | Kind::Slash
                | Kind::Eq
                | Kind::Ne
                | Kind::Gt
                | Kind::Lt => {
                    self.next_token();
                    self.parse_infix(left)
                }
                Kind::LParen => {
                    self.next_token();
                    self.parse_call_expression(left)
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(self.current_token.literal.clone())
    }

    fn parse_integer_literal(&mut self) -> Expression {
        Expression::IntegerLiteral(
            self.current_token
                .literal
                .parse::<i64>()
                .expect("could not parse integer literal"),
        )
    }

    fn parse_boolean_literal(&mut self) -> Expression {
        Expression::BooleanLiteral(self.current_token.kind == Kind::True)
    }

    fn parse_prefix(&mut self) -> Expression {
        let operator = self.current_token.literal.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(PREFIX));
        Expression::Prefix { operator, right }
    }

    fn parse_infix(&mut self, left: Expression) -> Expression {
        let left = Box::new(left);
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();
        let right = Box::new(self.parse_expression(precedence));

        Expression::Infix {
            left,
            operator,
            right,
        }
    }

    fn parse_grouped_expression(&mut self) -> Expression {
        self.next_token();

        let expression = self.parse_expression(LOWEST);

        if !self.expect_peek(Kind::RParen) {
            panic!("expected right paren")
        } else {
            expression
        }
    }

    fn parse_if_expression(&mut self) -> Expression {
        if !self.expect_peek(Kind::LParen) {
            panic!("expected left paren")
        }

        self.next_token();
        let condition = self.parse_expression(LOWEST);

        if !self.expect_peek(Kind::RParen) || !self.expect_peek(Kind::LBrace) {
            panic!("expected right paren or left brace")
        }

        let consequence = self.parse_block_statement();

        if self.peek_token.kind == Kind::Else {
            self.next_token();

            if !self.expect_peek(Kind::LBrace) {
                panic!("expected left brace")
            }

            let alternative = self.parse_block_statement();
            return Expression::If {
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Some(Box::new(alternative)),
            };
        }

        Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: None,
        }
    }

    fn parse_function_literal(&mut self) -> Expression {
        if !self.expect_peek(Kind::LParen) {
            panic!("Expected left paren")
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(Kind::LBrace) {
            panic!("Expected left brace")
        }

        let body = self.parse_block_statement();

        Expression::FunctionLiteral {
            parameters: parameters.unwrap(),
            body,
        }
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = vec![];
        self.next_token();

        while self.current_token.kind != Kind::RBrace && self.current_token.kind != Kind::Eof {
            let statement = self.parse_statement();
            statements.push(statement);
            self.next_token();
        }

        BlockStatement { statements }
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifer>> {
        let mut identifiers = vec![];

        if self.peek_token.kind == Kind::RParen {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let identifier = Identifer {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        identifiers.push(identifier);

        while self.peek_token.kind == Kind::Comma {
            self.next_token();
            self.next_token();
            let identifier = Identifer {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };
            identifiers.push(identifier);
        }

        if !self.expect_peek(Kind::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Expression {
        Expression::Call {
            function: Box::new(function),
            arguments: self.parse_call_arguments().unwrap(),
        }
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args = vec![];

        if self.peek_token.kind == Kind::RParen {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        args.push(self.parse_expression(LOWEST));

        while self.peek_token.kind == Kind::Comma {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(LOWEST));
        }

        if !self.expect_peek(Kind::RParen) {
            return None;
        }

        Some(args)
    }

    fn peek_precedence(&self) -> i32 {
        match self.peek_token.kind {
            Kind::Eq | Kind::Ne => EQUALITY,
            Kind::Lt | Kind::Gt => LESS_GREATER,
            Kind::Plus | Kind::Minus => SUM,
            Kind::Asterix | Kind::Slash => PRODUCT,
            Kind::LParen => CALL,
            _ => LOWEST,
        }
    }

    fn current_precedence(&self) -> i32 {
        match self.current_token.kind {
            Kind::Eq | Kind::Ne => EQUALITY,
            Kind::Lt | Kind::Gt => LESS_GREATER,
            Kind::Plus | Kind::Minus => SUM,
            Kind::Asterix | Kind::Slash => PRODUCT,
            Kind::LParen => CALL,
            _ => LOWEST,
        }
    }

    fn peek_error(&mut self, token_kind: Kind) {
        self.errors.push(format!(
            "expected next token to be {token_kind:?}, got {:?} instead",
            self.peek_token.kind
        ));
    }

    fn expect_peek(&mut self, expected_token: Kind) -> bool {
        if expected_token == self.peek_token.kind {
            self.next_token();
            true
        } else {
            self.peek_error(expected_token);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::ast::{Expression, Node, Statement};

    use super::Parser;

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
        let input = "let x = 5;".to_string();

        // Act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // Assert
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Let { name, value } => {
                assert_eq!(name.token_literal(), "x".to_string());
                match value {
                    Expression::IntegerLiteral(value) => {
                        assert_eq!(*value, 5);
                    }
                    e => panic!("{e} is not an integer"),
                };
            }
            s => panic!("{s} is not a let statement"),
        };
    }

    #[test]
    fn return_statement() {
        // Arrange
        let input = "return 5;".to_string();

        // Act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // Assert
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Return(value) => {
                match value {
                    Expression::IntegerLiteral(value) => {
                        assert_eq!(*value, 5);
                    }
                    e => panic!("{e} is not an integer"),
                };
            }
            s => panic!("{s} is not a return statement"),
        };
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
            Expression::Identifier(value) => {
                assert_eq!(expression_statement.token_literal(), "foobar");
                assert_eq!(value, "foobar");
            }
            e => panic!("{e} is not an identifier"),
        };
    }

    #[test]
    fn integer_literal_expression() {
        // Arrange
        let input = "5;".to_string();

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
            Expression::IntegerLiteral(value) => {
                assert_eq!(expression_statement.token_literal(), "5");
                assert_eq!(*value, 5);
            }
            e => panic!("{e} is not an identifier"),
        };
    }

    #[test]
    fn boolean_literal_expression() {
        // Arrange
        let input = "true;".to_string();

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
            Expression::BooleanLiteral(value) => {
                assert_eq!(expression_statement.token_literal(), "true");
                assert_eq!(*value, true);
            }
            e => panic!("{e} is not an identifier"),
        };
    }

    #[test]
    fn prefix_expression() {
        // Arrange
        let prefix_tests = vec![("!5;", "!", 5i64), ("-15;", "-", 15i64)];

        // Act
        for (input, op, integer_value) in prefix_tests {
            let lexer = Lexer::new(input.to_string());
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
                Expression::Prefix { operator, right } => {
                    assert_eq!(operator, op);
                    match &**right {
                        Expression::IntegerLiteral(i) => assert_eq!(*i, integer_value),
                        e => panic!("{e} is not an integer literal"),
                    }
                }
                e => panic!("{e} is not a prefix expression"),
            };
        }
    }

    #[test]
    fn integer_infix_expression() {
        // Arrange
        let infix_tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        // Act
        for (input, left_value, op, right_value) in infix_tests {
            let lexer = Lexer::new(input.to_string());
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
                Expression::Infix {
                    left,
                    operator,
                    right,
                } => {
                    match &**right {
                        Expression::IntegerLiteral(i) => assert_eq!(*i, right_value),
                        e => panic!("{e} is not an integer literal"),
                    }
                    assert_eq!(operator, op);
                    match &**left {
                        Expression::IntegerLiteral(i) => assert_eq!(*i, left_value),
                        e => panic!("{e} is not an integer literal"),
                    }
                }
                e => panic!("{e} is not an infix expression"),
            };
        }
    }

    #[test]
    fn boolean_infix_expression() {
        // Arrange
        let infix_tests = vec![
            ("true == true;", true, "==", true),
            ("true != false;", true, "!=", false),
            ("false == false;", false, "==", false),
        ];

        // Act
        for (input, left_value, op, right_value) in infix_tests {
            let lexer = Lexer::new(input.to_string());
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
                Expression::Infix {
                    left,
                    operator,
                    right,
                } => {
                    match &**right {
                        Expression::BooleanLiteral(i) => assert_eq!(*i, right_value),
                        e => panic!("{e} is not a boolean literal"),
                    }
                    assert_eq!(operator, op);
                    match &**left {
                        Expression::BooleanLiteral(i) => assert_eq!(*i, left_value),
                        e => panic!("{e} is not a boolean literal"),
                    }
                }
                e => panic!("{e} is not an infix expression"),
            };
        }
    }

    #[test]
    fn complex_infix_expressions() {
        // Arrange
        let infix_tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];

        // Act
        for (input, expected) in infix_tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            println!("{:#?}", program.statements[0]);

            // Assert
            check_parser_errors(&parser);
            assert_eq!(program.to_string(), expected);
        }
    }

    #[test]
    fn boolean_operator_precedence() {
        // Arrange
        let tests = vec![
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
        ];

        // Act
        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            println!("{:#?}", program.statements[0]);

            // Assert
            check_parser_errors(&parser);
            assert_eq!(program.to_string(), expected);
        }
    }

    #[test]
    fn if_expression() {
        // Arrange
        let input = "if (x < y) { x };".to_string();

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
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                assert_eq!(condition.to_string(), "(x < y)");
                assert_eq!(consequence.to_string(), "x");
                assert_eq!(*alternative, None);
            }
            e => panic!("{e} is not an identifier"),
        };
    }

    #[test]
    fn if_else_expression() {
        // Arrange
        let input = "if (x < y) { x } else { y };".to_string();

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
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                assert_eq!(condition.to_string(), "(x < y)");
                assert_eq!(consequence.to_string(), "x");
                assert_eq!(alternative.as_ref().unwrap().to_string(), "y");
            }
            e => panic!("{e} is not an identifier"),
        };
    }

    #[test]
    fn function_literal_expression() {
        // Arrange
        let input = "fn(x, y) { x + y; }".to_string();

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
            Expression::FunctionLiteral { parameters, body } => {
                assert_eq!(parameters[0].token_literal(), "x");
                assert_eq!(parameters[1].token_literal(), "y");
                assert_eq!(body.statements.len(), 1);
                match &body.statements[0] {
                    Statement::Expression {
                        token: _,
                        expression,
                    } => {
                        assert_eq!(expression.to_string(), "(x + y)")
                    }
                    s => panic!("{s} is not an expression statement"),
                }
            }
            e => panic!("{e} is not a function literal"),
        };
    }

    #[test]
    fn function_parameters() {
        // Arrange
        let tests = vec![
            ("fn() {};".to_string(), vec![]),
            ("fn(x) {};".to_string(), vec!["x"]),
            ("fn(x, y, z) {};".to_string(), vec!["x", "y", "z"]),
        ];

        for (input, expected) in tests {
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
                Expression::FunctionLiteral {
                    parameters,
                    body: _,
                } => {
                    assert_eq!(parameters.len(), expected.len());
                    for (i, parameter) in parameters.iter().enumerate() {
                        assert_eq!(parameter.token_literal(), expected[i]);
                    }
                }
                e => panic!("{e} is not a function literal"),
            };
        }
    }

    #[test]
    fn call_expression() {
        // Arrange
        let input = "add(1, 2 * 3, 4 + 5);".to_string();

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
            Expression::Call {
                function,
                arguments,
            } => {
                match &**function {
                    Expression::Identifier(value) => {
                        assert_eq!(*value, "add".to_string());
                    }
                    e => panic!("{e} is not an identifier"),
                }
                assert_eq!(arguments.len(), 3);
                assert_eq!(arguments[0].to_string(), "1".to_string());
                assert_eq!(arguments[1].to_string(), "(2 * 3)".to_string());
                assert_eq!(arguments[2].to_string(), "(4 + 5)".to_string());
            }
            e => panic!("{e} is not a function call"),
        };
    }
}
