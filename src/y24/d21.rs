use fxhash::FxHashMap;
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use strum::VariantArray;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy, VariantArray)]
enum Direction {
    LEFT = 0,
    RIGHT = 1,
    UP = 2,
    DOWN = 3,
}
const DIRECTIONS: [[isize; 2]; 4] = [[0, -1], [0, 1], [-1, 0], [1, 0]];

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Field {
    Direction(Direction),
    Number(u8),
    Activate,
    Empty,
}

const DIRECTIONAL_PAD: [[Field; 3]; 4] = [
    [
        Field::Empty,
        Field::Direction(Direction::UP),
        Field::Activate,
    ],
    [
        Field::Direction(Direction::LEFT),
        Field::Direction(Direction::DOWN),
        Field::Direction(Direction::RIGHT),
    ],
    [Field::Empty, Field::Empty, Field::Empty],
    [Field::Empty, Field::Empty, Field::Empty],
];

const NUMBER_PAD: [[Field; 3]; 4] = [
    [Field::Number(7), Field::Number(8), Field::Number(9)], //
    [Field::Number(4), Field::Number(5), Field::Number(6)], //
    [Field::Number(1), Field::Number(2), Field::Number(3)], //
    [Field::Empty, Field::Number(0), Field::Number(10)],    //
];

const FIELDS: [[[Field; 3]; 4]; 1 + 1] = [DIRECTIONAL_PAD, NUMBER_PAD];

#[derive(Clone, Debug, Default, PartialEq, Hash, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn from_directional_index(idx: usize) -> Self {
        Self::new(idx / 3, idx % 3)
    }
    pub fn dir_idx(&self) -> usize {
        self.row * 3 + self.col
    }
}

#[derive(Clone, Default, Debug, PartialEq, Hash, Eq)]
struct State {
    pos: [Pos; 2],
    idx: usize,
}
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum InputData {
    PressA,
    Direction(Direction),
}

struct ReconstructData {
    prev: State,
    direction: InputData,
}

#[derive(Default, Debug)]
struct HeapData {
    state: State,
    dist: u64,
}

impl Eq for HeapData {}

impl PartialEq<Self> for HeapData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

#[derive(Default)]
struct DijkstraData {
    seen: FxHashMap<State, u64>,
    queue: BinaryHeap<HeapData>,
}

impl DijkstraData {
    pub fn check_and_push(&mut self, state: State, dist: u64) {
        let mut better = true;
        if let Some(best_seen) =  self.seen.get(&state) {
            better = dist < *best_seen;
        }
        if better {
            self.seen.insert(state.clone(), dist);
            self.queue.push(HeapData { state, dist });
        }
    }
}

fn hamilton(a: &Pos, b: &Pos) -> usize {
    (a.row.abs_diff(b.row)) + a.col.abs_diff(b.col)
}

fn go(pos: Pos, grid: &[[Field; 3]; 4], d: Direction) -> Option<Pos> {
    let x = pos.row.checked_add_signed(DIRECTIONS[d as usize][0])?;
    let y = pos.col.checked_add_signed(DIRECTIONS[d as usize][1])?;
    let pos = Pos::new(x, y);

    if x < grid.len() && y < grid[x].len() && grid[x][y] != Field::Empty {
        Some(pos)
    } else {
        None
    }
}

