#[macro_use]
extern crate lalrpop_util;
use crate::ast::{Expr, Meta};

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_boolean_true() {
    let program = grammar::UntypedExprParser::new().parse("true").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "true".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 4 }
        }
    );
}

#[test]
fn test_boolean_false() {
    let program = grammar::UntypedExprParser::new().parse("false").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "false".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 5 }
        }
    );
}

#[test]
fn test_integer() {
    let program = grammar::UntypedExprParser::new().parse("124").unwrap();
    assert_eq!(
        program,
        Expr::Int {
            value: 124,
            typ: (),
            meta: Meta { start: 0, end: 3 }
        }
    );
}

#[test]
fn test_integer_underscore() {
    let program = grammar::UntypedExprParser::new().parse("123_456").unwrap();
    assert_eq!(
        program,
        Expr::Int {
            value: 123456,
            typ: (),
            meta: Meta { start: 0, end: 7 }
        }
    );
}

#[test]
fn test_float() {
    let program = grammar::UntypedExprParser::new().parse("124.123").unwrap();
    assert_eq!(
        program,
        Expr::Float {
            value: 124.123,
            typ: (),
            meta: Meta { start: 0, end: 7 }
        }
    );
}

#[test]
fn test_atom_colon() {
    let program = grammar::UntypedExprParser::new().parse(":atom").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "atom".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 5 }
        }
    );
}

#[test]
fn test_atom_mixed_case_colon() {
    let program = grammar::UntypedExprParser::new().parse(":AtOm").unwrap();
    assert_eq!(
        program,
        Expr::Atom {
            value: "AtOm".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 5 }
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
            value: "i am an atom".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 15 }
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
            value: "defmodule".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 9 }
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
            value: "Hello, world!".to_string(),
            typ: (),
            meta: Meta { start: 0, end: 15 }
        }
    );
}

#[test]
fn test_list() {
    let program = grammar::UntypedExprParser::new()
        .parse("[1, 2, 3]")
        .unwrap();
    assert_eq!(
        program,
        Expr::List {
            meta: Meta { start: 9, end: 8 },
            typ: (),
            head: Box::new(Expr::Int {
                value: 1,
                meta: Meta { start: 1, end: 2 },
                typ: ()
            }),
            tail: Box::new(Expr::List {
                meta: Meta { start: 9, end: 8 },
                typ: (),
                head: Box::new(Expr::Int {
                    value: 2,
                    meta: Meta { start: 4, end: 5 },
                    typ: ()
                }),
                tail: Box::new(Expr::List {
                    meta: Meta { start: 9, end: 8 },
                    typ: (),
                    head: Box::new(Expr::Int {
                        value: 3,
                        meta: Meta { start: 7, end: 8 },
                        typ: ()
                    }),
                    tail: Box::new(Expr::ListEnd {
                        meta: Meta { start: 9, end: 8 }
                    })
                })
            })
        }
    );
}

#[test]
fn mixed_type_list_test() {
    let program = grammar::UntypedExprParser::new()
        .parse("[1, :hi, \"woah\"]")
        .unwrap();
    assert_eq!(
        program,
        Expr::List {
            meta: Meta { start: 16, end: 15 },
            typ: (),
            head: Box::new(Expr::Int {
                value: 1,
                meta: Meta { start: 1, end: 2 },
                typ: ()
            }),
            tail: Box::new(Expr::List {
                meta: Meta { start: 16, end: 15 },
                typ: (),
                head: Box::new(Expr::Atom {
                    value: String::from("hi"),
                    meta: Meta { start: 4, end: 7 },
                    typ: ()
                }),
                tail: Box::new(Expr::List {
                    meta: Meta { start: 16, end: 15 },
                    typ: (),
                    head: Box::new(Expr::String {
                        value: String::from("woah"),
                        meta: Meta { start: 9, end: 15 },
                        typ: ()
                    }),
                    tail: Box::new(Expr::ListEnd {
                        meta: Meta { start: 16, end: 15 }
                    })
                })
            })
        }
    );
}
