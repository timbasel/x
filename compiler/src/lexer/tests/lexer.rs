use super::*;

use pretty_assertions::assert_eq;

use Token::*;

#[test]
fn lex_comment() {
    let source = r#"
// Comment
/* Comment */
// /* Nested Comment */
/* // Nested Comment */
/*
  Comment
  /* Nested Comment */
*/
"#;

    let want = &[
        LineComment(" Comment".into()),
        BlockComment(" Comment ".into()),
        LineComment(" /* Nested Comment */".into()),
        BlockComment(" // Nested Comment ".into()),
        BlockComment("\n  Comment\n  /* Nested Comment */\n".into()),
    ];

    assert_tokens_equal(source, want);
}

#[test]
fn lex_arithmetic_expressions() {
    let source = r#"
1+2
3-4
5 * 6
7 / 8
9 == 9
10 != 11
5 < 10 > 6
true
!false
(1 + 2) / 3.141
"#;

    let want = &[
        // 1+2
        Integer("1".into()),
        Plus,
        Integer("2".into()),
        // 3-4
        Integer("3".into()),
        Minus,
        Integer("4".into()),
        // 5 * 6
        Integer("5".into()),
        Asterisk,
        Integer("6".into()),
        // 7 / 8
        Integer("7".into()),
        Slash,
        Integer("8".into()),
        // 9 == 9
        Integer("9".into()),
        Equal,
        Integer("9".into()),
        // 10 != 11
        Integer("10".into()),
        NotEqual,
        Integer("11".into()),
        // 5 < 10 > 6
        Integer("5".into()),
        LessThan,
        Integer("10".into()),
        GreaterThan,
        Integer("6".into()),
        // true
        True,
        // !false
        ExclamationMark,
        False,
        // (1 + 2) / 3.141
        LeftParenthesis,
        Integer("1".into()),
        Plus,
        Integer("2".into()),
        RightParenthesis,
        Slash,
        Float("3.141".into()),
    ];

    assert_tokens_equal(source, want);
}

#[test]
fn lex_statements() {
    let source = r#"
mut a := 1;
a = 2
"#;

    let want = &[
        // mut a := 1
        Mutable,
        Identifier("a".into()),
        Declare,
        Integer("1".into()),
        Semicolon,
        // a = 2
        Identifier("a".into()),
        Assign,
        Integer("2".into()),
    ];

    assert_tokens_equal(source, want);
}

#[test]
fn lex_functions() {
    let source = r#"
fn test(): string {
    return "Hello World"
}
"#;

    let want = &[
        // fn test(): int {
        Function,
        Identifier("test".into()),
        LeftParenthesis,
        RightParenthesis,
        Colon,
        Identifier("string".into()),
        LeftBrace,
        // return 42
        Return,
        String("Hello World".into()),
        // }
        RightBrace,
    ];

    assert_tokens_equal(source, want);
}

fn assert_tokens_equal(source: &str, want: &[Token]) {
    let mut lexer = Lexer::new(source);
    let mut got = vec![];
    while lexer.next().expect("failed to lex token") != EOF {
        got.push((lexer.token(), lexer.position()));
    }

    for (want, (got, got_pos)) in want.iter().zip(&got) {
        assert_eq!(want, got, "token not equal");
        let got_literal = &source[got_pos.start.offset..got_pos.end.offset + 1];
        assert_eq!(
            want.to_string(),
            got_literal,
            "token literal does not match source"
        )
    }
    assert_eq!(
        want.len(),
        got.len(),
        "number of tokens does not match expected"
    )
}
