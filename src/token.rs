use std::fmt::{write, Display};

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
        match self {
            TT::Assign => write!(f, "TokenAssign"),
            TT::Bang => write!(f, "TokenBang"),
            TT::BangEqual => write!(f, "TokenNotEqual"),
            TT::Coma => write!(f, "TokenComa"),
            TT::Else => write!(f, "TokenElse"),
            TT::Eof => write!(f, "TokenEOF"),
            TT::Equal => write!(f, "TEqual"),
            TT::False => write!(f, "TFalse"),
            TT::Function => write!(f, "TFunction"),
            TT::Greater => write!(f, "TGreater"),
            TT::GreaterEqual => write!(f, "TGreaterEqual"),
            TT::Identifier => write!(f, "TIdentifier"),
            TT::If => write!(f, "TIf"),
            TT::IllegalToken => write!(f, "TIllegalToken"),
            TT::LBrace => write!(f, "TLBrace"),
            TT::LParen => write!(f, "TLParen"),
            TT::Less => write!(f, "TLess"),
            TT::LessEqual => write!(f, "TLessEqual"),
            TT::Let => write!(f, "TLet"),
            TT::Literal => write!(f, "TLiteral"),
            TT::Minus => write!(f, "TMinus"),
            _ => todo!(),
        }
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
