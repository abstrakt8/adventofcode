use adventofcode::y24;
use adventofcode::y24::p7::Part::{Part1, Part2};

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/y24/7.in");

#[divan::bench(sample_count = 300)]
fn p1() {
    y24::p7::run_part(INPUT, Part1);
}

#[divan::bench(sample_count = 10)]
fn p2() {
    y24::p7::run_part(INPUT, Part2);
}
