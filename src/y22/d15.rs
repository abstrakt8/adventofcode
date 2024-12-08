use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

pub fn run(content: &str) -> u32 {
    let re = Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    let mut pts: HashSet<Point> = HashSet::new();

    let beacons: Vec<(Point, Point)> = re
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

    let y_target : i32 = 10;
    // let y_target = 2000000;

    for (s, b) in beacons {
        let mut dist = hamilton(&s, &b) as i32;
        dist -= y_target.abs_diff(s.y) as i32;

        if dist < 0 {
            continue;
        }
        for x in -dist..=dist {
            let c = Point::new(s.x + x, y_target);
            if c != b {
                pts.insert(c);
            }
        }
    }

    pts.len() as u32
}
