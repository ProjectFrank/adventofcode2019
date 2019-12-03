use intcode_computer as intcode;

pub fn pt1(path_to_input: &str) -> Result<usize, String> {
    let mut intcode = intcode::parse_intcode(&intcode::read_file(path_to_input));
    intcode::process_inputs(12, 2, &mut intcode)
}

fn try_input(noun: usize, verb: usize, intcode: &Vec<usize>, required_output: usize) -> bool {
    let mut intcode = intcode.clone();
    match intcode::process_inputs(noun, verb, &mut intcode) {
        Ok(val) => val == required_output,
        _ => false,
    }
}

pub fn pt2(path_to_input: &str, required_output: usize) -> Option<(usize, usize)> {
    let intcode = intcode::parse_intcode(&intcode::read_file(path_to_input));

    for i in 0..1000 {
        for j in 0..1000 {
            if try_input(i, j, &intcode, required_output) {
                return Some((i, j));
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pt1_test() {
        assert_eq!(pt1("input"), Ok(3101844));
    }

    #[test]
    fn pt2_test() {
        assert_eq!(pt2("input", 19690720), Some((84, 78)));
    }
}
