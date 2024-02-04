// Helper functions
fn parse_vec(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn in_range(a: u64, b: u64, c: u64) -> bool {
    return a <= c && c <= b;
}

pub fn run(input: &str) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut seeds = parse_vec(sections[0].split(": ").last().unwrap());

    sections[1..].iter().for_each(|s| {
        let ranges: Vec<_> = s.lines().skip(1).map(parse_vec).collect();

        for it in seeds.iter_mut() {
            for ra in &ranges {
                let (dst, src, delta) = (ra[0], ra[1], ra[2]);
                if in_range(src, src + delta - 1, *it) {
                    *it = dst + (*it - src);
                    break;
                }
            }
            // In case not found, we can just do nothing because it should be mapping to the same element
        }
    });
    let ans = seeds.iter().min().unwrap();
    return *ans;
}