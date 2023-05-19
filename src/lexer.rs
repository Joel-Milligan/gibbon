use crate::token::{lookup_ident, Kind, Token};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new(Kind::Assign, self.ch.to_string()),
            '+' => Token::new(Kind::Plus, self.ch.to_string()),
            ';' => Token::new(Kind::SemiColon, self.ch.to_string()),
            ',' => Token::new(Kind::Comma, self.ch.to_string()),
            '(' => Token::new(Kind::LParen, self.ch.to_string()),
            ')' => Token::new(Kind::RParen, self.ch.to_string()),
            '{' => Token::new(Kind::LBrace, self.ch.to_string()),
            '}' => Token::new(Kind::RBrace, self.ch.to_string()),
            '\0' => Token::new(Kind::Eof, "".to_string()),
            c => {
                if identifier_character(&c) {
                    let literal = self.read_identifier();
                    let ident = lookup_ident(&literal);
                    return Token::new(ident, literal);
                } else if c.is_ascii_digit() {
                    return Token::new(Kind::Int, self.read_number());
                } else {
                    return Token::new(Kind::Illegal, "".to_string());
                }
            }
        };

        self.read_char();

        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while identifier_character(&self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
}

fn identifier_character(c: &char) -> bool {
    c.is_ascii_alphabetic() || c == &'_'
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Kind;

    #[test]
    fn single_chars() {
        // Arrange
        let input = "=;+(){},".to_string();
        let cases = vec![
            (Kind::Assign, "="),
            (Kind::SemiColon, ";"),
            (Kind::Plus, "+"),
            (Kind::LParen, "("),
            (Kind::RParen, ")"),
            (Kind::LBrace, "{"),
            (Kind::RBrace, "}"),
            (Kind::Comma, ","),
            (Kind::Eof, ""),
        ];

        // Act
        let mut lexer = Lexer::new(input);

        // Assert
        for case in cases {
            let token = lexer.next();
            assert_eq!(token.kind, case.0);
            assert_eq!(token.literal, case.1);
        }
    }

    #[test]
    fn simple_code_snippet() {
        // Arrange
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);"#
            .to_string();

        let cases = vec![
            (Kind::Let, "let"),
            (Kind::Ident, "five"),
            (Kind::Assign, "="),
            (Kind::Int, "5"),
            (Kind::SemiColon, ";"),
            (Kind::Let, "let"),
            (Kind::Ident, "ten"),
            (Kind::Assign, "="),
            (Kind::Int, "10"),
            (Kind::SemiColon, ";"),
            (Kind::Let, "let"),
            (Kind::Ident, "add"),
            (Kind::Assign, "="),
            (Kind::Function, "fn"),
            (Kind::LParen, "("),
            (Kind::Ident, "x"),
            (Kind::Comma, ","),
            (Kind::Ident, "y"),
            (Kind::RParen, ")"),
            (Kind::LBrace, "{"),
            (Kind::Ident, "x"),
            (Kind::Plus, "+"),
            (Kind::Ident, "y"),
            (Kind::SemiColon, ";"),
            (Kind::RBrace, "}"),
            (Kind::SemiColon, ";"),
            (Kind::Let, "let"),
            (Kind::Ident, "result"),
            (Kind::Assign, "="),
            (Kind::Ident, "add"),
            (Kind::LParen, "("),
            (Kind::Ident, "five"),
            (Kind::Comma, ","),
            (Kind::Ident, "ten"),
            (Kind::RParen, ")"),
            (Kind::SemiColon, ";"),
            (Kind::Eof, ""),
        ];

        // Act
        let mut lexer = Lexer::new(input);

        // Assert
        for case in cases {
            let token = lexer.next();
            assert_eq!(token.kind, case.0);
            assert_eq!(token.literal, case.1);
        }
    }
}
