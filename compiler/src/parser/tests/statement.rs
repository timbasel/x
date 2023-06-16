use super::*;

#[test]
fn parse_comment_statement() {
    let cases = &[
        (
            "// Comment",
            ast::Statement {
                kind: Comment(ast::Comment {
                    comment: " Comment".into(),
                    inline: true,
                }),
                position: Span::from((0, 1, 0), (9, 1, 0)),
            },
        ),
        (
            "/* Comment */",
            ast::Statement {
                kind: Comment(ast::Comment {
                    comment: " Comment ".into(),
                    inline: false,
                }),
                position: Span::from((0, 1, 0), (12, 1, 0)),
            },
        ),
    ];

    assert_statement_equal(cases);
}

#[test]
fn parse_declaration_statement() {
    let cases = &[
        (
            r#"a := 1"#,
            ast::Statement {
                kind: Declaration(ast::Declaration {
                    name: "a".into(),
                    mutable: false,
                    value: Box::new(ast::Expression {
                        typ: ast::Type::Integer,
                        kind: Integer("1".into()),
                        position: Span::from((5, 1, 0), (5, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (5, 1, 0)),
            },
        ),
        (
            r#"mut b := "Hello""#,
            ast::Statement {
                kind: Declaration(ast::Declaration {
                    name: "b".into(),
                    mutable: true,
                    value: Box::new(ast::Expression {
                        typ: ast::Type::String,
                        kind: String("Hello".into()),
                        position: Span::from((9, 1, 0), (15, 1, 0)),
                    }),
                }),
                position: Span::from((0, 1, 0), (15, 1, 0)),
            },
        ),
    ];

    assert_statement_equal(cases);
}

#[test]
fn parse_assignment_statement() {
    let cases = &[(
        r#"a = 3.141"#,
        ast::Statement {
            kind: Assignment(ast::Assignment {
                name: "a".into(),
                value: Box::new(ast::Expression {
                    typ: ast::Type::Float,
                    kind: Float("3.141".into()),
                    position: Span::from((4, 1, 0), (8, 1, 0)),
                }),
            }),
            position: Span::from((0, 1, 0), (8, 1, 0)),
        },
    )];

    assert_statement_equal(cases);
}

#[test]
fn parse_block_statement() {
    let cases = &[(
        r#"
{ 
    1234
    "Hello"; "World"
}"#,
        ast::Statement {
            kind: Block(ast::Block {
                statements: vec![
                    Box::new(ast::Statement {
                        kind: Expression(ast::Expression {
                            typ: ast::Type::Integer,
                            kind: Integer("1234".into()),
                            position: Span::from((8, 3, 4), (11, 3, 4)),
                        }),
                        position: Span::from((8, 3, 4), (11, 3, 4)),
                    }),
                    Box::new(ast::Statement {
                        kind: Expression(ast::Expression {
                            typ: ast::Type::String,
                            kind: String("Hello".into()),
                            position: Span::from((17, 4, 13), (23, 4, 13)),
                        }),
                        position: Span::from((17, 4, 13), (24, 4, 13)),
                    }),
                    Box::new(ast::Statement {
                        kind: Expression(ast::Expression {
                            typ: ast::Type::String,
                            kind: String("World".into()),
                            position: Span::from((26, 4, 13), (32, 4, 13)),
                        }),
                        position: Span::from((26, 4, 13), (32, 4, 13)),
                    }),
                ],
            }),
            position: Span::from((1, 2, 1), (34, 5, 34)),
        },
    )];

    assert_statement_equal(cases);
}

#[test]
fn parse_function_declaration() {
    let cases = &[
        (
            r#"
fn main() {
  return 0
}
"#,
            ast::Statement {
                kind: FunctionDeclaration(ast::FunctionDeclaration {
                    name: "main".into(),
                    typ: ast::FunctionType {
                        parameters: vec![],
                        return_type: None,
                    },
                    body: ast::Block {
                        statements: vec![Box::new(ast::Statement {
                            kind: Return(ast::Return {
                                value: Box::new(ast::Expression {
                                    kind: Integer("0".into()),
                                    typ: Type::Integer,
                                    position: Span::from((22, 3, 13), (22, 3, 13)),
                                }),
                            }),
                            position: Span::from((15, 3, 13), (22, 3, 13)),
                        })],
                    },
                }),
                position: Span::from((1, 2, 1), (24, 4, 24)),
            },
        ),
        (
            r#"
fn foo(bar: string): int {
  return 42
}
"#,
            ast::Statement {
                kind: FunctionDeclaration(ast::FunctionDeclaration {
                    name: "foo".into(),
                    typ: ast::FunctionType {
                        parameters: vec![ast::Parameter {
                            name: "bar".into(),
                            typ: Type::String,
                        }],
                        return_type: Some(Box::new(Type::Integer)),
                    },
                    body: ast::Block {
                        statements: vec![Box::new(ast::Statement {
                            kind: Return(ast::Return {
                                value: Box::new(ast::Expression {
                                    kind: Integer("42".into()),
                                    typ: Type::Integer,
                                    position: Span::from((37, 3, 28), (38, 3, 28)),
                                }),
                            }),
                            position: Span::from((30, 3, 28), (38, 3, 28)),
                        })],
                    },
                }),
                position: Span::from((1, 2, 1), (40, 4, 40)),
            },
        ),
    ];

    assert_statement_equal(cases);
}
