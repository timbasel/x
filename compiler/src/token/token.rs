mod position;
pub use position::*;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub position: Span,
}

impl TokenInfo {
    pub fn new() -> Self {
        return Self {
            token: Token::EOF,
            position: Span::new(),
        };
    }

    pub fn token(&self) -> Token {
        return self.token.clone();
    }

    pub fn position(&self) -> Span {
        return self.position.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    LineComment(String),
    BlockComment(String),

    /* Literals */
    /// name, main, value
    Identifier(String),
    /// 12345
    Integer(String),
    /// 3.14159265359
    Float(String),
    /// "Hello World"
    String(String),

    /* Operators */
    /// :=
    Declare,
    /// =
    Assign,
    /// .
    Dot,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// %
    Percent,
    /// !
    ExclamationMark,
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    LessThan,
    /// \>
    GreaterThan,

    /* Delimiters */
    /// ,
    Comma,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// (
    LeftParenthesis,
    /// )
    RightParenthesis,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// {
    LeftBrace,
    /// }
    RightBrace,

    /* Keywords */
    /// true
    True,
    /// false
    False,
    /// mut
    Mutable,
    /// fn
    Function,
    /// return
    Return,
}

impl Token {
    pub fn match_keyword(name: &str) -> Option<Token> {
        match name {
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "mut" => Some(Token::Mutable),
            "fn" => Some(Token::Function),
            "return" => Some(Token::Return),

            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Illegal => write!(f, "ILLEGAL"),
            EOF => write!(f, "ILLEGAL"),
            LineComment(comment) => write!(f, "//{}", comment),
            BlockComment(comment) => write!(f, "/*{}*/", comment),

            /* Literals */
            Identifier(name) => write!(f, "{}", name),
            Integer(integer) => write!(f, "{}", integer),
            Float(float) => write!(f, "{}", float),
            String(string) => write!(f, "\"{}\"", string),

            /* Operators */
            Declare => write!(f, ":="),
            Assign => write!(f, "="),
            Dot => write!(f, "."),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            Percent => write!(f, "%"),
            ExclamationMark => write!(f, "!"),
            Equal => write!(f, "=="),
            NotEqual => write!(f, "!="),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),

            /* Delimiters */
            Comma => write!(f, ","),
            Colon => write!(f, ":"),
            Semicolon => write!(f, ";"),
            LeftParenthesis => write!(f, "("),
            RightParenthesis => write!(f, ")"),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),

            /* Keywords */
            True => write!(f, "true"),
            False => write!(f, "false"),
            Mutable => write!(f, "mut"),
            Function => write!(f, "fn"),
            Return => write!(f, "return"),
        }
    }
}
