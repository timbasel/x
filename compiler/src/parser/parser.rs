mod error;
mod precedence;

use error::*;
use precedence::*;

mod expression;
mod statement;
mod types;

use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenInfo};

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,

    pub(super) previous: TokenInfo,
    pub(super) current: TokenInfo,
    pub(super) next: TokenInfo,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source),

            previous: TokenInfo::new(),
            current: TokenInfo::new(),
            next: TokenInfo::new(),
        };

        parser.next();
        parser.next();

        return parser;
    }

    pub fn parse(&mut self) -> Result<ast::File> {
        let mut file = ast::File::new();
        while self.current.token != Token::EOF {
            file.statements.push(Box::new(self.parse_statement()?))
        }
        return Ok(file);
    }

    pub(super) fn next(&mut self) {
        std::mem::swap(&mut self.current, &mut self.previous);
        std::mem::swap(&mut self.next, &mut self.current);
        self.next = self.lexer.next().unwrap_or(TokenInfo::new())
    }

    pub(super) fn expect(&mut self, token: Token) -> Result<()> {
        if self.current.token == token {
            return Ok(());
        } else {
            return Err(Error::UnexpectedToken {
                want: token.to_string(),
                got: self.current.token.to_string(),
                position: self.current.position(),
            });
        }
    }

    pub(super) fn consume(&mut self, token: Token) -> Result<()> {
        self.expect(token)?;
        self.next();
        return Ok(());
    }

    pub(super) fn is_statement_separator(&mut self) -> bool {
        if self.current.token == Token::Semicolon {
            return true;
        } else if self.on_new_line() || self.current.token == Token::EOF {
            return true;
        } else {
            return false;
        }
    }

    pub(super) fn consume_statement_separator(&mut self) -> Result<()> {
        if self.is_statement_separator() {
            if self.current.token == Token::Semicolon {
                self.next();
            }
            return Ok(());
        } else {
            return Err(Error::UnexpectedToken {
                want: "statement separator".into(),
                got: self.current.token.to_string(),
                position: self.current.position(),
            });
        }
    }

    pub(super) fn is_expression_separator(&self) -> bool {
        if self.current.token == Token::Comma {
            return true;
        } else if self.on_new_line() {
            return true;
        }
        return false;
    }

    pub(super) fn consume_expression_separator(&mut self) -> Result<()> {
        if self.current.token == Token::Comma {
            self.next();
            return Ok(());
        } else if self.on_new_line() || self.current.token == Token::EOF {
            return Ok(());
        }

        return Err(Error::UnexpectedToken {
            want: "expression separator".into(),
            got: format!("{}", self.current.token()),
            position: self.current.position(),
        });
    }

    fn on_new_line(&self) -> bool {
        return self.current.position.start.line > self.previous.position.end.line;
    }

    pub(super) fn get_identifier_name(&self) -> Result<String> {
        if let Token::Identifier(identifier) = &self.current.token {
            return Ok(identifier.into());
        } else {
            return Err(Error::UnexpectedToken {
                want: "identifier".into(),
                got: self.current.token.to_string(),
                position: self.current.position(),
            });
        }
    }
}

#[cfg(test)]
#[path = "tests/parser.rs"]
mod tests;
