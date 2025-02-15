use std::sync::LazyLock;

use rustc_hash::FxHashMap;

use crate::expression::{Binary, ExprAccept, ExprVisitor, Grouping, Literal, Operator, Unary};

struct AstPrinter;
impl AstPrinter {
    fn print<E: ExprAccept>(&self, expr: &E) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor for &AstPrinter {
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

    fn visit_unary<E: ExprAccept>(&self, unary: &Unary<E>) -> Self::Return {
        format!("({:?} {})", unary.operator, self.print(unary.expr.as_ref()))
    }

    fn visit_binary<L: ExprAccept, R: ExprAccept>(&self, binary: &Binary<L, R>) -> Self::Return {
        format!(
            "({:?} {} {})",
            binary.operator,
            self.print(binary.left.as_ref()),
            self.print(binary.right.as_ref())
        )
    }

    fn visit_grouping<E: ExprAccept>(&self, grouping: &Grouping<E>) -> Self::Return {
        format!("(group {})", self.print(grouping.expr.as_ref()))
    }
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = *OPERATORS.get(self).expect("Operator should be in hash map");
        write!(f, "{}", repr)
    }
}

static OPERATORS: LazyLock<FxHashMap<Operator, &str>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (Operator::Minus, "-"),
        (Operator::Plus, "+"),
        (Operator::Div, "/"),
        (Operator::Mult, "*"),
        (Operator::Not, "!"),
        (Operator::NotEqual, "!="),
        (Operator::Equal, "="),
        (Operator::EqualEqual, "=="),
        (Operator::Greater, ">"),
        (Operator::GreaterEqual, ">="),
        (Operator::Less, "<"),
        (Operator::LessEqual, "<="),
    ])
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_test() {
        let printer = AstPrinter;
        let literal = Literal::String("print test".to_string());
        assert_eq!(r#""print test""#.to_string(), printer.print(&literal));

        let literal = Literal::True;
        assert_eq!("true", printer.print(&literal));

        let literal = Literal::Number(64.0);
        assert_eq!("64", printer.print(&literal));
    }

    #[test]
    fn binary_test() {
        let printer = AstPrinter;
        let binary = Binary {
            operator: Operator::Plus,
            left: Box::new(Literal::Number(2.0)),
            right: Box::new(Literal::Number(3.0)),
        };
        assert_eq!("(+ 2 3)", printer.print(&binary));
    }

    #[test]
    fn expr_test() {
        let printer = AstPrinter;
        let expr = Binary {
            operator: Operator::Mult,
            left: Box::new(Unary {
                operator: Operator::Minus,
                expr: Box::new(Literal::Number(123.0)),
            }),
            right: Box::new(Grouping {
                expr: Box::new(Literal::Number(45.67)),
            }),
        };
        assert_eq!("(* (- 123) (group 45.67))", printer.print(&expr));

        let expr = Binary {
            operator: Operator::Mult,
            left: Box::new(Grouping {
                expr: Box::new(Binary {
                    operator: Operator::Plus,
                    left: Box::new(Literal::Number(2.0)),
                    right: Box::new(Literal::Number(2.0)),
                }),
            }),
            right: Box::new(Grouping {
                expr: Box::new(Binary {
                    operator: Operator::Plus,
                    left: Box::new(Literal::Number(3.0)),
                    right: Box::new(Unary {
                        operator: Operator::Minus,
                        expr: Box::new(Literal::Number(1.0)),
                    }),
                })
            })
        };
        assert_eq!("(* (group (+ 2 2)) (group (+ 3 (- 1))))", printer.print(&expr));
    }
}
