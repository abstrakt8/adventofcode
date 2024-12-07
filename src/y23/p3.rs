use std::collections::{HashMap, HashSet};
use std::fs;

fn process1(grid_str: &str) -> u32 {
    let grid: Vec<Vec<char>> = grid_str.lines().map(|s| s.chars().collect()).collect();
    let (n, m) = (grid.len(), grid[0].len());
    let adjacent_to_symbol = |i: usize, j: usize| -> bool {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let (ni, nj) = (i as i32 + dx, j as i32 + dy);
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    let c = grid[ni as usize][nj as usize];
                    if c != '.' && !c.is_ascii_digit() {
                        return true;
                    }
                }
            }
        }
        false
    };
    let mut ans = 0;
    for i in 0..grid.len() {
        let mut x = 0;
        let mut f = 0;

        for j in 0..grid.len() {
            if let Some(d) = grid[i][j].to_digit(10) {
                if adjacent_to_symbol(i, j) {
                    f = 1
                }
                x = x * 10 + d;
            } else {
                ans += f * x;
                f = 0;
                x = 0;
            }
        }
        ans += f * x;
    }
    ans
}

fn process2(grid_str: &str) -> u32 {
    let grid: Vec<Vec<char>> = grid_str.lines().map(|s| s.chars().collect()).collect();
    let (n, m) = (grid.len(), grid[0].len());
    let adjacent_stars = |i: usize, j: usize| -> Vec<[i32; 2]> {
        let mut out = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                let (ni, nj) = (i as i32 + dx, j as i32 + dy);
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    let c = grid[ni as usize][nj as usize];
                    if c == '*' {
                        out.push([ni, nj]);
                    }
                }
            }
        }
        out
    };

    let mut ans = 0;
    let mut reverse_pos: HashMap<[i32; 2], Vec<u32>> = HashMap::new();
    for i in 0..grid.len() {
        let mut x = 0;
        let mut stars: HashSet<[i32; 2]> = HashSet::new();

        for j in 0..grid.len() {
            if let Some(d) = grid[i][j].to_digit(10) {
                x = x * 10 + d;
                adjacent_stars(i, j).iter().for_each(|pos| { stars.insert(*pos); });
            } else {
                stars.iter().for_each(|pos| {
                    reverse_pos.entry(*pos).or_default().push(x);
                });
                stars.clear();
                x = 0;
            }
        }
        stars.iter().for_each(|pos| {
            reverse_pos.entry(*pos).or_default().push(x);
        });
        // flush(x); TODO How to do this cleanly?
    }

    for (_, numbers) in reverse_pos {
        if numbers.len() == 2 {
            ans += numbers[0] * numbers[1]
        }
    }
    ans
}

pub fn solve() {
    let contents = fs::read_to_string("../../inputs/y23/3.in")
        .expect("Should have been able to read the file");
    let ans1 = process1(&contents);
    println!("{:}", ans1);

    let ans2 = process2(&contents);
    println!("{:}", ans2);
}

#[cfg(test)]
mod tests {
    use super::{process1, process2};

    #[test]
    fn test1() {
        let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process1(s));
    }

    #[test]
    fn test2() {
        let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process2(s));
    }
}