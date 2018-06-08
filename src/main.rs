#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub selector);

fn main() {
    println!("{:?}", selector::TermParser::new().parse("foo").unwrap());
    println!("{:?}", selector::ExprParser::new().parse("foo!=bar").unwrap());
}

#[test]
fn selector() {
    assert!(selector::TermParser::new().parse("foo").is_ok());
    println!("{:?}", selector::TermParser::new().parse("foo").unwrap());
    assert!(selector::TermParser::new().parse("((foo))").is_ok());
    assert!(selector::TermParser::new().parse("((foo)").is_err());
    assert!(selector::TermParser::new().parse("()").is_err());
    assert!(selector::TermParser::new().parse("foo_bar").is_ok());

    assert!(selector::ExprParser::new().parse("foo == bar").is_ok());
}
