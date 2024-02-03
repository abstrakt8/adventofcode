type Grid = Vec<Vec<char>>;

fn check_horizontal(grid: &Grid) -> Option<usize> {
    for i in 1..grid.len() {
        let mut k = 0;
        let mut flag = true;
        while i >= k + 1 && i + k < grid.len() && flag {
            for j in 0..grid[i].len() {
                flag &= grid[i - 1 - k][j] == grid[i + k][j];
            }
            k += 1;
        }
        if flag {
            return Some(i);
        }
    }
    None
}

fn transpose(grid: &Grid) -> Grid {
    if grid.is_empty() { return vec![]; }

    (0..grid[0].len()).map(|i| {
        (0..grid.len()).map(|j|
            grid[j][i]
        ).collect()
    }).collect()
}

fn solve_one(input: &str) -> usize {
    if input.trim().is_empty() { return 0; }

    let grid = input.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();

    let row = check_horizontal(&grid).unwrap_or(0) * 100;
    let col = check_horizontal(&transpose(&grid)).unwrap_or(0) * 1;
    println!("{row} {col}");
    row + col
}


pub(crate) fn run(input: &str) -> usize {
    input.split("\n\n").map(solve_one).sum()
}


#[test]
pub fn test() {
    let input = r##"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."##;

    assert_eq!(solve_one(input), 5);

    let input = r##"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"##;

    assert_eq!(solve_one(input), 400);
}