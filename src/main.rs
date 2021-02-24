#[macro_use] extern crate lalrpop_util;


lalrpop_mod!(pub calculator1);
lalrpop_mod!(pub calculator2);
lalrpop_mod!(pub calculator3);


#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("(((22)))").is_ok());
    assert!(calculator1::TermParser::new().parse("(((22)").is_err());

}

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("(((22)))").is_ok());
    assert!(calculator2::TermParser::new().parse("(((22)").is_err());

}

#[test]
fn calculator3() {
    assert!(calculator3::ExprParser::new().parse("22 + 3 * 5").unwrap() == 37);
    assert!(calculator3::ExprParser::new().parse("(1 + 3) * 5").unwrap() == 20);
}

lalrpop_mod!(pub calculator4);
mod ast;

#[test]
fn calculator4() {
    assert_eq!(
        &format!("{:?}", calculator4::ExprParser::new().parse("11 * 12 + 13").unwrap()),
        "((11 * 12) + 13)"
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Calculator6Error {
    InputTooBig,
    OddNumber
}

lalrpop_mod!(pub calculator6);
use lalrpop_util::ParseError;

#[test]
fn calculator6() {
    let expr = calculator6::ExprParser::new().parse("2147483648");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::InputTooBig });

    let expr = calculator6::ExprParser::new().parse("1");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::OddNumber });
}

lalrpop_mod!(pub calculator7);

#[test]
fn calculator7() {
    let mut errors = Vec::new();

    let expr = calculator7::ExprParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * error) + 3)");

    let expr = calculator7::ExprParser::new()
        .parse(&mut errors, "*3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "(error * 3)");

    let expr = calculator7::ExprParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "(error * error)");

    assert_eq!(errors.len(), 4);
}

lalrpop_mod!(pub calculator8);

#[test]
fn calculator8() {
    let scale = 2;
    let expr = calculator8::ExprParser::new()
        .parse(scale, "11 * 22 + 33")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

fn main() {
    println!("Hello, world!");
}
