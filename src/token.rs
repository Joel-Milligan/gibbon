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
    Minus,
    Asterix,
    Slash,

    Bang,
    Lt,
    Gt,
    Eq,
    Ne,

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
    True,
    False,
    If,
    Else,
    Return,
}

static KEYWORDS: phf::Map<&'static str, Kind> = phf_map! {
    "fn" => Kind::Function,
    "let" => Kind::Let,
    "true" => Kind::True,
    "false" => Kind::False,
    "if" => Kind::If,
    "else" => Kind::Else,
    "return" => Kind::Return,
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
