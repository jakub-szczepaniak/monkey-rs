use std::fmt::Display;

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
    Equal,
    BangEqual,
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
    LessEqual,
    Greater,
    GreaterEqual,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hell")
    }
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

    pub fn is_illegal(&self) -> bool {
        self.ttype == TT::IllegalToken
    }

    pub fn is_eof(&self) -> bool {
        self.ttype == TT::Eof
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.ttype, self.lexeme)
    }
}
