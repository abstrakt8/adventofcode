use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{once, repeat_with};

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a & (16777216 - 1) // 2^24 - 1
}

fn next_secret(a: u64) -> u64 {
    let a = prune(mix(a, a << 6));
    let a = prune(mix(a, a >> 5));
    let a = prune(mix(a, a << 11));
    a
}

pub fn run(content: &str) -> impl Debug {
    let initial: Vec<u64> = content.lines().filter_map(|s| s.parse().ok()).collect();

    let all_secrets: Vec<Vec<_>> = initial
        .into_iter()
        .map(|mut s| {
            once(s)
                .chain(
                    repeat_with(move || {
                        s = next_secret(s);
                        s
                    })
                    .take(2000),
                )
                .collect::<Vec<_>>()
        })
        .collect();

    let ans1: u64 = all_secrets.iter().map(|secrets| secrets[2000]).sum();

    let mut global: HashMap<Vec<i32>, u32> = HashMap::new();
    for secrets in all_secrets {
        let mut local: HashMap<Vec<i32>, u32> = HashMap::new();

        let mut prev = 0;
        let mut seq = Vec::new();

        for (i, s) in secrets.iter().enumerate() {
            let cur = (*s % 10) as u32;
            if i > 0 {
                let d = (cur as i32) - (prev as i32);
                seq.push(d);
                if seq.len() > 4 {
                    seq.remove(0);
                }
                if seq.len() == 4 {
                    local
                        .entry(seq.to_owned())
                        .or_insert(cur);
                }
            }
            prev = cur;
        }

        for (def, max_val) in local {
            global
                .entry(def)
                .and_modify(|v| *v += max_val)
                .or_insert(max_val);
        }
    }

    let ans2 = global.iter().map(|(a, b)| (*b, a.clone())).max().unwrap();

    (ans1, ans2)
}
