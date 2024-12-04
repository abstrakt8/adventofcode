// .XMASAMX
// 01234567
use rayon::prelude::*;

use std::iter;
const STATE_COUNT: usize = 7;
const CHAR_COUNT: usize = 256; // Covers all possible byte values

const fn init_next_state() -> [u8; STATE_COUNT * CHAR_COUNT] {
    let mut matrix = [0u8; STATE_COUNT * CHAR_COUNT];
    let mut i = 0;
    while i < STATE_COUNT * CHAR_COUNT {
        let state = i / CHAR_COUNT;
        let ch = (i % CHAR_COUNT) as u8;
        matrix[i] = match ch {
            b'X' => 1,
            b'S' => 4,
            _ => 0,
        };
        i += 1;
    }

    // Set specific transitions
    matrix[1 * CHAR_COUNT + b'M' as usize] = 2;
    matrix[2 * CHAR_COUNT + b'A' as usize] = 3;
    matrix[3 * CHAR_COUNT + b'S' as usize] = 4;
    matrix[4 * CHAR_COUNT + b'A' as usize] = 5;
    matrix[5 * CHAR_COUNT + b'M' as usize] = 6;
    matrix[6 * CHAR_COUNT + b'X' as usize] = 1;

    matrix
}

const fn init_count_increment() -> [u8; STATE_COUNT * CHAR_COUNT] {
    let mut matrix = [0u8; STATE_COUNT * CHAR_COUNT];

    // Set count increments
    matrix[3 * CHAR_COUNT + b'S' as usize] = 1;
    matrix[6 * CHAR_COUNT + b'X' as usize] = 1;

    matrix
}

static NEXT_STATE: [u8; STATE_COUNT * CHAR_COUNT] = init_next_state();
static COUNT_INCREMENT: [u8; STATE_COUNT * CHAR_COUNT] = init_count_increment();

#[inline(always)]
pub fn go(state: &mut u8, next: u8, cnt: &mut u32) {
    let index = (*state as usize) * CHAR_COUNT + (next as usize);
    *cnt += COUNT_INCREMENT[index] as u32;
    *state = NEXT_STATE[index];
}

// Average: 637.573µs
pub fn run1(content: &str) -> u32 {
    let n = content.lines().count() as i32;
    let m = content.lines().next().map_or(0, |line| line.len()) as i32;

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
    let n = content.lines().count() as i32;
    let m = content.lines().next().map_or(0, |line| line.len()) as i32;
    let bytes = content.as_bytes();

    let get = |i: usize, j: usize| -> u8 {
        bytes[i * (m as usize + 1) + j]
    };

    let bit = |i: usize, j: usize| -> u32 {
        match get(i, j) {
            b'M' => 1,
            b'S' => 2,
            _ => 0
        }
    };

    let ok = |i, j| -> u32 {
        (get(i, j) == b'A' &&
            (bit(i - 1, j - 1) | bit(i + 1, j + 1)) == 3 &&  //
            (bit(i - 1, j + 1) | bit(i + 1, j - 1)) == 3) as u32
    };


    // This is slower than expected, but probably because too many threads are spawned
    // (1..n - 1).cartesian_product(1..m - 1).par_bridge().map(|(i, j)| ok(i as usize, j as usize)).sum()

    let return_value = (1..n - 1).into_par_iter()
        .map(|i| {
            (1..m - 1).map(|j| ok(i as usize, j as usize)).sum::<u32>()
        }).sum();
    return_value
}


pub fn run(content: &str) -> u32 {
    run1(content)
    // 1: 2551
    // 2: 1985
}
