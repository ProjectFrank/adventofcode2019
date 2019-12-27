use intcode_computer as intcode;

pub fn pt1(path_to_input: &str) -> Result<i32, String> {
    let mut computer = intcode::IntcodeComputer::new(&intcode::read_file(path_to_input), Vec::new());
    intcode::process_inputs(12, 2, &mut computer)
}

fn try_input(noun: i32, verb: i32, computer: &intcode::IntcodeComputer, required_output: i32) -> bool {
    let mut computer = computer.clone();
    match intcode::process_inputs(noun, verb, &mut computer) {
        Ok(val) => val == required_output,
        _ => false,
    }
}

pub fn pt2(path_to_input: &str, required_output: i32) -> Option<(i32, i32)> {
    let computer = intcode::IntcodeComputer::new(&intcode::read_file(path_to_input), Vec::new());
    for i in 0..1000 {
        for j in 0..1000 {
            if try_input(i, j, &computer, required_output) {
                return Some((i, j));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pt1_test() {
        assert_eq!(pt1("input"), Ok(3_101_844));
    }

    #[test]
    fn pt2_test() {
        assert_eq!(pt2("input", 19_690_720), Some((84, 78)));
    }
}
