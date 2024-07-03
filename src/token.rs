#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Plus,
    EOF
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String) -> Self {
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

