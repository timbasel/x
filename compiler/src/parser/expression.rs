use super::*;

use crate::ast::{ExpressionKind::*, Type};

impl<'a> Parser<'a> {
    /// Parses an expression using the Pratt parsing algorithm
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression> {
        let mut expression = self.parse_prefix()?;

        // Continue until we reach a statement seperator or a lower precedence operator.
        while !self.is_statement_separator() && precedence < Precedence::of(&self.current.token) {
            expression = self.parse_infix(expression)?;
        }

        return Ok(expression);
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression> {
        let expression = match &self.current.token {
            Token::Identifier(id) => ast::Expression {
                kind: Identifier(id.into()),
                typ: Type::Unknown,
                position: self.current.position(),
            },
            Token::Integer(i) => ast::Expression {
                kind: Integer(i.into()),
                typ: Type::Integer,
                position: self.current.position(),
            },
            Token::Float(f) => ast::Expression {
                kind: Float(f.into()),
                typ: Type::Float,
                position: self.current.position(),
            },
            Token::String(s) => ast::Expression {
                kind: String(s.into()),
                typ: Type::String,
                position: self.current.position(),
            },
            b @ Token::True | b @ Token::False => ast::Expression {
                kind: Boolean(*b == Token::True),
                typ: Type::Boolean,
                position: self.current.position(),
            },
            Token::LeftParenthesis => return self.parse_grouped_expression(),
            Token::Minus | Token::ExclamationMark => return self.parse_prefix_expression(),

            _ => {
                return Err(Error::UnexpectedToken {
                    want: "start of an expression".into(),
                    got: self.current.token.to_string(),
                    position: self.current.position(),
                })
            }
        };

        self.next();
        return Ok(expression);
    }

    fn parse_infix(&mut self, left: ast::Expression) -> Result<ast::Expression> {
        return match self.current.token {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Percent
            | Token::Equal
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan => self.parse_infix_expression(left),

            _ => Ok(left),
        };
    }

    fn parse_grouped_expression(&mut self) -> Result<ast::Expression> {
        let mut position = self.current.position();

        self.consume(Token::LeftParenthesis)?;

        let mut expression = self.parse_expression(Precedence::Lowest)?;

        // TODO: parse tuple literal if there is a comma

        self.consume(Token::RightParenthesis)?;

        position.end = self.previous.position.end();
        expression.position = position;
        return Ok(expression);
    }

    fn parse_prefix_expression(&mut self) -> Result<ast::Expression> {
        let mut position = self.current.position();

        let operator = self.current.token.clone();
        self.next(); // advance past operator

        let right = Box::new(self.parse_expression(Precedence::Prefix)?);
        let typ = right.typ.clone();

        position.end = right.position.end();
        return Ok(ast::Expression {
            kind: Prefix(ast::PrefixExpression { operator, right }),
            typ,
            position,
        });
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Result<ast::Expression> {
        let mut position = left.position.clone();

        let operator = self.current.token.clone();
        self.next(); // advance past operator

        let right = self.parse_expression(Precedence::of(&operator))?;

        position.end = right.position.end();
        return Ok(ast::Expression {
            kind: Infix(ast::InfixExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }),
            typ: Type::Unknown,
            position,
        });
    }
}
