use crate::token::Token;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equal,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

impl Precedence {
    pub fn of(token: &Token) -> Precedence {
        match token {
            Token::Equal => Precedence::Equal,
            Token::NotEqual => Precedence::Equal,
            Token::LessThan => Precedence::LessGreater,
            Token::GreaterThan => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Asterisk => Precedence::Product,
            Token::Slash => Precedence::Product,
            Token::Percent => Precedence::Product,
            Token::LeftParenthesis => Precedence::Call,
            Token::LeftBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
