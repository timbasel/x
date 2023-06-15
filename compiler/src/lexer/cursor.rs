use std::str::Chars;

use crate::token::Position;

const EOF: char = '\0';

/// A peekable iterator over a source `&str` that tracks its position
#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    chars: Chars<'a>,

    current: char,
    position: Position,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut cursor = Self {
            chars: source.chars(),
            current: EOF,
            position: Position::new(),
        };
        cursor.next();
        return cursor;
    }

    pub fn current(&self) -> char {
        return self.current;
    }

    pub fn position(&self) -> Position {
        let mut position = self.position.clone();
        position.offset -= self.current.len_utf8(); // get offset at start of character
        return position;
    }

    /// Advances the cursor and returns the next `char`.
    pub fn next(&mut self) -> char {
        if self.current == '\n' {
            self.position.line += 1;
            self.position.line_offset = self.position.offset;
        }

        self.position.offset += self.current.len_utf8();
        self.current = self.chars.next().unwrap_or(EOF);

        return self.current;
    }

    /// Advances the cursor by `n` steps without returning the consumed characters.
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    /// Returns the next `char` without advancing the cursor.
    pub fn peek(&mut self) -> char {
        return self.peek_nth(0);
    }

    /// Returns the nth `char` without advancing the cursor.
    pub fn peek_nth(&mut self, n: usize) -> char {
        return self.chars.clone().nth(n).unwrap_or(EOF);
    }
}
