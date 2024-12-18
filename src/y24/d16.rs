use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    d: usize,
}

#[derive(Debug, Clone)]
struct HeapData {
    state: State,
    cost: u32,
}

impl HeapData {
    pub fn cost(&self) -> u32 {
        self.cost
    }
}

impl Eq for HeapData {}

impl PartialEq<Self> for HeapData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

#[derive(Default)]
struct DijkstraQueue {
    heap: BinaryHeap<HeapData>,
    seen: HashMap<State, u32>,
    prev: HashMap<State, Vec<State>>,
}

impl DijkstraQueue {
    // Returns if it was not inserted, but would have been a candidate
    pub fn check_and_push(&mut self, data: HeapData, prev: Option<State>) {
        let mut also_best = false;
        if let Some(&seen) = self.seen.get(&data.state.clone()) {
            if seen == data.cost {
                also_best = true;
            } else if seen > data.cost {
                // In that case we also just override
                also_best = true;
                self.prev.remove(&data.state);
            }
        } else {
            also_best = true;
            self.heap.push(data.clone());
        }

        if also_best {
            // Overriding is whatever
            self.seen.insert(data.state.clone(), data.cost());
            if let Some(prev) = prev {
                self.prev.entry(data.state.clone()).or_default().push(prev);
            }
        }
    }
}
impl PartialOrd<Self> for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse() // Reverse for dijkstra
    }
}

pub fn run(content: &str) -> u32 {
    run_all(content).1
}
pub fn run_all(content: &str) -> (u32, u32) {
    let grid: Vec<&[u8]> = content.lines().map(|line| line.as_bytes()).collect();
    let n = grid.len();
    let m = grid[0].len();

    let find_pos = |byte: u8| -> Option<[usize; 2]> {
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == byte {
                    return Some([i, j]);
                }
            }
        }
        None
    };

    let start_pos = find_pos(b'S').unwrap();
    let end_pos = find_pos(b'E').unwrap();

    let go = |pos: [usize; 2], d: usize| -> Option<[usize; 2]> {
        let nx = pos[0].checked_add_signed(DIRECTIONS[d][0] as isize)?;
        let ny = pos[1].checked_add_signed(DIRECTIONS[d][1] as isize)?;
        if grid[nx][ny] != b'#' {
            Some([nx, ny])
        } else {
            None
        }
    };

    let mut dijsktra = DijkstraQueue::default();
    dijsktra.check_and_push(
        HeapData {
            cost: 0,
            state: State {
                x: start_pos[0],
                y: start_pos[1],
                d: 0,
            },
        },
        None,
    );

    let mut candidates = VecDeque::new();
    let mut min_cost = None::<u32>;
    while let Some(front) = dijsktra.heap.pop() {
        let state = front.state.clone();
        let HeapData {
            cost,
            state: State { x, y, d },
        } = front;

        if [x, y] == end_pos {
            if min_cost.is_none_or(|m| m == cost) {
                min_cost = Some(cost);
                candidates.push_back(state.clone());
            }
            continue;
        }

        // 1. Go into the direction +1
        if let Some([x, y]) = go([x, y], d) {
            let data = HeapData {
                cost: cost + 1,
                state: State { x, y, d },
            };
            dijsktra.check_and_push(data, Some(state.clone()));
        }
        // 2. Rotate +1000
        for dd in [1, 3] {
            dijsktra.check_and_push(
                HeapData {
                    cost: cost + 1000,
                    state: State {
                        x,
                        y,
                        d: (d + dd) % 4,
                    },
                },
                Some(state.clone()),
            );
        }
    }

    let mut pos_seen: HashSet<[usize; 2]> = Default::default();
    let mut state_seen: HashSet<State> = Default::default();
    //
    while let Some(front) = candidates.pop_front() {
        pos_seen.insert([front.x, front.y]);
        if let Some(prev) = dijsktra.prev.get(&front) {
            for p in prev {
                if state_seen.insert(p.clone()) {
                    candidates.push_back(p.clone())
                }
            }
        }
    }

    let ans1 = min_cost.unwrap();
    let ans2 = pos_seen.len() as u32;

    // for i in 0..n {
    //     for j in 0..m {
    //         if pos_seen.contains(&[i, j]){
    //             print!("O");
    //         } else {
    //             print!("{}", grid[i][j] as char);
    //         }
    //     }
    //     println!()
    // }
    (ans1, ans2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example2() {
        let content: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let ans = run(content);

        assert_eq!(ans, 64);

    }
}