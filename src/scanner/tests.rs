use super::*;

#[test]
fn scan_string_test() {
    let source = r#"
        "hello, world";
        "hello again!";
    "#;
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_source().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenType::String("hello, world".to_string()), 2),
            Token::new(TokenType::Semicolon, 2),
            Token::new(TokenType::String("hello again!".to_string()), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::EOF, 4),
        ]
    );
}

#[test]
fn scan_number_test() {
    let source = r#"
        12345;
        123.456;
    "#;
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_source().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenType::Number(12345.0), 2),
            Token::new(TokenType::Semicolon, 2),
            Token::new(TokenType::Number(123.456), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::EOF, 4),
        ]
    );
}

#[test]
fn scan_comment_test() {
    let source = r#"
        // This is a comment
        5 / 10;
    "#;
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_source().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenType::Number(5.0), 3),
            Token::new(TokenType::Slash, 3),
            Token::new(TokenType::Number(10.0), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::EOF, 4),
        ]
    );
}

#[test]
fn scan_identifier_test() {
    let source = r#"
        var test_var = "hello, world";
        print test_var;
    "#;
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_source().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenType::Var, 2),
            Token::new(TokenType::Identifier("test_var".to_string()), 2),
            Token::new(TokenType::Equal, 2),
            Token::new(TokenType::String("hello, world".to_string()), 2),
            Token::new(TokenType::Semicolon, 2),
            Token::new(TokenType::Print, 3),
            Token::new(TokenType::Identifier("test_var".to_string()), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::EOF, 4),
        ]
    );
}

#[test]
fn scanner_test() {
    let source = include_str!("scanner_test.lox");
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_source().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenType::Print, 2),
            Token::new(TokenType::String("Hello, world!".to_string()), 2),
            Token::new(TokenType::Semicolon, 2),
            Token::new(TokenType::Identifier("divide".to_string()), 3),
            Token::new(TokenType::Slash, 3),
            Token::new(TokenType::Identifier("me".to_string()), 3),
            Token::new(TokenType::Semicolon, 3),
            Token::new(TokenType::Minus, 4),
            Token::new(TokenType::Identifier("negateMe".to_string()), 4),
            Token::new(TokenType::Semicolon, 4),
            Token::new(TokenType::Identifier("lessThan".to_string()), 5),
            Token::new(TokenType::LessEqual, 5),
            Token::new(TokenType::Identifier("orEqual".to_string()), 5),
            Token::new(TokenType::Semicolon, 5),
            Token::new(TokenType::Identifier("greater".to_string()), 6),
            Token::new(TokenType::Greater, 6),
            Token::new(TokenType::Identifier("than".to_string()), 6),
            Token::new(TokenType::Semicolon, 6),
            Token::new(TokenType::Identifier("greaterThan".to_string()), 7),
            Token::new(TokenType::GreaterEqual, 7),
            Token::new(TokenType::Identifier("orEqual".to_string()), 7),
            Token::new(TokenType::Semicolon, 7),
            Token::new(TokenType::Number(123.0), 9),
            Token::new(TokenType::EqualEqual, 9),
            Token::new(TokenType::String("123".to_string()), 9),
            Token::new(TokenType::Semicolon, 9),
            Token::new(TokenType::Bang, 10),
            Token::new(TokenType::False, 10),
            Token::new(TokenType::Semicolon, 10),
            Token::new(TokenType::True, 11),
            Token::new(TokenType::And, 11),
            Token::new(TokenType::False, 11),
            Token::new(TokenType::Semicolon, 11),
            Token::new(TokenType::True, 12),
            Token::new(TokenType::Or, 12),
            Token::new(TokenType::False, 12),
            Token::new(TokenType::Semicolon, 12),
            Token::new(TokenType::If, 14),
            Token::new(TokenType::LeftParen, 14),
            Token::new(TokenType::Identifier("condition".to_string()), 14),
            Token::new(TokenType::RightParen, 14),
            Token::new(TokenType::LeftBrace, 14),
            Token::new(TokenType::Print, 15),
            Token::new(TokenType::String("yes".to_string()), 15),
            Token::new(TokenType::Semicolon, 15),
            Token::new(TokenType::RightBrace, 16),
            Token::new(TokenType::Else, 16),
            Token::new(TokenType::LeftBrace, 16),
            Token::new(TokenType::Print, 17),
            Token::new(TokenType::String("no".to_string()), 17),
            Token::new(TokenType::Semicolon, 17),
            Token::new(TokenType::RightBrace, 18),
            Token::new(TokenType::Fun, 20),
            Token::new(TokenType::Identifier("printSum".to_string()), 20),
            Token::new(TokenType::LeftParen, 20),
            Token::new(TokenType::Identifier("a".to_string()), 20),
            Token::new(TokenType::Comma, 20),
            Token::new(TokenType::Identifier("b".to_string()), 20),
            Token::new(TokenType::RightParen, 20),
            Token::new(TokenType::LeftBrace, 20),
            Token::new(TokenType::Print, 21),
            Token::new(TokenType::Identifier("a".to_string()), 21),
            Token::new(TokenType::Plus, 21),
            Token::new(TokenType::Identifier("b".to_string()), 21),
            Token::new(TokenType::Semicolon, 21),
            Token::new(TokenType::RightBrace, 22),
            Token::new(TokenType::Class, 24),
            Token::new(TokenType::Identifier("Breakfast".to_string()), 24),
            Token::new(TokenType::LeftBrace, 24),
            Token::new(TokenType::Identifier("cook".to_string()), 25),
            Token::new(TokenType::LeftParen, 25),
            Token::new(TokenType::RightParen, 25),
            Token::new(TokenType::LeftBrace, 25),
            Token::new(TokenType::Print, 26),
            Token::new(TokenType::String("Eggs a-fryin'!".to_string()), 26),
            Token::new(TokenType::Semicolon, 26),
            Token::new(TokenType::RightBrace, 27),
            Token::new(TokenType::Identifier("serve".to_string()), 29),
            Token::new(TokenType::LeftParen, 29),
            Token::new(TokenType::Identifier("who".to_string()), 29),
            Token::new(TokenType::RightParen, 29),
            Token::new(TokenType::LeftBrace, 29),
            Token::new(TokenType::Print, 30),
            Token::new(TokenType::String("Enjoy your breakfast, ".to_string()), 30),
            Token::new(TokenType::Plus, 30),
            Token::new(TokenType::Identifier("who".to_string()), 30),
            Token::new(TokenType::Plus, 30),
            Token::new(TokenType::String(".".to_string()), 30),
            Token::new(TokenType::Semicolon, 30),
            Token::new(TokenType::RightBrace, 31),
            Token::new(TokenType::RightBrace, 32),
            Token::new(TokenType::EOF, 32),
        ],
    );
}

#[test]
fn sytnax_error_test() {
    let source = r#"
        "
    "#;
    let scanner = Scanner::new(source.to_string());
    let errors = scanner.scan_source().err().unwrap();
    assert_eq!(errors, vec![SyntaxError { line_num: 2 }]);
}
