#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub selector);

fn main() {
    println!("{:?}", selector::LabelValueGroupParser::new().parse("(foo,bar,baz,)").unwrap());
    println!("{:?}", selector::LabelKeyParser::new().parse("foo").unwrap());
    println!("{:?}", selector::LabelKeyParser::new().parse("example.com/foo").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("example.com/foo").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo in (something,else)").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo in (something,else),bar==baz,bar != baz").unwrap());
    println!("{:?}", selector::LabelValueParser::new().parse("bar").unwrap());

    println!("{:?}", selector::SelectorParser::new().parse("foo").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("! foo").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo!=bar").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("foo=bar,something==else").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("app,foo!=bar").unwrap());
    println!("{:?}", selector::SelectorParser::new().parse("example.com/app=something,foo!=bar").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_labels() {
        assert!(selector::LabelKeyParser::new().parse("foo").is_ok());
        assert!(selector::LabelKeyParser::new().parse("example.com/foo").is_ok());
        assert!(selector::LabelKeyParser::new().parse("foo_bar").is_ok());
        assert!(selector::LabelKeyParser::new().parse("foo.bar").is_ok());
        assert!(selector::LabelKeyParser::new().parse("foo bar").is_err());
        assert!(selector::LabelKeyParser::new().parse("(foo)").is_err());
        assert!(selector::LabelKeyParser::new().parse("-foo").is_err());
        assert!(selector::LabelKeyParser::new().parse(".foo").is_err());
        assert!(selector::LabelKeyParser::new().parse("_foo").is_err());
        assert!(selector::LabelKeyParser::new().parse("bar/example.com/foo").is_err());
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
}
