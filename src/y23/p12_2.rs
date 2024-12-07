use itertools::Itertools;
use std::iter::{once, repeat};

const N: usize = 5;

pub fn solve(line: &str) -> u64 {
    if line.trim().is_empty() { return 0; };
    let mut it = line.split(' ');

    let field = repeat(it.next().unwrap().chars())
        .take(N)
        .intersperse("?".chars())
        .flatten()
        .chain(once('.'))
        .collect::<Vec<char>>();

    let groups = it.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap());
    let groups = repeat(groups).take(N).flatten().collect::<Vec<usize>>();

    // println!("{line}");
    // dp[i][j][k]
    // i .. first i positions -> field[0..i]
    // j .. first j groups -> groups[0..j]
    // k .. how many consecutive # in the suffix of field[0..i], i.e. field[i-k..i] consists entirely of '#' whilst field[i-k-1] is a '.'

    let n = field.len();
    let m = groups.len();

    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..=m {
            for k in 0..=(if j < m { groups[j] } else { 0 }) {
                let cur = dp[i][j][k];
                if cur == 0 {
                    continue;
                }

                let mut handle_pound = |next: &mut Vec<Vec<u64>>| {
                    if j < m && k < groups[j] {
                        next[j][k + 1] += cur;
                    }
                };

                let mut handle_dot = |next: &mut Vec<Vec<u64>>| {
                    // group j satisfied, let's consider next group j+1
                    if j < m && k > 0 && k == groups[j] {
                        next[j + 1][0] += cur;
                    }
                    // we were not in a group
                    if k == 0 {
                        next[j][k] += cur;
                    }
                };


                let next = &mut dp[i + 1];
                match field[i] {
                    '.' => handle_dot(next),
                    '#' => handle_pound(next),
                    '?' => {
                        handle_dot(next);
                        handle_pound(next);
                    }
                    _ => {}
                }
            }
        }
    }
    dp[n][m][0]
}

pub fn run(content: &str) -> u64 {
    content.lines().map(solve).sum()
}

// #[test]
// // #[case(, 1)]
// pub fn example() {
//     assert_eq!(solve("?.?.?????????. 1,1,1,2"), 136744815586);
// }
//
// #[cfg(test)]
// mod test {
//     use rstest::rstest;
//     use crate::p12_2::solve;
//
//     #[rstest]
//     #[case(".??..??...?##. 1,1,3", 16384)]
//     #[case("???.### 1,1,3", 1)]
//     #[case("?###???????? 3,2,1", 506250)]
//     pub fn more(#[case] input: &str, #[case] expected: u64) {
//         assert_eq!(expected, solve(input));
//     }
// }
