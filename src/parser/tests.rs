use insta::assert_yaml_snapshot;

use super::*;
use crate::{ast_print::AstPrinter, scanner::Scanner};

fn parse_expr(source: &'static str) -> Expression {
    let tokens = Scanner::new(source.to_string()).scan_source().unwrap();
    Parser::new(tokens).parse_expr().unwrap()
}

fn parse_stmts(source: &'static str) -> Vec<Stmt> {
    let tokens = Scanner::new(source.to_string()).scan_source().unwrap();
    Parser::new(tokens).parse().unwrap()
}

#[test]
fn test_parse_expr() {
    let mut printer = AstPrinter;
    let expr = parse_expr("2 + 3 + 4");
    assert_eq!("(+ (+ 2 3) 4)", printer.print_expr(&expr));

    let expr = parse_expr("-123 * (45.67)");
    assert_eq!("(* (- 123) (group 45.67))", printer.print_expr(&expr));

    let expr = parse_expr("(2 + 2) * (3 + -1)");
    assert_eq!(
        "(* (group (+ 2 2)) (group (+ 3 (- 1))))",
        printer.print_expr(&expr)
    );

    let expr = parse_expr(r#" "string" * (3 * 4) / (2 + 1 * 3)"#);
    assert_eq!(
        r#"(/ (* "string" (group (* 3 4))) (group (+ 2 (* 1 3))))"#,
        printer.print_expr(&expr)
    );

    let expr = parse_expr("1 <= 2 * 3");
    assert_eq!("(<= 1 (* 2 3))", printer.print_expr(&expr));

    let expr = parse_expr("!(-2 < 4)");
    assert_eq!("(! (group (< (- 2) 4)))", printer.print_expr(&expr));

    let expr = parse_expr("!!true != !!false");
    assert_eq!("(!= (! (! true)) (! (! false)))", printer.print_expr(&expr));

    let expr = parse_expr("4 == 2 - -2");
    assert_eq!("(== 4 (- 2 (- 2)))", printer.print_expr(&expr));

    let expr = parse_expr("true and !false");
    assert_eq!("(and true (! false))", printer.print_expr(&expr));

    let expr = parse_expr("2 + 2 == 4 and true and 3 <= 4");
    assert_eq!(
        "(and (and (== (+ 2 2) 4) true) (<= 3 4))",
        printer.print_expr(&expr)
    );
}

#[test]
fn test_parse_stmt() {
    let stmts = parse_stmts("print 5 < 10 and 4 >= 2; print false;");
    for stmt in stmts {
        assert_yaml_snapshot!(stmt);
    }
}
