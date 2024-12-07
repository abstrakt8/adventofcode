use rayon::prelude::*;
use std::collections::HashSet;

pub enum Part {
    Part1,
    Part2,
}

fn parse(line: &str) -> (u64, Vec<u64>) {
    line.split_once(':')
        .and_then(|(num, input)| {
            let num = num.trim().parse().ok()?;
            let vecs = input
                .split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect();
            Some((num, vecs))
        })
        .expect("Input not valid!")
}

struct Solver {
    expected: u64,
    vec: Vec<u64>,
    map: HashSet<(u64, usize)>,
}

pub fn concat(mut a: u64, mut b: u64) -> u64 {
    let mut v: Vec<u64> = Vec::with_capacity(10);
    while b > 0 {
        v.push(b % 10);
        b /= 10;
    }
    for it in v.iter().rev() {
        a = a * 10 + *it;
    }
    a
}

impl Solver {
    pub fn new(expected: u64, vec: Vec<u64>) -> Self {
        Self {
            expected,
            vec,
            map: Default::default(),
        }
    }
    pub fn solve1(&self) -> bool {
        let n = self.vec.len();
        for mask in 0..1 << (n - 1) {
            let mut cur: u64 = self.vec[0];
            for j in 1..n {
                if ((mask >> (j - 1)) & 1) > 0 {
                    cur *= self.vec[j];
                } else {
                    cur += self.vec[j];
                }
                if cur > self.expected {
                    break;
                }
            }
            if cur == self.expected {
                return true;
            }
        }
        false
    }

    pub fn solve2(&mut self) -> bool {
        self.recurse(self.vec[0], 1)
    }

    pub fn recurse(&mut self, cur: u64, i: usize) -> bool {
        if cur > self.expected {
            return false;
        }

        if i == self.vec.len() {
            return self.expected == cur;
        }

        if !self.map.insert((cur, i)) {
            return false;
        }

        if self.recurse(cur * self.vec[i], i + 1) {
            return true;
        }

        if self.recurse(cur + self.vec[i], i + 1) {
            return true;
        }

        if self.recurse(concat(cur, self.vec[i]), i + 1) {
            return true;
        }
        false
    }
}

pub fn run_part(content: &str, part: Part) -> u64 {
    content
        .par_lines()
        .map(|line| {
            let (expected, input) = parse(line);
            let mut solver = Solver::new(expected, input);

            let ok = match part {
                Part::Part1 => solver.solve1(),
                Part::Part2 => solver.solve2(),
            };

            if ok {
                expected
            } else {
                0
            }
        })
        .sum()
}

pub fn run(content: &str) -> u64 {
    // run_part(content, Part::Part1);
    run_part(content, Part::Part2)
}
