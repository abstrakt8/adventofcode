use bitvec::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering::{Equal, Less};

pub fn run1(content: &str) -> u32 {
    let mut it = content.split("\n\n").into_iter();
    let graph: &str = it.next().unwrap();
    let input: &str = it.next().unwrap();
    let mut bset = bitarr![0; 100000];

    graph.lines().for_each(|x| {
        let mut it = x.as_bytes().iter();
        let mut x: u32 = 0;
        loop {
            let Some(d) = it.next() else {
                break;
            };
            if d.is_ascii_digit() {
                x = x * 10 + (d & 15) as u32;
            } else {
                x = x * 10;
            }
        }
        bset.set(x as usize, true);
    });


    let contains = |x: u32, y: u32| -> bool {
        bset[(x * 1000 + y) as usize]
    };

    input.par_lines().map(|line| {
        let mut vec: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                if contains(vec[j], vec[i]) {
                    return 0;
                }
            }
        }
        return vec[vec.len() / 2];
    }).sum()
    // 6304
    // 6305
}

pub fn run2(content: &str) -> u32 {
    let mut it = content.split("\n\n").into_iter();
    let graph: &str = it.next().unwrap();
    let input: &str = it.next().unwrap();
    let mut bset = bitarr![0; 100000];

    graph.lines().for_each(|x| {
        let mut it = x.as_bytes().iter();
        let mut x: u32 = 0;
        loop {
            let Some(d) = it.next() else {
                break;
            };
            if d.is_ascii_digit() {
                x = x * 10 + (d & 15) as u32;
            } else {
                x = x * 10;
            }
        }
        bset.set(x as usize, true);
    });


    let contains = |x: u32, y: u32| -> bool {
        bset[(x * 1000 + y) as usize]
    };

    input.par_lines().map(|line| {
        let mut vec: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                if contains(vec[j], vec[i]) {
                    vec.sort_unstable_by(|x, y| if contains(*x, *y) {
                        Less
                    } else {
                        Equal
                    });
                    return vec[vec.len() / 2];
                }
            }
        }
        return 0;
    }).sum()
}


pub fn run(content: &str) -> u32 {
    // 6304
    // 6305
    run1(content)
}
