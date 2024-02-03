use std::fmt::Display;

use num::abs;

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    abs(x1 - x2) + abs(y1 - y2)
}

pub fn run(content: &str, expansion_rate: i32) -> i64 {
    let grid: Vec<&str> = content
        .lines()
        .collect();

    let mut col_empty = vec![true; grid[0].len()];
    for &row in &grid {
        for (c, col_empty) in row.chars().zip(col_empty.iter_mut()) {
            if c == '#' {
                *col_empty = false;
            }
        }
    }

    let mut expanded_rows = 0;
    let mut positions = vec![];

    let expand = |empty: bool| if empty { expansion_rate - 1 } else { 0 };

    for &row in &grid {
        let mut is_empty = true;
        let mut expanded_col = 0;
        for (c, col_empty) in row.chars().zip(col_empty.iter()) {
            if c == '#' {
                positions.push((expanded_rows, expanded_col));
                is_empty = false;
            }
            expanded_col += 1 + expand(*col_empty);
        }
        expanded_rows += 1 + expand(is_empty);
    }

    let mut sum = 0i64;
    for i in 0..positions.len() {
        for j in 0..i {
            let d = dist(positions[i], positions[j]);
            sum += d as i64;
            // println!("{i} {j} {d} {:},{:} vs {:},{:}", positions[i].0, positions[i].1, positions[j].0, positions[j].1);
        }
    }

    sum
}


#[test]
pub fn test_example() {
    let content = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(run(content, 2), 374);
    assert_eq!(run(content, 10), 1030);
    assert_eq!(run(content, 100), 8410);
}