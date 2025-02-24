use crate::expression::{
    Assign, Binary, Expression, ExpressionVisitor, Grouping, Literal, Unary, Var,
};

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, expr: &Expression) -> String {
        expr.accept(self)
    }
}

impl ExpressionVisitor for AstPrinter {
    type Return = String;
    fn visit_literal(&mut self, inner: &Literal) -> Self::Return {
        let repr = match inner {
            Literal::Number(n) => &format!("{}", n),
            Literal::String(s) => &format!(r#""{}""#, s),
            Literal::True => "true",
            Literal::False => "false",
            Literal::Nil => "nil",
        };
        format!("{}", repr)
    }

    fn visit_unary(&mut self, inner: &Unary) -> Self::Return {
        format!("({} {})", inner.operator, self.print(inner.expr.as_ref()))
    }

    fn visit_binary(&mut self, inner: &Binary) -> Self::Return {
        format!(
            "({} {} {})",
            inner.operator,
            self.print(inner.left.as_ref()),
            self.print(inner.right.as_ref())
        )
    }

    fn visit_grouping(&mut self, inner: &Grouping) -> Self::Return {
        format!("(group {})", self.print(inner.expr.as_ref()))
    }

    fn visit_var(&mut self, inner: &Var) -> Self::Return {
        format!("(var {})", inner.name)
    }

    fn visit_assign(&mut self, inner: &Assign) -> Self::Return {
        format!(
            "(= {} {})",
            self.print(inner.name.as_ref()),
            self.print(inner.value.as_ref())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::*;

    #[test]
    fn test_literal() {
        let mut printer = AstPrinter;
        let literal = Literal::String("print test".to_string());
        assert_eq!(
            r#""print test""#.to_string(),
            printer.print(&literal.into())
        );

        let literal = Literal::True;
        assert_eq!("true", printer.print(&literal.into()));

        let literal = Literal::Number(64.0);
        assert_eq!("64", printer.print(&literal.into()));
    }

    #[test]
    fn test_binary() {
        let mut printer = AstPrinter;
        let binary = Binary {
            operator: BinaryOperator::Plus,
            left: Box::new(Literal::Number(2.0).into()),
            right: Box::new(Literal::Number(3.0).into()),
        };
        assert_eq!("(+ 2 3)", printer.print(&binary.into()));
    }

    #[test]
    fn test_expr() {
        let mut printer = AstPrinter;
        let expr = Binary {
            operator: BinaryOperator::Mult,
            left: Box::new(
                Unary {
                    operator: UnaryOperator::Minus,
                    expr: Box::new(Literal::Number(123.0).into()),
                }
                .into(),
            ),
            right: Box::new(
                Grouping {
                    expr: Box::new(Literal::Number(45.67).into()),
                }
                .into(),
            ),
        };
        assert_eq!("(* (- 123) (group 45.67))", printer.print(&expr.into()));

        let expr = Binary {
            operator: BinaryOperator::Mult,
            left: Box::new(
                Grouping {
                    expr: Box::new(
                        Binary {
                            operator: BinaryOperator::Plus,
                            left: Box::new(Literal::Number(2.0).into()),
                            right: Box::new(Literal::Number(2.0).into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            right: Box::new(
                Grouping {
                    expr: Box::new(
                        Binary {
                            operator: BinaryOperator::Plus,
                            left: Box::new(Literal::Number(3.0).into()),
                            right: Box::new(
                                Unary {
                                    operator: UnaryOperator::Minus,
                                    expr: Box::new(Literal::Number(1.0).into()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        };
        assert_eq!(
            "(* (group (+ 2 2)) (group (+ 3 (- 1))))",
            printer.print(&expr.into())
        );
    }
}
