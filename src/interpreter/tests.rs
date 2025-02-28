use crate::{parser::Parser, scanner::Scanner, statement::Stmt};

use super::*;

impl Interpreter {
    fn get_var(&self, name: &str) -> &Value {
        self.env.get(name).unwrap()
    }
}

fn evaluate_expr(source: &'static str) -> Value {
    let mut int = Interpreter::default();
    let tokens = Scanner::new(source.into()).scan_source().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let Stmt::ExprStmt(ref expr) = stmts[0] else {
        panic!()
    };
    int.evaluate(&expr).unwrap()
}

fn interpret_stmts(source: &'static str, interpreter: &mut Interpreter) {
    let tokens = Scanner::new(source.into()).scan_source().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();

    for stmt in &stmts {
        interpreter.interpret(stmt).unwrap();
    }
}

#[test]
fn test_evaluate() {
    assert_eq!(evaluate_expr("2 + 3 * 5;"), Value::Number(17.0));
    assert_eq!(
        evaluate_expr(r#" "hello" * 3 ;"#),
        Value::String("hellohellohello".into())
    );
    assert_eq!(evaluate_expr("12 + 3 == 3 * 5;"), Value::Bool(true));
    assert_eq!(
        evaluate_expr(r#" "hello" * 3 == "hellohellohello" ;"#),
        Value::Bool(true)
    );
    assert_eq!(
        evaluate_expr("2 + 2 == 4 and true and 3 <= 4;"),
        Value::Bool(true)
    );
    assert_eq!(evaluate_expr(r#" "three" == 3 ;"#), Value::Bool(false));
}

#[test]
#[should_panic]
fn test_error() {
    evaluate_expr("true + false");
}

#[test]
fn test_int_expr_stmt() {
    let mut int = Interpreter::default();
    interpret_stmts("var x = 5 + 5; x = x + 1;", &mut int);
    assert_eq!(int.get_var("x"), &Value::Number(11.0));
}

#[test]
fn test_int_if_stmt() {
    let mut int = Interpreter::default();
    interpret_stmts(
        r#"
        var x = 5;
        if (x > 10)
            x = x * 2;
        else
            x = x * 3;
    "#,
        &mut int,
    );
    assert_eq!(int.get_var("x"), &Value::Number(15.0));
}

#[test]
fn test_int_while_stmt() {
    let mut int = Interpreter::default();
    interpret_stmts(
        r#"
        var x = 0;
        while (x < 10) {
            x = x + 1;
        }
    "#,
        &mut int,
    );
    assert_eq!(int.get_var("x"), &Value::Number(10.0));
}
