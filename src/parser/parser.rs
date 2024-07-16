use crate::lexer::Lexer;
use crate::token::{Kind, Token};

use super::ast::{BlockStatement, Expression, Identifer, Program, Statement};

const LOWEST: i32 = 0;
const EQUALITY: i32 = 1;
const LESS_GREATER: i32 = 2;
const SUM: i32 = 3;
const PRODUCT: i32 = 4;
const PREFIX: i32 = 5;
const CALL: i32 = 6;

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

    fn parse_statement(&mut self) -> Option<Statement> {
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
                if let Some(expression) = self.parse_expression(LOWEST) {
                    let statement = Statement::Expression {
                        token: self.current_token.clone(),
                        expression,
                    };

                    if self.peek_token.kind == Kind::SemiColon {
                        self.next_token();
                    }

                    Some(statement)
                } else {
                    None
                }
            }
        }
    }

    fn parse_expression(&mut self, precendence: i32) -> Option<Expression> {
        let mut left = match &self.current_token.kind {
            Kind::Ident => self.parse_identifier(),
            Kind::Int => self.parse_integer_literal(),
            Kind::True | Kind::False => self.parse_boolean_literal(),
            Kind::Bang | Kind::Minus => self.parse_prefix(),
            Kind::LParen => self.parse_grouped_expression(),
            Kind::If => self.parse_if_expression(),
            Kind::Function => self.parse_function_literal(),
            _ => None,
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
                    self.parse_infix(left.unwrap())
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_identifier(&self) -> Option<Expression> {
        Some(Expression::Identifier(self.current_token.literal.clone()))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        if let Ok(value) = self.current_token.literal.parse::<i64>() {
            Some(Expression::IntegerLiteral(value))
        } else {
            let err = format!("could not parse {} as integer", self.current_token.literal);
            self.errors.push(err);
            return None;
        }
    }

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        Some(Expression::BooleanLiteral(
            self.current_token.kind == Kind::True,
        ))
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(PREFIX).unwrap());
        Some(Expression::Prefix { operator, right })
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        let left = Box::new(left);
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).unwrap();

        let expression = Expression::Infix {
            left,
            operator,
            right: Box::new(right),
        };

        Some(expression)
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(LOWEST);

        if !self.expect_peek(Kind::RParen) {
            None
        } else {
            expression
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Kind::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(LOWEST).unwrap();

        if !self.expect_peek(Kind::RParen) || !self.expect_peek(Kind::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        if self.peek_token.kind == Kind::Else {
            self.next_token();

            if !self.expect_peek(Kind::LBrace) {
                return None;
            }

            let alternative = self.parse_block_statement();
            return Some(Expression::If {
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Some(Box::new(alternative)),
            });
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: None,
        })
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(Kind::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(Kind::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FunctionLiteral {
            parameters: parameters.unwrap(),
            body,
        })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = vec![];
        self.next_token();

        while self.current_token.kind != Kind::RBrace && self.current_token.kind != Kind::Eof {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                statements.push(statement);
            }
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

    fn peek_precedence(&self) -> i32 {
        match self.peek_token.kind {
            Kind::Eq | Kind::Ne => EQUALITY,
            Kind::Lt | Kind::Gt => LESS_GREATER,
            Kind::Plus | Kind::Minus => SUM,
            Kind::Asterix | Kind::Slash => PRODUCT,
            _ => LOWEST,
        }
    }

    fn current_precedence(&self) -> i32 {
        match self.current_token.kind {
            Kind::Eq | Kind::Ne => EQUALITY,
            Kind::Lt | Kind::Gt => LESS_GREATER,
            Kind::Plus | Kind::Minus => SUM,
            Kind::Asterix | Kind::Slash => PRODUCT,
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
            return true;
        } else {
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
}
