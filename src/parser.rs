use std::iter::Peekable;

use crate::{
    expression::{Binary, Expression, Grouping, Literal, Unary},
    token::{Token, TokenType, Tokens},
};

pub struct Parser {
    tokens: Peekable<Tokens>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = Tokens::from(tokens).peekable();
        let current = 0;
        Self { tokens, current }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while let Some(ref next) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::BangEqual | TokenType::EqualEqual))
        {
            expr = Binary {
                operator: next.try_into().unwrap(),
                left: Box::new(expr),
                right: Box::new(self.comparison()),
            }
            .into();
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while let Some(ref next) = self.tokens.next_if(|t| {
            matches!(
                t.token_type,
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            )
        }) {
            expr = Binary {
                operator: next.try_into().unwrap(),
                left: Box::new(expr),
                right: Box::new(self.term()),
            }
            .into();
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while let Some(ref next) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Minus | TokenType::Plus))
        {
            expr = Binary {
                operator: next.try_into().unwrap(),
                left: Box::new(expr),
                right: Box::new(self.factor()),
            }
            .into();
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while let Some(ref next) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Slash | TokenType::Star))
        {
            expr = Binary {
                operator: next.try_into().unwrap(),
                left: Box::new(expr),
                right: Box::new(self.unary()),
            }
            .into();
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if let Some(ref next) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Bang | TokenType::Minus))
        {
            return Unary {
                operator: next.try_into().unwrap(),
                expr: Box::new(self.unary()),
            }
            .into();
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        let Some(t) = self.tokens.next() else {
            panic!()
        };
        match t.token_type {
            TokenType::True => Literal::True.into(),
            TokenType::False => Literal::False.into(),
            TokenType::Nil => Literal::Nil.into(),
            TokenType::Number(n) => Literal::Number(n).into(),
            TokenType::String(s) => Literal::String(s).into(),
            TokenType::LeftParen => {
                let expr = self.expression();
                let Some(Token {
                    token_type: TokenType::RightParen,
                    ..
                }) = self.tokens.next()
                else {
                    panic!()
                };
                Grouping {
                    expr: Box::new(expr),
                }
                .into()
            }
            _ => panic!("{}", t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast_print::AstPrinter, scanner::Scanner};

    const PRINTER: AstPrinter = AstPrinter;

    fn parse_source(source: &'static str) -> Expression {
        let tokens = Scanner::new(source.to_string()).scan_source().unwrap();
        Parser::new(tokens).expression()
    }

    #[test]
    fn test_parse_expr() {
        let expr = parse_source("2 + 3");
        assert_eq!("(+ 2 3)", PRINTER.print(&expr));

        let expr = parse_source("-123 * (45.67)");
        assert_eq!("(* (- 123) (group 45.67))", PRINTER.print(&expr));

        let expr = parse_source("(2 + 2) * (3 + -1)");
        assert_eq!(
            "(* (group (+ 2 2)) (group (+ 3 (- 1))))",
            PRINTER.print(&expr)
        );

        let expr = parse_source(r#" "string" * (3 * 4) / (2 + 1 * 3)"#);
        assert_eq!(
            r#"(/ (* "string" (group (* 3 4))) (group (+ 2 (* 1 3))))"#,
            PRINTER.print(&expr)
        );
    }
}
