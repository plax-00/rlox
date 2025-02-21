use rlox_macros::Visitor;

use crate::{
    impl_from_inner,
    operator::{BinaryOperator, UnaryOperator},
};

#[derive(Debug, Visitor)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Var(Var),
    Assign(Assign),
}

impl_from_inner!(Literal, Expression);
impl_from_inner!(Unary, Expression);
impl_from_inner!(Binary, Expression);
impl_from_inner!(Grouping, Expression);
impl_from_inner!(Var, Expression);
impl_from_inner!(Assign, Expression);

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
