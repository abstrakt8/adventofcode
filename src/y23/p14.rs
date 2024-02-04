const ROCK: char = '#';
const BALL: char = 'O';

pub fn run(content: &str) -> u64 {
    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (n, m) = (grid.len(), grid[0].len());
    let mut ans = 0u64;
    for j in 0..m {
        let mut last = 0;
        for i in 0..m {
            if grid[i][j] == ROCK {
                last = i + 1;
            }
            if grid[i][j] == BALL {
                last += 1;
                ans += (n - last + 1) as u64;
            }
        }
    }
    ans
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
    assert_eq!(run(content), 136);
}