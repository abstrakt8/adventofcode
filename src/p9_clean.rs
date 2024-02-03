use std::fmt::Display;

pub fn solve_line(line: &str) -> i32 {
    let mut vec_a: Vec<_> = line
        .split(' ')
        .map(str::parse::<i32>)
        .flat_map(Result::ok)
        .collect();
    let mut vec_b: Vec<i32> = vec![];

    let mut sum = 0;

    let ( mut curr,  mut next) = (&mut vec_a, &mut vec_b);
    std::mem::swap(&mut curr, &mut next);

    loop {
        let mut has_non_zero = false;
        let it = curr
            .windows(2)
            .map(|w| w[1] - w[0])
            .inspect(|&x| has_non_zero |= x != 0);
        next.clear();
        next.extend(it);
        sum += curr.last().unwrap();
        if !has_non_zero {
            break;
        }
        std::mem::swap(&mut curr, &mut next);
    }
    sum
}

pub fn run(content: &str) -> impl Display {
    content.lines().map(solve_line).sum::<i32>()
}

#[test]
pub fn test_line() {
    let line = "0 3 6 9 12 15";
    assert_eq!(solve_line(line), 18);
}