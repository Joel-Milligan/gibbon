use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq)]
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

static KEYWORDS: phf::Map<&'static str, Kind> = phf_map! {
    "fn" => Kind::Function,
    "let" => Kind::Let,
};

pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Kind, literal: String) -> Token {
        Token { kind, literal }
    }
}

pub fn lookup_ident(ident: &str) -> Kind {
    *KEYWORDS.get(ident).unwrap_or(&Kind::Ident)
}
