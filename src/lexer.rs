use std::collections;

use crate::token::*;

pub struct Lexer {
    input: Vec<char>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if !self.is_at_end() {
            self.skip_whitespace();
            let ch = self.advance();
            match ch {
                '+' => return Token::new(TT::Plus, format!("{}", ch)),
                '=' => {
                    let _ = match self.peek() {
                        Some(v) => {
                            if v == '=' {
                                self.advance();
                                return Token::new(TT::Equal, "==".to_string());
                            } else {
                                return Token::new(TT::Assign, "=".to_string());
                            }
                        }
                        None => return Token::new(TT::Assign, "=".to_string()),
                    };
                }
                '(' => return Token::new(TT::LParen, format!("{}", ch)),
                ')' => return Token::new(TT::RParen, format!("{}", ch)),
                '{' => return Token::new(TT::LBrace, format!("{}", ch)),
                '}' => return Token::new(TT::RBrace, format!("{}", ch)),
                ',' => return Token::new(TT::Coma, format!("{}", ch)),
                '!' => {
                    let next = self.peek().unwrap();
                    if next == '=' {
                        self.advance();
                        return Token::new(TT::BangEqual, "!=".to_string());
                    } else {
                        return Token::new(TT::Bang, format!("{}", ch));
                    }
                }
                '-' => return Token::new(TT::Minus, format!("{}", ch)),
                '/' => return Token::new(TT::Slash, format!("{}", ch)),
                '*' => return Token::new(TT::Star, format!("{}", ch)),
                '<' => {
                    let next = self.peek().unwrap();
                    if next == '=' {
                        self.advance();
                        return Token::new(TT::LessEqual, "<=".to_string());
                    } else {
                        return Token::new(TT::Less, "<".to_string());
                    }
                }
                '>' => {
                    let next = self.peek().unwrap();
                    if next == '=' {
                        self.advance();
                        return Token::new(TT::GreaterEqual, ">=".to_string());
                    } else {
                        return Token::new(TT::Greater, format!("{}", ch));
                    }
                }
                ';' => return Token::new(TT::Semicolon, format!("{}", ch)),
                _ => {
                    if self.is_letter(Some(ch)) {
                        let identifier = self.read_identifier();
                        match identifier.as_str() {
                            "fn" => return Token::new(TT::Function, identifier),
                            "let" => return Token::new(TT::Let, identifier),
                            "if" => return Token::new(TT::If, identifier),
                            "else" => return Token::new(TT::Else, identifier),
                            "return" => return Token::new(TT::Return, identifier),
                            "true" => return Token::new(TT::True, identifier),
                            "false" => return Token::new(TT::False, identifier),
                            _ => return Token::new(TT::Identifier, identifier),
                        }
                    } else if self.is_digit(Some(ch)) {
                        let literal = self.read_number();
                        return Token::new(TT::Literal, literal);
                    } else {
                        return Token::new(TT::IllegalToken, format!("{}", ch));
                    }
                }
            }
        }
        Token::eof()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn is_letter(&self, ch: Option<char>) -> bool {
        match ch {
            Some(ch) => ch.is_ascii_alphabetic() || ch == '_',
            None => false,
        }
    }

    fn is_digit(&self, ch: Option<char>) -> bool {
        match ch {
            Some(ch) => ch.is_digit(10),
            None => false,
        }
    }

    fn is_whitespace(&self, ch: Option<char>) -> bool {
        match ch {
            Some(c) => c.is_whitespace(),
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace(self.peek()) {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.current - 1;
        while self.is_letter(self.peek()) {
            self.advance();
        }
        self.input[start..self.current]
            .iter()
            .collect::<Vec<&char>>()
            .into_iter()
            .collect()
    }

    fn read_number(&mut self) -> String {
        let start = self.current - 1;
        while self.is_digit(self.peek()) {
            self.advance();
        }
        self.input[start..self.current]
            .iter()
            .collect::<Vec<&char>>()
            .into_iter()
            .collect()
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
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
        let expected = vec![
            Token::from_str(TT::Assign, "="),
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

    #[rstest]
    fn test_simple_program() {
        let input: &str = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x+y;
}
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5<10) {
    return true;
    } else {
    return false;
}
10==10;
10!=9;
5>=4;
5<=6;"#;
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::from_str(TT::Let, "let"),
            Token::from_str(TT::Identifier, "five"),
            Token::from_str(TT::Assign, "="),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Let, "let"),
            Token::from_str(TT::Identifier, "ten"),
            Token::from_str(TT::Assign, "="),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Let, "let"),
            Token::from_str(TT::Identifier, "add"),
            Token::from_str(TT::Assign, "="),
            Token::from_str(TT::Function, "fn"),
            Token::from_str(TT::LParen, "("),
            Token::from_str(TT::Identifier, "x"),
            Token::from_str(TT::Coma, ","),
            Token::from_str(TT::Identifier, "y"),
            Token::from_str(TT::RParen, ")"),
            Token::from_str(TT::LBrace, "{"),
            Token::from_str(TT::Identifier, "x"),
            Token::from_str(TT::Plus, "+"),
            Token::from_str(TT::Identifier, "y"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::RBrace, "}"),
            Token::from_str(TT::Let, "let"),
            Token::from_str(TT::Identifier, "result"),
            Token::from_str(TT::Assign, "="),
            Token::from_str(TT::Identifier, "add"),
            Token::from_str(TT::LParen, "("),
            Token::from_str(TT::Identifier, "five"),
            Token::from_str(TT::Coma, ","),
            Token::from_str(TT::Identifier, "ten"),
            Token::from_str(TT::RParen, ")"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Bang, "!"),
            Token::from_str(TT::Minus, "-"),
            Token::from_str(TT::Slash, "/"),
            Token::from_str(TT::Star, "*"),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::Less, "<"),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::Greater, ">"),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::If, "if"),
            Token::from_str(TT::LParen, "("),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::Less, "<"),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::RParen, ")"),
            Token::from_str(TT::LBrace, "{"),
            Token::from_str(TT::Return, "return"),
            Token::from_str(TT::True, "true"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::RBrace, "}"),
            Token::from_str(TT::Else, "else"),
            Token::from_str(TT::LBrace, "{"),
            Token::from_str(TT::Return, "return"),
            Token::from_str(TT::False, "false"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::RBrace, "}"),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::Equal, "=="),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Literal, "10"),
            Token::from_str(TT::BangEqual, "!="),
            Token::from_str(TT::Literal, "9"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::GreaterEqual, ">="),
            Token::from_str(TT::Literal, "4"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Literal, "5"),
            Token::from_str(TT::LessEqual, "<="),
            Token::from_str(TT::Literal, "6"),
            Token::from_str(TT::Semicolon, ";"),
            Token::from_str(TT::Eof, ""),
        ];

        for exp in &expected {
            let token = lexer.next_token();
            assert_eq!(token, *exp)
        }
    }
}
