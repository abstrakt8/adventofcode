use std::collections::{HashMap, HashSet, VecDeque};
use enum_map::{Enum, enum_map, EnumMap};
use lazy_static::lazy_static;

#[derive(Hash, Eq, PartialEq, Debug, Enum, Clone, Copy)]
pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}


struct LazerLogic;


impl LazerLogic {}

lazy_static! {
    static ref DIRS_DELTAS: EnumMap<Direction, (i32, i32)> = enum_map! {
        Direction::LEFT => (0, -1),
        Direction::RIGHT => (0, 1),
        Direction::UP => (-1, 0),
        Direction::DOWN => (1, 0),
    };
}

type Data = (i32, i32, Direction);

fn step((i, j, dir): &Data) -> Data {
    let (dx, dy) = DIRS_DELTAS[*dir];
    (i + dx, j + dy, *dir)
}

fn split_dirs(direction: Direction, splitter: char) -> Vec<Direction> {
    match splitter {
        '|' => match direction {
            Direction::LEFT | Direction::RIGHT => vec![Direction::UP, Direction::DOWN],
            d => vec![d]
        },
        '-' => match direction {
            Direction::UP | Direction::DOWN => vec![Direction::LEFT, Direction::RIGHT],
            d => vec![d]
        },
        _ => vec![]
    }
}

fn apply_dirs(i: i32, j: i32, dirs: &[Direction]) -> Vec<Data> {
    dirs.iter().map(|new_dir| step(&(i, j, *new_dir))).collect()
}

fn reflect_dir(dir: Direction, mirror: char) -> Vec<Direction> {
    let c = match mirror {
        '\\' => match dir {
            Direction::RIGHT => Direction::DOWN,
            Direction::UP => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
        },
        '/' => match dir {
            Direction::RIGHT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
        },
        _ => return vec![]
    };
    vec![c]
}


fn lazer(data: Data, c: char) -> Vec<Data> {
    let (i, j, dir) = data;

    match c {
        '.' => apply_dirs(i, j, &[dir]),
        '|' | '-' => apply_dirs(i, j, &split_dirs(dir, c)),
        '\\' | '/' => apply_dirs(i, j, &reflect_dir(dir, c)),
        _ => panic!()
    }
}


pub fn run(content: &str) -> u32 {
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut vis: HashSet<Data> = HashSet::new();
    let mut vis_xy: HashSet<(i32, i32)> = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back((0, 0, Direction::RIGHT));

    while let Some(data) = q.pop_front() {
        vis_xy.insert((data.0, data.1));
        for (i, j, dir) in lazer(data, grid[data.0 as usize][data.1 as usize]) {
            if !(i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[i as usize].len() as i32) {
                continue;
            }
            if !vis.insert((i, j, dir)) {
                continue;
            }
            q.push_back((i, j, dir));
        }
    }
    vis_xy.len() as u32
}

#[test]
pub fn test_lazer() {
    assert_eq!(lazer((1, 0, Direction::RIGHT), '\\'), vec![(2, 0, Direction::DOWN)]);
    assert_eq!(lazer((1, 0, Direction::RIGHT), '|'), vec![(0, 0, Direction::UP), (2, 0, Direction::DOWN)]);

    assert_eq!(lazer((1, 0, Direction::RIGHT), '-'), vec![(1, 1, Direction::RIGHT)]);
    assert_eq!(lazer((1, 0, Direction::RIGHT), '.'), vec![(1, 1, Direction::RIGHT)]);


    assert_eq!(lazer((1, 0, Direction::DOWN), '\\'), vec![(1, 1, Direction::RIGHT)]);

    assert_eq!(lazer((1, 0, Direction::DOWN), '-'), vec![(1, -1, Direction::LEFT), (1, 1, Direction::RIGHT)]);

    assert_eq!(lazer((1, 0, Direction::DOWN), '|'), vec![(2, 0, Direction::DOWN)]);
    assert_eq!(lazer((1, 0, Direction::DOWN), '.'), vec![(2, 0, Direction::DOWN)]);
}
#[test]
pub fn test_run() {
    let content = r##".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."##;
    assert_eq!(run(content), 46);
}