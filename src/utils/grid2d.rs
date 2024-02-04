#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// Dirs in random orders
pub const ALL_DIRS: [Direction; 4] = [Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN];

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

// 2D GridCell 0-indexed
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct GridCell {
    x: i32,
    y: i32,
}

impl GridCell {
    fn new(x: i32, y: i32) -> Self {
        GridCell { x, y }
    }
}

impl GridCell {
    pub fn dirs(d: Direction) -> (i32, i32) {
        match d {
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
        }
    }
    pub fn inside(&self, rows: i32, cols: i32) -> bool {
        self.x >= 0 && self.x < rows && self.y >= 0 && self.y < cols
    }

    pub fn outside(&self, rows: i32, cols: i32) -> bool {
        !self.inside(rows, cols)
    }

    pub fn step(&self, dir: Direction) -> GridCell {
        let (dx, dy) = GridCell::dirs(dir);
        GridCell { x: self.x + dx, y: self.y + dy }
    }
}
