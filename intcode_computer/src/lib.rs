use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::default::Default;
use std::{fs, iter};

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

#[derive(Clone, PartialEq)]
pub enum State {
    WaitingForInput,
    Terminated,
    Running,
    Initialized,
}

pub struct IntcodeComputer {
    intcode: Vec<i32>,
    position: usize,
    input: VecDeque<i32>,
    pub output: Vec<i32>,
    pub state: State,
}

impl IntcodeComputer {
    pub fn new(raw_intcode: &str, input: Vec<i32>) -> Self {
        Self {
            intcode: parse_intcode(raw_intcode),
            input: input.iter().rev().copied().collect(),
            output: Vec::new(),
            position: 0,
            state: State::Initialized,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.state = State::Running;
        while self.position < self.intcode.len() && self.state == State::Running {
            let operation = Opcode::new(&self.intcode, self.position);
            if operation.opcode == 99 {
                self.state = State::Terminated;
            } else {
                operation.execute(self)?;
            }
        }

        match self.state {
            State::Running => Err(String::from("EOF error")),
            State::WaitingForInput | State::Terminated => Ok(()),
            _ => Err(String::from("This shouldn't have happened.")),
        }
    }

    pub fn consume_input(&mut self) -> Option<i32> {
        self.input.pop_back()
    }

    pub fn feed_input(&mut self, input: i32) -> Result<(), String> {
        self.input.push_front(input);
        match self.state {
            State::Terminated => Err(String::from(
                "Attempted to feed input to terminated computer.",
            )),
            State::WaitingForInput => {
                self.run()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn produce_output(&mut self, output: i32) -> Result<(), String> {
        self.output.push(output);
        Ok(())
    }
}

struct Opcode {
    pub opcode: i32,
    operands: Vec<i32>,
    param_modes: Vec<i32>,
}

#[derive(Default)]
/// Data representing the operation to execute
struct Operation {
    /// Tuple of index and value
    set_value: Option<(i32, i32)>,
    /// Position in intcode to jump to
    jump_to: Option<i32>,
    wait: bool,
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
        let operation = match self.opcode {
            1 => self.opcode_1(computer),
            2 => self.opcode_2(computer),
            3 => self.opcode_3(computer),
            4 => self.opcode_4(computer),
            5 => self.opcode_5(computer),
            6 => self.opcode_6(computer),
            7 => self.opcode_7(computer),
            8 => self.opcode_8(computer),
            _ => Err(format!("Cannot execute opcode: {}", self.opcode)),
        }?;
        if let Some((idx, val)) = operation.set_value {
            set_or_error(&mut computer.intcode, idx, val)?
        } else {
        }
        if let Some(position) = operation.jump_to {
            computer.position = usize::try_from(position).unwrap();
        } else if !operation.wait {
            computer.position += self.operands.len() + 1;
        }
        if operation.wait {
            computer.state = State::WaitingForInput;
        }
        Ok(())
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

    fn opcode_1(&self, computer: &IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        let sum = read_params[0] + read_params[1];
        Ok(Operation {
            set_value: Some((self.operands[2], sum)),
            ..Default::default()
        })
    }

    fn opcode_2(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        let product = read_params[0] * read_params[1];
        Ok(Operation {
            set_value: Some((self.operands[2], product)),
            ..Default::default()
        })
    }

    fn opcode_3(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        if let Some(input) = computer.consume_input() {
            Ok(Operation {
                set_value: Some((self.operands[0], input)),
                ..Default::default()
            })
        } else {
            Ok(Operation {
                wait: true,
                ..Default::default()
            })
        }
    }

    fn opcode_4(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        computer.produce_output(read_params[0])?;
        Ok(Default::default())
    }

    fn opcode_5(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        Ok(Operation {
            jump_to: if read_params[0] != 0 {
                Some(read_params[1])
            } else {
                None
            },
            ..Default::default()
        })
    }

    fn opcode_6(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        Ok(Operation {
            jump_to: if read_params[0] == 0 {
                Some(read_params[1])
            } else {
                None
            },
            ..Default::default()
        })
    }

    fn opcode_7(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        let value_to_store = if read_params[0] < read_params[1] {
            1
        } else {
            0
        };
        Ok(Operation {
            set_value: Some((self.operands[2], value_to_store)),
            ..Default::default()
        })
    }

    fn opcode_8(&self, computer: &mut IntcodeComputer) -> Result<Operation, String> {
        let read_params = self.read_params(&computer.intcode)?;
        let value_to_store = if read_params[0] == read_params[1] {
            1
        } else {
            0
        };
        Ok(Operation {
            set_value: Some((self.operands[2], value_to_store)),
            ..Default::default()
        })
    }

    pub fn new(intcode: &[i32], position: usize) -> Self {
        let num = intcode[position];
        let opcode = parse_opcode(num);
        let num_operands = num_operands(opcode);
        let slice = intcode[position + 1..].iter().copied().take(num_operands);
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
        3 => 1,
        4 => 1,
        5 => 2,
        6 => 2,
        7 => 3,
        8 => 3,
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

    computer.run()?;

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
        assert_eq!(
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            computer.intcode
        );
    }

    #[test]
    fn opcode_2_test() {
        let mut computer = IntcodeComputer::new("1,9,10,70,2,3,11,0,99,30,40,50", Vec::new());
        let operation = Opcode::new(&computer.intcode, 4);
        operation.execute(&mut computer).unwrap();
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            computer.intcode
        );
    }

    #[test]
    fn opcode_constructor_works() {
        let intcode = parse_intcode("1002,4,3,4,33");
        let operation = Opcode::new(&intcode, 0);
        assert_eq!(operation.operands, vec![4, 3, 4]);
        assert_eq!(operation.param_modes, vec![0, 1, 0]);
        assert_eq!(operation.opcode, 2);
    }

    #[test]
    fn input_output_works() {
        let mut computer = IntcodeComputer::new("3,0,4,0,99", vec![5]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![5]);
    }

    #[test]
    fn day5_comparison_test1() {
        let raw_code = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut computer = IntcodeComputer::new(raw_code, vec![8]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1]);

        let mut computer = IntcodeComputer::new(raw_code, vec![7]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn day5_comparison_test2() {
        let raw_code = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut computer = IntcodeComputer::new(raw_code, vec![8]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0]);

        let mut computer = IntcodeComputer::new(raw_code, vec![7]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn day5_comparison_test3() {
        let raw_code = "3,3,1108,-1,8,3,4,3,99";

        let mut computer = IntcodeComputer::new(raw_code, vec![8]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1]);

        let mut computer = IntcodeComputer::new(raw_code, vec![7]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0]);
    }

    #[test]
    fn day5_comparison_test4() {
        let raw_code = "3,3,1107,-1,8,3,4,3,99";

        let mut computer = IntcodeComputer::new(raw_code, vec![8]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0]);

        let mut computer = IntcodeComputer::new(raw_code, vec![7]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn day5_jump_tests() {
        let raw_code = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";

        let mut computer = IntcodeComputer::new(raw_code, vec![0]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0]);

        let mut computer = IntcodeComputer::new(raw_code, vec![1000]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn day5_larger_example() {
        let raw_code = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = IntcodeComputer::new(raw_code, vec![7]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![999]);

        let mut computer = IntcodeComputer::new(raw_code, vec![8]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1000]);

        let mut computer = IntcodeComputer::new(raw_code, vec![9]);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![1001]);
    }
}
