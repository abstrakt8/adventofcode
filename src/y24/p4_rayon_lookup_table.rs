// .XMASAMX
// 01234567
use rayon::prelude::*;

use memchr::memchr_iter;
use std::iter;

const STATE_COUNT: usize = 7;
const CHAR_COUNT: usize = 4; // Only X, M, A, S

const fn init_char_to_index() -> [usize; 256] {
    let mut table = [3usize; 256]; // Default all to 3 (index for 'S' and others)
    table[b'X' as usize] = 0;
    table[b'M' as usize] = 1;
    table[b'A' as usize] = 2;
    table
}

static CHAR_TO_INDEX: [usize; 256] = init_char_to_index();

#[inline(always)]
fn char_to_index(ch: u8) -> usize {
    CHAR_TO_INDEX[ch as usize]
}


const fn init_next_state() -> [u8; STATE_COUNT * CHAR_COUNT] {
    let mut matrix = [0u8; STATE_COUNT * CHAR_COUNT];
    let mut state = 0;
    while state < STATE_COUNT {
        let mut char_index = 0;
        while char_index < CHAR_COUNT {
            matrix[state * CHAR_COUNT + char_index] = match char_index {
                0 => 1, // 'X'
                3 => 4, // 'S'
                _ => 0,
            };
            char_index += 1;
        }
        state += 1;
    }

    // Set specific transitions
    matrix[1 * CHAR_COUNT + 1] = 2; // 'M'
    matrix[2 * CHAR_COUNT + 2] = 3; // 'A'
    matrix[3 * CHAR_COUNT + 3] = 4; // 'S'
    matrix[4 * CHAR_COUNT + 2] = 5; // 'A'
    matrix[5 * CHAR_COUNT + 1] = 6; // 'M'
    matrix[6 * CHAR_COUNT + 0] = 1; // 'X'

    matrix
}

const fn init_count_increment() -> [u8; STATE_COUNT * CHAR_COUNT] {
    let mut matrix = [0u8; STATE_COUNT * CHAR_COUNT];

    // Set count increments
    matrix[3 * CHAR_COUNT + 3] = 1; // 'S'
    matrix[6 * CHAR_COUNT + 0] = 1; // 'X'

    matrix
}

static NEXT_STATE: [u8; STATE_COUNT * CHAR_COUNT] = init_next_state();
static COUNT_INCREMENT: [u8; STATE_COUNT * CHAR_COUNT] = init_count_increment();

#[inline(always)]
pub fn go(state: &mut u8, next: u8, cnt: &mut u32) {
    let char_index = char_to_index(next);
    let index = (*state as usize) * CHAR_COUNT + char_index;
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

pub fn run2_4threads(content: &str) -> u32 {
    let n = content.lines().count() as i32;
    let m = content.lines().next().map_or(0, |line| line.len()) as i32;
    let bytes = content.as_bytes();

    let get = |i: usize, j: usize| -> u8 {
        bytes[i * (m as usize + 1) + j]
    };

    let bit = |i: i32, j: i32| -> u32 {
        if i >= 0 && i < n && j >= 0 && j < m {
            match get(i as usize, j as usize) {
                b'M' => 1,
                b'S' => 2,
                _ => 0
            }
        } else {
            0
        }
    };

    let xmas = |i, j| -> u32 {
        ((bit(i - 1, j - 1) | bit(i + 1, j + 1)) == 3 &&  //
            (bit(i - 1, j + 1) | bit(i + 1, j - 1)) == 3) as u32
    };


    // This is slower than expected, but probably because too many threads are spawned
    // (1..n - 1).cartesian_product(1..m - 1).par_bridge().map(|(i, j)| ok(i as usize, j as usize)).sum()

    let a = bytes.len();
    let c = (a + 3) / 4;

    let ans = [0..c, c..2 * c, 2 * c..3 * c, 3 * c..bytes.len()]
        .into_par_iter()
        .map(|range| {
            let chunk = &bytes[range.start..range.end];
            let mut ans = 0;
            for it in memchr_iter(b'A', chunk) {
                let x = range.start as i32 + it as i32;
                let i = x / (m + 1);
                let j = x % (m + 1);
                ans += xmas(i, j);
            }
            ans
        }).sum();
    ans
}

// 1: 2551
// 2: 1985
pub fn run(content: &str) -> u32 {
    run2_4threads(content)
}
