use std::mem::swap;

#[derive(Debug)]
struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    pos: (i64, i64),
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let numbers: Vec<Vec<i64>> = value
            .lines()
            .map(|line| {
                let line = line.split_once(":").unwrap().1;
                let line: Vec<i64> = line
                    .split(", ")
                    .map(|token| {
                        let token = token.split_once(&['+', '=']).unwrap().1;
                        token.parse().unwrap()
                    })
                    .collect();
                line
            })
            .collect();

        Self {
            a: (numbers[0][0], numbers[0][1]),
            b: (numbers[1][0], numbers[1][1]),
            pos: (numbers[2][0], numbers[2][1]),
        }
    }
}

const MAX: i64 = 100;
impl ClawMachine {
    pub fn solve1(&self) -> Option<i64> {
        let (a1, a2) = self.a;
        let (b1, b2) = self.b;
        let (c1, c2) = self.pos;

        let mut ans = None;

        for i in 0..=MAX {
            let (r1, r2) = (c1 - a1 * i, c2 - a2 * i);

            if r1 >= 0 && r2 >= 0 {
                let k = r1 / b1;

                if k * b1 != r1 || k * b2 != r2 {
                    continue;
                }

                if k > MAX {
                    continue;
                }

                let required_tokens = i * 3 + k;

                if required_tokens < ans.unwrap_or(i64::MAX) {
                    ans = Some(required_tokens);
                }
            }
        }
        ans
    }

    // These are symmetrical cases, but tokens are just different
    pub fn solve_case(&self, should_swap: bool) -> Option<i64> {
        let (mut x1, mut x2) = self.a;
        let (mut y1, mut y2) = self.b;
        let (mut z1, mut z2) = self.pos;

        z1 += 10000000000000;
        z2 += 10000000000000;

        if should_swap {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
            swap(&mut z1, &mut z2);
        }

        if x1 == 0 {
            return None;
        }

        let d = x1 * y2 - x2 * y1;

        if d == 0 {
            return None;
        }

        let n = x1 * z2 - x2 * z1;
        if n % d != 0 {
            return None;
        }
        let b = n / d;

        if b >= 0 && (z1 - y1 * b) % x1 == 0 {
            let a = (z1 - y1 * b) / x1;
            Some(a * 3 + b * 1)
        } else {
            None
        }
    }

    pub fn solve2(&self) -> Option<i64> {
        [false, true]
            .map(|b| self.solve_case(b))
            .into_iter()
            .flatten()
            .min()
    }
}

pub fn run_all(content: &str) -> (i64, i64) {
    let v: Vec<ClawMachine> = content.split("\n\n").map(ClawMachine::from).collect();
    let ans1: i64 = v.iter().filter_map(ClawMachine::solve1).sum();
    let ans2: i64 = v.iter().filter_map(ClawMachine::solve2).sum();
    (ans1, ans2)
}

pub fn run(content: &str) -> i64 {
    let v: Vec<ClawMachine> = content.split("\n\n").map(ClawMachine::from).collect();
    let ans1: i64 = v.iter().filter_map(ClawMachine::solve2).sum();
    ans1
}
