use intcode_computer::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::i32;

mod pt1 {
    use super::*;

    fn helper(
        raw_code: &str,
        input: i32,
        used_phase_settings: HashSet<i32>,
        recursion_counter: u32,
        biggest: &mut i32,
    ) {
        if recursion_counter < 5 {
            for x in 0..5 {
                if !used_phase_settings.contains(&x) {
                    let mut computer = IntcodeComputer::new(raw_code, vec![x, input]);
                    computer.run().unwrap();
                    let output = computer.output[0];
                    let mut used_phase_settings_clone = used_phase_settings.clone();
                    used_phase_settings_clone.insert(x);
                    helper(
                        raw_code,
                        output,
                        used_phase_settings_clone,
                        recursion_counter + 1,
                        biggest,
                    );
                }
            }
        } else if input > *biggest {
            *biggest = input;
        }
    }

    pub fn pt1(raw_code: &str) -> i32 {
        let mut biggest = i32::MIN;
        helper(raw_code, 0, HashSet::new(), 0, &mut biggest);

        biggest
    }
}

pub use pt1::pt1;

mod pt2 {
    use super::*;
    pub fn pt2(raw_code: &str) -> Result<i32, String> {
        let mut biggest = i32::MIN;

        for phase_settings in (5..10).permutations(5) {
            let mut computers: Vec<IntcodeComputer> = phase_settings
                .iter()
                .map(|&phase_setting| IntcodeComputer::new(raw_code, vec![phase_setting]))
                .collect();

            for computer in &mut computers {
                computer.run()?;
            }

            let mut i = 0;
            let mut next_input = 0;

            while computers.last().unwrap().state != State::Terminated {
                let computer = &mut computers[i % 5];
                let old_output_len = computer.output.len();
                computer.feed_input(next_input)?;
                let num_outputs_produced = computer.output.len() - old_output_len;
                if num_outputs_produced != 1 {
                    panic!("Computer #{} produced more than one output", i % 5)
                }
                i += 1;
                next_input = *computer.output.last().unwrap();
            }

            let &signal = computers.last().unwrap().output.last().unwrap();

            if signal > biggest {
                biggest = signal;
            }
        }

        Ok(biggest)
    }
}

pub use pt2::pt2;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pt1_test1() {
        let raw_code = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(pt1(raw_code), 43210);
    }

    #[test]
    fn pt1_test2() {
        let raw_code = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(pt1(raw_code), 54321);
    }

    #[test]
    fn pt1_test3() {
        let raw_code = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(pt1(raw_code), 65210);
    }

    #[test]
    fn pt1_test() {
        let raw_code = read_file("input");
        assert_eq!(pt1(&raw_code), 21000);
    }

    #[test]
    fn pt2_example1() {
        let raw_code =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(pt2(raw_code).unwrap(), 139_629_729);
    }

    #[test]
    fn pt2_example2() {
        let raw_code = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(pt2(raw_code).unwrap(), 18216);
    }

    #[test]
    fn pt2_test() {
        let raw_code = read_file("input");
        assert_eq!(pt2(&raw_code).unwrap(), 61_379_886);
    }
}
