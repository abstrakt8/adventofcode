use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::iter;
use std::iter::empty;
use std::slice::Iter;
use lazy_static::lazy_static;

/**
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 **/
// lazy_static! {
//     pub static ref DIRECTIONS: HashMap<char, [[i8; 2]; 2]> = {
//         let mut map = HashMap::new();
//         map.insert('|', [[0, 1], [0, -1]]);
//         map.insert('-', [[1, 0], [-1, 0]]);
//         map.insert('L', [[1, 0], [0, -1]]);
//         map.insert('J', [[-1, 0], [0, -1]]);
//         map.insert('7', [[-1, 0], [0, 1]]);
//         map.insert('F', [[1, 0], [0, 1]]);
//
//         map
//     };
// }
pub fn dirs() -> HashMap<char, [[i32; 2]; 2]> {
    let mut map = HashMap::new();
    map.insert('|', [[-1, 0], [1, 0]]);
    map.insert('-', [[0, 1], [0, -1]]);
    map.insert('L', [[0, 1], [-1, 0]]);
    map.insert('J', [[0, -1], [-1, 0]]);
    map.insert('7', [[0, -1], [1, 0]]);
    map.insert('F', [[0, 1], [1, 0]]);

    map
}


pub fn run(content: &str) -> i32 {
    let mut grid = content
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let (n, m) = (grid.len() as i32, grid[0].len() as i32);
    let directions = dirs();
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();

    let (mut sx, mut sy) = (-1, -1);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                (sx, sy) = (i as i32, j as i32);
            }
        }
    }

    assert!(sx >= 0 && sy >= 0);

    let mut q = VecDeque::new();
    let in_grid = |x: i32, y: i32| x >= 0 && x < n && y >= 0 && y < m;
    let neighbors = |x, y| {
        let it = directions.get(&grid[x as usize][y as usize]);
        let it = it
            .map(move |dirs|
                dirs
                    .iter()
                    .map(move |[dx, dy]| [x + dx, y + dy])
                    .filter(move |[nx, ny]| in_grid(*nx, *ny))
            );

        it
    };
    println!("Starting position = {sx},{sy}");
    let mut ans = 0;
    let mut num_inside = 0;

    let mut check = |x: i32, y: i32, d: i32, q: &mut VecDeque<_>, dist: &mut HashMap<(i32, i32), i32>| {
        if !dist.contains_key(&(x, y)) {
            dist.insert((x, y), d);
            q.push_back([x, y]);
            ans = max(ans, d);
        }
    };

    let mut k = 0;
    let mut a = [[0, 0], [0, 0]];
    let mut starting_letter = ' ';
    for [dx, dy] in [[0, 1], [-1, 0], [1, 0], [0, -1]] {
        let (nx, ny) = (sx + dx, sy + dy);
        if !in_grid(nx, ny) {
            continue;
        }
        if let Some(mut it) = neighbors(nx, ny) {
            if it.find(|[x, y]| *x == sx && *y == sy).is_some() {
                check(nx, ny, 1, &mut q, &mut dist);
                a[k] = [dx, dy];
                k += 1;
            }
        }
        // Find out what the letter is instead of S
    }

    a.sort();
    for (letter, dirs) in &directions {
        let mut t = dirs.clone();
        t.sort();
        if t == a {
            println!("Starting position is {}", letter);
            starting_letter = *letter;
            // grid[sx as usize][sy as usize] = *letter;
        }
    }

    // println!("q.size() = {}", q.len());

    while let Some([x, y]) = q.pop_front() {
        let d = *dist.get(&(x, y)).unwrap();
        if let Some(mut it) = neighbors(x, y) {
            it.for_each(|[x, y]| {
                check(x, y, d + 1, &mut q, &mut dist);
            });
        }
    }

    for i in 0..n {
        let mut mask = 0;
        for j in 0..m {
            // let c = grid[i as usize][j as usize];
            if dist.contains_key(&(i, j)) {
                let letter = match grid[i as usize][j as usize] {
                    'S' => starting_letter,
                    c => c
                };
                mask ^= match letter {
                    '|' => 0b11,
                    'L' | 'J' => 0b10,
                    '7' | 'F' => 0b01,
                    _ => 0b00,
                };
            } else {

                if mask == 0b11 {
                    num_inside += 1;
                } else if mask == 0b10 || mask == 0b01 {
                    println!("hhuh");
                }
            }
        }
    }

    num_inside
}

#[test]
pub fn test_example1() {
    let content = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(run(content), 1);
    // println!("{}", run(content));
}

#[test]
pub fn test_example2() {
    let content = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(run(content), 8);
    // println!("{}", run(content));
}
#[test]
pub fn test_example4() {
    let content = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    assert_eq!(run(content), 10);

}