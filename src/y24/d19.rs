use std::fmt::Debug;

pub fn run(content: &str) -> impl Debug {
    let (patterns, desired) = content.trim().split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let desired_design: Vec<&str> = desired.split("\n").collect();

    let mut ans1 = 0i64;
    let mut ans2 = 0;
    for design in desired_design {
        let mut dp = vec![0i64; design.len() + 1];

        dp[0] = 1;
        for i in 0..design.len() {
            if dp[i] == 0 {
                continue;
            }
            for pattern in &patterns {
                if design[i..].starts_with(pattern) {
                    dp[i + pattern.len()] += dp[i];
                }
            }
        }
        ans1 += (dp[design.len()] > 0) as i64;
        ans2 += dp[design.len()];
    }
    (ans1, ans2)
}
