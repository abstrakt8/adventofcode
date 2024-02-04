use std::cmp::{max, min};

// Helper functions
fn parse_vec(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn in_range(a: u64, b: u64, c: u64) -> bool {
    return a <= c && c <= b;
}

type Interval = [u64; 2];

fn intersect(a: Interval, b: Interval) -> Option<Interval> {
    let c = [max(a[0], b[0]), min(a[1], b[1])];
    if c[0] <= c[1] { Some(c) } else { None }
}


pub fn run(input: &str) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut int_list = parse_vec(sections[0].split(": ").last().unwrap());
    let mut intervals: Vec<Interval> = int_list.chunks(2).map(|c| [c[0], c[0] + c[1] - 1]).collect();

    sections[1..].iter().for_each(|s| {
        let mut ranges: Vec<_> = s.lines().skip(1).map(parse_vec).collect();
        ranges.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut next_intervals = Vec::new();
        for it in &intervals {
            let mut cur = it.clone();
            for ra in &ranges {
                let (dst, src, delta) = (ra[0], ra[1], ra[2]);
                if let Some(inter) = intersect(cur, [src, src + delta - 1]) {
                    if cur[0] < inter[0] {
                        next_intervals.push([cur[0], inter[0] - 1]);
                    }
                    next_intervals.push([dst + inter[0] - src, dst + inter[1] - src]);
                    cur = [inter[1] + 1, cur[1]];

                    if cur[0] > cur[1] {
                        break;
                    }
                }
            }
            if cur[0] <= cur[1] {
                next_intervals.push(cur)
            }
        }
        intervals = next_intervals;
    });

    return intervals.iter().map(|f| f[0]).min().unwrap();
}