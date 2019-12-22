type Digits = Vec<u32>;

fn to_digits(x: u32) -> Digits {
    let mut digits = Vec::new();
    let powers_of_10 = (0..6).map(|x| 10_u32.pow(x));
    powers_of_10.rev().fold(x, |remaining, pow_10| {
        digits.push(remaining / pow_10);
        remaining % pow_10
    });
    digits
}

fn is_increasing(digits: &[u32]) -> bool {
    let mut clone = Vec::new();
    clone.copy_from_slice(digits);
    clone.sort();
    digits == &clone[..]
}

fn has_matching_digits(digits: &[u32]) -> bool {
    let iter1 = digits.iter();
    let iter2 = digits.iter().skip(1);
    iter1.zip(iter2).any(|(x, y)| x == y)
}

fn count_consecutive_digits(digits: &[u32]) -> Vec<usize> {
    let mut streaks = Vec::new();
    let mut previous_digit = digits[0];
    let mut current_streak = 1;
    for &current_digit in &digits[1..] {
        if previous_digit == current_digit {
            current_streak += 1
        } else if current_streak != 1 {
            streaks.push(current_streak);
            current_streak = 1;
        }
        previous_digit = current_digit;
    }

    if current_streak != 1 {
        streaks.push(current_streak);
    }

    streaks
}

fn has_two_matching_digits(digits: &[u32]) -> bool {
    count_consecutive_digits(digits).iter().any(|x| *x == 2)
}

fn count_passwords<F>(lower: u32, upper: u32, pred: F) -> u32
where
    F: Fn(&[u32]) -> bool,
{
    let mut num_passwords = 0;
    for candidate_password in lower..=upper {
        let digits = to_digits(candidate_password);
        if pred(&digits)
            && is_increasing(&digits)
            && candidate_password <= upper
            && candidate_password >= lower
        {
            num_passwords += 1;
        }
    }

    num_passwords
}

pub fn pt1(lower: u32, upper: u32) -> u32 {
    count_passwords(lower, upper, has_matching_digits)
}

pub fn pt2(lower: u32, upper: u32) -> u32 {
    count_passwords(lower, upper, has_two_matching_digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_consecutive_digits_test() {
        assert_eq!(
            count_consecutive_digits(&vec![1, 2, 2, 3, 3, 3, 4]),
            vec![2, 3]
        );
    }

    #[test]
    fn has_two_matching_digits_test() {
        assert!(has_two_matching_digits(&vec![1, 1, 1, 1, 2, 2]));
        assert!(!has_two_matching_digits(&vec![1, 1, 1, 1, 1, 1]));
        assert!(!has_two_matching_digits(&vec![1, 2, 3, 4, 5, 6]));
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
