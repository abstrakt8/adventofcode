use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
enum State {
    Rock,
    Paper,
    Scissors,
}

fn map_state(state: State) -> u32 {
    match state {
        State::Rock => 1,
        State::Paper => 2,
        State::Scissors => 3,
    }
}

fn map_str(b: &str) -> State {
    match b {
        "A" => State::Rock,
        "B" => State::Paper,
        "C" => State::Scissors,
        "X" => State::Rock,
        "Y" => State::Paper,
        "Z" => State::Scissors,
        _ => unreachable!(),
    }
}

fn ordering(x: State, y: State) -> Ordering {
    match x {
        State::Rock => match y {
            State::Rock => Ordering::Equal,
            State::Paper => Ordering::Less,
            State::Scissors => Ordering::Greater,
        },
        State::Scissors => match y {
            State::Rock => Ordering::Less,
            State::Paper => Ordering::Greater,
            State::Scissors => Ordering::Equal,
        },
        State::Paper => match y {
            State::Rock => Ordering::Greater,
            State::Paper => Ordering::Equal,
            State::Scissors => Ordering::Less,
        },
    }
}

fn map_ordering(ordering: Ordering) -> u32 {
    match ordering {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

fn score(opponent: State, me: State) -> u32 {
    map_ordering(ordering(me, opponent)) + map_state(me)
}

pub fn run(content: &str) -> (u32, u32) {
    let ans1 = content
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_whitespace()
                .map(map_str)
                .collect_tuple()
                .unwrap();
            score(a, b)
        })
        .sum();

    (ans1, 0)
}
