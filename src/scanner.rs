use std::{iter::Peekable, str::Chars};

use crate::{
    error::SyntaxError,
    token::{Token, TokenType, KEYWORDS},
};

pub struct Scanner {
    source: String,
    line_num: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            line_num: 1,
        }
    }

    pub fn scan_source(mut self) -> Result<Vec<Token>, Vec<SyntaxError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let source = std::mem::take(&mut self.source);

        'lines: for line_text in source.lines() {
            let mut line = line_text.chars().peekable();
            loop {
                let next = line.next();
                let Some(c) = next else { break };
                match c {
                    // Single-character tokens.
                    ' ' | '\t' | '\r' => continue,
                    '(' => tokens.push(Token::new(TokenType::LeftParen, self.line_num)),
                    ')' => tokens.push(Token::new(TokenType::RightParen, self.line_num)),
                    '{' => tokens.push(Token::new(TokenType::LeftBrace, self.line_num)),
                    '}' => tokens.push(Token::new(TokenType::LeftParen, self.line_num)),
                    ',' => tokens.push(Token::new(TokenType::Comma, self.line_num)),
                    '.' => tokens.push(Token::new(TokenType::Dot, self.line_num)),
                    '-' => tokens.push(Token::new(TokenType::Minus, self.line_num)),
                    '+' => tokens.push(Token::new(TokenType::Plus, self.line_num)),
                    ';' => tokens.push(Token::new(TokenType::Semicolon, self.line_num)),
                    '*' => tokens.push(Token::new(TokenType::Star, self.line_num)),
                    // One or two character tokens.
                    '!' => match line.peek() {
                        Some(n) => {
                            if let &'=' = n {
                                line.next();
                                tokens.push(Token::new(TokenType::BangEqual, self.line_num));
                            } else {
                                tokens.push(Token::new(TokenType::Bang, self.line_num));
                            }
                        }
                        None => errors.push(SyntaxError {
                            line_num: self.line_num,
                        }),
                    },
                    '=' => match line.peek() {
                        Some(n) => {
                            if let &'=' = n {
                                line.next();
                                tokens.push(Token::new(TokenType::EqualEqual, self.line_num));
                            } else {
                                tokens.push(Token::new(TokenType::Equal, self.line_num));
                            }
                        }
                        None => errors.push(SyntaxError {
                            line_num: self.line_num,
                        }),
                    },
                    '>' => match line.peek() {
                        Some(n) => {
                            if let &'=' = n {
                                line.next();
                                tokens.push(Token::new(TokenType::GreaterEqual, self.line_num));
                            } else {
                                tokens.push(Token::new(TokenType::Greater, self.line_num));
                            }
                        }
                        None => errors.push(SyntaxError {
                            line_num: self.line_num,
                        }),
                    },
                    '<' => match line.peek() {
                        Some(n) => {
                            if let &'=' = n {
                                line.next();
                                tokens.push(Token::new(TokenType::LessEqual, self.line_num));
                            } else {
                                tokens.push(Token::new(TokenType::Less, self.line_num));
                            }
                        }
                        None => errors.push(SyntaxError {
                            line_num: self.line_num,
                        }),
                    },
                    '/' => match line.peek() {
                        Some(n) => {
                            if let &'/' = n {
                                self.line_num += 1;
                                continue 'lines;
                            } else {
                                tokens.push(Token::new(TokenType::Slash, self.line_num));
                            }
                        }
                        None => errors.push(SyntaxError {
                            line_num: self.line_num,
                        }),
                    },
                    // Literals.
                    '"' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        match self.scan_literal(&mut line, c) {
                            Ok(t) => tokens.push(t),
                            Err(e) => errors.push(e),
                        }
                    }
                    c if c.is_ascii_alphanumeric() => match self.scan_identifier(&mut line, c) {
                        Ok(t) => tokens.push(t),
                        Err(e) => errors.push(e),
                    },
                    _ => errors.push(SyntaxError {
                        line_num: self.line_num,
                    }),
                };
            }
            self.line_num += 1;
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_literal(
        &mut self,
        iter: &mut Peekable<Chars>,
        current: char,
    ) -> Result<Token, SyntaxError> {
        match current {
            '"' => {
                if let None = iter.peek() {
                    return Err(SyntaxError {
                        line_num: self.line_num,
                    });
                }
                let mut s = String::new();
                while let Some(c) = iter.next() {
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                    if c == '\n' {
                        self.line_num += 1
                    }
                }
                Ok(Token::new(TokenType::String(s), self.line_num))
            }
            digit if digit.is_ascii_digit() => {
                let mut number = digit.to_string();
                while let Some(d) = iter.peek() {
                    if !d.is_ascii_digit() && *d != '.' {
                        break;
                    }
                    number.push(iter.next().unwrap());
                }
                match number.parse::<f64>() {
                    Ok(n) => Ok(Token::new(TokenType::Number(n), self.line_num)),
                    Err(_) => Err(SyntaxError {
                        line_num: self.line_num,
                    }),
                }
            }
            _ => todo!(),
        }
    }

    fn scan_identifier(
        &mut self,
        iter: &mut Peekable<Chars>,
        current: char,
    ) -> Result<Token, SyntaxError> {
        let mut token = current.to_string();
        while let Some(n) = iter.peek() {
            if matches!(n, &'\n' | &'\t' | &' ' | '\r') || !(n.is_ascii_alphanumeric() || &'_' == n)
            {
                break;
            }
            token.push(iter.next().unwrap());
        }
        if let Some(token_type) = KEYWORDS.get(token.as_str()) {
            Ok(Token::new(token_type.clone(), self.line_num))
        } else {
            Ok(Token::new(TokenType::Identifier(token), self.line_num))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_string_test() {
        let source = r#"
                "hello, world";
                "hello again!";
            "#;
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_source().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::String("hello, world".to_string()), 2),
                Token::new(TokenType::Semicolon, 2),
                Token::new(TokenType::String("hello again!".to_string()), 3),
                Token::new(TokenType::Semicolon, 3),
            ]
        );
    }

    #[test]
    fn scan_number_test() {
        let source = r#"
            12345;
            123.456;
        "#;
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_source().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Number(12345.0), 2),
                Token::new(TokenType::Semicolon, 2),
                Token::new(TokenType::Number(123.456), 3),
                Token::new(TokenType::Semicolon, 3),
            ]
        );
    }

    #[test]
    fn scan_comment_test() {
        let source = r#"
            // This is a comment
            5 / 10;
        "#;
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_source().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Number(5.0), 3),
                Token::new(TokenType::Slash, 3),
                Token::new(TokenType::Number(10.0), 3),
                Token::new(TokenType::Semicolon, 3),
            ]
        );
    }

    #[test]
    fn scan_identifier_test() {
        let source = r#"
            var test_var = "hello, world";
            print test_var;
        "#;
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_source().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Var, 2),
                Token::new(TokenType::Identifier("test_var".to_string()), 2),
                Token::new(TokenType::Equal, 2),
                Token::new(TokenType::String("hello, world".to_string()), 2),
                Token::new(TokenType::Semicolon, 2),
                Token::new(TokenType::Print, 3),
                Token::new(TokenType::Identifier("test_var".to_string()), 3),
                Token::new(TokenType::Semicolon, 3),
            ]
        );
    }

    #[test]
    fn scanner_test() {
        let source = r#"
            // Your first Lox program!
            print "Hello, world!";
            divide / me;
            -negateMe;
            lessThan <= orEqual;
            greater > than;
            greaterThan >= orEqual;
        "#;
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_source().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Print, 3),
                Token::new(TokenType::String("Hello, world!".to_string()), 3),
                Token::new(TokenType::Semicolon, 3),
                Token::new(TokenType::Identifier("divide".to_string()), 4),
                Token::new(TokenType::Slash, 4),
                Token::new(TokenType::Identifier("me".to_string()), 4),
                Token::new(TokenType::Semicolon, 4),
                Token::new(TokenType::Minus, 5),
                Token::new(TokenType::Identifier("negateMe".to_string()), 5),
                Token::new(TokenType::Semicolon, 5),
                Token::new(TokenType::Identifier("lessThan".to_string()), 6),
                Token::new(TokenType::LessEqual, 6),
                Token::new(TokenType::Identifier("orEqual".to_string()), 6),
                Token::new(TokenType::Semicolon, 6),
                Token::new(TokenType::Identifier("greater".to_string()), 7),
                Token::new(TokenType::Greater, 7),
                Token::new(TokenType::Identifier("than".to_string()), 7),
                Token::new(TokenType::Semicolon, 7),
                Token::new(TokenType::Identifier("greaterThan".to_string()), 8),
                Token::new(TokenType::GreaterEqual, 8),
                Token::new(TokenType::Identifier("orEqual".to_string()), 8),
                Token::new(TokenType::Semicolon, 8),
            ]
        );
    }

    #[test]
    fn sytnax_error_test() {
        let source = r#"
            "
        "#;
        let scanner = Scanner::new(source.to_string());
        let errors = scanner.scan_source().err().unwrap();
        assert_eq!(errors, vec![SyntaxError { line_num: 2 }]);
    }
}
