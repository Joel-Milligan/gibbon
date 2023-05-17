#[derive(Debug, PartialEq)]
pub enum Kind {
    Illegal,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Kind, literal: String) -> Token {
        Token { kind, literal }
    }
}