pub fn solve(input: &[u8], num_directional: usize) -> u64 {
    let input: Vec<u8> = input
        .iter()
        .map(|d| if d.is_ascii_digit() { *d - b'0' } else { 10 })
        .collect();
    let mut number = 0usize;
    for i in &input {
        if *i < 10 {
            number = number * 10 + (*i as usize);
        }
    }
    let directional_a_pos = Pos::new(0, 2);

    // Ignore 0
    let mut cost = vec![vec![vec![0u64; 6]; 6]; num_directional + 1];

    // Base case: Human layer
    for i in 1..6 {
        for j in 1..6 {
            cost[0][i][j] = 1;
        }
    }

    let mut field_pos: HashMap<Field, Pos> = Default::default();
    for i in 0..2 {
        for j in 0..3 {
            field_pos.insert(DIRECTIONAL_PAD[i][j].clone(), Pos::new(i, j));
        }
    }

    let field_pos = |field: Field| -> Pos { field_pos.get(&field).unwrap().clone() };

    // Cost[i][a][b] = Cost to go from a to b in layer i and then press activate 'b', when last activated 'a'
    //                 This means i-1's last position was A and will end at A which will simplify things
    // Cost[i][<][>] = (Cost[i-1][A][>] + 1) + (Cost[i-1][>][>] + 1) + (Cost[i-1][>][A] + 1)
    // Some base cases:
    // Cost[0][*][*] = 0 -> Human layer has no movement cost basically, just press it
    // Cost[i][a][a] = 1

    // Numerical pad
    // Dijkstra, but the cost of moving to a direction lets say >, when last direction was <:
    // -> Cost[n][<][>]
    for layer in 1..cost.len() {
        for start in 1..6 {
            for end in 1..6 {
                if end == start {
                    // Base case
                    cost[layer][start][end] = 1;
                } else {
                    cost[layer][start][end] = u64::MAX;
                }
            }

            let mut dijk = DijkstraData::default();
            dijk.check_and_push(
                State {
                    idx: 0,
                    pos: [
                        directional_a_pos.clone(),
                        Pos::from_directional_index(start),
                    ],
                },
                0, // The cost for activating
            );

            while let Some(front) = dijk.queue.pop() {
                let [prev, cur] = &front.state.pos;
                let dist = *dijk.seen.get(&front.state).unwrap();
                
                if front.dist != dist {
                    continue;
                }

                cost[layer][start][cur.dir_idx()] = min(
                    cost[layer][start][cur.dir_idx()],
                    dist + cost[layer - 1][prev.dir_idx()][field_pos(Field::Activate).dir_idx()]
                );

                for d in Direction::VARIANTS {
                    let Some(new_cur) = go(cur.clone(), &DIRECTIONAL_PAD, *d) else {
                        continue;
                    };
                    let new_prev = field_pos(Field::Direction(*d));
                    let new_cost = dist + cost[layer - 1][prev.dir_idx()][new_prev.dir_idx()];

                    dijk.check_and_push(
                        State {
                            pos: [new_prev.clone(), new_cur],
                            idx: 0,
                        },
                        new_cost,
                    );
                }
            }
        }
    }

    let mut dijk = DijkstraData::default();

    dijk.check_and_push(
        State {
            pos: [directional_a_pos.clone(), Pos::new(3, 2)],
            idx: 0,
        },
        0,
    );

    let cost = &cost[cost.len() - 1];
    let mut shortest_path = None;
    while let Some(front) = dijk.queue.pop() {
        let state = front.state;
        let dist = *dijk.seen.get(&state).unwrap();
        
        if front.dist != dist {
            continue;
        }
        
        if state.idx == input.len() {
            shortest_path = Some(dist);
            break;
        }

        let [prev, cur] = state.pos.clone();

        // Press arrow in the last directional pad
        for d in Direction::VARIANTS {
            let Some(new_cur) = go(cur.clone(), &NUMBER_PAD, *d) else {
                continue;
            };
            let new_prev = field_pos(Field::Direction(*d));
            let new_dist = dist + cost[prev.dir_idx()][new_prev.dir_idx()];

            let mut new_state = state.clone();
            new_state.pos = [new_prev, new_cur];
            dijk.check_and_push(new_state, new_dist);
        }

        // Press A in last directional pad, only makes sense if cur is at the number we want
        if matches!(&NUMBER_PAD[cur.row][cur.col], Field::Number(n) if *n == input[state.idx]) {
            let new_prev = field_pos(Field::Activate);
            let new_dist = dist + cost[prev.dir_idx()][new_prev.dir_idx()];
            let new_state = State {
                pos: [new_prev, cur],
                idx: state.idx + 1,
            };
            dijk.check_and_push(new_state, new_dist);
        }
    }

    let dist = shortest_path.unwrap();
    let ans1 = dist * (number as u64);
    ans1
}

pub fn run(content: &str) -> (u64, u64) {
    let inputs: Vec<&[u8]> = content.lines().map(&str::as_bytes).collect();

    let ans1 = inputs.iter().map(|b| solve(b, 2)).sum();
    let ans2 = inputs.iter().map(|b| solve(b, 25)).sum();

    (ans1, ans2)
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn example() {}
}
