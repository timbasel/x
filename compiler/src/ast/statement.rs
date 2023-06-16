use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub position: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind {
    Comment(Comment),
    Declaration(Declaration),
    Assignment(Assignment),
    Block(Block),
    FunctionDeclaration(FunctionDeclaration),
    Return(Return),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub comment: String,
    pub inline: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub mutable: bool,
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Box<Statement>>,
}

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
