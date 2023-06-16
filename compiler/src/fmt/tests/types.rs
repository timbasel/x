use super::*;

use pretty_assertions::assert_eq;

#[test]
fn fmt_type() {
    let cases = &[
        (
            Type::Tuple(ast::TupleType {
                elements: vec![Type::String, Type::Boolean],
            }),
            "(string, bool)",
        ),
        (
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
            "{ foo: string, bar: bool = false }",
        ),
        (
            Type::Function(ast::FunctionType {
                parameters: vec![ast::Parameter {
                    name: "foo".into(),
                    typ: Type::String,
                }],
                return_type: None,
            }),
            "fn (foo: string)",
        ),
        (
            Type::Function(ast::FunctionType {
                parameters: vec![ast::Parameter {
                    name: "bar".into(),
                    typ: Type::Integer,
                }],
                return_type: Some(Box::new(Type::Tuple(ast::TupleType {
                    elements: vec![Type::String, Type::Boolean],
                }))),
            }),
            "fn (bar: int): (string, bool)",
        ),
    ];

    for case in cases {
        let want = case.1;
        let got = case.0.fmt(&mut Formatter::default());

        assert_eq!(want, got)
    }
}
