#[macro_use]
extern crate lalrpop_util;

mod ast;
mod parser;

use parser::SelectorParser;

fn main() {
    let p = SelectorParser::new();
    println!("{:?}", p.parse("example.com/foo").unwrap());
    println!("{:?}", p.parse("foo in (something,else)").unwrap());
    println!("{:?}", p.parse("foo in (something,else),bar==baz,bar != baz").unwrap());
    println!("{:?}", p.parse("foo").unwrap());
    println!("{:?}", p.parse("! foo").unwrap());
    println!("{:?}", p.parse("foo!=bar").unwrap());
    println!("{:?}", p.parse("foo=bar,something==else").unwrap());
    println!("{:?}", p.parse("app,foo!=bar").unwrap());
    println!("{:?}", p.parse("example.com/app=something,foo!=bar").unwrap());
}
