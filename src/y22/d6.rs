use std::collections::HashSet;

fn solve(bytes: &[u8], len: usize) -> usize {
    bytes
        .windows(len)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == len)
        .map_or(0, |pos| pos + len)
}

pub fn run(content: &str) -> (usize, usize) {
    let bytes = content.as_bytes();
    let ans1 = solve(bytes, 4);
    let ans2 = solve(bytes, 14);
    (ans1, ans2)
}
