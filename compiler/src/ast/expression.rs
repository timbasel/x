use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub typ: Type,
    pub position: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    Identifier(String),
    Boolean(bool),
    Integer(String),
    Float(String),
    String(String),

    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}
