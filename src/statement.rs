use crate::expression::Expression;

pub trait StmtVisitor {
    type Return;
    fn visit_expr_stmt(&self, expr: &Expression) -> Self::Return;
    fn visit_print_stmt(&self, expr: &Expression) -> Self::Return;
}

impl<T: StmtVisitor> StmtVisitor for &T {
    type Return = T::Return;
    #[inline]
    fn visit_expr_stmt(&self, expr: &Expression) -> Self::Return {
        (*self).visit_expr_stmt(expr)
    }

    #[inline]
    fn visit_print_stmt(&self, expr: &Expression) -> Self::Return {
        (*self).visit_print_stmt(expr)
    }
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expression),
    PrintStmt(Expression),
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: V) -> V::Return {
        match self {
            Self::ExprStmt(expr) => visitor.visit_expr_stmt(expr),
            Self::PrintStmt(expr) => visitor.visit_print_stmt(expr),
        }
    }
}
