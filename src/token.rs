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
    EOF
}

impl Token {
    pub fn new(ttype: TT, lexeme: String) -> Self {
        Self {
            ttype, 
            lexeme
        }
    }

    pub fn eof() -> Self {
        Self {
            ttype: TokenType::EOF,
            lexeme: "".to_string()
        }
    }
}

