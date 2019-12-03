use std::fs;

struct Operands {
    x: usize,
    y: usize,
    position_to_set: usize,
}

impl Operands {
    fn opcode_1(&self, intcode: &mut Vec<usize>) {
        intcode[self.position_to_set] = self.x + self.y;
    }

    fn opcode_2(&self, intcode: &mut Vec<usize>) {
        intcode[self.position_to_set] = self.x * self.y;
    }

    fn new(intcode: &Vec<usize>, position: usize) -> Operands {
        Operands {
            x: intcode[intcode[position + 1]],
            y: intcode[intcode[position + 2]],
            position_to_set: intcode[position + 3],
        }
    }
}

fn parse_intcode(code: &str) -> Vec<usize> {
    code.split(',')
        .map(|item| {
            item.parse().unwrap()
        })
        .collect()
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',')
        .collect()
}

fn restore_state(intcode: &mut Vec<usize>) {
    intcode[1] = 12;
    intcode[2] = 2;
}

pub fn pt1(path_to_input: &str) -> Vec<usize> {
    let mut intcode = parse_intcode(&read_file(path_to_input));
    let mut position = 0;

    restore_state(&mut intcode);
    while position < intcode.len() {
        let operands = Operands::new(&intcode, position);
        match intcode[position] {
            1 => operands.opcode_1(&mut intcode),
            2 => operands.opcode_2(&mut intcode),
            99 => return intcode,
            _ => panic!("Unrecognized intcode"),
        }
        position += 4;
    }

    intcode
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_1_test() {
        let mut intcode = parse_intcode("1,9,10,3,2,3,11,0,99,30,40,50");
        let operands = Operands::new(&intcode, 0);
        operands.opcode_1(&mut intcode);
        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }

    #[test]
    fn opcode_2_test() {
        let mut intcode = parse_intcode("1,9,10,70,2,3,11,0,99,30,40,50");
        let operands = Operands::new(&intcode, 4);
        operands.opcode_2(&mut intcode);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }

    fn pt1_test() {
        let spent_intcode = pt1("input");
        assert_eq!(spent_intcode[0], 3101844);
    }
}
