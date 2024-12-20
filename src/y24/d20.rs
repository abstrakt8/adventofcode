use fxhash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

const DIRECTIONS: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type State = Pos;

#[derive(Debug, Clone, Default)]
struct BfsData {
    dist: FxHashMap<State, usize>,
    queue: VecDeque<State>,
}

impl BfsData {
    pub fn check_and_push(&mut self, state: State, dist: usize) {
        if !self.dist.contains_key(&state) {
            self.dist.insert(state.clone(), dist);
            self.queue.push_back(state);
        }
    }

    pub fn dist(&self, pos: &Pos) -> usize {
        *self.dist.get(pos).unwrap()
    }
}

struct Solver {
    grid: Vec<Vec<u8>>,
}

const WALL: u8 = b'#';

impl Solver {
    pub fn from_str(content: &str) -> Self {
        let grid: Vec<Vec<u8>> = content
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        Self { grid }
    }

    #[allow(clippy::needless_lifetimes)]
    fn neighbours4<'a>(&'a self, pos: Pos) -> impl Iterator<Item = Pos> + use<'a> {
        (0..4).filter_map(move |d| self.go_dir(&pos, d))
    }

    fn contains_pos(&self, pos: &Pos) -> bool {
        let height = self.grid.len();
        let width = self.grid[0].len();
        (0..height).contains(&pos.x) && (0..width).contains(&pos.y)
    }

    fn find_pos(&self, byte: u8) -> Option<Pos> {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == byte {
                    return Some(Pos::new(i, j));
                }
            }
        }
        None
    }

    pub fn simulate_normal_only(&self, start: &Pos) -> BfsData {
        let mut bfs = BfsData::default();
        bfs.check_and_push(start.clone(), 0);

        while let Some(front) = bfs.queue.pop_front() {
            let dist = *bfs.dist.get(&front).unwrap();
            for pos in self.neighbours4(front) {
                let is_wall = self.grid[pos.x][pos.y] == b'#';
                if !is_wall {
                    bfs.check_and_push(pos, dist + 1);
                }
            }
        }
        bfs
    }

    pub fn go(&self, pos: &Pos, dx: isize, dy: isize) -> Option<Pos> {
        let x = pos.x.checked_add_signed(dx)?;
        let y = pos.y.checked_add_signed(dy)?;
        let pos = Pos::new(x, y);
        self.contains_pos(&pos).then_some(pos)
    }

    pub fn go_dir(&self, pos: &Pos, d: usize) -> Option<Pos> {
        let [dx, dy] = DIRECTIONS[d];
        self.go(pos, dx, dy)
    }

    fn is_wall(&self, pos: &Pos) -> bool {
        self.grid[pos.x][pos.y] == WALL
    }

    pub fn count_cheats(&self, max_cheat_dist: usize, saved: usize) -> usize {
        let start = self.find_pos(b'S').unwrap();
        let end = self.find_pos(b'E').unwrap();

        let from_start = self.simulate_normal_only(&start);
        let from_end = self.simulate_normal_only(&end);
        let original_dist = from_start.dist(&end);

        let max_cheat_dist = max_cheat_dist as isize;

        // dbg!(original_dist);

        let mut ans = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let p0 = Pos::new(i, j);
                if self.is_wall(&p0) {
                    continue;
                }
                for dx in -max_cheat_dist..=max_cheat_dist {
                    let max_dy = max_cheat_dist - dx.abs();
                    for dy in -max_dy..=max_dy {
                        let Some(p1) = self.go(&p0, dx, dy) else {
                            continue;
                        };
                        let cheat_dist = (dx.abs() + dy.abs()) as usize;
                        if !self.is_wall(&p1) {
                            let new_dist = from_start.dist(&p0) + cheat_dist + from_end.dist(&p1);
                            if new_dist + saved <= original_dist {
                                ans += 1;
                            }
                        }

                    }
                }
            }
        }

        ans
    }
}

pub fn run_all(content: &str) -> (usize, usize) {
    let solver = Solver::from_str(content);
    let ans1 = solver.count_cheats(2, 100);
    let ans2 = solver.count_cheats(20, 100);
    (ans1, ans2)
}

pub fn run(content: &str) -> usize {
    let solver = Solver::from_str(content);
    let ans1 = solver.count_cheats(2, 100);
    ans1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        let content = include_str!("../../inputs/y24/20_example.in");
        let solver = Solver::from_str(content);

        assert_eq!(solver.count_cheats(20, 76), 3);
        assert_eq!(solver.count_cheats(20, 74), 3 + 4);
        assert_eq!(solver.count_cheats(20, 72), 3 + 4 + 22);
        assert_eq!(solver.count_cheats(20, 70), 3 + 4 + 22 + 12);
        assert_eq!(solver.count_cheats(20, 68), 3 + 4 + 22 + 12 + 14);
        assert_eq!(solver.count_cheats(20, 66), 3 + 4 + 22 + 12 + 14 + 12);
        assert_eq!(solver.count_cheats(20, 64), 3 + 4 + 22 + 12 + 14 + 12 + 19);
        assert_eq!(
            solver.count_cheats(20, 62),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20
        );
        assert_eq!(
            solver.count_cheats(20, 60),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23
        );
        assert_eq!(
            solver.count_cheats(20, 58),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23 + 25
        );
        assert_eq!(
            solver.count_cheats(20, 56),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39
        );
        assert_eq!(
            solver.count_cheats(20, 54),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29
        );
        assert_eq!(
            solver.count_cheats(20, 52),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29 + 31
        );
        assert_eq!(
            solver.count_cheats(20, 50),
            3 + 4 + 22 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29 + 31 + 32
        );
    }
}
