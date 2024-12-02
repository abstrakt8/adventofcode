use std::cmp::min;

// pub fn run_1(content: &str) -> u32 {
//     let res: u32 = content.lines().map(|line| {
//         let v: Vec<u32> = line.split_whitespace()
//             .map(|x| x.parse::<u32>().unwrap())
//             .collect();
//
//         let state = v.windows(2).fold(0, |state, sub| {
//             let diff = sub[1].abs_diff(sub[0]);
//             if diff == 0 || diff > 3 {
//                 3
//             } else if sub[0] < sub[1] {
//                 state | (1 << 0)
//             } else {
//                 state | (1 << 1)
//             }
//         });
//         (state != 3) as u32
//     }).sum();
//     res
// }

#[inline(always)]
fn bit(a: u32, b: u32) -> u8 {
    if a == b || a.abs_diff(b) > 3 {
        3
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
        0
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
        let mut dp: Vec<[u8; 2]> = vec![[0, 0]; n + 2];

        for i in 1..=n + 1 {
            dp[i][0] = bitcoin(i - 1, i, &v) | dp[i - 1][0];
            if i > 2 {
                // Don't skip
                dp[i][1] = min(dp[i - 1][1] | bitcoin(i - 1, i, &v),
                               dp[i - 2][0] | bitcoin(i - 2, i, &v),
                );
            }
        }
        // Skipping at least one is always better (you can skip the first element for example)
        (dp[n + 1][1] != 3) as u32
    }).sum();
    // 536
    res
}
