use std::fs;

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

fn opcode_1(intcode: &mut Vec<usize>, position: usize) -> Result<(), String> {
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
        let position_to_set = intcode[position + 3];
        let addend1 = intcode[intcode[position + 1]];
        let addend2 = intcode[intcode[position + 2]];
        intcode[position_to_set] = addend1 + addend2;
        Ok(())
    }
}

fn opcode_2(intcode: &mut Vec<usize>, position: usize) -> Result<(), String> {
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
        let position_to_set = intcode[position + 3];
        let factor1 = intcode[intcode[position + 1]];
        let factor2 = intcode[intcode[position + 2]];
        intcode[position_to_set] = factor1 * factor2;
        Ok(())
    }
}

pub fn process_inputs(noun: usize, verb: usize, intcode: &mut Vec<usize>) -> Result<usize, String> {
    let mut position = 0;

    intcode[1] = noun;
    intcode[2] = verb;

    while position < intcode.len() {
        match intcode[position] {
            1 => opcode_1(intcode, position)?,
            2 => opcode_2(intcode, position)?,
            99 => return Ok(intcode[0]),
            _ => return Err(String::from("Unrecognized intcode")),
        }
        position += 4;
    }

    Err(String::from("EOF error"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_1_test() {
        let mut intcode = parse_intcode("1,9,10,3,2,3,11,0,99,30,40,50");
        opcode_1(&mut intcode, 0).unwrap();
        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }

    #[test]
    fn opcode_2_test() {
        let mut intcode = parse_intcode("1,9,10,70,2,3,11,0,99,30,40,50");
        opcode_2(&mut intcode, 4).unwrap();
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], intcode);
    }
}
