// .XMASAMX
// 01234567
use rayon::prelude::*;
use std::iter;
#[inline(always)]
pub fn go(state: &mut u8, next: u8, cnt: &mut u32) {
    match (*state, next) {
        (1, b'M') => *state = 2,
        (2, b'A') => *state = 3,
        (3, b'S') => {
            *state = 4;
            *cnt += 1;
        }
        (4, b'A') => *state = 5,
        (5, b'M') => *state = 6,
        (6, b'X') => {
            *state = 1;
            *cnt += 1;
        }
        _ => match next {
            b'X' => *state = 1,
            b'S' => *state = 4,
            _ => *state = 0
        }
    }
}

// Average: 637.573µs
pub fn run1(content: &str) -> u32 {
    let n = content.lines().count() as i32;
    let m = content.lines().next().map_or(0, |line| line.len()) as i32;
    // let m = n;

    let rows = (0..n).map(|i| (i, 0i32, 0i32, 1i32));
    let cols = (0..m).map(|j| (0, j, 1, 0));
    let d_sw1 = (0..m).map(|j| (0, j, 1, -1));
    let d_sw2 = (1..n).map(|i| (i, m - 1, 1, -1));
    let d_se1 = (0..n).map(|i| (i, 0, 1, 1));
    let d_se2 = (1..m).map(|j| (0, j, 1, 1));
    let bytes = content.as_bytes();

    let all = iter::empty()
        .chain(rows)
        .chain(cols)
        .chain(d_sw1)
        .chain(d_sw2)
        .chain(d_se1)
        .chain(d_se2);
    all.par_bridge()
        .map(|(mut i, mut j, di, dj)| {
            let mut s: u8 = 0;
            let mut c = 0;

            while 0 <= i && i < n && 0 <= j && j < m {
                go(&mut s, bytes[(i * (m + 1) + j) as usize], &mut c);
                i += di;
                j += dj;
            }
            c
        })
        .sum()
}


pub fn run2(content: &str) -> u32 {
    let g: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let matches2 = |i: usize, j: usize, c1: char, c2: char| -> u32 {
        if i >= 1 && i <= g.len() && j >= 1 && j <= g[i - 1].len() {
            let c = g[i - 1][j - 1];
            if c == c1 {
                1
            } else if c == c2 {
                2
            } else {
                0
            }
        } else {
            0
        }
    };
    let matches = |i: usize, j: usize, c: char| -> u32 {
        matches2(i, j, c, c)
    };

    let mut ans = 0;
    for i in 1..=g.len() {
        for j in 1..=g.len() {
            if matches(i, j, 'A') == 0 {
                continue;
            }
            if matches2(i - 1, j - 1, 'M', 'S') + matches2(i + 1, j + 1, 'M', 'S') == 3 &&
                matches2(i - 1, j + 1, 'M', 'S') + matches2(i + 1, j - 1, 'M', 'S') == 3 {
                ans += 1;
            }
        }
    }

    // 1985
    ans
}


pub fn run(content: &str) -> u32 {
    run1(content)
    // 1: 2551
    // 2: 1985
}
