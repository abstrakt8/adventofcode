use adventofcode::y24;
use adventofcode::y24::p6::Part::{Part1, Part2};

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/y24/6.in");

#[divan::bench(sample_count = 10000)]
fn p1() {
    y24::p6::run_part(INPUT, Part1);
}
#[divan::bench(sample_count = 100)]
fn p2() {
    y24::p6::run_part(INPUT, Part2);
}


// #[divan::bench(sample_count = 1000)]
// fn four_threads() {
//     y24::p4_rayon_lookup_table::run2_4threads(INPUT);
// }
