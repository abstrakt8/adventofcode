use std::cmp::min;
use std::fs;

pub(crate) fn solve() {
    let contents = fs::read_to_string("../../inputs/y23/4.in")
        .expect("Should have been able to read the file");
    let ans1 = process1(&contents);
    println!("{:}", ans1);

    let ans2 = process2(&contents);
    println!("{:}", ans2);
}

// For /benches
fn run(input: &str) -> u32 {
    process2(input)
}

fn count_line(line: &str) -> u32 {
    let card = line_parsing(line);
    let mut ans = 0;
    for i in card.1 {
        if card.0.contains(&i) {
            ans += 1;
        }
    }
    ans
}

fn process1(input: &str) -> u32 {
    input.lines().map(|l| {
        let c = count_line(l);
        if c == 0 { 0 } else { 1 << (c - 1) }
    }).sum()
}

fn process2(input: &str) -> u32 {
    let l: Vec<&str> = input.lines().collect();
    let n = l.len();
    let mut dp: Vec<u32> = vec![1; n];
    let mut suf: Vec<u32> = vec![0; n];

    for i in (0..l.len()).rev() {
        let x = count_line(l[i]) as usize;

        let end = min(dp.len() - 1, i + x);

        if i + 1 < l.len() {
            dp[i] += suf[i + 1];
        }

        if end + 1 < dp.len() {
            dp[i] -= suf[end + 1]
        }

        suf[i] = dp[i] ;
        if i + 1 < l.len() {
            suf[i] += suf[i + 1];
        }

        // dp[i] += dp[j];
        // for j in i + 1..=end {
        //     dp[i] += dp[j];
        // }

    }

    dp.iter().sum()
}

// struct Card<'a>(&'a Vec<u32>, &'a Vec<u32>);

fn number_parsing(input: &str) -> Vec<u32> {
    input.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
}

#[derive(PartialEq, Debug)]
struct Card(Vec<u32>, Vec<u32>);

fn line_parsing(input: &str) -> Card {
    let l = input.split(": ").last().unwrap();
    let x: Vec<Vec<u32>> = l.split("|").map(number_parsing).collect();
    Card(x[0].clone(), x[1].clone())
}

// mod tests {
//     use crate::p4::{Card, line_parsing, process1, count_line, process2};
//
//     #[test]
//     fn test_line_parsing() {
//         let f = vec![41, 48, 83, 86, 17];
//         let s = vec![83, 86, 6, 31, 17, 9, 48, 53];
//         assert_eq!(Card(f, s), line_parsing("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
//     }
//
//     #[test]
//     fn solve() {
//         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
//         assert_eq!(13, process1(input));
//     }
//
//     #[test]
//     fn test_solve_line() {
//         assert_eq!(0, count_line("Card 1: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
//     }
//
//     #[test]
//     fn test_solve_2() {
//         assert_eq!(30, process2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
//     }
// }