use anyhow::{anyhow, bail, Result};

use crate::{
    environment::Environment,
    expression::{Assign, Binary, Expression, ExpressionVisitor, Grouping, Literal, Unary, Var},
    operator::{BinaryOperator, UnaryOperator},
    statement::{Stmt, StmtVisitor, VarDecl},
    value::Value,
};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expression) -> Result<Value> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            stmt.accept(&mut *self)?;
        }
        Ok(())
    }
}

impl ExpressionVisitor for Interpreter {
    type Return = Result<Value>;
    fn visit_literal(&mut self, inner: &Literal) -> Self::Return {
        let val = match *inner {
            Literal::String(ref s) => Value::String(s.clone()),
            Literal::Number(n) => Value::Number(n),
            Literal::True => Value::Bool(true),
            Literal::False => Value::Bool(false),
            Literal::Nil => Value::Nil,
        };
        Ok(val)
    }

    fn visit_unary(&mut self, inner: &Unary) -> Self::Return {
        match inner.operator {
            UnaryOperator::Minus => {
                let Value::Number(n) = self.evaluate(&inner.expr)? else {
                    bail!("Expected a number, found {:?}", inner.expr)
                };
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let b = self.evaluate(&inner.expr)?;
                Ok(Value::Bool(!b.is_truthy()))
            }
        }
    }

    fn visit_binary(&mut self, inner: &Binary) -> Self::Return {
        let left = self.evaluate(&inner.left)?;
        let right = self.evaluate(&inner.right)?;

        match inner.operator {
            BinaryOperator::Minus => Ok((left - right)?),
            BinaryOperator::Plus => Ok((left + right)?),
            BinaryOperator::Mult => Ok((left * right)?),
            BinaryOperator::Div => Ok((left / right)?),
            BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperator::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOperator::Less => Ok(Value::Bool(left < right)),
            BinaryOperator::LessEqual => Ok(Value::Bool(left <= right)),
            BinaryOperator::Greater => Ok(Value::Bool(left > right)),
            BinaryOperator::GreaterEqual => Ok(Value::Bool(left >= right)),
            BinaryOperator::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinaryOperator::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            op => bail!("Unexpected operator: {}", op),
        }
    }

    fn visit_grouping(&mut self, inner: &Grouping) -> Self::Return {
        self.evaluate(&inner.expr)
    }

    fn visit_var(&mut self, inner: &Var) -> Self::Return {
        self.env
            .get(&inner.name)
            .cloned()
            .ok_or_else(|| anyhow!("{} is not defined.", inner.name))
    }

    fn visit_assign(&mut self, inner: &Assign) -> Self::Return {
        let target = inner.name.as_ref();
        let Expression::Var(Var { name }) = target else {
            bail!("Cannot assign to {:?}", target)
        };
        let value = self.evaluate(&inner.value)?;
        self.env.assign(name, value)
    }
}

impl StmtVisitor for Interpreter {
    type Return = Result<()>;
    fn visit_expr_stmt(&mut self, inner: &Expression) -> Self::Return {
        self.evaluate(inner)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, inner: &Expression) -> Self::Return {
        println!("{}", self.evaluate(inner)?);
        Ok(())
    }

    fn visit_var_decl(&mut self, inner: &VarDecl) -> Self::Return {
        let value = match &inner.initializer {
            Some(e) => self.evaluate(e.as_ref())?,
            None => Value::Nil,
        };
        self.env.define(&inner.name, value);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner, statement::Stmt};

    use super::*;

    fn evaluate_source(source: &'static str) -> Value {
        let mut int: Interpreter = Interpreter::default();
        let tokens = Scanner::new(source.into()).scan_source().unwrap();
        let stmts = Parser::new(tokens).parse().unwrap();
        let Stmt::ExprStmt(ref expr) = stmts[0] else {
            panic!()
        };
        int.evaluate(&expr).unwrap()
    }

    #[test]
    fn test_interpret() {
        assert_eq!(evaluate_source("2 + 3 * 5"), Value::Number(17.0));
        assert_eq!(
            evaluate_source(r#" "hello" * 3 "#),
            Value::String("hellohellohello".into())
        );
        assert_eq!(evaluate_source("12 + 3 == 3 * 5"), Value::Bool(true));
        assert_eq!(
            evaluate_source(r#" "hello" * 3 == "hellohellohello" "#),
            Value::Bool(true)
        );
        assert_eq!(
            evaluate_source("2 + 2 == 4 and true and 3 <= 4"),
            Value::Bool(true)
        );
        assert_eq!(evaluate_source(r#" "three" == 3 "#), Value::Bool(false))
    }

    #[test]
    #[should_panic]
    fn test_error() {
        evaluate_source("true + false");
    }
}
