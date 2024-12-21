use fxhash::{FxBuildHasher, FxHashMap};
use std::cmp::max;
use std::collections::HashMap;
use itertools::rev;
use strum::VariantArray;

#[derive(Clone, Debug, PartialEq, Default)]
enum CellType {
    #[default]
    Void,
    Wall,
    Empty,
}

impl From<u8> for CellType {
    fn from(value: u8) -> Self {
        match value {
            b' ' => CellType::Void,
            b'#' => CellType::Wall,
            b'.' => CellType::Empty,
            _ => unreachable!("Unexpected input {}", value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Data {
    cell_type: CellType,
    relative_position: Pos,
    neighbors: [Pos; 4],
}

#[derive(VariantArray, Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    RIGHT = 0,
    DOWN = 1,
    LEFT = 2,
    UP = 3,
}

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, -1];

impl Direction {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn left(self) -> Direction {
        Direction::VARIANTS[(self.to_usize() + 3) % 4]
    }
    fn right(self) -> Direction {
        Direction::VARIANTS[(self.to_usize() + 1) % 4]
    }
    fn reverse(self) -> Direction {
        Direction::VARIANTS[(self.to_usize() + 2) % 4]
    }
    fn add(self, u: usize) -> Self {
        Direction::VARIANTS[(self.to_usize() + u) % 4]
    }
}

#[derive(Default)]
struct Cube {
    idx: [[Option<usize>; 4]; 4],
    pos: [(usize, usize); 6],
    neighbors: [[usize; 4]; 6],
}

impl Cube {
    pub fn connect(&mut self, a: usize, b: Option<usize>, dir: Direction) {
        if let (a, Some(b)) = (a, b) {
            self.neighbors[a][dir as usize] = b;
        }
    }

    pub fn go(&self, cur: usize, d: Direction) -> Option<usize> {
        let (i, j): (usize, usize) = self.pos[cur];
        let x = (i as i32) + DX[d.to_usize()];
        let y = (j as i32) + DY[d.to_usize()];

        if (0..4).contains(&x) && (0..4).contains(&y) {
            self.idx[x as usize][y as usize]
        } else {
            None
        }
    }

    pub fn handle_opposite(&mut self, origin: usize, cur_cell: usize, cur_dir: Direction) {
        self.connect(origin, self.go(cur_cell, cur_dir), cur_dir.reverse());
        self.connect(origin, self.go(cur_cell, cur_dir.left()), cur_dir.left());
        self.connect(origin, self.go(cur_cell, cur_dir.right()), cur_dir.right());
    }

    pub fn handle_origin(&mut self, origin: usize) {
        for dir in Direction::VARIANTS {}
    }

    pub fn handle_side(&mut self, origin: usize, origin_dir: Direction, cur: usize, cur_dir: Direction, reversed: bool) {
        // self.connect(origin, Some(cur), cur_dir);
        // 
        // for d in [1, 3] {
        //     let dir = origin_dir.add(d);
        //     
        // 
        // }
        // 
        // if let Some(side) = self.go(cur, origin_dir.left()) {
        //     let mut dir = origin_dir.left();
        //     if reversed {
        //         dir = dir.reverse();
        //     }
        //     self.handle_side(origin, origin_dir, side, dir, reversed);
        // }
        // 
        // 
        // if let Some(opposite) = self.go(cur_cell, cur_dir)  {
        //     self.handle_opposite(origin, opposite, cur_dir);
        // }
        // 
        // // We handle the sides when going to the right
        // let mut tmp_cell = cur_cell;
        // let mut tmp_dir = cur_dir;
        // for _ in 0..3 {
        //     if let Some(next) = self.go(tmp_cell, cur_dir.right()) {
        //         tmp_dir = tmp_dir.right();
        //         tmp_cell = next;
        //         self.connect(origin, Some(tmp_cell), tmp_dir);
        // 
        //         if let Some(opposite) = self.go(tmp_cell, cur_dir) {
        //             self.handle_opposite(origin, opposite, tmp_dir)
        //         }
        //     }
        // }
        // 
        // let mut tmp_cell = cur_cell;
        // let mut tmp_dir = cur_dir;
        // for _ in 0..3 {
        //     if let Some(next) = self.go(tmp_cell, cur_dir.left()) {
        //         tmp_dir = tmp_dir.left();
        //         tmp_cell = next;
        //         self.connect(origin, Some(tmp_cell), tmp_dir);
        // 
        //         if let Some(opposite) = self.go(tmp_cell, cur_dir) {
        //             self.handle_opposite(origin, opposite, tmp_dir)
        //         }
        //     }
        // }
    }

    pub fn process_neighbors(&mut self) {
        for i in 0..6 {
            // for &dir in Direction::VARIANTS {
            //     if let Some(side) = self.go(i, dir) {
            //         self.handle_side(i, side, dir);
            //     }
            // }
        }
    }
}

pub fn run2(content: &str) -> usize {
    let (grid, moves) = content.split_once("\n\n").unwrap();

    // Wrap them with void cells to make processing easier
    let bytes: Vec<&[u8]> = grid.lines().map(&str::as_bytes).collect();
    let rows = bytes.len();
    let cols = bytes.iter().map(|row| row.len()).max().unwrap();
    let cube_length = max(rows, cols) / 4;
    let (rows, cols) = (cube_length * 4, cube_length * 4);

    let mut data: Vec<Vec<Data>> = vec![vec![Default::default(); cols]; rows];
    for i in 0..bytes.len() {
        let mut cnt = 0;
        for j in 0..bytes[i].len() {
            data[i][j].cell_type = CellType::from(bytes[i][j]);
            if data[i][j].cell_type != CellType::Void {
                data[i][j].relative_position = Pos::new(i + 1, cnt + 1);
                cnt += 1;
            }
        }
    }

    let mut cube_idx = [[None::<usize>; 4]; 4];
    let mut cube_pos = [(0, 0); 6];
    let mut cnt = 0;
    for i in 0..4 {
        for j in 0..4 {
            if data[i * cube_length][j * cube_length].cell_type != CellType::Void {
                cube_idx[i][j] = Some(cnt);
                cube_pos[cnt] = (i, j);
                cnt += 1;
            }
        }
    }

    let mut cube: Cube = Cube::default();
    cube.idx = cube_idx;
    cube.pos = cube_pos;
    cube.process_neighbors();

    let edge_pos = |c: usize, var: usize, dir: Direction| -> Pos {
        // Top left corner
        let (i, j) = (cube.pos[c].0 * cube_length, cube.pos[c].1 * cube_length);

        match dir {
            Direction::RIGHT => Pos::new(var, j + cube_length - 1),
            Direction::LEFT => Pos::new(var, j),
            Direction::UP => Pos::new(i, var),
            Direction::DOWN => Pos::new(i + cube_length - 1, var),
        }
    };

    let mut connect = |c1: usize, c2: usize, var: usize, dir: Direction| {
        let p1 = edge_pos(c1, var, dir);
        let p2 = edge_pos(c2, var, dir);
        data[p1.x][p1.y].neighbors[dir as usize] = p2;
    };

    for c in 0..6 {
        for &dir in Direction::VARIANTS {
            for var in 0..cube_length {
                connect(c, cube.neighbors[c][dir as usize], var, dir);
            }
        }
    }

    let (pos, dir) = simulate(&data, moves);
    let pos = data[pos.x][pos.y].relative_position.clone();

    score(pos, dir)
}

fn score(pos: Pos, dir: Direction) -> usize {
    pos.x * 1000 + pos.y * 4 + dir.to_usize()
}

fn simulate(data: &Vec<Vec<Data>>, moves: &str) -> (Pos, Direction) {
    let mut pos = data
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, data)| {
                if data.relative_position == Pos::new(1, 1) {
                    Some(Pos::new(i, j))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut len = 0;
    let mut dir = Direction::RIGHT;

    let simulate_move = |len: u32, mut pos: Pos, dir: Direction| -> Pos {
        for _ in 0..len {
            // println!("{:?} dir={:?}", pos, dir);
            let new = data[pos.x][pos.y].neighbors[dir as usize].clone();
            if data[new.x][new.y].cell_type == CellType::Wall {
                break;
            }
            pos = new;
        }
        pos
    };

    for &byte in moves.trim().as_bytes() {
        if byte.is_ascii_digit() {
            len = len * 10 + (byte - b'0') as u32;
        } else {
            pos = simulate_move(len, pos, dir);
            len = 0;
            dir = if byte == b'L' {
                dir.left()
            } else {
                dir.right()
            };
        }
    }

    pos = simulate_move(len, pos, dir);
    (pos, dir)
}

pub fn run1(content: &str) -> usize {
    let (grid, moves) = content.split_once("\n\n").unwrap();

    // Wrap them with void cells to make processing easier
    let bytes: Vec<&[u8]> = grid.lines().map(&str::as_bytes).collect();
    let rows = bytes.len() + 2;
    let cols = bytes.iter().map(|row| row.len()).max().unwrap() + 2;
    let mut idx_to_abs: FxHashMap<Pos, Pos> =
        HashMap::with_capacity_and_hasher(rows * cols, FxBuildHasher::default());
    let mut data: Vec<Vec<Data>> = vec![vec![Default::default(); cols]; rows];
    for i in 0..bytes.len() {
        for j in 0..bytes[i].len() {
            data[i + 1][j + 1].cell_type = CellType::from(bytes[i][j]);
        }
    }

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            data[row][col].neighbors[Direction::LEFT as usize] = Pos::new(row, col - 1);
            data[row][col].neighbors[Direction::RIGHT as usize] = Pos::new(row, col + 1);
            data[row][col].neighbors[Direction::UP as usize] = Pos::new(row - 1, col);
            data[row][col].neighbors[Direction::DOWN as usize] = Pos::new(row + 1, col);
        }
    }

    for row in 1..rows - 1 {
        let mut row_cnt = 0;
        let mut open: Option<Pos> = None;
        for col in 1..cols {
            match data[row][col].cell_type {
                CellType::Void => {
                    if let Some(open) = open.take() {
                        let last = Pos::new(row, col - 1);
                        data[open.x][open.y].neighbors[Direction::LEFT.to_usize()] = last.clone();
                        data[last.x][last.y].neighbors[Direction::RIGHT.to_usize()] = open;
                    }
                }
                CellType::Wall | CellType::Empty => {
                    row_cnt += 1;
                    data[row][col].relative_position = Pos::new(row, row_cnt);
                    idx_to_abs.insert(data[row][col].relative_position.clone(), Pos::new(row, col));

                    if open.is_none() {
                        open = Some(Pos::new(row, col));
                    }
                }
            }
        }
    }

    for col in 1..cols - 1 {
        let mut open: Option<Pos> = None;
        for row in 1..rows {
            match data[row][col].cell_type {
                CellType::Void => {
                    if let Some(open) = open.take() {
                        let last = Pos::new(row - 1, col);
                        data[open.x][open.y].neighbors[Direction::UP.to_usize()] = last.clone();
                        data[last.x][last.y].neighbors[Direction::DOWN.to_usize()] = open;
                    }
                }
                CellType::Wall | CellType::Empty => {
                    if open.is_none() {
                        open = Some(Pos::new(row, col));
                    }
                }
            }
        }
    }

    let (pos, dir) = simulate(&data, moves);
    let pos = data[pos.x][pos.y].relative_position.clone();

    score(pos, dir)
}

pub fn run(content: &str) -> usize {
    run2(content)
}