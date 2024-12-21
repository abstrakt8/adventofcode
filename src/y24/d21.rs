use fxhash::FxHashMap;
use std::collections::{HashSet, VecDeque};
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

const FIELDS: [[[Field; 3]; 4]; 2 + 1] = [DIRECTIONAL_PAD, DIRECTIONAL_PAD, NUMBER_PAD];

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct State {
    pos: [Pos; 2 + 1],
    idx: usize,
}

enum InputData {
    PressA,
    Direction(Direction),
}

struct ReconstructData {
    prev: State,
    direction: InputData,
}

#[derive(Default)]
struct BfsData {
    seen: FxHashMap<State, usize>,
    prev: FxHashMap<State, ReconstructData>,
    queue: VecDeque<State>,
}

impl BfsData {
    pub fn check_and_push(
        &mut self,
        state: State,
        dist: usize,
        reconstruct_data: Option<ReconstructData>,
    ) {
        if !self.seen.contains_key(&state) {
            self.seen.insert(state.clone(), dist);
            if let Some(reconstruct_data) = reconstruct_data {
                self.prev.insert(state.clone(), reconstruct_data);
            }
            self.queue.push_back(state);
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

pub fn solve(input: &[u8]) -> usize {
    let mut bfs = BfsData::default();

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

    let dir_pad_A_pos = Pos::new(0, 2);

    bfs.check_and_push(
        State {
            pos: [dir_pad_A_pos.clone(), dir_pad_A_pos.clone(), Pos::new(3, 2)],
            idx: 0,
        },
        0,
        None,
    );

    let mut ans1 = 0;
    while let Some(front) = bfs.queue.pop_front() {
        let dist = *bfs.seen.get(&front).unwrap();
        if front.idx == input.len() {
            ans1 = dist * (number);
            break;
        }

        // Press arrow, not so interesting
        for d in Direction::VARIANTS {
            let Some(p) = go(front.pos[0].clone(), &FIELDS[0], *d) else {
                continue;
            };
            let mut new_state = front.clone();
            new_state.pos[0] = p;
            bfs.check_and_push(
                new_state,
                dist + 1,
                Some(ReconstructData {
                    prev: front.clone(),
                    direction: InputData::Direction(*d),
                }),
            );
        }

        // Press A, potential chain reactions

        'press_a: {
            let mut new_state = front.clone();
            for i in 0..2 + 1 {
                let p = &front.pos[i];

                match &FIELDS[i][p.row][p.col] {
                    Field::Direction(d) => {
                        let Some(pos) = go(front.pos[i + 1].clone(), &FIELDS[i + 1], *d) else {
                            break 'press_a;
                        };
                        new_state.pos[i + 1] = pos;
                        break;
                    }
                    Field::Number(n) => {
                        if *n != input[new_state.idx] {
                            break 'press_a;
                        }
                        new_state.idx += 1;
                        break;
                    }
                    Field::Activate => {
                        continue;
                    }
                    Field::Empty => {
                        unreachable!("Should not reach empty")
                    }
                }
            }

            bfs.check_and_push(
                new_state,
                dist + 1,
                Some(ReconstructData {
                    prev: front.clone(),
                    direction: InputData::PressA,
                }),
            );
        }
    }



    // Ignore 0
    let mut dp = vec![vec![0u64; 6]; 23];

    let inputs_required = vec![
        // 0: Empty
        vec![],
        // 1: ^
        vec![Direction::LEFT, Direction::RIGHT],
        // 2: A
        vec![],
        // 3: <
        vec![
            Direction::LEFT,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
            Direction::UP,
            Direction::RIGHT,
        ],
        // 4: v
        vec![
            Direction::LEFT,
            Direction::DOWN,
            Direction::RIGHT,
            Direction::UP,
        ],
        // 5: >
        vec![Direction::DOWN, Direction::UP],
    ];

    let mapping: [usize; 4] = [3, 5, 1, 4];

    for i in 0..4 {
        dp[0][mapping[i]] = inputs_required[mapping[i]].len() as u64 + 1;
    }
    for i in 1..dp.len() {
        for j in 0..4 {
            dp[i][j] = 1;
            for k in &inputs_required[mapping[j]] {
                let k_mapping = mapping[*k as usize];
                dp[i][j] += dp[i - 1][k_mapping];
            }
        }
    }

    ans1
}

pub fn run(content: &str) -> usize {
    let inputs: Vec<&[u8]> = content.lines().map(&str::as_bytes).collect();

    inputs.into_iter().map(solve).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn example() {}
}
