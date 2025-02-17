use crate::expression::{Binary, ExprVisitor, Expression, Grouping, Literal, Unary};

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&self, expr: &Expression) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor for AstPrinter {
    type Return = String;
    fn visit_literal(&self, literal: &Literal) -> Self::Return {
        let repr = match literal {
            Literal::Number(n) => &format!("{}", n),
            Literal::String(s) => &format!(r#""{}""#, s),
            Literal::True => "true",
            Literal::False => "false",
            Literal::Nil => "nil",
        };
        format!("{}", repr)
    }

    fn visit_unary(&self, unary: &Unary) -> Self::Return {
        format!("({} {})", unary.operator, self.print(unary.expr.as_ref()))
    }

    fn visit_binary(&self, binary: &Binary) -> Self::Return {
        format!(
            "({} {} {})",
            binary.operator,
            self.print(binary.left.as_ref()),
            self.print(binary.right.as_ref())
        )
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Self::Return {
        format!("(group {})", self.print(grouping.expr.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::*;

    #[test]
    fn literal_test() {
        let printer = AstPrinter;
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
    fn binary_test() {
        let printer = AstPrinter;
        let binary = Binary {
            operator: BinaryOperator::Plus,
            left: Box::new(Literal::Number(2.0).into()),
            right: Box::new(Literal::Number(3.0).into()),
        };
        assert_eq!("(+ 2 3)", printer.print(&binary.into()));
    }

    #[test]
    fn expr_test() {
        let printer = AstPrinter;
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
