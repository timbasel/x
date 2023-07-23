use super::*;

impl<'a> Parser<'a> {
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

    pub(super) fn parse_return_statement(&mut self) -> Result<ast::Return> {
        self.consume(Token::Return)?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        return Ok(ast::Return { value });
    }
}