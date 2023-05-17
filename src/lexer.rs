use crate::token::{self, Token};

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
        let tok = match self.ch {
            '=' => Token::new(token::Kind::Assign, self.ch.to_string()),
            '+' => Token::new(token::Kind::Plus, self.ch.to_string()),
            ';' => Token::new(token::Kind::SemiColon, self.ch.to_string()),
            ',' => Token::new(token::Kind::Comma, self.ch.to_string()),
            '(' => Token::new(token::Kind::LParen, self.ch.to_string()),
            ')' => Token::new(token::Kind::RParen, self.ch.to_string()),
            '{' => Token::new(token::Kind::LBrace, self.ch.to_string()),
            '}' => Token::new(token::Kind::RBrace, self.ch.to_string()),
            '\0' => Token::new(token::Kind::Eof, "".to_string()),
            _ => Token::new(token::Kind::Illegal, "".to_string()),
        };

        self.read_char();

        return tok;
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token};

    #[test]
    fn next_token() {
        // Arrange
        let input = "=+(){},;".to_string();
        let cases = vec![
            (token::Kind::Assign, "="),
            (token::Kind::Plus, "+"),
            (token::Kind::LParen, "("),
            (token::Kind::RParen, ")"),
            (token::Kind::LBrace, "{"),
            (token::Kind::RBrace, "}"),
            (token::Kind::Comma, ","),
            (token::Kind::SemiColon, ";"),
            (token::Kind::Eof, ""),
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
