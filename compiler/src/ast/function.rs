use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub typ: FunctionType,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
    pub value: Box<Expression>,
}