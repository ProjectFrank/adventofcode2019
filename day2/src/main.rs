use std::env;
use day2::*;

fn main() {
    let path: String = env::args().skip(1).next().unwrap();

    let result = pt1(&path);
    
    println!("{}", result[0]);
}
