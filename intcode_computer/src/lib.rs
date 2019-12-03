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

    fn new(intcode: &Vec<usize>, position: usize) -> Result<Operands, String> {
        let len = intcode.len();
        if position + 1 >= len
            || position + 2 >= len
            || position + 3 >= len
            || intcode[position + 1] >= len
            || intcode[position + 2] >= len
            || intcode[position + 3] >= len
        {
            Err(String::from("index out of bounds"))
        } else {
            Ok(Operands {
                x: intcode[intcode[position + 1]],
                y: intcode[intcode[position + 2]],
                position_to_set: intcode[position + 3],
            })
        }
    }
}

pub fn parse_intcode(code: &str) -> Vec<usize> {
    code.split(',').map(|item| item.parse().unwrap()).collect()
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',')
        .collect()
}

pub fn process_inputs(noun: usize, verb: usize, intcode: &mut Vec<usize>) -> Result<usize, String> {
    let mut position = 0;

    intcode[1] = noun;
    intcode[2] = verb;

    while position < intcode.len() {
        if let Ok(operands) = Operands::new(&intcode, position) {
            match intcode[position] {
                1 => operands.opcode_1(intcode),
                2 => operands.opcode_2(intcode),
                99 => return Ok(intcode[0]),
                _ => return Err(String::from("Unrecognized intcode")),
            }
            position += 4;
        } else {
            return Err(String::from("index out of bounds"));
        }
    }

    Err(String::from("EOF error"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_1_test() {
        let mut intcode = parse_intcode("1,9,10,3,2,3,11,0,99,30,40,50");
        let operands = Operands::new(&intcode, 0).unwrap();
        operands.opcode_1(&mut intcode);
        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }

    #[test]
    fn opcode_2_test() {
        let mut intcode = parse_intcode("1,9,10,70,2,3,11,0,99,30,40,50");
        let operands = Operands::new(&intcode, 4).unwrap();
        operands.opcode_2(&mut intcode);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }
}
