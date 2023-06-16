use super::*;

use crate::ast::StatementKind::*;

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<ast::Statement> {
        let mut position = self.current.position();

        let kind = match &self.current.token {
            Token::LineComment(_) => Comment(self.parse_comment_statement()?),
            Token::BlockComment(_) => Comment(self.parse_comment_statement()?),
            Token::Mutable => Declaration(self.parse_declaration_statement(true)?),
            Token::LeftBrace => Block(self.parse_block_statement()?),
            Token::Function => FunctionDeclaration(self.parse_function_declaration()?),
            Token::Return => Return(self.parse_return_statement()?),
            _ => match &self.next.token {
                Token::Declare => Declaration(self.parse_declaration_statement(false)?),
                Token::Assign => Assignment(self.parse_assignment_statement()?),
                _ => Expression(self.parse_expression(Precedence::Lowest)?),
            },
        };

        self.consume_statement_separator()?;
        position.end = self.previous.position.end();
        return Ok(ast::Statement { kind, position });
    }

    fn parse_comment_statement(&mut self) -> Result<ast::Comment> {
        let comment = match self.current.token() {
            Token::LineComment(comment) => ast::Comment {
                comment,
                inline: true,
            },
            Token::BlockComment(comment) => ast::Comment {
                comment,
                inline: false,
            },
            _ => panic!("Statement is not a comment."),
        };

        self.next();
        return Ok(comment);
    }

    fn parse_declaration_statement(&mut self, mutable: bool) -> Result<ast::Declaration> {
        if mutable {
            self.consume(Token::Mutable)?;
        }

        let name = self.get_identifier_name()?;
        self.next();

        self.consume(Token::Declare)?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        return Ok(ast::Declaration {
            mutable,
            name,
            value,
        });
    }

    fn parse_assignment_statement(&mut self) -> Result<ast::Assignment> {
        let name = self.get_identifier_name()?;
        self.next();

        self.consume(Token::Assign)?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        return Ok(ast::Assignment { name, value });
    }

    fn parse_block_statement(&mut self) -> Result<ast::Block> {
        self.consume(Token::LeftBrace)?;

        let mut statements = vec![];
        while self.current.token != Token::RightBrace {
            statements.push(Box::new(self.parse_statement()?));
        }

        self.consume(Token::RightBrace)?;

        return Ok(ast::Block { statements });
    }

    pub(super) fn parse_function_declaration(&mut self) -> Result<ast::FunctionDeclaration> {
        self.consume(Token::Function)?;

        let name = self.get_identifier_name()?;
        self.next();

        self.consume(Token::LeftParenthesis)?;

        let mut parameters = vec![];
        while self.current.token != Token::RightParenthesis {
            parameters.push(self.parse_parameter()?)
        }

        self.consume(Token::RightParenthesis)?;

        let mut return_type = None;
        if self.current.token == Token::Colon {
            self.consume(Token::Colon)?;
            return_type = Some(Box::new(self.parse_type()?));
        }

        let block = self.parse_block_statement()?;

        return Ok(ast::FunctionDeclaration {
            name,
            typ: ast::FunctionType {
                parameters,
                return_type,
            },
            body: block,
        });
    }

    fn parse_return_statement(&mut self) -> Result<ast::Return> {
        self.consume(Token::Return)?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        return Ok(ast::Return { value });
    }
}
