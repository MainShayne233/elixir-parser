#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_atom_colon() {
    let program = grammar::LiteralParser::new()
        .parse(":atom")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom(\"atom\")");
}

#[test]
fn test_atom_mixed_case_colon() {
    let program = grammar::LiteralParser::new()
        .parse(":AtOm")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom(\"AtOm\")");
}

#[test]
fn test_atom_with_spaces_colon() {
    let program = grammar::LiteralParser::new()
        .parse(":\"i am an atom\"")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom(\"i am an atom\")");
}

#[test]
fn test_atom_literal() {
    let program = grammar::LiteralParser::new()
        .parse("defmodule")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom(\"defmodule\")");
}

#[test]
fn test_string() {
    let program = grammar::LiteralParser::new()
        .parse("\"hello, world!\"")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "String(\"hello, world!\")");
}
