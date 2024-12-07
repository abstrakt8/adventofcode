type Grid = Vec<Vec<char>>;

fn check_horizontal(grid: &Grid) -> Vec<usize> {
    let mut ans = vec![];
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
            ans.push(i);
        }
    }
    ans
}

fn transpose(grid: &Grid) -> Grid {
    if grid.is_empty() { return vec![]; }

    (0..grid[0].len()).map(|i| {
        (0..grid.len()).map(|j|
            grid[j][i]
        ).collect()
    }).collect()
}

fn other(c: char) -> char {
    if c == '#' { '.' } else { '#' }
}

fn solve_one(input: &str) -> usize {
    println!("Input={input}");
    if input.trim().is_empty() { return 0; }

    let mut grid = input.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();

    for factor in [100, 1] {
        let old = check_horizontal(&grid);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] = other(grid[i][j]);
                for cand in check_horizontal(&grid) {
                    if !old.contains(&cand) {
                        return factor * cand;
                    }
                }
                grid[i][j] = other(grid[i][j]);
            }
        }
        grid = transpose(&grid);
    }

    panic!("Not solvable???");
}


pub(crate) fn run(input: &str) -> usize {
    input.split("\n\n").map(solve_one).sum()
}



pub fn test() {
    let input = r##"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."##;

    assert_eq!(solve_one(input), 300);

    let input = r##"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"##;

    assert_eq!(solve_one(input), 100);
}