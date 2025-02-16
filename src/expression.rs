use crate::{
    error::OperatorParseError,
    token::{Token, TokenType},
};

pub trait ExprVisitor {
    type Return;
    fn visit_literal(&self, literal: &Literal) -> Self::Return;
    fn visit_unary(&self, unary: &Unary) -> Self::Return;
    fn visit_binary(&self, binary: &Binary) -> Self::Return;
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Return;
}

pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

impl Expression {
    pub fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return {
        match self {
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
        }
    }
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

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Unary {
    pub operator: Operator,
    pub expr: Box<Expression>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Operator {
    Minus,
    Plus,
    Div,
    Mult,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl<'a> TryFrom<&'a Token> for Operator {
    type Error = OperatorParseError<'a>;
    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::Minus => Ok(Self::Minus),
            TokenType::Plus => Ok(Self::Plus),
            TokenType::Slash => Ok(Self::Div),
            TokenType::Star => Ok(Self::Mult),
            TokenType::Equal => Ok(Self::Equal),
            TokenType::EqualEqual => Ok(Self::EqualEqual),
            TokenType::Greater => Ok(Self::Greater),
            TokenType::GreaterEqual => Ok(Self::GreaterEqual),
            TokenType::Less => Ok(Self::Less),
            TokenType::LessEqual => Ok(Self::LessEqual),
            _ => Err(OperatorParseError { token: value }),
        }
    }
}

pub struct Binary {
    pub operator: Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Grouping {
    pub expr: Box<Expression>,
}
