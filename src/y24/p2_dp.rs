use std::cmp::{max, min};

pub fn run_1(content: &str) -> u32 {
    let res: u32 = content.lines().map(|line| {
        let v: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let state = v.windows(2).fold(0, |state, sub| {
            let diff = sub[1].abs_diff(sub[0]);
            if diff == 0 || diff > 3 {
                3
            } else if sub[0] < sub[1] {
                state | (1 << 0)
            } else {
                state | (1 << 1)
            }
        });
        (state != 3) as u32
    }).sum();
    res
}

#[inline(always)]
fn bit(a: u32, b: u32) -> u8 {
    if a == b || a.abs_diff(b) > 3 {
        0
    } else if a < b {
        1
    } else {
        2
    }
}

// i < j
#[inline(always)]
fn bitcoin(i: usize, j: usize, v: &[u32]) -> u8 {
    if i == 0 || v.len() < j {
        3
    } else {
        bit(v[i - 1], v[j - 1])
    }
}

pub fn run(content: &str) -> u32 {
    let res: u32 = content.lines().map(|line| {
        let v: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let n = v.len();

        // dp[i][k] = Whether subarray [0..i] is safe if we can skip k elements
        let mut dp: Vec<[u8; 2]> = vec![[3, 3]; n + 2];

        for i in 1..=n + 1 {
            // Don't skip
            dp[i][0] = dp[i - 1][0] & bitcoin(i - 1, i, &v);

            let mut foo =  dp[i - 1][1] & bitcoin(i - 1, i, &v);
            // Skip if possible
            if i >= 2 {
                foo |= dp[i - 2][0] & bitcoin(i - 2, i, &v);
            }
            dp[i][1] = foo;
        }
        // Skipping at least one is always better (you can skip the first element for example)
        let res  = (dp[n + 1][1] != 0) as u32;
        println!("{line} {res}");

        res
    }).sum();
    // 536
    res
}

