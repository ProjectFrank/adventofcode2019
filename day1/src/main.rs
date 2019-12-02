use std::env;
use day1::*;

fn main() {
    let path: String = env::args().skip(1).next().unwrap();

    let result = pt2(&path);

    println!("{}", result);
}
