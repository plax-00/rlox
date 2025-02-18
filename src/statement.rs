use rlox_macros::Visitor;

use crate::expression::Expression;

#[derive(Debug, Visitor)]
pub enum Stmt {
    ExprStmt(Expression),
    PrintStmt(Expression),
}
