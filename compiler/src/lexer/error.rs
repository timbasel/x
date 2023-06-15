use std::fmt;

use crate::token::Position;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    UnexpectedEOF { position: Position },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            UnexpectedEOF { position } => write!(f, "Unexpected EOF @ {}", position),
        }
    }
}
