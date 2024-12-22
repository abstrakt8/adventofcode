use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{once, repeat_with};

fn mix(a: u32, b: u32) -> u32 {
    a ^ b
}

fn prune(a: u32) -> u32 {
    a & (16777216 - 1) // 2^24 - 1
}

fn next_secret(a: u32) -> u32 {
    let a = prune(mix(a, a << 6));
    let a = prune(mix(a, a >> 5));
    let a = prune(mix(a, a << 11));
    a
}

struct Solver {
    data: Vec<Vec<u32>>,
}

impl Solver {
    pub fn from_input(content: &str) -> Self {
        let initial: Vec<u32> = content.lines().filter_map(|s| s.parse().ok()).collect();

        let data: Vec<Vec<_>> = initial
            .into_iter()
            .map(|mut s| {
                once(s)
                    .chain(
                        repeat_with(move || {
                            s = next_secret(s);
                            s
                        })
                        .take(2000),
                    )
                    .collect::<Vec<_>>()
            })
            .collect();

        Self { data }
    }

    pub fn part1(&self) -> u64 {
        self.data.iter().map(|secrets| secrets[2000] as u64).sum()
    }

    pub fn part2(&self) -> u32 {
        let mut global: HashMap<Vec<i32>, u32> = HashMap::new();
        for secrets in self.data.iter() {
            let mut local: HashMap<Vec<i32>, u32> = HashMap::new();

            let mut prev = 0;
            let mut seq = Vec::new();

            for (i, s) in secrets.iter().enumerate() {
                let cur = (*s % 10);
                if i > 0 {
                    let d = (cur as i32) - (prev as i32);
                    seq.push(d);
                    if seq.len() > 4 {
                        seq.remove(0);
                    }
                    if seq.len() == 4 {
                        local.entry(seq.to_owned()).or_insert(cur);
                    }
                }
                prev = cur;
            }

            for (def, max_val) in local {
                global
                    .entry(def)
                    .and_modify(|v| *v += max_val)
                    .or_insert(max_val);
            }
        }

        *global.values().max().unwrap()
    }
}

pub fn run1(content: &str) -> u64 {
    let solver = Solver::from_input(content);
    solver.part1()
}

pub fn run2(content: &str) -> u32 {
    let solver = Solver::from_input(content);
    solver.part2()
}

pub fn run(content: &str) -> impl Debug {
    let solver = Solver::from_input(content);
    let ans1 = solver.part1();
    let ans2 = solver.part2();
    (ans1, ans2)
}
