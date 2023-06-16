use crate::{lexer, token::Span};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    LexerError(lexer::Error),
    UnexpectedToken {
        want: String,
        got: String,
        position: Span,
    },
    UnexpectedEOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexerError(e) => write!(f, "lexer error: {}", e),
            Error::UnexpectedToken {
                want,
                got,
                position,
            } => write!(
                f,
                "unexpected token: {} @ {} (expected: {})",
                got, position, want
            ),
            Error::UnexpectedEOF => write!(f, "unexpected EOF"),
        }
    }
}
