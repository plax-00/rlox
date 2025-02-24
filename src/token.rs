use std::{fmt::Display, hash::Hash, sync::LazyLock, vec::IntoIter};

use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_num: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line_num: u32) -> Self {
        Token {
            token_type,
            line_num,
        }
    }

    pub fn lexeme(&self) -> String {
        self.token_type.lexeme()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.token_type)
    }
}

pub struct Tokens {
    iter: IntoIter<Token>,
}

impl From<Vec<Token>> for Tokens {
    fn from(tokens: Vec<Token>) -> Self {
        let iter = tokens.into_iter();
        Self { iter }
    }
}

impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl Eq for TokenType {}
impl Hash for TokenType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Self::Number(_) = self {
            panic!("Cannot hash float.");
        }
        self.hash(state)
    }
}

impl TokenType {
    pub fn lexeme(&self) -> String {
        match &self {
            TokenType::Identifier(inner) | TokenType::String(inner) => inner.clone(),
            TokenType::Number(inner) => format!("{}", inner),
            _ => LEXEMES.get(self).unwrap().to_string(),
        }
    }
}

pub static KEYWORDS: LazyLock<FxHashMap<&str, TokenType>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

static LEXEMES: LazyLock<FxHashMap<TokenType, &str>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        // Single-character tokens.
        (TokenType::LeftParen, "("),
        (TokenType::RightParen, ")"),
        (TokenType::LeftBrace, "{"),
        (TokenType::RightBrace, "}"),
        (TokenType::Comma, ","),
        (TokenType::Dot, "."),
        (TokenType::Minus, "-"),
        (TokenType::Plus, "+"),
        (TokenType::Semicolon, ";"),
        (TokenType::Slash, "/"),
        (TokenType::Star, "*"),
        // One or two character tokens.
        (TokenType::Bang, "!"),
        (TokenType::BangEqual, "!="),
        (TokenType::Equal, "="),
        (TokenType::EqualEqual, "=="),
        (TokenType::Greater, ">"),
        (TokenType::GreaterEqual, ">="),
        (TokenType::Less, "<"),
        (TokenType::LessEqual, "<="),
        // Keywords
        (TokenType::And, "and"),
        (TokenType::Class, "class"),
        (TokenType::Else, "else"),
        (TokenType::False, "false"),
        (TokenType::Fun, "fun"),
        (TokenType::For, "for"),
        (TokenType::If, "if"),
        (TokenType::Nil, "nil"),
        (TokenType::Or, "or"),
        (TokenType::Print, "print"),
        (TokenType::Return, "return"),
        (TokenType::Super, "super"),
        (TokenType::This, "this"),
        (TokenType::True, "true"),
        (TokenType::Var, "var"),
        (TokenType::While, "while"),
    ])
});
