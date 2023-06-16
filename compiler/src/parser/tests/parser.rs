use super::*;

use pretty_assertions::assert_eq;

use crate::ast::ExpressionKind::*;
use crate::ast::StatementKind::*;
use crate::ast::Type;
use crate::token::Span;

mod expression;
mod statement;
mod types;

fn assert_expression_equal(cases: &[(&str, ast::Expression)]) {
    for case in cases {
        let mut parser = Parser::new(case.0);
        let got = parser.parse_expression(Precedence::Lowest).unwrap();

        assert_eq!(case.1, got);
    }
}

fn assert_statement_equal(cases: &[(&str, ast::Statement)]) {
    for case in cases {
        let mut parser = Parser::new(case.0);
        let got = parser.parse_statement().unwrap();

        assert_eq!(case.1, got);
    }
}

fn assert_type_equal(cases: &[(&str, ast::Type)]) {
    for case in cases {
        let mut parser = Parser::new(case.0);
        let got = parser.parse_type().unwrap();

        assert_eq!(case.1, got);
    }
}
