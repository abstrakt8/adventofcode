use adventofcode::y24;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/y24/4.in");

#[divan::bench(sample_count = 1000)]
fn all_threads() {
    y24::p4_rayon_lookup_table::run2(INPUT);
}

#[divan::bench(sample_count = 1000)]
fn four_threads() {
    y24::p4_rayon_lookup_table::run2_4threads(INPUT);
}
