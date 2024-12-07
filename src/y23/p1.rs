use std::fs;

fn calibration_value(input: &str) -> u32 {
    let first = find_first_digit(input, false);
    let last = find_first_digit(input, true);

    // let reversed_str: String = input.chars().rev().collect();
    first * 10 + last
}

fn calibration_value_old(input: &str) -> u32 {
    let lst = parse_digits_normal(input);
    let n = lst.len();
    assert!(n >= 1);
    lst[0] * 10 + lst[n - 1]
}

fn parse_digits_normal(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

const DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse_digits_wtf(input: &str) -> Vec<u32> {
    let mut i: usize = 0;
    let mut out: Vec<u32> = vec![];
    let digits: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    while i < input.len() {
        let substr: &str = &input[i..];
        let old = out.len();

        for j in 0..digits.len() {
            if substr.starts_with(digits[j]) {
                out.push(j as u32);
                i += digits[j].len();
                break;
            }
            if substr.starts_with(j.to_string().as_str()) {
                out.push(j as u32);
                i += 1;
                break;
            }
        }
        if old == out.len() {
            i += 1;
        }
    }
    out
}

fn find_first_digit(input: &str, reversed: bool) -> u32 {
    for i in 0..input.len() {
        let idx = if reversed { input.len() - 1 - i } else { i };
        let s = &input[idx..];

        for j in 0..DIGITS.len() {
            if s.starts_with(&j.to_string()) || s.starts_with(DIGITS[j]) {
                return j as u32;
            }
        }
    }
    panic!();
}

pub fn run() {
    let contents = fs::read_to_string("../../inputs/y23/1.in")
        .expect("Should have been able to read the file");

    let ferris: u32 = contents.lines().map(calibration_value).sum();
    contents.lines().for_each(|l| {
        let a = calibration_value(l);
        let b = calibration_value_old(l);

        if a != b {
            println!("{:} {:} {:}", a, b, l);
        }
    });
    // let ferris: u32 = contents.lines().map(|l| calibration_value(&l)).sum();
    println!("{}", ferris);
}


#[cfg(test)]
mod tests {
    use super::{calibration_value, parse_digits_wtf};

    #[test]
    fn test_calibration_value() {
        assert_eq!(17, calibration_value("onetwozero7"));

        assert_eq!(29, calibration_value("two1nine"));
        assert_eq!(83, calibration_value("eightwothree"));
        assert_eq!(13, calibration_value("abcone2threexyz"));
        assert_eq!(24, calibration_value("xtwone3four"));
        assert_eq!(42, calibration_value("4nineeightseven2"));
        assert_eq!(14, calibration_value("zoneight234"));
        assert_eq!(76, calibration_value("7pqrstsixteen"));
        assert_eq!(55, calibration_value("5n"));

        assert_eq!(68, calibration_value("6bjztkxhsixkgnkroneightht"));
    }

    #[test]
    fn test_parse_digits() {
        assert_eq!(vec![8], parse_digits_wtf("eightwo"));
        assert_eq!(vec![8, 3], parse_digits_wtf("eightwothree"));
        assert_eq!(vec![1, 2, 0, 7], parse_digits_wtf("onetwozero7"));
        assert_eq!(vec![3, 2, 0, 7], parse_digits_wtf("on3etwozero7"));
        assert_eq!(vec![5], parse_digits_wtf("5n"));
        assert_eq!(vec![9, 5, 2, 7], parse_digits_wtf("qfzclldsvzvcdqfxhtqqtknine527"));
    }
}

