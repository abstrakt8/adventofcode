use std::collections::HashSet;

fn solve(bytes: &[u8], len: usize) -> usize {
    bytes
        .windows(len)
        .enumerate()
        .filter_map(|(i, b)| {
            let s: HashSet<u8> = HashSet::from_iter(b.into_iter().copied());
            if s.len() == len {
                Some(i + len)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

pub fn run(content: &str) -> (usize, usize) {
    let bytes = content.as_bytes();
    let ans1 = solve(bytes, 4);
    let ans2 = solve(bytes, 14);
    (ans1, ans2)
}
