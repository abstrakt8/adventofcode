use std::cmp::{max, min};

type Interval = [u32; 2];

fn intersection(a: Interval, b: Interval) -> Interval {
    [max(a[0], b[0]), min(a[1], b[1])]
}

fn equals(a: &Interval, b: &Interval) -> bool {
    a.iter().zip(b).all(|(a, b)| a == b)
}

pub fn run(content: &str) -> (u32, u32) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    content.lines().for_each(|line| {
        let intervals: Vec<Interval> = line
            .split(",")
            .map(|s| {
                let v: Vec<u32> = s.split("-").map(|x| x.parse().unwrap()).collect();
                [v[0], v[1]]
            })
            .collect();
        let inter = intersection(intervals[0], intervals[1]);
        if equals(&inter, &intervals[0]) || equals(&inter, &intervals[1]) {
            ans1 += 1;
        }
        if inter[0] <= inter[1] {
            ans2 += 1;
        }
    });

    (ans1 as u32, ans2)
}
