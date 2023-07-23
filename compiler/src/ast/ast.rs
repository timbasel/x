mod expression;
mod function;
mod statement;
mod types;

pub use expression::*;
pub use function::*;
pub use statement::*;
pub use types::*;

use crate::token::{Span, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct File {
    pub statements: Vec<Box<Statement>>,
}

impl File {
    pub fn new() -> Self {
        return Self { statements: vec![] };
    }
}
