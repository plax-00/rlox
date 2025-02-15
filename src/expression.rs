pub trait ExprVisitor {
    type Return;
    fn visit_literal(&self, literal: &Literal) -> Self::Return;
    fn visit_unary<E: ExprAccept>(&self, unary: &Unary<E>) -> Self::Return;
    fn visit_binary<L: ExprAccept, R: ExprAccept>(&self, binary: &Binary<L, R>) -> Self::Return;
    fn visit_grouping<E: ExprAccept>(&self, grouping: &Grouping<E>) -> Self::Return;
}

pub trait Expr {}

pub trait ExprAccept: Expr {
    fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return;
}

impl ExprAccept for Literal {
    fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return {
        visitor.visit_literal(self)
    }
}
impl<E: ExprAccept> ExprAccept for Unary<E> {
    fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return {
        visitor.visit_unary(self)
    }
}
impl<L: ExprAccept, R: ExprAccept> ExprAccept for Binary<L, R> {
    fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return {
        visitor.visit_binary(self)
    }
}
impl<E: ExprAccept> ExprAccept for Grouping<E> {
    fn accept<V: ExprVisitor>(&self, visitor: V) -> V::Return {
        visitor.visit_grouping(self)
    }
}

impl Expr for Literal {}
impl<E: ExprAccept> Expr for Unary<E> {}
impl<L: ExprAccept, R: ExprAccept> Expr for Binary<L, R> {}
impl<E: ExprAccept> Expr for Grouping<E> {}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Unary<E: ExprAccept> {
    pub operator: Operator,
    pub expr: Box<E>,
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

pub struct Binary<L: ExprAccept, R: ExprAccept> {
    pub operator: Operator,
    pub left: Box<L>,
    pub right: Box<R>,
}

pub struct Grouping<E: ExprAccept> {
    pub expr: Box<E>,
}
