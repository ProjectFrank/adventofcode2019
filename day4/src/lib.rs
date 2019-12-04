mod alternate;

// a number can be modeled as the difference between the each digit,
// going from left to right, starting with an implicit zero. For
// example, 111111 can be modeled as 100000.

// we can meet the requirement of having at least two adjacent digits
// being the same by ensuring that there is at least one zero

// the sum of the digits of this number must be less than 9

// the first digit of said number cannot be zero

// we need a function to convert the model numbers back to normal
// numbers in order to make sure they are within range

type IncreasingNumber = [u32; 6];

fn to_number(x: &IncreasingNumber) -> u32 {
    let mut digits = Vec::new();
    let mut current_digit = 0;
    for n in x {
        let digit = current_digit + n;
        digits.push(digit);
        current_digit = digit;
    }
    let powers_of_10 = (0..).map(|x| 10_u32.pow(x));
    digits
        .iter()
        .rev()
        .zip(powers_of_10)
        .fold(0, |acc, (digit, pow_10)| acc + digit * pow_10)
}

fn contains_nonleading_zero(increasing_number: &IncreasingNumber) -> bool {
    match increasing_number[1..].iter().find(|x| **x == 0) {
        Some(_) => true,
        None => false,
    }
}

fn count_passwords<F>(lower: u32, upper: u32, pred: F) -> u32
where
    F: Fn(&IncreasingNumber) -> bool,
{
    let mut num_passwords = 0;
    for a in 1..10 {
        for b in 0..(10 - a) {
            for c in 0..(10 - a - b) {
                for d in 0..(10 - a - b - c) {
                    for e in 0..(10 - a - b - c - d) {
                        for f in 0..(10 - a - b - c - d - e) {
                            let increasing_number: IncreasingNumber = [a, b, c, d, e, f];
                            let number = to_number(&increasing_number);
                            if pred(&increasing_number) && number <= upper && number >= lower {
                                num_passwords += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    num_passwords
}

fn pt1(lower: u32, upper: u32) -> u32 {
    count_passwords(lower, upper, contains_nonleading_zero)
}

fn count_consecutive_zeroes(increasing_number: &IncreasingNumber) -> Vec<usize> {
    let mut streaks = Vec::new();
    let mut previous_digit = increasing_number[0];
    let mut current_streak = 0;
    for &current_digit in &increasing_number[1..] {
        if previous_digit == 0 && current_digit != 0 {
            streaks.push(current_streak);
            current_streak = 0;
        }
        if current_digit == 0 {
            current_streak += 1;
        }
        previous_digit = current_digit;
    }
    if current_streak != 0 {
        streaks.push(current_streak);
    }

    streaks
}

fn pt2_predicate(increasing_number: &IncreasingNumber) -> bool {
    match count_consecutive_zeroes(&increasing_number)
        .iter()
        .find(|x| **x == 1)
    {
        Some(_) => true,
        None => false,
    }
}

fn pt2(lower: u32, upper: u32) -> u32 {
    count_passwords(lower, upper, pt2_predicate)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_number_test() {
        assert_eq!(to_number(&[1, 1, 1, 1, 1, 1]), 123456);
    }

    #[test]
    fn count_consecutive_zeroes_test() {
        assert_eq!(count_consecutive_zeroes(&[1, 0, 0, 1, 0, 0]), vec![2, 2]);
    }

    #[test]
    fn pt1_test() {
        assert_eq!(pt1(134564, 585159), 1929);
    }

    #[test]
    fn pt2_test() {
        assert_eq!(pt2(134564, 585159), 1306);
    }
}
