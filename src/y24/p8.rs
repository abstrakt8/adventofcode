use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type Grid<'a> = Vec<&'a [u8]>;

fn mark_anti_nodes(grid: &Grid, harmonics: bool) -> HashSet<Point> {
    let mut map: HashMap<u8, Vec<Point>> = HashMap::new();
    let n = grid.len();
    let m = grid[0].len();
    let mut nodes: HashSet<Point> = HashSet::new();
    grid.iter().enumerate().for_each(|(i, &row)| {
        row.iter().enumerate().for_each(|(j, &b)| {
            if b.is_ascii_alphanumeric() {
                map.entry(b)
                    .or_default()
                    .push(Point::new(i as i32, j as i32));
            }
        });
    });

    map.iter().for_each(|(_, v)| {
        // v.iter().tuple_combinations::<(_, _)>().for_each(|(a, b)| {
        // })
        // TODO: Check efficiency?
        v.iter().cartesian_product(v).for_each(|(&x, &y)| {
            if x == y {
                return;
            }
            let d = x - y;
            let mut p = x;

            while (0..n as i32).contains(&p.x) && (0..m as i32).contains(&p.y) {
                nodes.insert(p);
                p = p + d;
                if !harmonics {
                    return;
                }
            }
        });
    });

    nodes
}

pub fn run(content: &str) -> (u32, u32) {
    let grid: Vec<&[u8]> = content.lines().map(&str::as_bytes).collect();
    let nodes1 = mark_anti_nodes(&grid, false);
    let nodes2 = mark_anti_nodes(&grid, true);
    let ans1 = nodes1.len();
    let ans2 = nodes2.len();

    (ans1 as u32, ans2 as u32)
}
