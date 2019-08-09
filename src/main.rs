#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_atom() {
    let program = grammar::LiteralParser::new()
        .parse(":atom")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "Atom(\"atom\")");
}

#[test]
fn test_string() {
    let program = grammar::LiteralParser::new()
        .parse("\"hello, world!\"")
        .unwrap();
    assert_eq!(&format!("{:?}", program), "String(\"hello, world!\")");
}
