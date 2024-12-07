use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

const ROCK: char = '#';
const BALL: char = 'O';

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum Cell {
    EMPTY,
    BALL,
    ROCK,
}

#[derive(Hash, Clone, Eq, PartialEq)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.cells.iter()
            .map(|row| row.iter().map(|x| match x {
                Cell::EMPTY => ".".to_string(),
                Cell::BALL => "O".to_string(),
                Cell::ROCK => "#".to_string(),
            }).collect::<String>()
            ).join("\n");
        write!(f, "{}", str)
    }
}

impl Grid {
    fn score(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j] == Cell::BALL {
                    sum += self.cells.len() - i;
                }
            }
        }
        sum
    }
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let t = self.cells[x1][y1];
        self.cells[x1][y1] = self.cells[x2][y2];
        self.cells[x2][y2] = t;
    }

    fn tilt_cycle(&mut self) {
        let (n, m) = (self.cells.len(), self.cells[0].len());

        // NORTH
        for j in 0..m {
            let mut last = 0;
            for i in 0..n {
                match self.cells[i][j] {
                    Cell::ROCK => last = i + 1,
                    Cell::BALL => {
                        self.swap(i, j, last, j);
                        last += 1;
                    }
                    _ => {}
                }
            }
        }
        // println!("NORTH\n{}\n", self);

        // WEST
        for i in 0..n {
            let mut last = 0;
            for j in 0..m {
                match self.cells[i][j] {
                    Cell::ROCK => last = j + 1,
                    Cell::BALL => {
                        self.swap(i, j, i, last);
                        last += 1;
                    }
                    _ => {}
                }
            }
        }

        // println!("WEST\n{}\n", self);

        // SOUTH
        for j in 0..m {
            let mut last = n - 1;
            for i in (0..n).rev() {
                match self.cells[i][j] {
                    Cell::ROCK => last = i.saturating_sub(1),
                    Cell::BALL => {
                        self.swap(i, j, last, j);
                        last = last.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        // println!("SOUTH\n{}\n", self);

        // EAST
        for i in 0..n {
            let mut last = m - 1;
            for j in (0..m).rev() {
                match self.cells[i][j] {
                    Cell::ROCK => last = j.saturating_sub(1),
                    Cell::BALL => {
                        self.swap(i, j, i, last);
                        last = last.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        // println!("EAST\n{}\n", self);
    }
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::ROCK),
            'O' => Ok(Cell::BALL),
            '.' => Ok(Cell::EMPTY),
            _ => Err(())
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines()
            .map(|line|
                line.chars()
                    .filter_map(|c| c.try_into().ok())
                    // .filter_map(|c| c.try_into::<Cell>().ok())
                    .collect::<Vec<Cell>>()
            )
            .collect::<Vec<Vec<Cell>>>();
        Ok(Grid { cells })
    }
}


pub fn run(content: &str) -> u64 {
    let mut seen = HashMap::new();

    let mut iterations = 0;

    let mut grid = content.parse::<Grid>().unwrap();

    let last_time = loop {
        // println!("Cycle={iterations}\n{}\n", grid);
        if let Some(last_time) = seen.insert(grid.clone(), iterations) {
            break last_time;
        }
        iterations += 1;
        grid.tilt_cycle();
    };

    println!("{last_time} {iterations}");

    let cycle_len = iterations - last_time;
    let left = 1_000_000_000 - iterations;

    for k in 0..left % cycle_len {
        grid.tilt_cycle();
    }

    grid.score() as u64
}

#[test]
pub fn test_example() {
    let content = r##"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."##;

    assert_eq!(run(content), 64);
}