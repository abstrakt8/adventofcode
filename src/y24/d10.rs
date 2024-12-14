use bitvec::prelude::*;
use std::collections::VecDeque;

pub fn run(content: &str) -> u32 {
    run2_fast(content)
}

pub fn run1(content: &str) -> u32 {
    type Grid = Vec<Vec<bool>>;
    let grid: Vec<Vec<u8>> = content
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect();
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    let go = |i: usize, j: usize, d: usize| -> Option<(usize, usize)> {
        let ni = (i as i32) + dirs[d][0];
        let nj = (j as i32) + dirs[d][1];
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == grid[i][j] + 1 {
                Some((ni, nj))
            } else {
                None
            }
        } else {
            None
        }
    };

    let mut vis: Grid = vec![vec![false; m]; n];
    let simulate = |vis: &mut Grid, i: usize, j: usize| {
        let mut q: VecDeque<(usize, usize)> = VecDeque::from([(i, j)]);
        let mut ans = 0;
        vis[i][j] = true;

        let mut k = 0;
        while k < q.len() {
            let (i, j) = q[k];
            k += 1;
            for (i, j) in (0..4).filter_map(|d| go(i, j, d)) {
                if !vis[i][j] {
                    vis[i][j] = true;
                    q.push_back((i, j));
                    if grid[i][j] == 9 {
                        ans += 1;
                    }
                }
            }
        }
        // Cleanup
        while let Some((i, j)) = q.pop_front() {
            vis[i][j] = false;
        }
        ans
    };

    let mut ans1 = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                let i1 = simulate(&mut vis, i, j);
                ans1 += i1;
            }
        }
    }

    ans1
}

pub fn run2(content: &str) -> u32 {
    type Grid = Vec<Vec<u32>>;
    let grid: Vec<Vec<u8>> = content
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect();
    let n = grid.len();
    let m = grid[0].len();

    let mut ways: Grid = vec![vec![0; m]; n];

    let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    let update = |ways: &mut Grid, i: usize, j: usize, d: usize| {
        let ni = (i as i32) + dirs[d][0];
        let nj = (j as i32) + dirs[d][1];
        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] != grid[i][j] + 1 {
            } else {
                ways[i][j] += ways[ni][nj];
            }
        } 
    };

    let mut ans1 = 0;
    // This be optimised
    for digit in (0..=9).rev() {
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != digit {
                    continue;
                }

                if digit == 9 {
                    ways[i][j] = 1;
                } else {
                    for d in 0..4 {
                        update(&mut ways, i, j, d);
                    }
                }

                if digit == 0 {
                    ans1 += ways[i][j];
                }
            }
        }
    }

    // println!("{:?}", ways);

    ans1
}

pub fn run2_fast(content: &str) -> u32 {
    let bytes = content.as_bytes();
    let nbytes = bytes.len();

    // We just include new line as a character as well, doesn't really matter because it's not 1 away from any of the digits.
    let cols = memchr::memchr(b'\n', bytes).unwrap() + 1;
    let mut ways = vec![0; nbytes];
    let mut vis = bitvec![0; nbytes];

    let mut q: VecDeque<usize> = VecDeque::new();
    for pos in memchr::memchr_iter(b'9', bytes) {
        ways[pos] = 1;
        vis.set(pos, true);
        q.push_back(pos);
    }

    let mut ans = 0;

    let mut check = |q: &mut VecDeque<usize>, ways: &mut Vec<u32>, cur: usize, delta: i32| {
        let new = (cur as i32) + delta;
        if new >= 0 && (new as usize) < nbytes {
            let new = new as usize;
            if bytes[cur] == bytes[new] + 1 {
                ways[new] += ways[cur];
                if !vis[new] {
                    vis.set(new, true);
                    q.push_back(new);
                }
            }
        }
    };
    
    while let Some(pos) = q.pop_front() {
        if bytes[pos] == b'0' {
            ans += ways[pos];
        } else {
            for delta in [-(cols as i32), cols as i32, -1, 1] {
                check(&mut q, &mut ways, pos, delta);
            }
        }
    }

    ans
}
