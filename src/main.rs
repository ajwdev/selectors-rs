#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub selector);

fn main() {
    println!("{:?}", selector::LabelKeyParser::new().parse("foo").unwrap());
    println!("{:?}", selector::LabelKeyParser::new().parse("example.com/foo").unwrap());
    println!("{:?}", selector::LabelValueParser::new().parse("bar").unwrap());

    println!("{:?}", selector::SelectorParser::new().parse("foo").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo!=bar").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo=bar,something==else").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("app,foo!=bar").unwrap());
}

#[test]
fn selector_labels() {
    assert!(selector::LabelKeyParser::new().parse("foo").is_ok());
    assert!(selector::LabelKeyParser::new().parse("foo_bar").is_ok());
    assert!(selector::LabelKeyParser::new().parse("foo.bar").is_ok());
    assert!(selector::LabelKeyParser::new().parse("foo bar").is_err());
    assert!(selector::LabelKeyParser::new().parse("(foo)").is_err());
    assert!(selector::LabelKeyParser::new().parse("-foo").is_err());
    assert!(selector::LabelKeyParser::new().parse(".foo").is_err());
    assert!(selector::LabelKeyParser::new().parse("_foo").is_err());
}

#[test]
fn selector() {
    assert!(selector::SelectorParser::new().parse("foo").is_ok());
    assert!(selector::SelectorParser::new().parse("foo_bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo.bar").is_ok());

    assert!(selector::SelectorParser::new().parse("foo = bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo == bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo != bar").is_ok());

    assert!(selector::SelectorParser::new().parse("foo=bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo==bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo!=bar").is_ok());

    assert!(selector::SelectorParser::new().parse("foo= bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo== bar").is_ok());
    assert!(selector::SelectorParser::new().parse("foo!= bar").is_ok());

    //assert!(selector::SelectorParser::new().parse("").is_ok());

    assert!(selector::SelectorParser::new().parse("foo baz = bar").is_err());
    assert!(selector::SelectorParser::new().parse("foo = ").is_err());
}
