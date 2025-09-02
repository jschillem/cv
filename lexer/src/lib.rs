//! The lexer for the CV programming language.
//!
//! This module provides functionality to tokenize CV source code into a series of tokens.

#![allow(dead_code)]

use crate::tokens::{NumberLiteral, Token, TokenKind};
use thiserror::Error;

pub mod tokens;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a str,
    position: usize,
    error_state: Option<LexerError>,
}

type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Error, PartialEq, Clone, Copy)]
enum LexerError {
    #[error("Unexpected character '{0}' at position {1}")]
    UnexpectedCharacter(char, usize),
    #[error("Invalid number format starting at position {0}")]
    InvalidNumberFormat(usize),
}

impl<'a> Lexer<'a> {
    /// Create a new lexer instance with the given input content.
    fn new(content: &'a str) -> Self {
        Self {
            content,
            position: 0,
            error_state: None,
        }
    }

    /// Peek at the current character without advancing the position.
    fn peek_char(&self, num_ahead: usize) -> Option<char> {
        self.content.chars().nth(self.position + num_ahead)
    }

    /// Advance the current position by the specified number of steps.
    fn advance(&mut self, steps: usize) {
        self.position += steps;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char(0) {
            if c.is_whitespace() && c != '\n' {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn create_simple_token(&mut self, kind: TokenKind, length: usize) -> Token {
        let start = self.position;
        self.advance(length);
        Token::new(kind, start, length)
    }

    fn create_optional_eq_token(
        &mut self,
        without_eq_kid: TokenKind,
        with_eq_kind: TokenKind,
    ) -> Token {
        let start = self.position;
        if self.peek_char(1) == Some('=') {
            self.advance(2);
            Token::new(with_eq_kind, start, 2)
        } else {
            self.advance(1);
            Token::new(without_eq_kid, start, 1)
        }
    }

    fn get_identifier_string(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.peek_char(0) {
            if c.is_alphanumeric() || c == '_' {
                self.advance(1);
            } else {
                break;
            }
        }

        String::from(&self.content[start..self.position])
    }

    fn create_number_token(&mut self) -> Result<Token> {
        let start = self.position;
        let mut has_decimal_point = false;

        while let Some(c) = self.peek_char(0) {
            if c.is_digit(10) {
                self.advance(1);
            } else if c == '.' {
                has_decimal_point = true;
                self.advance(1);
            } else {
                break;
            }
        }

        let length = self.position - start;
        let value = &self.content[start..self.position];

        if has_decimal_point {
            match value.parse::<f64>() {
                Ok(num) => Ok(Token::new(
                    TokenKind::Number(NumberLiteral::Float(num)),
                    start,
                    length,
                )),
                Err(_) => Err(LexerError::InvalidNumberFormat(start)),
            }
        } else {
            match value.parse::<i64>() {
                Ok(num) => Ok(Token::new(
                    TokenKind::Number(NumberLiteral::Integer(num)),
                    start,
                    length,
                )),
                Err(_) => Err(LexerError::InvalidNumberFormat(start)),
            }
        }
    }

    /// Get the next token from the input content. Returns `None` if the end of input is reached.
    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        if let Some(char) = self.peek_char(0) {
            match char {
                '@' => Ok(Some(self.create_simple_token(TokenKind::Mut, 1))),
                ';' => Ok(Some(self.create_simple_token(TokenKind::Semicolon, 1))),
                ',' => Ok(Some(self.create_simple_token(TokenKind::Comma, 1))),
                '&' => Ok(Some(self.create_simple_token(TokenKind::Ampersand, 1))),
                '|' => Ok(Some(self.create_simple_token(TokenKind::Pipe, 1))),
                '{' => Ok(Some(self.create_simple_token(TokenKind::LeftBrace, 1))),
                '}' => Ok(Some(self.create_simple_token(TokenKind::RightBrace, 1))),
                '[' => Ok(Some(self.create_simple_token(TokenKind::LeftBracket, 1))),
                ']' => Ok(Some(self.create_simple_token(TokenKind::RightBracket, 1))),
                '(' => Ok(Some(self.create_simple_token(TokenKind::LeftParen, 1))),
                ')' => Ok(Some(self.create_simple_token(TokenKind::RightParen, 1))),
                '\n' => Ok(Some(self.create_simple_token(TokenKind::Newline, 1))),
                '\'' => Ok(Some(self.create_simple_token(TokenKind::SingleQuote, 1))),
                '"' => Ok(Some(self.create_simple_token(TokenKind::DoubleQuote, 1))),
                '=' => Ok(Some(self.create_optional_eq_token(
                    TokenKind::Equal,
                    TokenKind::DoubleEqual,
                ))),
                '<' => Ok(Some(self.create_optional_eq_token(
                    TokenKind::LessThan,
                    TokenKind::LessEqual,
                ))),
                '>' => Ok(Some(self.create_optional_eq_token(
                    TokenKind::GreaterThan,
                    TokenKind::GreaterEqual,
                ))),
                '+' => Ok(Some(
                    self.create_optional_eq_token(TokenKind::Plus, TokenKind::PlusEqual),
                )),
                '%' => Ok(Some(self.create_optional_eq_token(
                    TokenKind::Modulo,
                    TokenKind::ModuloEqual,
                ))),
                '*' => Ok(Some(
                    self.create_optional_eq_token(TokenKind::Star, TokenKind::TimesEqual),
                )),
                ':' => match self.peek_char(1) {
                    Some(':') => Ok(Some(self.create_simple_token(TokenKind::Scope, 2))),
                    _ => Ok(Some(self.create_simple_token(TokenKind::Colon, 1))),
                },
                '!' => match self.peek_char(1) {
                    Some('=') => Ok(Some(self.create_simple_token(TokenKind::NotEqual, 2))),
                    _ => Err(LexerError::UnexpectedCharacter('!', self.position)),
                },
                '.' => match self.peek_char(1) {
                    Some('.') => match self.peek_char(2) {
                        Some('=') => {
                            Ok(Some(self.create_simple_token(TokenKind::RangeInclusive, 3)))
                        }
                        _ => Ok(Some(self.create_simple_token(TokenKind::Range, 2))),
                    },
                    _ => Ok(Some(self.create_simple_token(TokenKind::Dot, 1))),
                },
                '-' => match self.peek_char(1) {
                    Some('>') => Ok(Some(self.create_simple_token(TokenKind::RightArrow, 2))),
                    Some('=') => Ok(Some(self.create_simple_token(TokenKind::MinusEqual, 2))),
                    _ => Ok(Some(self.create_simple_token(TokenKind::Minus, 1))),
                },
                '/' => match self.peek_char(1) {
                    Some('/') => {
                        // Skip single-line comment
                        self.advance(2);
                        while let Some(c) = self.peek_char(0) {
                            if c == '\n' {
                                break;
                            }
                            self.advance(1);
                        }
                        self.next_token()
                    }
                    Some('*') => {
                        // Skip multi-line comment
                        self.advance(2);
                        while let Some(c) = self.peek_char(0) {
                            if c == '*' && self.peek_char(1) == Some('/') {
                                self.advance(2);
                                break;
                            }
                            self.advance(1);
                        }
                        self.next_token()
                    }
                    Some('=') => Ok(Some(self.create_simple_token(TokenKind::DivideEqual, 2))),
                    _ => Ok(Some(self.create_simple_token(TokenKind::Divide, 1))),
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.get_identifier_string();
                    let length = identifier.len();
                    let kind = match identifier.as_str() {
                        // Keywords
                        "break" => TokenKind::Break,
                        "else" => TokenKind::Else,
                        "end" => TokenKind::End,
                        "false" => TokenKind::False,
                        "fn" => TokenKind::Fun,
                        "for" => TokenKind::For,
                        "if" => TokenKind::If,
                        "in" => TokenKind::In,
                        "loop" => TokenKind::Loop,
                        "patch" => TokenKind::Patch,
                        "record" => TokenKind::Record,
                        "return" => TokenKind::Return,
                        "true" => TokenKind::True,
                        "union" => TokenKind::Union,
                        "when" => TokenKind::When,

                        // Operators
                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,

                        // Default to identifier
                        _ => TokenKind::Identifier(identifier),
                    };
                    let start = self.position - length;
                    Ok(Some(Token::new(kind, start, length)))
                }
                '0'..='9' => Ok(Some(self.create_number_token()?)),
                char => Err(LexerError::UnexpectedCharacter(char, self.position)),
            }
        } else {
            return Ok(None);
        }
    }

    pub fn error(&self) -> Option<&LexerError> {
        self.error_state.as_ref()
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        for token_result in self.by_ref() {
            tokens.push(token_result?);
        }

        Ok(tokens)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => {
                self.error_state = Some(e.clone());
                Some(Err(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn expect_token(lexer: &mut Lexer, expected_kind: TokenKind) {
        match lexer.next_token() {
            Ok(token) => {
                let token = token.expect("Expected a token but got None");
                assert_eq!(token.kind, expected_kind);
            }
            Err(e) => panic!("Lexer error: {}", e),
        }
    }

    fn expect_eof(lexer: &mut Lexer) {
        match lexer.next_token() {
            Ok(token) => {
                assert!(token.is_none(), "Expected EOF but got a token");
            }
            Err(e) => panic!("Lexer error: {}", e),
        }
    }

    #[test]
    fn test_keywords() {
        let input = "break else end false fn for if in loop patch record return true union when";
        let mut lexer = Lexer::new(input);

        expect_token(&mut lexer, TokenKind::Break);
        expect_token(&mut lexer, TokenKind::Else);
        expect_token(&mut lexer, TokenKind::End);
        expect_token(&mut lexer, TokenKind::False);
        expect_token(&mut lexer, TokenKind::Fun);
        expect_token(&mut lexer, TokenKind::For);
        expect_token(&mut lexer, TokenKind::If);
        expect_token(&mut lexer, TokenKind::In);
        expect_token(&mut lexer, TokenKind::Loop);
        expect_token(&mut lexer, TokenKind::Patch);
        expect_token(&mut lexer, TokenKind::Record);
        expect_token(&mut lexer, TokenKind::Return);
        expect_token(&mut lexer, TokenKind::True);
        expect_token(&mut lexer, TokenKind::Union);
        expect_token(&mut lexer, TokenKind::When);

        expect_eof(&mut lexer);
    }

    #[test]
    fn test_syntax_tokens() {
        let input = ": :: ; -> . .. ..= , & * | { } [ ] ( ) < > ' \" \n";
        let mut lexer = Lexer::new(input);

        expect_token(&mut lexer, TokenKind::Colon);
        expect_token(&mut lexer, TokenKind::Scope);
        expect_token(&mut lexer, TokenKind::Semicolon);
        expect_token(&mut lexer, TokenKind::RightArrow);
        expect_token(&mut lexer, TokenKind::Dot);
        expect_token(&mut lexer, TokenKind::Range);
        expect_token(&mut lexer, TokenKind::RangeInclusive);
        expect_token(&mut lexer, TokenKind::Comma);
        expect_token(&mut lexer, TokenKind::Ampersand);
        expect_token(&mut lexer, TokenKind::Star);
        expect_token(&mut lexer, TokenKind::Pipe);
        expect_token(&mut lexer, TokenKind::LeftBrace);
        expect_token(&mut lexer, TokenKind::RightBrace);
        expect_token(&mut lexer, TokenKind::LeftBracket);
        expect_token(&mut lexer, TokenKind::RightBracket);
        expect_token(&mut lexer, TokenKind::LeftParen);
        expect_token(&mut lexer, TokenKind::RightParen);
        expect_token(&mut lexer, TokenKind::LessThan);
        expect_token(&mut lexer, TokenKind::GreaterThan);
        expect_token(&mut lexer, TokenKind::SingleQuote);
        expect_token(&mut lexer, TokenKind::DoubleQuote);
        expect_token(&mut lexer, TokenKind::Newline);

        expect_eof(&mut lexer);
    }

    #[test]
    fn test_operators() {
        let input = "and or not + - / % = == != <= >= += -= *= /= %=";
        let mut lexer = Lexer::new(input);

        expect_token(&mut lexer, TokenKind::And);
        expect_token(&mut lexer, TokenKind::Or);
        expect_token(&mut lexer, TokenKind::Not);
        expect_token(&mut lexer, TokenKind::Plus);
        expect_token(&mut lexer, TokenKind::Minus);
        expect_token(&mut lexer, TokenKind::Divide);
        expect_token(&mut lexer, TokenKind::Modulo);
        expect_token(&mut lexer, TokenKind::Equal);
        expect_token(&mut lexer, TokenKind::DoubleEqual);
        expect_token(&mut lexer, TokenKind::NotEqual);
        expect_token(&mut lexer, TokenKind::LessEqual);
        expect_token(&mut lexer, TokenKind::GreaterEqual);
        expect_token(&mut lexer, TokenKind::PlusEqual);
        expect_token(&mut lexer, TokenKind::MinusEqual);
        expect_token(&mut lexer, TokenKind::TimesEqual);
        expect_token(&mut lexer, TokenKind::DivideEqual);
        expect_token(&mut lexer, TokenKind::ModuloEqual);

        assert!(lexer.next_token().unwrap().is_none());
    }

    #[test]
    fn test_identifiers_and_numbers() {
        let input = "var1 _var2 123 45.67 0.589";
        let mut lexer = Lexer::new(input);

        expect_token(&mut lexer, TokenKind::Identifier("var1".to_string()));
        expect_token(&mut lexer, TokenKind::Identifier("_var2".to_string()));
        expect_token(&mut lexer, TokenKind::Number(NumberLiteral::Integer(123)));
        expect_token(&mut lexer, TokenKind::Number(NumberLiteral::Float(45.67)));
        expect_token(&mut lexer, TokenKind::Number(NumberLiteral::Float(0.589)));

        expect_eof(&mut lexer);
    }

    #[test]
    fn test_comments() {
        let input = "x // single line comment\ny /* block comment \n\n\n */ z";
        let mut lexer = Lexer::new(input);

        expect_token(&mut lexer, TokenKind::Identifier("x".to_string()));
        expect_token(&mut lexer, TokenKind::Newline);
        expect_token(&mut lexer, TokenKind::Identifier("y".to_string()));
        expect_token(&mut lexer, TokenKind::Identifier("z".to_string()));

        expect_eof(&mut lexer);
    }

    #[test_case("x !", '!', 2 ; "after identifier")]
    #[test_case("x $", '$', 2 ; "after identifier with $")]
    #[test_case("!", '!', 0 ; "at start")]
    #[test_case("$", '$', 0 ; "at start with $")]
    fn test_unexpected_character(input: &str, char: char, position: usize) {
        let mut lexer = Lexer::new(input);

        loop {
            match lexer.next_token() {
                Ok(Some(_)) => continue,
                Ok(None) => panic!("Expected an error but got EOF"),
                Err(e) => {
                    assert_eq!(e, LexerError::UnexpectedCharacter(char, position));
                    return;
                }
            }
        }
    }

    #[test]
    fn test_invalid_number_format() {
        let input = "123.45.67";
        let mut lexer = Lexer::new(input);

        match lexer.next_token() {
            Ok(Some(_)) => panic!("Expected an error but got a token"),
            Ok(None) => panic!("Expected an error but got EOF"),
            Err(e) => {
                assert_eq!(e, LexerError::InvalidNumberFormat(0));
            }
        }
    }

    #[test]
    fn test_cv_code_sample() {
        let code = r#"
        fn add(i32 a, i32 b) -> i32 {
            a + b
        }
        "#;
        let mut lexer = Lexer::new(code);

        expect_token(&mut lexer, TokenKind::Newline);
        expect_token(&mut lexer, TokenKind::Fun);
        expect_token(&mut lexer, TokenKind::Identifier("add".to_string()));
        expect_token(&mut lexer, TokenKind::LeftParen);
        expect_token(&mut lexer, TokenKind::Identifier("i32".to_string()));
        expect_token(&mut lexer, TokenKind::Identifier("a".to_string()));
        expect_token(&mut lexer, TokenKind::Comma);
        expect_token(&mut lexer, TokenKind::Identifier("i32".to_string()));
        expect_token(&mut lexer, TokenKind::Identifier("b".to_string()));
        expect_token(&mut lexer, TokenKind::RightParen);
        expect_token(&mut lexer, TokenKind::RightArrow);
        expect_token(&mut lexer, TokenKind::Identifier("i32".to_string()));
        expect_token(&mut lexer, TokenKind::LeftBrace);
        expect_token(&mut lexer, TokenKind::Newline);
        expect_token(&mut lexer, TokenKind::Identifier("a".to_string()));
        expect_token(&mut lexer, TokenKind::Plus);
        expect_token(&mut lexer, TokenKind::Identifier("b".to_string()));
        expect_token(&mut lexer, TokenKind::Newline);
        expect_token(&mut lexer, TokenKind::RightBrace);
        expect_token(&mut lexer, TokenKind::Newline);
        expect_eof(&mut lexer);
    }

    #[test]
    fn test_variable_declaration() {
        let code = "variable = 22;";
        let mut lexer = Lexer::new(code);

        expect_token(&mut lexer, TokenKind::Identifier("variable".to_string()));
        expect_token(&mut lexer, TokenKind::Equal);
        expect_token(&mut lexer, TokenKind::Number(NumberLiteral::Integer(22)));
        expect_token(&mut lexer, TokenKind::Semicolon);
        expect_eof(&mut lexer);
    }

    #[test]
    fn test_iterator_trait() {
        let code = "x = 10 + 20;";
        let lexer = Lexer::new(code);
        let tokens: Vec<Token> = lexer.map(|res| res.expect("Lexer error")).collect();

        let expected_kinds = vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Equal,
            TokenKind::Number(NumberLiteral::Integer(10)),
            TokenKind::Plus,
            TokenKind::Number(NumberLiteral::Integer(20)),
            TokenKind::Semicolon,
        ];

        for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
            assert_eq!(&token.kind, expected_kind);
        }
    }
}
