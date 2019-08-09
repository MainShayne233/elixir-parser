#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_boolean_true() {
    let program = grammar::UntypedExprParser::new()
        .parse("true")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"true\" }");
}

#[test]
fn test_boolean_false() {
    let program = grammar::UntypedExprParser::new()
        .parse("false")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"false\" }");
}

#[test]
fn test_integer() {
    let program = grammar::UntypedExprParser::new()
        .parse("124")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Int { value: 124 }");
}

#[test]
fn test_integer_underscore() {
    let program = grammar::UntypedExprParser::new()
        .parse("123_456")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Int { value: 123456 }");
}

#[test]
fn test_float() {
    let program = grammar::UntypedExprParser::new()
        .parse("124.123")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Float { value: 124.123 }");
}

#[test]
fn test_atom_colon() {
    let program = grammar::UntypedExprParser::new()
        .parse(":atom")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"atom\" }");
}

#[test]
fn test_atom_mixed_case_colon() {
    let program = grammar::UntypedExprParser::new()
        .parse(":AtOm")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"AtOm\" }");
}

#[test]
fn test_atom_with_spaces_colon() {
    let program = grammar::UntypedExprParser::new()
        .parse(":\"i am an atom\"")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"i am an atom\" }");
}

#[test]
fn test_atom_literal() {
    let program = grammar::UntypedExprParser::new()
        .parse("defmodule")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom { value: \"defmodule\" }");
}

#[test]
fn test_string() {
    let program = grammar::UntypedExprParser::new()
        .parse("\"Hello, world!\"")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "String { value: \"Hello, world!\" }");
}
