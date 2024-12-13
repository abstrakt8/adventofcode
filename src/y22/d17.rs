use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
enum WindDirection {
    LEFT,
    RIGHT,
}

impl From<char> for WindDirection {
    fn from(value: char) -> Self {
        match value {
            '<' => WindDirection::LEFT,
            '>' => WindDirection::RIGHT,
            _ => unreachable!("Input wrong! {}", value),
        }
    }
}

type CellType = u8;
type GridType = Vec<Vec<CellType>>;

const TETROMINO_WIDTH: usize = 4;
const TETROMINO_HEIGHT: usize = 4;
const GRID_WIDTH: usize = 7;
type TetrominoCells = [[u8; TETROMINO_WIDTH]; TETROMINO_HEIGHT];

const HORIZONTAL_SHAPE: TetrominoCells = [
    [1, 1, 1, 1], //
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

const X_SHAPE: TetrominoCells = [
    [0, 1, 0, 0],
    [1, 1, 1, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0], //
];

const J_SHAPE: TetrominoCells = [
    [1, 1, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 0], //
];

const I_SHAPE: TetrominoCells = [
    [1, 0, 0, 0], //
    [1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0],
];
const O_SHAPE: TetrominoCells = [
    [1, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0], //
    [0, 0, 0, 0],
];

static TETROMINOS: [(TetrominoCells, usize, usize); 5] = [
    (HORIZONTAL_SHAPE, 1, 4),
    (X_SHAPE, 3, 3),
    (J_SHAPE, 3, 3),
    (I_SHAPE, 4, 1),
    (O_SHAPE, 2, 2),
];

#[derive(Default)]
struct Tetris {
    grid: GridType,
}

impl Debug for Tetris {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter().rev() {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|&x| if x == 1 { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Tetris {
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn contains(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.grid.len() as i32 && j >= 0 && j < self.grid[0].len() as i32
    }
    // (grid_row, grid_col) represents the position of the bottom left position of the Tetromino.
    pub fn intersects(&self, cells: &TetrominoCells, grid_row: usize, grid_col: usize) -> bool {
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                if cells[i][j] == 1
                    && grid_row + i < self.height()
                    && self.grid[grid_row + i][grid_col + j] == 1
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn put(&mut self, (row, col): (usize, usize), tetromino_type: usize) {
        let (cells, t_height, t_width) = TETROMINOS[tetromino_type];
        let mut h = self.height();
        for i in 0..t_height {
            if row + i >= h {
                self.grid.push(vec![0; GRID_WIDTH]);
                h += 1;
            }
            for j in 0..t_width {
                self.grid[row + i][col + j] |= cells[i][j];
            }
        }
    }

    pub fn next_pos(
        &self,
        (row, col): (usize, usize),
        tetromino_type: usize,
        wind_direction: WindDirection,
    ) -> (usize, usize) {
        let (cells, _, t_width) = &TETROMINOS[tetromino_type];
        // Update column with wind direction
        let col = {
            let new_col = col as i32
                + match wind_direction {
                    WindDirection::LEFT => -1,
                    WindDirection::RIGHT => 1,
                };
            if new_col < 0
                || new_col + (*t_width as i32) - 1 >= GRID_WIDTH as i32
                || self.intersects(cells, row, new_col as usize)
            {
                col // Either revert back
            } else {
                new_col as usize // Or we know it's a valid column and does not intersect
            }
        };

        // Already touching rock bottom
        if row == 0 {
            return (row, col);
        }

        // Update row with gravity
        let row = row - 1;
        if row < self.height() && self.intersects(cells, row, col) {
            (row + 1, col)
        } else {
            (row, col)
        }
    }
}

struct Solver {
    wind_direction: Vec<WindDirection>,
    tetris: Tetris,

    heights: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Solver {
    pub fn new(content: &str) -> Self {
        let wind_direction = content.trim().chars().map(WindDirection::from).collect();
        Self {
            wind_direction,
            tetris: Default::default(),
            heights: Default::default(),
        }
    }

    pub fn solve(&mut self, num_pieces: usize) -> usize {
        let mut wind_index = 0;
        let mut i = 0;
        let mut skipped_cycles_height = 0;
        let mut found_cycle = false;
        
        while i < num_pieces {
            let mut pos = (self.tetris.height() + 3, 2);
            let tetromino_type = i % TETROMINOS.len();

            loop {
                let wind_direction = self.wind_direction[wind_index % self.wind_direction.len()];
                let new_pos = self.tetris.next_pos(pos, tetromino_type, wind_direction);
                wind_index = (wind_index + 1) % self.wind_direction.len();
                if new_pos.0 == pos.0 {
                    pos = new_pos;
                    break;
                }
                pos = new_pos;
            }
            self.tetris.put(pos, tetromino_type);

            if !found_cycle {
                let key = (wind_index, tetromino_type);
                let h1 = self.tetris.height();
                if let Some(seen_heights) = self.heights.get(&key) {
                    for &(h2, j)in seen_heights.iter().rev() {
                        let h_diff = h1 - h2;
                        if h2 < h_diff {
                            continue;
                        }

                        found_cycle = self.tetris.grid[(h2- h_diff +1)..h2] == self.tetris.grid[(h1- h_diff +1)..h1];
                        if found_cycle {
                            let cycle_num_pieces = i - j;
                            let pieces_left = num_pieces - i;
                            let cycles_fit = pieces_left / cycle_num_pieces;
                            skipped_cycles_height = cycles_fit * h_diff;
                            i += cycles_fit * cycle_num_pieces;
                            // dbg!(skipped_cycles_height);
                            break;
                        }
                    }
                }
                self.heights
                    .entry(key)
                    .or_default()
                    .push((h1, i));
            }
            i += 1;
        }
        skipped_cycles_height + self.tetris.height()
    }
}

pub fn run(content: &str) -> (usize, usize) {
    let ans1 = Solver::new(content).solve(2022);
    let ans2 = Solver::new(content).solve(1000000000000);
    (ans1, ans2)
}
