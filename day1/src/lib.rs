use std::io::{BufRead, BufReader, Error};
use std::fs::File;

fn read_file(path: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn naive_fuel_cost(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_cost(mass: i32) -> i32 {
    let mut cost = naive_fuel_cost(mass);
    let mut incremental_cost = naive_fuel_cost(cost);

    while incremental_cost > 0 {
        cost += incremental_cost;
        incremental_cost = naive_fuel_cost(incremental_cost);
    }

    cost
}

fn naive_total_fuel_cost(reader: BufReader<File>) -> i32 {
    reader.lines()
        .map(|line| {
            naive_fuel_cost(line.unwrap().parse().unwrap())
        })
        .sum()
}

fn total_fuel_cost(reader: BufReader<File>) -> i32 {
    reader.lines()
        .map(|line| {
            fuel_cost(line.unwrap().parse().unwrap())
        })
        .sum()
}

pub fn pt1(path: &str) -> i32 {
    let reader = read_file(&path).unwrap();
    naive_total_fuel_cost(reader)
}

pub fn pt2(path: &str) -> i32 {
    let reader = read_file(&path).unwrap();
    total_fuel_cost(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        assert_eq!(3402609, pt1("input"));
    }

    #[test]
    fn pt2_test() {
        assert_eq!(5101025, pt2("input"));
    }
}
