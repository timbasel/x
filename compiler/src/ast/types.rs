use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Unknown,
    Name(String),

    Boolean,
    Integer,
    Float,
    String,

    Tuple(TupleType),
    Struct(StructType),
    Function(FunctionType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TupleType {
    pub elements: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructType {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub typ: Type,
    pub default: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<Type>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
}
