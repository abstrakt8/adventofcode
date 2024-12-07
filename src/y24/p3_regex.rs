use regex::Regex;

pub fn run1(content: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // https://docs.rs/regex/latest/regex/#example-finding-dates-in-a-haystack
    re.captures_iter(content).map(|caps| {
        let (_, [a, b]) = caps.extract();
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        a * b
    }).sum()
}
pub fn run2(content: &str) -> u32 {
    let re2 = Regex::new(r"d(o)(n)'t\(\)|(d)(o)\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut flag = 1;
    re2.captures_iter(content).map(|caps| {
        let (e, [a, b]) = caps.extract();

        if e.starts_with("don") {
            flag = 0;
        } else if e.starts_with("do") {
            flag = 1;
        } else {
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();

            return flag * a * b;
        }
        0
    }).sum()
}

