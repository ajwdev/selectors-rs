extern crate lalrpop;

fn main() {
    println!("Test!");
    lalrpop::process_root().unwrap();
}
