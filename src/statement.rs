use rlox_macros::Visitor;

use crate::{expression::Expression, impl_from_inner};

#[derive(Debug, Visitor)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum Stmt {
    ExprStmt(Expression),
    PrintStmt(Expression),
    VarDecl(VarDecl),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
}

impl_from_inner!(VarDecl, Stmt);
impl_from_inner!(BlockStmt, Stmt);
impl_from_inner!(IfStmt, Stmt);

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct VarDecl {
    pub name: String,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct IfStmt {
    pub condition: Box<Expression>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
