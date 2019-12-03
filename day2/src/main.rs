use std::env;
use day2::*;

fn main() {
    let path: String = env::args().skip(1).next().unwrap();

    let result = pt2(&path, 19690720);
    
    println!("{:?}", result);
}
