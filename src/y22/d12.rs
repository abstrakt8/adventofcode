use std::cmp::min;
use std::collections::VecDeque;
use std::iter;

type Pos = (i32, i32);
const D: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn go(p: Pos, d: usize) -> Pos {
    (p.0 + D[d][0], p.1 + D[d][1])
}

fn find_it<'a>(grid: &'a Vec<&[u8]>, b: u8) -> impl Iterator<Item = Pos> + 'a {
    let mut i = 0;
    let mut j = 0;
    iter::from_fn(move || {
        while i < grid.len() {
            while j < grid[i].len() {
                if grid[i][j] == b {
                    let ret = Some((i as i32, j as i32));
                    j += 1;
                    return ret;
                } else {
                    j += 1;
                }
            }
            i += 1;
            j = 0;
        }
        None
    })
}

pub fn run(content: &str) -> (i32, i32) {
    let grid: Vec<&[u8]> = content.lines().map(|x| x.as_bytes()).collect();
    let n = grid.len();
    let m = grid[0].len();

    let height_at = |p: Pos| -> Option<i32> {
        if (0..n as i32).contains(&p.0) && (0..m as i32).contains(&p.1) {
            match grid[p.0 as usize][p.1 as usize] {
                b'S' => Some(0),
                b'E' => Some(25),
                x if x.is_ascii_lowercase() => Some((x - b'a') as i32),
                _ => unreachable!(),
            }
        } else {
            None
        }
    };

    let start = find_it(&grid, b'S').next().unwrap();
    let end = find_it(&grid, b'E').next().unwrap();

    let simulate = |start: Pos| {
        let mut dist = vec![vec![i32::MAX; m]; n];
        dist[start.0 as usize][start.1 as usize] = 0;

        let mut q: VecDeque<Pos> = VecDeque::from([start]);

        loop {
            let Some(cur) = q.pop_front() else {
                break;
            };
            if cur == end {
                break;
            }
            let cur_h = height_at(cur).unwrap();
            (0..4).for_each(|d| {
                let nxt = go(cur, d);
                if let Some(nxt_h) = height_at(nxt) {
                    if nxt_h - cur_h <= 1 && dist[nxt.0 as usize][nxt.1 as usize] == i32::MAX {
                        dist[nxt.0 as usize][nxt.1 as usize] =
                            dist[cur.0 as usize][cur.1 as usize] + 1;
                        q.push_back(nxt);
                    }
                }
            })
        }
        dist[end.0 as usize][end.1 as usize]
    };

    let ans1 = simulate(start);
    let ans2 = min(
        ans1,
        find_it(&grid, b'a').map(simulate).min().unwrap_or_default(),
    );

    (ans1, ans2)
}
