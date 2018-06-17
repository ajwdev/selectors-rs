lalrpop_mod!(selector);
pub use self::selector::SelectorParser;

#[cfg(test)]
mod tests {
    use super::selector::*;

    #[test]
    fn selector_label_keys() {
        let p = LabelKeyParser::new();

        assert!(p.parse("f").is_ok());
        assert!(p.parse("fo").is_ok());
        assert!(p.parse("foo").is_ok());
        assert!(p.parse("example.com/foo").is_ok());
        assert!(p.parse("foo_bar").is_ok());
        assert!(p.parse("foo.bar").is_ok());

        assert!(p.parse("foo bar").is_err());
        assert!(p.parse("(foo)").is_err());
        assert!(p.parse("-foo").is_err());
        assert!(p.parse(".foo").is_err());
        assert!(p.parse("_foo").is_err());
        assert!(p.parse("bar/example.com/foo").is_err());
    }

    #[test]
    fn selector_label_values() {
        let p = LabelValueParser::new();

        assert!(p.parse("f").is_ok());
        assert!(p.parse("fo").is_ok());
        assert!(p.parse("foo").is_ok());
        assert!(p.parse("foo_bar").is_ok());
        assert!(p.parse("foo.bar").is_ok());

        assert!(p.parse("example.com/foo").is_err());
        assert!(p.parse("foo bar").is_err());
        assert!(p.parse("(foo)").is_err());
        assert!(p.parse("-foo").is_err());
        assert!(p.parse(".foo").is_err());
        assert!(p.parse("_foo").is_err());
        assert!(p.parse("bar/example.com/foo").is_err());
    }

    #[test]
    fn selector_label_values_group() {
        let p = LabelValueGroupParser::new();

        assert!(p.parse("(foo)").is_ok());
        assert!(p.parse("(foo,)").is_ok());
        assert!(p.parse("(foo,bar,baz)").is_ok());
        assert!(p.parse("(foo,bar,baz,)").is_ok());
        assert!(p.parse("foo,bar,baz,").is_err());
    }

    #[test]
    fn selector_expressions() {
        let p = ExprParser::new();

        assert!(p.parse("env in (qa)").is_ok());
        assert!(p.parse("env notin (qa)").is_ok());
    }

    #[test]
    fn selector_lexing() {
        let p = SelectorParser::new();

        assert!(p.parse("foo").is_ok());
        assert!(p.parse("foo_bar").is_ok());
        assert!(p.parse("foo.bar").is_ok());

        assert!(p.parse("foo = bar").is_ok());
        assert!(p.parse("foo == bar").is_ok());
        assert!(p.parse("foo != bar").is_ok());

        assert!(p.parse("foo=bar").is_ok());
        assert!(p.parse("foo==bar").is_ok());
        assert!(p.parse("foo!=bar").is_ok());

        assert!(p.parse("foo= bar").is_ok());
        assert!(p.parse("foo== bar").is_ok());
        assert!(p.parse("foo!= bar").is_ok());

        //assert!(SelectorParser::new().parse("").is_ok());

        assert!(p.parse("foo baz = bar").is_err());
        assert!(p.parse("foo = ").is_err());

        println!("{:?}", p.parse("env in (qa),app!=foo,!bar").unwrap());
        assert!(p.parse("env in (qa),app!=foo,!bar").is_ok());
    }

    #[test]
    #[ignore]
    fn set_operators_as_keys_and_labels() {
        let p = SelectorParser::new();

        assert!(p.parse("foo = notin").is_ok());
        assert!(p.parse("notin = foo").is_ok());
        assert!(p.parse("foo = in").is_ok());
        assert!(p.parse("in = foo").is_ok());
    }
}
