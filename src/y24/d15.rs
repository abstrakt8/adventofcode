use std::collections::{HashMap};

#[derive(Debug, Copy, Clone)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Direction::UP,
            b'<' => Direction::LEFT,
            b'>' => Direction::RIGHT,
            b'v' => Direction::DOWN,
            _ => unreachable!("Unexpected character"),
        }
    }
}

impl Direction {
    pub fn into_direction(self) -> [i32; 2] {
        match self {
            Direction::LEFT => [0, -1],
            Direction::RIGHT => [0, 1],
            Direction::UP => [-1, 0],
            Direction::DOWN => [1, 0],
        }
    }
    pub fn reverse(self) -> Self {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
        }
    }
}

fn go(pos: [usize; 2], d: Direction) -> [usize; 2] {
    let [dx, dy] = d.into_direction();
    [(pos[0] as i32 + dx) as usize, (pos[1] as i32 + dy) as usize]
}

pub fn gps_score(grid: &Grid) -> usize {
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                BOX | BOX1 => {
                    ans += i * 100 + j;
                }
                _ => {}
            }
        }
    }
    ans
}

fn print(grid: &Vec<Vec<u8>>) {
    for row in grid {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    }
}

pub fn run1(content: &str) -> usize {
    let (grid, moves) = content.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<u8>> = grid.lines().map(|line| line.as_bytes().to_vec()).collect();

    let n = grid.len();
    let m = grid[0].len();

    let mut pos = 'initial: {
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == b'@' {
                    break 'initial [i, j];
                }
            }
        }
        unreachable!("")
    };

    moves.lines().for_each(|line| {
        'lo: for &b in line.as_bytes() {
            let d = Direction::from(b);
            let [ni, nj] = go(pos, d);
            // print(&grid);
            // println!("{}", b as char);

            match grid[ni][nj] {
                b'#' => continue 'lo,
                b'O' => {
                    let mut o = [ni, nj];
                    loop {
                        o = go(o, d);
                        match grid[o[0]][o[1]] {
                            b'#' => continue 'lo,
                            b'O' => continue,
                            b'.' => {
                                grid[pos[0]][pos[1]] = b'.';
                                pos = [ni, nj];
                                grid[ni][nj] = b'@';
                                grid[o[0]][o[1]] = b'O';
                                continue 'lo;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                b'.' => {
                    grid[pos[0]][pos[1]] = b'.';
                    pos = [ni, nj];
                    grid[ni][nj] = b'@';
                }
                _ => unreachable!(),
            }
        }
    });
    print(&grid);

    gps_score(&grid)
}

type Grid = Vec<Vec<u8>>;
fn transform_grid(grid: Grid) -> Grid {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|b| match b {
                    b'#' | b'.' => [b, b],
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!("Unexpected char {}", b),
                })
                .flatten()
                .collect()
        })
        .collect()
}

const WALL: u8 = b'#';
const BOX: u8 = b'O';
const BOX1: u8 = b'[';
const BOX2: u8 = b']';
const EMPTY: u8 = b'.';
const DOG: u8 = b'@';

// Hash
fn can_move(
    grid: &Grid,
    new_grid: &mut Grid,
    seen: &mut HashMap<[usize; 2], bool>,
    // processed: &mut HashSet<[usize; 2]>,
    i: usize,
    j: usize,
    d: Direction,
) -> bool {
    if let Some(val) = seen.get(&[i, j]) {
        return *val;
    }

    let [ni, nj] = go([i, j], d);
    let mut to_pos = vec![[ni, nj]];
    let can = match grid[i][j] {
        EMPTY => return true,
        WALL => return false,
        // Here we continue
        BOX | DOG => can_move(grid, new_grid, seen, ni, nj, d),
        BOX1 | BOX2 => {
            if matches!(d, Direction::DOWN | Direction::UP) {
                let ox: i32 = if grid[i][j] == BOX1 { 1 } else { -1i32 };
                let [oi, oj] = [i, (j as i32 + ox) as usize];
                to_pos.push(go([oi, oj], d));
            }
            to_pos
                .iter()
                .all(|p| can_move(grid, new_grid, seen, p[0], p[1], d))
        }
        _ => unreachable!(),
    };

    for to in to_pos {
        let from = go(to, d.reverse());
        if can {
            new_grid[to[0]][to[1]] = grid[from[0]][from[1]];
            new_grid[from[0]][from[1]] = b'.';
        }
        seen.insert(from, can);
    }

    can
}

pub fn coutn_boxes(grid: &Grid) -> usize {
    let mut box1: usize = 0;
    for i in 0..grid.len(){
        for j in 0..grid[i].len() {
            if grid[i][j] == BOX1 {
                box1 += 1;
            }
        }
    }
    box1
}

pub fn run2(content: &str) -> usize {
    let (grid, moves) = content.split_once("\n\n").unwrap();

    let grid: Vec<Vec<u8>> = grid.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut grid = transform_grid(grid);
    let moves: Vec<u8> = moves
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .flatten()
        .collect();

    let mut pos = 'initial: {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'@' {
                    break 'initial [i, j];
                }
            }
        }
        unreachable!("")
    };

    print(&grid);


    let box1 = coutn_boxes(&grid);
    for byte in moves {
        let d = Direction::from(byte);
        // print(&grid);
        let mut new_grid = grid.clone();
        let mut seen = HashMap::new();
        // let mut processed = HashSet::new();
        if can_move(
            &grid,
            &mut new_grid,
            &mut seen,
            pos[0],
            pos[1],
            d,
        ) {
            pos = go(pos, d);
            grid = new_grid;
        }

        // for i in 0..grid.len() {
        //     for j in 1..grid[i].len() {
        //         if (new_grid[i][j] == BOX2) ^ (new_grid[i][j - 1] == BOX1) {
        //             println!("{}", byte as char);
        //             print(&grid);
        //             print(&new_grid);
        // 
        //             panic!()
        //         };
        //     }
        // }
        // if box1 != coutn_boxes(&new_grid) {
        //     panic!("boxes not same");
        // }
    }

    print(&grid);
    gps_score(&grid)
}

pub fn run(content: &str) -> usize {
    run2(content)
}
