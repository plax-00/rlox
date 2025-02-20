use std::iter::Peekable;

use anyhow::{bail, Result};

use crate::{
    expression::{Assign, Binary, Expression, Grouping, Literal, Unary, Var},
    operator::BinaryOperator,
    statement::{Stmt, VarDecl},
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self
            .tokens
            .peek()
            .is_some_and(|t| t.token_type != TokenType::EOF)
        {
            stmts.push(self.parse_decl()?);
        }
        Ok(stmts)
    }

    #[inline]
    fn expect_token(&mut self, token_type: TokenType) -> bool {
        self.tokens
            .next_if(|t| t.token_type == token_type)
            .is_some()
    }

    fn parse_decl(&mut self) -> Result<Stmt> {
        let stmt = if self.expect_token(TokenType::Var) {
            let Expression::Var(Var { name }) = self.parse_primary()? else {
                bail!("Expected variable name")
            };

            let mut initializer = None;
            if self.expect_token(TokenType::Equal) {
                let Stmt::ExprStmt(expr) = self.parse_stmt()? else {
                    bail!("Expected expression")
                };
                initializer = Some(Box::new(expr));
            }

            VarDecl { name, initializer }.into()
        } else {
            self.parse_stmt()?
        };
        Ok(stmt)
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        let stmt = if self.expect_token(TokenType::Print) {
            Stmt::PrintStmt(self.parse_expr()?)
        } else {
            Stmt::ExprStmt(self.parse_expr()?)
        };
        if !self.expect_token(TokenType::Semicolon) {
            bail!("Expected `;`")
        }

        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expression> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, right_bp: u8) -> Result<Expression> {
        let mut expr = self.parse_head()?;
        while let Some(op) = self
            .tokens
            .peek()
            .and_then(|t| BinaryOperator::try_from(t).ok())
            .filter(|o| infix_binding_power(o) > right_bp)
        {
            self.tokens.next();
            expr = self.parse_tail(expr, op.into())?;
        }
        Ok(expr)
    }

    #[inline]
    fn parse_head(&mut self) -> Result<Expression> {
        if let Some(ref t) = self
            .tokens
            .next_if(|t| matches!(t.token_type, TokenType::Minus | TokenType::Bang))
        {
            let operator = t.try_into().unwrap();
            let expr = self.parse_expr_bp(PREFIX_BINDING_POWER).map(Box::new)?;
            let unary = Unary { operator, expr }.into();

            Ok(unary)
        } else {
            self.parse_primary()
        }
    }

    #[inline]
    fn parse_tail(&mut self, left: Expression, op: BinaryOperator) -> Result<Expression> {
        let operator_bp = infix_binding_power(&op);
        let right = self.parse_expr_bp(operator_bp)?;
        let expr = match op {
            BinaryOperator::Equal => Assign {
                name: Box::new(left),
                value: Box::new(right),
            }
            .into(),
            _ => Binary {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            }
            .into(),
        };

        Ok(expr)
    }

    #[inline]
    fn parse_primary(&mut self) -> Result<Expression> {
        let t = self.tokens.next().expect("Expected token(s) to parse.");
        let expr = match t.token_type {
            TokenType::True => Literal::True.into(),
            TokenType::False => Literal::False.into(),
            TokenType::Nil => Literal::Nil.into(),
            TokenType::Number(n) => Literal::Number(n).into(),
            TokenType::String(s) => Literal::String(s).into(),
            TokenType::Identifier(name) => Var { name }.into(),
            TokenType::LeftParen => {
                let expr = self.parse_expr()?;
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
        };
        Ok(expr)
    }

    fn synchronize(&mut self) {
        todo!()
    }
}

enum BindingPower {
    Assignment = 1,
    AndOr,
    Equality,
    Comparison,
    Additive,
    Multiplicative,
    Unary,
}

fn infix_binding_power(op: &BinaryOperator) -> u8 {
    let bp = match *op {
        BinaryOperator::Equal => BindingPower::Assignment,
        BinaryOperator::And | BinaryOperator::Or => BindingPower::AndOr,
        BinaryOperator::EqualEqual | BinaryOperator::NotEqual => BindingPower::Equality,
        BinaryOperator::Greater
        | BinaryOperator::Less
        | BinaryOperator::GreaterEqual
        | BinaryOperator::LessEqual => BindingPower::Comparison,
        BinaryOperator::Minus | BinaryOperator::Plus => BindingPower::Additive,
        BinaryOperator::Mult | BinaryOperator::Div => BindingPower::Multiplicative,
    };
    bp as u8
}

const PREFIX_BINDING_POWER: u8 = BindingPower::Unary as u8;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast_print::AstPrinter, scanner::Scanner};

    const PRINTER: AstPrinter = AstPrinter;

    fn parse_source(source: &'static str) -> Expression {
        let tokens = Scanner::new(source.to_string()).scan_source().unwrap();
        Parser::new(tokens).parse_expr().unwrap()
    }

    #[test]
    fn test_parse_expr() {
        let expr = parse_source("2 + 3 + 4");
        assert_eq!("(+ (+ 2 3) 4)", PRINTER.print(&expr));

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

        let expr = parse_source("true and !false");
        assert_eq!("(and true (! false))", PRINTER.print(&expr));

        let expr = parse_source("2 + 2 == 4 and true and 3 <= 4");
        assert_eq!(
            "(and (and (== (+ 2 2) 4) true) (<= 3 4))",
            PRINTER.print(&expr)
        );
    }
}
