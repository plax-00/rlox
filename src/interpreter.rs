use anyhow::{anyhow, bail, Result};

use crate::{
    environment::Environment,
    expression::{Assign, Binary, Expression, ExpressionVisitor, Grouping, Literal, Unary, Var},
    operator::{BinaryOperator, UnaryOperator},
    statement::{BlockStmt, IfStmt, Stmt, StmtVisitor, VarDecl, WhileStmt},
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

    pub fn interpret(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(&mut *self)?;
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

        // short circuit logical operators
        match inner.operator {
            BinaryOperator::Or => {
                return left
                    .is_truthy()
                    .then_some(left)
                    .map_or_else(|| self.evaluate(&inner.right), Ok);
            }
            BinaryOperator::And => {
                return (!left.is_truthy())
                    .then_some(left)
                    .map_or_else(|| self.evaluate(&inner.right), Ok);
            }
            _ => (),
        }

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

    fn visit_block_stmt(&mut self, inner: &BlockStmt) -> Self::Return {
        self.env.push_scope();
        for stmt in &inner.stmts {
            self.interpret(stmt)?;
        }
        self.env.pop_scope();

        Ok(())
    }

    fn visit_if_stmt(&mut self, inner: &IfStmt) -> Self::Return {
        let condition = self.evaluate(inner.condition.as_ref())?;
        if condition.is_truthy() {
            self.interpret(inner.then_branch.as_ref())?;
        } else if let Some(s) = &inner.else_branch {
            self.interpret(s.as_ref())?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, inner: &WhileStmt) -> Self::Return {
        while self.evaluate(inner.condition.as_ref())?.is_truthy() {
            self.interpret(inner.body.as_ref())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
