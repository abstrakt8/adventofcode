fn parse_vec(input: &str) -> Vec<u32> {
    input.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub(crate) fn run(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let time = parse_vec(lines[0].split(':').last().unwrap().trim());
    let dist = parse_vec(lines[1].split(':').last().unwrap().trim());
    // eprintln!(time);
    // dbg!(time);

    let mut ans: u64 = 1;
    for i in 0..time.len() {
        let (t, d) = (time[i], dist[i]);
        let mut cnt = 0;
        for j in 1..t {
            let m = (t - j) * j;
            if m > d {
                cnt += 1;
            }
        }
        ans *= cnt as u64;
    }

    return ans;
}