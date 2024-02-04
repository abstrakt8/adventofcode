#![allow(dead_code)]

use std::cmp::{max, min};

use color_eyre::eyre::eyre;
use color_eyre::Result;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn from_letter(letter: char) -> Result<Direction> {
        Ok(match letter.to_ascii_uppercase() {
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            'R' => Direction::RIGHT,
            'L' => Direction::LEFT,
            c => Err(eyre!("{} not a direction", c))?
        })
    }
}

// Dirs in random orders
pub const ALL_4_DIRS: [Direction; 4] = [Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN];

impl Direction {
    pub fn opposite(self: &Self) -> Self {
        match *self {
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
            Direction::DOWN => Direction::UP,
            Direction::UP => Direction::DOWN,
        }
    }
}

type T = i32;

// 2D GridCell 0-indexed
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

impl GridCell {
    pub fn new(x: T, y: T) -> Self {
        GridCell { x, y }
    }

    // Up, down, left, right neighbors
    // TODO: Remove '_ and check why it's needed
    pub fn neighbors4(&self) -> impl Iterator<Item=GridCell> + '_ {
        ALL_4_DIRS.iter().map(|d: &Direction| self.step_once(*d))
    }

    pub fn dirs(d: Direction) -> (i32, i32) {
        match d {
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
        }
    }

    // Inside the rectangle given by this
    pub fn inside_rectangle(&self, min_cell: GridCell, max_cell: GridCell) -> bool {
        self.x >= min_cell.x && self.x <= max_cell.x && self.y >= min_cell.y && self.y <= max_cell.y
    }

    pub fn inside_matrix(&self, rows: i32, cols: i32) -> bool {
        self.x >= 0 && self.x < rows && self.y >= 0 && self.y < cols
    }

    pub fn outside0(&self, rows: i32, cols: i32) -> bool {
        !self.inside_matrix(rows, cols)
    }

    pub fn step(&self, dir: Direction, n: i32) -> GridCell {
        let (dx, dy) = GridCell::dirs(dir);
        GridCell { x: self.x + dx * n, y: self.y + dy * n }
    }

    pub fn step_once(&self, dir: Direction) -> GridCell {
        self.step(dir, 1)
    }

    pub fn min_components(lhs: GridCell, rhs: GridCell) -> GridCell {
        GridCell::new(min(lhs.x, rhs.x), min(lhs.y, rhs.y))
    }

    pub fn max_components(lhs: GridCell, rhs: GridCell) -> GridCell {
        GridCell::new(max(lhs.x, rhs.x), max(lhs.y, rhs.y))
    }
}

#[cfg(test)]
mod test {
    use crate::utils::grid2d::Direction;

    #[test]
    pub fn test_direction_from_letter() -> color_eyre::Result<()> {
        assert_eq!(Direction::LEFT, Direction::from_letter('L')?);
        assert_eq!(Direction::RIGHT, Direction::from_letter('R')?);
        assert_eq!(Direction::RIGHT, Direction::from_letter('r')?);

        Ok(())
    }

    #[test]
    pub fn test_direction_from_letter_err() {
        assert!(Direction::from_letter('>').is_err());
        assert!(Direction::from_letter('^').is_err());
        assert!(Direction::from_letter('\n').is_err());
    }
}