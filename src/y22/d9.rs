use std::cmp::max;
use std::collections::HashSet;

const DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn str2dir(s: &str) -> usize {
    match s {
        "R" => 0,
        "D" => 1,
        "L" => 2,
        "U" => 3,
        _ => unreachable!(),
    }
}

type Pos = (i32, i32);

fn chess(a: Pos, b: Pos) -> u32 {
    max(a.0.abs_diff(b.0), a.1.abs_diff(b.1))
}

fn go(a: Pos, d: usize) -> Pos {
    (a.0 + DIRS[d][0], a.1 + DIRS[d][1])
}

fn catch(t: Pos, h: Pos) -> Option<Pos> {
    (0..4)
        .filter_map(|d| {
            let x = go(h, d);
            if chess(x, t) <= 1 {
                Some(x)
            } else {
                None
            }
        })
        .next()
}

pub fn solve(content: &str, len: usize) -> u32 {
    let mut vis: HashSet<Pos> = HashSet::new();
    let mut pos = vec![(0, 0); len];

    vis.insert((0, 0));

    content.lines().for_each(|line| {
        let (dir, a) = line.split_once(" ").unwrap();
        let a: usize = a.parse().unwrap();
        let d: usize = str2dir(dir);

        for _ in 0..a {
            let mut new = vec![go(pos[0], d)];
            for i in 1..len {
                if chess(pos[i], new[i - 1]) <= 1 {
                    new.extend(pos.drain(i..));
                    break;
                } else {
                    new.push(catch(pos[i], new[i - 1]).unwrap_or(pos[i - 1]));
                }
            }
            pos = new;
            vis.insert(*pos.last().unwrap());
        }
    });

    vis.len() as u32
}

pub fn run(content: &str) -> (u32, u32) {
    // 6642,
    (solve(content, 2), solve(content, 10))
}
