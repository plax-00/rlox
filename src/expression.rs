use rlox_macros::Visitor;

use crate::operator::{BinaryOperator, UnaryOperator};

#[derive(Debug, Visitor)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

impl From<Literal> for Expression {
    #[inline]
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}
impl From<Unary> for Expression {
    #[inline]
    fn from(value: Unary) -> Self {
        Self::Unary(value)
    }
}
impl From<Binary> for Expression {
    #[inline]
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}
impl From<Grouping> for Expression {
    #[inline]
    fn from(value: Grouping) -> Self {
        Self::Grouping(value)
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expr: Box<Expression>,
}

#[derive(Debug)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expression>,
}
