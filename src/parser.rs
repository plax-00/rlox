use std::iter::Peekable;

use crate::{
    expression::{Binary, Expression, Grouping, Literal, Operator, Unary},
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

    fn parse_expr(&mut self) -> Expression {
        self.parse_recursive(0)
    }

    fn parse_recursive(&mut self, right_bp: u8) -> Expression {
        let mut expr = self.parse_head();
        while let Some(op) = self
            .tokens
            .peek()
            .and_then(|t| Operator::try_from(t).ok())
            .filter(|o| infix_binding_power(&o) > right_bp)
        {
            self.tokens.next();
            expr = self.parse_tail(expr, op);
        }
        expr
    }

    #[inline]
    fn parse_head(&mut self) -> Expression {
        if let Some(ref t) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Minus | TokenType::Bang))
        {
            let operator = t.try_into().unwrap();
            let expr = Box::new(self.parse_recursive(prefix_binding_power(&operator)));
            Unary { operator, expr }.into()
        } else {
            self.parse_primary()
        }
    }

    #[inline]
    fn parse_tail(&mut self, left: Expression, op: Operator) -> Expression {
        let operator_bp = infix_binding_power(&op);
        let right = self.parse_recursive(operator_bp);
        Binary {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        }
        .into()
    }

    #[inline]
    fn parse_primary(&mut self) -> Expression {
        let t = self.tokens.next().expect("Expected token(s) to parse.");
        match t.token_type {
            TokenType::True => Literal::True.into(),
            TokenType::False => Literal::False.into(),
            TokenType::Nil => Literal::Nil.into(),
            TokenType::Number(n) => Literal::Number(n).into(),
            TokenType::String(s) => Literal::String(s).into(),
            TokenType::LeftParen => {
                let expr = self.parse_expr();
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

enum BindingPower {
    Equality = 1,
    Comparison,
    Additive,
    Multiplicative,
    Unary,
}

fn infix_binding_power(op: &BinaryOperator) -> u8 {
    let bp = match *op {
        Operator::EqualEqual | Operator::NotEqual => BindingPower::Equality,
        Operator::Greater | Operator::Less | Operator::GreaterEqual | Operator::LessEqual => {
            BindingPower::Comparison
        }
        Operator::Minus | Operator::Plus => BindingPower::Additive,
        Operator::Mult | Operator::Div => BindingPower::Multiplicative,
        _ => todo!(),
    };
    bp as u8
}

fn prefix_binding_power(op: &Operator) -> u8 {
    let (Operator::Minus | Operator::Not) = *op else {
        panic!()
    };
    BindingPower::Unary as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast_print::AstPrinter, scanner::Scanner};

    const PRINTER: AstPrinter = AstPrinter;

    fn parse_source(source: &'static str) -> Expression {
        let tokens = Scanner::new(source.to_string()).scan_source().unwrap();
        Parser::new(tokens).parse_expr()
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

        let expr = parse_source("1 <= 2 * 3");
        assert_eq!("(<= 1 (* 2 3))", PRINTER.print(&expr));

        let expr = parse_source("!(-2 < 4)");
        assert_eq!("(! (group (< (- 2) 4)))", PRINTER.print(&expr));

        let expr = parse_source("!!true != !!false");
        assert_eq!("(!= (! (! true)) (! (! false)))", PRINTER.print(&expr));

        let expr = parse_source("4 == 2 - -2");
        assert_eq!("(== 4 (- 2 (- 2)))", PRINTER.print(&expr));
    }
}
