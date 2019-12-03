use day1::*;
use std::env;

fn main() {
    let path: String = env::args().skip(1).next().unwrap();

    let result = pt2(&path);

    println!("{}", result);
}
