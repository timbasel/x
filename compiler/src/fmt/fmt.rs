use std::fmt;

mod expression;
mod function;
mod statement;
mod types;

use crate::ast::*;

pub trait Format {
    fn fmt(&self, fmt: &mut Formatter) -> String;
}

impl<T> Format for &T
where
    T: Format,
{
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return (*self).fmt(fmt);
    }
}

#[derive(Clone, Debug)]
pub struct Formatter {
    pub verbose: bool,

    indent_string: String,
    indent_level: usize,
}

impl Default for Formatter {
    fn default() -> Self {
        return Formatter {
            verbose: false,

            indent_string: "  ".into(),
            indent_level: 0,
        };
    }
}

impl Formatter {
    pub fn join<S: Format, T: std::iter::Iterator<Item = S>>(
        &mut self,
        strings: T,
        separator: &str,
    ) -> String {
        return strings
            .map(|s| s.fmt(self))
            .collect::<Vec<String>>()
            .join(separator);
    }

    pub fn indent(&self) -> String {
        return self.indent_string.repeat(self.indent_level);
    }

    pub fn push_indent(&mut self) {
        self.indent_level = self.indent_level.saturating_add(1);
    }

    pub fn pop_indent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
}

impl Format for File {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("{}\n", self.statements.fmt(fmt));
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = Formatter::default();
        write!(f, "{}", Format::fmt(self, &mut fmt))
    }
}

#[cfg(test)]
#[path = "tests/fmt.rs"]
mod tests;
