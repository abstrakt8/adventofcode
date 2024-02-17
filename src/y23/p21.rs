use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::grid2d::GridCell;

const STEPS: usize = 64;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Node {
    cell: GridCell,
    steps: usize,
}

fn run(content: &str, required_steps: usize) -> u32 {
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut set = HashSet::new();


    let starting_cell = 'outer: loop {
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 'S' {
                    break 'outer Some(GridCell::new(i as i32, j as i32));
                }
            }
        }
        break 'outer None;
    }
    .unwrap(); // Yes, I'm lazy

    println!("{:?}", starting_cell);

    let mut q = VecDeque::new();

    let mut try_add = |node: Node, set: &mut HashSet<Node>, q: &mut VecDeque<Node>| {
        if set.insert(node.clone()) {
            q.push_back(node);
        }
    };
    try_add(
        Node {
            cell: starting_cell,
            steps: 0,
        },
        &mut set,
        &mut q,
    );

    let mut ans = 0;
    while let Some(Node { cell, steps }) = q.pop_front() {
        if steps == required_steps {
            ans += 1;
            continue;
        }
        for neighbor in cell.neighbors4() {
            if neighbor.inside_matrix(rows, cols)
                && grid[neighbor.x as usize][neighbor.y as usize] != '#'
            {
                try_add(
                    Node {
                        cell: neighbor,
                        steps: steps + 1,
                    },
                    &mut set,
                    &mut q,
                );
            }
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    #[test]
    pub fn test_example() {
        let content = r##"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."##;
        assert_eq!(run(content, 6), 16);
    }
    #[test]
    pub fn test_input() {
        let content = fs::read_to_string("21.in").expect("Should have been able to read the file");
        let ans = run(&content, 64);
        assert_eq!(ans, 3);
    }
}
