use fxhash::FxHashMap;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
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

const FIELDS: [[[Field; 3]; 4]; 25+1] = [
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    DIRECTIONAL_PAD,
    NUMBER_PAD,
];

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
    pos: [Pos; 25+1],
    idx: usize,
}

#[derive(Default)]
struct BfsData {
    seen: FxHashMap<State, usize>,
    queue: VecDeque<State>,
}

impl BfsData {
    pub fn check_and_push(&mut self, state: State, dist: usize) {
        if !self.seen.contains_key(&state) {
            self.seen.insert(state.clone(), dist);
            self.queue.push_back(state);
        }
    }
}

fn go(pos: Pos, grid: &[[Field; 3]; 4], d: usize) -> Option<Pos> {
    let x = pos.row.checked_add_signed(DIRECTIONS[d][0])?;
    let y = pos.col.checked_add_signed(DIRECTIONS[d][1])?;
    let pos = Pos::new(x, y);

    if x < grid.len() && y < grid[x].len() && grid[x][y] != Field::Empty {
        Some(pos)
    } else {
        None
    }
}

fn neighbours4(pos: Pos, i: usize) -> impl Iterator<Item = Pos> {
    (0..4).filter_map(move |d| go(pos.clone(), &FIELDS[i], d))
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

    bfs.check_and_push(
        State {
            pos: [Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),
                Pos::new(0, 2),


                Pos::new(3, 2)],
            idx: 0,
        },
        0,
    );

    while let Some(front) = bfs.queue.pop_front() {
        let dist = *bfs.seen.get(&front).unwrap();
        if front.idx == input.len() {
            return dist * (number as usize);
        }

        // Press arrow, not so interesting
        for p in neighbours4(front.pos[0].clone(), 0) {
            let mut new_state = front.clone();
            new_state.pos[0] = p;
            bfs.check_and_push(new_state, dist + 1);
        }

        // Press A, potential chain reactions

        'press_a: {
            let mut new_state = front.clone();
            for i in 0..25+1 {
                let p = &front.pos[i];

                match &FIELDS[i][p.row][p.col] {
                    Field::Direction(d) => {
                        let Some(pos) = go(front.pos[i + 1].clone(), &FIELDS[i + 1], *d as usize)
                        else {
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

            bfs.check_and_push(new_state, dist + 1);
        }
    }
    unreachable!();
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
