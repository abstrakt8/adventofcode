use itertools::Itertools;

fn priority(b: u8) -> u32 {
    if b.is_ascii_lowercase() {
        1 + (b - b'a') as u32
    } else if b.is_ascii_uppercase() {
        27 + (b - b'A') as u32
    } else {
        unreachable!()
    }
}
pub fn run(content: &str) -> (u32, u32) {
    let ans1 = content
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let n = bytes.len();
            let first = &bytes[0..n / 2];
            let second = &bytes[n / 2..n];
            for it in first.iter() {
                if second.contains(it) {
                    return priority(*it);
                }
            }
            unreachable!("?")
        })
        .sum();

    let ans2 = content
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let v: Vec<&str> = chunk.collect();
            let c = (b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .find_or_first(|c| v.iter().all(|line| line.as_bytes().contains(c)))
                .expect("Not found");
            priority(c)
        })
        .sum();

    (ans1, ans2)
}
