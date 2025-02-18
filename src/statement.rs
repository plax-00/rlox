use rlox_macros::Visitor;

use crate::expression::Expression;

#[derive(Debug, Visitor)]
pub enum Stmt {
    ExprStmt(Expression),
    PrintStmt(Expression),
    Var(VarDecl),
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub initializer: Option<Box<Expression>>,
}

impl From<VarDecl> for Stmt {
    #[inline]
    fn from(value: VarDecl) -> Self {
        Self::Var(value)
    }
}
