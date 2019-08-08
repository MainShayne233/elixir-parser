#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);

pub mod ast;

#[test]
fn calculator1() {
    let program = calculator1::ProgramParser::new()
        .parse(":atom")
        .unwrap();
    assert_eq!(&format!("{:?}", program), ":atom");
}
