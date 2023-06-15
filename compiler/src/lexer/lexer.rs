mod cursor;
mod error;
use cursor::*;
pub use error::*;

use crate::token::{Span, Token};

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,

    token: Token,
    position: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        return Lexer {
            cursor: Cursor::new(source),

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

    pub fn next(&mut self) -> Result<Token> {
        self.skip_whitespace();

        self.position.start = self.cursor.position();
        self.token = match self.cursor.current() {
            '\0' => Token::EOF,
            ':' => match self.cursor.peek() {
                '=' => {
                    self.cursor.next();
                    Token::Declare
                }
                _ => Token::Colon,
            },
            '.' => Token::Dot,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => match self.cursor.peek() {
                '/' => return self.read_line_comment(),
                '*' => return self.read_block_comment(),
                _ => Token::Slash,
            },
            '%' => Token::Percent,
            '!' => match self.cursor.peek() {
                '=' => {
                    self.cursor.next();
                    Token::NotEqual
                }
                _ => Token::ExclamationMark,
            },
            '=' => match self.cursor.peek() {
                '=' => {
                    self.cursor.next();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,

            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            '"' => return self.read_string(),
            _ => {
                if is_letter(self.cursor.current()) {
                    return self.read_identifier();
                } else if is_digit(self.cursor.current()) {
                    return self.read_number();
                } else {
                    Token::Illegal
                }
            }
        };
        self.position.end = self.cursor.position();

        self.cursor.next();
        return Ok(self.token());
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let mut name = String::new();

        // first character must be a letter
        if is_letter(self.cursor.current()) {
            name.push(self.cursor.current());
            self.cursor.next();
        }

        // next characters can be letter or digit
        while is_letter(self.cursor.current()) || is_digit(self.cursor.current()) {
            name.push(self.cursor.current());
            self.cursor.next();
        }

        self.position.end = self.cursor.position().back(1);
        if let Some(keyword) = Token::match_keyword(&name) {
            self.token = keyword;
            return Ok(self.token());
        } else {
            self.token = Token::Identifier(name);
            return Ok(self.token());
        }
    }

    fn read_number(&mut self) -> Result<Token> {
        let mut number = String::new();

        while is_digit(self.cursor.current()) {
            number.push(self.cursor.current());
            self.cursor.next();
        }

        if self.cursor.current() == '.' {
            number.push(self.cursor.current());
            self.cursor.next();

            while is_digit(self.cursor.current()) {
                number.push(self.cursor.current());
                self.cursor.next();
            }

            self.token = Token::Float(number);
        } else {
            self.token = Token::Integer(number);
        }

        self.position.end = self.cursor.position().back(1);
        return Ok(self.token());
    }

    fn read_string(&mut self) -> Result<Token> {
        self.cursor.next(); // skip "

        let mut string = String::new();

        loop {
            match self.cursor.current() {
                '"' => break,
                '\0' => {
                    return Err(Error::UnexpectedEOF {
                        position: self.cursor.position(),
                    });
                }
                _ => {
                    string.push(self.cursor.current());
                    self.cursor.next();
                }
            }
        }
        self.position.end = self.cursor.position();
        self.token = Token::String(string);

        self.cursor.next(); // "
        return Ok(self.token());
    }

    fn read_line_comment(&mut self) -> Result<Token> {
        self.cursor.skip(2);

        let mut comment = String::new();

        loop {
            match self.cursor.current() {
                '\n' | '\0' => {
                    self.token = Token::LineComment(comment);
                    self.position.end = self.cursor.position().back(1);
                    return Ok(self.token());
                }
                _ => {
                    comment.push(self.cursor.current());
                    self.cursor.next();
                }
            }
        }
    }

    fn read_block_comment(&mut self) -> Result<Token> {
        self.cursor.skip(2);

        let mut comment = String::new();

        loop {
            match self.cursor.current() {
                '*' => {
                    if self.cursor.peek() == '/' {
                        self.cursor.skip(2);
                        self.token = Token::BlockComment(comment);
                        self.position.end = self.cursor.position().back(1);
                        return Ok(self.token());
                    } else {
                        comment.push(self.cursor.current());
                        self.cursor.next();
                    }
                }
                '/' => {
                    if self.cursor.peek() == '*' {
                        if let Token::BlockComment(block) = self.read_block_comment()? {
                            comment.push_str(&format!("/*{}*/", block));
                        } else {
                            panic!("Invalid `read_block_comment` return type");
                        }
                    } else {
                        comment.push(self.cursor.current());
                        self.cursor.next();
                    }
                }
                '\0' => {
                    return Err(Error::UnexpectedEOF {
                        position: self.cursor.position(),
                    });
                }
                _ => {
                    comment.push(self.cursor.current());
                    self.cursor.next();
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.cursor.current()) {
            self.cursor.next();
        }
    }
}

fn is_letter(c: char) -> bool {
    return c == '_' || c.is_alphabetic();
}

fn is_digit(c: char) -> bool {
    return '0' <= c && c <= '9';
}

fn is_whitespace(c: char) -> bool {
    return c.is_ascii_whitespace();
}

#[cfg(test)]
#[path = "tests/lexer.rs"]
mod tests;
