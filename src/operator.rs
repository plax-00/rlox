use std::sync::LazyLock;

use anyhow::bail;
use rustc_hash::FxHashMap;

use crate::token::{Token, TokenType};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum BinaryOperator {
    Minus,
    Plus,
    Div,
    Mult,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

impl From<UnaryOperator> for Operator {
    fn from(value: UnaryOperator) -> Self {
        Self::Unary(value)
    }
}

impl From<BinaryOperator> for Operator {
    fn from(value: BinaryOperator) -> Self {
        Self::Binary(value)
    }
}

impl<'a> TryFrom<&'a Token> for UnaryOperator {
    type Error = anyhow::Error;
    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::Minus => Ok(Self::Minus),
            TokenType::Bang => Ok(Self::Not),
            _ => bail!("{:?} is not a unary operator", value),
        }
    }
}

impl<'a> TryFrom<&'a Token> for BinaryOperator {
    type Error = anyhow::Error;
    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value.token_type {
            TokenType::Minus => Ok(Self::Minus),
            TokenType::Plus => Ok(Self::Plus),
            TokenType::Slash => Ok(Self::Div),
            TokenType::Star => Ok(Self::Mult),
            TokenType::BangEqual => Ok(Self::NotEqual),
            TokenType::Equal => Ok(Self::Equal),
            TokenType::EqualEqual => Ok(Self::EqualEqual),
            TokenType::Greater => Ok(Self::Greater),
            TokenType::GreaterEqual => Ok(Self::GreaterEqual),
            TokenType::Less => Ok(Self::Less),
            TokenType::LessEqual => Ok(Self::LessEqual),
            TokenType::And => Ok(Self::And),
            TokenType::Or => Ok(Self::Or),
            _ => bail!("{:?} is not a binary operator", value),
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = *OPERATORS
            .get(&Operator::Binary(*self))
            .expect("Operator should be in hash map");
        write!(f, "{}", repr)
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = *OPERATORS
            .get(&Operator::Unary(*self))
            .expect("Operator should be in hash map");
        write!(f, "{}", repr)
    }
}

static OPERATORS: LazyLock<FxHashMap<Operator, &str>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (Operator::Binary(BinaryOperator::Minus), "-"),
        (Operator::Unary(UnaryOperator::Minus), "-"),
        (Operator::Binary(BinaryOperator::Plus), "+"),
        (Operator::Binary(BinaryOperator::Div), "/"),
        (Operator::Binary(BinaryOperator::Mult), "*"),
        (Operator::Unary(UnaryOperator::Not), "!"),
        (Operator::Binary(BinaryOperator::NotEqual), "!="),
        (Operator::Binary(BinaryOperator::Equal), "="),
        (Operator::Binary(BinaryOperator::EqualEqual), "=="),
        (Operator::Binary(BinaryOperator::Greater), ">"),
        (Operator::Binary(BinaryOperator::GreaterEqual), ">="),
        (Operator::Binary(BinaryOperator::Less), "<"),
        (Operator::Binary(BinaryOperator::LessEqual), "<="),
        (Operator::Binary(BinaryOperator::And), "and"),
        (Operator::Binary(BinaryOperator::Or), "or"),
    ])
});
