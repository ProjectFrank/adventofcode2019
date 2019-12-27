use std::{fs, iter};
use std::convert::TryFrom;

pub fn parse_intcode(code: &str) -> Vec<i32> {
    code.split(',').map(|item| item.parse().unwrap()).collect()
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',' || c == '-')
        .collect()
}

#[derive(Clone)]
pub struct IntcodeComputer {
    intcode: Vec<i32>,
    position: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl IntcodeComputer {
    pub fn new(raw_intcode: &str, input: Vec<i32>) -> Self {
        Self {
            intcode: parse_intcode(raw_intcode),
            input,
            output: Vec::new(),
            position: 0
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.position < self.intcode.len() {
            let operation = Opcode::new(&self.intcode, self.position);
            if operation.opcode == 99 {
                return Ok(());
            } else {
                operation.execute(self)?;
            }
        }
        Err(String::from("EOF error"))
    }
}

struct Opcode {
    pub opcode: i32,
    operands: Vec<i32>,
    param_modes: Vec<i32>,
}

fn get_or_error(intcode: &[i32], idx: i32) -> Result<i32, String> {
    if let Ok(coerced_idx) = usize::try_from(idx) {
        if let Some(&x) = intcode.get(coerced_idx) {
            Ok(x)
        } else {
            Err(format!(
                "Index out of bounds. intcode length is: {}, index is: {}",
                intcode.len(),
                idx
            ))
        }        
    } else {
        Err(format!("Int {} could not be coerced into a usize.", idx))
    }
}

fn set_or_error(intcode: &mut Vec<i32>, idx: i32, val: i32) -> Result<(), String> {
    if let Ok(coerced_idx) = usize::try_from(idx) {
        if let Some(x) = intcode.get_mut(coerced_idx) {
            *x = val;
            Ok(())
        } else {
            Err(format!(
                "Index out of bounds. intcode length is: {}, index is: {}",
                intcode.len(),
                idx
            ))
        }        
    } else {
        Err(format!("Int {} could not be coerced into a usize.", idx))
    }
}

impl Opcode {
    pub fn execute(&self, computer: &mut IntcodeComputer) -> Result<(), String> {
        let result = match self.opcode {
            1 => self.opcode_1(computer),
            2 => self.opcode_2(computer),
            _ => Err(format!("Cannot execute opcode: {}", self.opcode)),
        };
        computer.position += self.operands.len() + 1;
        result
    }

    fn read_params(&self, intcode: &[i32]) -> Result<Vec<i32>, String> {
        self.param_modes
            .iter()
            .zip(self.operands.iter())
            .map(|(param_mode, &num_at_position)| match param_mode {
                0 => get_or_error(intcode, num_at_position),
                1 => Ok(num_at_position),
                _ => Err(String::from("unknown param mode")),
            })
            .collect()
    }

    fn opcode_1(&self, computer: &mut IntcodeComputer) -> Result<(), String> {
        let read_params = self.read_params(&computer.intcode)?;
        let sum = read_params[0] + read_params[1];
        set_or_error(&mut computer.intcode, self.operands[2], sum)
    }

    fn opcode_2(&self, computer: &mut IntcodeComputer) -> Result<(), String> {
        let read_params = self.read_params(&computer.intcode)?;
        let product = read_params[0] * read_params[1];
        set_or_error(&mut computer.intcode, self.operands[2], product)
    }

    pub fn new(intcode: &[i32], position: usize) -> Self {
        let num = intcode[position];
        let opcode = parse_opcode(num);
        let num_operands = num_operands(opcode);
        let slice = intcode[position + 1..]
            .iter()
            .copied()
            .take(num_operands);
        let param_modes = parse_parameter_modes(num)
            .iter()
            .copied()
            .chain(iter::repeat(0))
            .take(num_operands)
            .collect();
        Self {
            operands: slice.collect(),
            opcode,
            param_modes,
        }
    }
}

fn parse_opcode(num: i32) -> i32 {
    num % 100
}

fn num_operands(opcode: i32) -> usize {
    match opcode {
        1 => 3,
        2 => 3,
        99 => 0,
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

/// Takes a number representing parameter modes stuck to an opcode
fn parse_parameter_modes(num: i32) -> Vec<i32> {
    let mut parameter_modes = Vec::new();
    let mut remaining = num / 100;
    while remaining > 0 {
        parameter_modes.push(remaining % 10);
        remaining /= 10;
    }

    parameter_modes
}

pub fn process_inputs(noun: i32, verb: i32, computer: &mut IntcodeComputer) -> Result<i32, String> {

    computer.intcode[1] = noun;
    computer.intcode[2] = verb;

    computer.run();

    Ok(computer.intcode[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_1_test() {
        let mut computer = IntcodeComputer::new("1,9,10,3,2,3,11,0,99,30,40,50", Vec::new());
        let operation = Opcode::new(&computer.intcode, 0);
        operation.execute(&mut computer).unwrap();
        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], computer.intcode);
    }

    #[test]
    fn opcode_2_test() {
        let mut computer = IntcodeComputer::new("1,9,10,70,2,3,11,0,99,30,40,50", Vec::new());
        let operation = Opcode::new(&computer.intcode, 4);
        operation.execute(&mut computer).unwrap();
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], computer.intcode);
    }

    #[test]
    fn opcode_constructor_works() {
        let intcode = parse_intcode("1002,4,3,4,33");
        let operation = Opcode::new(&intcode, 0);
        assert_eq!(operation.operands, vec![4, 3, 4]);
        assert_eq!(operation.param_modes, vec![0, 1, 0]);
        assert_eq!(operation.opcode, 2);
    }
}
