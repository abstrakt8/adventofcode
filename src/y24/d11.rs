use std::collections::HashMap;

#[derive(Default)]
struct Solver {
    cache: HashMap<(u64, u8), u64>,
}

// Every odd-digit number starting with 5..9 would become an odd number again.
// Almost every odd-digit number that starts with digit 1..3 should become even.
// For 4, only the top numbers would become an odd-digit number again e.g. 495 * 2024
// Reason is that 2024 = 1000 * 2.024 increases the number of digits by 3 and potentially by 4
// if the first digit is 4..9.
// My guess is that although it might seem exponential, the number of distinct numbers is probably
// linear to the highest number in the input
impl Solver {
    pub fn solve(&mut self, x: u64, retries: u8) -> u64 {
        // dbg!(x, retries);
        if retries == 0 {
            1
        } else if let Some(ans) = self.cache.get(&(x, retries)) {
            *ans
        } else {
            let ans = {
                if x == 0 {
                    self.solve(1, retries - 1)
                } else {
                    let log10 = x.ilog10();
                    if (log10 + 1) % 2 == 0 {
                        let factor = u64::pow(10, (log10 + 1) / 2);
                        let x1 = x / factor;
                        let x2 = x % factor;
                        self.solve(x1, retries - 1) + self.solve(x2, retries - 1)
                    } else {
                        self.solve(x * 2024, retries - 1)
                    }
                }
            };

            self.cache.insert((x, retries), ans);
            ans
        }
    }
}

pub fn solve(content: &str, retries: u8) -> u64 {
    let numbers: Vec<u64> = content
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut solver = Solver::default();
    numbers.iter().map(|x| solver.solve(*x, retries)).sum()
}

pub fn run1(content: &str) -> u64 {
    let retries = 25;
    solve(content, retries)
}

pub fn run2(content: &str) -> u64 {
    let retries = 75;
    solve(content, retries)
}

pub fn run(content: &str) -> u64 {
    run1(content)
}
