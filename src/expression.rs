use std::{marker::PhantomData, sync::LazyLock};

use rustc_hash::FxHashMap;

pub trait ExprVisitor<Ret> {
    fn visit_literal(&self, expr: &Literal) -> Ret;
    fn visit_unary<E: Expr<Ret>>(&self, expr: &Unary<E, Ret>) -> Ret;
    fn visit_binary<L: Expr<Ret>, R: Expr<Ret>>(&self, expr: &Binary<L, R, Ret>) -> Ret;
    fn visit_grouping<E: Expr<Ret>>(&self, expr: &Grouping<E, Ret>) -> Ret;
}

pub trait Expr<Ret> {
    fn accept<V: ExprVisitor<Ret>>(&self, visitor: V) -> Ret;
}

impl<Ret> Expr<Ret> for Literal {
    fn accept<V: ExprVisitor<Ret>>(&self, visitor: V) -> Ret {
        visitor.visit_literal(self)
    }
}
impl<E: Expr<Ret>, Ret> Expr<Ret> for Unary<E, Ret> {
    fn accept<V: ExprVisitor<Ret>>(&self, visitor: V) -> Ret {
        visitor.visit_unary(self)
    }
}
impl<L: Expr<Ret>, R: Expr<Ret>, Ret> Expr<Ret> for Binary<L, R, Ret> {
    fn accept<V: ExprVisitor<Ret>>(&self, visitor: V) -> Ret {
        visitor.visit_binary(self)
    }
}
impl<E: Expr<Ret>, Ret> Expr<Ret> for Grouping<E, Ret> {
    fn accept<V: ExprVisitor<Ret>>(&self, visitor: V) -> Ret {
        visitor.visit_grouping(self)
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

pub struct Unary<E: Expr<Ret>, Ret> {
    pub operator: Operator,
    pub expr: Box<E>,
    _phantom: PhantomData<Ret>
}

#[derive(Debug)]
pub enum BinaryOperator {
    EqEq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    Plus,
    Minus,
    Mult,
    Divide,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Operator {
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = *OPERATORS.get(self).expect("Operator should be in hash map");
        write!(f, "{}", repr)
    }
}

pub struct Binary<L: Expr<Ret>, R: Expr<Ret>, Ret> {
    pub left: Box<L>,
    pub right: Box<R>,
    pub operator: Operator,
    _phantom: PhantomData<Ret>
}

pub struct Grouping<E: Expr<Ret>, Ret> {
    pub expr: Box<E>,
    _phantom: PhantomData<Ret>
}

static OPERATORS: LazyLock<FxHashMap<Operator, &str>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (Operator::Minus,        "-"),
        (Operator::Plus,         "+"),
        (Operator::Slash,        "/"),
        (Operator::Star,         "*"),
        (Operator::Bang,         "!"),
        (Operator::BangEqual,    "!="),
        (Operator::Equal,        "="),
        (Operator::EqualEqual,   "=="),
        (Operator::Greater,      ">"),
        (Operator::GreaterEqual, ">="),
        (Operator::Less,         "<"),
        (Operator::LessEqual,    "<="),
    ])
});
