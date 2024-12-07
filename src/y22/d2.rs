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

fn str_to_state(b: &str) -> State {
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

fn str_to_ordering(s: &str) -> Ordering {
    match s {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
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

fn which_state(opp: State, outcome: Ordering) -> State {
    [State::Paper, State::Scissors, State::Rock]
        .iter()
        .find(|&me| ordering(*me, opp) == outcome)
        .cloned()
        .unwrap()
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
                .map(str_to_state)
                .collect_tuple()
                .unwrap();
            score(a, b)
        })
        .sum();

    let ans2 = content
        .lines()
        .map(|line| {
            let (a, w) = line.split_whitespace().collect_tuple().unwrap();
            let a = str_to_state(a);
            let outcome = str_to_ordering(w);
            let b = which_state(a, outcome);
            score(a, b)
        })
        .sum();
    (ans1, ans2)
}
