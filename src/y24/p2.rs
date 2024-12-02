
pub fn run(content: &str) -> u32 {
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
