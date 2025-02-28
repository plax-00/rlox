use std::iter::Peekable;

use anyhow::{anyhow, bail, Result};

use crate::{
    expression::{Assign, Binary, Expression, Grouping, Literal, Unary, Var},
    operator::BinaryOperator,
    statement::{BlockStmt, IfStmt, Stmt, VarDecl, WhileStmt},
    token::{Token, TokenType, Tokens},
};

pub struct Parser {
    tokens: Peekable<Tokens>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = Tokens::from(tokens).peekable();
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        self.parse_until(TokenType::EOF)
    }

    fn parse_until(&mut self, until: TokenType) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self.tokens.peek().is_some_and(|t| t.token_type != until) {
            stmts.push(self.parse_decl()?);
        }
        self.tokens.next();

        Ok(stmts)
    }

    #[inline]
    fn expect_token(&mut self, token_type: TokenType) -> bool {
        self.tokens
            .next_if(|t| t.token_type == token_type)
            .is_some()
    }

    #[inline]
    fn expect_semicolon(&mut self) -> Result<()> {
        if !self.expect_token(TokenType::Semicolon) {
            bail!("Expected `;`");
        }
        Ok(())
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
        let stmt = match self.tokens.peek().map(|t| &t.token_type) {
            Some(TokenType::Print) => self.parse_print_stmt()?,
            Some(TokenType::LeftBrace) => self.parse_block_stmt()?,
            Some(TokenType::If) => self.parse_if_stmt()?,
            Some(TokenType::While) => self.parse_while_stmt()?,
            _ => {
                let stmt = Stmt::ExprStmt(self.parse_expr()?);
                self.expect_semicolon()?;
                stmt
            }
        };

        Ok(stmt)
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt> {
        self.tokens.next();
        let stmt = Stmt::PrintStmt(self.parse_expr()?);
        self.expect_semicolon()?;
        Ok(stmt)
    }

    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        self.tokens.next();
        let stmts = self.parse_until(TokenType::RightBrace)?;

        Ok(BlockStmt { stmts }.into())
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.tokens.next();
        let Expression::Grouping(grouping) = self.parse_expr()? else {
            bail!("Expected `(`")
        };
        let condition = grouping.expr;
        let then_branch = self.parse_stmt().map(Box::new)?;
        let else_branch = match self.expect_token(TokenType::Else) {
            true => self.parse_stmt().map(Box::new).map(Some)?,
            false => None,
        };

        Ok(IfStmt {
            condition,
            then_branch,
            else_branch,
        }
        .into())
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.tokens.next();
        let Expression::Grouping(grouping) = self.parse_expr()? else {
            bail!("Expected `(`")
        };
        let condition = grouping.expr;
        let body = self.parse_stmt().map(Box::new)?;

        Ok(WhileStmt { condition, body }.into())
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
            expr = self.parse_tail(expr, op)?;
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
        let t = self
            .tokens
            .next()
            .ok_or_else(|| anyhow!("Expected token"))?;

        let expr = match t.token_type {
            TokenType::True => Literal::True.into(),
            TokenType::False => Literal::False.into(),
            TokenType::Nil => Literal::Nil.into(),
            TokenType::Number(n) => Literal::Number(n).into(),
            TokenType::String(s) => Literal::String(s).into(),
            TokenType::Identifier(name) => Var { name }.into(),
            TokenType::LeftParen => {
                let expr = self.parse_expr()?;
                if !self.expect_token(TokenType::RightParen) {
                    bail!("Expected `)`");
                }

                Grouping {
                    expr: Box::new(expr),
                }
                .into()
            }
            TokenType::EOF => bail!("Unexpected EOF"),
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
    Or,
    And,
    Equality,
    Comparison,
    Additive,
    Multiplicative,
    Unary,
}

fn infix_binding_power(op: &BinaryOperator) -> u8 {
    let bp = match *op {
        BinaryOperator::Equal => BindingPower::Assignment,
        BinaryOperator::Or => BindingPower::Or,
        BinaryOperator::And => BindingPower::And,
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
mod tests;
