use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// Just some random order
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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct GridCell {
    x: i32,
    y: i32,
}

impl GridCell {
    fn new(x: i32, y: i32) -> Self {
        GridCell { x, y }
    }
}

// 2D GridCell 0-indexed
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

#[derive(Eq, PartialEq)]
struct DijkstraData {
    dist: i32,
    node: Node,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Node {
    cell: GridCell,

    // k stands for konsekutive
    k: i32,

    dir: Direction,
}

impl Node {
    pub fn new(cell: GridCell, k: i32, dir: Direction) -> Self {
        Node { cell, k, dir }
    }
}

impl PartialOrd<Self> for DijkstraData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist.cmp(&other.dist).reverse())
    }
}

impl Ord for DijkstraData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

}


pub(crate) fn run(content: &str) -> u32 {
    let grid: Vec<Vec<u32>> = content
        .lines()
        .map(|line| line.chars().flat_map(|x| x.to_digit(10)).collect())
        .collect();
    let (n, m) = (grid.len() as i32, grid[0].len() as i32);

    let mut q: BinaryHeap<DijkstraData> = BinaryHeap::new();
    let mut lookup: HashMap<Node, i32> = HashMap::new();

    let mut check = |node: Node, new_dist: i32, q: &mut BinaryHeap<DijkstraData>, lookup: &mut HashMap<Node, i32>| {
        let mut updated = false;
        lookup.entry(node).and_modify(|cur_dist| {
            if new_dist < *cur_dist {
                *cur_dist = new_dist;
                updated = true;
            }
        }).or_insert_with(|| {
            updated = true;
            new_dist
        });
        if updated {
            q.push(DijkstraData { node, dist: new_dist });
        }
    };

    check(Node::new(GridCell::new(0, 1), 1, Direction::RIGHT), grid[0][1] as i32, &mut q, &mut lookup);
    check(Node::new(GridCell::new(1, 0), 1, Direction::DOWN), grid[1][0] as i32, &mut q, &mut lookup);

    let final_cell = GridCell::new(n - 1, m - 1);
    while let Some(DijkstraData { node, dist }) = q.pop() {
        if Some(&dist) != lookup.get(&node) {
            continue;
        }

        if node.cell == final_cell {
            return dist as u32;
        }

        let other_dirs = ALL_DIRS.iter().filter(|&&d| d != node.dir.opposite());
        for &new_dir in other_dirs {
            let new_cell = node.cell.step(new_dir);
            let new_k = if new_dir == node.dir { node.k + 1 } else { 1 };

            if new_k > 3 || new_cell.outside(n, m) {
                continue;
            }

            let new_dist = dist + (grid[new_cell.x as usize][new_cell.y as usize] as i32);
            check(Node::new(new_cell, new_k, new_dir), new_dist, &mut q, &mut lookup);
        }
    }
    panic!("Could not reach the end!")
}

#[test]
fn test_example() {
    let s = r##"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"##;
    assert_eq!(run(s), 102);
}