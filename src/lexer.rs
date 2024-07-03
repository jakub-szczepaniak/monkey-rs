use crate::token::*;

pub struct Lexer {
    input: Vec<char>,
    start: usize,
    current: usize,


}

impl Lexer {
    pub fn new(input:&str) -> Self {
        Self { 
            input: input.chars().collect(),
            start: 0,
            current: 0 
        }
    }

    pub fn next_token(&mut self) -> Token {
        if !self.is_at_end() {
           let ch = self.advance();
           match ch {
            '+' => return Token::new(TT::Plus, format!("{}", ch)),
            '=' => return Token::new(TT::Assign, format!("{}", ch)),
            '(' => return Token::new(TT::LParen, format!("{}", ch)),
            ')' => return Token::new(TT::RParen, format!("{}", ch)),
            '{' => return Token::new(TT::LBrace, format!("{}", ch)),
            '}' => return Token::new(TT::RBrace, format!("{}", ch)),
            ',' => return Token::new(TT::Coma, format!("{}", ch)),
            ';' => return Token::new(TT::Semicolon, format!("{}", ch)),
            _ => return Token::new(TT::Plus, format!("{}", ch)),

           }
        }
       Token::eof()
    }

    fn is_at_end(&self) -> bool { 
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        let result = *self.input.get(self.current).unwrap();
        self.current += 1;
        result
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    #[case("+", Token::new(TT::Plus, "+".to_string()))]
    #[case("=", Token::new(TT::Assign, "=".to_string()))]
    #[case("(", Token::new(TT::LParen, "(".to_string()))]
    #[case(")", Token::new(TT::RParen, ")".to_string()))]
    #[case("{", Token::new(TT::LBrace, "{".to_string()))]
    #[case("}", Token::new(TT::RBrace, "}".to_string()))]
    #[case(",", Token::new(TT::Coma, ",".to_string()))]
    #[case(";", Token::new(TT::Semicolon, ";".to_string()))]
    fn test_next_token(#[case] input: &str, #[case] expected: Token) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), expected);
        assert_eq!(lexer.next_token(), Token::eof())
    }

    #[rstest]
    fn test_multiple_tokens() {
        let mut lexer = Lexer::new("=+(){},;");
        let expected = vec![Token::from_str(TT::Assign,"="),
        Token::from_str(TT::Plus, "+"),
        Token::from_str(TT::LParen, "("),
        Token::from_str(TT::RParen, ")"),
        Token::from_str(TT::LBrace, "{"),
        Token::from_str(TT::RBrace, "}"),
        Token::from_str(TT::Coma, ","),
        Token::from_str(TT::Semicolon, ";"),
        Token::from_str(TT::Eof, ""),
        ];

        for exp in &expected { 
            let token = lexer.next_token();
            assert_eq!(token, *exp)
        }
    }
}