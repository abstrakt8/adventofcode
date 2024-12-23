use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn parse_pair(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(i)
}

pub fn run(content: &str) -> impl Debug {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut connected: HashSet<(&str, &str)> = HashSet::new();
    for line in content.lines() {
        let (_, (a, b)) = parse_pair(line).unwrap();
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
        connected.insert((a, b));
        connected.insert((b, a));
    }

    let mut answers: HashSet<Vec<&str>> = Default::default();

    let mut ans1 = 0;
    for (v1, adj) in &map {
        if !v1.starts_with("t") {
            continue;
        }
        for i in 0..adj.len() {
            for j in i + 1..adj.len() {
                if connected.contains(&(adj[i], adj[j])) {
                    let mut v = vec![v1, adj[i], adj[j]];
                    v.sort();
                    answers.insert(v);
                }
            }
        }
    }
    ans1 = answers.len();

    let mut ans2: Vec<&str> = vec![];

    for (&v1, adj) in &map {
        let len = adj.len();
        'mask: for mask in 0u32..(1 << len) {
            if mask.count_ones() < ans2.len() as u32 {
                continue;
            }

            let mut set = vec![v1];
            for i in 0..len {
                if (mask >> i) & 1 == 0 {
                    continue;
                }
                set.push(&adj[i]);
                for j in i + 1..len {
                    if (mask >> j) & 1 == 0 {
                        continue;
                    }
                    if !connected.contains(&(adj[i], adj[j])) {
                        continue 'mask;
                    }
                }
            }
            // All connected
            ans2 = set;
        }
    }
    
    ans2.sort();
    let ans2 = ans2.join(",");
    (ans1, ans2)
}
