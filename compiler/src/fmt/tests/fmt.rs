use super::*;

mod types;

use pretty_assertions::assert_eq;

use crate::ast;
use crate::ast::Type;
use crate::parser::Parser;
use crate::token::Span;

#[test]
fn fmt_formatted_program() {
    let source = r#"// Comment
1 + 2
"Hello World"
a := 5 > 10
"#;

    let want = r#"// Comment
1 + 2
"Hello World"
a := 5 > 10
"#;

    assert_fmt_equal(source, want);
}

#[test]
fn fmt_unformatted_program() {
    let source = r#"
// Comment
1   +  2
"Hello World";
a  := 5>10
"#;

    let want = r#"// Comment
1 + 2
"Hello World"
a := 5 > 10
"#;

    assert_fmt_equal(source, want);
}

#[test]
fn fmt_verbose_program() {
    let source = r#"// Comment
((1 + 2) - 3)
mut a := true
"#;
    let want = r#"// Comment
1 + 2 - 3
mut a := true
"#;

    assert_fmt_equal(source, want);
}

fn assert_fmt_equal(source: &str, want: &str) {
    let mut parser = Parser::new(source);
    let file = parser.parse().unwrap();

    let got = format!("{}", file);

    assert_eq!(want, got);
}
