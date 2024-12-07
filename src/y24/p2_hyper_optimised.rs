use rayon::prelude::*;

const INCREASING: u8 = 1;
const DECREASING: u8 = 2;
const ALL: u8 = INCREASING | DECREASING;
const JOKER: i32 = -1;
const CAT: i32 = -2;

#[inline(always)]
fn bitcoin(a: i32, b: i32) -> u8 {
    if a == JOKER || b == JOKER {
        return ALL;
    }
    let diff = (a - b).abs();
    if !(1..=3).contains(&diff) {
        0
    } else if a < b {
        INCREASING
    } else {
        DECREASING
    }
}

pub fn run(content: &str) -> u32 {
    unsafe {
        content.par_lines().map(|line| {
            let mut a: u8 = ALL;
            let mut b: u8 = ALL;
            let mut c: u8 = ALL;
            let mut prev_prev: i32 = CAT;
            let mut prev: i32 = JOKER;
            let bytes = line.as_bytes();
            let mut i = 0;
            let len = bytes.len();

            while i < len {
                let mut cur = 0;
                while i < len && bytes[i] >= b'0' && bytes[i] <= b'9' {
                    cur = cur * 10 + (bytes[i] - b'0') as i32;
                    i += 1;
                }
                i += 1;

                c &= bitcoin(prev, cur);
                if prev_prev != CAT {
                    c |= bitcoin(prev_prev, cur) & a;
                }
                a = b;
                b &= bitcoin(prev, cur);
                if c | a == 0 {
                    return 0;
                }
                prev_prev = prev;
                prev = cur;
            }

            c &= bitcoin(prev, JOKER);
            if prev_prev != CAT {
                c |= bitcoin(prev_prev, JOKER) & a;
            }

            (c != 0) as u32
        }).sum()
    }
}

