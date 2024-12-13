use std::collections::HashSet;
use std::fmt::Display;

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

struct Solver<'a> {
    grid: Vec<&'a [u8]>,
    vis: Vec<Vec<bool>>,
    n: usize,
    m: usize,
    map: [HashSet<(usize, usize)>; 4],
}

impl<'a> Solver<'a> {
    pub fn new(content: &'a str) -> Self {
        let grid: Vec<&'a [u8]> = content.lines().map(&str::as_bytes).collect();
        let n = grid.len();
        let m = grid[0].len(); // lines() does not include new line, so no -1 needed
        Self {
            grid,
            n,
            m,
            vis: vec![vec![false; m]; n],
            map: Default::default(),
        }
    }

    pub fn same_color(&self, i: usize, j: usize, d: usize) -> (bool, usize, usize) {
        let [dx, dy] = DIRECTIONS[d];
        let (nx, ny) = ((i as i32) + dx, (j as i32) + dy);
        let same = (0..self.n as i32).contains(&nx)
            && (0..self.m as i32).contains(&ny)
            && self.grid[nx as usize][ny as usize] == self.grid[i][j];

        (same, nx as usize, ny as usize)
    }

    // (Perimeter, Area)
    pub fn dfs(&mut self, i: usize, j: usize) -> (u32, u32) {
        if self.vis[i][j] {
            return (0, 0);
        }
        self.vis[i][j] = true;
        let mut perimeter = 0;
        let mut area = 1;

        for d in 0..4 {
            let (same, nx, ny) = self.same_color(i, j, d);
            if same {
                let (p, a) = self.dfs(nx, ny);
                perimeter += p;
                area += a;
            } else {
                self.map[d].insert((i, j));
                perimeter += 1;
            };
        }
        (perimeter, area)
    }

    pub fn sides(&self) -> u32 {
        let mut ans = 0;
        // Calculate the sides for each direction
        for d in 0..4 {
            let mut sides = 0;
            for (i, j) in &self.map[d] {
                // Assume no partner to the left and right
                sides += 2;
                
                for o in [3, 1] {
                    let nd = (d + o + 4) % 4;
                    let (same, nx, ny) = self.same_color(*i, *j, nd);
                    if same && self.map[d].contains(&(nx, ny)) {
                        sides -= 1;
                    }
                }
            }
            ans += sides / 2;
        }
        ans
    }

    pub fn solve(&mut self) -> (u32, u32) {
        let mut ans1 = 0;
        let mut ans2 = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                let (p, a) = self.dfs(i, j);
                ans1 += p * a;
                
                let s = self.sides();
                ans2 += s * a;
                
                self.map.iter_mut().for_each(|m| m.clear());
            }
        }
        (ans1, ans2)
    }
}

pub fn run(content: &str) -> u32 {
    let ans = Solver::new(content).solve().1;
    // assert_eq!(size_of::<usize>(), 2);
    ans
}
