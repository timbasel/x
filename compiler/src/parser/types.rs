use super::*;

use ast::Type;

impl<'a> Parser<'a> {
    pub(super) fn parse_type(&mut self) -> Result<ast::Type> {
        let typ = match &self.current.token {
            Token::Identifier(_) => self.parse_identifier_type()?,
            Token::LeftParenthesis => Type::Tuple(self.parse_tuple_type()?),
            Token::LeftBrace => Type::Struct(self.parse_struct_type()?),
            Token::Function => Type::Function(self.parse_function_type()?),

            _ => {
                return Err(Error::UnexpectedToken {
                    want: "typ".to_string(),
                    got: format!("{}", self.current.token()),
                    position: self.current.position(),
                })
            }
        };

        return Ok(typ);
    }

    fn parse_identifier_type(&mut self) -> Result<ast::Type> {
        let identifier = self.get_identifier_name()?;

        let typ = match identifier.as_str() {
            "bool" => Type::Boolean,
            "int" => Type::Integer,
            "float" => Type::Float,
            "string" => Type::String,
            _ => Type::Name(identifier.into()),
        };

        self.next();

        return Ok(typ);
    }

    fn parse_tuple_type(&mut self) -> Result<ast::TupleType> {
        self.consume(Token::LeftParenthesis)?;

        let mut elements = vec![];
        while self.current.token != Token::RightParenthesis {
            elements.push(self.parse_type()?);

            if self.current.token == Token::RightParenthesis {
                break;
            } else {
                self.consume_expression_separator()?;
            }
        }

        self.consume(Token::RightParenthesis)?;

        return Ok(ast::TupleType { elements });
    }

    fn parse_struct_type(&mut self) -> Result<ast::StructType> {
        self.consume(Token::LeftBrace)?;

        let mut fields = vec![];
        while self.current.token != Token::RightBrace {
            fields.push(self.parse_field()?);

            if self.current.token == Token::RightBrace {
                break;
            } else {
                self.consume_expression_separator()?;
            }
        }

        self.consume(Token::RightBrace)?;

        return Ok(ast::StructType { fields });
    }

    fn parse_field(&mut self) -> Result<ast::Field> {
        let name = self.get_identifier_name()?;
        self.next();

        self.consume(Token::Colon)?;

        let typ = self.parse_type()?;

        let mut default = None;
        if self.current.token == Token::Assign {
            self.next();

            default = Some(Box::new(self.parse_expression(Precedence::Lowest)?));
        }

        return Ok(ast::Field { name, typ, default });
    }

    fn parse_function_type(&mut self) -> Result<ast::FunctionType> {
        self.consume(Token::Function)?;

        self.consume(Token::LeftParenthesis)?;

        let mut parameters = vec![];
        while self.current.token != Token::RightParenthesis {
            parameters.push(self.parse_parameter()?);

            if self.current.token == Token::RightParenthesis {
                break;
            } else {
                self.consume_expression_separator()?;
            }
        }

        self.consume(Token::RightParenthesis)?;

        let mut return_type = None;
        if self.current.token == Token::Colon {
            self.next();

            return_type = Some(Box::new(self.parse_type()?));
        }

        return Ok(ast::FunctionType {
            parameters,
            return_type,
        });
    }

    pub(super) fn parse_parameter(&mut self) -> Result<ast::Parameter> {
        let name = self.get_identifier_name()?;
        self.next();

        self.consume(Token::Colon)?;

        let typ = self.parse_type()?;

        return Ok(ast::Parameter { name, typ });
    }
}
