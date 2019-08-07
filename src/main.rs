#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);

pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator1::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
