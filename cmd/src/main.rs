extern crate ralculate;

use std::env;

fn main() {
    let input = env::args().nth(1).expect("usage: string");
    println!("{:?}", ralculate::parse_expr(&input))
}
