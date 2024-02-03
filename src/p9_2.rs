use std::fmt::Display;

//noinspection ALL
pub fn solve_line(line: &str) -> i32 {
    let v: Vec<_> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
    let mut all = vec![v];

    loop {
        let mut t = vec![];
        let mut has_non_zero = false;
        {
            let last = all.last().unwrap();
            for i in 1..last.len() {
                let x = last[i] - last[i - 1];
                t.push(x);
                has_non_zero |= x != 0;
            }
        }
        all.push(t);
        if !has_non_zero {
            break;
        }
    }
    all.iter_mut().for_each(|x| x.reverse());

    for i in (0..all.len()).rev() {
        if i == all.len() - 1 {
            all[i].push(0);
        } else {
            let value = {
                let cur = all[i].last().unwrap();
                let next = all[i + 1].last().unwrap();
                cur - next
            };
            all[i].push(value);
        }
    }

    return *all[0].last().unwrap();
}

pub fn run(content: &str) -> impl Display {
    content.lines().map(solve_line).sum::<i32>()
}

#[test]
pub fn test_line() {
    let line = "0 3 6 9 12 15";
    assert_eq!(solve_line(line), -3);
}