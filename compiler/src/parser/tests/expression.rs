use super::*;

#[test]
fn parse_literals() {
    let cases = &[
        (
            r#"name"#,
            ast::Expression {
                typ: ast::Type::Unknown,
                kind: Identifier("name".into()),
                position: Span::from((0, 1, 0), (3, 1, 0)),
            },
        ),
        (
            r#"12345"#,
            ast::Expression {
                typ: ast::Type::Integer,
                kind: Integer("12345".into()),
                position: Span::from((0, 1, 0), (4, 1, 0)),
            },
        ),
        (
            r#"3.14159265359"#,
            ast::Expression {
                typ: ast::Type::Float,
                kind: Float("3.14159265359".into()),
                position: Span::from((0, 1, 0), (12, 1, 0)),
            },
        ),
        (
            r#""Hello World""#,
            ast::Expression {
                typ: ast::Type::String,
                kind: String("Hello World".into()),
                position: Span::from((0, 1, 0), (12, 1, 0)),
            },
        ),
        (
            r#"true"#,
            ast::Expression {
                typ: ast::Type::Boolean,
                kind: Boolean(true),
                position: Span::from((0, 1, 0), (3, 1, 0)),
            },
        ),
        (
            r#"false"#,
            ast::Expression {
                typ: ast::Type::Boolean,
                kind: Boolean(false),
                position: Span::from((0, 1, 0), (4, 1, 0)),
            },
        ),
    ];

    assert_expression_equal(cases);
}

#[test]
fn parse_prefix_expression() {
    let cases = &[
        (
            r#"-5"#,
            ast::Expression {
                typ: ast::Type::Integer,
                kind: Prefix(ast::PrefixExpression {
                    operator: Token::Minus,
                    right: Box::new(ast::Expression {
                        typ: ast::Type::Integer,
                        kind: Integer("5".into()),
                        position: Span::from((1, 1, 0), (1, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (1, 1, 0)),
            },
        ),
        (
            r#"!true"#,
            ast::Expression {
                typ: ast::Type::Boolean,
                kind: Prefix(ast::PrefixExpression {
                    operator: Token::ExclamationMark,
                    right: Box::new(ast::Expression {
                        typ: ast::Type::Boolean,
                        kind: Boolean(true),
                        position: Span::from((1, 1, 0), (4, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (4, 1, 0)),
            },
        ),
    ];

    assert_expression_equal(cases);
}

#[test]
fn parse_infix_expression() {
    let cases = &[
        (
            r#"1 + 2"#,
            ast::Expression {
                typ: ast::Type::Unknown,
                kind: Infix(ast::InfixExpression {
                    left: Box::new(ast::Expression {
                        typ: ast::Type::Integer,
                        kind: Integer("1".into()),
                        position: Span::from((0, 1, 0), (0, 1, 0)),
                    }),
                    operator: Token::Plus,
                    right: Box::new(ast::Expression {
                        typ: ast::Type::Integer,
                        kind: Integer("2".into()),
                        position: Span::from((4, 1, 0), (4, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (4, 1, 0)),
            },
        ),
        (
            r#""Hello" != "World""#,
            ast::Expression {
                typ: ast::Type::Unknown,
                kind: Infix(ast::InfixExpression {
                    left: Box::new(ast::Expression {
                        typ: ast::Type::String,
                        kind: String("Hello".into()),
                        position: Span::from((0, 1, 0), (6, 1, 0)),
                    }),
                    operator: Token::NotEqual,
                    right: Box::new(ast::Expression {
                        typ: ast::Type::String,
                        kind: String("World".into()),
                        position: Span::from((11, 1, 0), (17, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (17, 1, 0)),
            },
        ),
        (
            r#"5 < 10 > 7"#,
            ast::Expression {
                typ: ast::Type::Unknown,
                kind: Infix(ast::InfixExpression {
                    left: Box::new(ast::Expression {
                        typ: ast::Type::Unknown,
                        kind: Infix(ast::InfixExpression {
                            left: Box::new(ast::Expression {
                                typ: ast::Type::Integer,
                                kind: Integer("5".into()),
                                position: Span::from((0, 1, 0), (0, 1, 0)),
                            }),
                            operator: Token::LessThan,
                            right: Box::new(ast::Expression {
                                typ: ast::Type::Integer,
                                kind: Integer("10".into()),
                                position: Span::from((4, 1, 0), (5, 1, 0)),
                            }),
                        }),
                        position: Span::from((0, 1, 0), (5, 1, 0)),
                    }),
                    operator: Token::GreaterThan,
                    right: Box::new(ast::Expression {
                        typ: ast::Type::Integer,
                        kind: Integer("7".into()),
                        position: Span::from((9, 1, 0), (9, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (9, 1, 0)),
            },
        ),
    ];

    assert_expression_equal(cases);
}
