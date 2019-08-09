#[macro_use]
extern crate lalrpop_util;
use crate::ast::Expr;

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_boolean_true() {
    let program = grammar::UntypedExprParser::new().parse("true").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "true".to_string()
        }
    );
}

#[test]
fn test_boolean_false() {
    let program = grammar::UntypedExprParser::new().parse("false").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "false".to_string()
        }
    );
}

#[test]
fn test_integer() {
    let program = grammar::UntypedExprParser::new().parse("124").unwrap();
    assert_eq!(program, Expr::Int { value: 124 });
}

#[test]
fn test_integer_underscore() {
    let program = grammar::UntypedExprParser::new().parse("123_456").unwrap();
    assert_eq!(program, Expr::Int { value: 123456 });
}

#[test]
fn test_float() {
    let program = grammar::UntypedExprParser::new().parse("124.123").unwrap();
    assert_eq!(program, Expr::Float { value: 124.123 });
}

#[test]
fn test_atom_colon() {
    let program = grammar::UntypedExprParser::new().parse(":atom").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "atom".to_string()
        }
    );
}

#[test]
fn test_atom_mixed_case_colon() {
    let program = grammar::UntypedExprParser::new().parse(":AtOm").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "AtOm".to_string()
        }
    );
}

#[test]
fn test_atom_with_spaces_colon() {
    let program = grammar::UntypedExprParser::new()
        .parse(":\"i am an atom\"")
        .unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "i am an atom".to_string()
        }
    );
}

#[test]
fn test_atom_literal() {
    let program = grammar::UntypedExprParser::new()
        .parse("defmodule")
        .unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "defmodule".to_string()
        }
    );
}

#[test]
fn test_string() {
    let program = grammar::UntypedExprParser::new()
        .parse("\"Hello, world!\"")
        .unwrap();
    assert_eq!(
        program,
        Expr::String {
            value: "Hello, world!".to_string()
        }
    );
}
