use crate::expression::{Binary, Expr, ExprVisitor, Grouping, Literal, Unary};

struct AstPrinter;
impl ExprVisitor<String> for AstPrinter {
    fn visit_literal(&self, expr: &Literal) -> String {
        let repr = match expr {
            Literal::Number(n) => &format!("{}", n),
            Literal::String(s) => &format!(r#""{}""#, s),
            Literal::True => "true",
            Literal::False => "false",
            Literal::Nil => "nil",
        };
        format!("{}", repr)
    }

    fn visit_unary<E: Expr<String>>(&self, expr: &Unary<E, String>) -> String {
        format!("({:?} {})", expr.operator, expr.expr.accept(AstPrinter))
    }

    fn visit_binary<L: Expr<String>, R: Expr<String>>(&self, expr: &Binary<L, R, String>) -> String {
        let _ = expr;
        todo!()
    }

    fn visit_grouping<E: Expr<String>>(&self, expr: &Grouping<E, String>) -> String {
        let _ = expr;
        todo!()
    }
}

// pub trait AstPrint: Expr {
//     fn print(&self) -> String;
// }
//
// impl AstPrint for Literal {
//     fn print(&self) -> String {
//         let repr = match self {
//             Self::Number(n) => &format!("{}", n),
//             Self::String(s) => &format!(r#""{}""#, s),
//             Self::True => "true",
//             Self::False => "false",
//             Self::Nil => "nil",
//         };
//         format!("{}", repr)
//     }
// }
//
// impl<T: Expr> AstPrint for Unary<T> {
//     fn print(&self) -> String {
//         format!("({:?} {})", self.operator, self.expr.print())
//     }
// }

// trait AstPrint: ExprVisitor<String> {
//     fn print<E: Expr>(expr)
// }

// pub struct AstPrinter;
// impl ExprVisitor<Literal, String> for AstPrinter {
//     fn visit(expr: &Literal) -> String {
//         let _ = expr;
//         format!("Hello world")
//     }
// }

// impl AstPrinter {
//     fn print<T: AstPrint>(expression: &T) -> String {
//         format!("({})", expression.print())
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::*;

    #[test]
    fn print_test() {
        let literal = Literal::String("Hello world".to_string());
        assert_eq!(r#"("print test")"#.to_string(), literal.accept(AstPrinter));

        // let literal = Literal::True;
        // assert_eq!(r#"(True)"#, AstPrinter::print(&literal));
    }
}
