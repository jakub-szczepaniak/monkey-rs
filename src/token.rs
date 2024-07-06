#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
}

pub type TT = TokenType;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Plus,
    Assign,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Coma,
    Semicolon,
    Identifier,
    Let,
    Literal,
    Function,
    Less,
    Greater,
    Slash,
    Star,
    Minus,
    Bang,
    True,
    False,
    Return,
    If,
    Else,
    Eof,
    IllegalToken,
}

impl Token {
    pub fn new(ttype: TT, lexeme: String) -> Self {
        Self { ttype, lexeme }
    }

    pub fn eof() -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
        }
    }
    pub fn from_str(ttype: TT, lexeme: &str) -> Self {
        Self::new(ttype, lexeme.to_string())
    }
}
