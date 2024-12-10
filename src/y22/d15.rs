use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn hamilton(p: &Point, q: &Point) -> u32 {
    p.x.abs_diff(q.x) + p.y.abs_diff(q.y)
}

pub fn run(content: &str) -> u64 {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let signal_pairs: Vec<(Point, Point)> = re
        .captures_iter(content)
        .map(|capture| {
            let (_, [a, b, c, d]) = capture.extract();
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            let c: i32 = c.parse().unwrap();
            let d: i32 = d.parse().unwrap();

            (Point::new(a, b), Point::new(c, d))
        })
        .collect();
    let beacon_pos: HashSet<Point> = signal_pairs.iter().map(|(_, b)| b.clone()).collect();

    // let y_target : i32 = 10;

    let mut ans: Vec<Point> = vec![];

    let mut calc_intervals = |y_target: i32| {
        let mut intervals: Vec<RangeInclusive<i32>> = vec![];
        for (s, b) in &signal_pairs {
            let mut dist = hamilton(&s, &b) as i32;
            dist -= y_target.abs_diff(s.y) as i32;

            if dist < 0 {
                continue;
            }

            let x1 = s.x - dist;
            let x2 = s.x + dist;

            if b.y == y_target {
                if x1 < b.x {
                    intervals.push(x1..=b.x - 1);
                }
                if b.x < x2 {
                    intervals.push(b.x + 1..=x2);
                }
            } else {
                intervals.push(x1..=x2);
            }
        }
        intervals.sort_by(|x, y| x.start().cmp(&y.start()));

        let mut close: i32 = 0;
        for range in intervals {
            if close + 1 < *range.start() {
                let potential = Point::new(close + 1, y_target);
                if !beacon_pos.contains(&potential) {
                    ans.push(potential);
                    println!(
                        "y={y_target} Range open between x={} and {}",
                        close + 1,
                        *range.start() - 1
                    );
                }
            }
            if close < *range.end() {
                close = *range.end();
            }
        }
    };

    // let range = 0..=20;
    let range = 0..=4000000;

    for y in range {
        calc_intervals(y);
    }

    if ans.len() == 1 {
        let p = ans.first().unwrap();
        (p.x as u64) * 4000000 + p.y as u64
    } else {
        0
    }
}
