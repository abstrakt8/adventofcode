use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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

fn pos_delta_signum(a: Point, b: Point) -> Point {
    Point::new((a.x - b.x).signum(), (a.y - b.y).signum())
}

#[derive(Default, Clone)]
struct Grid {
    blocked: HashSet<Point>,
    lowest: HashMap<i32, i32>,
    all_lowest: i32,
}

impl Grid {
    fn add_block(&mut self, p: Point) {
        self.blocked.insert(p);
        self.lowest
            .entry(p.x)
            .and_modify(|y| {
                *y = max(*y, p.y);
            })
            .or_insert(p.y);
        self.all_lowest = max(self.all_lowest, p.y);
    }

    fn is_blocked(&self, p: &Point) -> bool {
        self.blocked.contains(p)
    }

    fn is_blocked_or_floor(&self, p: &Point) -> bool {
        self.is_blocked(p) || p.y == self.all_lowest + 2
    }

    fn simulate_place1(&mut self, mut cur: Point) -> bool {
        loop {
            if self.lowest.get(&cur.x).is_none_or(|h| *h < cur.y) {
                return true;
            }

            let dir = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)]
                .into_iter()
                .find(|&d| {
                    let nxt = cur + d;
                    !self.is_blocked(&nxt)
                });

            if let Some(dir) = dir {
                cur = cur + dir;
            } else {
                self.blocked.insert(cur);
                return false;
            }
        }
    }
    fn solve_1(&mut self) -> u32 {
        let origin = Point::new(500, 0);

        let mut ans1 = 0;
        while !self.simulate_place1(origin) {
            ans1 += 1;
        }

        ans1
    }

    fn simulate_place2(&mut self, mut cur: Point) -> bool {
        if self.is_blocked_or_floor(&cur) {
            return true;
        }
        loop {
            let dir = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)]
                .into_iter()
                .find(|&d| {
                    let nxt = cur + d;
                    !self.is_blocked_or_floor(&nxt)
                });

            if let Some(dir) = dir {
                cur = cur + dir;
            } else {
                self.blocked.insert(cur);
                return false;
            }
        }
    }
    fn solve_2(&mut self) -> u32 {
        let origin = Point::new(500, 0);

        let mut ans1 = 0;
        while !self.simulate_place2(origin) {
            ans1 += 1;
        }

        ans1
    }
}

pub fn run(content: &str) -> (u32, u32) {
    let mut grid: Grid = Grid::default();
    content.lines().for_each(|line| {
        let v = line
            .split(" -> ")
            .map(|s| {
                let d = s
                    .split(",")
                    .filter_map(|x| x.parse().ok())
                    .collect::<Vec<i32>>();
                match d.as_slice() {
                    [a, b] => Point::new(*a, *b),
                    _ => panic!(),
                }
            })
            .collect::<Vec<_>>();

        v.windows(2).for_each(|x| {
            let mut cur = x[0];
            let nxt = x[1];
            let dir = pos_delta_signum(nxt, cur);

            grid.add_block(cur);
            while cur != nxt {
                cur = cur + dir;
                grid.add_block(cur);
            }
        })
    });

    let ans1 = grid.clone().solve_1();
    let ans2 = grid.solve_2();

    (ans1, ans2)
}
