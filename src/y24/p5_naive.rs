use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
struct Node {
    inc: HashSet<u32>,
    out: Vec<u32>,

    order: u32,
}

pub fn run_dag(content: &str) -> u32 {
    let mut it = content.split("\n\n");

    let graph = it.next().unwrap();
    let input = it.next().unwrap();

    let mut map: HashMap<u32, Node> = HashMap::default();
    println!("{}", graph);
    println!("{}", input);

    graph.lines().for_each(|line| {
        let v: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
        map.entry(v[0]).or_default().out.push(v[1]);
        map.entry(v[1]).or_default().inc.insert(v[0]);

        println!("{}", map[&v[0]].inc.len());
    });

    let mut q: VecDeque<u32> = VecDeque::new();
    for (u, v) in &map {
        if v.inc.is_empty() {
            q.push_back(*u);
        }
    }

    let mut cnt = 0;
    loop {
        let Some(u) = q.pop_front() else {
            break;
        };

        let out_len = map[&u].out.len();
        for i in 0..out_len {
            let v = map[&u].out[i];
            if let Some(node) = map.get_mut(&v) {
                node.inc.remove(&u);
                if node.inc.is_empty() {
                    q.push_back(v);
                }
            }
        }

        map.entry(u).and_modify(|node| {
            node.order = cnt;
        });
        cnt += 1;
    }

    let ans: u32 = input.lines().map(|line| {
        let vec: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();

        let mut flag = true;
        for i in 1..vec.len() {
            if map[&vec[i - 1]].order >= map[&vec[i]].order {
                flag = false;
                break;
            }
        }

        if flag {
            vec[vec.len() / 2]
        } else {
            0u32
        }
    }).sum();
    ans
}

pub fn run_1(content: &str) -> u32 {
    let mut it = content.split("\n\n");

    let graph = it.next().unwrap();
    let input = it.next().unwrap();

    let mut map: HashSet<(u32, u32)> = Default::default();

    graph.lines().for_each(|line| {
        let v: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
        map.insert((v[0], v[1]));
    });

    let ans: u32 = input.lines().map(|line| {
        let vec: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();

        let mut flag = true;
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                if map.contains(&(vec[j], vec[i])) {
                    flag = false;
                    break;
                }
            }
        }

        if flag {
            vec[vec.len() / 2]
        } else {
            0u32
        }
    }).sum();
    ans
}

pub fn run(content: &str) -> u32 {
    let mut it = content.split("\n\n");

    let graph = it.next().unwrap();
    let input = it.next().unwrap();

    let mut map: HashSet<(u32, u32)> = Default::default();

    graph.lines().for_each(|line| {
        let v: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
        map.insert((v[0], v[1]));
    });

    let ans: u32 = input.lines().map(|line| {
        let mut vec: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();

        let mut flag = true;
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                if map.contains(&(vec[j], vec[i])) {
                    flag = false;
                    break;
                }
            }
        }

        if !flag {
            vec.sort_by(|a, b| {
                if map.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            vec[vec.len() / 2]
        } else {
            0
        }
    }).sum();
    ans
}