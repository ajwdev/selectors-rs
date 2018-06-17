lalrpop_mod!(selector);
pub use self::selector::SelectorParser;

#[cfg(test)]
mod tests {
    use super::selector::*;

    macro_rules! parse_ok {
        ($p:ident, $e:expr) => {
            match $p.parse($e) {
                // This arm won't be needed once we're done testing parser "by hand"
                Ok(ast) => { println!("== ast for \"{}\" -> {:?}", $e, ast); }
                Err(why) => { panic!("error with \"{}\", line {}: {:?}", $e, line!(), why); }
            }
        }
    }

    macro_rules! parse_err {
        ($p:ident, $e:expr) => {
            if let Ok(ast) = $p.parse($e) {
                panic!("expected error with \"{}\", line {}; ast: {:?}", $e, line!(), ast);
            }
        }
    }

    #[test]
    fn selector_label_keys() {
        let p = LabelKeyParser::new();

        parse_ok!(p, "f");
        parse_ok!(p, "fo");
        parse_ok!(p, "foo");
        parse_ok!(p, "example.com/foo");
        parse_ok!(p, "foo_bar");
        parse_ok!(p, "foo.bar");

        parse_err!(p, "foo bar");
        parse_err!(p, "(foo)");
        parse_err!(p, "-foo");
        parse_err!(p, ".foo");
        parse_err!(p, "_foo");
        parse_err!(p, "baz/baz/foo");
        parse_err!(p, "bar/example.com/foo");
    }

    #[test]
    fn selector_label_values() {
        let p = LabelValueParser::new();

        parse_ok!(p, "f");
        parse_ok!(p, "fo");
        parse_ok!(p, "foo");
        parse_ok!(p, "foo_bar");
        parse_ok!(p, "foo.bar");

        parse_err!(p, "example.com/foo");
        parse_err!(p, "foo bar");
        parse_err!(p, "(foo)");
        parse_err!(p, "-foo");
        parse_err!(p, ".foo");
        parse_err!(p, "_foo");
        parse_err!(p, "baz/baz/foo");
    }

    #[test]
    fn selector_label_values_group() {
        let p = LabelValueGroupParser::new();

        parse_ok!(p, "(foo)");
        parse_ok!(p, "(foo,)");
        parse_ok!(p, "(foo,bar,baz)");
        parse_ok!(p, "(foo,bar,baz,)");
        parse_ok!(p, "()");

        parse_err!(p, "foo,bar,baz,");
    }

    #[test]
    fn selector_expressions() {
        let p = ExprParser::new();

        parse_ok!(p, "env in (qa)");
        parse_ok!(p, "env notin (qa)");
    }

    #[test]
    fn selector_lexing() {
        let p = SelectorParser::new();

        parse_ok!(p, "foo");
        parse_ok!(p, "foo_bar");
        parse_ok!(p, "foo.bar");

        parse_ok!(p, "foo = bar");
        parse_ok!(p, "foo == bar");
        parse_ok!(p, "foo != bar");

        parse_ok!(p, "foo=bar");
        parse_ok!(p, "foo==bar");
        parse_ok!(p, "foo!=bar");

        parse_ok!(p, "foo= bar");
        parse_ok!(p, "foo== bar");
        parse_ok!(p, "foo!= bar");

        parse_err!(p, "foo baz = bar");

        parse_ok!(p, "env in (qa),app!=foo,!bar");

    }

    #[test]
    #[ignore]
    fn empty_labels() {
        let p = SelectorParser::new();

        parse_ok!(p, "foo=");
    }

    #[test]
    #[ignore]
    fn set_operators_as_keys_and_labels() {
        let p = SelectorParser::new();

        parse_ok!(p, "foo = notin");
        parse_ok!(p, "notin = foo");
        parse_ok!(p, "foo = in");
        parse_ok!(p, "in = foo");
    }
}
