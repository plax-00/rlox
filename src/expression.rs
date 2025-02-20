use rlox_macros::Visitor;

use crate::operator::{BinaryOperator, UnaryOperator};

#[derive(Debug, Visitor)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Var(Var),
    Assign(Assign),
}

macro_rules! impl_from {
    ($from:ident, $for:ty) => {
        impl From<$from> for $for {
            fn from(value: $from) -> Self {
                Self::$from(value)
            }
        }
    };
}

impl_from!(Literal, Expression);
impl_from!(Unary, Expression);
impl_from!(Binary, Expression);
impl_from!(Grouping, Expression);
impl_from!(Var, Expression);
impl_from!(Assign, Expression);

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

#[derive(Debug)]
pub struct Var {
    pub name: String,
}

#[derive(Debug)]
pub struct Assign {
    pub name: Box<Expression>,
    pub value: Box<Expression>,
}
