use insta::{assert_yaml_snapshot, with_settings};

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
    assert_eq!("(+ (+ 2 3) 4)", printer.print(&expr));

    let expr = parse_expr("-123 * (45.67)");
    assert_eq!("(* (- 123) (group 45.67))", printer.print(&expr));

    let expr = parse_expr("(2 + 2) * (3 + -1)");
    assert_eq!(
        "(* (group (+ 2 2)) (group (+ 3 (- 1))))",
        printer.print(&expr)
    );

    let expr = parse_expr(r#" "string" * (3 * 4) / (2 + 1 * 3)"#);
    assert_eq!(
        r#"(/ (* "string" (group (* 3 4))) (group (+ 2 (* 1 3))))"#,
        printer.print(&expr)
    );

    let expr = parse_expr("1 <= 2 * 3");
    assert_eq!("(<= 1 (* 2 3))", printer.print(&expr));

    let expr = parse_expr("!(-2 < 4)");
    assert_eq!("(! (group (< (- 2) 4)))", printer.print(&expr));

    let expr = parse_expr("!!true != !!false");
    assert_eq!("(!= (! (! true)) (! (! false)))", printer.print(&expr));

    let expr = parse_expr("4 == 2 - -2");
    assert_eq!("(== 4 (- 2 (- 2)))", printer.print(&expr));

    let expr = parse_expr("true and !false");
    assert_eq!("(and true (! false))", printer.print(&expr));

    let expr = parse_expr("2 + 2 == 4 and true and 3 <= 4");
    assert_eq!(
        "(and (and (== (+ 2 2) 4) true) (<= 3 4))",
        printer.print(&expr)
    );
}

macro_rules! snapshot_test {
    ($name:ident, $( $source:literal ),*) => {
        #[test]
        fn $name() {
            $(
                let source: &'static str = $source;
                with_settings!({ description => source }, {
                    assert_yaml_snapshot!(parse_stmts(source));
                });
            )*
        }
    };
}

macro_rules! should_panic {
    ($name:ident, $source:literal) => {
        #[test]
        #[should_panic]
        fn $name() {
            let source: &'static str = $source;
            parse_stmts(source);
        }
    };
}

snapshot_test!(test_parse_expr_stmt, "2 + 5 < 2 * 5 == true;");
snapshot_test!(
    test_parse_print_stmt,
    r#" print "Hello" + ", " + "world!"; "#
);
snapshot_test!(test_parse_var_decl, "var x = 12 / 2;");
snapshot_test!(
    test_parse_block_stmt,
    r#" {
    var x = 5;
    print x;
    x = x * 2;
    print x;
}"#
);
snapshot_test!(
    test_parse_if_stmt,
    r#"
    if (x < 0) {
        print "x is negative.";
    } else if (x > 0) {
        print "x is positive.";
    } else {
        print "x is 0.";
    }
"#
);

should_panic!(test_missing_semicolon, "print 5 + 5");
should_panic!(test_missing_var_name, "var 5 = 5;");
should_panic!(test_missing_var_assign, "var x = ;");
