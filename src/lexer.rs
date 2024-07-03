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
        while !self.is_at_end() {
           let ch = self.advance();
           match ch {
            '+' => return Token::new(TokenType::Plus, format!("{}", ch)),
            _ => return Token::new(TokenType::Plus, format!("{}", ch)),

           }
        }
       return Token::eof();
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
    #[case("+", Token::new(TokenType::Plus, "+".to_string()))]
    fn test_next_token(#[case] input: &str, #[case] expected: Token) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), expected);
        assert_eq!(lexer.next_token(), Token::eof())
    }
}