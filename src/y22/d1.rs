pub fn run(content: &str) -> (u32, u32) {
    let mut v: Vec<u32> = content
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|x| {
                    
                    x.parse::<u32>().unwrap()
                })
                .sum()
        })
        .collect();
    v.sort();
    let n = v.len();
    let ans1 = v[n - 1];
    let ans2 = v[n - 3..n].iter().sum();
    (ans1, ans2)
}
