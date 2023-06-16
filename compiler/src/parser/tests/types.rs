use super::*;

use ast::Type;

#[test]
fn parse_tuple_type() {
    let cases = &[
        (
            "(string, bool)",
            Type::Tuple(ast::TupleType {
                elements: vec![Type::String, Type::Boolean],
            }),
        ),
        (
            "{ foo: string, bar: bool = false }",
            Type::Struct(ast::StructType {
                fields: vec![
                    ast::Field {
                        name: "foo".into(),
                        typ: Type::String,
                        default: None,
                    },
                    ast::Field {
                        name: "bar".into(),
                        typ: Type::Boolean,
                        default: Some(Box::new(ast::Expression {
                            typ: Type::Boolean,
                            kind: ast::ExpressionKind::Boolean(false),
                            position: Span::from((27, 1, 0), (31, 1, 0)),
                        })),
                    },
                ],
            }),
        ),
        (
            "fn (foo: string)",
            Type::Function(ast::FunctionType {
                parameters: vec![ast::Parameter {
                    name: "foo".into(),
                    typ: Type::String,
                }],
                return_type: None,
            }),
        ),
        (
            "fn (bar: int): (string, bool)",
            Type::Function(ast::FunctionType {
                parameters: vec![ast::Parameter {
                    name: "bar".into(),
                    typ: Type::Integer,
                }],
                return_type: Some(Box::new(Type::Tuple(ast::TupleType {
                    elements: vec![Type::String, Type::Boolean],
                }))),
            }),
        ),
    ];

    assert_type_equal(cases);
}
