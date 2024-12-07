use regex::Regex;
use std::collections::VecDeque;

type Crates = Vec<VecDeque<u8>>;

pub fn parse_crates(input: &str) -> Crates {
    let v: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
    let mut crates: Crates = vec![];

    let last_line = *v.last().unwrap();

    // Assumption is that no two digits
    for x in 0..last_line.len() {
        let mut s = VecDeque::new();
        if last_line[x].is_ascii_digit() {
            for y in (0..v.len() - 1).rev() {
                if v[y][x].is_ascii_uppercase() {
                    s.push_back(v[y][x]);
                }
            }
            crates.push(s);
        }
    }

    crates
}

fn print_crates(crates: &Crates) {
    for i in 0..crates.len() {
        println!("[{i}] : {:?}", crates[i]);
    }
}

type Instructions = Vec<(usize, usize, usize)>;

fn solve1(mut crates: Crates, instructions: &Instructions) -> Crates {
    for &(a, b, c) in instructions {
        for _ in 0..a {
            if let Some(x) = crates[b].pop_back() {
                crates[c].push_back(x);
            }
        }
    }
    crates
}

fn solve2(mut crates: Crates, instructions: &Instructions) -> Crates {
    for &(a, b, c) in instructions {
        let mut v = vec![];
        for _ in 0..a {
            if let Some(x) = crates[b].pop_back() {
                v.push(x);
            }
        }
        crates[c].extend(v.into_iter().rev());
    }
    crates
}

pub fn top_of_the_crates(crates: Crates) -> String {
    String::from_utf8(crates.iter().filter_map(|c| c.back().copied()).collect()).unwrap()
}

pub fn run(content: &str) -> (String, String) {
    let mut it = content.split("\n\n");

    let crates = parse_crates(it.next().unwrap());
    let input = it.next().unwrap();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    print_crates(&crates);

    let instructions: Instructions = re
        .captures_iter(input)
        .map(|x| {
            let (_, [a, b, c]) = x.extract();
            let [a, b, c] = [a, b, c].map(|x| x.parse::<usize>().unwrap());
            (a, b - 1, c - 1)
        })
        .collect();

    let ans1 = top_of_the_crates(solve1(crates.clone(), &instructions));
    let ans2 = top_of_the_crates(solve2(crates, &instructions));
    (ans1, ans2)
}
