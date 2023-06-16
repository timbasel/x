use core::fmt;

/// Position of a single character in the source.
/// `offset` and `line_offset` start at 0. `line` starts at 1.
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub line_offset: usize,
}

impl Position {
    pub fn new() -> Self {
        return Self {
            offset: 0,
            line: 1,
            line_offset: 0,
        };
    }

    pub fn from(p: (usize, usize, usize)) -> Self {
        return Self {
            offset: p.0,
            line: p.1,
            line_offset: p.2,
        };
    }

    //// Advances the position `offset` by the given amount. Does not effect `line` or `line_offset`
    pub fn forward(mut self, n: usize) -> Self {
        self.offset = self.offset.saturating_add(n);
        return self;
    }

    //// Decreases the position `offset` by the given amount. Does not effect `line` or `line_offset`
    pub fn back(mut self, n: usize) -> Self {
        self.offset = self.offset.saturating_sub(n);
        return self;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.offset - self.line_offset)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new() -> Self {
        return Self {
            start: Position::new(),
            end: Position::new(),
        };
    }

    pub fn from(s: (usize, usize, usize), e: (usize, usize, usize)) -> Self {
        return Self {
            start: Position::from(s),
            end: Position::from(e),
        };
    }

    pub fn start(&self) -> Position {
        return self.start.clone();
    }

    pub fn end(&self) -> Position {
        return self.end.clone();
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}
