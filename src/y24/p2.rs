use rayon::prelude::*;

// Bitmask represents whether it's still possible to keep the sequence increasing / decreasing if the bit is on
const INCREASING: u8 = 1 << 0;
const DECREASING: u8 = 1 << 1;
const ALL: u8 = INCREASING | DECREASING;
const JOKER: i32 = -1;
const CAT: i32 = -2;

#[inline(always)]
fn bit(a: i32, b: i32) -> u8 {
    if !(1..=3).contains(&(a - b).abs()) {
        0
    } else if a < b {
        INCREASING
    } else {
        DECREASING
    }
}

#[inline(always)]
fn bitcoin(a: i32, b: i32) -> u8 {
    if a == JOKER || b == JOKER {
        ALL
    } else {
        bit(a, b)
    }
}

pub fn run(content: &str) -> u32 {
    // a = i - 2, 0
    // b = i - 1, 0
    // c = i - 1, 1
    let res: u32 = content.par_lines().map(|line| {
        let mut a: u8 = ALL;
        let mut b: u8 = ALL;
        let mut c: u8 = ALL;
        let mut prev_prev: i32 = CAT;
        let mut prev: i32 = JOKER;
        let it = line.split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        for cur in it.chain([JOKER]) {
            c &= bitcoin(prev, cur);
            if prev_prev != CAT {
                c |= bitcoin(prev_prev, cur) & a;
            }
            a = b;
            b &= bitcoin(prev, cur);
            if c | a == 0 {
                break;
            }
            [prev_prev, prev] = [prev, cur];
        }
        let res = (c != 0) as u32;
        // println!("{line} {res}");
        res
    }).sum();
    // 536
    res
}
