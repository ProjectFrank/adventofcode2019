use intcode_computer::*;

fn pt1 (path_to_input: &str) -> Result<i32, String> {
    let raw_code = read_file(path_to_input);
    let mut computer = IntcodeComputer::new(&raw_code, vec![1]);
    computer.run()?;
    let nonzero_outputs: Vec<i32> = computer
        .output
        .iter()
        .copied()
        .skip_while(|&x| x == 0)
        .collect();
    if nonzero_outputs.len() > 1 {
        Err(String::from("Intcode computer is not working correctly."))
    } else {
        Ok(nonzero_outputs[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(pt1("input").unwrap(), 13_547_311);
    }
}
